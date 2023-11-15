use axum::{
    routing::{get,post},
    http::StatusCode,
    response::IntoResponse,
    response::Json,
    Router
};
use serde_json::{Value, json};

#[tokio::main]

async fn main() {

    let app = Router::new().route("/", get(handler));

   async fn handler()-> Json<Value> {
       Json(json!({"data":"Json Data"}))
    }



    axum::Server::bind(&"0.0.0.0:5500".parse().unwrap()).serve(app.into_make_service()).await.unwrap();    
}

