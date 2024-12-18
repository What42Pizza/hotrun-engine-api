macro_rules! dummy_fn {
	($return:ty; $($var:ident: $type:ty)*) => {
		{
			fn dummy($($var: $type, )*) -> $return {todo!()}
			dummy
		}
	};
}



#[derive(Copy, Clone)]
pub struct HotRun {
	
	pub exit: fn(),
	
}

impl HotRun {
	
	pub const fn empty() -> Self {
		Self {
			
			exit: dummy_fn!(();),
			
		}
	}
	
}
