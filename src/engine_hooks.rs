use std::mem::MaybeUninit;



pub static mut HOTRUN_FNS: MaybeUninit<crate::shared::HotRunFns> = MaybeUninit::uninit();



#[macro_export]
macro_rules! create_hooks {
	(
		$static_data_fn:path: $module_data_type:ty,
		$on_load_fn:path,
		$post_reload_fn:path,
		$pre_reload_reload_fn:path,
		$on_unload_reload_fn:path,
		$can_unload_reload_fn:path,
		$on_tick_reload_fn:path,
		$before_render:path,
	) => {
		
		pub fn static_data() -> &'static $module_data_type {
			unsafe {
				#[allow(static_mut_refs)]
				engine_hooks::MODULE_DATA.assume_init_ref()
			}
		}
		
		pub unsafe fn static_data_mut() -> &'static mut $module_data_type {
			unsafe {
				#[allow(static_mut_refs)]
				engine_hooks::MODULE_DATA.assume_init_mut()
			}
		}
		
		mod engine_hooks {
			
			use std::mem::MaybeUninit;
			
			pub static mut MODULE_DATA: MaybeUninit<$module_data_type> = MaybeUninit::uninit();
			
			
			
			// load
			
			#[unsafe(no_mangle)]
			pub static ON_LOAD_ORDER: f32 = $on_load_order;
			
			// this is guaranteed to be the first function called, which is why is takes the HotRun argument
			#[unsafe(no_mangle)]
			pub extern "C" fn on_load(hotrun_fns: $crate::shared::HotRunFns) -> $crate::errors::Result<()> {
				unsafe {
					$crate::engine_to_game_hooks::HOTRUN_FNS = MaybeUninit::new(hotrun_fns);
					MODULE_DATA = MaybeUninit::new($static_data_fn()?);
				}
				$on_load_fn()
			}
			
			
			
			// reload
			
			#[unsafe(no_mangle)]
			pub extern "C" fn post_reload() -> $crate::errors::Result<()> {
				$post_reload_fn()
			}
			
			#[unsafe(no_mangle)]
			pub extern "C" fn pre_reload() -> $crate::errors::Result<()> {
				$pre_reload_reload_fn()?;
				unsafe {
					MODULE_DATA.assume_init_drop();
				}
				$crate::errors::Result::Ok(())
			}
			
			
			
			// unload
			
			#[unsafe(no_mangle)]
			pub extern "C" fn on_unload() -> $crate::errors::Result<()> {
				$on_unload_reload_fn()?;
				unsafe {
					MODULE_DATA.assume_init_drop();
				}
				$crate::errors::Result::Ok(())
			}
			
			// if you have, for example, multiple threads that may still be running, this can let the engine know to wait until your conditions are met before unloading the dll
			#[unsafe(no_mangle)]
			pub extern "C" fn can_unload(is_for_reload: bool) -> $crate::errors::Result<bool> {
				$can_unload_reload_fn(is_for_reload)
			}
			
			
			
			// update functions
			
			#[unsafe(no_mangle)]
			pub extern "C" fn on_tick(dt: f32) -> $crate::errors::Result<()> { // this is called roughly 100 times per second, independently if framerate. You should aim to put as much game logic as you can in here
				$on_tick_reload_fn(dt)
			}
			
			#[unsafe(no_mangle)]
			pub extern "C" fn before_render(dt: f32) -> $crate::errors::Result<()> { // this is called every frame before rendering. Ideally, this would ONLY ever contain whatever is absolutely required
				$before_render(dt)
			}
			
			
			
		}
	};
}
