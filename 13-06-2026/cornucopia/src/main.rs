use dotenvy::dotenv;
use std::env;
use tokio_postgres::NoTls;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenv().ok();
    let database_url = env::var("DATABASE_URL")
        .unwrap_or_else(|_| "postgres://postgres@localhost:5432/Cornucopia-learn".to_string());

    println!("Connecting to database...");
    let (client, connection) = tokio_postgres::connect(&database_url, NoTls).await?;

    tokio::spawn(async move {
        if let Err(e) = connection.await {
            eprintln!("Connection error: {}", e);
        }
    });

    client
        .batch_execute(include_str!("../schema/001_app.sql"))
        .await?;

    let project_key = "OPS";

    db::queries::app::delete_project_by_key()
        .bind(&client, &project_key)
        .await?;

    let mut user = db::queries::app::upsert_user()
        .bind(&client, &"alex@example.com", &"Alex Morgan")
        .one()
        .await?;

    println!("user before update: {:?}", user);

    user = db::queries::app::upsert_user()
        .bind(&client, &"alex@example.com", &"Bahi Habash")
        .one()
        .await?;

    println!("user after update: {:?}", user);

    let project = db::queries::app::upsert_project()
        .bind(&client, &user.id, &project_key, &"Operations Dashboard")
        .one()
        .await?;

    let first_task = db::queries::app::create_task()
        .bind(
            &client,
            &project.id,
            &user.id,
            &"Review failed payment alerts",
            &1,
        )
        .one()
        .await?;

    db::queries::app::create_task()
        .bind(
            &client,
            &project.id,
            &user.id,
            &"Prepare weekly incident report",
            &2,
        )
        .one()
        .await?;

    db::queries::app::complete_task()
        .bind(&client, &first_task.id)
        .one()
        .await?;

    let summary = db::queries::app::project_summary()
        .bind(&client, &project.id)
        .one()
        .await?;

    println!(
        "{} - {}: {} total, {} open, {} done",
        summary.key, summary.name, summary.total_tasks, summary.open_tasks, summary.done_tasks
    );

    let tasks = db::queries::app::task_board()
        .bind(&client, &project.id)
        .all()
        .await?;

    for task in tasks {
        println!(
            "[{}] P{} #{} {} ({})",
            task.status, task.priority, task.id, task.title, task.assignee_name
        );
    }

    Ok(())
}
