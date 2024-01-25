// src/utils.rs

use crate::models::Content;

pub fn read_content_from_piranha() -> Content {
    // Simulated function to read content from Piranha
    // Replace this with your actual logic to fetch content from Piranha CMS
    Content {
        title: String::from("Sample Title"),
        body: String::from("Sample Body Content. This content contains important keywords."),
        images: vec![String::from("image1.jpg"), String::from("image2.jpg")],
    }
}
