use std::{backtrace::Backtrace, convert::Infallible, fmt::Display, ops::{ControlFlow, FromResidual, Try}, result::Result as StdResult};
use ffi_string::*;
use self::Result::*;





#[derive(Debug)]
#[repr(C)]
pub struct Error {
	pub messages: Vec<FFIString>,
	pub trace: FFIString,
}

impl Display for Error {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		let mut messages = self.messages.iter();
		if let Some(msg) = messages.next() {
			writeln!(f, "{msg}")?;
		}
		for msg in messages {
			writeln!(f, "| {msg}")?;
		}
		write!(f, "Trace: {}", self.trace)
	}
}

impl Error {
	
	pub fn new(msg: impl Into<String>) -> Self {
		Self {
			messages: vec!(msg.into().into_ffi_string()),
			trace: Self::get_backtrace(1),
		}
	}
	
	pub fn get_backtrace(pop_count: usize) -> FFIString {
		let mut output = String::new();
		for frame in Backtrace::capture().frames().iter().skip(pop_count + 1) {
			output += &format!("{frame:?}\n");
		}
		output.into_ffi_string()
	}
	
}





#[derive(Debug)]
#[repr(C)]
pub enum Result<T> {
	Ok (T),
	Err (Error),
}

impl<T> Try for Result<T> {
	type Output = T;
	type Residual = Result<Infallible>;
	fn branch(self) -> ControlFlow<Self::Residual, Self::Output> {
		match self {
			Ok(v) => ControlFlow::Continue(v),
			Err(err) => ControlFlow::Break(Result::Err(err)),
		}
	}
	fn from_output(output: Self::Output) -> Self {
		Ok(output)
	}
}

impl<T> FromResidual for Result<T> {
	fn from_residual(residual: <Self as Try>::Residual) -> Self {
		let Result::Err(err) = residual;
		Err(err)
	}
}

impl<T, E: Into<Error>> FromResidual<StdResult<Infallible, E>> for Result<T> {
	fn from_residual(residual: StdResult<Infallible, E>) -> Self {
		let StdResult::Err(err) = residual;
		Err(err.into())
	}
}

pub trait StdResultFns<T> {
	fn to_api_result(self) -> Result<T>;
}

impl<T, E: ToString> StdResultFns<T> for StdResult<T, E> {
	fn to_api_result(self) -> Result<T> {
		match self {
			StdResult::Ok(v) => Result::Ok(v),
			StdResult::Err(err) => Result::Err(Error {
				messages: vec!(err.to_string().into_ffi_string()),
				trace: Error::get_backtrace(2),
			}),
		}
	}
}

impl<T> Result<T> {
	
	#[cfg(feature = "result-unwrap")]
	pub fn unwrap(self, msg: impl AsRef<str>) -> T {
		match self {
			Ok(v) => v,
			Err(err) => panic!("Called 'unwrap' on an err variant: {err}", msg.as_ref()),
		}
	}
	
	pub fn expect(self, msg: impl AsRef<str>) -> T {
		match self {
			Ok(v) => v,
			Err(err) => panic!("Called 'expect' on an err variant: {}\nError: {err}", msg.as_ref()),
		}
	}
	
	pub fn context(mut self, msg: impl Display) -> Self {
		let Err(error) = &mut self else { return self; };
		error.messages.push(msg.to_string().into_ffi_string());
		self
	}
	
	pub fn with_context<F, M>(mut self, msg: F) -> Self
	where 
		M: Display,
		F: FnOnce() -> M,
	{
		let Err(error) = &mut self else { return self; };
		error.messages.push(msg().to_string().to_ffi_string());
		self
	}
	
}
