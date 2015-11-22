use libc::{c_short, c_int, uint32_t};
use base::WINDOW;

pub type mmask_t = uint32_t;

macro_rules! MASK {
	($btn:expr, $ev:expr) => (
		$ev << (($btn - 1) * 5)
	)
}

pub const BUTTON_RELEASED: mmask_t = 0o01;
pub const BUTTON_PRESSED:  mmask_t = 0o02;
pub const BUTTON_CLICKED:  mmask_t = 0o04;
pub const DOUBLE_CLICKED:  mmask_t = 0o10;
pub const TRIPLE_CLICKED:  mmask_t = 0o20;
pub const RESERVED_EVENT:  mmask_t = 0o40;

pub const BUTTON1_RELEASED:       mmask_t = MASK!(1, BUTTON_RELEASED);
pub const BUTTON1_PRESSED:        mmask_t = MASK!(1, BUTTON_PRESSED);
pub const BUTTON1_CLICKED:        mmask_t = MASK!(1, BUTTON_CLICKED);
pub const BUTTON1_DOUBLE_CLICKED: mmask_t = MASK!(1, DOUBLE_CLICKED);
pub const BUTTON1_TRIPLE_CLICKED: mmask_t = MASK!(1, TRIPLE_CLICKED);

pub const BUTTON2_RELEASED:       mmask_t = MASK!(2, BUTTON_RELEASED);
pub const BUTTON2_PRESSED:        mmask_t = MASK!(2, BUTTON_PRESSED);
pub const BUTTON2_CLICKED:        mmask_t = MASK!(2, BUTTON_CLICKED);
pub const BUTTON2_DOUBLE_CLICKED: mmask_t = MASK!(2, DOUBLE_CLICKED);
pub const BUTTON2_TRIPLE_CLICKED: mmask_t = MASK!(2, TRIPLE_CLICKED);

pub const BUTTON3_RELEASED:       mmask_t = MASK!(3, BUTTON_RELEASED);
pub const BUTTON3_PRESSED:        mmask_t = MASK!(3, BUTTON_PRESSED);
pub const BUTTON3_CLICKED:        mmask_t = MASK!(3, BUTTON_CLICKED);
pub const BUTTON3_DOUBLE_CLICKED: mmask_t = MASK!(3, DOUBLE_CLICKED);
pub const BUTTON3_TRIPLE_CLICKED: mmask_t = MASK!(3, TRIPLE_CLICKED);

pub const BUTTON4_RELEASED:       mmask_t = MASK!(4, BUTTON_RELEASED);
pub const BUTTON4_PRESSED:        mmask_t = MASK!(4, BUTTON_PRESSED);
pub const BUTTON4_CLICKED:        mmask_t = MASK!(4, BUTTON_CLICKED);
pub const BUTTON4_DOUBLE_CLICKED: mmask_t = MASK!(4, DOUBLE_CLICKED);
pub const BUTTON4_TRIPLE_CLICKED: mmask_t = MASK!(4, TRIPLE_CLICKED);

pub const BUTTON5_RELEASED:       mmask_t = MASK!(5, BUTTON_RELEASED);
pub const BUTTON5_PRESSED:        mmask_t = MASK!(5, BUTTON_PRESSED);
pub const BUTTON5_CLICKED:        mmask_t = MASK!(5, BUTTON_CLICKED);
pub const BUTTON5_DOUBLE_CLICKED: mmask_t = MASK!(5, DOUBLE_CLICKED);
pub const BUTTON5_TRIPLE_CLICKED: mmask_t = MASK!(5, TRIPLE_CLICKED);

pub const BUTTON_CTRL:  mmask_t = MASK!(6, 0o01);
pub const BUTTON_SHIFT: mmask_t = MASK!(6, 0o02);
pub const BUTTON_ALT:   mmask_t = MASK!(6, 0o04);

pub const REPORT_MOUSE_POSITION: mmask_t = MASK!(6, 0o10);
pub const ALL_MOUSE_EVENTS:      mmask_t = REPORT_MOUSE_POSITION - 1;

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct MEVENT {
	pub id:     c_short,
	pub x:      c_int,
	pub y:      c_int,
	pub z:      c_int,
	pub bstate: mmask_t,
}

#[cfg_attr(feature = "wide", link(name = "ncursesw"))]
#[cfg_attr(not(feature = "wide"), link(name = "ncurses"))]
extern "C" {
	pub fn has_mouse() -> bool;
	pub fn getmouse(event: *mut MEVENT) -> c_int;
	pub fn ungetmouse(event: *const MEVENT) -> c_int;
	pub fn mousemask(newmask: mmask_t, oldmask: *mut mmask_t) -> mmask_t;
	pub fn wenclose(win: *const WINDOW, y: c_int, x: c_int) -> bool;
	pub fn mouse_trafo(pY: *mut c_int, pX: *mut c_int, to_screen: bool) -> bool;
	pub fn wmouse_trafo(win: *const WINDOW, pY: *mut c_int, pX: *mut c_int, to_screen: bool) -> bool;
	pub fn mouseinterval(erval: c_int) -> c_int;
}
