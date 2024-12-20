use crate::shared::{HotRunFns, MessageButtons, MessageLevel};
use std::mem::MaybeUninit;



static mut HOTRUN_FNS: MaybeUninit<HotRunFns> = MaybeUninit::uninit();
static mut IS_SET: bool = false;

pub fn init_dll_connection(hotrun_fns: HotRunFns) {
	unsafe {
		if IS_SET { panic!("cannot init dll twice"); }
		IS_SET = true;
		HOTRUN_FNS = MaybeUninit::new(hotrun_fns);
	}
}



#[inline] pub fn exit() { unsafe { (HOTRUN_FNS.assume_init().exit)() } }

#[inline] pub fn log(message: &str) { unsafe { (HOTRUN_FNS.assume_init().log)(message) } }
#[inline] pub fn debug(message: &str) { unsafe { (HOTRUN_FNS.assume_init().debug)(message) } }
#[inline] pub fn message_box(title: &str, message: &str, level: MessageLevel, buttons: MessageButtons) { unsafe { (HOTRUN_FNS.assume_init().message_box)(title, message, level, buttons) } }



#[macro_export]
macro_rules! log {
	($format:expr $(, $value:expr)*) => {
		let mut message = format!($format $(, $value)*);
		message.push('\n');
		hotrun_engine_api::for_games::log_str(&message);
	};
}
