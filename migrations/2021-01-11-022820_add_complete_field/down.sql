-- This file should undo anything in `up.sql`
CREATE TABLE temp_todo (
    id INTEGER PRIMARY KEY AUTOINCREMENT NOT NULL,
    project_id INTEGER,
    parent_id INTEGER,
    title TEXT NOT NULL,
    details TEXT,
    uuid TEXT NOT NULL,
    FOREIGN KEY(project_id) REFERENCES project(id),
    FOREIGN KEY(parent_id) REFERENCES todo(id)
);

INSERT INTO temp_todo (id,project_id,title,details,uuid)
SELECT id,project_id,title,details,uuid from todo where true;

DROP TABLE todo;

ALTER TABLE temp_todo RENAME TO todo;
