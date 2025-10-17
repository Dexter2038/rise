#[derive(Default)]
pub struct WebAppConfig {
    pub kind: Option<WebApp>,
    pub backend: Option<Backend>,
    pub frontend: Option<Frontend>,
    pub database: Option<Database>,
    pub deployment: Option<Deployment>,
    pub testings: Vec<Testing>,
    pub features: Vec<Features>,
}

#[derive(strum_macros::Display, PartialEq, Clone, Copy, strum_macros::EnumIter)]
pub enum WebApp {
    Backend,
    Frontend,
    Fullstack,
}

#[derive(strum_macros::Display, Clone, Copy, strum_macros::EnumIter)]
pub enum Backend {
    Actix,
    Axum,
    Rocket,
    Warp,
    Tide,
    Salvo,
    Gotham,
}

#[derive(strum_macros::Display, Clone, Copy, PartialEq)]
pub enum Frontend {
    Yew,
    Leptos,
    Seed,
    Svelte,
    React,
    Vue,
    Sauron,
    Minijinja,
    Tera,
    Askama,
    Handlebars,
}

#[derive(strum_macros::Display, Clone, Copy, strum_macros::EnumIter)]
pub enum Database {
    SQLx,
    Diesel,
    SeaORM,
    Sled,
}

#[derive(strum_macros::Display, Clone, Copy, strum_macros::EnumIter)]
pub enum Deployment {
    Docker,
    Kubernetes,
    Serverless,
    SelfHosted,
}

#[derive(strum_macros::Display, Clone, Copy, strum_macros::EnumIter)]
pub enum Testing {
    Api,
    SQLx,
    Mock,
}

#[derive(strum_macros::Display, Clone, Copy, strum_macros::EnumIter)]
#[allow(non_camel_case_types)]
pub enum Features {
    WebSockets,
    gRPC,
    GraphQL,
    RateLimiting,
}
