use sqlx::{migrate::MigrateDatabase, Sqlite, SqlitePool};
use anyhow::Result;

use crate::database::{ProjectDatabase, ProjectFeatures, ProjectTags};
use super::{Project, Tag, Feature};

// Database versioning - increment when schema changes
const DB_VERSION: &str = "1";

// Using constcat to create compile-time constant path (following clipboard manager pattern)
const DB_FILENAME: &str = constcat::concat!("com.github.kitfoxboy.cosmi-kit-projects-v", DB_VERSION, ".db");

/// SqliteDatabase wraps a connection pool for efficient reuse
/// Clone is cheap - it just increments a reference count to the shared pool
#[derive(Clone, Debug)]
pub struct SqliteDatabase {
    pool: SqlitePool,
}

impl SqliteDatabase {
    /// Initialize database connection pool in the COSMIC data directory
    /// 
    /// Returns a SqliteDatabase that can be cloned cheaply - cloning just
    /// increments a reference count to the shared connection pool.
    /// This means you can pass it around without creating new connections!
    pub async fn new() -> Result<Self> {
        // Get the COSMIC data directory (follows XDG Base Directory spec)
        // This is typically ~/.local/share/cosmic/com.github.kitfoxboy.cosmi-kit/
        let data_dir = dirs::data_local_dir()
            .ok_or_else(|| anyhow::anyhow!("Could not find data directory"))?
            .join("cosmic")
            .join("com.github.kitfoxboy.cosmi-kit");
        
        // Create the directory if it doesn't exist
        std::fs::create_dir_all(&data_dir)?;
        
        // Full path to database file
        let db_path = data_dir.join(DB_FILENAME);
        let db_path_str = db_path.to_str()
            .ok_or_else(|| anyhow::anyhow!("Invalid database path"))?;
        
        // Create database if it doesn't exist
        if !Sqlite::database_exists(db_path_str).await? {
            eprintln!("Creating database at: {}", db_path_str);
            Sqlite::create_database(db_path_str).await?;
        }
        
        // Create connection pool with URL format "sqlite:///path/to/file.db"
        let db_url = format!("sqlite://{}", db_path_str);
        let pool = SqlitePool::connect(&db_url).await?;
        
        // Run migrations - this is idempotent (safe to run multiple times)
        // sqlx will track which migrations have been applied in a table called _sqlx_migrations
        sqlx::migrate!("./migrations")
            .run(&pool)
            .await?;
        
        println!("Database migrations completed successfully");
        
        Ok(Self { pool })
    }
}

#[async_trait::async_trait]
impl ProjectDatabase for SqliteDatabase {
    async fn create_project(&mut self, name: String, description: Option<String>) -> Result<Project> {
        // Insert the project and get the ID
        let result = sqlx::query(
            "INSERT INTO projects (name, description) VALUES (?, ?)"
        )
        .bind(&name)
        .bind(&description)
        .execute(&self.pool)
        .await?;
        
        let project_id = result.last_insert_rowid();
        
        // Fetch the created project to return it
        let project = sqlx::query_as::<_, Project>(
            "SELECT id, name, description, created_at FROM projects WHERE id = ?"
        )
        .bind(project_id)
        .fetch_one(&self.pool)
        .await?;
        
        Ok(project)
    }
    async fn get_all_projects(&self) -> Result<Vec<(Project, ProjectTags, ProjectFeatures)>> {
        // First, get all projects
        // on a larger dataset, I'd join tags and features in a single query
        // returning a flattened structure and then group in Rust
        // but for simplicity and small datasets of many small projects, this is fine
        let projects = sqlx::query_as::<_, Project>(
            "SELECT id, name, description, created_at FROM projects ORDER BY created_at DESC"
        )
        .fetch_all(&self.pool)
        .await?;
        
        // For each project, fetch its tags and features
        let mut result = Vec::new();
        for project in projects {
            // Get tags for this project
            let tags = sqlx::query_as::<_, Tag>(
                r#"
                SELECT t.id, t.name, t.color
                FROM tags t
                INNER JOIN project_tags pt ON t.id = pt.tag_id
                WHERE pt.project_id = ?
                "#
            )
            .bind(project.id)
            .fetch_all(&self.pool)
            .await?;
            
            // Get features for this project
            let features = sqlx::query_as::<_, Feature>(
                "SELECT id, project_id, description, completed, created_at 
                 FROM features 
                 WHERE project_id = ?
                 ORDER BY created_at DESC"
            )
            .bind(project.id)
            .fetch_all(&self.pool)
            .await?;
            
            result.push((project, tags, features));
        }
        
        Ok(result)
    }
    async fn delete_project(&mut self, id: i64) -> Result<()> {
        todo!("Implement delete_project")
    }

    async fn create_tag(&mut self, name: String) -> Result<Tag> {
        todo!("Implement create_tag")
    }
    async fn get_all_tags(&self) -> Result<Vec<Tag>> {
        todo!("Implement get_all_tags")
    }
    async fn get_project_tags(&self, project_id: i64) -> Result<Vec<Tag>> {
        todo!("Implement get_project_tags")
    }
    async fn add_tag_to_project(&mut self, project_id: i64, tag_id: i64) -> Result<()> {
        todo!("Implement add_tag_to_project")
    }
    async fn remove_tag_from_project(&mut self, project_id: i64, tag_id: i64) -> Result<()> {
        todo!("Implement remove_tag_from_project")
    }

    async fn add_feature(&mut self, project_id: i64, description: String) -> Result<Feature> {
        todo!("Implement add_feature")
    }
    async fn get_project_features(&self, project_id: i64) -> Result<Vec<Feature>> {
        todo!("Implement get_project_features")
    }
    async fn remove_feature(&mut self, feature_id: i64) -> Result<()> {
        todo!("Implement remove_feature")
    }
}