#![deny(clippy::all)]

#[macro_use]
extern crate napi_derive;

#[napi]
pub fn pid_path(pid: i32) -> Result<String, napi::Error> {
  return match darwin_libproc::pid_path(pid) {
    Ok(path) => match path.into_os_string().into_string() {
      Ok(path_string) => Result::Ok(path_string),
      Err(err) => Result::Err(napi::Error::from_reason(["Cannot represent as UTF-8 string: ", err.to_string_lossy().to_string().as_str()].join(""))),
    },
    Err(err) => Result::Err(napi::Error::from(err)),
  };
}
