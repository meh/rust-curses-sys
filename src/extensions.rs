use libc::{c_void, c_char, c_int};
use base::{WINDOW, SCREEN};

pub type WINDOW_CB = extern "C" fn(*mut WINDOW, *mut c_void) -> c_int;
pub type SCREEN_CB = extern "C" fn(*mut SCREEN, *mut c_void) -> c_int;

#[cfg_attr(feature = "wide", link(name = "ncursesw"))]
#[cfg_attr(not(feature = "wide"), link(name = "ncurses"))]
extern "C" {
	pub fn is_cleared(win: *const WINDOW) -> bool;
	pub fn is_idcok(win: *const WINDOW) -> bool;
	pub fn is_idlok(win: *const WINDOW) -> bool;
	pub fn is_immedok(win: *const WINDOW) -> bool;
	pub fn is_keypad(win: *const WINDOW) -> bool;
	pub fn is_leaveok(win: *const WINDOW) -> bool;
	pub fn is_nodelay(win: *const WINDOW) -> bool;
	pub fn is_notimeout(win: *const WINDOW) -> bool;
	pub fn is_pad(win: *const WINDOW) -> bool;
	pub fn is_scrollok(win: *const WINDOW) -> bool;
	pub fn is_subwin(win: *const WINDOW) -> bool;
	pub fn is_syncok(win: *const WINDOW) -> bool;
	pub fn wgetparent(win: *const WINDOW) -> *mut WINDOW;
	pub fn wgetdelay(win: *const WINDOW) -> c_int;
	pub fn wgetscrreg(win: *const WINDOW, top: *mut c_int, bottom: *mut c_int) -> c_int;

	pub fn is_term_resized(lines: c_int, columns: c_int) -> bool;
	pub fn resize_term(lines: c_int, columns: c_int) -> bool;
	pub fn resizeterm(lines: c_int, columns: c_int) -> c_int;

	pub fn getattrs(win: *const WINDOW) -> c_int;
	pub fn getbegx(win: *const WINDOW) -> c_int;
	pub fn getbegy(win: *const WINDOW) -> c_int;
	pub fn getcurx(win: *const WINDOW) -> c_int;
	pub fn getcury(win: *const WINDOW) -> c_int;
	pub fn getmaxx(win: *const WINDOW) -> c_int;
	pub fn getmaxy(win: *const WINDOW) -> c_int;
	pub fn getparx(win: *const WINDOW) -> c_int;
	pub fn getpary(win: *const WINDOW) -> c_int;

	pub fn use_default_colors() -> c_int;
	pub fn assume_default_colors(fg: c_int, bg: c_int) -> c_int;

	pub fn get_escdelay() -> c_int;
	pub fn set_escdelay(size: c_int) -> c_int;
	pub fn set_tabsize(size: c_int) -> c_int;
	pub fn use_screen(scr: *mut SCREEN, cb: SCREEN_CB, data: *mut c_void) -> c_int;
	pub fn use_window(win: *mut WINDOW, cb: WINDOW_CB, data: *mut c_void) -> c_int;

	pub fn use_legacy_coding(level: c_int) -> c_int;

	pub fn wresize(win: *mut WINDOW, lines: c_int, columns: c_int) -> c_int;

	pub fn define_key(definition: *const c_char, keycode: c_int) -> c_int;
	pub fn keybound(keycode: c_int, count: c_int) -> *const c_char;
	pub fn key_deifned(definition: *const c_char) -> c_int;

	pub fn curses_version() -> *const c_char;
	pub fn use_extended_names(enable: bool) -> c_int;
}
