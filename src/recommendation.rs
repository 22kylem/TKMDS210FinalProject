use rand::seq::SliceRandom;
use rand::thread_rng;


pub fn recommend_anime(
    user_ratings: &std::collections::HashMap<u32, std::collections::HashMap<u32, f32>>,
    similarities: &std::collections::HashMap<u32, f32>,
    target_user_id: u32,
    anime_id_to_name: &std::collections::HashMap<u32, String>,
    anime_id_to_members: &std::collections::HashMap<u32, u32>,
) -> Vec<String> {
    let mut scores = std::collections::HashMap::new();
    let mut total_similarity = std::collections::HashMap::new();

    for (&other_user_id, &similarity) in similarities {
        if similarity <= 0.0 {
            continue;
        }

        for (&anime_id, &rating) in &user_ratings[&other_user_id] {
            if user_ratings[&target_user_id].contains_key(&anime_id) {
                continue; // skip already rated anime
            }

            *scores.entry(anime_id).or_insert(0.0) += similarity * rating;
            *total_similarity.entry(anime_id).or_insert(0.0) += similarity;
        }
    }

    let mut rankings: Vec<(u32, f32)> = scores
        .iter()
        .map(|(&anime_id, &score)| {
            let normalized_score = score / total_similarity[&anime_id];
            (anime_id, normalized_score)
        })
        .collect();

    // Sort by predicted score descending
    rankings.sort_by(|a, b| b.1.partial_cmp(&a.1).unwrap_or(std::cmp::Ordering::Equal));

    // Filter by popularity
    let popularity_threshold = 5000; // adjust as needed
    rankings.retain(|(anime_id, _score)| {
        let members = *anime_id_to_members.get(anime_id).unwrap_or(&0);
        members > popularity_threshold
    });

    // Introduce variety:
    // If we have more than 30 anime after filtering, take top 30 then randomly select 10
    let mut rng = thread_rng();
    let chosen_count = 10;
    let pool_count = 30;

    if rankings.len() > pool_count {
        // Take top 30
        let top_slice = &rankings[..pool_count];
        let mut top_vec = top_slice.to_vec();
        top_vec.shuffle(&mut rng); // Shuffle them
        top_vec.truncate(chosen_count); // Take random 10 from these 30
        top_vec
            .into_iter()
            .filter_map(|(anime_id, _)| anime_id_to_name.get(&anime_id).cloned())
            .collect()
    } else {
        // If fewer than 30 after filtering, just take top 10 directly
        rankings
            .into_iter()
            .take(chosen_count)
            .filter_map(|(anime_id, _)| anime_id_to_name.get(&anime_id).cloned())
            .collect()
    }
}