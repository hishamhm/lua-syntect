extern crate libc;

use libc::c_int;
use libc::size_t;
use std::ffi::CStr;
use std::ffi::CString;
use std::os::raw::c_char;

use syntect::parsing::SyntaxSet;
use syntect::highlighting::{ThemeSet, Style};
use syntect::easy::HighlightFile;
use std::io::BufRead;

#[repr(C)] pub struct lua_State { _private: [u8; 0] }

extern "C" {
    pub fn lua_createtable(L: *mut lua_State, narr: c_int, nrec: c_int);
    pub fn lua_pushnil(L: *mut lua_State);
    pub fn lua_pushstring(L: *mut lua_State, str: *const c_char);
    pub fn lua_pushcclosure(L: *mut lua_State, f: extern "C" fn(*mut lua_State) -> c_int, nup: c_int);
    pub fn lua_settable(L: *mut lua_State, idx: c_int);
    pub fn lua_pushinteger(L: *mut lua_State, idx: c_int);
    pub fn lua_rawseti(L: *mut lua_State, tbl: c_int, i: c_int);
    pub fn luaL_checklstring(L: *mut lua_State, idx: c_int, len: *const size_t) -> *const c_char;
}

fn checkstring(l: *mut lua_State, idx: i32) -> String {
    unsafe {
        CStr::from_ptr(luaL_checklstring(l, idx, std::ptr::null())).to_string_lossy().into_owned()
    }
}

pub extern "C" fn l_highlight_file(l: *mut lua_State) -> c_int {
    let ss = SyntaxSet::load_defaults_newlines();
    let ts = ThemeSet::load_defaults();

    let theme = &ts.themes["base16-ocean.dark"];

    let filename = checkstring(l, 1);

    let mut i = 1;
    unsafe {
        lua_createtable(l, 0, 0);
    }

    match HighlightFile::new(filename, &ss, theme) {
        Ok(mut highlighter) => {
            let mut line = String::new();
            while highlighter.reader.read_line(&mut line).unwrap() > 0 {
                unsafe {
                    lua_createtable(l, 0, 0);
                }
                {
                    let regions: Vec<(Style, &str)> = highlighter.highlight_lines.highlight(&line, &ss);
                    let mut j = 1;
                    for (style, text) in regions {
                        unsafe {
                            let fg = style.foreground;
                            lua_pushinteger(l, (fg.r as i32) << 16 | (fg.g as i32) << 8 | fg.b as i32);
                            lua_rawseti(l, -2, j);
                            j += 1;
                            lua_pushstring(l, CString::new(text).unwrap().as_ptr());
                            lua_rawseti(l, -2, j);
                            j += 1;
                        }
                    }
                } // until NLL this scope is needed so we can clear the buffer after
                line.clear(); // read_line appends so we need to clear between lines
                unsafe {
                    lua_rawseti(l, -2, i);
                }
                i += 1;
            }
            return 1;
        }
        Err(e) => {
            unsafe {
                lua_pushnil(l);
                lua_pushstring(l, CString::new(e.to_string()).unwrap().as_ptr());
            }
            return 2;
        }
    }
}

#[no_mangle]
pub extern "C" fn luaopen_syntect(l: *mut lua_State) -> c_int {
    unsafe {
        lua_createtable(l, 0, 0);
        lua_pushstring(l, CString::new("highlight_file").unwrap().as_ptr());
        lua_pushcclosure(l, l_highlight_file, 0);
        lua_settable(l, -3);
    }
    return 1;
}
