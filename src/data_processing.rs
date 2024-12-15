use serde::Deserialize;
use std::collections::{HashMap, HashSet};
use std::error::Error;
use std::fs::File;
use html_escape::decode_html_entities;

#[derive(Debug)]
pub struct RatingRecord {
    pub user_id: u32,
    pub anime_id: u32,
    pub rating: i32, 
}

pub fn load_data() -> Result<
    (
        HashSet<u32>,
        HashSet<u32>,
        Vec<RatingRecord>,
        HashMap<u32, HashMap<u32, f32>>, // user_ratings
        HashMap<u32, String>,            // anime_id_to_name
        HashMap<u32, u32>,               // anime_id_to_members
    ),
    Box<dyn Error>,
> {
    let mut users = HashSet::new();
    let mut animes = HashSet::new();
    let mut ratings = Vec::new();
    let mut user_ratings: HashMap<u32, HashMap<u32, f32>> = HashMap::new();

    // Load ratings
    let rating_file = File::open("rating.csv")?;
    let mut rating_rdr = csv::ReaderBuilder::new()
        .has_headers(true)
        .from_reader(rating_file);

    #[derive(Debug, Deserialize)]
    struct RawRatingRecord {
        #[serde(rename = "user_id")]
        user_id: u32,
        #[serde(rename = "anime_id")]
        anime_id: u32,
        #[serde(rename = "rating")]
        rating: i32,
    }

    for result in rating_rdr.deserialize() {
        let record: RawRatingRecord = result?;
        if record.rating == -1 {
            continue;
        }

        let rating_record = RatingRecord {
            user_id: record.user_id,
            anime_id: record.anime_id,
            rating: record.rating,
        };

        users.insert(rating_record.user_id);
        animes.insert(rating_record.anime_id);

        user_ratings
            .entry(rating_record.user_id)
            .or_insert_with(HashMap::new)
            .insert(rating_record.anime_id, rating_record.rating as f32);

        ratings.push(rating_record);
    }

    // Load anime names and members from anime.csv
    let anime_file = File::open("anime.csv")?;
    let mut anime_rdr = csv::ReaderBuilder::new()
        .has_headers(true)
        .from_reader(anime_file);

    #[derive(Debug, Deserialize)]
    struct AnimeRecord {
        #[serde(rename = "anime_id")]
        pub anime_id: u32,
        #[serde(rename = "name")]
        pub name: String,
        #[serde(rename = "members")]
        pub members: u32,
    }

    let mut anime_id_to_name = HashMap::new();
    let mut anime_id_to_members = HashMap::new();

    for result in anime_rdr.deserialize() {
        let record: AnimeRecord = result?;
        let decoded_name = decode_html_entities(&record.name).into_owned(); 
        anime_id_to_name.insert(record.anime_id, decoded_name);
        anime_id_to_members.insert(record.anime_id, record.members);
    }

    println!("Total users loaded: {}", users.len());
    println!("Total ratings loaded: {}", ratings.len());

    Ok((users, animes, ratings, user_ratings, anime_id_to_name, anime_id_to_members))
}