use axum::{
    extract::{Path, State},
    http::StatusCode,
    response::Json,
    routing::{get, post, put, delete},
    Router,
};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::RwLock;

// 数据模型
#[derive(Debug, Serialize, Deserialize, Clone)]
struct User {
    id: u32,
    name: String,
    email: String,
}

// 应用状态
#[derive(Clone)]
struct AppState {
    users: Arc<RwLock<HashMap<u32, User>>>,
}

impl AppState {
    fn new() -> Self {
        let mut users = HashMap::new();
        users.insert(1, User {
            id: 1,
            name: String::from("Alice"),
            email: String::from("alice@example.com"),
        });
        users.insert(2, User {
            id: 2,
            name: String::from("Bob"),
            email: String::from("bob@example.com"),
        });
        
        AppState {
            users: Arc::new(RwLock::new(users)),
        }
    }
}

// 请求和响应类型
#[derive(Debug, Deserialize)]
struct CreateUserRequest {
    name: String,
    email: String,
}

#[derive(Debug, Deserialize)]
struct UpdateUserRequest {
    name: Option<String>,
    email: Option<String>,
}

// 路由处理函数
async fn root() -> Json<serde_json::Value> {
    Json(serde_json::json!({
        "message": "Hello from Axum!"
    }))
}

async fn get_users(State(state): State<AppState>) -> Json<Vec<User>> {
    let users = state.users.read().await;
    let user_list: Vec<User> = users.values().cloned().collect();
    Json(user_list)
}

async fn get_user(
    State(state): State<AppState>,
    Path(id): Path<u32>,
) -> Result<Json<User>, StatusCode> {
    let users = state.users.read().await;
    
    if let Some(user) = users.get(&id) {
        Ok(Json(user.clone()))
    } else {
        Err(StatusCode::NOT_FOUND)
    }
}

async fn create_user(
    State(state): State<AppState>,
    Json(payload): Json<CreateUserRequest>,
) -> Result<Json<User>, StatusCode> {
    if payload.name.is_empty() || payload.email.is_empty() {
        return Err(StatusCode::BAD_REQUEST);
    }
    
    let mut users = state.users.write().await;
    let new_id = users.keys().max().unwrap_or(&0) + 1;
    
    let new_user = User {
        id: new_id,
        name: payload.name,
        email: payload.email,
    };
    
    users.insert(new_id, new_user.clone());
    Ok(Json(new_user))
}

async fn update_user(
    State(state): State<AppState>,
    Path(id): Path<u32>,
    Json(payload): Json<UpdateUserRequest>,
) -> Result<Json<User>, StatusCode> {
    let mut users = state.users.write().await;
    
    if let Some(user) = users.get_mut(&id) {
        if let Some(name) = payload.name {
            user.name = name;
        }
        if let Some(email) = payload.email {
            user.email = email;
        }
        Ok(Json(user.clone()))
    } else {
        Err(StatusCode::NOT_FOUND)
    }
}

async fn delete_user(
    State(state): State<AppState>,
    Path(id): Path<u32>,
) -> StatusCode {
    let mut users = state.users.write().await;
    
    if users.remove(&id).is_some() {
        StatusCode::NO_CONTENT
    } else {
        StatusCode::NOT_FOUND
    }
}

// 异步数据路由
async fn async_data() -> Json<serde_json::Value> {
    // 模拟异步操作
    tokio::time::sleep(tokio::time::Duration::from_secs(1)).await;
    
    Json(serde_json::json!({
        "message": "异步数据",
        "timestamp": chrono::Utc::now().timestamp()
    }))
}

// 认证中间件
// async fn auth_middleware(
//     headers: axum::http::HeaderMap,
// ) -> Result<(), StatusCode> {
//     if let Some(auth_header) = headers.get("authorization") {
//         if auth_header.to_str().unwrap_or("").starts_with("Bearer ") {
//             return Ok(());
//         }
//     }
//     Err(StatusCode::UNAUTHORIZED)
// }

async fn protected_route() -> Json<serde_json::Value> {
    Json(serde_json::json!({
        "message": "受保护的路由",
        "user": {
            "id": 1,
            "name": "Authenticated User"
        }
    }))
}

// 创建路由
fn create_router() -> Router {
    let state = AppState::new();
    
    Router::new()
        .route("/", get(root))
        .route("/api/users", get(get_users))
        .route("/api/users", post(create_user))
        .route("/api/users/{id}", get(get_user))
        .route("/api/users/{id}", put(update_user))
        .route("/api/users/{id}", delete(delete_user))
        .route("/api/async-data", get(async_data))
        .route("/api/protected", get(protected_route))
        .with_state(state)
}

#[tokio::main]
async fn main() {
    let app = create_router();
    
    println!("服务器运行在 http://localhost:3000");
    
    let listener = tokio::net::TcpListener::bind("127.0.0.1:3000")
        .await
        .unwrap();
    
    axum::serve(listener, app).await.unwrap();
}