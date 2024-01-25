// src/piranha_integration.rs

use crate::algorithms::{alt_text, keyword_density, link_analysis, metadata, sitemap};
use crate::models::{Content, SEOReport};
use crate::utils::read_content_from_piranha;

impl PiranhaIntegration {
    pub fn integrate_seo_optimizations() -> SEOReport {
        // Read content from Piranha
        let mut content = read_content_from_piranha();

        // Perform SEO analysis
        let metadata_suggestion = metadata::suggest_metadata(&content);
        let alt_text_suggestions = alt_text::suggest_alt_text(&content.images, &content.body);
        let sitemap = sitemap::generate_sitemap(&content);
        let keyword_density = keyword_density::calculate_keyword_density(&content);

        // Apply SEO optimizations   pub fn perform_link_analysis(content: &Content) {
            let broken_links = link_analysis::check_broken_links(content);
            if !broken_links.is_empty() {
                println!("Broken Links Found: {:?}", broken_links);
            } else {
                println!("No Broken Links Found.");
            }
        
        metadata::apply_metadata_optimization(&mut content, &metadata_suggestion);
        alt_text::apply_alt_text_optimization(&mut content, &alt_text_suggestions);

        // Prepare SEO report
        SEOReport {
            metadata_suggestion,
            alt_text_suggestions,
            sitemap,
            keyword_density,
        }
    }
}
