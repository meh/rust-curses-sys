use libc::{c_void, c_char, c_int, c_short, wchar_t, uint32_t, FILE};

pub type chtype  = uint32_t;
pub type attr_t  = chtype;

pub type SCREEN = c_void;
pub type WINDOW = c_void;

pub const CCHARW_MAX: usize = 5;

macro_rules! BITS {
	($mask:expr, $shift:expr) => (
		($mask as chtype) << ($shift + 8)
	)
}

pub const A_NORMAL:     chtype = 0;
pub const A_ATTRIBUTES: chtype = BITS!(!0, 0);
pub const A_CHARTEXT:   chtype = BITS!(1, 0) - 1;
pub const A_COLOR:      chtype = BITS!((1 << 8) - 1, 0);
pub const A_STANDOUT:   chtype = BITS!(1, 8);
pub const A_UNDERLINE:  chtype = BITS!(1, 9);
pub const A_REVERSE:    chtype = BITS!(1, 10);
pub const A_BLINK:      chtype = BITS!(1, 11);
pub const A_DIM:        chtype = BITS!(1, 12);
pub const A_BOLD:       chtype = BITS!(1, 13);
pub const A_ALTCHARSET: chtype = BITS!(1, 14);
pub const A_INVIS:      chtype = BITS!(1, 15);
pub const A_PROTECT:    chtype = BITS!(1, 16);
pub const A_HORIZONTAL: chtype = BITS!(1, 17);
pub const A_LEFT:       chtype = BITS!(1, 18);
pub const A_LOW:        chtype = BITS!(1, 19);
pub const A_RIGHT:      chtype = BITS!(1, 20);
pub const A_TOP:        chtype = BITS!(1, 21);
pub const A_VERTICAL:   chtype = BITS!(1, 22);
pub const A_ITALIC:     chtype = BITS!(1, 23);

pub const COLOR_BLACK:   c_short = 0;
pub const COLOR_RED:     c_short = 1;
pub const COLOR_GREEN:   c_short = 2;
pub const COLOR_YELLOW:  c_short = 3;
pub const COLOR_BLUE:    c_short = 4;
pub const COLOR_MAGENTA: c_short = 5;
pub const COLOR_CYAN:    c_short = 6;
pub const COLOR_WHITE:   c_short = 7;

pub const KEY_CODE_YES:  c_int = 0o400;
pub const KEY_MIN:       c_int = 0o401;
pub const KEY_BREAK:     c_int = 0o401;
pub const KEY_SRESET:    c_int = 0o530;
pub const KEY_RESET:     c_int = 0o531;
pub const KEY_DOWN:      c_int = 0o402;
pub const KEY_UP:        c_int = 0o403;
pub const KEY_LEFT:      c_int = 0o404;
pub const KEY_RIGHT:     c_int = 0o405;
pub const KEY_HOME:      c_int = 0o406;
pub const KEY_BACKSPACE: c_int = 0o407;
pub const KEY_F0:        c_int = 0o410;
pub const KEY_F1:        c_int = 0o411;
pub const KEY_F2:        c_int = 0o412;
pub const KEY_F3:        c_int = 0o413;
pub const KEY_F4:        c_int = 0o414;
pub const KEY_F5:        c_int = 0o415;
pub const KEY_F6:        c_int = 0o416;
pub const KEY_F7:        c_int = 0o417;
pub const KEY_F8:        c_int = 0o420;
pub const KEY_F9:        c_int = 0o421;
pub const KEY_F10:       c_int = 0o422;
pub const KEY_F11:       c_int = 0o423;
pub const KEY_F12:       c_int = 0o424;
pub const KEY_DL:        c_int = 0o510;
pub const KEY_IL:        c_int = 0o511;
pub const KEY_DC:        c_int = 0o512;
pub const KEY_IC:        c_int = 0o513;
pub const KEY_EIC:       c_int = 0o514;
pub const KEY_CLEAR:     c_int = 0o515;
pub const KEY_EOS:       c_int = 0o516;
pub const KEY_EOL:       c_int = 0o517;
pub const KEY_SF:        c_int = 0o520;
pub const KEY_SR:        c_int = 0o521;
pub const KEY_NPAGE:     c_int = 0o522;
pub const KEY_PPAGE:     c_int = 0o523;
pub const KEY_STAB:      c_int = 0o524;
pub const KEY_CTAB:      c_int = 0o525;
pub const KEY_CATAB:     c_int = 0o526;
pub const KEY_ENTER:     c_int = 0o527;
pub const KEY_PRINT:     c_int = 0o532;
pub const KEY_LL:        c_int = 0o533;
pub const KEY_A1:        c_int = 0o534;
pub const KEY_A3:        c_int = 0o535;
pub const KEY_B2:        c_int = 0o536;
pub const KEY_C1:        c_int = 0o537;
pub const KEY_C3:        c_int = 0o540;
pub const KEY_BTAB:      c_int = 0o541;
pub const KEY_BEG:       c_int = 0o542;
pub const KEY_CANCEL:    c_int = 0o543;
pub const KEY_CLOSE:     c_int = 0o544;
pub const KEY_COMMAND:   c_int = 0o545;
pub const KEY_COPY:      c_int = 0o546;
pub const KEY_CREATE:    c_int = 0o547;
pub const KEY_END:       c_int = 0o550;
pub const KEY_EXIT:      c_int = 0o551;
pub const KEY_FIND:      c_int = 0o552;
pub const KEY_HELP:      c_int = 0o553;
pub const KEY_MARK:      c_int = 0o554;
pub const KEY_MESSAGE:   c_int = 0o555;
pub const KEY_MOVE:      c_int = 0o556;
pub const KEY_NEXT:      c_int = 0o557;
pub const KEY_OPEN:      c_int = 0o560;
pub const KEY_OPTIONS:   c_int = 0o561;
pub const KEY_PREVIOUS:  c_int = 0o562;
pub const KEY_REDO:      c_int = 0o563;
pub const KEY_REFERENCE: c_int = 0o564;
pub const KEY_REFRESH:   c_int = 0o565;
pub const KEY_REPLACE:   c_int = 0o566;
pub const KEY_RESTART:   c_int = 0o567;
pub const KEY_RESUME:    c_int = 0o570;
pub const KEY_SAVE:      c_int = 0o571;
pub const KEY_SBEG:      c_int = 0o572;
pub const KEY_SCANCEL:   c_int = 0o573;
pub const KEY_SCOMMAND:  c_int = 0o574;
pub const KEY_SCOPY:     c_int = 0o575;
pub const KEY_SCREATE:   c_int = 0o576;
pub const KEY_SDC:       c_int = 0o577;
pub const KEY_SDL:       c_int = 0o600;
pub const KEY_SELECT:    c_int = 0o601;
pub const KEY_SEND:      c_int = 0o602;
pub const KEY_SEOL:      c_int = 0o603;
pub const KEY_SEXIT:     c_int = 0o604;
pub const KEY_SFIND:     c_int = 0o605;
pub const KEY_SHELP:     c_int = 0o606;
pub const KEY_SHOME:     c_int = 0o607;
pub const KEY_SIC:       c_int = 0o610;
pub const KEY_SLEFT:     c_int = 0o611;
pub const KEY_SMESSAGE:  c_int = 0o612;
pub const KEY_SMOVE:     c_int = 0o613;
pub const KEY_SNEXT:     c_int = 0o614;
pub const KEY_SOPTIONS:  c_int = 0o615;
pub const KEY_SPREVIOUS: c_int = 0o616;
pub const KEY_SPRINT:    c_int = 0o617;
pub const KEY_SREDO:     c_int = 0o620;
pub const KEY_SREPLACE:  c_int = 0o621;
pub const KEY_SRIGHT:    c_int = 0o622;
pub const KEY_SRSUME:    c_int = 0o623;
pub const KEY_SSAVE:     c_int = 0o624;
pub const KEY_SSUSPEND:  c_int = 0o625;
pub const KEY_SUNDO:     c_int = 0o626;
pub const KEY_SUSPEND:   c_int = 0o627;
pub const KEY_UNDO:      c_int = 0o630;
pub const KEY_MOUSE:     c_int = 0o631;
pub const KEY_RESIZE:    c_int = 0o632;
pub const KEY_EVENT:     c_int = 0o633;
pub const KEY_MAX:       c_int = 0o777;

#[derive(Copy, Clone, Eq, PartialEq, Debug)]
#[repr(C)]
pub struct cchar_t {
	pub attr:      attr_t,
	pub chars:     [wchar_t; CCHARW_MAX],
	pub ext_color: c_int,
}

#[inline(always)]
pub unsafe fn ACS(c: u8) -> chtype {
	*acs_map.offset(c as isize)
}

pub const ULCORNER: u8 = b'l';
pub const LLCORNER: u8 = b'm';
pub const URCORNER: u8 = b'k';
pub const LRCORNER: u8 = b'j';
pub const LTEE:     u8 = b't';
pub const RTEE:     u8 = b'u';
pub const BTEE:     u8 = b'v';
pub const TTEE:     u8 = b'w';
pub const HLINE:    u8 = b'q';
pub const VLINE:    u8 = b'x';
pub const PLUS:     u8 = b'n';
pub const S1:       u8 = b'o';
pub const S9:       u8 = b's';
pub const DIAMOND:  u8 = b'`';
pub const CKBOARD:  u8 = b'a';
pub const DEGREE:   u8 = b'f';
pub const PLMINUS:  u8 = b'g';
pub const BULLET:   u8 = b'~';
pub const LARROW:   u8 = b',';
pub const RARROW:   u8 = b'+';
pub const DARROW:   u8 = b'.';
pub const UARROW:   u8 = b'-';
pub const BOARD:    u8 = b'h';
pub const LANTERN:  u8 = b'i';
pub const BLOCK:    u8 = b'0';
pub const S3:       u8 = b'p';
pub const S7:       u8 = b'r';
pub const LEQUAL:   u8 = b'y';
pub const GEQUAL:   u8 = b'z';
pub const PI:       u8 = b'{';
pub const NEQUAL:   u8 = b'|';
pub const STERLING: u8 = b'}';

pub const BSSB:     u8 = ULCORNER;
pub const SSBB:     u8 = LLCORNER;
pub const BBSS:     u8 = URCORNER;
pub const SBBS:     u8 = LRCORNER;
pub const SBSS:     u8 = RTEE;
pub const SSSB:     u8 = LTEE;
pub const SSBS:     u8 = BTEE;
pub const BSSS:     u8 = TTEE;
pub const BSBS:     u8 = HLINE;
pub const SBSB:     u8 = VLINE;
pub const SSSS:     u8 = PLUS;

#[cfg_attr(feature = "wide", link(name = "ncursesw"))]
#[cfg_attr(not(feature = "wide"), link(name = "ncurses"))]
extern "C" {
	pub static curscr: *mut WINDOW;
	pub static newscr: *mut WINDOW;
	pub static stdscr: *mut WINDOW;

	pub static ttytype: *const c_char;
	pub static acs_map: *const chtype;

	pub static COLORS:      c_int;
	pub static COLOR_PAIRS: c_int;
	pub static COLS:        c_int;
	pub static ESCDELAY:    c_int;
	pub static LINES:       c_int;
	pub static TABSIZE:     c_int;

	pub fn initscr() -> *mut WINDOW;
	pub fn endwin() -> c_int;
	pub fn isendwin() -> bool;
	pub fn newterm(kind: *const c_char, outf: *mut FILE, infd: *mut FILE) -> *mut SCREEN;
	pub fn set_term(new: *mut SCREEN);
	pub fn delscreen(sp: *mut SCREEN);

	pub fn newwin(nlines: c_int, ncols: c_int, begin_y: c_int, begin_x: c_int) -> *mut WINDOW;
	pub fn delwin(win: *mut WINDOW) -> c_int;
	pub fn mvwin(win: *mut WINDOW, y: c_int, x: c_int) -> c_int;
	pub fn subwin(orig: *mut WINDOW, nlines: c_int, ncols: c_int, begin_y: c_int, begin_x: c_int) -> *mut WINDOW;
	pub fn derwin(orig: *mut WINDOW, nlines: c_int, ncols: c_int, begin_y: c_int, begin_x: c_int) -> *mut WINDOW;
	pub fn mvderwin(orig: *mut WINDOW, par_y: c_int, par_x: c_int) -> c_int;
	pub fn dupwin(win: *const WINDOW) -> *mut WINDOW;
	pub fn wsyncup(win: *mut WINDOW);
	pub fn syncok(win: *mut WINDOW, bf: bool) -> c_int;
	pub fn wcursyncup(win: *mut WINDOW);
	pub fn wsyncdown(win: *mut WINDOW);

	pub fn border(ls: chtype, rs: chtype, ts: chtype, bs: chtype, tl: chtype, tr: chtype, bl: chtype, br: chtype) -> c_int;
	pub fn wborder(win: *mut WINDOW, ls: chtype, rs: chtype, ts: chtype, bs: chtype, tl: chtype, tr: chtype, bl: chtype, br: chtype) -> c_int;
	#[link_name="box"] pub fn box_(win: *const WINDOW, verch: chtype, horch: chtype) -> c_int;

	pub fn hline(ch: chtype, n: c_int) -> c_int;
	pub fn whline(win: *mut WINDOW, ch: chtype, n: c_int) -> c_int;
	pub fn vline(ch: chtype, n: c_int) -> c_int;
	pub fn wvline(win: *mut WINDOW, ch: chtype, n: c_int) -> c_int;

	pub fn mvhline(y: c_int, x: c_int, ch: chtype, n: c_int) -> c_int;
	pub fn mvwhline(win: *mut WINDOW, y: c_int, x: c_int, ch: chtype, n: c_int) -> c_int;
	pub fn mvvline(y: c_int, x: c_int, ch: chtype, n: c_int) -> c_int;
	pub fn mvwvline(win: *mut WINDOW, y: c_int, x: c_int, ch: chtype, n: c_int) -> c_int;

	pub fn addch(ch: chtype) -> c_int;
	pub fn waddch(win: *mut WINDOW, ch: chtype) -> c_int;
	pub fn mvaddch(y: c_int, x: c_int, ch: chtype) -> c_int;
	pub fn mvwaddch(win: *mut WINDOW, y: c_int, x: c_int, ch: chtype) -> c_int;
	pub fn echochar(ch: chtype) -> c_int;
	pub fn wechochar(win: *mut WINDOW, ch: chtype) -> c_int;

	pub fn addchstr(chstr: *const chtype) -> c_int;
	pub fn addchnstr(chstr: *const chtype, n: c_int) -> c_int;
	pub fn waddchstr(win: *mut WINDOW, chstr: *const chtype) -> c_int;
	pub fn waddchnstr(win: *mut WINDOW, chstr: *const chtype, n: c_int) -> c_int;
	pub fn mvaddchstr(y: c_int, x: c_int, chstr: *const chtype) -> c_int;
	pub fn mvaddchnstr(y: c_int, x: c_int, chstr: *const chtype, n: c_int) -> c_int;
	pub fn mvwaddchstr(win: *mut WINDOW, y: c_int, x: c_int, chstr: *const chtype) -> c_int;
	pub fn mvwaddchnstr(win: *mut WINDOW, y: c_int, x: c_int, chstr: *const chtype, n: c_int) -> c_int;

	pub fn addstr(string: *const c_char) -> c_int;
	pub fn addnstr(string: *const c_char, n: c_int) -> c_int;
	pub fn waddstr(win: *mut WINDOW, string: *const c_char) -> c_int;
	pub fn waddnstr(win: *mut WINDOW, string: *const c_char, n: c_int) -> c_int;
	pub fn mvaddstr(y: c_int, x: c_int, string: *const c_char) -> c_int;
	pub fn mvaddnstr(y: c_int, x: c_int, string: *const c_char, n: c_int) -> c_int;
	pub fn mvwaddstr(win: *mut WINDOW, y: c_int, x: c_int, string: *const c_char) -> c_int;
	pub fn mvwaddnstr(win: *mut WINDOW, y: c_int, x: c_int, string: *const c_char, n: c_int) -> c_int;

	pub fn attroff(attrs: c_int) -> c_int;
	pub fn wattroff(win: *mut WINDOW, attrs: c_int) -> c_int;
	pub fn attron(attrs: c_int) -> c_int;
	pub fn wattron(win: *mut WINDOW, attrs: c_int) -> c_int;
	pub fn attrset(attrs: c_int) -> c_int;
	pub fn wattrset(win: *mut WINDOW, attrs: c_int) -> c_int;
	pub fn color_set(color_pair_number: c_short, opts: *const c_void) -> c_int;
	pub fn wcolor_set(win: *mut WINDOW, color_pair_number: c_short, opts: *const c_void) -> c_int;
	pub fn standend() -> c_int;
	pub fn wstandend(win: *mut WINDOW) -> c_int;
	pub fn standout() -> c_int;
	pub fn wstandout(win: *mut WINDOW) -> c_int;
	pub fn attr_get(attrs: *mut attr_t, pair: *mut c_short, opts: *const c_void) -> c_int;
	pub fn wattr_get(win: *const WINDOW, attrs: *mut attr_t, pair: *mut c_short, opts: *const c_void) -> c_int;
	pub fn attr_off(attrs: attr_t, opts: *const c_void) -> c_int;
	pub fn wattr_off(win: *mut WINDOW, attrs: attr_t, opts: *const c_void) -> c_int;
	pub fn attr_on(attrs: attr_t, opts: *const c_void) -> c_int;
	pub fn wattr_on(win: *mut WINDOW, attrs: attr_t, opts: *const c_void) -> c_int;
	pub fn attr_set(attrs: attr_t, pair: c_short, opts: *const c_void) -> c_int;
	pub fn wattr_set(win: *mut WINDOW, attrs: attr_t, pair: c_short, opts: *const c_void) -> c_int;
	pub fn chgat(n: c_int, attr: attr_t, color: c_short, opts: *const c_void) -> c_int;
	pub fn wchgat(win: *mut WINDOW, n: c_int, attr: attr_t, color: c_short, opts: *const c_void) -> c_int;
	pub fn mvchgat(y: c_int, x: c_int, n: c_int, attr: attr_t, color: c_short, opts: *const c_void) -> c_int;
	pub fn mvwchgat(win: *mut WINDOW, y: c_int, x: c_int, n: c_int, attr: attr_t, color: c_short, opts: *const c_void) -> c_int;

	pub fn beep() -> c_int;
	pub fn flash() -> c_int;

	pub fn baudrate() -> c_int;
	pub fn erasechar() -> c_char;
	pub fn erasewchar(ch: *mut wchar_t) -> c_int;
	pub fn has_ic() -> bool;
	pub fn has_il() -> bool;
	pub fn killchar() -> c_char;
	pub fn killwchar(ch: *mut wchar_t) -> c_int;
	pub fn longname() -> *const c_char;
	pub fn term_attrs() -> attr_t;
	pub fn termattrs() -> chtype;
	pub fn termname() -> *const c_char;

	pub fn erase() -> c_int;
	pub fn werase(win: *mut WINDOW) -> c_int;
	pub fn clear() -> c_int;
	pub fn wclear(win: *mut WINDOW) -> c_int;
	pub fn clrtobot() -> c_int;
	pub fn wclrtobot(win: *mut WINDOW) -> c_int;
	pub fn clrtoeol() -> c_int;
	pub fn wclrtoeol(win: *mut WINDOW) -> c_int;

	pub fn bkgdset(ch: chtype);
	pub fn wbkgdset(win: *mut WINDOW, ch: chtype);
	pub fn bkgd(ch: chtype) -> c_int;
	pub fn wbkgd(win: *mut WINDOW, ch: chtype) -> c_int;
	pub fn getbkgd(win: *const WINDOW) -> chtype;

	pub fn start_color() -> c_int;
	pub fn init_pair(pair: c_short, f: c_short, b: c_short) -> c_int;
	pub fn init_color(color: c_short, r: c_short, g: c_short, b: c_short) -> c_int;
	pub fn has_colors() -> bool;
	pub fn can_change_color() -> bool;
	pub fn color_content(color: c_short, r: *mut c_short, g: *mut c_short, b: *mut c_short) -> c_int;
	pub fn pair_content(pair: c_short, f: *mut c_short, b: *mut c_short) -> c_int;
	pub fn COLOR_PAIR(n: c_int) -> c_int;

	pub fn unctrl(c: chtype) -> *const c_char;
	pub fn wunctrl(c: *const cchar_t) -> *const wchar_t;
	pub fn keyname(c: c_int) -> *const c_char;
	pub fn key_name(w: wchar_t) -> *const c_char;
	pub fn filter();
	pub fn nofilter();
	pub fn use_env(f: bool);
	pub fn use_tioctl(f: bool);
	pub fn putwin(win: *mut WINDOW, filep: *mut FILE);
	pub fn getwin(filep: *mut FILE) -> *mut WINDOW;
	pub fn delay_output(ms: c_int) -> c_int;
	pub fn flushinp() -> c_int;

	pub fn cbreak() -> c_int;
	pub fn nocbreak() -> c_int;
	pub fn echo() -> c_int;
	pub fn noecho() -> c_int;
	pub fn halfdelay(tenths: c_int) -> c_int;
	pub fn intrflush(win: *mut WINDOW, bf: bool) -> c_int;
	pub fn keypad(win: *mut WINDOW, bf: bool) -> c_int;
	pub fn meta(win: *mut WINDOW, bf: bool) -> c_int;
	pub fn nodelay(win: *mut WINDOW, bf: bool) -> c_int;
	pub fn raw() -> c_int;
	pub fn noraw() -> c_int;
	pub fn noqiflush();
	pub fn qiflush();
	pub fn notimeout(win: *mut WINDOW, bf: bool) -> c_int;
	pub fn timeout(delay: c_int);
	pub fn wtimeout(win: *mut WINDOW, delay: c_int);
	pub fn typeahead(fd: c_int) -> c_int;

	pub fn inch() -> chtype;
	pub fn winch(win: *mut WINDOW) -> chtype;
	pub fn mvinch(y: c_int, x: c_int) -> chtype;
	pub fn mvwinch(win: *mut WINDOW, y: c_int, x: c_int) -> chtype;

	pub fn inchstr(chstr: *mut chtype) -> c_int;
	pub fn inchnstr(chstr: *mut chtype, n: c_int) -> c_int;
	pub fn winchstr(win: *mut WINDOW, chstr: *mut chtype) -> c_int;
	pub fn winchnstr(win: *mut WINDOW, chstr: *mut chtype, n: c_int) -> c_int;
	pub fn mvinchstr(y: c_int, x: c_int, chstr: *mut chtype) -> c_int;
	pub fn mvinchnstr(y: c_int, x: c_int, chstr: *mut chtype, n: c_int) -> c_int;
	pub fn mvwinchstr(win: *mut WINDOW, y: c_int, x: c_int, chstr: *mut chtype) -> c_int;
	pub fn mvwinchnstr(win: *mut WINDOW, y: c_int, x: c_int, chstr: *mut chtype, n: c_int) -> c_int;

	pub fn deleteln() -> c_int;
	pub fn wdeleteln(win: *mut WINDOW) -> c_int;
	pub fn insdelln(n: c_int) -> c_int;
	pub fn winsdelln(win: *mut WINDOW, n: c_int) -> c_int;
	pub fn insertln() -> c_int;
	pub fn winsertln(win: *mut WINDOW) -> c_int;

	pub fn clearok(win: *mut WINDOW, bf: bool) -> c_int;
	pub fn idlok(win: *mut WINDOW, bf: bool) -> c_int;
	pub fn idcok(win: *mut WINDOW, bf: bool);
	pub fn immedok(win: *mut WINDOW, bf: bool);
	pub fn leaveok(win: *mut WINDOW, bf: bool) -> c_int;
	pub fn setscrreg(top: c_int, bot: c_int) -> c_int;
	pub fn wsetscrreg(win: *mut WINDOW, top: c_int, bot: c_int) -> c_int;
	pub fn scrollok(win: *mut WINDOW, bf: bool) -> c_int;
	pub fn nl() -> c_int;
	pub fn nonl() -> c_int;

	#[link_name = "move"] pub fn move_(y: c_int, x: c_int) -> c_int;
	pub fn wmove(win: *mut WINDOW, y: c_int, x: c_int) -> c_int;

	pub fn getch() -> c_int;
	pub fn wgetch(win: *mut WINDOW) -> c_int;
	pub fn mvgetch(y: c_int, x: c_int) -> c_int;
	pub fn mvwgetch(win: *mut WINDOW, y: c_int, x: c_int) -> c_int;
	pub fn ungetch(ch: c_int) -> c_int;
	pub fn has_key(ch: c_int) -> c_int;

	pub fn overlay(srcwin: *const WINDOW, dstwin: *mut WINDOW) -> c_int;
	pub fn overwrite(srcwin: *const WINDOW, dstwin: *mut WINDOW) -> c_int;
	pub fn copywin(srcwin: *const WINDOW, dstwin: *mut WINDOW, sminrow: c_int, smincol: c_int, dminrow: c_int, dmincol: c_int, dmaxrow: c_int, dmaxcol: c_int, overlay: c_int);

	pub fn refresh() -> c_int;
	pub fn wrefresh(win: *mut WINDOW) -> c_int;
	pub fn wnoutrefresh(win: *mut WINDOW) -> c_int;
	pub fn doupdate() -> c_int;
	pub fn redrawwin(win: *mut WINDOW) -> c_int;
	pub fn wredrawln(win: *mut WINDOW, beg_line: c_int, num_lines: c_int) -> c_int;

	pub fn def_prog_mode() -> c_int;
	pub fn def_shell_mode() -> c_int;
	pub fn reset_prog_mode() -> c_int;
	pub fn reset_shell_mode() -> c_int;
	pub fn resetty() -> c_int;
	pub fn savetty() -> c_int;
	pub fn getsyx(y: c_int, x: c_int);
	pub fn setsyx(y: c_int, x: c_int);
	pub fn ripoffline(line: c_int, init: extern "C" fn(*mut WINDOW, c_int) -> c_int) -> c_int;
	pub fn curs_set(visibility: c_int) -> c_int;
	pub fn napms(ms: c_int) -> c_int;

	pub fn scr_dump(filename: *const c_char) -> c_int;
	pub fn scr_restore(filename: *const c_char) -> c_int;
	pub fn scr_init(filename: *const c_char) -> c_int;
	pub fn scr_set(filename: *const c_char) -> c_int;

	pub fn scroll(win: *mut WINDOW) -> c_int;
	pub fn scrl(n: c_int) -> c_int;
	pub fn wscrl(win: *mut WINDOW, n: c_int) -> c_int;

	pub fn newpad(nlines: c_int, ncols: c_int) -> *mut WINDOW;
	pub fn subpad(orig: *mut WINDOW, nlines: c_int, ncols: c_int, begin_y: c_int, begin_x: c_int) -> *mut WINDOW;
	pub fn prefresh(pad: *mut WINDOW, pminrow: c_int, pmincol: c_int, sminrow: c_int, smincol: c_int, smaxrow: c_int, smaxcol: c_int) -> c_int;
	pub fn pnoutrefresh(pad: *mut WINDOW, pminrow: c_int, pmincol: c_int, sminrow: c_int, smincol: c_int, smaxrow: c_int, smaxcol: c_int) -> c_int;
	pub fn pechochar(pad: *mut WINDOW, ch: chtype) -> c_int;
	pub fn pecho_wchar(pad: *mut WINDOW, wch: *const cchar_t) -> c_int;

	pub fn slk_init(fmt: c_int) -> c_int;
	pub fn slk_set(labnum: c_int, label: *const c_char, fmt: c_int) -> c_int;
	pub fn slk_refresh() -> c_int;
	pub fn slk_noutrefresh() -> c_int;
	pub fn slk_label(labnum: c_int) -> *const c_char;
	pub fn slk_clear() -> c_int;
	pub fn slk_restore() -> c_int;
	pub fn slk_touch() -> c_int;
	pub fn slk_attron(attrs: chtype) -> c_int;
	pub fn slk_attroff(attrs: chtype) -> c_int;
	pub fn slk_attrset(attrs: chtype) -> c_int;
	pub fn slk_attr_on(attrs: attr_t, opts: *const c_void) -> c_int;
	pub fn slk_attr_off(attrs: attr_t, opts: *const c_void) -> c_int;
	pub fn slk_attr_set(attrs: attr_t, color_pair: c_short, opts: *const c_void) -> c_int;
	pub fn slk_attr() -> attr_t;
	pub fn slk_color(color_pair: c_short) -> c_int;
	pub fn slk_wset(labnum: c_int, label: *const wchar_t, fmt: c_int) -> c_int;

	pub fn instr(string: *mut c_char) -> c_int;
	pub fn innstr(string: *mut c_char, n: c_int) -> c_int;
	pub fn winstr(win: *mut WINDOW, string: *mut c_char) -> c_int;
	pub fn winnstr(win: *mut WINDOW, string: *mut c_char, n: c_int) -> c_int;
	pub fn mvinstr(y: c_int, x: c_int, string: *mut c_char) -> c_int;
	pub fn mvinnstr(y: c_int, x: c_int, string: *mut c_char, n: c_int) -> c_int;
	pub fn mvwinstr(win: *mut WINDOW, y: c_int, x: c_int, string: *mut c_char) -> c_int;
	pub fn mvwinnstr(win: *mut WINDOW, y: c_int, x: c_int, string: *mut c_char, n: c_int) -> c_int;

	pub fn touchwin(win: *mut WINDOW) -> c_int;
	pub fn touchline(win: *mut WINDOW, start: c_int, count: c_int) -> c_int;
	pub fn untouchwin(win: *mut WINDOW) -> c_int;
	pub fn wtouchln(win: *mut WINDOW, y: c_int, n: c_int, changed: c_int) -> c_int;
	pub fn is_linetouched(win: *const WINDOW, line: c_int) -> bool;
	pub fn is_wintouched(win: *const WINDOW) -> bool;
}
