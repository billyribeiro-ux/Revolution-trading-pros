-- Stable unguessable delivery keys keep published image URLs valid across
-- replacements while allowing the selected derivative to change atomically.
ALTER TABLE media ADD COLUMN public_token UUID NOT NULL DEFAULT gen_random_uuid();
CREATE UNIQUE INDEX media_public_token_unique ON media(public_token);
