# Database Migrations with sqlx

## Overview

We're using **sqlx's built-in migration system** for managing database schema changes. This is a compile-time checked, type-safe approach that's perfect for SQLite.

## How It Works

### 1. Migration Files

Migrations live in `migrations/` directory with this naming convention:
```
migrations/
  └── {timestamp}_{description}.sql
      └── 20250107000001_initial_schema.sql
```

**Naming Pattern:**
- `YYYYMMDDHHMMSS` timestamp (ensures ordering)
- Underscore separator
- Descriptive name
- `.sql` extension

### 2. What Happens When Migrations Run

```rust
sqlx::migrate!("./migrations")
    .run(&pool)
    .await?;
```

1. sqlx creates a `_sqlx_migrations` table in your database
2. Tracks which migrations have been applied
3. Runs any new migrations in order
4. **Idempotent** - safe to run multiple times!

### 3. Migration Tracking

Our database will have:
```sql
CREATE TABLE _sqlx_migrations (
    version BIGINT PRIMARY KEY,
    description TEXT NOT NULL,
    installed_on TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
    success BOOLEAN NOT NULL,
    checksum BLOB NOT NULL,
    execution_time BIGINT NOT NULL
);
```

## Creating New Migrations

### Manual Approach (What We're Doing)

```bash
# Create new migration file
touch migrations/20250107120000_add_priority_to_projects.sql
```

```sql
-- Add priority column
ALTER TABLE projects ADD COLUMN priority INTEGER DEFAULT 0;

-- Create index
CREATE INDEX idx_projects_priority ON projects(priority);
```

### CLI Approach (Optional)

If you install sqlx-cli:
```bash
# Install sqlx CLI
cargo install sqlx-cli --no-default-features --features sqlite

# Create new migration
sqlx migrate add add_priority_to_projects

# This creates: migrations/{timestamp}_add_priority_to_projects.sql
```

## Migration Best Practices

### ✅ DO:

1. **One logical change per migration**
   ```sql
   -- Good: Single purpose
   -- 20250107120000_add_user_authentication.sql
   CREATE TABLE users (...);
   CREATE INDEX idx_users_email ON users(email);
   ```

2. **Use IF NOT EXISTS for safety**
   ```sql
   CREATE TABLE IF NOT EXISTS projects (...);
   CREATE INDEX IF NOT EXISTS idx_projects_name ON projects(name);
   ```

3. **Add indexes for foreign keys**
   ```sql
   CREATE INDEX idx_features_project ON features(project_id);
   CREATE INDEX idx_project_tags_project ON project_tags(project_id);
   ```

4. **Use meaningful names**
   ```sql
   -- Good
   20250107120000_add_project_priorities.sql
   
   -- Bad
   20250107120000_update.sql
   ```

### ❌ DON'T:

1. **Don't modify existing migrations**
   - Once deployed, migrations are immutable
   - Create a new migration to fix issues
   - It's tempting but bad practice
   - It also serves as a historical record of changes

2. **Don't use transactions manually**
   - sqlx wraps each migration in a transaction automatically

3. **Don't put data changes and schema changes together**
   ```sql
   -- Bad
   CREATE TABLE users (...);
   INSERT INTO users VALUES (...);  -- Data modification!
   
   -- Good: Separate migrations
   -- 001_create_users_table.sql
   -- 002_seed_admin_user.sql
   ```

## Example Migrations

### Adding a Column
```sql
-- migrations/20250107120000_add_project_status.sql
ALTER TABLE projects ADD COLUMN status TEXT DEFAULT 'active';
CREATE INDEX idx_projects_status ON projects(status);
```

### Creating a New Table
```sql
-- migrations/20250107130000_add_comments.sql
CREATE TABLE comments (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    project_id INTEGER NOT NULL,
    user_name TEXT NOT NULL,
    content TEXT NOT NULL,
    created_at INTEGER NOT NULL DEFAULT (strftime('%s', 'now')),
    FOREIGN KEY (project_id) REFERENCES projects(id) ON DELETE CASCADE
);

CREATE INDEX idx_comments_project ON comments(project_id);
CREATE INDEX idx_comments_created_at ON comments(created_at);
```

### Modifying Existing Data
```sql
-- migrations/20250107140000_normalize_tag_names.sql
-- SQLite doesn't support direct column modifications, so we:
-- 1. Create new column
-- 2. Copy data with transformation
-- 3. Drop old column
-- 4. Rename new column

ALTER TABLE tags ADD COLUMN name_normalized TEXT;
UPDATE tags SET name_normalized = LOWER(TRIM(name));
ALTER TABLE tags DROP COLUMN name;
ALTER TABLE tags RENAME COLUMN name_normalized TO name;
```

## SQLite-Specific Notes

### Data Types
SQLite has flexible typing. Common mappings:
- `INTEGER` → i64, i32, bool (0/1)
- `TEXT` → String
- `REAL` → f64
- `BLOB` → Vec<u8>

### Timestamps
We use Unix epoch (seconds since 1970):
```sql
created_at INTEGER NOT NULL DEFAULT (strftime('%s', 'now'))
```

Rust side:
```rust
use chrono::{DateTime, Utc, NaiveDateTime};

let timestamp: i64 = 1704672000;
let dt = NaiveDateTime::from_timestamp_opt(timestamp, 0)
    .unwrap()
    .and_utc();
```

### Boolean Values
SQLite doesn't have native boolean, use INTEGER:
```sql
completed INTEGER NOT NULL DEFAULT 0  -- 0 = false, 1 = true
```

Rust side (sqlx handles this automatically):
```rust
#[derive(sqlx::FromRow)]
struct Feature {
    completed: bool,  // sqlx converts 0/1 to bool
}
```

### Foreign Keys
Must be enabled per connection (sqlx does this automatically with pools):
```sql
PRAGMA foreign_keys = ON;
```

## Querying in Rust

### Simple Query
```rust
let projects = sqlx::query_as::<_, Project>(
    "SELECT id, name, description, created_at FROM projects"
)
.fetch_all(&self.pool)
.await?;
```

### Query with Parameters
SQL Injection not a problem with query binding, similar to prepared statements or PDO in PHP:
```rust
let project = sqlx::query_as::<_, Project>(
    "SELECT * FROM projects WHERE id = ?"
)
.bind(project_id)
.fetch_one(&self.pool)
.await?;
```

### Insert and Get ID
```rust
let result = sqlx::query(
    "INSERT INTO projects (name, description) VALUES (?, ?)"
)
.bind(&name)
.bind(&description)
.execute(&self.pool)
.await?;

let project_id = result.last_insert_rowid();
```

### Complex Joins
```rust
let project_data = sqlx::query!(
    r#"
    SELECT 
        p.id, p.name, p.description,
        t.id as tag_id, t.name as tag_name,
        f.id as feature_id, f.description as feature_description
    FROM projects p
    LEFT JOIN project_tags pt ON p.id = pt.project_id
    LEFT JOIN tags t ON pt.tag_id = t.id
    LEFT JOIN features f ON p.id = f.project_id
    WHERE p.id = ?
    "#,
    project_id
)
.fetch_all(&self.pool)
.await?;
```

## Troubleshooting

### Migration Failed
```
Error: migration 20250107120000 failed
```
**Solution:**
1. Fix the SQL in the migration file. I once had a comma screw this up X3
2. Delete the database file (during development obviously)
3. Restart the app (migrations run on first connection)

### Database Locked
```
Error: database is locked
```
**Solution:**
- SQLite only allows one writer at a time
- Use connection pool (we already do this)
- Keep transactions short

### Schema Mismatch
```
Error: no such table: projects
```
**Solution:**
- Ensure migrations ran: check for `_sqlx_migrations` table
- Check database file path
- Verify migrations/ directory is included in build

## Rolling Back (Development Only)

sqlx alas doesn't have automatic rollback. For development:

```bash
# Delete database and restart
rm ~/.local/share/cosmic/com.github.kitfoxboy.cosmi-kit/*.db

# Or manually with sqlite3
sqlite3 database.db
sqlite> DROP TABLE _sqlx_migrations;
sqlite> DROP TABLE projects;
# ... etc
```

**Production:** Never delete migrations. Create new "down" migrations:
```sql
-- migrations/20250107150000_remove_priority_column.sql
ALTER TABLE projects DROP COLUMN priority;
```

## Next Steps

1. ✅ Migrations are set up and run automatically
2. Implement CRUD methods in `sqlite.rs`
3. Use `sqlx::query!` macro for compile-time SQL checking
4. Add more migrations as features evolve

## Resources

- [sqlx Book](https://github.com/launchbadge/sqlx)
- [SQLite Documentation](https://www.sqlite.org/docs.html)
- [SQL Tutorial](https://www.sqlitetutorial.net/)


## Thoughts on Async with Database Operations
Database operations can be slow, so we should run them asynchronously to avoid blocking the UI thread. Using `cosmic::Task::perform`, we can offload database queries to a background thread and handle results in the UI thread. I prefer querying with `sqlx::query!` macro for type safety and compile-time checks a lot more than most ORMs I've used in other languages. It's reminiscent of using prepared statements in PHP with PDO, which is a good thing imho. There's room for improvement in error handling and user feedback during long operations, but this is a solid foundation. I'd also like to use generics to reduce boilerplate in CRUD methods, but that can come later once I've actually built out some functionality. When learning, I prefer to possibly repeat myself a bit rather than abstract too early and make things more complex than they need to be. I've found that refactoring opportunities naturally arise as features are added.