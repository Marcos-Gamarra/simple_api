use axum::{routing::get, Router};

async fn hello() -> String {
    let my_local_ip = local_ip_address::local_ip().unwrap().to_string();

    format!("The local ip of this container is: {}", my_local_ip)
}

#[tokio::main]
async fn main() {
    println!("Starting server on port 3000");
    // build our application with a single route
    let app = Router::new().route("/", get(hello));

    // run our app with hyper, listening globally on port 3000
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}
