use serde::{Deserialize, Serialize};
use mongodb::{bson::doc, options::ClientOptions, Client, Collection};
use futures::stream::TryStreamExt; // Required for `.try_collect()`

#[derive(Debug, Serialize, Deserialize, Clone)]
struct Song {
    #[serde(rename = "_id")]
    id: u64, // MongoDB uses `_id` for unique document identifiers
    title: String,
    artist: String,
    genre: String,
    play_count: u64,
}

async fn get_collection() -> Result<Collection<Song>, Box<dyn std::error::Error>> {
    // MongoDB connection URI
    let uri = "mongodb://localhost:27017"; // Adjust for your environment

    // Initialize MongoDB client
    let client_options = ClientOptions::parse(uri).await?;
    let client = Client::with_options(client_options)?;

    // Access the database and collection
    let database = client.database("song_library");
    let collection = database.collection::<Song>("songs");

    Ok(collection)
}

async fn add_song(collection: &Collection<Song>, song: Song) -> Result<(), Box<dyn std::error::Error>> {
    collection.insert_one(song, None).await?;
    Ok(())
}

async fn get_all_songs(collection: &Collection<Song>) -> Result<Vec<Song>, Box<dyn std::error::Error>> {
    let cursor = collection.find(None, None).await?;
    let songs: Vec<Song> = cursor.try_collect().await?;
    Ok(songs)
}

async fn increment_play_count(collection: &Collection<Song>, song_id: u64) -> Result<(), Box<dyn std::error::Error>> {
    let filter = doc! { "_id": song_id };
    let update = doc! { "$inc": { "play_count": 1 } };
    collection.update_one(filter, update, None).await?;
    Ok(())
}

async fn delete_song(collection: &Collection<Song>, song_id: u64) -> Result<(), Box<dyn std::error::Error>> {
    let filter = doc! { "_id": song_id };
    collection.delete_one(filter, None).await?;
    Ok(())
}



#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Connect to the MongoDB collection
    let collection = get_collection().await?;

    // Add a new song
    let new_song = Song {
        id: 1,
        title: "Imagine".to_string(),
        artist: "John Lennon".to_string(),
        genre: "Pop".to_string(),
        play_count: 0,
    };
    add_song(&collection, new_song.clone()).await?;

    // Increment play count
    increment_play_count(&collection, 1).await?;

    // Fetch all songs
    let songs = get_all_songs(&collection).await?;
    println!("Songs in library: {:?}", songs);

    // Delete a song
    delete_song(&collection, 1).await?;

    Ok(())
}

