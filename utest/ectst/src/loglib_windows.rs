use winapi::um::{debugapi};
use super::wchar_windows;

#[allow(dead_code)]
pub fn win_output_debug(s :&str) {
	let wso : Option<Box<[u16]>> =  wchar_windows::str_to_c_wstr(s);
	if wso.is_none() {
		return;
	}
	let ws = wso.unwrap();

    unsafe {
        debugapi::OutputDebugStringW(ws.as_ptr());
    }
    return;
}