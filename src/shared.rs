use ffi_string::*;



#[derive(Copy, Clone)]
#[repr(C)]
pub struct HotRunFns {
	
	pub exit: extern "C" fn(),
	pub set_world_tick_rate: extern "C" fn(f32),
	
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
// if more args are needed, just combine args into tuples



// taken from rfd:

#[derive(Debug, Copy, Clone)]
#[repr(C)]
pub enum MessageLevel {
	Info,
	Warning,
	Error,
}

#[derive(Debug, Copy, Clone)]
#[repr(C)]
pub enum MessageButtons {
	Ok,
	OkCancel,
	YesNo,
	YesNoCancel,
}
