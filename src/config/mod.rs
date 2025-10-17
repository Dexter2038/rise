pub mod project;
pub mod render;
pub mod web;

pub trait BuildConfig {
    fn build(&mut self) -> Result<(), Box<dyn std::error::Error>>;
}

