use axum::{
    routing::{get, post,put,delete},
    http::StatusCode,
    Json, Router,
};
use diesel::result::Error;
use serde::{Deserialize, Serialize};
pub mod models;
pub mod schema;
use self::schema::todos::dsl::*;

use diesel::prelude::*;
use dotenvy::dotenv;
use std::env;
use diesel_async::{RunQueryDsl, AsyncConnection, AsyncMysqlConnection};
#[tokio::main] 
async fn main() {
    println!("{:?}",get_all_todos().await);
    let app: Router<()> = Router::new()
    .route("/task", post(post_tasks))
    .route("/task", get(get_tasks))
    .route("/task", put(update_tasks))
    .route("/task", delete(delete_tasks))
    .route("/", get(hello_world))
    .with_state(());
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}
//root
async fn hello_world()-> &'static str {
    "hello_world"
}
//-------------------------------------------------------------------------------------------------------------------------------------
#[derive(Deserialize)]
struct CreateTodo {
    pub date: String,
    pub inhalt: String,
    pub percent: i32,
}


async fn post_tasks(Json(payload) : Json<CreateTodo>) -> StatusCode{
    let new_post = models::NewTodo{date: payload.date.as_str(), inhalt: payload.inhalt.as_str(),percent:payload.percent};
    
    match create_new_todo(new_post).await{
        Ok(_) => StatusCode::OK,
        Err(_) => StatusCode::INTERNAL_SERVER_ERROR
    }
}

//-------------------------------------------------------------------------------------------------------------------------------------
async fn get_tasks() -> (StatusCode,Json<Vec<models::Todo>>){
    let all_todos = get_all_todos().await;
    match all_todos {
        Ok(todo) => (StatusCode::OK, Json(todo)),
        Err(_) => (StatusCode::INTERNAL_SERVER_ERROR,Json(vec![])),
    }
}

//-------------------------------------------------------------------------------------------------------------------------------------

async fn update_tasks(Json(payload) : Json<models::Todo>) -> StatusCode{
    match update_todo_by_id(payload).await{
        Ok(_) => StatusCode::OK,
        Err(_) => StatusCode::INTERNAL_SERVER_ERROR,

    }
}

//-------------------------------------------------------------------------------------------------------------------------------------
#[derive(Deserialize)]
struct DeleteTasks{
    pub id:i32
}
async fn delete_tasks(Json(payload) : Json<DeleteTasks>)-> StatusCode{
    match delete_todo_by_id(payload.id).await{
        Ok(_) => StatusCode::OK,
        Err(_) => StatusCode::NOT_FOUND,
    }
}

//-------------------------------------------------------------------------------------------------------------------------------------
async fn establish_connection() -> AsyncMysqlConnection{
    dotenv().ok();
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    return AsyncMysqlConnection::establish(&database_url).await.expect("cool");
}
async fn create_new_todo<'a>( new_post : models::NewTodo<'a>) -> Result<(), Error>{
    let mut conn = establish_connection().await;
    diesel::insert_into(schema::todos::table).values(&new_post).execute(&mut conn).await.expect("asd");
    Ok(())
}
async fn get_all_todos() -> Result<Vec<models::Todo>,Error>{

    let mut conn = establish_connection().await;
    let result = todos.select(models::Todo::as_select()).load(&mut conn).await?;

    Ok(result)
}

async fn delete_todo_by_id(id_: i32) -> Result<(), Error>{
    let mut conn = establish_connection().await;
    let result = diesel::delete(todos.filter(id.eq(id_)))
        .execute(&mut conn)
        .await
        .expect("Error deleting posts");

    if result == 0 {
        Err(diesel::NotFound)
    } else {
        println!("Successfully deleted {} todo(s)", result);
        Ok(())
    }
}
async fn update_todo_by_id(new_todo: models::Todo) -> Result<(), Error>{
    let mut conn = establish_connection().await;
    let result = diesel::update(todos.find(new_todo.id))
                .set((date.eq(new_todo.date),inhalt.eq(new_todo.inhalt),percent.eq(new_todo.percent)))
                .execute(&mut conn).await.expect("didnt work lil bro");
    if result == 0 {
        // You can use a custom error if desired, or wrap the standard Diesel error
        Err(diesel::NotFound)
    } else {
        println!("Successfully deleted {} todo(s)", result);
        Ok(())
    }

}

