// SPDX-License-Identifier: MPL-2.0

//! Application data layer - handles all data operations
//! 
//! This module acts as a repository/service layer between the UI and the database.
//! Pages emit messages, AppModel delegates to AppData, AppData handles the actual work.

use crate::database::{Feature, Project, ProjectDatabase, ProjectId, ProjectJoin, SqliteDatabase, Tag};
use anyhow::Result;
use serde::{Deserialize, Serialize};
use chrono::Utc;

/// Shared data structures used across the application

/// A saved OC (Original Character) from the generator
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SavedOC {
    pub text: String,
    pub created_at: i64,
}

impl SavedOC {
    pub fn new(text: String) -> Self {
        Self {
            text,
            created_at: Utc::now().timestamp(),
        }
    }
}

/// Application data manager - coordinates all data operations
/// 
/// This struct acts as a service layer/repository pattern implementation.
/// It handles:
/// - Database operations
/// - Data transformation
/// - Business logic
/// - Caching (future)
/// 
/// Clone is cheap - SqliteDatabase uses Arc internally for the connection pool
#[derive(Clone)]
pub struct AppData {
    db: Option<SqliteDatabase>,
}

impl AppData {
    /// Create a new AppData instance with no database connection
    pub fn new() -> Self {
        Self { db: None }
    }

    /// Set the database connection (called after async initialization)
    pub fn set_database(&mut self, db: SqliteDatabase) {
        self.db = Some(db);
    }

    /// Check if database is available
    pub fn has_database(&self) -> bool {
        self.db.is_some()
    }

    // Project Manager Data Operations

    /// Load all projects with their tags and features
    pub async fn load_projects(&self) -> Result<Vec<ProjectJoin>> {
        match &self.db {
            Some(db) => db.get_all_projects().await,
            None => Err(anyhow::anyhow!("Database not initialized")),
        }
    }

    /// Get a single project by ID with its tags and features
    pub async fn get_project(&self, id: i64) -> Result<ProjectJoin> {
        match &self.db {
            Some(db) => db.get_project_by_id(id).await,
            None => Err(anyhow::anyhow!("Database not initialized")),
        }
    }

    /// Create a new project
    pub async fn create_project(&self, name: String, description: Option<String>) -> Result<Project> {
        match &self.db {
            Some(db) => {
                let mut db = db.clone(); // Cheap clone of the pool
                db.create_project(name, description).await
            }
            None => Err(anyhow::anyhow!("Database not initialized")),
        }
    }
    
    /// Update an existing project
    pub async fn update_project(&self, id: i64, name: String, description: Option<String>) -> Result<Project> {
        match &self.db {
            Some(db) => {
                let mut db = db.clone();
                db.update_project(id, name, description).await
            }
            None => Err(anyhow::anyhow!("Database not initialized")),
        }
    }

    /// Delete a project by ID
    pub async fn delete_project(&self, id: i64) -> Result<()> {
        match &self.db {
            Some(db) => {
                let mut db = db.clone();
                db.delete_project(id).await
            }
            None => Err(anyhow::anyhow!("Database not initialized")),
        }
    }

    /// Create a new tag
    pub async fn create_tags(&self,project_id: ProjectId, names: Vec<String>) -> Result<Vec<Tag>> {
        match &self.db {
            Some(db) => {
                let mut db = db.clone();
                db.create_tags(project_id, names).await
            }
            None => Err(anyhow::anyhow!("Database not initialized")),
        }
    }

    /// Get all available tags
    pub async fn get_all_tags(&self) -> Result<Vec<Tag>> {
        match &self.db {
            Some(db) => db.get_all_tags().await,
            None => Err(anyhow::anyhow!("Database not initialized")),
        }
    }

    /// Add a feature to a project
    pub async fn add_feature(&self, project_id: i64, description: String) -> Result<Feature> {
        match &self.db {
            Some(db) => {
                let mut db = db.clone();
                db.add_feature(project_id, description).await
            }
            None => Err(anyhow::anyhow!("Database not initialized")),
        }
    }
    
    /// Remove tags from a project by name
    pub async fn remove_tags(&self, project_id: i64, tag_names: Vec<String>) -> Result<()> {
        match &self.db {
            Some(db) => {
                let mut db = db.clone();
                db.remove_tags_by_name(project_id, tag_names).await
            }
            None => Err(anyhow::anyhow!("Database not initialized")),
        }
    }
    
    /// Remove features from a project by description
    pub async fn remove_features(&self, project_id: i64, descriptions: Vec<String>) -> Result<()> {
        match &self.db {
            Some(db) => {
                let mut db = db.clone();
                db.remove_features_by_description(project_id, descriptions).await
            }
            None => Err(anyhow::anyhow!("Database not initialized")),
        }
    }
    
    /// Toggle a feature's completed status
    pub async fn toggle_feature_completed(&self, feature_id: i64) -> Result<bool> {
        match &self.db {
            Some(db) => {
                let mut db = db.clone();
                db.toggle_feature_completed(feature_id).await
            }
            None => Err(anyhow::anyhow!("Database not initialized")),
        }
    }

    // Future: Add caching, validation, transformation logic here
    // For example:
    // - Cache frequently accessed projects
    // - Validate project names (no duplicates, length limits)
    // - Transform database models to UI-friendly view models
    // - Batch operations for efficiency
    // - Currently this isn't an issue, but if it were to use an online database, this becomes important
}

impl Default for AppData {
    fn default() -> Self {
        Self::new()
    }
}
