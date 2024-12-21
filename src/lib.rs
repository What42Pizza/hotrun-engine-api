#![feature(try_trait_v2)]
#![feature(backtrace_frames)]



// suggested prelude usage:
// `pub use hotrun_engine_api::{prelude as api, log, errors::{Result::*, StdResultFns}};`
// `pub use hotrun_engine_api::ffi_string::{StrToFFI, StringToFFI};`

#[cfg(not(feature = "is-engine-dep"))]
pub mod prelude {
	pub use crate::for_games::*;
	pub use crate::shared::*;
	pub use crate::errors::*;
	pub use ffi_string::*;
}
#[cfg(feature = "is-engine-dep")]
pub mod prelude {
	pub use crate::shared::*;
	pub use crate::errors::*;
	pub use ffi_string::*;
}
pub use ffi_string;

#[cfg(not(feature = "is-engine-dep"))]
pub mod for_games;
#[cfg(not(feature = "is-engine-dep"))]
pub mod engine_to_game_hooks;
pub mod shared;
pub mod errors;
