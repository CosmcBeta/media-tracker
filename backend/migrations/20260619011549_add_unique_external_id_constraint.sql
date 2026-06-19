-- Add migration script here
CREATE UNIQUE INDEX idx_items_external_id_media_type
ON items(external_id, media_type)
WHERE external_id IS NOT NULL;
