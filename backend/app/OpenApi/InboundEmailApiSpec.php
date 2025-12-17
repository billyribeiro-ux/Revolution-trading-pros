<?php

declare(strict_types=1);

namespace App\OpenApi;

/**
 * OpenAPI Specification for Inbound Email API
 *
 * This file contains the OpenAPI/Swagger documentation for the
 * inbound email webhook and conversation management endpoints.
 *
 * @OA\Info(
 *     version="1.0.0",
 *     title="CRM Pro Inbound Email API",
 *     description="API for processing inbound emails and managing email conversations",
 *     @OA\Contact(
 *         name="API Support",
 *         email="api@revolutiontradingpros.com"
 *     ),
 *     @OA\License(
 *         name="Proprietary",
 *         url="https://revolution-trading-pros.pages.dev/license"
 *     )
 * )
 *
 * @OA\Server(
 *     url="https://revolution-trading-pros.fly.dev",
 *     description="Production API Server"
 * )
 *
 * @OA\Server(
 *     url="https://revolution-trading-pros-staging.fly.dev",
 *     description="Staging API Server"
 * )
 *
 * @OA\SecurityScheme(
 *     securityScheme="bearerAuth",
 *     type="http",
 *     scheme="bearer",
 *     bearerFormat="JWT"
 * )
 *
 * @OA\Tag(
 *     name="Webhooks",
 *     description="Email provider webhook endpoints"
 * )
 *
 * @OA\Tag(
 *     name="Conversations",
 *     description="Email conversation management"
 * )
 *
 * @OA\Tag(
 *     name="Health",
 *     description="System health and monitoring"
 * )
 */
class InboundEmailApiSpec
{
    // This class serves as a container for OpenAPI annotations
}

/**
 * @OA\Schema(
 *     schema="EmailConversation",
 *     type="object",
 *     required={"id", "contact_id", "subject", "status"},
 *     @OA\Property(property="id", type="string", format="uuid", example="550e8400-e29b-41d4-a716-446655440000"),
 *     @OA\Property(property="contact_id", type="string", format="uuid"),
 *     @OA\Property(property="assigned_to", type="string", format="uuid", nullable=true),
 *     @OA\Property(property="subject", type="string", example="Re: Account inquiry"),
 *     @OA\Property(property="status", type="string", enum={"open", "pending", "resolved", "archived", "spam"}),
 *     @OA\Property(property="priority", type="string", enum={"low", "normal", "high", "urgent"}),
 *     @OA\Property(property="message_count", type="integer", example=5),
 *     @OA\Property(property="unread_count", type="integer", example=2),
 *     @OA\Property(property="last_message_at", type="string", format="date-time"),
 *     @OA\Property(property="created_at", type="string", format="date-time"),
 *     @OA\Property(property="updated_at", type="string", format="date-time")
 * )
 *
 * @OA\Schema(
 *     schema="EmailMessage",
 *     type="object",
 *     required={"id", "conversation_id", "direction", "from_email", "subject"},
 *     @OA\Property(property="id", type="string", format="uuid"),
 *     @OA\Property(property="conversation_id", type="string", format="uuid"),
 *     @OA\Property(property="direction", type="string", enum={"inbound", "outbound"}),
 *     @OA\Property(property="from_email", type="string", format="email"),
 *     @OA\Property(property="from_name", type="string", nullable=true),
 *     @OA\Property(property="to_email", type="string", format="email"),
 *     @OA\Property(property="subject", type="string"),
 *     @OA\Property(property="text_body", type="string", nullable=true),
 *     @OA\Property(property="html_body", type="string", nullable=true),
 *     @OA\Property(property="is_read", type="boolean"),
 *     @OA\Property(property="is_spam", type="boolean"),
 *     @OA\Property(property="attachment_count", type="integer"),
 *     @OA\Property(property="received_at", type="string", format="date-time"),
 *     @OA\Property(property="created_at", type="string", format="date-time")
 * )
 *
 * @OA\Schema(
 *     schema="PostmarkWebhookPayload",
 *     type="object",
 *     required={"From", "To", "Subject"},
 *     @OA\Property(property="MessageID", type="string", example="msg-123@postmarkapp.com"),
 *     @OA\Property(property="From", type="string", example="\"John Doe\" <john@example.com>"),
 *     @OA\Property(property="To", type="string", example="inbox@crm.example.com"),
 *     @OA\Property(property="Subject", type="string", example="Hello World"),
 *     @OA\Property(property="TextBody", type="string"),
 *     @OA\Property(property="HtmlBody", type="string"),
 *     @OA\Property(property="Date", type="string", format="date-time"),
 *     @OA\Property(property="MailboxHash", type="string", nullable=true),
 *     @OA\Property(property="SpamScore", type="number", format="float"),
 *     @OA\Property(property="SpamStatus", type="string", enum={"Yes", "No"}),
 *     @OA\Property(
 *         property="Headers",
 *         type="array",
 *         @OA\Items(
 *             type="object",
 *             @OA\Property(property="Name", type="string"),
 *             @OA\Property(property="Value", type="string")
 *         )
 *     ),
 *     @OA\Property(
 *         property="Attachments",
 *         type="array",
 *         @OA\Items(
 *             type="object",
 *             @OA\Property(property="Name", type="string"),
 *             @OA\Property(property="ContentType", type="string"),
 *             @OA\Property(property="ContentLength", type="integer"),
 *             @OA\Property(property="Content", type="string", format="byte")
 *         )
 *     )
 * )
 *
 * @OA\Schema(
 *     schema="HealthResponse",
 *     type="object",
 *     @OA\Property(property="status", type="string", enum={"healthy", "degraded", "unhealthy"}),
 *     @OA\Property(property="version", type="string", example="1.0.0"),
 *     @OA\Property(property="environment", type="string", example="production"),
 *     @OA\Property(
 *         property="checks",
 *         type="object",
 *         @OA\Property(
 *             property="database",
 *             type="object",
 *             @OA\Property(property="healthy", type="boolean"),
 *             @OA\Property(property="latency_ms", type="number")
 *         ),
 *         @OA\Property(
 *             property="cache",
 *             type="object",
 *             @OA\Property(property="healthy", type="boolean"),
 *             @OA\Property(property="latency_ms", type="number")
 *         )
 *     ),
 *     @OA\Property(property="timestamp", type="string", format="date-time")
 * )
 *
 * @OA\Schema(
 *     schema="ErrorResponse",
 *     type="object",
 *     @OA\Property(property="error", type="string"),
 *     @OA\Property(property="code", type="integer"),
 *     @OA\Property(property="correlation_id", type="string")
 * )
 */

/**
 * Webhook Endpoints
 *
 * @OA\Post(
 *     path="/api/webhooks/email/postmark",
 *     summary="Postmark Inbound Email Webhook",
 *     description="Receives inbound emails from Postmark and processes them into the CRM",
 *     operationId="postmarkWebhook",
 *     tags={"Webhooks"},
 *     @OA\RequestBody(
 *         required=true,
 *         @OA\JsonContent(ref="#/components/schemas/PostmarkWebhookPayload")
 *     ),
 *     @OA\Parameter(
 *         name="X-Postmark-Signature",
 *         in="header",
 *         required=true,
 *         description="HMAC-SHA256 signature for payload verification",
 *         @OA\Schema(type="string")
 *     ),
 *     @OA\Parameter(
 *         name="X-Idempotency-Key",
 *         in="header",
 *         description="Idempotency key to prevent duplicate processing",
 *         @OA\Schema(type="string")
 *     ),
 *     @OA\Response(
 *         response=200,
 *         description="Email processed successfully",
 *         @OA\JsonContent(
 *             @OA\Property(property="success", type="boolean", example=true),
 *             @OA\Property(property="message_id", type="string", format="uuid"),
 *             @OA\Property(property="conversation_id", type="string", format="uuid")
 *         )
 *     ),
 *     @OA\Response(
 *         response=401,
 *         description="Invalid signature",
 *         @OA\JsonContent(ref="#/components/schemas/ErrorResponse")
 *     ),
 *     @OA\Response(
 *         response=429,
 *         description="Rate limit exceeded",
 *         @OA\Header(
 *             header="Retry-After",
 *             description="Seconds until rate limit resets",
 *             @OA\Schema(type="integer")
 *         )
 *     )
 * )
 *
 * @OA\Post(
 *     path="/api/webhooks/email/ses",
 *     summary="AWS SES Inbound Email Webhook",
 *     description="Receives inbound emails from AWS SES via SNS notifications",
 *     operationId="sesWebhook",
 *     tags={"Webhooks"},
 *     @OA\RequestBody(
 *         required=true,
 *         @OA\JsonContent(
 *             @OA\Property(property="Type", type="string", example="Notification"),
 *             @OA\Property(property="Message", type="string", description="JSON-encoded SES notification")
 *         )
 *     ),
 *     @OA\Response(
 *         response=200,
 *         description="Email processed successfully"
 *     )
 * )
 */

/**
 * Conversation Endpoints
 *
 * @OA\Get(
 *     path="/api/admin/email/conversations",
 *     summary="List Email Conversations",
 *     description="Retrieves paginated list of email conversations with filtering",
 *     operationId="listConversations",
 *     tags={"Conversations"},
 *     security={{"bearerAuth": {}}},
 *     @OA\Parameter(
 *         name="status",
 *         in="query",
 *         description="Filter by status",
 *         @OA\Schema(type="string", enum={"open", "pending", "resolved", "archived", "spam"})
 *     ),
 *     @OA\Parameter(
 *         name="priority",
 *         in="query",
 *         description="Filter by priority",
 *         @OA\Schema(type="string", enum={"low", "normal", "high", "urgent"})
 *     ),
 *     @OA\Parameter(
 *         name="assigned_to",
 *         in="query",
 *         description="Filter by assigned user ID",
 *         @OA\Schema(type="string", format="uuid")
 *     ),
 *     @OA\Parameter(
 *         name="contact_id",
 *         in="query",
 *         description="Filter by contact ID",
 *         @OA\Schema(type="string", format="uuid")
 *     ),
 *     @OA\Parameter(
 *         name="search",
 *         in="query",
 *         description="Search in subject and messages",
 *         @OA\Schema(type="string")
 *     ),
 *     @OA\Parameter(
 *         name="per_page",
 *         in="query",
 *         description="Items per page (default: 20, max: 100)",
 *         @OA\Schema(type="integer", default=20)
 *     ),
 *     @OA\Response(
 *         response=200,
 *         description="List of conversations",
 *         @OA\JsonContent(
 *             @OA\Property(property="data", type="array", @OA\Items(ref="#/components/schemas/EmailConversation")),
 *             @OA\Property(property="meta", type="object",
 *                 @OA\Property(property="current_page", type="integer"),
 *                 @OA\Property(property="total", type="integer"),
 *                 @OA\Property(property="per_page", type="integer")
 *             )
 *         )
 *     ),
 *     @OA\Response(response=401, description="Unauthenticated")
 * )
 *
 * @OA\Get(
 *     path="/api/admin/email/conversations/{id}",
 *     summary="Get Conversation Details",
 *     description="Retrieves a single conversation with its messages",
 *     operationId="getConversation",
 *     tags={"Conversations"},
 *     security={{"bearerAuth": {}}},
 *     @OA\Parameter(
 *         name="id",
 *         in="path",
 *         required=true,
 *         @OA\Schema(type="string", format="uuid")
 *     ),
 *     @OA\Response(
 *         response=200,
 *         description="Conversation details",
 *         @OA\JsonContent(ref="#/components/schemas/EmailConversation")
 *     ),
 *     @OA\Response(response=404, description="Conversation not found")
 * )
 *
 * @OA\Patch(
 *     path="/api/admin/email/conversations/{id}",
 *     summary="Update Conversation",
 *     description="Updates conversation status, priority, or assignment",
 *     operationId="updateConversation",
 *     tags={"Conversations"},
 *     security={{"bearerAuth": {}}},
 *     @OA\Parameter(
 *         name="id",
 *         in="path",
 *         required=true,
 *         @OA\Schema(type="string", format="uuid")
 *     ),
 *     @OA\RequestBody(
 *         @OA\JsonContent(
 *             @OA\Property(property="status", type="string", enum={"open", "pending", "resolved", "archived", "spam"}),
 *             @OA\Property(property="priority", type="string", enum={"low", "normal", "high", "urgent"}),
 *             @OA\Property(property="assigned_to", type="string", format="uuid", nullable=true)
 *         )
 *     ),
 *     @OA\Response(
 *         response=200,
 *         description="Conversation updated",
 *         @OA\JsonContent(ref="#/components/schemas/EmailConversation")
 *     )
 * )
 *
 * @OA\Post(
 *     path="/api/admin/email/conversations/{id}/reply",
 *     summary="Reply to Conversation",
 *     description="Sends a reply to the conversation",
 *     operationId="replyToConversation",
 *     tags={"Conversations"},
 *     security={{"bearerAuth": {}}},
 *     @OA\Parameter(
 *         name="id",
 *         in="path",
 *         required=true,
 *         @OA\Schema(type="string", format="uuid")
 *     ),
 *     @OA\RequestBody(
 *         required=true,
 *         @OA\JsonContent(
 *             required={"body"},
 *             @OA\Property(property="body", type="string", description="Reply message body"),
 *             @OA\Property(property="html_body", type="string", description="HTML version of reply")
 *         )
 *     ),
 *     @OA\Response(
 *         response=201,
 *         description="Reply sent",
 *         @OA\JsonContent(ref="#/components/schemas/EmailMessage")
 *     )
 * )
 */

/**
 * Health Endpoints
 *
 * @OA\Get(
 *     path="/health/live",
 *     summary="Liveness Probe",
 *     description="Returns 200 if application is alive (for Kubernetes liveness probe)",
 *     operationId="liveness",
 *     tags={"Health"},
 *     @OA\Response(
 *         response=200,
 *         description="Application is alive",
 *         @OA\JsonContent(
 *             @OA\Property(property="status", type="string", example="alive"),
 *             @OA\Property(property="timestamp", type="string", format="date-time")
 *         )
 *     )
 * )
 *
 * @OA\Get(
 *     path="/health/ready",
 *     summary="Readiness Probe",
 *     description="Returns 200 if application is ready to receive traffic",
 *     operationId="readiness",
 *     tags={"Health"},
 *     @OA\Response(
 *         response=200,
 *         description="Application is ready",
 *         @OA\JsonContent(
 *             @OA\Property(property="status", type="string", example="ready"),
 *             @OA\Property(property="checks", type="object")
 *         )
 *     ),
 *     @OA\Response(
 *         response=503,
 *         description="Application is not ready"
 *     )
 * )
 *
 * @OA\Get(
 *     path="/health",
 *     summary="Detailed Health Check",
 *     description="Returns detailed health status of all system components",
 *     operationId="healthDetailed",
 *     tags={"Health"},
 *     @OA\Response(
 *         response=200,
 *         description="System is healthy",
 *         @OA\JsonContent(ref="#/components/schemas/HealthResponse")
 *     ),
 *     @OA\Response(
 *         response=503,
 *         description="System is degraded",
 *         @OA\JsonContent(ref="#/components/schemas/HealthResponse")
 *     )
 * )
 *
 * @OA\Get(
 *     path="/metrics",
 *     summary="Prometheus Metrics",
 *     description="Returns metrics in Prometheus exposition format",
 *     operationId="metrics",
 *     tags={"Health"},
 *     @OA\Response(
 *         response=200,
 *         description="Prometheus metrics",
 *         @OA\MediaType(
 *             mediaType="text/plain",
 *             @OA\Schema(type="string")
 *         )
 *     )
 * )
 */
