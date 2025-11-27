<!DOCTYPE html>
<html lang="en">
<head>
    <meta charset="UTF-8">
    <meta name="viewport" content="width=device-width, initial-scale=1.0">
    <meta http-equiv="X-UA-Compatible" content="IE=edge">
    <title>@yield('title', 'Revolution Trading Pros')</title>
    <!--[if mso]>
    <noscript>
        <xml>
            <o:OfficeDocumentSettings>
                <o:PixelsPerInch>96</o:PixelsPerInch>
            </o:OfficeDocumentSettings>
        </xml>
    </noscript>
    <![endif]-->
    <style>
        /* Reset styles */
        body, table, td, p, a, li, blockquote {
            -webkit-text-size-adjust: 100%;
            -ms-text-size-adjust: 100%;
        }
        table, td {
            mso-table-lspace: 0pt;
            mso-table-rspace: 0pt;
        }
        img {
            -ms-interpolation-mode: bicubic;
            border: 0;
            height: auto;
            line-height: 100%;
            outline: none;
            text-decoration: none;
        }
        body {
            margin: 0 !important;
            padding: 0 !important;
            width: 100% !important;
            background-color: #f4f7fa;
        }

        /* Typography */
        .email-body {
            font-family: -apple-system, BlinkMacSystemFont, 'Segoe UI', Roboto, Helvetica, Arial, sans-serif;
            font-size: 16px;
            line-height: 1.6;
            color: #374151;
        }

        /* Container */
        .email-container {
            max-width: 600px;
            margin: 0 auto;
            background-color: #ffffff;
            border-radius: 12px;
            overflow: hidden;
            box-shadow: 0 4px 6px -1px rgba(0, 0, 0, 0.1), 0 2px 4px -1px rgba(0, 0, 0, 0.06);
        }

        /* Header */
        .email-header {
            background: linear-gradient(135deg, #1e3a5f 0%, #2563eb 100%);
            padding: 32px 40px;
            text-align: center;
        }
        .email-header img {
            max-width: 180px;
            height: auto;
        }
        .email-header h1 {
            color: #ffffff;
            font-size: 28px;
            font-weight: 700;
            margin: 16px 0 0 0;
            text-shadow: 0 2px 4px rgba(0, 0, 0, 0.2);
        }

        /* Content */
        .email-content {
            padding: 40px;
        }
        .email-content h2 {
            color: #1e3a5f;
            font-size: 24px;
            font-weight: 600;
            margin: 0 0 16px 0;
        }
        .email-content p {
            margin: 0 0 16px 0;
        }

        /* Button */
        .btn-primary {
            display: inline-block;
            background: linear-gradient(135deg, #2563eb 0%, #1d4ed8 100%);
            color: #ffffff !important;
            text-decoration: none;
            padding: 16px 32px;
            border-radius: 8px;
            font-weight: 600;
            font-size: 16px;
            text-align: center;
            box-shadow: 0 4px 14px 0 rgba(37, 99, 235, 0.4);
            transition: transform 0.2s ease;
        }
        .btn-primary:hover {
            transform: translateY(-2px);
        }
        .btn-secondary {
            display: inline-block;
            background: #f3f4f6;
            color: #374151 !important;
            text-decoration: none;
            padding: 12px 24px;
            border-radius: 8px;
            font-weight: 500;
            font-size: 14px;
            border: 1px solid #e5e7eb;
        }

        /* Info box */
        .info-box {
            background: #f0f9ff;
            border-left: 4px solid #2563eb;
            padding: 16px 20px;
            border-radius: 0 8px 8px 0;
            margin: 24px 0;
        }
        .info-box p {
            margin: 0;
            color: #1e40af;
        }

        /* Success box */
        .success-box {
            background: #ecfdf5;
            border-left: 4px solid #10b981;
            padding: 16px 20px;
            border-radius: 0 8px 8px 0;
            margin: 24px 0;
        }
        .success-box p {
            margin: 0;
            color: #047857;
        }

        /* Warning box */
        .warning-box {
            background: #fffbeb;
            border-left: 4px solid #f59e0b;
            padding: 16px 20px;
            border-radius: 0 8px 8px 0;
            margin: 24px 0;
        }
        .warning-box p {
            margin: 0;
            color: #b45309;
        }

        /* Feature list */
        .feature-list {
            list-style: none;
            padding: 0;
            margin: 24px 0;
        }
        .feature-list li {
            padding: 12px 0 12px 36px;
            position: relative;
            border-bottom: 1px solid #f3f4f6;
        }
        .feature-list li:last-child {
            border-bottom: none;
        }
        .feature-list li::before {
            content: "✓";
            position: absolute;
            left: 0;
            color: #10b981;
            font-weight: bold;
            font-size: 18px;
        }

        /* Order summary */
        .order-summary {
            background: #f9fafb;
            border-radius: 8px;
            padding: 24px;
            margin: 24px 0;
        }
        .order-row {
            display: flex;
            justify-content: space-between;
            padding: 8px 0;
            border-bottom: 1px solid #e5e7eb;
        }
        .order-row:last-child {
            border-bottom: none;
            font-weight: 600;
            padding-top: 16px;
            margin-top: 8px;
            border-top: 2px solid #e5e7eb;
        }

        /* Footer */
        .email-footer {
            background: #f9fafb;
            padding: 32px 40px;
            text-align: center;
            border-top: 1px solid #e5e7eb;
        }
        .email-footer p {
            color: #6b7280;
            font-size: 14px;
            margin: 0 0 8px 0;
        }
        .email-footer a {
            color: #2563eb;
            text-decoration: none;
        }
        .social-links {
            margin: 16px 0;
        }
        .social-links a {
            display: inline-block;
            margin: 0 8px;
            width: 36px;
            height: 36px;
            background: #e5e7eb;
            border-radius: 50%;
            line-height: 36px;
            text-align: center;
            color: #6b7280;
            text-decoration: none;
        }
        .social-links a:hover {
            background: #2563eb;
            color: #ffffff;
        }

        /* Responsive */
        @media only screen and (max-width: 600px) {
            .email-container {
                margin: 0;
                border-radius: 0;
            }
            .email-header, .email-content, .email-footer {
                padding: 24px 20px;
            }
            .email-header h1 {
                font-size: 22px;
            }
            .btn-primary {
                display: block;
                width: 100%;
                box-sizing: border-box;
            }
        }
    </style>
</head>
<body>
    <table role="presentation" width="100%" cellspacing="0" cellpadding="0" style="background-color: #f4f7fa; padding: 40px 20px;">
        <tr>
            <td align="center">
                <div class="email-container email-body">
                    <!-- Header -->
                    <div class="email-header">
                        <h1>Revolution Trading Pros</h1>
                        @hasSection('header-subtitle')
                            <p style="color: rgba(255,255,255,0.9); margin: 8px 0 0 0; font-size: 16px;">@yield('header-subtitle')</p>
                        @endif
                    </div>

                    <!-- Content -->
                    <div class="email-content">
                        @yield('content')
                    </div>

                    <!-- Footer -->
                    <div class="email-footer">
                        <div class="social-links">
                            <a href="https://twitter.com/revtradingpros" title="Twitter">𝕏</a>
                            <a href="https://discord.gg/revtradingpros" title="Discord">D</a>
                            <a href="https://youtube.com/@revtradingpros" title="YouTube">▶</a>
                        </div>
                        <p><a href="{{ config('app.url') }}">Revolution Trading Pros</a></p>
                        <p style="font-size: 12px; color: #9ca3af;">
                            You're receiving this email because you have an account with us.<br>
                            <a href="{{ config('app.url') }}/settings/notifications">Manage email preferences</a> |
                            <a href="{{ config('app.url') }}/unsubscribe">Unsubscribe</a>
                        </p>
                        <p style="font-size: 12px; color: #9ca3af; margin-top: 16px;">
                            &copy; {{ date('Y') }} Revolution Trading Pros. All rights reserved.
                        </p>
                    </div>
                </div>
            </td>
        </tr>
    </table>
</body>
</html>
