use sqlx::sqlite::SqlitePool;
use sqlx::sqlite::SqlitePoolOptions;

// Create a connection pool to the SQLite database
pub async fn create_pool() -> Result<SqlitePool, sqlx::Error> {
 let pool = SqlitePoolOptions::new()
     .max_connections(5)
     .connect("sqlite:Measure_Tool.db")
     .await?;

 Ok(pool)
}

// Initialize the database with tables
pub async fn initialize_database(pool: &SqlitePool) -> Result<(), sqlx::Error> {
 // Create projects table
 sqlx::query(
  r#"
        CREATE TABLE IF NOT EXISTS projects (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            name TEXT NOT NULL,
            address TEXT NOT NULL,
            phone TEXT,
            email TEXT,
            number_of_floors INTEGER,
            interior_substrate TEXT,
            exterior_substrate TEXT,
            comments TEXT,
            created_at DATETIME DEFAULT CURRENT_TIMESTAMP,
            updated_at DATETIME DEFAULT CURRENT_TIMESTAMP
        )
        "#
 )
     .execute(pool)
     .await?;

 Ok(())
}

