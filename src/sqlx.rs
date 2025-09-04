use axum::{
    extract::{Path, State},
    http::StatusCode,
    response::Json,
    routing::{delete, get, post, put},
    Router,
};
use sqlx::{postgres::PgPoolOptions, PgPool, FromRow};
use serde::{Deserialize, Serialize};

// 数据模型
#[derive(Debug, Serialize, Deserialize, FromRow)]
struct User {
    id: Option<i32>,
    name: String,
    email: String,
    created_at: i64,
}

// 数据库操作
#[derive(Clone)]
struct UserRepository {
    pool: PgPool,
}

impl UserRepository {
    fn new(pool: PgPool) -> Self {
        Self { pool }
    }
    
    async fn create_user(&self, user: &User) -> Result<User, sqlx::Error> {
        let row = sqlx::query_as::<_, User>(
            r#"
            INSERT INTO users (name, email, created_at)
            VALUES ($1, $2, $3)
            RETURNING id, name, email, created_at
            "#
        )
        .bind(&user.name)
        .bind(&user.email)
        .bind(&user.created_at)
        .fetch_one(&self.pool)
        .await?;
        
        Ok(row)
    }
    
    async fn get_user(&self, id: i32) -> Result<Option<User>, sqlx::Error> {
        let user = sqlx::query_as::<_, User>(
            r#"
            SELECT id, name, email, created_at
            FROM users
            WHERE id = $1
            "#
        )
        .bind(id)
        .fetch_optional(&self.pool)
        .await?;
        
        Ok(user)
    }
    
    async fn get_all_users(&self) -> Result<Vec<User>, sqlx::Error> {
        let users = sqlx::query_as::<_, User>(
            r#"
            SELECT id, name, email, created_at
            FROM users
            ORDER BY created_at DESC
            "#
        )
        .fetch_all(&self.pool)
        .await?;
        
        Ok(users)
    }
    
    async fn update_user(&self, id: i32, user: &User) -> Result<Option<User>, sqlx::Error> {
        let updated_user = sqlx::query_as::<_, User>(
            r#"
            UPDATE users
            SET name = $1, email = $2
            WHERE id = $3
            RETURNING id, name, email, created_at
            "#
        )
        .bind(&user.name)
        .bind(&user.email)
        .bind(id)
        .fetch_optional(&self.pool)
        .await?;
        
        Ok(updated_user)
    }
    
    async fn delete_user(&self, id: i32) -> Result<bool, sqlx::Error> {
        let result = sqlx::query(
            r#"
            DELETE FROM users
            WHERE id = $1
            "#
        )
        .bind(id)
        .execute(&self.pool)
        .await?;
        
        Ok(result.rows_affected() > 0)
    }
}

// 应用状态
#[derive(Clone)]
struct AppState {
    user_repo: UserRepository,
}

// 路由处理函数
async fn create_user_handler(
    State(state): State<AppState>,
    Json(user): Json<User>,
) -> Result<Json<User>, StatusCode> {
    println!("user: {:?}", user);
    let created_user = state.user_repo
        .create_user(&user)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
    
    println!("created_user: {:?}", created_user);

    Ok(Json(created_user))
}

async fn get_user_handler(
    State(state): State<AppState>,
    Path(id): Path<i32>,
) -> Result<Json<User>, StatusCode> {
    let user = state.user_repo
        .get_user(id)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?
        .ok_or(StatusCode::NOT_FOUND)?;
    
    Ok(Json(user))
}

async fn get_users_handler(
    State(state): State<AppState>,
) -> Result<Json<Vec<User>>, StatusCode> {
    let users = state.user_repo
        .get_all_users()
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
    
    Ok(Json(users))
}

async fn update_user_handler(
    State(state): State<AppState>,
    Path(id): Path<i32>,
    Json(user): Json<User>,
) -> Result<Json<User>, StatusCode> {
    let updated_user = state.user_repo
        .update_user(id, &user)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?
        .ok_or(StatusCode::NOT_FOUND)?;
    
    Ok(Json(updated_user))
}

async fn delete_user_handler(
    State(state): State<AppState>,
    Path(id): Path<i32>,
) -> Result<StatusCode, StatusCode> {
    let deleted = state.user_repo
        .delete_user(id)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
    
    if deleted {
        Ok(StatusCode::NO_CONTENT)
    } else {
        Ok(StatusCode::NOT_FOUND)
    }
}

// 数据库初始化
async fn init_database(pool: &PgPool) -> Result<(), sqlx::Error> {
    sqlx::query(
        r#"
        CREATE TABLE IF NOT EXISTS users (
            id SERIAL PRIMARY KEY,
            name VARCHAR(255) NOT NULL,
            email VARCHAR(255) UNIQUE NOT NULL,
            created_at TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP
        )
        "#
    )
    .execute(pool)
    .await?;
    
    Ok(())
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // 数据库连接
    let database_url = std::env::var("DATABASE_URL")
        .unwrap_or_else(|_| "postgresql://admin:123456@localhost:5433/rust_web".to_string());
    
    let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect(&database_url)
        .await?;
    
    // 初始化数据库
    init_database(&pool).await?;
    
    // 创建应用状态
    let user_repo = UserRepository::new(pool);
    let state = AppState { user_repo };
    
    // 创建路由
    let app = Router::new()
        .route("/api/users", post(create_user_handler))
        .route("/api/users", get(get_users_handler))
        .route("/api/users/{id}", get(get_user_handler))
        .route("/api/users/{id}", put(update_user_handler))
        .route("/api/users/{id}", delete(delete_user_handler))
        .with_state(state);
    
    println!("服务器运行在 http://localhost:3000");
    
    let listener = tokio::net::TcpListener::bind("127.0.0.1:3000")
        .await
        .unwrap();
    
    axum::serve(listener, app).await.unwrap();
    
    Ok(())
}
