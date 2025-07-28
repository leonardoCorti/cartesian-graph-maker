use axum::{Json, Router, http::Method, routing::get};
use clap::Parser;
use rand::distr::{Distribution, Uniform};
use serde::Serialize;
use sqlx::sqlite::SqlitePoolOptions;
use tower_http::cors::CorsLayer;

#[derive(Parser)]
struct Args {
    #[arg(long)]
    populate: bool,
}

#[derive(Serialize, sqlx::FromRow)]
struct DataRow {
    name: String,
    x: i32,
    y: i32,
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let args = Args::parse();
    let db_url = "sqlite://points.db";
    let pool = SqlitePoolOptions::new()
        .max_connections(5)
        .connect(db_url)
        .await?;

    let cors = CorsLayer::new()
        .allow_origin(tower_http::cors::Any)
        .allow_methods([Method::GET, Method::POST]);

    sqlx::query(
        "CREATE TABLE IF NOT EXISTS points (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            name TEXT NOT NULL,
            x INTEGER NOT NULL,
            y INTEGER NOT NULL
        )",
    )
    .execute(&pool)
    .await?;

    if args.populate {
        println!("Populating DB with random data...");
        populate_db(&pool).await?;
    }

    let app = Router::new().route("/all", get(get_all)).layer(cors);

    let listener = tokio::net::TcpListener::bind("0.0.0.0:5322").await?;
    println!("Server running at http://0.0.0.0:5322/");

    axum::serve(listener, app).await?;

    Ok(())
}

async fn get_all() -> Json<Vec<DataRow>> {
    print!("ao");
    let pool = sqlx::sqlite::SqlitePoolOptions::new()
        .connect("sqlite://points.db")
        .await
        .unwrap();

    fetch_all(pool).await
}

async fn fetch_all(pool: sqlx::SqlitePool) -> Json<Vec<DataRow>> {
    let rows = sqlx::query_as::<_, DataRow>("SELECT name, x, y FROM points")
        .fetch_all(&pool)
        .await
        .unwrap_or_default();
    Json(rows)
}

async fn populate_db(pool: &sqlx::SqlitePool) -> anyhow::Result<()> {
    const NUM_POINTS: usize = 20;
    let between = Uniform::try_from(-100..=100)?;
    let mut rng = rand::rng();

    let mut tx = pool.begin().await?;
    for i in 0..NUM_POINTS {
        let name = format!("Point{}", i);
        let x = between.sample(&mut rng);
        let y = between.sample(&mut rng);
        sqlx::query("INSERT INTO points (name, x, y) VALUES (?, ?, ?)")
            .bind(name)
            .bind(x)
            .bind(y)
            .execute(&mut *tx)
            .await?;
    }
    tx.commit().await?;
    Ok(())
}
