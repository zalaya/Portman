mod details;
mod kill;
mod table;

pub use details::{ ProcessDetails, details };
pub use kill::{ KillOutcome, KillSignal, kill };
pub use table::{ ProcessTable, UserDirectory };
