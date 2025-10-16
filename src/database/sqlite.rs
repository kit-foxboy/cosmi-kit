use sqlx::{migrate::MigrateDatabase, Sqlite, SqlitePool};
use anyhow::Result;

use crate::database::{ProjectDatabase, ProjectId, ProjectJoin};
use super::{Project, Tag, Feature};

/// Generate a random pastel color in hex format
/// Using HSL color space: H=random, S=70%, L=75% for nice pastel colors
fn generate_random_color() -> String {
    let hue: u16 = fastrand::u16(0..360);
    
    // Convert HSL to RGB for a nice pastel color
    // S=0.7 (70% saturation), L=0.75 (75% lightness)
    let (r, g, b) = hsl_to_rgb(hue as f32 / 360.0, 0.7, 0.75);
    
    format!("#{:02X}{:02X}{:02X}", r, g, b)
}

/// Convert HSL to RGB
fn hsl_to_rgb(h: f32, s: f32, l: f32) -> (u8, u8, u8) {
    let c = (1.0 - (2.0 * l - 1.0).abs()) * s;
    let x = c * (1.0 - ((h * 6.0) % 2.0 - 1.0).abs());
    let m = l - c / 2.0;
    
    let (r, g, b) = match (h * 6.0) as u8 {
        0 => (c, x, 0.0),
        1 => (x, c, 0.0),
        2 => (0.0, c, x),
        3 => (0.0, x, c),
        4 => (x, 0.0, c),
        _ => (c, 0.0, x),
    };
    
    (
        ((r + m) * 255.0) as u8,
        ((g + m) * 255.0) as u8,
        ((b + m) * 255.0) as u8,
    )
}

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
    async fn get_all_projects(&self) -> Result<Vec<ProjectJoin>> {
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
        let mut result = vec![];
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
            
            let mut project_join = ProjectJoin::new();
            project_join.insert(project.id, (project.clone(), tags.clone(), features.clone()));
            result.push(project_join);
        }
        
        Ok(result)
    }
    
    async fn get_project_by_id(&self, id: i64) -> Result<ProjectJoin> {
        // Get the project
        let project = sqlx::query_as::<_, Project>(
            "SELECT id, name, description, created_at, updated_at FROM projects WHERE id = ?"
        )
        .bind(id)
        .fetch_one(&self.pool)
        .await?;
        
        // Get tags for this project
        let tags = sqlx::query_as::<_, Tag>(
            r#"
            SELECT t.id, t.name, t.color, t.created_at
            FROM tags t
            INNER JOIN project_tags pt ON t.id = pt.tag_id
            WHERE pt.project_id = ?
            "#
        )
        .bind(id)
        .fetch_all(&self.pool)
        .await?;
        
        // Get features for this project
        let features = sqlx::query_as::<_, Feature>(
            "SELECT id, project_id, description, completed, created_at 
             FROM features 
             WHERE project_id = ?
             ORDER BY created_at DESC"
        )
        .bind(id)
        .fetch_all(&self.pool)
        .await?;
        
        let mut project_join = ProjectJoin::new();
        project_join.insert(id, (project, tags, features));
        
        Ok(project_join)
    }
    
    async fn update_project(&mut self, id: i64, name: String, description: Option<String>) -> Result<Project> {
        // Update the project
        sqlx::query(
            "UPDATE projects SET name = ?, description = ?, updated_at = CURRENT_TIMESTAMP WHERE id = ?"
        )
        .bind(&name)
        .bind(&description)
        .bind(id)
        .execute(&self.pool)
        .await?;
        
        // Fetch and return the updated project
        let project = sqlx::query_as::<_, Project>(
            "SELECT id, name, description, created_at, updated_at FROM projects WHERE id = ?"
        )
        .bind(id)
        .fetch_one(&self.pool)
        .await?;
        
        Ok(project)
    }
    
    async fn delete_project(&mut self, _id: i64) -> Result<()> {
        let _result = sqlx::query(
            "DELETE FROM projects WHERE id = ?"
        )
        .bind(_id)
        .execute(&self.pool)
        .await?;

        Ok(())
    }

    async fn create_tags(&mut self, project_id: ProjectId, names: Vec<String>) -> Result<Vec<Tag>> {
        // Use a transaction to ensure atomicity
        let mut tx = self.pool.begin().await?;
        
        let mut tags = Vec::new();
        for name in names {
            // Check if tag already exists (tags are reusable across projects)
            let existing_tag: Option<Tag> = sqlx::query_as::<_, Tag>(
                "SELECT id, name, color FROM tags WHERE name = ?"
            )
            .bind(&name)
            .fetch_optional(&mut *tx)
            .await?;
            
            let tag = if let Some(tag) = existing_tag {
                // Tag exists, just link it to the project
                tag
            } else {
                // Create new tag with random color
                let color = generate_random_color();
                sqlx::query_as::<_, Tag>(
                    "INSERT INTO tags (name, color) VALUES (?, ?) RETURNING id, name, color"
                )
                .bind(&name)
                .bind(&color)
                .fetch_one(&mut *tx)
                .await?
            };
            
            // Check if this tag is already linked to the project
            let existing_link: Option<(i64,)> = sqlx::query_as(
                "SELECT project_id FROM project_tags WHERE project_id = ? AND tag_id = ?"
            )
            .bind(project_id)
            .bind(tag.id)
            .fetch_optional(&mut *tx)
            .await?;
            
            // Only insert the link if it doesn't already exist
            if existing_link.is_none() {
                sqlx::query(
                    "INSERT INTO project_tags (project_id, tag_id) VALUES (?, ?)"
                )
                .bind(project_id)
                .bind(tag.id)
                .execute(&mut *tx)
                .await?;
            }
            
            tags.push(tag);
        }
        
        // Commit transaction
        tx.commit().await?;
        Ok(tags)
    }
    async fn get_all_tags(&self) -> Result<Vec<Tag>> {
        let tags = sqlx::query_as::<_, Tag>(
            "SELECT id, name, color FROM tags ORDER BY name ASC"
        )
        .fetch_all(&self.pool)
        .await?;
        
        Ok(tags)
    }
    
    async fn remove_tags_by_name(&mut self, project_id: i64, tag_names: Vec<String>) -> Result<()> {
        let mut tx = self.pool.begin().await?;
        
        for tag_name in tag_names {
            // Find the tag by name
            let tag: Option<Tag> = sqlx::query_as::<_, Tag>(
                "SELECT id, name, color FROM tags WHERE name = ?"
            )
            .bind(&tag_name)
            .fetch_optional(&mut *tx)
            .await?;
            
            if let Some(tag) = tag {
                // Remove the link between project and tag
                sqlx::query("DELETE FROM project_tags WHERE project_id = ? AND tag_id = ?")
                    .bind(project_id)
                    .bind(tag.id)
                    .execute(&mut *tx)
                    .await?;
            }
        }
        
        tx.commit().await?;
        Ok(())
    }

    async fn add_feature(&mut self, project_id: i64, description: String) -> Result<Feature> {
        let feature = sqlx::query_as::<_, Feature>(
            "INSERT INTO features (project_id, description, completed) VALUES (?, ?, 0) 
             RETURNING id, project_id, description, completed, created_at"
        )
        .bind(project_id)
        .bind(description)
        .fetch_one(&self.pool)
        .await?;
        
        Ok(feature)
    }
    
    async fn remove_features_by_description(&mut self, project_id: i64, descriptions: Vec<String>) -> Result<()> {
        let mut tx = self.pool.begin().await?;
        
        for description in descriptions {
            // Delete features matching the project_id and description
            sqlx::query("DELETE FROM features WHERE project_id = ? AND description = ?")
                .bind(project_id)
                .bind(&description)
                .execute(&mut *tx)
                .await?;
        }
        
        tx.commit().await?;
        Ok(())
    }
    
    async fn toggle_feature_completed(&mut self, feature_id: i64) -> Result<bool> {
        // Toggle the completed status (flip from 0 to 1 or 1 to 0)
        sqlx::query(
            "UPDATE features SET completed = NOT completed WHERE id = ?"
        )
        .bind(feature_id)
        .execute(&self.pool)
        .await?;
        
        // Fetch and return the new completed status
        let feature: Feature = sqlx::query_as(
            "SELECT id, project_id, description, completed, created_at FROM features WHERE id = ?"
        )
        .bind(feature_id)
        .fetch_one(&self.pool)
        .await?;
        
        Ok(feature.completed)
    }
}