use super::BuildConfig;

use super::web::WebAppConfig;
use inquire::Select;

use super::render::get_render_cfg;

#[derive(strum_macros::Display)]
pub enum ProjectConfig {
    WebApp(WebAppConfig),
    CliApp,
    Library,
    Embedded,
    Game,
}

impl ProjectConfig {
    pub fn new() -> Result<ProjectConfig, Box<dyn std::error::Error>> {
        let kinds = vec![
            ProjectConfig::WebApp(WebAppConfig::default()),
            ProjectConfig::CliApp,
            ProjectConfig::Library,
            ProjectConfig::Embedded,
            ProjectConfig::Game,
        ];
        let mut project = Select::new("What type of project do you want to create?", kinds)
            .with_render_config(get_render_cfg())
            .prompt()?;
        match &mut project {
            ProjectConfig::WebApp(config) => {
                config.build()?;
            }
            ProjectConfig::CliApp => (),
            ProjectConfig::Library => (),
            ProjectConfig::Embedded => (),
            ProjectConfig::Game => (),
        };
        Ok(project)
    }
}
