-- Live mode: show a public Google Slides presentation instead of the
-- uploaded slide images, so edits to the presentation show up on the
-- screen without re-uploading anything.
ALTER TABLE settings ADD COLUMN live_mode BOOLEAN NOT NULL DEFAULT FALSE;
ALTER TABLE settings ADD COLUMN live_slides_url TEXT NOT NULL DEFAULT '';
