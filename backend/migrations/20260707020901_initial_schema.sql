CREATE TABLE items (
    id          UUID PRIMARY KEY,
    media_type  TEXT NOT NULL CHECK (media_type IN ('movie', 'show', 'album', 'artist', 'book', 'game', 'podcast')),
    title       TEXT NOT NULL,
    external_id TEXT,
    metadata    JSONB,
    created_at  TIMESTAMPTZ NOT NULL,
    updated_at  TIMESTAMPTZ NOT NULL
);

CREATE TABLE lists (
    id         UUID PRIMARY KEY,
    name       TEXT NOT NULL,
    icon       TEXT,
    created_at TIMESTAMPTZ NOT NULL,
    updated_at TIMESTAMPTZ NOT NULL
);

CREATE TABLE list_items (
    list_id    UUID NOT NULL REFERENCES lists(id) ON DELETE CASCADE,
    item_id    UUID NOT NULL REFERENCES items(id) ON DELETE CASCADE,
    added_at   TIMESTAMPTZ NOT NULL,
    sort_order INTEGER DEFAULT 0,
    PRIMARY KEY (list_id, item_id)
);

CREATE TABLE progress (
    id         UUID PRIMARY KEY,
    item_id    UUID NOT NULL REFERENCES items(id) ON DELETE CASCADE,
    kind       TEXT NOT NULL CHECK (kind IN ('episode', 'page', 'percentage', 'complete')), -- might add song, etc.
    value      TEXT,
    note       TEXT,
    logged_at  TIMESTAMPTZ NOT NULL
);

CREATE UNIQUE INDEX idx_items_external_id_media_type
ON items(external_id, media_type)
WHERE external_id IS NOT NULL;
