#![allow(unused_imports)]

extern crate libc;

use crate::libssh;
use crate::libssh_server;
use crate::libssh_server::*;
use crate::ssh_session::SSHSession;
use crate::ssh_key::SSHKey;
use crate::ssh_message::SSHMessage;

use std::mem;
use std::ptr;
//use self::libc::types::common::c95::c_void;
use std::ffi::*;

pub struct SSHChannel {
	_channel: *mut ssh_channel_struct
}

impl SSHChannel {
	pub fn new(session: &SSHSession) -> Result<SSHChannel, ()> {
		let c = unsafe { ssh_channel_new(session.raw() as *mut libssh_server::ssh_session_struct) };
		if c.is_null(){
			return Err(())
		}
		
		Ok(SSHChannel{ _channel: c })
	}

	pub fn raw(self: &Self) -> *mut ssh_channel_struct {
		assert!(!self._channel.is_null());

		self._channel
	}

	pub fn send_eof(self: &Self) -> i32 {
		assert!(!self._channel.is_null());
		
		unsafe { ssh_channel_send_eof(self._channel) as i32 }
	}

	pub fn is_eof(self: &Self) -> bool {
		assert!(!self._channel.is_null());
		
		match unsafe { ssh_channel_is_eof(self._channel) } {
			0 => false,
			_ => true,
		}
	}

	pub fn is_open(self: &Self) -> bool {
		assert!(!self._channel.is_null());

		match unsafe { ssh_channel_is_open(self._channel) } {
			0 => false,
			_ => true
		}
	}

	pub fn open_session(self: &Self) -> i32 {
		assert!(!self._channel.is_null());
		
		unsafe { ssh_channel_open_session(self._channel) }
	}

	pub fn open_reverse_forward(self: &Self, remotehost: &str, remoteport: i32, sourcehost: &str, localport: i32) -> i32 {
		assert!(!self._channel.is_null());

		unsafe { 
			ssh_channel_open_reverse_forward(
			self._channel,
			CString::new(remotehost).unwrap().as_ptr(),
			remoteport as libc::c_int,
			CString::new(sourcehost).unwrap().as_ptr(),
			localport as libc::c_int)
		}
	}

	pub fn read_nonblocking(self: &Self, dest: &mut [u8], count: usize, is_stderr: bool) -> i32 {
		assert!(!self._channel.is_null());
		let pointer = std::ptr::NonNull::new(dest).unwrap().cast::<c_void>().as_ptr();
		let stderr = match is_stderr{
			true => 1,
			false => 0,
		};
		unsafe { ssh_channel_read_nonblocking(self._channel, pointer, count as u32, stderr) }
	}

	pub fn write(self: &Self, buf: &mut [u8], count: usize) -> i32 {
		assert!(!self._channel.is_null());
		let pointer = std::ptr::NonNull::new(buf).unwrap().cast::<c_void>().as_ptr();
		unsafe { ssh_channel_write(self._channel, pointer, count as u32) }
	}
}

impl Drop for SSHChannel {
	fn drop(self: &mut Self){
		unsafe {
		ssh_channel_close(self._channel); 
		ssh_channel_free(self._channel);
		}
	}
}
