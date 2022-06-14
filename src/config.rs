use proc_macro::inject_from_file;

#[inject_from_file(path = "Config.toml")]
#[derive(Debug)]
pub struct Config {
    pub name: &'static str,
    pub pseudonym: &'static str,
}
