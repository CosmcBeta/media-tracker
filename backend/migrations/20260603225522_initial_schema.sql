CREATE TABLE items (
    id          TEXT PRIMARY KEY,
    media_type  TEXT NOT NULL CHECK(media_type IN ('movie', 'show', 'album', 'artist', 'book', 'game', 'podcast')),
    title       TEXT NOT NULL,
    external_id TEXT,
    metadata    TEXT,
    created_at  TEXT NOT NULL,
    updated_at  TEXT NOT NULL
);

CREATE TABLE lists (
    id         TEXT PRIMARY KEY,
    name       TEXT NOT NULL,
    icon       TEXT,
    created_at TEXT NOT NULL
);

CREATE TABLE list_items (
    list_id    TEXT NOT NULL REFERENCES lists(id) ON DELETE CASCADE,
    item_id    TEXT NOT NULL REFERENCES items(id) ON DELETE CASCADE,
    added_at   TEXT NOT NULL,
    sort_order INTEGER DEFAULT 0,
    PRIMARY KEY (list_id, item_id)
);

CREATE TABLE progress (
    id         TEXT PRIMARY KEY,
    item_id    TEXT NOT NULL REFERENCES items(id) ON DELETE CASCADE,
    kind       TEXT NOT NULL CHECK(kind IN ('episode', 'page', 'percentage', 'complete')), -- might add other stuff like song or such.
    value      TEXT,
    note       TEXT,
    logged_at  TEXT NOT NULL
);
