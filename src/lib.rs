extern crate log;

use std::sync::Once;

pub mod libssh_server;
pub mod libssh;
pub mod ssh_key;
pub mod ssh_session;
pub mod ssh_bind;
pub mod ssh_message;

static SSH_INIT: Once = Once::new();

pub fn ssh_init() {
	SSH_INIT.call_once(|| {
		unsafe { libssh::ssh_init() };
	})
}

pub fn ssh_finalize() {
	log::debug!("calling ssh_finalize().");
	unsafe { libssh::ssh_finalize() };
}

pub struct SSHFinalizer;
impl Drop for SSHFinalizer {
	fn drop(&mut self) {
		ssh_finalize();
	}
}

pub fn with_ssh<F>(func: F) where F: Fn() {
	ssh_init();

	let finalizer = SSHFinalizer;
	func();
	drop(finalizer);
}
