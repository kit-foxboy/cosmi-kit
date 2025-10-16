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
    pub id: ProjectId,
    pub name: String,
    pub description: Option<String>,  //Note: Use Option for nullable fields
    pub created_at: Time,
}

/// Tag for categorizing projects
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize, sqlx::FromRow)]
pub struct Tag {
    pub id: i64,
    pub name: String,
    pub color: Option<String>,  // Optional hex color
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
    async fn get_all_projects(&self) -> Result<Vec<ProjectJoin>>;
    async fn get_project_by_id(&self, id: i64) -> Result<ProjectJoin>;
    async fn update_project(&mut self, id: i64, name: String, description: Option<String>) -> Result<Project>;
    async fn delete_project(&mut self, id: i64) -> Result<()>;
    
    // Tag management
    async fn create_tags(&mut self, project_id: ProjectId, names: Vec<String>) -> Result<Vec<Tag>>;
    async fn get_all_tags(&self) -> Result<Vec<Tag>>;
    async fn remove_tags_by_name(&mut self, project_id: i64, tag_names: Vec<String>) -> Result<()>;

    // Feature management
    async fn add_feature(&mut self, project_id: i64, description: String) -> Result<Feature>;
    async fn remove_features_by_description(&mut self, project_id: i64, descriptions: Vec<String>) -> Result<()>;
    async fn toggle_feature_completed(&mut self, feature_id: i64) -> Result<bool>;
}

// Re-export SQLite implementation
pub mod sqlite;
pub use sqlite::SqliteDatabase;
