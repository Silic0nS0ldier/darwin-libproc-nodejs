#![deny(clippy::all)]

#[macro_use]
extern crate napi_derive;

use crate::osx_libproc_bindings::{
  proc_libversion, proc_name, proc_pidinfo, proc_pidpath, proc_regionfilename,
  PROC_PIDPATHINFO_MAXSIZE,
};

#[napi]
pub fn sum(a: i32, b: i32) -> i32 {
  a + b
}

#[napi]
pub fn pidpath(pid: i32) -> Result<String, String> {
  let mut buf: Vec<u8> = Vec::with_capacity((PROC_PIDPATHINFO_MAXSIZE - 1) as _);
  let buffer_ptr = buf.as_mut_ptr() as *mut c_void;
  let buffer_size = buf.capacity() as u32;
  let ret: i32;

  unsafe {
      ret = proc_pidpath(pid, buffer_ptr, buffer_size as _);
  };

  helpers::check_errno(ret, &mut buf)
}
