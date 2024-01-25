// src/models.rs

#[derive(Debug)]
pub struct Content {
    pub title: String,
    pub body: String,
    pub images: Vec<String>,
}

#[derive(Debug)]
pub struct SEOReport {
    pub metadata_suggestion: String,
    pub alt_text_suggestions: Vec<String>,
    pub sitemap: String,
    pub keyword_density: f64,
}
