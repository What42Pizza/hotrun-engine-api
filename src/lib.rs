#![feature(try_trait_v2)]



#[cfg(not(feature = "is-engine-dep"))]
pub mod prelude {
	pub use crate::log;
	pub use crate::errors::{Result::*, StdResultFns};
	pub use ffi_string::{StrToFFI, StringToFFI};
	pub mod api {
		pub use crate::for_games::*;
		pub use crate::shared::*;
		pub use crate::errors::*;
		pub use ffi_string::*;
	}
}
#[cfg(feature = "is-engine-dep")]
pub mod prelude {
	pub use crate::errors::{Result::*, StdResultFns};
	pub use ffi_string::{StrToFFI, StringToFFI};
	pub mod api {
		pub use crate::shared::*;
		pub use crate::errors::*;
		pub use ffi_string::*;
	}
}
pub use ffi_string;

#[cfg(not(feature = "is-engine-dep"))]
pub mod for_games;
#[cfg(not(feature = "is-engine-dep"))]
pub mod engine_hooks;
pub mod shared;
pub mod errors;
