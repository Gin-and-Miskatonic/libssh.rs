#![allow(unused_imports)]

extern crate libc;

use crate::sftp::*;
use crate::libssh;
use crate::libssh_server;
use crate::ssh_session::SSHSession;
use crate::sftp_session::SFTPSession;

use std::{mem, ptr};
use std::ffi::*;

pub struct SFTPAttributes {
	_att: *mut sftp_attributes_struct
}

impl SFTPAttributes {
//TODO: implement more functions for pulling info from the attributes struct
	pub fn new(att: *mut sftp_attributes_struct) -> Result<SFTPAttributes, ()> {
		if att.is_null() {
			Err(())
		} else {
			Ok(SFTPAttributes{ _att: att})
		}
	}

	pub fn raw(self: &Self) -> *mut sftp_attributes_struct {
		assert!(!self._att.is_null());

		self._att
	}

	pub fn name(self: &Self) -> &str {
		assert!(!self._att.is_null());
		let s = unsafe{ std::ffi::CStr::from_ptr((*self._att).name) };
		s.to_str().unwrap()
	}

	pub fn longname(self: &Self) -> &str {
		assert!(!self._att.is_null());
		let s = unsafe{ std::ffi::CStr::from_ptr((*self._att).longname) };
		s.to_str().unwrap()
	}

	pub fn owner(self: &Self) -> &str {
		assert!(!self._att.is_null());
		let s = unsafe{ std::ffi::CStr::from_ptr((*self._att).owner) };
		s.to_str().unwrap()
	}
}

impl Drop for SFTPAttributes {
	fn drop(self: &mut Self) {
		unsafe { sftp_attributes_free(self._att); }
	}
}
