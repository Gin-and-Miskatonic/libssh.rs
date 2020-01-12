#![allow(unused_imports)]
#![allow(missing_copy_implementations)]

extern crate libc;
extern crate log;

use log::debug;

use crate::libssh_server::*;
use crate::ssh_key;
use crate::ssh_session::SSHSession;
use crate::ssh_message;

use std::mem;
use std::ptr;
use std::ffi::*;
//use self::libc::types::common::c95::c_void;

pub struct SSHBind {
	_bind: *mut ssh_bind_struct
}

impl SSHBind {
	pub fn new(priv_key_file: &str, host: Option<&str>, port: Option<u32>)
		-> Result<SSHBind, &'static str>
	{
		let ptr = unsafe { ssh_bind_new() };
		assert!(!ptr.is_null());

		let bind = SSHBind { _bind: ptr };
		
		if host.is_some() {
			bind.set_host(host.unwrap())?;
		}
		bind.set_port(port.unwrap_or(22))?;

		bind.set_private_key_file(priv_key_file)?;

		Ok(bind)
	}

	pub fn set_host(&self, host: &str) -> Result<(),&'static str> {
		assert!(!self._bind.is_null());

		let opt = ssh_bind_options_e::SSH_BIND_OPTIONS_BINDADDR as u32;
		let res = unsafe { ssh_bind_options_set(self._bind, opt, CString::new(host).unwrap().as_ptr() as *const c_void) };

		match res {
			SSH_OK => Ok(()),
			_              => Err("ssh_bind_options_set() failed for setting host")
		}
	}

	pub fn set_port(&self, mut port: u32) -> Result<(),&'static str> {
		assert!(!self._bind.is_null());
		//when port was a &str let mut port = port.parse::<u32>().unwrap();
		//the line of nonsense below safely converts an unsigned rust int to a struct that can be turned into a c_void ptr
		let portref: std::ptr::NonNull<c_void> = std::ptr::NonNull::new(&mut port).unwrap().cast();
		let opt = ssh_bind_options_e::SSH_BIND_OPTIONS_BINDPORT as u32;
		//let res = unsafe { ssh_bind_options_set(self._bind, opt, CString::new(port).unwrap().as_ptr() as *const c_void) };
		let res = unsafe { ssh_bind_options_set(self._bind, opt, portref.as_ptr() as *const c_void) };

		match res {
			SSH_OK => Ok(()),
			_              => Err("ssh_bind_options_set() failed for setting port")
		}
	}

	pub fn set_private_key_file(&self, key_file: &str) -> Result<(),&'static str> {
		assert!(!self._bind.is_null());

		let opt = ssh_bind_options_e::SSH_BIND_OPTIONS_HOSTKEY as u32;
		let res = unsafe { ssh_bind_options_set(self._bind, opt, CString::new(key_file).unwrap().as_ptr() as *const c_void) };

		match res {
			SSH_OK => Ok(()),
			_              => Err("ssh_bind_options_set() failed for private key (RSAKEY)")
		}
	}

	pub fn listen(&self) -> Result<(),&'static str> {
		assert!(!self._bind.is_null());

		let res = unsafe { ssh_bind_listen(self._bind) };
		debug!("listen={}", res);
		match res {
			SSH_OK => Ok(()),
			_              => Err("ssh_bind_listen() failed")
		}
	}

	pub fn accept(&self, session: &SSHSession) -> Result<(),&'static str> {
		assert!(!self._bind.is_null());

		let res = unsafe { ssh_bind_accept(self._bind, mem::transmute(session.raw())) };
		match res {
			SSH_OK => Ok(()),
			_              => Err("ssh_bind_accept() failed")
		}
	}

	pub fn set_log_level(&self, level: i32) -> Result<(),&'static str> {
		assert!(!self._bind.is_null());
		let res = unsafe { ssh_set_log_level(level) };
		match res {
			SSH_OK => Ok(()),
			_              => Err("ssh_set_log_level() failed")
		}
	}
}
