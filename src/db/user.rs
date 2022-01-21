use crate::db::get_db_pool;
use crate::models::user::{User, UserConditions};

pub async fn find_all(conditions: UserConditions) -> Result<Vec<User>, sqlx::Error> {
    let pool = get_db_pool().await;
    let mut query = sqlx::query_as::<_, User>("select * from users");
    if let Some(name) = conditions.name {
        query = sqlx::query_as::<_, User>("select * from users where name = $1").bind(name)
    }
    query.fetch_all(pool).await
}
