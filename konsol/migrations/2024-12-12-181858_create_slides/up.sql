CREATE TABLE slides (
    id TEXT PRIMARY KEY NOT NULL,
    caption TEXT NOT NULL,
    start_date DATETIME NOT NULL,
    end_date DATETIME NOT NULL,
    active BOOLEAN NOT NULL DEFAULT 0,
    filetype TEXT NOT NULL
)