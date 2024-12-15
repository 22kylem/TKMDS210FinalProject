mod data_processing;
mod recommendation;
mod similarity;

use crate::data_processing::load_data;
use crate::recommendation::recommend_anime;
use crate::similarity::calculate_similarity;
use std::collections::HashMap;
use std::error::Error;
use std::io::{stdin, stdout, Write};  

fn main() -> Result<(), Box<dyn Error>> {
    let (_users, _animes, _ratings, user_ratings, anime_id_to_name, anime_id_to_members) = load_data()?;

    let user_input = get_user_input(&anime_id_to_name)?;

    let target_user_id = 0;
    if user_input.len() < 3 {
        println!("You must rate at least 3 anime before typing 'done'. Please restart.");
        return Ok(());
    }

    let mut user_ratings_with_target = user_ratings.clone();
    let target_user_ratings: HashMap<u32, f32> = user_input.into_iter().collect();
    user_ratings_with_target.insert(target_user_id, target_user_ratings);

    let similarities = calculate_similarity(&user_ratings_with_target, target_user_id);

    if similarities.is_empty() {
        println!("No similar users found based on your current ratings.");
        println!("Try rating more anime to improve recommendations.");
        return Ok(());
    }

    let recommendations = recommend_anime(
        &user_ratings_with_target,
        &similarities,
        target_user_id,
        &anime_id_to_name,
        &anime_id_to_members,
    );

    if recommendations.is_empty() {
        println!("No suitable recommendations found. Try rating different or more anime.");
    } else {
        println!("\nRecommended Anime:");
        for anime in recommendations {
            println!("{}", anime);
        }
    }

    Ok(())
}

fn get_user_input(
    anime_id_to_name: &HashMap<u32, String>,
) -> Result<Vec<(u32, f32)>, Box<dyn Error>> {
    let stdin = stdin();
    let mut stdout = stdout();
    let mut user_input = Vec::new();

    println!("Enter at least 3 anime titles (and up to 10) and their ratings (1-10).");
    println!("Type 'done' when you are finished.\n");

    while user_input.len() < 10 {
        print!("Enter anime title: ");
        stdout.flush()?;

        let mut anime_title = String::new();
        stdin.read_line(&mut anime_title)?;
        let anime_title = anime_title.trim();

        if anime_title.eq_ignore_ascii_case("done") {
            if user_input.len() < 3 {
                println!("You must rate at least 3 anime before typing 'done'. Please continue.");
                continue;
            } else {
                break;
            }
        }

        let anime_id = match find_anime_id(anime_title, anime_id_to_name) {
            Some(id) => id,
            None => {
                println!("Anime title not recognized. Please try again.");
                continue;
            }
        };

        print!("Enter your rating for '{}': ", anime_title);
        stdout.flush()?;
        let mut rating_str = String::new();
        stdin.read_line(&mut rating_str)?;
        let rating_str = rating_str.trim();

        let rating: f32 = match rating_str.parse() {
            Ok(r) if r >= 1.0 && r <= 10.0 => r,
            _ => {
                println!("Invalid rating. Please enter a number between 1 and 10.");
                continue;
            }
        };

        user_input.push((anime_id, rating));
        println!(); 
    }

    Ok(user_input)
}

fn find_anime_id(input_title: &str, anime_id_to_name: &HashMap<u32, String>) -> Option<u32> {
    if let Some(&id) = anime_id_to_name.iter().find_map(|(id, name)| {
        if name.eq_ignore_ascii_case(input_title) {
            Some(id)
        } else {
            None
        }
    }) {
        return Some(id);
    }

    let suggestion = find_similar_anime(input_title, anime_id_to_name)?;
    println!("Did you mean '{}'? (yes/no)", suggestion);

    let stdin = stdin();
    let mut stdout = stdout();
    stdout.flush().ok()?;

    let mut response = String::new();
    stdin.read_line(&mut response).ok()?;
    let response = response.trim().to_lowercase();

    if response == "yes" || response == "y" {
        anime_id_to_name
            .iter()
            .find(|(_, name)| name.eq_ignore_ascii_case(&suggestion))
            .map(|(id, _)| *id)
    } else {
        None
    }
}

fn find_similar_anime<'a>(
    input_title: &str,
    anime_id_to_name: &'a HashMap<u32, String>,
) -> Option<String> {
    let input_title_normalized = normalize_title(input_title);
    let mut max_similarity = 0.0;
    let mut closest_match = None;

    for anime_title in anime_id_to_name.values() {
        let anime_title_normalized = normalize_title(anime_title);
        let similarity = strsim::jaro_winkler(&input_title_normalized, &anime_title_normalized);
        if similarity > max_similarity {
            max_similarity = similarity;
            closest_match = Some(anime_title.clone());
        }
    }

    if max_similarity >= 0.85 {
        closest_match
    } else {
        None
    }
}

fn normalize_title(title: &str) -> String {
    title.to_lowercase().chars().filter(|c| c.is_alphanumeric() || c.is_whitespace()).collect()
}