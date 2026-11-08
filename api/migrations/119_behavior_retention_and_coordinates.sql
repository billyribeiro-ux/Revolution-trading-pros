ALTER TABLE behavior_events ADD COLUMN coordinate_space VARCHAR(32);
CREATE INDEX behavior_events_retention_idx ON behavior_events (received_at, id);

-- Older clients sent CSS IDs/classes and protected paths. Remove those values
-- before the new dashboards can expose historical rows.
UPDATE behavior_events SET element_selector = NULL
WHERE element_selector IS NOT NULL
  AND element_selector NOT IN ('a','button','div','span','img','svg','path','li','nav','header','footer','section','article','p','h1','h2','h3','h4','h5','h6','label','form','main','summary','details');
DELETE FROM behavior_events
WHERE page_url ~* '^/(admin|account|auth|checkout|login|register|dashboard|my|profile|settings|billing|orders|member|members|rooms|classroom|reset-password|verify-email|forgot-password|unsubscribe)(/|$)'
   OR position('@' in page_url) > 0
   OR position('%40' in lower(page_url)) > 0;
