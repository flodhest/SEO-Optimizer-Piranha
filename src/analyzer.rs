// src/analyzer.rs

use crate::models::{Content, SEOReport};
use crate::algorithms::{alt_text, keyword_density, link_analysis, metadata};

pub struct SEOAnalyzer;

impl SEOAnalyzer {

       // New Function: Check for broken links during analysis
       pub fn check_broken_links_during_analysis(content: &Content) -> Vec<String> {
        link_analysis::check_broken_links(content)
    }


    pub fn analyze(content: &Content) -> SEOReport {
        // Perform SEO analysis
        let metadata_suggestion = metadata::suggest_metadata(content);
        let alt_text_suggestions = alt_text::suggest_alt_text(&content.images, &content.body);
        let keyword_density = keyword_density::calculate_keyword_density(content);

        // Prepare SEO report
   #[derive(Debug)]
pub struct SEOReport {
    pub metadata_suggestion: String,
    pub alt_text_suggestions: Vec<String>,
    pub sitemap: String,  // Add this line
    pub keyword_density: f64,
}

        
    }
}
