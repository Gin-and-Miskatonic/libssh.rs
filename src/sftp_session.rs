#![allow(unused_imports)]

extern crate libc;

use crate::sftp::*;
use crate::libssh;
use crate::libssh_server;
use crate::ssh_session::SSHSession;
use crate::ssh_key::SSHKey;
use crate::ssh_message::SSHMessage;

use std::mem;
use std::ptr;
use std::ffi::*;

pub struct SFTPSession {
	_sftp: *mut sftp_session_struct
}

impl SFTPSession {
	pub fn new(session: &SSHSession) -> Result<SFTPSession, ()> {
		let sftp = unsafe { sftp_new(session.raw()) };
		assert!(!sftp.is_null());

		Ok(SFTPSession { _sftp: sftp })
	}

	pub fn raw(self: &Self) -> *mut sftp_session_struct {
		assert!(!self._sftp.is_null());
		self._sftp
	}

	pub fn init(self: &Self) -> i32 {
		assert!(!self._sftp.is_null());

		unsafe { sftp_init(self._sftp) }
	}

	pub fn get_error(self: &Self) -> sftp_server_responses_e {
		assert!(!self._sftp.is_null());

		unsafe { sftp_server_responses_e::from_i32(sftp_get_error(self._sftp)) }
	}

}

impl Drop for SFTPSession {
	fn drop(self: &mut Self) {
		unsafe { sftp_free(self._sftp); }
	}
}
