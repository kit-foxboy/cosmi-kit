// SPDX-License-Identifier: MPL-2.0

//! Application data layer - handles all data operations
//! 
//! This module acts as a repository/service layer between the UI and the database.
//! Pages emit messages, AppModel delegates to AppData, AppData handles the actual work.

use crate::database::{Feature, Project, ProjectDatabase, ProjectJoin, SqliteDatabase, Tag};
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
    pub async fn create_tag(&self, name: String) -> Result<Tag> {
        match &self.db {
            Some(db) => {
                let mut db = db.clone();
                db.create_tag(name).await
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

    /// Add a tag to a project
    pub async fn add_tag_to_project(&self, project_id: i64, tag_id: i64) -> Result<()> {
        match &self.db {
            Some(db) => {
                let mut db = db.clone();
                db.add_tag_to_project(project_id, tag_id).await
            }
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

    /// Get all features for a project
    pub async fn get_project_features(&self, project_id: i64) -> Result<Vec<Feature>> {
        match &self.db {
            Some(db) => db.get_project_features(project_id).await,
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
