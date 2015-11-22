use libc::{c_void, c_int};
use base::{WINDOW};

#[derive(Debug)]
#[repr(C)]
pub struct PANEL {
	pub win:   *mut WINDOW,
	pub below: *mut PANEL,
	pub above: *mut PANEL,
	pub user:  *mut c_void,
}

#[cfg_attr(feature = "wide", link(name = "panelw"))]
#[cfg_attr(not(feature = "wide"), link(name = "panel"))]
extern "C" {
	pub fn new_panel(win: *mut WINDOW) -> *mut PANEL;
	pub fn bottom_panel(pan: *mut PANEL) -> c_int;
	pub fn top_panel(pan: *mut PANEL) -> c_int;
	pub fn show_panel(pan: *mut PANEL) -> c_int;
	pub fn update_panels();
	pub fn hide_panel(pan: *mut PANEL) -> c_int;
	pub fn panel_window(pan: *const PANEL) -> *mut WINDOW;
	pub fn replace_panel(pan: *mut PANEL, window: *mut WINDOW) -> c_int;
	pub fn move_panel(pan: *mut PANEL, starty: c_int, startx: c_int) -> c_int;
	pub fn panel_hidden(pan: *const PANEL) -> c_int;
	pub fn panel_above(pan: *const PANEL) -> *mut PANEL;
	pub fn panel_below(pan: *const PANEL) -> *mut PANEL;
	pub fn set_panel_userptr(pan: *mut PANEL, ptr: *const c_void) -> c_int;
	pub fn panel_userptr(pan: *const PANEL) -> *const c_void;
	pub fn del_panel(pan: *mut PANEL) -> c_int;
}
