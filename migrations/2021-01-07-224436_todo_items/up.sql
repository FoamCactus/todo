-- Your SQL goes here
CREATE TABLE project (
    id INTEGER PRIMARY KEY AUTOINCREMENT NOT NULL,
    title TEXT NOT NULL
);

CREATE TABLE todo (
    id INTEGER PRIMARY KEY AUTOINCREMENT NOT NULL,
    project_id INTEGER NOT NULL,
    title TEXT NOT NULL,
    details TEXT,
    FOREIGN KEY(project_id) REFERENCES Project(id)
);
