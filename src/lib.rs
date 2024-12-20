#[cfg(not(feature = "is-engine-dep"))]
pub mod prelude {
	pub use crate::for_games::*;
	pub use crate::shared::*;
}
#[cfg(feature = "is-engine-dep")]
pub mod prelude {
	pub use crate::for_engine::*;
	pub use crate::shared::*;
}

#[cfg(not(feature = "is-engine-dep"))]
pub mod for_games;
#[cfg(feature = "is-engine-dep")]
pub mod for_engine;
pub mod shared;
