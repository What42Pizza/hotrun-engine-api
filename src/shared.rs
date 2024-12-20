use anyhow::*;



#[derive(Copy, Clone)]
#[repr(C)]
pub struct HotRunFns {
	
	pub exit: fn(),
	pub get_fn: fn(name: &str) -> Option<fn()>,
	pub set_fn: fn(name: &str, func: fn()) -> Result<()>,
	
	pub log: fn(&str),
	pub debug: fn(message: &str),
	pub message_box: fn(title: &str, message: &str, level: MessageLevel, buttons: MessageButtons),
	
}



pub(crate) trait IsFunctionPointer: Copy {}

impl<Arg1, Ret> IsFunctionPointer for extern "C" fn(Arg1) -> Ret {}
impl<Arg1, Arg2, Ret> IsFunctionPointer for extern "C" fn(Arg1, Arg2) -> Ret {}
impl<Arg1, Arg2, Arg3, Ret> IsFunctionPointer for extern "C" fn(Arg1, Arg2, Arg3) -> Ret {}
impl<Arg1, Arg2, Arg3, Arg4, Ret> IsFunctionPointer for extern "C" fn(Arg1, Arg2, Arg3, Arg4) -> Ret {}
impl<Arg1, Arg2, Arg3, Arg4, Arg5, Ret> IsFunctionPointer for extern "C" fn(Arg1, Arg2, Arg3, Arg4, Arg5) -> Ret {}
impl<Arg1, Arg2, Arg3, Arg4, Arg5, Arg6, Ret> IsFunctionPointer for extern "C" fn(Arg1, Arg2, Arg3, Arg4, Arg5, Arg6) -> Ret {}
impl<Arg1, Arg2, Arg3, Arg4, Arg5, Arg6, Arg7, Ret> IsFunctionPointer for extern "C" fn(Arg1, Arg2, Arg3, Arg4, Arg5, Arg6, Arg7) -> Ret {}
impl<Arg1, Arg2, Arg3, Arg4, Arg5, Arg6, Arg7, Arg8, Ret> IsFunctionPointer for extern "C" fn(Arg1, Arg2, Arg3, Arg4, Arg5, Arg6, Arg7, Arg8) -> Ret {}



// taken from rfd:

#[derive(Debug, Clone, Copy)]
#[repr(C)]
pub enum MessageLevel {
	Info,
	Warning,
	Error,
}

#[derive(Debug, Clone)]
#[repr(C)]
pub enum MessageButtons {
	Ok,
	OkCancel,
	YesNo,
	YesNoCancel,
	/// One customizable button.
	/// Notice that in Windows, this only works with the feature *common-controls-v6* enabled
	OkCustom(String),
	/// Two customizable buttons.
	/// Notice that in Windows, this only works with the feature *common-controls-v6* enabled
	OkCancelCustom(String, String),
	/// Three customizable buttons.
	/// Notice that in Windows, this only works with the feature *common-controls-v6* enabled
	YesNoCancelCustom(String, String, String),
}
