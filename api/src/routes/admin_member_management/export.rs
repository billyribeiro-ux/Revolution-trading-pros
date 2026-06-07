//! Member export: CSV (default), XLSX, PDF.
//! Includes minimal XML/PDF generation helpers (production would use real libraries).

use axum::{
    extract::{Query, State},
    http::{header, StatusCode},
    response::IntoResponse,
    Json,
};
use chrono::{NaiveDateTime, Utc};
use serde_json::json;

use crate::middleware::admin::AdminUser;
use crate::AppState;

use super::ExportQuery;

/// GET /admin/member-management/export - Export members
/// ICT 7 SECURITY: Uses AdminUser extractor for automatic authorization
/// ICT 7 SECURITY: Parameterized queries prevent SQL injection
pub(super) async fn export_members(
    State(state): State<AppState>,
    admin: AdminUser,
    Query(query): Query<ExportQuery>,
) -> Result<impl IntoResponse, (StatusCode, Json<serde_json::Value>)> {
    let _ = &admin; // Admin authorization handled by extractor

    let format = query.format.as_deref().unwrap_or("csv");

    // ICT 7 SECURITY FIX: Parse dates safely and use parameterized queries
    let date_from: Option<chrono::NaiveDate> = query.date_from.as_ref().and_then(|d| {
        if is_valid_date(d) {
            chrono::NaiveDate::parse_from_str(d, "%Y-%m-%d").ok()
        } else {
            None
        }
    });

    let date_to: Option<chrono::NaiveDate> = query.date_to.as_ref().and_then(|d| {
        if is_valid_date(d) {
            chrono::NaiveDate::parse_from_str(d, "%Y-%m-%d").ok()
        } else {
            None
        }
    });

    // ICT 7 SECURITY FIX: Use parameterized query with optional filters
    // Status filter: 'active', 'trial', 'churned', or NULL for all
    let status_filter = query.status.as_deref();

    // Fetch members with parameterized query
    #[allow(clippy::type_complexity)]
    let members: Vec<(i64, Option<String>, String, Option<String>, NaiveDateTime)> =
        sqlx::query_as(
            r"
            SELECT u.id, u.name, u.email, u.role, u.created_at
            FROM users u
            WHERE ($1::text IS NULL OR (
                CASE $1
                    WHEN 'active' THEN u.id IN (SELECT DISTINCT user_id FROM user_memberships WHERE status = 'active')
                    WHEN 'trial' THEN u.id IN (SELECT DISTINCT user_id FROM user_memberships WHERE status = 'trial')
                    WHEN 'churned' THEN u.id IN (SELECT DISTINCT user_id FROM user_memberships WHERE status IN ('cancelled', 'expired'))
                        AND u.id NOT IN (SELECT DISTINCT user_id FROM user_memberships WHERE status = 'active')
                    ELSE TRUE
                END
            ))
            AND ($2::date IS NULL OR u.created_at >= $2::date)
            AND ($3::date IS NULL OR u.created_at <= $3::date + INTERVAL '1 day')
            ORDER BY u.created_at DESC
            LIMIT 10000
            ",
        )
        .bind(status_filter)
        .bind(date_from)
        .bind(date_to)
        .fetch_all(&state.db.pool)
        .await
        .map_err(|e| {
            tracing::error!("Export members query failed: {}", e);
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(json!({"error": "Failed to export members"})),
            )
        })?;

    match format {
        "xlsx" => {
            // Generate Excel file
            let xlsx_content = generate_xlsx(&members);
            Ok((
                [
                    (
                        header::CONTENT_TYPE,
                        "application/vnd.openxmlformats-officedocument.spreadsheetml.sheet",
                    ),
                    (
                        header::CONTENT_DISPOSITION,
                        "attachment; filename=\"members.xlsx\"",
                    ),
                ],
                xlsx_content,
            )
                .into_response())
        }
        "pdf" => {
            // Generate PDF (simplified - in production use a proper PDF library)
            let pdf_content = generate_pdf(&members);
            Ok((
                [
                    (header::CONTENT_TYPE, "application/pdf"),
                    (
                        header::CONTENT_DISPOSITION,
                        "attachment; filename=\"members.pdf\"",
                    ),
                ],
                pdf_content,
            )
                .into_response())
        }
        _ => {
            // CSV (default)
            let mut csv = String::from("ID,Name,Email,Role,Created At\n");
            for (id, name, email, role, created_at) in &members {
                csv.push_str(&format!(
                    "{},{},{},{},{}\n",
                    id,
                    name.as_deref().unwrap_or("").replace(',', ";"),
                    email.replace(',', ";"),
                    role.as_deref().unwrap_or("user"),
                    created_at
                ));
            }
            Ok((
                [
                    (header::CONTENT_TYPE, "text/csv"),
                    (
                        header::CONTENT_DISPOSITION,
                        "attachment; filename=\"members.csv\"",
                    ),
                ],
                csv,
            )
                .into_response())
        }
    }
}

fn is_valid_date(date: &str) -> bool {
    let parts: Vec<&str> = date.split('-').collect();
    parts.len() == 3
        && parts[0].len() == 4
        && parts[1].len() == 2
        && parts[2].len() == 2
        && parts.iter().all(|p| p.chars().all(|c| c.is_ascii_digit()))
}

#[allow(clippy::type_complexity)]
fn generate_xlsx(
    members: &[(i64, Option<String>, String, Option<String>, NaiveDateTime)],
) -> Vec<u8> {
    // Simple XLSX generation using xlsx-writer pattern
    // In production, use the `rust_xlsxwriter` or `calamine` crate
    // For now, return a simple XML-based spreadsheet
    let mut content = String::from(
        r#"<?xml version="1.0" encoding="UTF-8"?>
<Workbook xmlns="urn:schemas-microsoft-com:office:spreadsheet"
 xmlns:ss="urn:schemas-microsoft-com:office:spreadsheet">
 <Worksheet ss:Name="Members">
  <Table>
   <Row>
    <Cell><Data ss:Type="String">ID</Data></Cell>
    <Cell><Data ss:Type="String">Name</Data></Cell>
    <Cell><Data ss:Type="String">Email</Data></Cell>
    <Cell><Data ss:Type="String">Role</Data></Cell>
    <Cell><Data ss:Type="String">Created At</Data></Cell>
   </Row>
"#,
    );

    for (id, name, email, role, created_at) in members {
        content.push_str(&format!(
            r#"   <Row>
    <Cell><Data ss:Type="Number">{}</Data></Cell>
    <Cell><Data ss:Type="String">{}</Data></Cell>
    <Cell><Data ss:Type="String">{}</Data></Cell>
    <Cell><Data ss:Type="String">{}</Data></Cell>
    <Cell><Data ss:Type="String">{}</Data></Cell>
   </Row>
"#,
            id,
            escape_xml(name.as_deref().unwrap_or("")),
            escape_xml(email),
            escape_xml(role.as_deref().unwrap_or("user")),
            created_at
        ));
    }

    content.push_str(
        r"  </Table>
 </Worksheet>
</Workbook>",
    );

    content.into_bytes()
}

#[allow(clippy::type_complexity)]
fn generate_pdf(
    members: &[(i64, Option<String>, String, Option<String>, NaiveDateTime)],
) -> Vec<u8> {
    // Simple PDF generation
    // In production, use a proper PDF library like `printpdf` or `genpdf`
    // For now, generate a text-based PDF structure
    let mut lines = vec![
        "Revolution Trading Pros - Members Export".to_string(),
        format!("Generated: {}", Utc::now().format("%Y-%m-%d %H:%M:%S")),
        format!("Total Members: {}", members.len()),
        String::new(),
        "ID | Name | Email | Role | Created At".to_string(),
        "-".repeat(80),
    ];

    for (id, name, email, role, created_at) in members.iter().take(1000) {
        lines.push(format!(
            "{} | {} | {} | {} | {}",
            id,
            name.as_deref().unwrap_or("N/A"),
            email,
            role.as_deref().unwrap_or("user"),
            created_at.format("%Y-%m-%d")
        ));
    }

    // Create a minimal valid PDF
    let content = lines.join("\n");
    let pdf = format!(
        r"%PDF-1.4
1 0 obj
<< /Type /Catalog /Pages 2 0 R >>
endobj
2 0 obj
<< /Type /Pages /Kids [3 0 R] /Count 1 >>
endobj
3 0 obj
<< /Type /Page /Parent 2 0 R /MediaBox [0 0 612 792] /Contents 4 0 R /Resources << /Font << /F1 5 0 R >> >> >>
endobj
4 0 obj
<< /Length {} >>
stream
BT
/F1 10 Tf
50 750 Td
12 TL
{}
ET
endstream
endobj
5 0 obj
<< /Type /Font /Subtype /Type1 /BaseFont /Courier >>
endobj
xref
0 6
0000000000 65535 f
0000000009 00000 n
0000000058 00000 n
0000000115 00000 n
0000000268 00000 n
trailer
<< /Size 6 /Root 1 0 R >>
startxref
{}
%%EOF",
        content.len() + 50,
        content
            .lines()
            .map(|l| format!("({}) Tj T*", escape_pdf(l)))
            .collect::<Vec<_>>()
            .join("\n"),
        500 + content.len()
    );

    pdf.into_bytes()
}

fn escape_xml(s: &str) -> String {
    s.replace('&', "&amp;")
        .replace('<', "&lt;")
        .replace('>', "&gt;")
        .replace('"', "&quot;")
        .replace('\'', "&apos;")
}

fn escape_pdf(s: &str) -> String {
    s.replace('\\', "\\\\")
        .replace('(', "\\(")
        .replace(')', "\\)")
}
