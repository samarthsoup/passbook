use axum::{routing::get, routing::post, Router};

mod functions;
use crate::functions::{
    deposit, depositactivity, home, login, loginactivity, delete, signup, signupactivity,
    userpage, withdraw, history, withdrawactivity, assign, assignactivity
};

mod html;

#[tokio::main]
async fn main() {
    let app = Router::new()
        .route("/", get(home))
        .route("/signup", get(signup))
        .route("/signupactivity", post(signupactivity))
        .route("/login", get(login))
        .route("/loginactivity", post(loginactivity))
        .route("/userpage/:userid", get(userpage))
        .route("/history/:userid", get(history))
        .route("/deposit/:userid", get(deposit))
        .route("/depositactivity/:userid", post(depositactivity))
        .route("/withdraw/:userid", get(withdraw))
        .route("/withdrawactivity/:userid", post(withdrawactivity))
        .route("/assign/:userid", get(assign))
        .route("/assignactivity/:userid", post(assignactivity))
        .route("/delete/:userid", get(delete)); 
    
    axum::Server::bind(&"0.0.0.0:3000".parse().unwrap())
        .serve(app.into_make_service())
        .await
        .unwrap();
}