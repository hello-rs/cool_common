use config::{Config, File};
use serde::Deserialize;

lazy_static::lazy_static! {
  #[derive(Debug)]
  pub static ref C_CONFIG: AppConfig ={
    let c= Config::builder().add_source(File::with_name("config/cool")).build().unwrap();
    c.try_deserialize().unwrap()
  };
}

#[derive(Debug, Deserialize)]
pub struct AppConfig {
    pub tracing: Tracing,
}

#[derive(Debug, Deserialize)]
pub struct Tracing {
    pub level: String,
    pub dir: String,
    pub name: String,
    pub console: bool,
}
