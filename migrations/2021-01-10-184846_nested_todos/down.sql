-- This file should undo anything in `up.sql`
PRAGMA foreign_keys=off;

CREATE TABLE temp_todo (
    id INTEGER PRIMARY KEY AUTOINCREMENT NOT NULL,
    project_id INTEGER NOT NULL,
    title TEXT NOT NULL,
    details TEXT,
    uuid TEXT NOT NULL DEFAULT 0,
    FOREIGN KEY(project_id) REFERENCES project(id)
);

INSERT INTO temp_todo values (project_id,title,details,uuid)
SELECT project_id, title,title,details,uuid from todo;

DROP TABLE todo;

ALTER TABLE temp_todo RENAME TO todo;

PRAGMA foreign_keys=on;
