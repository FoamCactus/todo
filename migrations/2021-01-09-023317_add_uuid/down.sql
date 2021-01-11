-- This file should undo anything in `up.sql
PRAGMA foreign_keys=off;
CREATE TABLE temp_project (
    id INTEGER PRIMARY KEY AUTOINCREMENT NOT NULL,
    title TEXT NOT NULL,
    uuid TEXT NOT NULL DEFAULT 0

);

CREATE TABLE temp_todo (
    id INTEGER PRIMARY KEY AUTOINCREMENT NOT NULL,
    project_id INTEGER NOT NULL,
    title TEXT NOT NULL,
    details TEXT,
    uuid TEXT NOT NULL DEFAULT 0,
    FOREIGN KEY(project_id) REFERENCES project(id)
);

INSERT INTO temp_todo values (project_id,title,details)
SELECT project_id, title,title,details from todo;

INSERT INTO temp_project values (id,title)
SELECT id,title from project;

DROP TABLE todo;
DROP TABLE project;

ALTER TABLE temp_todo RENAME TO todo;
ALTER TABLE temp_project RENAME TO project;

PRAGMA foreign_keys=on;
