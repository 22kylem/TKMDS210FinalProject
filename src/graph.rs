use petgraph::graph::UnGraph;
use std::collections::HashMap;

use crate::data_processing::RatingRecord;

pub fn build_graph(
    users: &std::collections::HashSet<u32>,
    animes: &std::collections::HashSet<u32>,
    ratings: &Vec<RatingRecord>,
) -> UnGraph<String, f32> {
    let mut graph = UnGraph::<String, f32>::new_undirected();
    let mut user_indices = HashMap::new();
    let mut anime_indices = HashMap::new();

    // Add user nodes
    for &user_id in users {
        let index = graph.add_node(format!("User:{}", user_id));
        user_indices.insert(user_id, index);
    }

    // Add anime nodes
    for &anime_id in animes {
        let index = graph.add_node(format!("Anime:{}", anime_id));
        anime_indices.insert(anime_id, index);
    }

    // Add edges with ratings as weights
    for rating in ratings {
        if let (Some(&user_idx), Some(&anime_idx)) = (
            user_indices.get(&rating.user_id),
            anime_indices.get(&rating.anime_id),
        ) {
            graph.add_edge(user_idx, anime_idx, rating.rating);
        }
    }

    graph
}
