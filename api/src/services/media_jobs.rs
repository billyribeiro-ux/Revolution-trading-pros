//! Durable image jobs. Processing uses actual scanned object bytes and retains
//! originals; a failed attempt never marks media optimized or publishes variants.
use anyhow::{Context, Result, bail};
use axum::body::Bytes;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use sqlx::{FromRow, Postgres, Transaction};
use std::time::{Duration, Instant};

use crate::{
    AppState,
    services::{malware, media_processor::ProcessOptions},
};

#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct Preset {
    pub id: i64,
    pub name: String,
    pub slug: String,
    pub description: Option<String>,
    pub quality: Value,
    pub compression_mode: String,
    pub convert_to_webp: bool,
    pub convert_to_avif: bool,
    pub generate_retina: bool,
    pub responsive_sizes: Value,
    pub is_default: bool,
}

/// Queue the default preset in the same transaction that publishes an image.
/// A missing processor leaves uploads available; the deployment health check
/// exposes that optimization is disabled.
pub async fn enqueue_default_if_image(
    state: &AppState,
    tx: &mut Transaction<'_, Postgres>,
    media_id: i64,
    mime: &str,
    scan_status: &str,
    source_version: i64,
) -> std::result::Result<bool, sqlx::Error> {
    if state.services.media_processor.is_none()
        || scan_status == "infected"
        || !matches!(
            mime,
            "image/jpeg" | "image/png" | "image/gif" | "image/webp" | "image/avif"
        )
    {
        return Ok(false);
    }
    let preset: Value = sqlx::query_scalar(
        "SELECT to_jsonb(p) FROM media_optimization_presets p WHERE is_default=TRUE LIMIT 1",
    )
    .fetch_one(&mut **tx)
    .await?;
    sqlx::query(
        "INSERT INTO media_optimization_jobs(media_id,preset,source_version) VALUES($1,$2,$3)",
    )
    .bind(media_id)
    .bind(preset)
    .bind(source_version)
    .execute(&mut **tx)
    .await?;
    Ok(true)
}
impl Preset {
    pub fn validate(&self) -> Result<()> {
        if !matches!(
            self.compression_mode.as_str(),
            "lossy" | "lossless" | "auto"
        ) {
            bail!("Invalid compression mode");
        }
        let qualities = self
            .quality
            .as_object()
            .context("Preset quality must be an object")?;
        for format in ["webp", "avif", "jpeg", "png"] {
            if !qualities
                .get(format)
                .and_then(Value::as_u64)
                .is_some_and(|q| (1..=100).contains(&q))
            {
                bail!("Preset requires quality from 1 to 100 for each format");
            }
        }
        let sizes = self
            .responsive_sizes
            .as_object()
            .context("Responsive sizes must be an object")?;
        if sizes.len() > 3
            || sizes.iter().any(|(key, value)| {
                key.is_empty()
                    || key.len() > 32
                    || !value.as_u64().is_some_and(|v| (1..=4096).contains(&v))
            })
        {
            bail!("Preset supports at most three named widths from 1 to 4096");
        }
        if !self.convert_to_webp && !self.convert_to_avif {
            bail!("Choose at least one output format");
        }
        Ok(())
    }
}

#[derive(Debug, FromRow)]
struct Claim {
    id: i64,
    attempts: i32,
}

/// One atomic lease; expired final attempts become visible failures. Locked jobs
/// are skipped, including a long-running attempt holding its transaction lock.
pub async fn process_one(state: &AppState) -> Result<bool> {
    let Some(processor) = &state.services.media_processor else {
        return Ok(false);
    };
    let pool = state.db.pool();
    sqlx::query("UPDATE media_optimization_jobs SET status='failed', error='Worker lease expired after final attempt', completed_at=NOW(), lease_until=NULL WHERE status='processing' AND lease_until < NOW() AND attempts >= max_attempts AND id IN (SELECT id FROM media_optimization_jobs WHERE status='processing' AND lease_until < NOW() FOR UPDATE SKIP LOCKED)")
        .execute(pool).await?;
    let claim: Option<Claim> = sqlx::query_as("UPDATE media_optimization_jobs SET status='processing', attempts=attempts+1, started_at=NOW(), lease_until=NOW()+INTERVAL '30 minutes' WHERE id=(SELECT id FROM media_optimization_jobs WHERE attempts < max_attempts AND ((status='pending' AND available_at<=NOW()) OR (status='processing' AND lease_until<NOW())) ORDER BY priority DESC, available_at, id FOR UPDATE SKIP LOCKED LIMIT 1) RETURNING id, attempts")
        .fetch_optional(pool).await?;
    let Some(claim) = claim else { return Ok(false) };
    let mut tx = pool.begin().await?;
    let row: Option<(i64,Value,i64)> = sqlx::query_as("SELECT media_id,preset,source_version FROM media_optimization_jobs WHERE id=$1 AND status='processing' AND attempts=$2")
        .bind(claim.id).bind(claim.attempts).fetch_optional(&mut *tx).await?;
    let Some((media_id, preset, source_version)) = row else {
        return Ok(true);
    };
    // All content changes/deletions lock media before jobs. The outer locks
    // survive savepoint rollback so a scan result cannot race a replacement.
    let row: Option<(Option<String>, i64)> =
        sqlx::query_as("SELECT path,content_version FROM media WHERE id=$1 FOR UPDATE")
            .bind(media_id)
            .fetch_optional(&mut *tx)
            .await?;
    let Some((path, current_version)) = row else {
        return Ok(true);
    };
    let valid: Option<i64> = sqlx::query_scalar("SELECT id FROM media_optimization_jobs WHERE id=$1 AND status='processing' AND attempts=$2 FOR UPDATE")
        .bind(claim.id).bind(claim.attempts).fetch_optional(&mut *tx).await?;
    if valid.is_none() {
        return Ok(true);
    }
    if current_version != source_version {
        sqlx::query("UPDATE media_optimization_jobs SET status='cancelled',error='Original changed before processing',lease_until=NULL,completed_at=NOW() WHERE id=$1").bind(claim.id).execute(&mut *tx).await?;
        tx.commit().await?;
        return Ok(true);
    }
    sqlx::query("SAVEPOINT image_attempt")
        .execute(&mut *tx)
        .await?;
    let started = Instant::now();
    let mut uploaded = Vec::new();
    let mut observed_scan = None;
    let mut terminal = false;
    let result = async {
        let preset: Preset = serde_json::from_value(preset)?;
        preset.validate()?;
        let input = state.services.storage.download_for_scan(path.as_deref().context("Media has no stored original")?).await?;
        let address = std::env::var("CLAMD_ADDRESS").ok();
        let verdict = malware::scan(address.as_deref(), &input).await;
        observed_scan = Some((media_id, verdict.status.clone(), serde_json::to_value(&verdict)?));
        terminal = verdict.status == "infected";
        if !verdict.is_clean || verdict.status != "clean" {
            bail!("Original image scan is {}: {}", verdict.status, verdict.error.unwrap_or_else(|| verdict.threat_name.unwrap_or_default()));
        }
        let input = Bytes::from(input);
        let original_size = input.len() as i64;
        let info = processor.inspect(input.clone()).await?;
        let mut outputs = Vec::new();
        for format in ["webp", "avif"] {
            if (format == "webp" && !preset.convert_to_webp) || (format == "avif" && !preset.convert_to_avif) { continue; }
            if info.animated && format == "avif" { bail!("Animated originals require a WebP-only preset to preserve frames"); }
            let quality = preset.quality[format].as_u64().context("Invalid preset quality")? as u8;
            outputs.push((format.to_owned(), "full".to_owned(), None, false, quality));
            for (name, width) in preset.responsive_sizes.as_object().context("Invalid responsive sizes")? {
                let width = width.as_u64().context("Invalid responsive width")? as u32;
                if width < info.width { outputs.push((format.to_owned(), name.clone(), Some(width), false, quality)); }
                if preset.generate_retina && width.saturating_mul(2) < info.width {
                    outputs.push((format.to_owned(), name.clone(), Some(width * 2), true, quality));
                }
            }
        }
        let mut primary_size = None;
        for (format, name, width, retina, quality) in outputs {
            let options = ProcessOptions { format: format.clone(), width, height: None, quality, lossless: preset.compression_mode == "lossless" };
            let output = processor.process(input.clone(), &options).await?;
            if output.info.size >= original_size as u64 {
                if name == "full" {
                    primary_size = Some(primary_size.unwrap_or(original_size).min(original_size));
                }
                continue;
            }
            let key = format!("media/variants/{media_id}/{}/{}.{}", claim.id, uuid::Uuid::new_v4(), format);
            uploaded.push(key.clone());
            let url = state.services.storage.upload_at_key(&key, output.bytes.to_vec(), &output.content_type).await?;
            if name == "full" { primary_size = Some(primary_size.unwrap_or(original_size).min(output.info.size as i64)); }
            let savings = 100.0 * (1.0 - output.info.size as f64 / original_size as f64);
            sqlx::query("INSERT INTO media_variants(media_id,job_id,type,size,width,height,pages,path,url,format,file_size,is_retina,savings_percent,source_version) VALUES($1,$2,$3,$4,$5,$6,$7,$8,$9,$10,$11,$12,$13,$14)")
                .bind(media_id).bind(claim.id).bind(if name == "full" { "optimized" } else { "responsive" }).bind(&name)
                .bind(output.info.width as i32).bind(output.info.height as i32).bind(output.info.pages as i32)
                .bind(key).bind(url).bind(format).bind(output.info.size as i64).bind(retina).bind(savings).bind(source_version).execute(&mut *tx).await?;
        }
        sqlx::query("UPDATE media SET is_optimized=TRUE,width=$2,height=$3,scan_status='clean',scan_result=$4,size=$5,updated_at=NOW() WHERE id=$1")
            .bind(media_id).bind(info.width as i32).bind(info.height as i32).bind(serde_json::to_value(verdict)?).bind(original_size).execute(&mut *tx).await?;
        sqlx::query("UPDATE media_optimization_jobs SET status='completed', original_size=$2, optimized_size=$3, processing_time_ms=$4,error=NULL,lease_until=NULL,completed_at=NOW() WHERE id=$1")
            .bind(claim.id).bind(original_size).bind(primary_size.context("Preset produced no image")?).bind(started.elapsed().as_millis() as i64).execute(&mut *tx).await?;
        Ok::<(),anyhow::Error>(())
    }.await;
    if let Err(error) = result {
        // Undo variant rows and success flags, retaining the original row lock.
        sqlx::query("ROLLBACK TO SAVEPOINT image_attempt")
            .execute(&mut *tx)
            .await?;
        let mut message = error.to_string();
        for key in uploaded {
            if let Err(cleanup) = state.services.storage.delete(&key).await {
                tracing::error!(%key,%cleanup,"Failed to clean an unpublished image variant");
            }
        }
        if let Some((_, status, verdict)) = observed_scan {
            sqlx::query(
                "UPDATE media SET scan_status=$2,scan_result=$3,updated_at=NOW() WHERE id=$1",
            )
            .bind(media_id)
            .bind(&status)
            .bind(&verdict)
            .execute(&mut *tx)
            .await?;
            if status == "infected" {
                sqlx::query("UPDATE media SET url=NULL,is_optimized=FALSE WHERE id=$1")
                    .bind(media_id)
                    .execute(&mut *tx)
                    .await?;
                sqlx::query("UPDATE media_versions SET snapshot=snapshot || jsonb_build_object('scan_status','infected','scan_result',$3::jsonb,'url',NULL) WHERE media_id=$1 AND version_number=$2").bind(media_id).bind(source_version).bind(verdict).execute(&mut *tx).await?;
                let quarantine = async {
                    if let Some(path) = &path {
                        state.services.storage.delete(path).await?;
                    }
                    let paths: Vec<String> = sqlx::query_scalar(
                        "SELECT path FROM media_variants WHERE media_id=$1 AND source_version=$2",
                    )
                    .bind(media_id)
                    .bind(source_version)
                    .fetch_all(&mut *tx)
                    .await?;
                    for path in paths {
                        state.services.storage.delete(&path).await?;
                    }
                    sqlx::query(
                        "DELETE FROM media_variants WHERE media_id=$1 AND source_version=$2",
                    )
                    .bind(media_id)
                    .bind(source_version)
                    .execute(&mut *tx)
                    .await?;
                    Ok::<(), anyhow::Error>(())
                }
                .await;
                if let Err(cleanup) = quarantine {
                    terminal = false;
                    message.push_str(&format!("; quarantine failed: {cleanup}"));
                }
            }
        }
        tracing::warn!(job_id=claim.id,%message,"Media processing attempt failed");
        let message: String = message.chars().take(1000).collect();
        sqlx::query("UPDATE media_optimization_jobs SET status=CASE WHEN attempts>=max_attempts OR $4 THEN 'failed' ELSE 'pending' END,error=$3,lease_until=NULL,available_at=NOW()+make_interval(secs=>LEAST(300,5*power(2,attempts)::int)),completed_at=CASE WHEN attempts>=max_attempts OR $4 THEN NOW() ELSE NULL END WHERE id=$1 AND attempts=$2 AND status='processing'")
            .bind(claim.id).bind(claim.attempts).bind(message).bind(terminal).execute(&mut *tx).await?;
    }
    tx.commit().await?;
    Ok(true)
}

pub async fn run(state: AppState) {
    let mut interval = tokio::time::interval(Duration::from_secs(2));
    loop {
        interval.tick().await;
        if let Err(error) = cleanup_upload(&state).await {
            tracing::error!(%error,"Pending upload cleanup failed");
        }
        if let Err(error) = process_one(&state).await {
            tracing::error!(%error, "Media job worker failed");
        }
    }
}

/// Pending presigned keys never become immutable published originals. Retried
/// cleanup also removes abandoned expired uploads without touching final keys.
pub async fn cleanup_upload(state: &AppState) -> Result<bool> {
    let mut tx = state.db.pool().begin().await?;
    let ticket: Option<(String,bool)> = sqlx::query_as("SELECT file_key,expires_at<NOW() FROM media_upload_tickets WHERE (cleanup_pending OR expires_at<NOW()) AND cleanup_after<=NOW() ORDER BY cleanup_after FOR UPDATE SKIP LOCKED LIMIT 1")
        .fetch_optional(&mut *tx).await?;
    let Some((key, expired)) = ticket else {
        return Ok(false);
    };
    match state.services.storage.delete(&key).await {
        Ok(()) if expired => {
            sqlx::query("DELETE FROM media_upload_tickets WHERE file_key=$1")
                .bind(&key)
                .execute(&mut *tx)
                .await?;
        }
        Ok(()) => {
            sqlx::query("UPDATE media_upload_tickets SET cleanup_pending=FALSE,cleanup_error=NULL WHERE file_key=$1").bind(&key).execute(&mut *tx).await?;
        }
        Err(error) => {
            let message: String = error.to_string().chars().take(1000).collect();
            sqlx::query("UPDATE media_upload_tickets SET cleanup_error=$2,cleanup_after=NOW()+INTERVAL '1 minute' WHERE file_key=$1").bind(&key).bind(message).execute(&mut *tx).await?;
            tracing::warn!(%key,%error,"Upload cleanup retained for retry");
        }
    }
    tx.commit().await?;
    Ok(true)
}

/// Delete every derived object before deleting its rows. Retry is idempotent;
/// storage failure leaves the rows available for another deletion attempt.
pub async fn delete_variants(
    state: &AppState,
    tx: &mut Transaction<'_, Postgres>,
    media_id: i64,
) -> Result<()> {
    let paths: Vec<String> =
        sqlx::query_scalar("SELECT path FROM media_variants WHERE media_id=$1")
            .bind(media_id)
            .fetch_all(&mut **tx)
            .await?;
    for path in paths {
        state.services.storage.delete(&path).await?;
    }
    Ok(())
}

pub async fn quarantine(state: &AppState, media_id: i64) -> Result<()> {
    sqlx::query(
        "UPDATE media SET url=NULL,is_optimized=FALSE WHERE id=$1 AND scan_status='infected'",
    )
    .bind(media_id)
    .execute(state.db.pool())
    .await?;
    let mut tx = state.db.pool().begin().await?;
    let row:Option<(Option<String>,i64,Option<Value>)>=sqlx::query_as("SELECT path,content_version,scan_result FROM media WHERE id=$1 AND scan_status='infected' FOR UPDATE").bind(media_id).fetch_optional(&mut *tx).await?;
    let Some((path, version, verdict)) = row else {
        return Ok(());
    };
    sqlx::query("UPDATE media_versions SET snapshot=snapshot || jsonb_build_object('url',NULL,'scan_status','infected','scan_result',$3::jsonb) WHERE media_id=$1 AND version_number=$2").bind(media_id).bind(version).bind(verdict).execute(&mut *tx).await?;
    if let Some(path) = path {
        state.services.storage.delete(&path).await?;
    }
    let paths: Vec<String> = sqlx::query_scalar(
        "SELECT path FROM media_variants WHERE media_id=$1 AND source_version=$2",
    )
    .bind(media_id)
    .bind(version)
    .fetch_all(&mut *tx)
    .await?;
    for path in paths {
        state.services.storage.delete(&path).await?;
    }
    sqlx::query("DELETE FROM media_variants WHERE media_id=$1 AND source_version=$2")
        .bind(media_id)
        .bind(version)
        .execute(&mut *tx)
        .await?;
    tx.commit().await?;
    Ok(())
}
