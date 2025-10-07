use std::collections::HashMap;

// Database abstraction layer for project management
use anyhow::Result;

pub type ProjectId = i64;
pub type ProjectTags = Vec<Tag>;
pub type ProjectFeatures = Vec<Feature>;
pub type ProjectJoin = HashMap<ProjectId, (Project, ProjectTags, ProjectFeatures)>;
pub type Time = i64; // Unix epoch timestamp, my preferred database time format

/// Project data structure
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize, sqlx::FromRow)]
pub struct Project {
    pub id: i64,
    pub name: String,
    pub description: Option<String>,  //Note: Use Option for nullable fields
    pub created_at: i64,
}

/// Tag for categorizing projects
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize, sqlx::FromRow)]
pub struct Tag {
    pub id: i64,
    pub name: String,
    pub color: Option<String>,  // Optional hex color
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize, sqlx::FromRow)]
pub struct ProjectTag {
    pub id: i64,
    pub project_id: i64,
    pub tag_id: i64
}

/// Project feature/task
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize, sqlx::FromRow)]
pub struct Feature {
    pub id: i64,
    pub project_id: i64,
    pub description: String,  // Made required to match SQL schema
    pub completed: bool,
    pub created_at: i64,
}

/// Trait that any database backend must implement
#[async_trait::async_trait]
pub trait ProjectDatabase: Send + Sync {
    // Project CRUD
    async fn create_project(&mut self, name: String, description: Option<String>) -> Result<Project>;
    async fn get_all_projects(&self) -> Result<Vec<(Project, ProjectTags, ProjectFeatures)>>;
    async fn delete_project(&mut self, id: i64) -> Result<()>;
    
    // Tag management
    async fn create_tag(&mut self, name: String) -> Result<Tag>;
    async fn get_all_tags(&self) -> Result<Vec<Tag>>;
    async fn get_project_tags(&self, project_id: i64) -> Result<Vec<Tag>>;
    async fn add_tag_to_project(&mut self, project_id: i64, tag_id: i64) -> Result<()>;
    async fn remove_tag_from_project(&mut self, project_id: i64, tag_id: i64) -> Result<()>;

    // Feature management
    async fn add_feature(&mut self, project_id: i64, description: String) -> Result<Feature>;
    async fn get_project_features(&self, project_id: i64) -> Result<Vec<Feature>>;
    async fn remove_feature(&mut self, feature_id: i64) -> Result<()>;
}

// Re-export SQLite implementation
pub mod sqlite;
pub use sqlite::SqliteDatabase;
