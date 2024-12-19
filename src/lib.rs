#[derive(Copy, Clone)]
pub struct HotRun {
	
	pub exit: fn(),
	
	pub debug: fn(message: &str),
	pub message_box: fn(title: &str, message: &str, level: MessageLevel),
	pub adv_message_box: fn(title: &str, message: &str, level: MessageLevel, buttons: MessageButtons),
	
}



// taken from rfd:

#[derive(Debug, Clone, Copy)]
pub enum MessageLevel {
	Info,
	Warning,
	Error,
}

#[derive(Debug, Clone)]
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
