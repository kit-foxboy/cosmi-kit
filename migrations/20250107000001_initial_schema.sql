-- Create projects table
CREATE TABLE IF NOT EXISTS projects (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    name TEXT NOT NULL,
    description TEXT,
    created_at INTEGER NOT NULL DEFAULT (strftime('%s', 'now'))
);

-- Create tags table
CREATE TABLE IF NOT EXISTS tags (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    name TEXT NOT NULL UNIQUE,
    color TEXT  -- Optional hex color like "#FF5733"
);

-- Create junction table for project-tag many-to-many relationship
CREATE TABLE IF NOT EXISTS project_tags (
    project_id INTEGER NOT NULL,
    tag_id INTEGER NOT NULL,
    PRIMARY KEY (project_id, tag_id),
    FOREIGN KEY (project_id) REFERENCES projects(id) ON DELETE CASCADE,
    FOREIGN KEY (tag_id) REFERENCES tags(id) ON DELETE CASCADE
);

-- Create features table (tasks/features for each project)
CREATE TABLE IF NOT EXISTS features (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    project_id INTEGER NOT NULL,
    description TEXT NOT NULL,
    completed INTEGER NOT NULL DEFAULT 0,  -- SQLite uses 0/1 for boolean
    created_at INTEGER NOT NULL DEFAULT (strftime('%s', 'now')),
    FOREIGN KEY (project_id) REFERENCES projects(id) ON DELETE CASCADE
);

-- Create indexes for better query performance
CREATE INDEX IF NOT EXISTS idx_project_tags_project ON project_tags(project_id);
CREATE INDEX IF NOT EXISTS idx_project_tags_tag ON project_tags(tag_id);
CREATE INDEX IF NOT EXISTS idx_features_project ON features(project_id);
CREATE INDEX IF NOT EXISTS idx_features_completed ON features(completed);
