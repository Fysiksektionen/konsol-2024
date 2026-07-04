-- Your SQL goes here
CREATE TABLE settings (
    id INTEGER NOT NULL PRIMARY KEY CHECK (id = 1),
    layout_type TEXT NOT NULL DEFAULT 'mixed',
    color_mode TEXT NOT NULL DEFAULT 'light_mode'
)