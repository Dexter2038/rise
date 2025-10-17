use super::web::WebAppConfig;

#[derive(strum_macros::Display, strum_macros::EnumIter)]
pub enum ProjectConfig {
    WebApp(WebAppConfig),
    CliApp,
    Library,
    Embedded,
    Game,
}
