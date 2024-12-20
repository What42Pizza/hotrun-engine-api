use crate::{engine_to_game_hooks::HOTRUN_FNS, shared::{IsCFunctionPointer, MessageButtons, MessageLevel}};
use std::mem::transmute;
use ffi_string::*;





#[inline] pub fn exit() { unsafe { (HOTRUN_FNS.assume_init().exit)() } }

#[allow(private_bounds)]
#[inline] pub fn get_fn<T: IsCFunctionPointer>(name: &str) -> Option<T> {
	unsafe {
		let func = ((HOTRUN_FNS.assume_init().get_fn)(FFIStr::new(name)))?;
		let func = *transmute::<&extern "C" fn(), &T>(&func);
		Some(func)
	}
}

#[allow(private_bounds)]
#[inline] pub fn set_fn<T: IsCFunctionPointer>(name: &str, func: T) -> bool {
	unsafe {
		let func = *transmute::<&T, &extern "C" fn()>(&func);
		(HOTRUN_FNS.assume_init().set_fn)(FFIStr::new(name), func)
	}
}



#[inline] pub fn log(message: &str) { unsafe { (HOTRUN_FNS.assume_init().log)(FFIStr::new(message)) } }
#[inline] pub fn debug(message: &str) { unsafe { (HOTRUN_FNS.assume_init().debug)(FFIStr::new(message)) } }
#[inline] pub fn message_box(title: &str, message: &str, level: MessageLevel, buttons: MessageButtons) { unsafe { (HOTRUN_FNS.assume_init().message_box)(FFIStr::new(title), FFIStr::new(message), level, buttons) } }





#[macro_export]
macro_rules! log {
	($format:expr $(, $value:expr)*) => {
		let mut message = format!($format $(, $value)*);
		message.push('\n');
		print!("{message}");
		hotrun_engine_api::for_games::log(&message);
	};
}
