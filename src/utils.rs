use crate::datatypes::Result;
use js_sys::Uint8Array;
use std::convert::TryFrom;
use wasm_bindgen::JsValue;

pub fn set_panic_hook() {
  // When the `console_error_panic_hook` feature is enabled, we can call the
  // `set_panic_hook` function at least once during initialization, and then
  // we will get better error messages if our code ever panics.
  //
  // For more details see
  // https://github.com/rustwasm/console_error_panic_hook#readme
  #[cfg(feature = "console_error_panic_hook")]
  console_error_panic_hook::set_once();
}

pub fn to_vec_u8(js: &JsValue) -> Vec<u8> {
  let typed_array = Uint8Array::new(js);
  let mut vec = vec![0; typed_array.length() as usize];
  typed_array.copy_to(&mut vec[..]);
  vec
}

pub fn to_i16(array: &[u8], offset: usize) -> Result<i16> {
  Ok(i16::from_le_bytes(<[u8; 2]>::try_from(
    &array[offset..offset + 2],
  )?))
}

pub fn to_u16(array: &[u8], offset: usize) -> Result<u16> {
  Ok(u16::from_le_bytes(<[u8; 2]>::try_from(
    &array[offset..offset + 2],
  )?))
}

pub fn to_u32(array: &[u8], offset: usize) -> Result<u32> {
  Ok(u32::from_le_bytes(<[u8; 4]>::try_from(
    &array[offset..offset + 4],
  )?))
}
