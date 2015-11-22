use libc::c_int;

pub const E_OK:              c_int = 0;
pub const E_SYSTEM_ERROR:    c_int = -1;
pub const E_BAD_ARGUMENT:    c_int = -2;
pub const E_POSTED:          c_int = -3;
pub const E_CONNECTED:       c_int = -4;
pub const E_BAD_STATE:       c_int = -5;
pub const E_NO_ROOM:         c_int = -6;
pub const E_NOT_POSTED:      c_int = -7;
pub const E_UNKNOWN_COMMAND: c_int = -8;
pub const E_NO_MATCH:        c_int = -9;
pub const E_NOT_SELECTABLE:  c_int = -10;
pub const E_NOT_CONNECTED:   c_int = -11;
pub const E_REQUEST_DENIED:  c_int = -12;
pub const E_INVALID_FIELD:   c_int = -13;
pub const E_CURRENT:         c_int = -14;
