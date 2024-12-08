use hyper::{Body, Method, Request, Response, Server, StatusCode};
use hyper::service::{make_service_fn, service_fn};
use serde::{Deserialize, Serialize};
use std::sync::{Arc, Mutex};
use serde_json::json;
use std::fs::{File, OpenOptions};
use std::io::{Read, Write};

// Define the structures for request and response
#[derive(Debug, Deserialize)]
struct NewSongRequest {
    title: String,
    artist: String,
    genre: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
struct Song {
    id: u64,
    title: String,
    artist: String,
    genre: String,
    play_count: u64,
}

type SongsDB = Arc<Mutex<Vec<Song>>>;
type VisitCount = Arc<Mutex<u64>>;
type IdCount = Arc<Mutex<u64>>;

const LIBRARY_FILE: &str = "library.json";

fn load_library() -> Vec<Song> {
    // Try to open the library file
    if let Ok(mut file) = File::open(LIBRARY_FILE) {
        let mut contents = String::new();
        if file.read_to_string(&mut contents).is_ok() {
            if let Ok(songs) = serde_json::from_str::<Vec<Song>>(&contents) {
                return songs;
            }
        }
    }

    // Return an empty library if the file doesn't exist or is invalid
    vec![]
}

fn save_library(songs: &Vec<Song>) {
    if let Ok(mut file) = OpenOptions::new()
        .write(true)
        .create(true)
        .truncate(true)
        .open(LIBRARY_FILE)
    {
        if let Ok(contents) = serde_json::to_string_pretty(songs) {
            let _ = file.write_all(contents.as_bytes());
        }
    }
}



#[tokio::main]
async fn main() {
    // Shared state
    let songs_db: SongsDB = Arc::new(Mutex::new(load_library()));
    let visit_count: VisitCount = Arc::new(Mutex::new(0));
    
    // println!("{:?}", songs_db.lock().unwrap().len());
    let length: u64 = songs_db.lock().unwrap().len().try_into().unwrap();
    let id_count: IdCount = Arc::new(Mutex::new(length));

    
    let make_svc = make_service_fn(move |_| {
        let songs_db = songs_db.clone();
        let visit_count = visit_count.clone();
        let id_count = id_count.clone();

        async move {
            Ok::<_, hyper::Error>(service_fn(move |req| {
                let songs_db = songs_db.clone();
                let visit_count = visit_count.clone();
                let id_count = id_count.clone();
                route_request(req, songs_db, visit_count, id_count)
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
    id_count: IdCount,
) -> Result<Response<Body>, hyper::Error> {
    if req.method() == Method::GET && req.uri().path().starts_with("/songs/play/") {
        return handle_play_song(req, songs_db).await;
    }

    match (req.method(), req.uri().path()) {
        (&Method::GET, "/") => Ok(Response::new(Body::from(
            "Welcome to the Rust-powered web server!",
        ))),
        (&Method::GET, "/count") => {
            let mut count = visit_count.lock().unwrap();
            *count += 1;
            let response = format!("Visit count: {}", count);
            Ok(Response::new(Body::from(response)))
        }
        (&Method::POST, "/songs/new") => handle_add_song(req, songs_db, id_count).await,
        (&Method::GET, "/songs/search") => handle_search_songs(req, songs_db).await,
        _ => Ok(Response::builder()
            .status(StatusCode::NOT_FOUND)
            .body(Body::from("Not Found"))
            .unwrap()),
    }
}


async fn handle_add_song(
    req: Request<Body>,
    songs_db: SongsDB,
    id_count: IdCount,
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

    let mut count = id_count.lock().unwrap();
    *count += 1;

    let new_song = Song {
        id: *count,
        title: new_song_request.title,
        artist: new_song_request.artist,
        genre: new_song_request.genre,
        play_count: 0,
    };

    {
        let mut db = songs_db.lock().unwrap();
        db.push(new_song.clone());
        // Persist the updated library to the file
        save_library(&db);
    }

    let response_body = serde_json::to_string(&new_song).unwrap();
    Ok(Response::builder()
        .status(StatusCode::OK)
        .header("Content-Type", "application/json")
        .body(Body::from(response_body))
        .unwrap())
}


async fn handle_search_songs(
    req: Request<Body>,
    songs_db: SongsDB,
) -> Result<Response<Body>, hyper::Error> {
    let query = req.uri().query().unwrap_or("");
    let query_params: Vec<_> = query.split('&').collect();

    let mut title_filter = None;
    let mut artist_filter = None;
    let mut genre_filter = None;
    for param in query_params {
        let mut parts = param.split('=');
        let key = parts.next().unwrap_or("");
        let value = parts.next().unwrap_or("").replace('+', " ");

        match key {
            "title" => title_filter = Some(value.to_lowercase()),
            "artist" => artist_filter = Some(value.to_lowercase()),
            "genre" => genre_filter = Some(value.to_lowercase()),
            _ => {}
        }
    }

    let songs = songs_db.lock().unwrap();
    let filtered_songs: Vec<Song> = songs
        .iter()
        .filter(|song| {
            let title_match = if let Some(ref title_filter) = title_filter {
                song.title.to_lowercase().contains(title_filter)
            } else {
                true
            };

            let artist_match = if let Some(ref artist_filter) = artist_filter {
                song.artist.to_lowercase().contains(artist_filter)
            } else {
                true
            };

            let genre_match = if let Some(ref genre_filter) = genre_filter {
                song.genre.to_lowercase().contains(genre_filter)
            } else {
                true
            };

            title_match && artist_match && genre_match
        })
        .cloned()
        .collect();

    let response_body = serde_json::to_string(&filtered_songs).unwrap();
    Ok(Response::builder()
        .status(StatusCode::OK)
        .header("Content-Type", "application/json")
        .body(Body::from(response_body))
        .unwrap())
}


async fn handle_play_song(
    req: Request<Body>,
    songs_db: SongsDB,
) -> Result<Response<Body>, hyper::Error> {
    let path = req.uri().path();
    let id_segment = path.trim_start_matches("/songs/play/");
    let song_id: u64 = match id_segment.parse() {
        Ok(id) => id,
        Err(_) => {
            let error_body = json!({ "error": "Invalid song ID" });
            return Ok(Response::builder()
                .status(StatusCode::BAD_REQUEST)
                .header("Content-Type", "application/json")
                .body(Body::from(error_body.to_string()))
                .unwrap());
        }
    };

    let mut songs = songs_db.lock().unwrap();
    // Search for the song index
    if let Some(index) = songs.iter().position(|s| s.id == song_id) {
        // Update play count using the index
        songs[index].play_count += 1;

        // Persist the updated library to the file
        save_library(&songs);

        // Return the updated song as JSON
        let response_body = serde_json::to_string(&songs[index]).unwrap();
        Ok(Response::builder()
            .status(StatusCode::OK)
            .header("Content-Type", "application/json")
            .body(Body::from(response_body))
            .unwrap())
    } else {
        // Song not found
        let error_body = json!({"error":"Song not found"});
        Ok(Response::builder()
            .status(StatusCode::NOT_FOUND)
            .header("Content-Type", "application/json")
            .body(Body::from(error_body.to_string()))
            .unwrap())
    }
}