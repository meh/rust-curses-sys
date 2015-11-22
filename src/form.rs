use libc::{c_void, c_char, c_short, c_ushort, c_int, c_uint, c_long, wchar_t};
use base::{WINDOW, chtype, KEY_MAX};

pub type Form_Options  = c_int;
pub type Field_Options = c_int;

pub type FIELD_CELL = c_void;

#[derive(Debug)]
#[repr(C)]
pub struct PAGE {
	pmin: c_short,
	pmax: c_short,
	smin: c_short,
	smax: c_short,
}

#[derive(Debug)]
#[repr(C)]
pub struct FIELD {
	pub status:  c_ushort,
	pub rows:    c_short,
	pub cols:    c_short,
	pub frow:    c_short,
	pub fcol:    c_short,
	pub drows:   c_int,
	pub dcols:   c_int,
	pub maxgrow: c_int,
	pub nrow:    c_int,
	pub nbuf:    c_short,
	pub just:    c_short,
	pub page:    c_short,
	pub index:   c_short,
	pub pad:     c_int,
	pub fore:    chtype,
	pub back:    chtype,
	pub opts:    Field_Options,
	pub snext:   *mut FIELD,
	pub sprev:   *mut FIELD,
	pub link:    *mut FIELD,
	pub form:    *mut FORM,
	pub type_:   *mut FIELDTYPE,
	pub arg:     *mut c_void,
	pub buf:     *mut FIELD_CELL,
	pub usrptr:  *mut c_void,
}

#[repr(C)]
pub struct FORM {
	pub status:   c_ushort,
	pub rows:     c_short,
	pub cols:     c_short,
	pub currow:   c_int,
	pub curcol:   c_int,
	pub toprow:   c_int,
	pub begincol: c_int,
	pub maxfield: c_short,
	pub maxpage:  c_short,
	pub curpage:  c_short,
	pub opts:     Form_Options,
	pub win:      *mut WINDOW,
	pub sub:      *mut WINDOW,
	pub w:        *mut WINDOW,
	pub field:    *mut *mut FIELD,
	pub current:  *mut FIELD,
	pub page:     *mut PAGE,
	pub usrptr:   *mut c_void,

	pub forminit:  Form_Hook,
	pub formterm:  Form_Hook,
	pub fieldinit: Form_Hook,
	pub fieldterm: Form_Hook,
}

#[repr(C)]
pub struct FIELDTYPE {
	pub status: c_ushort,
	pub ref_:   c_long,
	pub left:   *mut FIELDTYPE,
	pub right:  *mut FIELDTYPE,

	pub makearg: extern "C" fn(*mut c_void) -> *mut c_void,
	pub copyarg: extern "C" fn(*const c_void) -> *mut c_void,
	pub freearg: extern "C" fn(*mut c_void),

	pub fcheck: extern "C" fn(*mut FIELD, *const c_void) -> bool,
	pub ccheck: extern "C" fn(c_int, *const c_void) -> bool,

	pub next: extern "C" fn(*mut FIELD, *const c_void) -> bool,
	pub prev: extern "C" fn(*mut FIELD, *const c_void) -> bool,
}

pub type Form_Hook = extern "C" fn(*mut FORM);

pub const NO_JUSTIFICATION: c_int = 0;
pub const JUSTIFY_LEFT:     c_int = 1;
pub const JUSTIFY_CENTER:   c_int = 2;
pub const JUSTIFY_RIGHT:    c_int = 3;

pub const O_VISIBLE:         c_uint = 0x0001;
pub const O_ACTIVE:          c_uint = 0x0002;
pub const O_PUBLIC:          c_uint = 0x0004;
pub const O_EDIT:            c_uint = 0x0008;
pub const O_WRAP:            c_uint = 0x0010;
pub const O_BLANK:           c_uint = 0x0020;
pub const O_AUTOSKIP:        c_uint = 0x0040;
pub const O_NULLOK:          c_uint = 0x0080;
pub const O_PASSOK:          c_uint = 0x0100;
pub const O_STATIC:          c_uint = 0x0200;
pub const O_DYNAMIC_JUSTIFY: c_uint = 0x0400;
pub const O_NL_OVERLOAD:     c_uint = 0x0001;
pub const O_BS_OVERLOAD:     c_uint = 0x0002;

pub const REQ_NEXT_PAGE:    c_int = KEY_MAX + 1;
pub const REQ_PREV_PAGE:    c_int = KEY_MAX + 2;
pub const REQ_FIRST_PAGE:   c_int = KEY_MAX + 3;
pub const REQ_LAST_PAGE:    c_int = KEY_MAX + 4;
pub const REQ_NEXT_FIELD:   c_int = KEY_MAX + 5;
pub const REQ_PREV_FIELD:   c_int = KEY_MAX + 6;
pub const REQ_FIRST_FIELD:  c_int = KEY_MAX + 7;
pub const REQ_LAST_FIELD:   c_int = KEY_MAX + 8;
pub const REQ_SNEXT_FIELD:  c_int = KEY_MAX + 9;
pub const REQ_SPREV_FIELD:  c_int = KEY_MAX + 10;
pub const REQ_SFIRST_FIELD: c_int = KEY_MAX + 11;
pub const REQ_SLAST_FIELD:  c_int = KEY_MAX + 12;
pub const REQ_LEFT_FIELD:   c_int = KEY_MAX + 13;
pub const REQ_RIGHT_FIELD:  c_int = KEY_MAX + 14;
pub const REQ_UP_FIELD:     c_int = KEY_MAX + 15;
pub const REQ_DOWN_FIELD:   c_int = KEY_MAX + 16;
pub const REQ_NEXT_CHAR:    c_int = KEY_MAX + 17;
pub const REQ_PREV_CHAR:    c_int = KEY_MAX + 18;
pub const REQ_NEXT_LINE:    c_int = KEY_MAX + 19;
pub const REQ_PREV_LINE:    c_int = KEY_MAX + 20;
pub const REQ_NEXT_WORD:    c_int = KEY_MAX + 21;
pub const REQ_PREV_WORD:    c_int = KEY_MAX + 22;
pub const REQ_BEG_FIELD:    c_int = KEY_MAX + 23;
pub const REQ_END_FIELD:    c_int = KEY_MAX + 24;
pub const REQ_BEG_LINE:     c_int = KEY_MAX + 25;
pub const REQ_END_LINE:     c_int = KEY_MAX + 26;
pub const REQ_LEFT_CHAR:    c_int = KEY_MAX + 27;
pub const REQ_RIGHT_CHAR:   c_int = KEY_MAX + 28;
pub const REQ_UP_CHAR:      c_int = KEY_MAX + 29;
pub const REQ_DOWN_CHAR:    c_int = KEY_MAX + 30;
pub const REQ_NEW_LINE:     c_int = KEY_MAX + 31;
pub const REQ_INS_CHAR:     c_int = KEY_MAX + 32;
pub const REQ_INS_LINE:     c_int = KEY_MAX + 33;
pub const REQ_DEL_CHAR:     c_int = KEY_MAX + 34;
pub const REQ_DEL_PREV:     c_int = KEY_MAX + 35;
pub const REQ_DEL_LINE:     c_int = KEY_MAX + 36;
pub const REQ_DEL_WORD:     c_int = KEY_MAX + 37;
pub const REQ_CLR_EOL:      c_int = KEY_MAX + 38;
pub const REQ_CLR_EOF:      c_int = KEY_MAX + 39;
pub const REQ_CLR_FIELD:    c_int = KEY_MAX + 40;
pub const REQ_OVL_MODE:     c_int = KEY_MAX + 41;
pub const REQ_INS_MODE:     c_int = KEY_MAX + 42;
pub const REQ_SCR_FLINE:    c_int = KEY_MAX + 43;
pub const REQ_SCR_BLINE:    c_int = KEY_MAX + 44;
pub const REQ_SCR_FPAGE:    c_int = KEY_MAX + 45;
pub const REQ_SCR_BPAGE:    c_int = KEY_MAX + 46;
pub const REQ_SCR_FHPAGE:   c_int = KEY_MAX + 47;
pub const REQ_SCR_BHPAGE:   c_int = KEY_MAX + 48;
pub const REQ_SCR_FCHAR:    c_int = KEY_MAX + 49;
pub const REQ_SCR_BCHAR:    c_int = KEY_MAX + 50;
pub const REQ_SCR_HFLINE:   c_int = KEY_MAX + 51;
pub const REQ_SCR_HBLINE:   c_int = KEY_MAX + 52;
pub const REQ_SCR_HFHALF:   c_int = KEY_MAX + 53;
pub const REQ_SCR_HBHALF:   c_int = KEY_MAX + 54;
pub const REQ_VALIDATION:   c_int = KEY_MAX + 55;
pub const REQ_NEXT_CHOICE:  c_int = KEY_MAX + 56;
pub const REQ_PREV_CHOICE:  c_int = KEY_MAX + 57;

#[cfg_attr(feature = "wide", link(name = "formw"))]
#[cfg_attr(not(feature = "wide"), link(name = "form"))]
extern "C" {
	pub static TYPE_ALPHA:   *mut FIELDTYPE;
	pub static TYPE_ALNUM:   *mut FIELDTYPE;
	pub static TYPE_ENUM:    *mut FIELDTYPE;
	pub static TYPE_INTEGER: *mut FIELDTYPE;
	pub static TYPE_NUMERIC: *mut FIELDTYPE;
	pub static TYPE_REGEXP:  *mut FIELDTYPE;

	#[cfg(feature = "extensions")]
	pub static TYPE_IPV4: *mut FIELDTYPE;

	pub fn new_fieldtype(field_check: extern "C" fn(*mut FIELD, *const c_void) -> bool,
	                     char_check: extern "C" fn(c_int, *const c_void) -> bool) -> *mut FIELDTYPE;
	pub fn free_fieldtype(fieldtype: *mut FIELDTYPE);
	pub fn set_fieldtype_arg(fieldtype: *mut FIELDTYPE,
	                         make_arg: extern "C" fn(*mut c_void) -> *mut c_void,
	                         copy_arg: extern "C" fn(*const c_void) -> *mut c_void,
	                         free_arg: extern "C" fn(*mut c_void)) -> c_int;
	pub fn set_fieldtype_choice(fieldtype: *mut FIELDTYPE,
	                            next_choice: extern "C" fn(*mut FIELD, *const c_void) -> bool,
	                            prev_choice: extern "C" fn(*mut FIELD, *const c_void) -> bool) -> c_int;
	pub fn link_fieldtype(type1: *mut FIELDTYPE, type2: *mut FIELDTYPE) -> *mut FIELDTYPE;

	pub fn new_field(height: c_int, width: c_int, toprow: c_int, leftcol: c_int, offscreen: c_int, nbuffers: c_int) -> *mut FIELD;
	pub fn dup_field(field: *mut FIELD, toprow: c_int, leftcol: c_int) -> *mut FIELD;
	pub fn link_field(field: *mut FIELD, toprow: c_int, leftcol: c_int) -> *mut FIELD;
	pub fn free_field(field: *mut FIELD) -> c_int;

	pub fn field_info(field: *const FIELD, rows: *mut c_int, cols: *mut c_int, frow: *mut c_int, fcol: *mut c_int, nrow: *mut c_int, nbuf: *mut c_int) -> c_int;
	pub fn dynamic_field_info(field: *const FIELD, rows: *mut c_int, cols: *mut c_int, max: *mut c_int) -> c_int;

	pub fn set_field_buffer(field: *mut FIELD, buf: c_int, value: *const c_char) -> c_int;
	pub fn field_buffer(field: *const FIELD, buffer: c_int) -> *mut c_char;
	pub fn set_field_status(field: *mut FIELD, status: bool) -> c_int;
	pub fn field_status(field: *const FIELD) -> bool;
	pub fn set_max_field(field: *mut FIELD, max: c_int) -> c_int;

	pub fn set_new_page(field: *mut FIELD, new_page_flag: bool) -> c_int;
	pub fn new_page(field: *const FIELD) -> bool;

	pub fn set_field_just(field: *mut FIELD, justification: c_int) -> c_int;
	pub fn field_just(field: *const FIELD) -> c_int;

	pub fn set_field_fore(field: *mut FIELD, attr: chtype) -> c_int;
	pub fn field_fore(field: *const FIELD) -> chtype;
	pub fn set_field_back(field: *mut FIELD, attr: chtype) -> c_int;
	pub fn field_back(field: *const FIELD) -> chtype;
	pub fn set_field_pad(field: *mut FIELD, pad: c_int) -> c_int;
	pub fn field_pad(field: *const FIELD) -> c_int;

	pub fn set_field_opts(field: *mut FIELD, opts: Field_Options) -> c_int;
	pub fn field_opts_on(field: *mut FIELD, opts: Field_Options) -> c_int;
	pub fn field_opts_off(field: *mut FIELD, opts: Field_Options) -> c_int;
	pub fn field_opts(field: *const FIELD) -> Field_Options;

	pub fn set_field_type(field: *mut FIELD, type_: *mut FIELDTYPE, ...) -> c_int;
	pub fn field_type(field: *const FIELD) -> *mut FIELDTYPE;
	pub fn field_arg(field: *const FIELD) -> *mut c_void;

	pub fn set_field_userptr(field: *mut FIELD, userptr: *mut c_void) -> c_int;
	pub fn field_userptr(field: *const FIELD) -> *mut c_void;

	pub fn new_form(fields: *mut *mut FIELD) -> *mut FORM;
	pub fn free_form(form: *mut FORM) -> c_int;

	pub fn set_form_fields(form: *mut FORM, fields: *mut *mut FIELD) -> c_int;
	pub fn form_fields(form: *const FORM) -> *mut *mut FIELD;
	pub fn field_count(form: *const FORM) -> c_int;
	pub fn move_field(field: *mut FIELD, frow: c_int, fcol: c_int) -> c_int;

	pub fn set_form_win(form: *mut FORM, win: *mut WINDOW) -> c_int;
	pub fn form_win(form: *const FORM) -> *mut WINDOW;
	pub fn set_form_sub(form: *mut FORM, win: *mut WINDOW) -> c_int;
	pub fn form_sub(form: *const FORM) -> *mut WINDOW;
	pub fn scale_form(form: *const FORM, rows: *mut c_int, columns: *mut c_int) -> c_int;

	pub fn set_field_init(form: *mut FORM, func: Form_Hook) -> c_int;
	pub fn field_init(form: *const FORM) -> Form_Hook;
	pub fn set_field_term(form: *mut FORM, func: Form_Hook) -> c_int;
	pub fn field_term(form: *const FORM) -> Form_Hook;
	pub fn set_form_init(form: *mut FORM, func: Form_Hook) -> c_int;
	pub fn form_init(form: *const FORM) -> Form_Hook;
	pub fn set_form_term(form: *mut FORM, func: Form_Hook) -> c_int;
	pub fn form_term(form: *const FORM) -> Form_Hook;

	pub fn set_current_field(form: *mut FORM, field: *mut FIELD) -> c_int;
	pub fn current_field(form: *const FORM) -> *mut FIELD;
	pub fn set_form_page(form: *mut FORM, n: c_int) -> c_int;
	pub fn form_page(form: *const FORM) -> c_int;
	pub fn form_index(field: *const FIELD) -> c_int;

	pub fn post_form(form: *mut FORM) -> c_int;
	pub fn unpost_form(form: *mut FORM) -> c_int;

	pub fn pos_form_cursor(form: *mut FORM) -> c_int;

	pub fn set_form_userptr(form: *mut FORM, userptr: *mut c_void) -> c_int;
	pub fn form_userptr(form: *const FORM) -> *mut c_void;

	pub fn set_form_opts(form: *mut FORM, opts: Field_Options) -> c_int;
	pub fn form_opts_on(form: *mut FORM, opts: Field_Options) -> c_int;
	pub fn form_opts_off(form: *mut FORM, opts: Field_Options) -> c_int;
	pub fn form_opts(form: *const FORM) -> Field_Options;

	pub fn form_driver(form: *mut FORM, c: c_int) -> c_int;
	#[cfg(feature = "wide")]
	pub fn form_driver_w(form: *mut FORM, c: c_int, wch: wchar_t) -> c_int;

	pub fn form_request_name(request: c_int) -> *const c_char;
	pub fn form_request_by_name(name: *const c_char) -> c_int;

	pub fn data_ahead(form: *const FORM) -> bool;
	pub fn data_behind(form: *const FORM) -> bool;
}
