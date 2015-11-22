use libc::{c_int, wchar_t};
use base::{WINDOW, cchar_t};

#[inline(always)]
pub unsafe fn WACS(c: u8) -> cchar_t {
	*_nc_wacs.offset(c as isize)
}

#[link(name = "ncursesw")]
extern "C" {
	pub static _nc_wacs: *const cchar_t;

	pub fn add_wch(wch: *const cchar_t) -> c_int;
	pub fn wadd_wch(win: *mut WINDOW, wch: *const cchar_t) -> c_int;
	pub fn mvadd_wch(y: c_int, x: c_int, wch: *const cchar_t) -> c_int;
	pub fn mavwadd_wch(win: *mut WINDOW, y: c_int, x: c_int, wch: *const cchar_t) -> c_int;
	pub fn echo_wchar(wch: *const cchar_t) -> c_int;
	pub fn wecho_wchar(win: *mut WINDOW, wch: *const cchar_t) -> c_int;

	pub fn bkgrnd(wch: *const cchar_t) -> c_int;
	pub fn wbkgrnd(win: *mut WINDOW, wch: *const cchar_t) -> c_int;
	pub fn bkgrndset(wch: *const cchar_t);
	pub fn wbkgrndset(win: *mut WINDOW, wch: *const cchar_t) -> c_int;
	pub fn getbkgrnd(wch: *mut cchar_t) -> c_int;
	pub fn wgetbkgrnd(win: *mut WINDOW, wch: *mut cchar_t) -> c_int;

	pub fn get_wch(wch: *mut wchar_t) -> c_int;
	pub fn wget_wch(win: *mut WINDOW, wch: *mut wchar_t) -> c_int;
	pub fn mvget_wch(y: c_int, x: c_int, wch: *mut wchar_t) -> c_int;
	pub fn mvwget_wch(win: *mut WINDOW, y: c_int, x: c_int, wch: *mut wchar_t) -> c_int;
	pub fn unget_wch(wch: wchar_t) -> c_int;

	pub fn in_wch(wcval: *mut cchar_t) -> c_int;
	pub fn mvin_wch(y: c_int, x: c_int, wcval: *mut cchar_t) -> c_int;
	pub fn mvwin_wch(win: *mut WINDOW, y: c_int, x: c_int, wcval: *mut cchar_t) -> c_int;
	pub fn win_wch(win: *mut WINDOW, wcval: *mut cchar_t) -> c_int;

	pub fn ins_wstr(wstr: *const wchar_t) -> c_int;
	pub fn ins_nwstr(wstr: *const wchar_t, n: c_int) -> c_int;
	pub fn wins_wstr(win: *mut WINDOW, wstr: *const wchar_t) -> c_int;
	pub fn wins_nwstr(win: *mut WINDOW, wstr: *const wchar_t, n: c_int) -> c_int;
	pub fn mvins_wstr(y: c_int, x: c_int, wstr: *const wchar_t) -> c_int;
	pub fn mvins_nwstr(y: c_int, x: c_int, wstr: *const wchar_t, n: c_int) -> c_int;
	pub fn mvwins_wstr(win: *mut WINDOW, y: c_int, x: c_int, wstr: *const wchar_t) -> c_int;
	pub fn mvwins_nwstr(win: *mut WINDOW, y: c_int, x: c_int, wstr: *const wchar_t, n: c_int) -> c_int;

	pub fn addwstr(wstr: *const wchar_t) -> c_int;
	pub fn addnwstr(wstr: *const wchar_t, n: c_int) -> c_int;
	pub fn waddwstr(win: *mut WINDOW, wstr: *const wchar_t) -> c_int;
	pub fn waddnwstr(win: *mut WINDOW, wstr: *const wchar_t) -> c_int;
	pub fn mvaddwstr(y: c_int, x: c_int, wstr: *const wchar_t) -> c_int;
	pub fn mvaddnwstr(y: c_int, x: c_int, wstr: *const wchar_t, n: c_int) -> c_int;
	pub fn mvwaddwstr(win: *mut WINDOW, y: c_int, x: c_int, wstr: *const wchar_t) -> c_int;
	pub fn mvwaddnwstr(win: *mut WINDOW, y: c_int, x: c_int, wstr: *const wchar_t, n: c_int) -> c_int;

	pub fn inwstr(string: *mut wchar_t) -> c_int;
	pub fn innwstr(string: *mut wchar_t, n: c_int) -> c_int;
	pub fn winwstr(win: *mut WINDOW, string: *mut wchar_t) -> c_int;
	pub fn winnwstr(win: *mut WINDOW, string: *mut wchar_t, n: c_int) -> c_int;
	pub fn mvinwstr(y: c_int, x: c_int, string: *mut wchar_t) -> c_int;
	pub fn mvinnwstr(y: c_int, x: c_int, string: *mut wchar_t, n: c_int) -> c_int;
	pub fn mvwinwstr(win: *mut WINDOW, y: c_int, x: c_int, string: *mut wchar_t) -> c_int;
	pub fn mvwinnwstr(win: *mut WINDOW, y: c_int, x: c_int, string: *mut wchar_t, n: c_int) -> c_int;

	pub fn border_set(ls: *const cchar_t, rs: *const cchar_t, ts: *const cchar_t, bs: *const cchar_t, tl: *const cchar_t, tr: *const cchar_t, bl: *const cchar_t, br: *const cchar_t) -> c_int;
	pub fn wborder_set(win: *mut WINDOW, ls: *const cchar_t, rs: *const cchar_t, ts: *const cchar_t, bs: *const cchar_t, tl: *const cchar_t, tr: *const cchar_t, bl: *const cchar_t, br: *const cchar_t) -> c_int;
	pub fn box_set(win: *mut WINDOW, verch: *const cchar_t, horch: *const cchar_t) -> c_int;
	pub fn hline_set(wch: *const cchar_t, n: c_int) -> c_int;
	pub fn whline_set(win: *mut WINDOW, wch: *const cchar_t, n: c_int) -> c_int;
	pub fn mvhline_set(y: c_int, x: c_int, wch: *const cchar_t, n: c_int) -> c_int;
	pub fn mvwhline_set(win: *mut WINDOW, y: c_int, x: c_int, wch: *const cchar_t, n: c_int) -> c_int;
	pub fn vline_set(wch: *const cchar_t, n: c_int) -> c_int;
	pub fn wvline_set(win: *mut WINDOW, wch: *const cchar_t, n: c_int) -> c_int;
	pub fn mvvline_set(y: c_int, x: c_int, wch: *const cchar_t, n: c_int) -> c_int;
	pub fn mvwvline_set(win: *mut WINDOW, y: c_int, x: c_int, wch: *const cchar_t, n: c_int) -> c_int;
}
