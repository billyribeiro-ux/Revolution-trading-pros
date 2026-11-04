-- Local template slugs have always allowed255 characters. An explicit test of
-- saved content must preserve its complete identity in the delivery receipt.
ALTER TABLE email_logs ALTER COLUMN template_alias TYPE TEXT;
