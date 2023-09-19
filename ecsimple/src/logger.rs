
use std::env;
use std::io::{Write};
use std::fs;
//use std::io::prelude::*;
use lazy_static::lazy_static;
use chrono::{Local,Timelike,Datelike};



fn _ecsimple_get_environ_var(envname :&str) -> String {
	match env::var(envname) {
		Ok(v) => {
			format!("{}",v)
		},
		Err(_e) => {
			String::from("")
		}
	}
}

#[allow(dead_code)]
struct LogVar {
	level :i32,
	nostderr : bool,
	wfile : Option<fs::File>,
}


fn ecsimple_proc_log_init(prefix :&str) -> LogVar {
	let mut getv :String;
	let mut retv :i32 = 0;
	let mut nostderr :bool = false;
	let mut coptfile :Option<fs::File> = None;
	let mut key :String;

	key = format!("{}_LEVEL", prefix);
	getv = _ecsimple_get_environ_var(&key);
	if getv.len() > 0 {
		match getv.parse::<i32>() {
			Ok(v) => {
				retv = v;
			},
			Err(e) => {
				retv = 0;
				eprintln!("can not parse [{}] error[{}]", getv,e);
			}
		}
	}

	key = format!("{}_NOSTDERR",prefix);
	getv = _ecsimple_get_environ_var(&key);
	if getv.len() > 0 {
		nostderr = true;
	}



	key = format!("{}_LOGFILE",prefix);
	getv = _ecsimple_get_environ_var(&key);
	if getv.len() > 0 {
		let fo = fs::File::create(&getv);
		if fo.is_err() {
			eprintln!("can not open [{}]", getv);
		} else {
			coptfile = Some(fo.unwrap());
		}
	}

	return LogVar {
		level : retv,
		nostderr : nostderr,
		wfile : coptfile,		
	};
}


lazy_static! {
	static ref ECSIMPLE_LOG_LEVEL : LogVar = {
		ecsimple_proc_log_init("ECSIMPLE")
	};
}


#[allow(dead_code)]
pub (crate)  fn ecsimple_debug_out(level :i32, outs :&str) {
	if ECSIMPLE_LOG_LEVEL.level >= level {
		let c = format!("{}\n",outs);
		if !ECSIMPLE_LOG_LEVEL.nostderr {
			let _ = std::io::stderr().write_all(c.as_bytes());
		}

		if ECSIMPLE_LOG_LEVEL.wfile.is_some() {
			let mut wf = ECSIMPLE_LOG_LEVEL.wfile.as_ref().unwrap();
			let _ = wf.write(c.as_bytes());
		}
	}
	return;
}

pub (crate) fn ecsimple_log_get_timestamp() -> String {
	let now = Local::now();
	return format!("{}/{}/{} {}:{}:{}",now.year(),now.month(),now.day(),now.hour(),now.minute(),now.second());
}

#[macro_export]
macro_rules! ecsimple_log_error {
	($($arg:tt)+) => {
		let mut c :String= format!("[ECSIMPLE]<ERROR>{}[{}:{}]  ",ecsimple_log_get_timestamp(),file!(),line!());
		c.push_str(&(format!($($arg)+)[..]));
		ecsimple_debug_out(0,&c);
	}
}

#[macro_export]
macro_rules! ecsimple_log_warn {
	($($arg:tt)+) => {
		let mut c :String= format!("[ECSIMPLE]<WARN>{}[{}:{}]  ",ecsimple_log_get_timestamp(),file!(),line!());
		c.push_str(&(format!($($arg)+)[..]));
		ecsimple_debug_out(10,&c);
	}
}


#[macro_export]
macro_rules! ecsimple_log_info {
	($($arg:tt)+) => {
		let mut c :String= format!("[ECSIMPLE]<INFO>{}[{}:{}]  ",ecsimple_log_get_timestamp(),file!(),line!());
		c.push_str(&(format!($($arg)+)[..]));
		ecsimple_debug_out(20,&c);
	}
}

#[cfg(feature="debug_mode")]
#[macro_export]
macro_rules! ecsimple_log_trace {
	($($arg:tt)+) => {
		let mut _c :String= format!("[ECSIMPLE]<TRACE>{}[{}:{}]  ",ecsimple_log_get_timestamp(),file!(),line!());
		_c.push_str(&(format!($($arg)+)[..]));
		ecsimple_debug_out(40, &_c);
	}
}

#[cfg(not(feature="debug_mode"))]
#[macro_export]
macro_rules! ecsimple_log_trace {
	($($arg:tt)+) => {}
}


#[macro_export]
macro_rules! ecsimple_assert {
	($v:expr , $($arg:tt)+) => {
		if !($v) {
			let mut _c :String= format!("[ECSIMPLE][{}:{}] ",file!(),line!());
			_c.push_str(&(format!($($arg)+)[..]));
			panic!("{}", _c);
		}
	}
}


#[macro_export]
macro_rules! ecsimple_format_buffer_log {
	($buf:expr,$len:expr,$info:tt,$iv:expr,$($arg:tt)+) => {
		let mut c :String = format!("[ECSIMPLE][{}:{}]",file!(),line!());
		c.push_str(&format!("{} ",$info));
		c.push_str(&ecsimple_log_get_timestamp());
		c.push_str(": ");
		c.push_str(&(format!($($arg)+)[..]));
		let _ptr :*const u8 = $buf as *const u8;
		let  mut _ci :usize;
		let _totallen: usize = $len as usize;
		let mut _lasti :usize = 0;
		let mut _nb :u8;
		c.push_str(&format!(" buffer [{:?}][{}]",_ptr,_totallen));
		_ci = 0;
		while _ci < _totallen {
			if (_ci % 16) == 0 {
				if _ci > 0 {
					c.push_str("    ");
					while _lasti < _ci {
						unsafe{
							_nb = *_ptr.offset(_lasti as isize);	
						}
						
						if _nb >= 0x20 && _nb <= 0x7e {
							c.push(_nb as char);
						} else {
							c.push_str(".");
						}
						_lasti += 1;
					}
				}
				c.push_str(&format!("\n0x{:08x}:", _ci));
			}
			unsafe {_nb = *_ptr.offset(_ci as isize);}			
			c.push_str(&format!(" 0x{:02x}",_nb));
			_ci += 1;
		}

		if _lasti < _ci {
			while (_ci % 16) != 0 {
				c.push_str("     ");
				_ci += 1;
			}

			c.push_str("    ");

			while _lasti < _totallen {
				unsafe {_nb = *_ptr.offset(_lasti as isize);}				
				if _nb >= 0x20 && _nb <= 0x7e {
					c.push(_nb as char);
				} else {
					c.push_str(".");
				}
				_lasti += 1;
			}
			//c.push_str("\n");
		}
		ecsimple_debug_out($iv,&c);
	}
}

#[macro_export]
macro_rules! ecsimple_debug_buffer_error {
	($buf:expr,$len:expr,$($arg:tt)+) => {
		ecsimple_format_buffer_log!($buf,$len,"<ERROR>",0,$($arg)+);
	}
}

#[macro_export]
macro_rules! ecsimple_debug_buffer_warn {
	($buf:expr,$len:expr,$($arg:tt)+) => {
		ecsimple_format_buffer_log!($buf,$len,"<WARN>",10,$($arg)+);
	}
}

#[macro_export]
macro_rules! ecsimple_debug_buffer_info {
	($buf:expr,$len:expr,$($arg:tt)+) => {
		ecsimple_format_buffer_log!($buf,$len,"<INFO>",20,$($arg)+);
	}
}

#[macro_export]
macro_rules! ecsimple_debug_buffer_debug {
	($buf:expr,$len:expr,$($arg:tt)+) => {
		ecsimple_format_buffer_log!($buf,$len,"<DEBUG>",30,$($arg)+);
	}
}

#[cfg(feature="debug_mode")]
#[macro_export]
macro_rules! ecsimple_debug_buffer_trace {
	($buf:expr,$len:expr,$($arg:tt)+) => {
		ecsimple_format_buffer_log!($buf,$len,"<TRACE>",40,$($arg)+);
	}
}

#[cfg(not(feature="debug_mode"))]
#[macro_export]
macro_rules! ecsimple_debug_buffer_trace {
	($buf:expr,$len:expr,$($arg:tt)+) => {}
}
