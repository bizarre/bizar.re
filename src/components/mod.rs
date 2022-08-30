mod github;
mod header;
pub(crate) mod journal;
mod text_section;
mod tooltip;

pub use github::*;
pub use header::component as Header;
pub use journal::*;
pub use text_section::component as AboutSection;
pub use tooltip::component as Tooltip;
