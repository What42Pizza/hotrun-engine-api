use std::{convert::Infallible, fmt::Display, ops::{ControlFlow, FromResidual, Try}, result::Result as StdResult};
use ffi_string::*;
use self::Result::*;





#[derive(Debug, Clone)]
#[repr(C)]
pub struct Error {
	pub msg: FFIString,
	pub contexts: Vec<FFIString>,
	pub cause: Option<Box<Error>>,
}

impl Display for Error {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		writeln!(f, "Caused by: {}", self.msg)?;
		for context in &self.contexts {
			writeln!(f, "While: {context}")?;
		}
		if let Some(cause) = &self.cause {
			writeln!(f, "Caused by:")?;
			cause.fmt(f)
		} else {
			StdResult::Ok(())
		}
	}
}

impl Error {
	pub fn new(msg: impl ToString) -> Self {
		Self {
			msg: msg.to_string().into_ffi_string(),
			contexts: vec!(),
			cause: None,
		}
	}
	pub fn with_cause(msg: impl ToString, cause: Error) -> Self {
		Self {
			msg: msg.to_string().into_ffi_string(),
			contexts: vec!(),
			cause: Some(Box::new(cause)),
		}
	}
}

impl<T: std::error::Error> From<T> for Error {
	fn from(value: T) -> Self {
		Self::new(value)
	}
}





#[derive(Debug, Clone)]
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

impl<T, E: Into<Error>> StdResultFns<T> for StdResult<T, E> {
	fn to_api_result(self) -> Result<T> {
		match self {
			StdResult::Ok(v) => Result::Ok(v),
			StdResult::Err(err) => Result::Err(err.into()),
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
	
	pub fn context(mut self, msg: impl ToString) -> Self {
		let Err(error) = &mut self else { return self; };
		error.contexts.push(msg.to_string().into_ffi_string());
		self
	}
	
	pub fn with_context<F, M>(mut self, msg: F) -> Self
	where 
		M: Display,
		F: FnOnce() -> M,
	{
		let Err(error) = &mut self else { return self; };
		error.contexts.push(msg().to_string().to_ffi_string());
		self
	}
	
}
