use libc::{c_void, c_char, c_uchar, c_short, c_ushort, c_int};
use base::{WINDOW, chtype, KEY_MAX};

pub type Menu_Options = c_int;
pub type Item_Options = c_int;

#[derive(Debug)]
#[repr(C)]
pub struct TEXT {
	pub string: *const c_char,
	pub length: c_ushort,
}

#[repr(C)]
pub struct ITEM {
	pub name:        TEXT,
	pub description: TEXT,
	pub imenu:       *const MENU,
	pub userptr:     *mut c_void,
	pub opt:         Item_Options,
	pub index:       c_short,
	pub y:           c_short,
	pub x:           c_short,
	pub value:       bool,

	pub left:  *mut ITEM,
	pub right: *mut ITEM,
	pub up:    *mut ITEM,
	pub down:  *mut ITEM,
}

pub type Menu_Hook = extern "C" fn(*mut MENU);

#[repr(C)]
pub struct MENU {
	pub height:   c_short,
	pub width:    c_short,
	pub rows:     c_short,
	pub cols:     c_short,
	pub frows:    c_short,
	pub fcols:    c_short,
	pub arows:    c_short,
	pub namelen:  c_short,
	pub desclen:  c_short,
	pub marklen:  c_short,
	pub itemlen:  c_short,
	pub spc_desc: c_short,
	pub spc_cols: c_short,
	pub spc_rows: c_short,
	pub pattern:  *mut c_char,
	pub pindex:   c_short,
	pub win:      *mut WINDOW,
	pub sub:      *mut WINDOW,
	pub userwin:  *mut WINDOW,
	pub usersub:  *mut WINDOW,
	pub items:    *mut *mut ITEM,
	pub nitems:   c_short,
	pub curitem:  *mut ITEM,
	pub toprow:   c_short,
	pub fore:     chtype,
	pub back:     chtype,
	pub grey:     chtype,
	pub pad:      c_uchar,

	pub menuinit: Menu_Hook,
	pub menuterm: Menu_Hook,
	pub iteminit: Menu_Hook,
	pub itemterm: Menu_Hook,

	pub userptr: *mut c_void,
	pub mark:    *mut c_char,

	pub opt:    Menu_Options,
	pub status: c_ushort,
}

pub const REQ_LEFT_ITEM:     c_int = KEY_MAX + 1;
pub const REQ_RIGHT_ITEM:    c_int = KEY_MAX + 2;
pub const REQ_UP_ITEM:       c_int = KEY_MAX + 3;
pub const REQ_DOWN_ITEM:     c_int = KEY_MAX + 4;
pub const REQ_SCR_ULINE:     c_int = KEY_MAX + 5;
pub const REQ_SCR_DLINE:     c_int = KEY_MAX + 6;
pub const REQ_SCR_DPAGE:     c_int = KEY_MAX + 7;
pub const REQ_SCR_UPAGE:     c_int = KEY_MAX + 8;
pub const REQ_FIRST_ITEM:    c_int = KEY_MAX + 9;
pub const REQ_LAST_ITEM:     c_int = KEY_MAX + 10;
pub const REQ_NEXT_ITEM:     c_int = KEY_MAX + 11;
pub const REQ_PREV_ITEM:     c_int = KEY_MAX + 12;
pub const REQ_TOGGLE_ITEM:   c_int = KEY_MAX + 13;
pub const REQ_CLEAR_PATTERN: c_int = KEY_MAX + 14;
pub const REQ_BACK_PATTERN:  c_int = KEY_MAX + 15;
pub const REQ_NEXT_MATCH:    c_int = KEY_MAX + 16;
pub const REQ_PREV_MATCH:    c_int = KEY_MAX + 17;

pub const MIN_MENU_COMMAND: c_int = KEY_MAX + 1;
pub const MAX_MENU_COMMAND: c_int = KEY_MAX + 17;

#[cfg_attr(feature = "wide", link(name = "menuw"))]
#[cfg_attr(not(feature = "wide"), link(name = "menu"))]
extern "C" {
	pub fn new_menu(items: *mut *mut ITEM) -> *mut MENU;
	pub fn free_menu(menu: *mut MENU) -> c_int;

	pub fn set_menu_items(menu: *mut MENU, items: *mut *mut ITEM) -> c_int;
	pub fn menu_items(menu: *const MENU) -> *mut *mut ITEM;
	pub fn item_count(menu: *const MENU) -> c_int;

	pub fn set_current_item(menu: *mut MENU, item: *const ITEM) -> c_int;
	pub fn current_item(menu: *const MENU) -> *mut ITEM;
	pub fn set_top_row(menu: *mut MENU, row: c_int) -> c_int;
	pub fn top_row(menu: *const MENU) -> c_int;
	pub fn item_index(item: *const ITEM) -> c_int;

	pub fn set_item_opts(item: *mut ITEM, opts: Item_Options) -> c_int;
	pub fn item_opts_on(item: *mut ITEM, opts: Item_Options) -> c_int;
	pub fn item_opts_off(item: *mut ITEM, opts: Item_Options) -> c_int;
	pub fn item_opts(item: *const ITEM) -> Item_Options;

	pub fn set_item_init(menu: *mut MENU, func: Menu_Hook) -> c_int;
	pub fn item_init(menu: *const MENU) -> Menu_Hook;
	pub fn set_item_term(menu: *mut MENU, func: Menu_Hook) -> c_int;
	pub fn item_term(menu: *mut MENU, func: Menu_Hook) -> c_int;
	pub fn set_menu_init(menu: *mut MENU, func: Menu_Hook) -> c_int;
	pub fn menu_init(menu: *const MENU) -> Menu_Hook;
	pub fn set_menu_term(menu: *mut MENU, func: Menu_Hook) -> c_int;
	pub fn menu_term(menu: *const MENU) -> c_int;

	pub fn new_item(name: *const c_char, description: *const c_char) -> *mut ITEM;
	pub fn free_item(item: *mut ITEM) -> c_int;

	pub fn item_name(item: *const ITEM) -> *const c_char;
	pub fn item_description(item: *const ITEM) -> *const c_char;

	pub fn set_menu_mark(menu: *mut MENU, mark: *const c_char) -> c_int;
	pub fn menu_mark(menu: *const MENU) -> *const c_char;

	pub fn set_menu_pattern(menu: *mut MENU, pattern: *const c_char) -> c_int;
	pub fn menu_pattern(menu: *const MENU) -> *mut c_char;

	pub fn set_menu_fore(menu: *mut MENU, attr: chtype) -> c_int;
	pub fn menu_fore(menu: *const MENU) -> chtype;
	pub fn set_menu_back(menu: *mut MENU, attr: chtype) -> c_int;
	pub fn menu_back(menu: *const MENU) -> c_int;
	pub fn set_menu_grey(menu: *mut MENU, attr: chtype) -> c_int;
	pub fn menu_grey(menu: *const MENU) -> chtype;
	pub fn set_menu_pad(menu: *mut MENU, pad: c_int) -> c_int;
	pub fn menu_pad(menu: *const MENU) -> c_int;

	pub fn set_item_userptr(item: *mut ITEM, userptr: *mut c_void) -> c_int;
	pub fn item_userptr(item: *const ITEM) -> *mut c_void;

	pub fn set_item_value(item: *mut ITEM, value: bool) -> c_int;
	pub fn item_value(item: *const ITEM) -> bool;

	pub fn set_menu_win(menu: *mut MENU, win: *mut WINDOW) -> c_int;
	pub fn menu_win(menu: *const MENU) -> *mut WINDOW;
	pub fn set_menu_sub(menu: *mut MENU, sub: *mut WINDOW) -> c_int;
	pub fn menu_sub(menu: *const MENU) -> *mut WINDOW;
	pub fn scale_menu(menu: *const MENU, rows: *mut c_int, columns: *mut c_int) -> c_int;

	pub fn menu_request_name(request: c_int) -> *const c_char;
	pub fn menu_request_by_name(name: *const c_char) -> c_int;

	pub fn post_menu(menu: *mut MENU) -> c_int;
	pub fn unpost_menu(menu: *mut MENU) -> c_int;

	pub fn set_menu_spacing(menu: *mut MENU, spc_description: c_int, spc_rows: c_int, spc_columns: c_int) -> c_int;
	pub fn mneu_spacing(menu: *const MENU, spc_description: *mut c_int, spc_rows: *mut c_int, spc_columns: *mut c_int) -> c_int;

	pub fn item_visible(item: *const ITEM) -> bool;

	pub fn set_menu_format(menu: *mut MENU, rows: c_int, cols: c_int) -> c_int;
	pub fn menu_format(menu: *const MENU, rows: *mut c_int, cols: *mut c_int) -> c_int;
}
