# API Documentation

## AI Generation

### POST `/api/cms/ai/generate`

Generate content using AI.

**Request Body:**

```json
{
	"prompt": "Write a paragraph about trading strategies",
	"model": "claude-3-5-sonnet-20241022",
	"type": "generate"
}
```

**Response:**

```json
{
	"output": "Generated content here...",
	"usage": {
		"input_tokens": 50,
		"output_tokens": 200
	}
}
```

**Types:**

- `generate` - General content generation
- `summary` - Text summarization
- `translation` - Language translation

## Image Upload

### POST `/api/cms/upload/image`

Upload an image file.

**Request:** multipart/form-data with `file` field

**Response:**

```json
{
	"url": "https://cdn.example.com/image.jpg",
	"filename": "image.jpg",
	"size": 1024000,
	"type": "image/jpeg"
}
```

**Limits:**

- Max size: 10MB
- Allowed types: JPEG, PNG, WebP, GIF

## Newsletter

### POST `/api/cms/newsletter/subscribe`

Subscribe to newsletter.

**Request Body:**

```json
{
	"email": "user@example.com"
}
```

**Response:**

```json
{
	"success": true,
	"message": "Successfully subscribed"
}
```

## Error Responses

All endpoints may return:

**400 Bad Request:**

```json
{
	"message": "Invalid email address"
}
```

**500 Internal Server Error:**

```json
{
	"message": "AI generation failed"
}
```
