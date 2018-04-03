// https://github.com/dwrensha/seer/blob/master/example/standalone/base64.rs
//
// Copyright (c) 2017 The Seer Developers
// Copyright (c) 2016 The Miri Developers
//
// Permission is hereby granted, free of charge, to any
// person obtaining a copy of this software and associated
// documentation files (the "Software"), to deal in the
// Software without restriction, including without
// limitation the rights to use, copy, modify, merge,
// publish, distribute, sublicense, and/or sell copies of
// the Software, and to permit persons to whom the Software
// is furnished to do so, subject to the following
// conditions:
//
// The above copyright notice and this permission notice
// shall be included in all copies or substantial portions
// of the Software.
//
// THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF
// ANY KIND, EXPRESS OR IMPLIED, INCLUDING BUT NOT LIMITED
// TO THE WARRANTIES OF MERCHANTABILITY, FITNESS FOR A
// PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT
// SHALL THE AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY
// CLAIM, DAMAGES OR OTHER LIABILITY, WHETHER IN AN ACTION
// OF CONTRACT, TORT OR OTHERWISE, ARISING FROM, OUT OF OR
// IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER
// DEALINGS IN THE SOFTWARE.

#![crate_type = "staticlib"]

use std::ffi::CStr;
use std::os::raw::c_char;

#[no_mangle]
pub extern "C" fn base64(c_str: *const c_char) -> i32 {
  let slice = unsafe { CStr::from_ptr(c_str) };
  let data = slice.to_bytes();
  // $ echo -n h1baby | base64
  // aDFiYWJ5
  let value_to_decode = "aDFiYWJ5";

  let result = base64_encode(data);
  if result.starts_with(value_to_decode) {
    1
  } else {
    0
  }
}

// copied from `rustc_serialize`.
fn base64_encode(input: &[u8]) -> String {
  static BYTES: &[u8] =
    b"ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789+/";
  let mut result = String::new();
  {
    let len = input.len();
    let output = unsafe { result.as_mut_vec() };
    *output = vec![b'='; (len + 2) / 3 * 4];
    let mod_len = len % 3;
    {
      let mut s_in = input[..len - mod_len].iter().map(|&x| x as u32);
      let mut s_out = output.iter_mut();
      let enc = |val| BYTES[val as usize];
      let mut write = |val| *s_out.next().unwrap() = val;

      while let (Some(first), Some(second), Some(third)) =
        (s_in.next(), s_in.next(), s_in.next())
      {
        let n = first << 16 | second << 8 | third;
        write(enc((n >> 18) & 63));
        write(enc((n >> 12) & 63));
        write(enc((n >> 6) & 63));
        write(enc((n >> 0) & 63));
      }

      match mod_len {
        0 => (),
        1 => {
          let n = (input[len - 1] as u32) << 16;
          write(enc((n >> 18) & 63));
          write(enc((n >> 12) & 63));
        },
        2 => {
          let n = (input[len - 2] as u32) << 16 | (input[len - 1] as u32) << 8;
          write(enc((n >> 18) & 63));
          write(enc((n >> 12) & 63));
          write(enc((n >> 6) & 63));
        },
        _ => unreachable!(),
      }
    }
  }
  result
}
