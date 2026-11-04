//! Validate large provider arrays without buffering their entries in application memory.
use anyhow::{Result, anyhow};
use serde::{Deserialize, de::DeserializeOwned};
use std::io::{BufReader, Cursor, Read};
use tokio::sync::mpsc;

struct ChunkReader {
    receiver: mpsc::Receiver<Vec<u8>>,
    current: Cursor<Vec<u8>>,
}
impl Read for ChunkReader {
    fn read(&mut self, output: &mut [u8]) -> std::io::Result<usize> {
        if output.is_empty() {
            return Ok(0);
        }
        loop {
            if self.current.position() < self.current.get_ref().len() as u64 {
                return self.current.read(output);
            }
            let Some(chunk) = self.receiver.blocking_recv() else {
                return Ok(0);
            };
            self.current = Cursor::new(chunk);
        }
    }
}

async fn stream<T: DeserializeOwned + Send + 'static>(response: reqwest::Response) -> Result<T> {
    stream_bounded(response, 64 * 1024 * 1024).await
}

async fn stream_bounded<T: DeserializeOwned + Send + 'static>(
    mut response: reqwest::Response,
    limit: usize,
) -> Result<T> {
    if response
        .content_length()
        .is_some_and(|length| length > limit as u64)
    {
        return Err(anyhow!("Provider response exceeds verification size limit"));
    }
    let (sender, receiver) = mpsc::channel(2);
    let parser = tokio::task::spawn_blocking(move || {
        let reader = BufReader::with_capacity(
            16_384,
            ChunkReader {
                receiver,
                current: Cursor::new(Vec::new()),
            },
        );
        serde_json::from_reader::<_, T>(reader)
    });
    let mut received = 0usize;
    while let Some(chunk) = response
        .chunk()
        .await
        .map_err(|_| anyhow!("Provider response interrupted"))?
    {
        received = received
            .checked_add(chunk.len())
            .ok_or_else(|| anyhow!("Provider response exceeds verification size limit"))?;
        if received > limit {
            return Err(anyhow!("Provider response exceeds verification size limit"));
        }
        for part in chunk.chunks(16_384) {
            if sender.send(part.to_vec()).await.is_err() {
                return parser
                    .await
                    .map_err(|_| anyhow!("Provider response parser failed"))?
                    .map_err(|_| anyhow!("Invalid provider response"));
            }
        }
    }
    drop(sender);
    parser
        .await
        .map_err(|_| anyhow!("Provider response parser failed"))?
        .map_err(|_| anyhow!("Invalid provider response"))
}

// Markers are zero-sized, so a directory/model array retains no per-entry objects.
#[derive(Deserialize)]
#[serde(try_from = "DirectoryIdentity")]
struct DirectoryMarker;
#[derive(Deserialize)]
struct DirectoryIdentity {
    #[serde(rename = "ObjectName")]
    name: String,
    #[serde(rename = "IsDirectory")]
    directory: bool,
}
impl TryFrom<DirectoryIdentity> for DirectoryMarker {
    type Error = &'static str;
    fn try_from(value: DirectoryIdentity) -> std::result::Result<Self, Self::Error> {
        let _kind = value.directory;
        if value.name.is_empty() {
            Err("Missing directory object identity")
        } else {
            Ok(Self)
        }
    }
}
pub(super) async fn directory(response: reqwest::Response) -> Result<()> {
    let _: Vec<DirectoryMarker> = stream(response).await?;
    Ok(())
}
#[derive(Deserialize)]
#[serde(try_from = "ModelIdentity")]
struct ModelMarker;
#[derive(Deserialize)]
struct ModelIdentity {
    id: String,
    object: String,
}
impl TryFrom<ModelIdentity> for ModelMarker {
    type Error = &'static str;
    fn try_from(value: ModelIdentity) -> std::result::Result<Self, Self::Error> {
        if value.id.is_empty() || value.object != "model" {
            Err("Invalid model identity")
        } else {
            Ok(Self)
        }
    }
}
#[derive(Deserialize)]
struct Models {
    object: String,
    data: Vec<ModelMarker>,
}
pub(super) async fn models(response: reqwest::Response) -> Result<bool> {
    let models: Models = stream(response).await?;
    let _entry_count = models.data.len();
    Ok(models.object == "list")
}
#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
struct VideoIdentity {
    guid: String,
    video_library_id: i64,
}
struct VideoSummary {
    count: i64,
    library_id: Option<i64>,
    consistent: bool,
}
fn video_summary<'de, D: serde::Deserializer<'de>>(
    deserializer: D,
) -> std::result::Result<VideoSummary, D::Error> {
    struct Visitor;
    impl<'de> serde::de::Visitor<'de> for Visitor {
        type Value = VideoSummary;
        fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            formatter.write_str("a video array")
        }
        fn visit_seq<A: serde::de::SeqAccess<'de>>(
            self,
            mut sequence: A,
        ) -> std::result::Result<Self::Value, A::Error> {
            let mut summary = VideoSummary {
                count: 0,
                library_id: None,
                consistent: true,
            };
            while let Some(video) = sequence.next_element::<VideoIdentity>()? {
                if video.guid.is_empty() || video.video_library_id <= 0 {
                    return Err(serde::de::Error::custom("Invalid video identity"));
                }
                if let Some(library) = summary.library_id {
                    summary.consistent &= library == video.video_library_id;
                }
                summary.library_id = Some(video.video_library_id);
                summary.count = summary.count.saturating_add(1);
            }
            Ok(summary)
        }
    }
    deserializer.deserialize_seq(Visitor)
}
#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
struct Videos {
    #[serde(deserialize_with = "video_summary")]
    items: VideoSummary,
    total_items: i64,
    current_page: i64,
    items_per_page: i64,
}
pub(super) async fn videos(response: reqwest::Response, library_id: i64) -> Result<bool> {
    let videos: Videos = stream(response).await?;
    Ok(videos.total_items >= 0
        && videos.current_page >= 1
        && videos.items_per_page >= 1
        && videos.items.count <= videos.total_items
        && videos.items.consistent
        && videos
            .items
            .library_id
            .is_none_or(|value| value == library_id))
}

#[cfg(test)]
mod tests {
    use super::*;
    use axum::{Router, body::Body, http::Response, routing::get};

    #[tokio::test]
    async fn body_budget_applies_to_declared_and_chunked_responses() {
        let listener = tokio::net::TcpListener::bind("127.0.0.1:0").await.unwrap();
        let address = listener.local_addr().unwrap();
        let app = Router::new()
            .route("/valid", get(|| async { "[1,2]" }))
            .route("/length", get(|| async { "[1,2,3,4,5,6,7,8,9]" }))
            .route(
                "/chunks",
                get(|| async {
                    let chunks = futures::stream::iter([
                        Ok::<_, std::io::Error>("[1,2,3,4,"),
                        Ok("5,6,7,8,9]"),
                    ]);
                    Response::new(Body::from_stream(chunks))
                }),
            );
        let server = tokio::spawn(async move { axum::serve(listener, app).await.unwrap() });
        let client = reqwest::Client::builder()
            .no_proxy()
            .timeout(std::time::Duration::from_secs(2))
            .build()
            .unwrap();
        let valid = client
            .get(format!("http://{address}/valid"))
            .send()
            .await
            .unwrap();
        assert_eq!(stream_bounded::<Vec<i64>>(valid, 16).await.unwrap(), [1, 2]);
        for path in ["length", "chunks"] {
            let response = client
                .get(format!("http://{address}/{path}"))
                .send()
                .await
                .unwrap();
            if path == "chunks" {
                assert!(response.content_length().is_none());
            }
            let error = stream_bounded::<Vec<i64>>(response, 16).await.unwrap_err();
            assert!(
                error.to_string().contains("verification size limit"),
                "{error}"
            );
        }
        server.abort();
    }
}
