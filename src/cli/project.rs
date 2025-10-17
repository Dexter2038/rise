use inquire::Select;
use strum::IntoEnumIterator;

use crate::cli::render::get_render_cfg;
use crate::{cli::web::prompt_web_config, config::project::ProjectConfig};

pub fn prompt_project_config() -> Result<ProjectConfig, Box<dyn std::error::Error>> {
    let kinds = ProjectConfig::iter().collect();
    let mut project = Select::new("What type of project do you want to create?", kinds)
        .with_render_config(get_render_cfg())
        .prompt()?;
    match &mut project {
        ProjectConfig::WebApp(config) => {
            prompt_web_config(config)?;
        }
        ProjectConfig::CliApp => (),
        ProjectConfig::Library => (),
        ProjectConfig::Embedded => (),
        ProjectConfig::Game => (),
    };
    Ok(project)
}
