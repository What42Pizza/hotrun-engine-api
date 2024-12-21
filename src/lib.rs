#![feature(try_trait_v2)]
#![feature(impl_trait_in_assoc_type)]



// suggested prelude usage: `use hotrun_engine_api::prelude as api;`

#[cfg(not(feature = "is-engine-dep"))]
pub mod prelude {
	pub use crate::for_games::*;
	pub use crate::shared::*;
	pub use crate::errors::{*, Result::*};
	pub use ffi_string::*;
}
#[cfg(feature = "is-engine-dep")]
pub mod prelude {
	pub use crate::for_engine::*;
	pub use crate::shared::*;
	pub use crate::errors::{*, Result::*};
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
