use hyper::service::{make_service_fn, service_fn};
use hyper::{Body, Request, Response, Server};
use std::sync::Arc;
use tokio::sync::Mutex;
use std::convert::Infallible;
use std::net::SocketAddr;
use hyper::{Method, StatusCode};

#[derive(Default)]
struct VisitCounter {
    total_visits: u64,
}

async fn handle_request(
    req: Request<Body>,
    counter: Arc<Mutex<VisitCounter>>,
) -> Result<Response<Body>, Infallible> {
    match (req.method(), req.uri().path()) {
        // Serve some instructions at /
        (&Method::GET, "/") => Ok(Response::new(Body::from(
            "Welcome to the Rust-powered web server!",
        ))),

        // Increment and fetch the visit count
        (&Method::GET, "/count") => {
            let mut counter = counter.lock().await;
            counter.total_visits += 1;

            let response = format!("Visit count: {}", counter.total_visits);
            Ok(Response::new(Body::from(response)))
        }
        // Handle other paths
        _ => Ok(Response::builder()
            .status(404)
            .body(Body::from("Not Found"))
            .unwrap()),
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    // Shared state for visit count
    let counter = Arc::new(Mutex::new(VisitCounter::default()));
    println!("{}", counter);
    


    let addr = ([127, 0, 0, 1], 8080).into();
    // Define the service
    let make_svc = make_service_fn(move |_conn| {
        let counter = Arc::clone(&counter);
        async move {
            Ok::<_, Infallible>(service_fn(move |req| {
                handle_request(req, Arc::clone(&counter))
            }))
        }
    });
    // Bind and serve the server
    let server = Server::bind(&addr).serve(make_svc);
    println!("The server is currently listening on localhost:8080.");

    server.await?;
    Ok(())
}