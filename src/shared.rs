use anyhow::*;



#[repr(C)]
pub struct FFIStr<'a> {
	bytes: &'a u8,
	len: u32,
}

impl<'a> FFIStr<'a> {
	pub fn new(from: &'a str) -> Self {
		unsafe {
			Self {
				bytes: &*from.as_ptr(),
				len: from.as_bytes().len() as u32,
			}
		}
	}
	pub fn to_string(&self) -> String {
		unsafe {
			core::str::from_raw_parts(self.bytes, self.len as usize).to_string()
		}
	}
}



#[derive(Copy, Clone)]
#[repr(C)]
pub struct HotRunFns {
	
	pub exit: extern "C" fn(),
	pub get_fn: extern "C" fn(name: FFIStr) -> Option<extern "C" fn()>,
	pub set_fn: extern "C" fn(name: FFIStr, func: extern "C" fn()) -> Result<()>,
	
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
