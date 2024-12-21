use std::{backtrace::Backtrace, convert::Infallible, fmt::Display, ops::{ControlFlow, FromResidual, Try}, result::Result as StdResult};
use self::Result::*;





#[derive(Debug)]
pub struct Error {
	pub messages: Vec<String>,
	pub trace: Backtrace,
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
	
	pub const BACKTRACE_TRAIL_LEN: usize = 1;
	
	pub fn new(msg: impl Into<String>) -> Self {
		Self {
			messages: vec!(msg.into()),
			trace: Backtrace::capture(),
		}
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
	
	pub fn context(&mut self, msg: impl ToString) {
		let Err(error) = self else { return; };
		error.messages.push(msg.to_string());
	}
	
	pub fn with_context(&mut self, msg: impl FnOnce() -> String) {
		let Err(error) = self else { return; };
		error.messages.push(msg());
	}
	
}
