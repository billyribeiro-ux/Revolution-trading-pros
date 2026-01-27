//! CMS AI Assist Routes - Apple ICT 7+ Principal Engineer Grade
//! January 2026
//!
//! AI-powered content assistance using Claude API:
//! - Text improvement, expansion, and summarization
//! - Grammar and spelling fixes
//! - Tone adjustments
//! - SEO content generation (meta descriptions, alt text)
//! - FAQ generation
//! - Related topic suggestions
//!
//! Features:
//! - Rate limiting (10 requests/minute per user)
//! - SSE streaming for real-time responses
//! - Usage history tracking
//! - Comprehensive error handling

use axum::{
    extract::{Query, State},
    http::StatusCode,
    response::sse::{Event, KeepAlive, Sse},
    routing::{get, post},
    Json, Router,
};
use chrono::{DateTime, Utc};
use futures::stream::Stream;
use serde::{Deserialize, Serialize};
use serde_json::{json, Value as JsonValue};
use std::{convert::Infallible, time::Duration};
use uuid::Uuid;

use crate::{models::User, AppState};

// ═══════════════════════════════════════════════════════════════════════════════════════
// CONSTANTS
// ═══════════════════════════════════════════════════════════════════════════════════════

/// Claude model to use for AI assistance
const CLAUDE_MODEL: &str = "claude-sonnet-4-20250514";

/// Rate limit: 10 requests per minute per user
const RATE_LIMIT_MAX_REQUESTS: i64 = 10;
const RATE_LIMIT_WINDOW_SECONDS: u64 = 60;

/// API endpoint for Claude
const CLAUDE_API_URL: &str = "https://api.anthropic.com/v1/messages";

// ═══════════════════════════════════════════════════════════════════════════════════════
// TYPES AND ENUMS
// ═══════════════════════════════════════════════════════════════════════════════════════

/// AI actions supported by the CMS
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum AiAction {
    /// Improve overall writing quality
    Improve,
    /// Make content more concise (30-50% reduction)
    Shorten,
    /// Add more detail and depth
    Expand,
    /// Fix grammar, spelling, and punctuation
    FixGrammar,
    /// Rewrite in a specific tone
    ChangeTone,
    /// Create a 2-3 sentence summary
    Summarize,
    /// Generate 5 FAQs from content
    GenerateFaq,
    /// Generate SEO meta description (max 155 chars)
    GenerateMeta,
    /// Generate image alt text (max 125 chars)
    GenerateAlt,
    /// Suggest 5 related topics
    SuggestRelated,
}

impl std::fmt::Display for AiAction {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            AiAction::Improve => write!(f, "improve"),
            AiAction::Shorten => write!(f, "shorten"),
            AiAction::Expand => write!(f, "expand"),
            AiAction::FixGrammar => write!(f, "fix_grammar"),
            AiAction::ChangeTone => write!(f, "change_tone"),
            AiAction::Summarize => write!(f, "summarize"),
            AiAction::GenerateFaq => write!(f, "generate_faq"),
            AiAction::GenerateMeta => write!(f, "generate_meta"),
            AiAction::GenerateAlt => write!(f, "generate_alt"),
            AiAction::SuggestRelated => write!(f, "suggest_related"),
        }
    }
}

/// Tone options for change_tone action
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum ToneOption {
    Professional,
    Casual,
    Friendly,
    Authoritative,
    Educational,
}

impl std::fmt::Display for ToneOption {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ToneOption::Professional => write!(f, "professional"),
            ToneOption::Casual => write!(f, "casual"),
            ToneOption::Friendly => write!(f, "friendly"),
            ToneOption::Authoritative => write!(f, "authoritative"),
            ToneOption::Educational => write!(f, "educational"),
        }
    }
}

/// Request options
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct AiAssistOptions {
    /// Tone for change_tone action
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tone: Option<ToneOption>,
    /// Optional maximum length for output
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_length: Option<u32>,
}

/// Request body for AI assist endpoints
#[derive(Debug, Clone, Deserialize)]
pub struct AiAssistRequest {
    /// The AI action to perform
    pub action: AiAction,
    /// Content to process
    pub content: String,
    /// Optional configuration
    #[serde(default)]
    pub options: AiAssistOptions,
    /// Optional content ID for tracking
    #[serde(skip_serializing_if = "Option::is_none")]
    pub content_id: Option<Uuid>,
    /// Optional block ID for tracking
    #[serde(skip_serializing_if = "Option::is_none")]
    pub block_id: Option<String>,
}

/// Response from AI assist endpoint
#[derive(Debug, Clone, Serialize)]
pub struct AiAssistResponse {
    /// Whether the request was successful
    pub success: bool,
    /// The AI-generated result
    pub result: String,
    /// Action that was performed
    pub action: String,
    /// Metadata about the request
    pub metadata: AiAssistMetadata,
}

/// Metadata about an AI assist request
#[derive(Debug, Clone, Serialize)]
pub struct AiAssistMetadata {
    /// Model used
    pub model: String,
    /// Input token count (estimated)
    pub input_tokens: u32,
    /// Output token count (estimated)
    pub output_tokens: u32,
    /// Processing time in milliseconds
    pub processing_time_ms: u64,
    /// Content ID if provided
    #[serde(skip_serializing_if = "Option::is_none")]
    pub content_id: Option<Uuid>,
    /// Block ID if provided
    #[serde(skip_serializing_if = "Option::is_none")]
    pub block_id: Option<String>,
}

/// History record from database
#[derive(Debug, Clone, Serialize, sqlx::FromRow)]
pub struct AiAssistHistoryRecord {
    pub id: i64,
    pub user_id: i64,
    pub action: String,
    pub input_content: String,
    pub output_content: String,
    pub options: Option<JsonValue>,
    pub content_id: Option<Uuid>,
    pub block_id: Option<String>,
    pub model: String,
    pub input_tokens: i32,
    pub output_tokens: i32,
    pub processing_time_ms: i32,
    pub created_at: DateTime<Utc>,
}

/// Query parameters for history endpoint
#[derive(Debug, Deserialize)]
pub struct HistoryQuery {
    /// Limit number of results
    pub limit: Option<i64>,
    /// Offset for pagination
    pub offset: Option<i64>,
    /// Filter by action type
    pub action: Option<String>,
    /// Filter by content ID
    pub content_id: Option<Uuid>,
}

/// Paginated history response
#[derive(Debug, Serialize)]
pub struct HistoryResponse {
    pub data: Vec<AiAssistHistoryRecord>,
    pub meta: PaginationMeta,
}

#[derive(Debug, Serialize)]
pub struct PaginationMeta {
    pub total: i64,
    pub limit: i64,
    pub offset: i64,
}

// ═══════════════════════════════════════════════════════════════════════════════════════
// API RESULT TYPES
// ═══════════════════════════════════════════════════════════════════════════════════════

type ApiResult<T> = Result<Json<T>, (StatusCode, Json<JsonValue>)>;
type ApiResultEmpty = Result<Json<JsonValue>, (StatusCode, Json<JsonValue>)>;

fn api_error(status: StatusCode, message: &str) -> (StatusCode, Json<JsonValue>) {
    (status, Json(json!({ "error": message })))
}

fn api_error_with_details(
    status: StatusCode,
    message: &str,
    details: JsonValue,
) -> (StatusCode, Json<JsonValue>) {
    (
        status,
        Json(json!({
            "error": message,
            "details": details
        })),
    )
}

// ═══════════════════════════════════════════════════════════════════════════════════════
// AUTHORIZATION HELPERS
// ═══════════════════════════════════════════════════════════════════════════════════════

fn require_cms_editor(user: &User) -> Result<(), (StatusCode, Json<JsonValue>)> {
    let role = user.role.as_deref().unwrap_or("user");
    if matches!(
        role,
        "admin" | "super-admin" | "super_admin" | "editor" | "marketing" | "developer"
    ) {
        Ok(())
    } else {
        Err(api_error(
            StatusCode::FORBIDDEN,
            "Editor access required for AI assist",
        ))
    }
}

// ═══════════════════════════════════════════════════════════════════════════════════════
// RATE LIMITING
// ═══════════════════════════════════════════════════════════════════════════════════════

async fn check_rate_limit(
    state: &AppState,
    user_id: i64,
) -> Result<(), (StatusCode, Json<JsonValue>)> {
    if let Some(ref redis) = state.services.redis {
        let key = format!("ai_assist_rate_limit:{}", user_id);
        match redis
            .check_rate_limit(&key, RATE_LIMIT_MAX_REQUESTS, RATE_LIMIT_WINDOW_SECONDS)
            .await
        {
            Ok(allowed) if allowed => Ok(()),
            Ok(_) => {
                tracing::warn!(
                    target: "security",
                    event = "ai_assist_rate_limited",
                    user_id = %user_id,
                    "AI assist rate limit exceeded"
                );
                Err(api_error_with_details(
                    StatusCode::TOO_MANY_REQUESTS,
                    "Rate limit exceeded. Please wait before making more requests.",
                    json!({
                        "limit": RATE_LIMIT_MAX_REQUESTS,
                        "window_seconds": RATE_LIMIT_WINDOW_SECONDS,
                        "retry_after": RATE_LIMIT_WINDOW_SECONDS
                    }),
                ))
            }
            Err(e) => {
                tracing::error!(
                    target: "ai_assist",
                    error = %e,
                    "Failed to check rate limit"
                );
                // Allow request if rate limit check fails (fail open for availability)
                Ok(())
            }
        }
    } else {
        // No Redis available, allow request
        Ok(())
    }
}

// ═══════════════════════════════════════════════════════════════════════════════════════
// PROMPT GENERATION
// ═══════════════════════════════════════════════════════════════════════════════════════

fn generate_prompt(request: &AiAssistRequest) -> String {
    let max_length_instruction = request
        .options
        .max_length
        .map(|len| format!("\n\nKeep the output under {} characters.", len))
        .unwrap_or_default();

    match request.action {
        AiAction::Improve => format!(
            "You are a professional editor. Improve the following text to enhance clarity, \
            readability, and impact while preserving the original meaning and voice. \
            Fix any errors and improve flow. Return only the improved text without \
            any commentary or explanation.{}\n\nText to improve:\n{}",
            max_length_instruction, request.content
        ),
        AiAction::Shorten => format!(
            "You are a concise editor. Reduce the following text by 30-50% while preserving \
            all essential information and meaning. Remove redundancy and unnecessary words. \
            Return only the shortened text without any commentary.{}\n\nText to shorten:\n{}",
            max_length_instruction, request.content
        ),
        AiAction::Expand => format!(
            "You are a content writer. Expand the following text by adding more detail, \
            examples, and depth. Maintain the original tone and style. Return only the \
            expanded text without any commentary.{}\n\nText to expand:\n{}",
            max_length_instruction, request.content
        ),
        AiAction::FixGrammar => format!(
            "You are a proofreader. Fix all grammar, spelling, and punctuation errors in \
            the following text. Do not change the meaning, style, or tone. Return only \
            the corrected text without any commentary.\n\nText to correct:\n{}",
            request.content
        ),
        AiAction::ChangeTone => {
            let tone = request
                .options
                .tone
                .as_ref()
                .map(|t| t.to_string())
                .unwrap_or_else(|| "professional".to_string());
            format!(
                "You are a skilled writer. Rewrite the following text in a {} tone while \
                preserving the core message. Return only the rewritten text without \
                any commentary.{}\n\nText to rewrite:\n{}",
                tone, max_length_instruction, request.content
            )
        }
        AiAction::Summarize => format!(
            "You are an expert summarizer. Create a concise 2-3 sentence summary of the \
            following text that captures the key points. Return only the summary without \
            any introduction or commentary.\n\nText to summarize:\n{}",
            request.content
        ),
        AiAction::GenerateFaq => format!(
            "You are a content strategist. Based on the following content, generate exactly \
            5 frequently asked questions (FAQs) with clear, helpful answers. Format as:\n\
            Q1: [question]\nA1: [answer]\n\nQ2: [question]\nA2: [answer]\n\n... and so on.\n\
            Return only the FAQs without any introduction.\n\nContent:\n{}",
            request.content
        ),
        AiAction::GenerateMeta => format!(
            "You are an SEO expert. Create a compelling meta description for the following \
            content. The description must be exactly 150-155 characters, include the main \
            topic/keyword, and encourage clicks. Return only the meta description, nothing else.\n\n\
            Content:\n{}",
            request.content
        ),
        AiAction::GenerateAlt => format!(
            "You are an accessibility expert. Write descriptive alt text for an image based on \
            the following description or context. The alt text must be under 125 characters, \
            descriptive, and accessible. Return only the alt text, nothing else.\n\n\
            Image context:\n{}",
            request.content
        ),
        AiAction::SuggestRelated => format!(
            "You are a content strategist. Based on the following content, suggest exactly 5 \
            related topics that would interest the same audience. Return only a numbered list \
            of topics, one per line, without explanations.\n\nContent:\n{}",
            request.content
        ),
    }
}

// ═══════════════════════════════════════════════════════════════════════════════════════
// CLAUDE API INTEGRATION
// ═══════════════════════════════════════════════════════════════════════════════════════

/// Call Claude API and return the response
async fn call_claude_api(
    api_key: &str,
    prompt: &str,
    stream: bool,
) -> Result<ClaudeResponse, String> {
    let client = reqwest::Client::new();

    let request_body = json!({
        "model": CLAUDE_MODEL,
        "max_tokens": 4096,
        "stream": stream,
        "messages": [{
            "role": "user",
            "content": prompt
        }]
    });

    let response = client
        .post(CLAUDE_API_URL)
        .header("Content-Type", "application/json")
        .header("x-api-key", api_key)
        .header("anthropic-version", "2023-06-01")
        .json(&request_body)
        .send()
        .await
        .map_err(|e| format!("Failed to call Claude API: {}", e))?;

    if !response.status().is_success() {
        let status = response.status();
        let error_text = response
            .text()
            .await
            .unwrap_or_else(|_| "Unknown error".to_string());
        tracing::error!(
            target: "ai_assist",
            status = %status,
            error = %error_text,
            "Claude API error"
        );
        return Err(format!("Claude API error ({}): {}", status, error_text));
    }

    let response_data: ClaudeApiResponse = response
        .json()
        .await
        .map_err(|e| format!("Failed to parse Claude response: {}", e))?;

    // Extract text from response
    let text = response_data
        .content
        .first()
        .map(|c| c.text.clone())
        .unwrap_or_default();

    Ok(ClaudeResponse {
        text,
        input_tokens: response_data.usage.input_tokens,
        output_tokens: response_data.usage.output_tokens,
    })
}

#[derive(Debug, Deserialize)]
struct ClaudeApiResponse {
    content: Vec<ClaudeContentBlock>,
    usage: ClaudeUsage,
}

#[derive(Debug, Deserialize)]
struct ClaudeContentBlock {
    text: String,
}

#[derive(Debug, Deserialize)]
struct ClaudeUsage {
    input_tokens: u32,
    output_tokens: u32,
}

struct ClaudeResponse {
    text: String,
    input_tokens: u32,
    output_tokens: u32,
}

// ═══════════════════════════════════════════════════════════════════════════════════════
// DATABASE OPERATIONS
// ═══════════════════════════════════════════════════════════════════════════════════════

async fn save_history(
    pool: &sqlx::PgPool,
    user_id: i64,
    request: &AiAssistRequest,
    result: &str,
    model: &str,
    input_tokens: u32,
    output_tokens: u32,
    processing_time_ms: u64,
) -> Result<i64, sqlx::Error> {
    let options_json = serde_json::to_value(&request.options).ok();

    let record: (i64,) = sqlx::query_as(
        r#"
        INSERT INTO cms_ai_assist_history (
            user_id, action, input_content, output_content, options,
            content_id, block_id, model, input_tokens, output_tokens,
            processing_time_ms, created_at
        ) VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11, NOW())
        RETURNING id
        "#,
    )
    .bind(user_id)
    .bind(request.action.to_string())
    .bind(&request.content)
    .bind(result)
    .bind(&options_json)
    .bind(request.content_id)
    .bind(&request.block_id)
    .bind(model)
    .bind(input_tokens as i32)
    .bind(output_tokens as i32)
    .bind(processing_time_ms as i32)
    .fetch_one(pool)
    .await?;

    Ok(record.0)
}

// ═══════════════════════════════════════════════════════════════════════════════════════
// ENDPOINT HANDLERS
// ═══════════════════════════════════════════════════════════════════════════════════════

/// POST /api/cms/ai/assist
/// Run AI action on text
async fn ai_assist(
    State(state): State<AppState>,
    user: User,
    Json(request): Json<AiAssistRequest>,
) -> ApiResult<AiAssistResponse> {
    require_cms_editor(&user)?;
    check_rate_limit(&state, user.id).await?;

    // Validate request
    if request.content.trim().is_empty() {
        return Err(api_error(
            StatusCode::BAD_REQUEST,
            "Content cannot be empty",
        ));
    }

    if request.content.len() > 100_000 {
        return Err(api_error(
            StatusCode::BAD_REQUEST,
            "Content exceeds maximum length of 100,000 characters",
        ));
    }

    // Validate tone is provided for change_tone action
    if request.action == AiAction::ChangeTone && request.options.tone.is_none() {
        return Err(api_error(
            StatusCode::BAD_REQUEST,
            "Tone option is required for change_tone action",
        ));
    }

    // Get API key from environment
    let api_key = std::env::var("ANTHROPIC_API_KEY").map_err(|_| {
        tracing::error!(target: "ai_assist", "ANTHROPIC_API_KEY not configured");
        api_error(
            StatusCode::SERVICE_UNAVAILABLE,
            "AI service is not configured",
        )
    })?;

    // Generate prompt and call API
    let prompt = generate_prompt(&request);
    let start_time = std::time::Instant::now();

    let claude_response = call_claude_api(&api_key, &prompt, false)
        .await
        .map_err(|e| {
            tracing::error!(target: "ai_assist", error = %e, "AI processing failed");
            api_error(
                StatusCode::INTERNAL_SERVER_ERROR,
                "AI processing failed. Please try again.",
            )
        })?;

    let processing_time_ms = start_time.elapsed().as_millis() as u64;

    // Save to history
    if let Err(e) = save_history(
        &state.db.pool,
        user.id,
        &request,
        &claude_response.text,
        CLAUDE_MODEL,
        claude_response.input_tokens,
        claude_response.output_tokens,
        processing_time_ms,
    )
    .await
    {
        tracing::warn!(
            target: "ai_assist",
            error = %e,
            "Failed to save AI assist history"
        );
        // Don't fail the request, just log
    }

    tracing::info!(
        target: "ai_assist",
        user_id = %user.id,
        action = %request.action,
        input_tokens = %claude_response.input_tokens,
        output_tokens = %claude_response.output_tokens,
        processing_time_ms = %processing_time_ms,
        "AI assist completed"
    );

    Ok(Json(AiAssistResponse {
        success: true,
        result: claude_response.text,
        action: request.action.to_string(),
        metadata: AiAssistMetadata {
            model: CLAUDE_MODEL.to_string(),
            input_tokens: claude_response.input_tokens,
            output_tokens: claude_response.output_tokens,
            processing_time_ms,
            content_id: request.content_id,
            block_id: request.block_id,
        },
    }))
}

/// POST /api/cms/ai/assist/stream
/// Stream AI response using Server-Sent Events
async fn ai_assist_stream(
    State(state): State<AppState>,
    user: User,
    Json(request): Json<AiAssistRequest>,
) -> Result<Sse<impl Stream<Item = Result<Event, Infallible>>>, (StatusCode, Json<JsonValue>)> {
    require_cms_editor(&user)?;
    check_rate_limit(&state, user.id).await?;

    // Validate request
    if request.content.trim().is_empty() {
        return Err(api_error(
            StatusCode::BAD_REQUEST,
            "Content cannot be empty",
        ));
    }

    if request.content.len() > 100_000 {
        return Err(api_error(
            StatusCode::BAD_REQUEST,
            "Content exceeds maximum length of 100,000 characters",
        ));
    }

    // Validate tone is provided for change_tone action
    if request.action == AiAction::ChangeTone && request.options.tone.is_none() {
        return Err(api_error(
            StatusCode::BAD_REQUEST,
            "Tone option is required for change_tone action",
        ));
    }

    // Get API key from environment
    let api_key = std::env::var("ANTHROPIC_API_KEY").map_err(|_| {
        tracing::error!(target: "ai_assist", "ANTHROPIC_API_KEY not configured");
        api_error(
            StatusCode::SERVICE_UNAVAILABLE,
            "AI service is not configured",
        )
    })?;

    let prompt = generate_prompt(&request);
    let action = request.action.to_string();
    let user_id = user.id;
    let pool = state.db.pool.clone();
    let content_id = request.content_id;
    let block_id = request.block_id.clone();
    let input_content = request.content.clone();
    let options = request.options.clone();

    let stream = async_stream::stream! {
        // Send initial event
        yield Ok(Event::default()
            .event("start")
            .data(json!({
                "action": action,
                "model": CLAUDE_MODEL
            }).to_string()));

        let start_time = std::time::Instant::now();

        // Call Claude API with streaming
        let client = reqwest::Client::new();
        let request_body = json!({
            "model": CLAUDE_MODEL,
            "max_tokens": 4096,
            "stream": true,
            "messages": [{
                "role": "user",
                "content": prompt
            }]
        });

        let response = match client
            .post(CLAUDE_API_URL)
            .header("Content-Type", "application/json")
            .header("x-api-key", &api_key)
            .header("anthropic-version", "2023-06-01")
            .json(&request_body)
            .send()
            .await
        {
            Ok(r) => r,
            Err(e) => {
                yield Ok(Event::default()
                    .event("error")
                    .data(json!({ "error": format!("API request failed: {}", e) }).to_string()));
                return;
            }
        };

        if !response.status().is_success() {
            let status = response.status();
            let error_text = response.text().await.unwrap_or_else(|_| "Unknown error".to_string());
            yield Ok(Event::default()
                .event("error")
                .data(json!({ "error": format!("API error ({}): {}", status, error_text) }).to_string()));
            return;
        }

        // Process streaming response
        let mut full_text = String::new();
        let mut input_tokens: u32 = 0;
        let mut output_tokens: u32 = 0;
        let mut bytes_stream = response.bytes_stream();

        use futures::StreamExt;

        while let Some(chunk) = bytes_stream.next().await {
            match chunk {
                Ok(bytes) => {
                    let text = String::from_utf8_lossy(&bytes);
                    // Parse SSE format from Claude
                    for line in text.lines() {
                        if line.starts_with("data: ") {
                            let data = &line[6..];
                            if data == "[DONE]" {
                                continue;
                            }

                            if let Ok(event) = serde_json::from_str::<JsonValue>(data) {
                                // Handle content_block_delta events
                                if event.get("type").and_then(|t| t.as_str()) == Some("content_block_delta") {
                                    if let Some(delta) = event.get("delta") {
                                        if let Some(text_delta) = delta.get("text").and_then(|t| t.as_str()) {
                                            full_text.push_str(text_delta);
                                            yield Ok(Event::default()
                                                .event("chunk")
                                                .data(json!({ "text": text_delta }).to_string()));
                                        }
                                    }
                                }
                                // Handle message_delta for usage
                                else if event.get("type").and_then(|t| t.as_str()) == Some("message_delta") {
                                    if let Some(usage) = event.get("usage") {
                                        output_tokens = usage.get("output_tokens").and_then(|t| t.as_u64()).unwrap_or(0) as u32;
                                    }
                                }
                                // Handle message_start for input tokens
                                else if event.get("type").and_then(|t| t.as_str()) == Some("message_start") {
                                    if let Some(message) = event.get("message") {
                                        if let Some(usage) = message.get("usage") {
                                            input_tokens = usage.get("input_tokens").and_then(|t| t.as_u64()).unwrap_or(0) as u32;
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
                Err(e) => {
                    yield Ok(Event::default()
                        .event("error")
                        .data(json!({ "error": format!("Stream error: {}", e) }).to_string()));
                    break;
                }
            }
        }

        let processing_time_ms = start_time.elapsed().as_millis() as u64;

        // Save to history
        let options_json = serde_json::to_value(&options).ok();
        let _ = sqlx::query(
            r#"
            INSERT INTO cms_ai_assist_history (
                user_id, action, input_content, output_content, options,
                content_id, block_id, model, input_tokens, output_tokens,
                processing_time_ms, created_at
            ) VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11, NOW())
            "#,
        )
        .bind(user_id)
        .bind(&action)
        .bind(&input_content)
        .bind(&full_text)
        .bind(&options_json)
        .bind(content_id)
        .bind(&block_id)
        .bind(CLAUDE_MODEL)
        .bind(input_tokens as i32)
        .bind(output_tokens as i32)
        .bind(processing_time_ms as i32)
        .execute(&pool)
        .await;

        tracing::info!(
            target: "ai_assist",
            user_id = %user_id,
            action = %action,
            input_tokens = %input_tokens,
            output_tokens = %output_tokens,
            processing_time_ms = %processing_time_ms,
            "AI assist stream completed"
        );

        // Send completion event
        yield Ok(Event::default()
            .event("done")
            .data(json!({
                "success": true,
                "metadata": {
                    "model": CLAUDE_MODEL,
                    "input_tokens": input_tokens,
                    "output_tokens": output_tokens,
                    "processing_time_ms": processing_time_ms
                }
            }).to_string()));
    };

    Ok(Sse::new(stream).keep_alive(
        KeepAlive::new()
            .interval(Duration::from_secs(15))
            .text("heartbeat"),
    ))
}

/// GET /api/cms/ai/history
/// Get user's AI usage history
async fn get_ai_history(
    State(state): State<AppState>,
    user: User,
    Query(query): Query<HistoryQuery>,
) -> ApiResult<HistoryResponse> {
    require_cms_editor(&user)?;

    let limit = query.limit.unwrap_or(50).min(100);
    let offset = query.offset.unwrap_or(0);

    // Build query with optional filters
    let mut sql = String::from(
        r#"
        SELECT id, user_id, action, input_content, output_content, options,
               content_id, block_id, model, input_tokens, output_tokens,
               processing_time_ms, created_at
        FROM cms_ai_assist_history
        WHERE user_id = $1
        "#,
    );

    let mut param_index = 2;
    let mut conditions = Vec::new();

    if query.action.is_some() {
        conditions.push(format!("action = ${}", param_index));
        param_index += 1;
    }
    if query.content_id.is_some() {
        conditions.push(format!("content_id = ${}", param_index));
    }

    if !conditions.is_empty() {
        sql.push_str(" AND ");
        sql.push_str(&conditions.join(" AND "));
    }

    sql.push_str(" ORDER BY created_at DESC LIMIT $");
    sql.push_str(&param_index.to_string());
    sql.push_str(" OFFSET $");
    sql.push_str(&(param_index + 1).to_string());

    // Execute query with dynamic binding
    let records: Vec<AiAssistHistoryRecord> = if let (Some(action), Some(content_id)) =
        (&query.action, query.content_id)
    {
        sqlx::query_as(&sql)
            .bind(user.id)
            .bind(action)
            .bind(content_id)
            .bind(limit)
            .bind(offset)
            .fetch_all(&state.db.pool)
            .await
    } else if let Some(action) = &query.action {
        sqlx::query_as(&sql)
            .bind(user.id)
            .bind(action)
            .bind(limit)
            .bind(offset)
            .fetch_all(&state.db.pool)
            .await
    } else if let Some(content_id) = query.content_id {
        sqlx::query_as(&sql)
            .bind(user.id)
            .bind(content_id)
            .bind(limit)
            .bind(offset)
            .fetch_all(&state.db.pool)
            .await
    } else {
        sqlx::query_as(
            r#"
            SELECT id, user_id, action, input_content, output_content, options,
                   content_id, block_id, model, input_tokens, output_tokens,
                   processing_time_ms, created_at
            FROM cms_ai_assist_history
            WHERE user_id = $1
            ORDER BY created_at DESC
            LIMIT $2 OFFSET $3
            "#,
        )
        .bind(user.id)
        .bind(limit)
        .bind(offset)
        .fetch_all(&state.db.pool)
        .await
    }
    .map_err(|e| {
        tracing::error!(target: "ai_assist", error = %e, "Failed to fetch history");
        api_error(
            StatusCode::INTERNAL_SERVER_ERROR,
            "Failed to fetch history",
        )
    })?;

    // Get total count
    let total: (i64,) = sqlx::query_as("SELECT COUNT(*) FROM cms_ai_assist_history WHERE user_id = $1")
        .bind(user.id)
        .fetch_one(&state.db.pool)
        .await
        .map_err(|e| {
            tracing::error!(target: "ai_assist", error = %e, "Failed to count history");
            api_error(
                StatusCode::INTERNAL_SERVER_ERROR,
                "Failed to fetch history",
            )
        })?;

    Ok(Json(HistoryResponse {
        data: records,
        meta: PaginationMeta {
            total: total.0,
            limit,
            offset,
        },
    }))
}

// ═══════════════════════════════════════════════════════════════════════════════════════
// ROUTER
// ═══════════════════════════════════════════════════════════════════════════════════════

/// CMS AI Assist routes (requires authentication)
pub fn router() -> Router<AppState> {
    Router::new()
        .route("/assist", post(ai_assist))
        .route("/assist/stream", post(ai_assist_stream))
        .route("/history", get(get_ai_history))
}
