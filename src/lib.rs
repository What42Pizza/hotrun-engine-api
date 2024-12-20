#[cfg(not(feature = "is-engine-dep"))]
pub mod prelude {
	pub use crate::for_games::*;
	pub use crate::shared::{*, FFIResult::*};
	pub use crate::engine_to_game_hooks as engine_hooks;
	pub use ffi_string::*;
}
#[cfg(feature = "is-engine-dep")]
pub mod prelude {
	pub use crate::for_engine::*;
	pub use crate::shared::{*, FFIResult::*};
	pub use ffi_string::*;
}

#[cfg(not(feature = "is-engine-dep"))]
pub mod for_games;
#[cfg(not(feature = "is-engine-dep"))]
pub mod engine_to_game_hooks;
#[cfg(feature = "is-engine-dep")]
pub mod for_engine;
pub mod shared;
