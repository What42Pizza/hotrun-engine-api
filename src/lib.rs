#![feature(try_trait_v2)]
#![feature(backtrace_frames)]



// suggested prelude usage: `use hotrun_engine_api::{prelude as api, log, errors::{Result::*, StdResultFns}};`

#[cfg(not(feature = "is-engine-dep"))]
pub mod prelude {
	pub use crate::for_games::*;
	pub use crate::shared::*;
	pub use crate::errors::*;
	pub use ffi_string::*;
}
#[cfg(feature = "is-engine-dep")]
pub mod prelude {
	pub use crate::for_engine::*;
	pub use crate::shared::*;
	pub use crate::errors::*;
	pub use ffi_string::*;
}

#[cfg(not(feature = "is-engine-dep"))]
pub mod for_games;
#[cfg(not(feature = "is-engine-dep"))]
pub mod engine_to_game_hooks;
#[cfg(feature = "is-engine-dep")]
pub mod for_engine;
pub mod shared;
pub mod errors;
