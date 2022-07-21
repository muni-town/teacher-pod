use sqlx::{Pool, Postgres};

pub fn schedule_task(task_pool: Pool<Postgres>) {
    tokio::task::spawn(async move {
        let pool = task_pool;
        loop {
            let sql = format!(
                "delete from auth where expire <= {};",
                chrono::Local::now().timestamp()
            );
            let r = sqlx::query(&sql).execute(&pool).await;
            if let Err(e) = r {
                log::warn!("clean auth data failed: {}", e.to_string());
            }
            // two day task
            tokio::time::sleep(std::time::Duration::from_secs(60 * 60 * 24 * 5)).await;
        }
    });
}
