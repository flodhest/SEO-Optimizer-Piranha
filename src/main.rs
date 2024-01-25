// src/main.rs

mod piranha_integration;
mod algorithms;
mod models;
mod utils;
mod analyzer;

use seo_optimizer::piranha_integration::PiranhaIntegration;
use seo_optimizer::analyzer::SEOAnalyzer;

fn main() {
    // Integrate SEO optimizations with Piranha
    let seo_report = PiranhaIntegration::integrate_seo_optimizations();

    // Display the SEO report
    println!("SEO Report: {:?}", seo_report);

    // Analyze SEO using the SEO Analyzer
    let content = seo_optimizer::utils::read_content_from_piranha();
    let analyzed_report = SEOAnalyzer::analyze(&content);

    // Display the analyzed SEO report
    println!("Analyzed SEO Report: {:?}", analyzed_report);
}
