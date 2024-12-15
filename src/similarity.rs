use std::collections::HashMap;

pub fn calculate_similarity(
    user_ratings: &HashMap<u32, HashMap<u32, f32>>,
    target_user: u32,
) -> HashMap<u32, f32> {
    let mut similarities = HashMap::new();
    let target_ratings = &user_ratings[&target_user];

    for (&other_user, other_ratings) in user_ratings.iter() {
        if other_user == target_user {
            continue;
        }

        let mut sum_products = 0.0;
        let mut sum_rating_sq = 0.0;
        let mut sum_other_sq = 0.0;
        let mut common_count = 0;

        for (&anime_id, &rating) in target_ratings {
            if let Some(&other_rating) = other_ratings.get(&anime_id) {
                sum_products += rating * other_rating;
                sum_rating_sq += rating.powi(2);
                sum_other_sq += other_rating.powi(2);
                common_count += 1;
            }
        }
        // skip if no common anime
        if common_count == 0 {
            continue;
        }

        let denominator = (sum_rating_sq.sqrt()) * (sum_other_sq.sqrt());
        if denominator != 0.0 {
            let cosine_similarity = sum_products / denominator;
            similarities.insert(other_user, cosine_similarity);
        }
    }

    similarities
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::HashMap;

    #[test]
    fn test_cosine_similarity_identical_ratings() {
        // Two users with identical ratings for the same two anime
        let mut user_ratings = HashMap::new();

        let mut target = HashMap::new();
        target.insert(1, 10.0);
        target.insert(2, 9.0);
        user_ratings.insert(0, target);

        let mut other = HashMap::new();
        other.insert(1, 10.0);
        other.insert(2, 9.0);
        user_ratings.insert(1, other);

        let similarities = calculate_similarity(&user_ratings, 0);
        // User 1 should have a similarity of 1.0 with User 0
        let sim = similarities.get(&1).copied().unwrap_or(0.0);
        assert!((sim - 1.0).abs() < 1e-6, "Expected similarity of 1.0, got {}", sim);
    }

    #[test]
    fn test_cosine_similarity_no_overlap() {
        // Two users with no common anime
        let mut user_ratings = HashMap::new();

        let mut target = HashMap::new();
        target.insert(1, 8.0);
        user_ratings.insert(0, target);

        let mut other = HashMap::new();
        other.insert(2, 9.0);
        user_ratings.insert(1, other);

        let similarities = calculate_similarity(&user_ratings, 0);
        // No overlap, user 1 should not appear in similarities
        assert!(similarities.get(&1).is_none(), "Expected no similarity since no overlap");
    }

    #[test]
    fn test_cosine_similarity_partial_overlap() {
        // Two users have partial overlap
        let mut user_ratings = HashMap::new();

        let mut target = HashMap::new();
        target.insert(1, 10.0);
        target.insert(2, 5.0);
        user_ratings.insert(0, target);

        let mut other = HashMap::new();
        other.insert(1, 10.0);
        other.insert(3, 5.0); // No overlap 
        user_ratings.insert(1, other);

        let similarities = calculate_similarity(&user_ratings, 0);
        let sim = similarities.get(&1).copied().unwrap_or(0.0);
        assert!((sim - 1.0).abs() < 1e-6, "Expected similarity of 1.0 for single identical rating");
    }

    #[test]
    fn test_cosine_similarity_different_ratings() {
        // Users have 2 anime in common, but different ratings
        let mut user_ratings = HashMap::new();

        let mut target = HashMap::new();
        target.insert(1, 10.0);
        target.insert(2, 5.0);
        user_ratings.insert(0, target);

        let mut other = HashMap::new();
        other.insert(1, 5.0);  // different rating
        other.insert(2, 10.0); // swapped 
        user_ratings.insert(1, other);

        let similarities = calculate_similarity(&user_ratings, 0);
        // Compute expected cosine similarity manually:
        // Target vector: [10, 5]
        // Other vector: [5, 10]
        // Dot product = (10*5) + (5*10) = 50 + 50 = 100
        // |target| = sqrt(10^2 + 5^2) = sqrt(100 + 25) = sqrt(125)
        // |other| = sqrt(5^2 + 10^2) = sqrt(25 + 100) = sqrt(125)
        // similarity = 100 / (sqrt(125) * sqrt(125)) = 100 / 125 = 0.8
        // lines left here to demonstrate how the value of 0.8 was found
        let sim = similarities.get(&1).copied().unwrap_or(0.0);
        assert!((sim - 0.8).abs() < 1e-6, "Expected similarity ~0.8, got {}", sim);
    }
}