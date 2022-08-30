mod journal;
mod photography;
mod programmer;

pub use journal::page as JournalEntryPage;
pub use photography::page as PhotographyPage;
pub use programmer::page as ProgrammerPage;

pub(crate) use journal::JournalEntry;
