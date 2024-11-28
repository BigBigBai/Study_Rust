use hyper::{Body, Method, Request, Response, Server, StatusCode};
use hyper::service::{make_service_fn, service_fn};
use serde::{Deserialize, Serialize};
use serde_json::json;
use std::sync::{Arc, Mutex};
use uuid::Uuid;

// Define the structures for request and response
#[derive(Debug, Deserialize)]
struct NewSongRequest {
    title: String,
    artist: String,
    genre: String,
}

#[derive(Debug, Serialize, Clone)]
struct Song {
    id: String,
    title: String,
    artist: String,
    genre: String,
    play_count: u32,
}

type SongsDB = Arc<Mutex<Vec<Song>>>;
type VisitCount = Arc<Mutex<u64>>;




#[tokio::main]
async fn main() {
    // Shared state
    let songs_db: SongsDB = Arc::new(Mutex::new(Vec::new()));
    let visit_count: VisitCount = Arc::new(Mutex::new(0));

    let make_svc = make_service_fn(move |_| {
        let songs_db = songs_db.clone();
        let visit_count = visit_count.clone();

        async move {
            Ok::<_, hyper::Error>(service_fn(move |req| {
                let songs_db = songs_db.clone();
                let visit_count = visit_count.clone();
                route_request(req, songs_db, visit_count)
            }))
        }
    });

    let addr = ([127, 0, 0, 1], 8080).into();
    let server = Server::bind(&addr).serve(make_svc);
    println!("The server is currently listening on localhost:8080.");

    if let Err(e) = server.await {
        eprintln!("Server error: {}", e);
    }
}





async fn route_request(
    req: Request<Body>,
    songs_db: SongsDB,
    visit_count: VisitCount,
) -> Result<Response<Body>, hyper::Error> {
    
    // {
    //     let mut count = visit_count.lock().unwrap();
    //     *count += 1;
    // }
    
    
    match (req.method(), req.uri().path()) {
        (&Method::GET, "/") => Ok(Response::new(Body::from(
            "Welcome to the Rust-powered web server!",
        ))),
        (&Method::GET, "/count") => {
            let mut count = visit_count.lock().unwrap();
            *count += 1;
            let response = format!("Visit count: {}", count);
            Ok(Response::new(Body::from(response)))
            // handle_visit_count(visit_count).await
        }
        (&Method::POST, "/songs/new") => handle_add_song(req, songs_db).await,
        _ => Ok(Response::builder()
            .status(StatusCode::NOT_FOUND)
            .body(Body::from("Not Found"))
            .unwrap()),
    }
}

async fn handle_add_song(
    req: Request<Body>,
    songs_db: SongsDB,
) -> Result<Response<Body>, hyper::Error> {
    let whole_body = hyper::body::to_bytes(req.into_body()).await?;
    let new_song_request: NewSongRequest = match serde_json::from_slice(&whole_body) {
        Ok(p) => p,
        Err(_) => {
            return Ok(Response::builder()
                .status(StatusCode::BAD_REQUEST)
                .body(Body::from("Invalid JSON"))
                .unwrap());
        }
    };

    let new_song = Song {
        id: Uuid::new_v4().to_string(),
        title: new_song_request.title,
        artist: new_song_request.artist,
        genre: new_song_request.genre,
        play_count: 0,
    };

    {
        let mut db = songs_db.lock().unwrap();
        db.push(new_song.clone());
    }

    let response_body = serde_json::to_string(&new_song).unwrap();

    Ok(Response::builder()
        .status(StatusCode::OK)
        .header("Content-Type", "application/json")
        .body(Body::from(response_body))
        .unwrap())
}

// async fn handle_visit_count(visit_count: VisitCount) -> Result<Response<Body>, hyper::Error> {
//     let count = {
//         let count = visit_count.lock().unwrap();
//         *count
//     };
//     let response_body = json!({ "visit_count": count }).to_string();
//     Ok(Response::builder()
//         .status(StatusCode::OK)
//         .header("Content-Type", "application/json")
//         .body(Body::from(response_body))
//         .unwrap())
// }
