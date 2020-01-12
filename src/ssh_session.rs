#![allow(unused_imports)]

extern crate libc;

use crate::libssh::*;
use crate::libssh_server;
use crate::ssh_key::SSHKey;
use crate::ssh_message::SSHMessage;

use std::mem;
use std::ptr;
//use self::libc::types::common::c95::c_void;
use std::ffi::*;

pub struct SSHSession {
	_session: *mut ssh_session_struct
}

impl SSHSession {
	pub fn new(user: Option<&str>, host: Option<&str>) -> Result<SSHSession, ()> {
		let ptr = unsafe { ssh_new() };
		assert!(!ptr.is_null());

		let session = SSHSession {_session: ptr};
		if user.is_some() {
			session.set_user(user.unwrap())?
		}
		if host.is_some() {
			session.set_host(host.unwrap())?
		}

		Ok(session)
	}

	pub fn set_host(&self, host: &str) -> Result<(),()> {
		assert!(!self._session.is_null());

		let opt = ssh_options_e::SSH_OPTIONS_HOST as u32;
		let res = unsafe { ssh_options_set(self._session, opt, CString::new(host).unwrap().as_ptr() as *const c_void) } ;

		match res {
			SSH_OK => Ok(()),
			_           => Err(())
		}
	}

	pub fn set_user(&self, user: &str) -> Result<(),()> {
		assert!(!self._session.is_null());

		let opt = ssh_options_e::SSH_OPTIONS_USER as u32;
		let res = unsafe { ssh_options_set(self._session, opt, CString::new(user).unwrap().as_ptr() as *const c_void) } ;

		match res {
			SSH_OK => Ok(()),
			_           => Err(())
		}
	}

 //formerly pub fn connect(&self, verify_public_key: |public_key: &SSHKey| -> bool)
	pub fn connect<F>(&self, verify_public_key: F) -> Result<(), String> where F: Fn(&SSHKey) -> bool {
		assert!(!self._session.is_null());

		let res = unsafe { ssh_connect(self._session) };
		if res != SSH_OK {
			let ptr = self._session as *mut c_void;

			let err_msg = unsafe {
				let err = ssh_get_error(ptr);
				assert!(!err.is_null());

				//String::from_raw_buf(err as *const u8)
				CString::from_raw(err as *mut i8).into_string().unwrap()
			};
			return Err(err_msg);
		}

		let remote_public_key = SSHKey::from_session(self).map_err(|err| err.to_string());
		if !verify_public_key(&remote_public_key.unwrap()) {
			self.disconnect();
			return Err("authentication failed".to_string());
		}
		else {
			Ok(())
		}
	}

	pub fn disconnect(&self) {
		assert!(!self._session.is_null());

		unsafe {
			ssh_disconnect(self._session);
		}
	}

	pub fn auth_by_public_key(&self, username: Option<&str>, pubkey: &SSHKey)
		-> Result<(),ssh_auth_e>
	{
		/*
		    SSH_AUTH_ERROR: A serious error happened.
		    SSH_AUTH_DENIED: The server doesn't accept that public key as an authentication token. Try another key or another method.
		    SSH_AUTH_PARTIAL: You've been partially authenticated, you still have to use another method.
		    SSH_AUTH_SUCCESS: The public key is accepted, you want now to use ssh_userauth_pubkey(). SSH_AUTH_AGAIN: In nonblocking mode, you've got to call this again later.
		*/
		assert!(!self._session.is_null());

		let key = pubkey.raw();
		let func = |usr| unsafe { ssh_userauth_try_publickey(self._session, usr, key) };

		let ires = if username.is_none() { func(ptr::null()) } else
			{ func(CString::new(username.unwrap()).unwrap().as_ptr()) };

		let res = ssh_auth_e::from_i32(ires);
		match res {
			ssh_auth_e::SSH_AUTH_SUCCESS => println!("Key accepted, proceeding with attempt"),
			ssh_auth_e::SSH_AUTH_PARTIAL |
			ssh_auth_e::SSH_AUTH_DENIED |
			ssh_auth_e::SSH_AUTH_AGAIN |
			ssh_auth_e::SSH_AUTH_ERROR => return Err(res),
			x => {panic!("{:?}", x);}
		}

		let func = |usr| unsafe { ssh_userauth_publickey(self._session, usr, key) };

		let ires = if username.is_none() { func(ptr::null()) } else
			{ func(CString::new(username.unwrap()).unwrap().as_ptr()) };
		match ssh_auth_e::from_i32(ires) {
			ssh_auth_e::SSH_AUTH_SUCCESS => Ok(()),
			ssh_auth_e::SSH_AUTH_PARTIAL |
			ssh_auth_e::SSH_AUTH_DENIED |
			ssh_auth_e::SSH_AUTH_AGAIN |
			ssh_auth_e::SSH_AUTH_ERROR => Err(res),
			x => {panic!("{:?}", x);}
		}
	}

	pub fn raw(&self) -> *mut ssh_session_struct {
		assert!(!self._session.is_null());
		self._session
	}

	pub fn set_port(&self, mut port: i32) -> Result<(),&'static str> {
		assert!(!self._session.is_null());

		let opt = ssh_options_e::SSH_OPTIONS_PORT as u32;
		let pointer = std::ptr::NonNull::new(&mut port).unwrap().cast::<libc::c_void>().as_ptr();
		let res = unsafe{ ssh_options_set(self._session, opt, pointer) };

		match res {
			SSH_OK => Ok(()),
			_              => Err("ssh_options_set() failed for setting port")
		}
	}

	pub fn auth_with_public_key<'a, F>(&self, verify_public_key: F) -> Result<(),&'a str> where F: Fn(&SSHKey) -> bool{
		const MAX_ATTEMPTS: u32 = 5;

		for _  in 0..MAX_ATTEMPTS {
			let msg = SSHMessage::from_session(self)?;

			let type_ = msg.get_type();
			let subtype = msg.get_subtype();

			match (type_, subtype) {
				(libssh_server::ssh_requests_e::SSH_REQUEST_AUTH,
						libssh_server::SSH_AUTH_METHOD_PUBLICKEY) =>
				{
					let remote_public_key = SSHKey::from_message(&msg)?;
					
					if verify_public_key(&remote_public_key) {
						unsafe { libssh_server::ssh_message_auth_reply_success(msg.raw(), 0); }
						return Ok(());
					}
				},

				_ => {
					msg.reply_default()?
				}
			}
		}
		Err("authentication with public key failed")
	}

	pub fn handle_key_exchange(&self) -> Result<(),&'static str> {
		assert!(!self._session.is_null());

		let session: *mut libssh_server::ssh_session_struct = unsafe {
			mem::transmute(self._session)
		};
		let res = unsafe { libssh_server::ssh_handle_key_exchange(session) };
		match res {
			SSH_OK => Ok(()),
			_              => Err("ssh_handle_key_exchange() failed")
		}
	}

	pub fn set_log_level(&self, level: i32) -> Result<(),&'static str> {
		assert!(!self._session.is_null());
		let res = unsafe { ssh_set_log_level(level) };
		match res {
			SSH_OK => Ok(()),
			_              => Err("ssh_set_log_level() failed")
		}
	}
}

impl Drop for SSHSession {
	fn drop(&mut self) {
		unsafe {
			ssh_disconnect(self._session);
			ssh_free(self._session);
		}
	}
}
