use crate::prelude::*;
use std::mem::MaybeUninit;



pub(crate) static mut HOTRUN_FNS: MaybeUninit<HotRunFns> = MaybeUninit::uninit();



#[macro_export]
macro_rules! create_hooks {
	(
		$on_load_fn:ident: order $on_load_order:expr,
		$post_reload_fn:ident: order $post_reload_order:expr,
		$pre_reload_reload_fn:ident: order $pre_reload_order:expr,
		$on_unload_reload_fn:ident: order $on_unload_order:expr,
		$can_unload_reload_fn:ident,
		$on_tick_reload_fn:ident: order $on_tick_order:expr,
		$on_world_update_reload_fn:ident: order $on_world_update_order:expr,
	) => { mod engine_hooks {
		
		
		
		// load
		
		#[unsafe(no_mangle)]
		pub static ON_LOAD_ORDER: f32 = $on_load_order;
		
		// this is guaranteed to be the first function called, which is why is takes the HotRun argument
		#[unsafe(no_mangle)]
		pub extern "C" fn on_load(hotrun_fns: $crate::shared::HotRunFns) -> $crate::shared::FFIResult<()> {
			unsafe {
				HOTRUN_FNS = MaybeUninit::new(hotrun_fns);
			}
			$on_load_fn().to_anyhow()
		}
		
		
		
		// reload
		
		#[unsafe(no_mangle)]
		pub static POST_RELOAD_ORDER: f32 = $post_reload_order;
		
		#[unsafe(no_mangle)]
		pub extern "C" fn post_reload() -> $crate::shared::FFIResult<()> {
			$post_reload_fn().to_anyhow()
		}
		
		#[unsafe(no_mangle)]
		pub static PRE_RELOAD_ORDER: f32 = $pre_reload_order;
		
		#[unsafe(no_mangle)]
		pub extern "C" fn pre_reload() -> $crate::shared::FFIResult<()> {
			$pre_reload_reload_fn().to_anyhow()
		}
		
		
		
		// unload
		
		#[unsafe(no_mangle)]
		pub static ON_UNLOAD_ORDER: f32 = $on_unload_order;
		
		#[unsafe(no_mangle)]
		pub extern "C" fn on_unload() -> $crate::shared::FFIResult<()> {
			$on_unload_reload_fn().to_anyhow()
		}
		
		// if you have, for example, multiple threads that may still be running, this can let the engine know to wait until your conditions are met before unloading the dll
		#[unsafe(no_mangle)]
		pub extern "C" fn can_unload(is_for_reload: bool) -> $crate::shared::FFIResult<bool> {
			$can_unload_reload_fn(is_for_reload).to_anyhow()
		}
		
		
		
		// update functions
		
		#[unsafe(no_mangle)]
		pub static ON_TICK_ORDER: f32 = $on_tick_order;
		
		#[unsafe(no_mangle)]
		pub extern "C" fn on_tick(dt: f32) -> $crate::shared::FFIResult<()> { // this is called roughly 100 times per second, independently if framerate. You should aim to put as much game logic as you can in here
			$on_tick_reload_fn(dt).to_anyhow()
		}
		
		#[unsafe(no_mangle)]
		pub static ON_WORLD_UPDATE_ORDER: f32 = $on_world_update_order;
		
		#[unsafe(no_mangle)]
		pub extern "C" fn on_world_update(dt: f32) -> $crate::shared::FFIResult<()> { // this is called every frame before rendering. Ideally, this would ONLY ever contain whatever is absolutely required
			$on_world_update_reload_fn(dt).to_anyhow()
		}
		
		
		
	}};
}
