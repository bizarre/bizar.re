use proc_macro::inject_from_file;

pub struct ProgrammingConfig {
    pub bio: &'static str,
    pub github: &'static str,
}

#[inject_from_file(path = "Config.toml")]
pub struct Config {
    pub name: &'static str,
    pub pseudonym: &'static str,
    pub headline: &'static str,
    pub programming: ProgrammingConfig,
}
