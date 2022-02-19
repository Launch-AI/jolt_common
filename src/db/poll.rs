use sqlx::postgres;

pub async fn pool(url: &String) -> Result<postgres::PgPool, sqlx::Error> {
    //    connect to the database
    let pool = postgres::PgPoolOptions::new()
        .max_connections(32)
        .connect(&url)
        .await?;

    // Return Result Pool
    Ok(pool)
}
