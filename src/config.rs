use proc_macro::inject_from_file;

pub struct ProgrammingConfig {
    pub bio: &'static str,
    pub github: &'static str,
    pub github_repos: &'static [&'static str],
}

#[derive(PartialEq)]
pub struct SocialConfig {
    pub twitter: &'static str,
    pub github: &'static str,
    pub linkedin: &'static str,
}

#[inject_from_file(path = "Config.toml")]
pub struct Config {
    pub name: &'static str,
    pub pseudonym: &'static str,
    pub headline: &'static str,
    pub programming: ProgrammingConfig,
    pub social: SocialConfig,
}
