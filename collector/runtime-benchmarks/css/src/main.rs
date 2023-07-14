//! This benchmark tests the performance of CSS stylesheet parsing.
//! It uses the `lightningcss` crate, which internally uses `cssparser`, which is a browser-grade
//! CSS parser from Servo.
use benchlib::benchmark::run_benchmark_group;
use lightningcss::stylesheet::{ParserOptions, StyleSheet};

static FB_CSS: &str = include_str!("../data/fb.css");

fn main() {
    // Inflate the CSS data a bit
    let fb_css_minified = FB_CSS.repeat(10);

    run_benchmark_group(|group| {
        group.register_benchmark("css-parse-fb", || {
            || StyleSheet::parse(&fb_css_minified, ParserOptions::default()).unwrap()
        });
    });
}
