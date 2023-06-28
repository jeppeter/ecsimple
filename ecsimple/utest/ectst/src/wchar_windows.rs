use std::{convert::TryInto, ptr::{null_mut,null}};
use winapi::um::{stringapiset::{MultiByteToWideChar,WideCharToMultiByte}, winnls::CP_UTF8};

/// Convert a given `str` into a null-terminated wide character string. Does not
/// check for unexpected null characters.
///
/// Returns `None` if the input string is too long or anything goes wrong.
#[allow(dead_code)]
pub fn str_to_c_wstr(s: &str) -> Option<Box<[u16]>> {
    if s.len() == 0 {
        Some(Box::new([0]))
    } else {
        unsafe {
            let in_len = s.len().try_into().ok()?;
            let num_wchars =
                MultiByteToWideChar(CP_UTF8, 0, s.as_ptr() as *const i8, in_len, null_mut(), 0);
            if num_wchars == 0 {
                return None;
            }

            let len: usize = num_wchars.try_into().ok()?;
            let len = len.checked_add(1)?; // null termination

            let mut out = Vec::<u16>::with_capacity(len);
            let out_num_wchars = MultiByteToWideChar(
                CP_UTF8,
                0,
                s.as_ptr() as *const i8,
                in_len,
                out.as_mut_ptr(),
                num_wchars,
            );
            if out_num_wchars != num_wchars {
                return None;
            }
            out.as_mut_ptr().offset(len as isize - 1).write(0); // null termination
            out.set_len(len);

            Some(out.into())
        }
    }
}

#[allow(dead_code)]
pub fn wstr_to_str(s: &[u16]) -> Option<Box<[u8]>> {
    if s.len() == 0 {
        Some(Box::new([0]))
    } else {
        unsafe {
            let in_len = s.len().try_into().ok()?;
            let num_chars =
                WideCharToMultiByte(CP_UTF8, 0, s.as_ptr() as *const u16, in_len, null_mut(), 0,null(),null_mut());
            if num_chars == 0 {
                return None;
            }

            let len: usize = num_chars.try_into().ok()?;
            let len = len.checked_add(1)?; // null termination

            let mut out = Vec::<u8>::with_capacity(len);
            let out_num_chars = WideCharToMultiByte(
                CP_UTF8,
                0,
                s.as_ptr() as *const u16,
                in_len,
                out.as_mut_ptr() as *mut i8,
                num_chars,
                null(),null_mut());
            if out_num_chars != num_chars {
                return None;
            }
            out.as_mut_ptr().offset(len as isize - 1).write(0); // null termination
            out.set_len(len-1);

            Some(out.into())
        }
    }
}
