#![allow(unused_imports)]

extern crate libc;

use crate::sftp::*;
use crate::libssh;
use crate::libssh_server;
use crate::ssh_session::SSHSession;
use crate::sftp_session::SFTPSession;
use crate::sftp_attributes::SFTPAttributes;

use std::{mem, ptr};
use std::ffi::*;

pub struct SFTPDir {
	_dir: *mut sftp_dir_struct
}

impl SFTPDir {
	pub fn open(sftp: &SFTPSession, path: &str) -> Result<SFTPDir, ()> {
		let f = unsafe { sftp_opendir(sftp.raw(), CString::new(path).unwrap().as_ptr()) };
		if f.is_null() {
			let e = sftp.get_error();
			panic!("Could not open directory: {:?}", e);
		}
		Ok(SFTPDir { _dir: f })
	}		

	pub fn raw(self: &Self) -> *mut sftp_dir_struct {
		assert!(!self._dir.is_null());

		self._dir
	}

	pub fn is_eof(self: &Self) -> bool {
		assert!(!self._dir.is_null());

		match unsafe { sftp_dir_eof(self._dir) } {
			0 => false,
			_ => true,
		}
	}

	pub fn read(self: &Self, sftp: &SFTPSession) -> Result<SFTPAttributes, ()> {
		//note that returning an Err here isn't necessarily bad. It can also mean EOF
		assert!(!self._dir.is_null());

		let att = unsafe { sftp_readdir(sftp.raw(), self._dir) };
		if att.is_null() {
			return Err(())
		}
		Ok(SFTPAttributes::new(att).unwrap())
	}
}

impl Drop for SFTPDir {
	fn drop(self: &mut Self) {
		unsafe { sftp_closedir(self._dir); }
	}
}
