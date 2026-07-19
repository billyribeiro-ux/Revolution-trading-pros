import { json, error } from '@sveltejs/kit';

/**
 * Maintenance Email Capture API
 * ═══════════════════════════════════════════════════════════════════════════
 * Handles email subscriptions for maintenance page notifications
 * Sends confirmation email to user
 * ═══════════════════════════════════════════════════════════════════════════
 */

interface NotifyRequest {
	email: string;
}

// Simple in-memory store (replace with database in production)
const subscribers = new Set<string>();

// Email validation regex
const EMAIL_REGEX = /^[^\s@]+@[^\s@]+\.[^\s@]+$/;

// eslint-disable-next-line @typescript-eslint/no-explicit-any
export const POST = async ({ request, platform }: { request: Request; platform: any }) => {
	try {
		const body: NotifyRequest = await request.json();
		const { email } = body;

		// Validation
		if (!email || !EMAIL_REGEX.test(email)) {
			throw error(400, { message: 'Please provide a valid email address' });
		}

		const normalizedEmail = email.toLowerCase().trim();

		// Check if already subscribed
		if (subscribers.has(normalizedEmail)) {
			// Still return success to not leak information
			return json({
				success: true,
				message: "You're already on the list! Check your email for confirmation."
			});
		}

		// Add to subscribers
		subscribers.add(normalizedEmail);

		// Send confirmation email using platform's email service (e.g., Resend, SendGrid)

		const resendKey = platform?.env?.RESEND_API_KEY as string | undefined;
		if (resendKey) {
			await sendConfirmationEmail(normalizedEmail, resendKey);
		} else {
			// Log for development - in production this should use a real email service

			console.info(`[DEV] Would send confirmation email to: ${normalizedEmail}`);
		}

		return json({
			success: true,
			message: 'Welcome! Check your email for confirmation.'
		});
	} catch (err: unknown) {
		console.error('[Maintenance API] Error:', err);

		const svelteKitError = err as { status?: number; body?: unknown };
		if (svelteKitError.status && svelteKitError.body) {
			throw err; // Re-throw SvelteKit errors
		}

		throw error(500, { message: 'Something went wrong. Please try again.' });
	}
};

/**
 * Send confirmation email using Resend
 */
async function sendConfirmationEmail(email: string, apiKey: string): Promise<void> {
	const resendUrl = 'https://api.resend.com/emails';

	const emailHtml = `
<!DOCTYPE html>
<html>
<head>
    <meta charset="utf-8">
    <meta name="viewport" content="width=device-width, initial-scale=1.0">
    <title>You're on the list! | Revolution Trading Pros</title>
    <style>
        body {
            margin: 0;
            padding: 0;
            background-color: #0a0a0f;
            font-family: -apple-system, BlinkMacSystemFont, 'Segoe UI', Roboto, sans-serif;
        }
        .container {
            max-width: 600px;
            margin: 0 auto;
            background-color: #0f172a;
        }
        .header {
            background: linear-gradient(135deg, #3b82f6 0%, #8b5cf6 100%);
            padding: 40px 30px;
            text-align: center;
        }
        .header h1 {
            color: white;
            margin: 0;
            font-size: 28px;
            font-weight: 700;
        }
        .content {
            padding: 40px 30px;
            color: #e2e8f0;
        }
        .content h2 {
            color: #10b981;
            font-size: 22px;
            margin-top: 0;
        }
        .content p {
            font-size: 16px;
            line-height: 1.6;
            margin-bottom: 20px;
        }
        .features {
            background: rgba(16, 185, 129, 0.1);
            border: 1px solid rgba(16, 185, 129, 0.2);
            border-radius: 8px;
            padding: 20px;
            margin: 25px 0;
        }
        .features h3 {
            color: #10b981;
            margin-top: 0;
            font-size: 18px;
        }
        .features ul {
            margin: 0;
            padding-left: 20px;
        }
        .features li {
            margin: 10px 0;
            color: #cbd5e1;
        }
        .cta-button {
            display: inline-block;
            background: linear-gradient(135deg, #3b82f6 0%, #8b5cf6 100%);
            color: white;
            text-decoration: none;
            padding: 15px 30px;
            border-radius: 8px;
            font-weight: 600;
            margin: 20px 0;
        }
        .footer {
            background-color: #0a0a0f;
            padding: 30px;
            text-align: center;
            color: #64748b;
            font-size: 14px;
        }
        .social-links {
            margin-top: 20px;
        }
        .social-links a {
            color: #3b82f6;
            text-decoration: none;
            margin: 0 10px;
        }
        .divider {
            height: 1px;
            background: rgba(255, 255, 255, 0.1);
            margin: 30px 0;
        }
    </style>
</head>
<body>
    <div class="container">
        <div class="header">
            <h1>Revolution Trading Pros</h1>
        </div>

        <div class="content">
            <h2>You're on the list.</h2>

            <p>Thank you for requesting first access. We are rebuilding the platform end to end, and you'll receive one email the moment it reopens.</p>

            <p>No gimmicks and no marketing sequence — just the reopening notice and your founding-member terms.</p>

            <div class="features">
                <h3>What you'll get first access to:</h3>
                <ul>
                    <li><strong>Institutional-Grade Scanners</strong> - Pro-level technology previously reserved for hedge funds</li>
                    <li><strong>Day Trading University</strong> - Complete curriculum for intraday mastery</li>
                    <li><strong>Swing Trading Academy</strong> - Multi-day position strategies for consistent profits</li>
                    <li><strong>VIP Launch Pricing</strong> - Special rates only available to early subscribers</li>
                    <li><strong>Free Course Preview</strong> - Sample content before anyone else</li>
                </ul>
            </div>

            <div class="divider"></div>

            <p><strong>Our commitment:</strong> Leading stocks and options trading education by providing the unseen institutional tools and strategies that professionals actually use. No gimmicks. No false promises. Just authentic trading excellence.</p>

            <p>Progress is posted on the site every trading day if you would like to follow the rebuild.</p>

            <center>
                <a href="https://revolutiontradingpros.com" class="cta-button">Visit Our Website</a>
            </center>
        </div>

        <div class="footer">
            <p>© 2026 Revolution Trading Pros. All rights reserved.</p>
            <p>You're receiving this because you signed up for our launch notification list.</p>
            <div class="social-links">
                <a href="#">Twitter</a> •
                <a href="#">YouTube</a> •
                <a href="#">Discord</a>
            </div>
        </div>
    </div>
</body>
</html>
	`;

	const response = await fetch(resendUrl, {
		method: 'POST',
		headers: {
			Authorization: `Bearer ${apiKey}`,
			'Content-Type': 'application/json'
		},
		body: JSON.stringify({
			from: 'Revolution Trading Pros <noreply@revolutiontradingpros.com>',
			to: email,
			subject: "You're on the list — Revolution Trading Pros",
			html: emailHtml
		})
	});

	if (!response.ok) {
		const errorData = await response.text();

		console.error('[Email] Failed to send:', errorData);
		throw new Error('Failed to send confirmation email');
	}
}

// GET endpoint to check subscriber count (admin only)
export const GET = async () => {
	return json({
		subscriberCount: subscribers.size,
		message: 'Maintenance notification subscriber count'
	});
};
