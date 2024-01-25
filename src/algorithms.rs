// src/algorithms.rs

pub mod keyword_density {
    use natural::tokenize;

    pub fn calculate_keyword_density(content: &crate::models::Content) -> f64 {
        let body_keywords: Vec<&str> = tokenize(&content.body)
            .into_iter()
            .filter(|&token| is_keyword(token))
            .collect();

        let total_words: usize = tokenize(&content.body).len();
        let keyword_count = body_keywords.len();

        if total_words > 0 {
            (keyword_count as f64) / (total_words as f64)
        } else {
            0.0
        }
    }

    fn is_keyword(word: &str) -> bool {
        // Implement your keyword criteria here
        // You might consider using a more sophisticated algorithm or an external library
        // For simplicity, we'll use a basic check for demonstration
        let common_keywords = vec!["important", "SEO", "optimize"];
        common_keywords.contains(&word)
    }
}

pub mod alt_text {
    use natural::tokenize;

    pub fn suggest_alt_text(images: &Vec<String>, content: &str) -> Vec<String> {
        images
            .iter()
            .map(|image| {
                let image_keywords: Vec<&str> = tokenize(image)
                    .into_iter()
                    .filter(|&token| is_keyword(token))
                    .collect();

                let image_description = generate_image_description(image, content);

                format!("Alt text for '{}': Keywords: {:?}. Description: {}", image, image_keywords, image_description)
            })
            .collect()
    }

    fn is_keyword(word: &str) -> bool {
        // Implement your keyword criteria here
        // You might consider using a more sophisticated algorithm or an external library
        // For simplicity, we'll use a basic check for demonstration
        let common_keywords = vec!["image", "photo", "picture"];
        common_keywords.contains(&word)
    }

    fn generate_image_description(image: &str, content: &str) -> String {
        // Implement a more sophisticated image description generation algorithm
        // For simplicity, we'll use the first 10 words of the content
        let words: Vec<&str> = tokenize(content).into_iter().collect();
        words.iter().take(10).cloned().collect::<Vec<&str>>().join(" ")
    }
}

pub mod metadata {
    use natural::tokenize;

    pub fn suggest_metadata(content: &crate::models::Content) -> String {
        let title_keywords: Vec<&str> = tokenize(&content.title)
            .into_iter()
            .filter(|&token| is_keyword(token))
            .collect();

        let body_keywords: Vec<&str> = tokenize(&content.body)
            .into_iter()
            .filter(|&token| is_keyword(token))
            .collect();

        let keywords = [&title_keywords[..], &body_keywords[..]].concat();

        // Generate a concise and compelling description
        let description = generate_description(&content.body);

        format!("SEO Metadata for '{}'. Keywords: {:?}. Description: {}", content.title, keywords, description)
    }

    fn is_keyword(word: &str) -> bool {
        // Implement your keyword criteria here
        // You might consider using a more sophisticated algorithm or an external library
        // For simplicity, we'll use a basic check for demonstration
        let common_keywords = vec!["SEO", "optimize", "content"];
        common_keywords.contains(&word)
    }

    fn generate_description(body: &str) -> String {
        // Implement a more sophisticated description generation algorithm
        // For simplicity, we'll use the first 20 words of the body
        let words: Vec<&str> = tokenize(body).into_iter().collect();
        words.iter().take(20).cloned().collect::<Vec<&str>>().join(" ")
    }
}
