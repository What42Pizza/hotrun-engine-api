use anyhow::*;
use ffi_string::*;



#[derive(Copy, Clone)]
#[repr(C)]
pub struct HotRunFns {
	
	pub exit: extern "C" fn(),
	pub get_fn: extern "C" fn(name: FFIStr) -> Option<extern "C" fn()>,
	pub set_fn: extern "C" fn(name: FFIStr, func: extern "C" fn()) -> bool,
	
	pub log: extern "C" fn(FFIStr),
	pub debug: extern "C" fn(message: FFIStr),
	pub message_box: extern "C" fn(title: FFIStr, message: FFIStr, level: MessageLevel, buttons: MessageButtons),
	
}



pub(crate) trait IsCFunctionPointer: Copy {}

impl<Ret> IsCFunctionPointer for extern "C" fn() -> Ret {}
impl<Arg1, Ret> IsCFunctionPointer for extern "C" fn(Arg1) -> Ret {}
impl<Arg1, Arg2, Ret> IsCFunctionPointer for extern "C" fn(Arg1, Arg2) -> Ret {}
impl<Arg1, Arg2, Arg3, Ret> IsCFunctionPointer for extern "C" fn(Arg1, Arg2, Arg3) -> Ret {}
impl<Arg1, Arg2, Arg3, Arg4, Ret> IsCFunctionPointer for extern "C" fn(Arg1, Arg2, Arg3, Arg4) -> Ret {}
impl<Arg1, Arg2, Arg3, Arg4, Arg5, Ret> IsCFunctionPointer for extern "C" fn(Arg1, Arg2, Arg3, Arg4, Arg5) -> Ret {}
impl<Arg1, Arg2, Arg3, Arg4, Arg5, Arg6, Ret> IsCFunctionPointer for extern "C" fn(Arg1, Arg2, Arg3, Arg4, Arg5, Arg6) -> Ret {}
impl<Arg1, Arg2, Arg3, Arg4, Arg5, Arg6, Arg7, Ret> IsCFunctionPointer for extern "C" fn(Arg1, Arg2, Arg3, Arg4, Arg5, Arg6, Arg7) -> Ret {}
impl<Arg1, Arg2, Arg3, Arg4, Arg5, Arg6, Arg7, Arg8, Ret> IsCFunctionPointer for extern "C" fn(Arg1, Arg2, Arg3, Arg4, Arg5, Arg6, Arg7, Arg8) -> Ret {}



pub enum FFIResult<T> {
	Ok (T),
	Err (Error),
}

impl<T> FFIResult<T> {
	pub fn to_anyhow(self) -> Result<T> {
		match self {
			Self::Ok(v) => Ok(v),
			Self::Err(err) => Err(err),
		}
	}
}



// taken from rfd:

#[derive(Debug, Clone, Copy)]
#[repr(C)]
pub enum MessageLevel {
	Info,
	Warning,
	Error,
}

#[repr(C)]
pub enum MessageButtons<'a> {
	Ok,
	OkCancel,
	YesNo,
	YesNoCancel,
	/// One customizable button.
	/// Notice that in Windows, this only works with the feature *common-controls-v6* enabled
	OkCustom(FFIStr<'a>),
	/// Two customizable buttons.
	/// Notice that in Windows, this only works with the feature *common-controls-v6* enabled
	OkCancelCustom(FFIStr<'a>, FFIStr<'a>),
	/// Three customizable buttons.
	/// Notice that in Windows, this only works with the feature *common-controls-v6* enabled
	YesNoCancelCustom(FFIStr<'a>, FFIStr<'a>, FFIStr<'a>),
}
