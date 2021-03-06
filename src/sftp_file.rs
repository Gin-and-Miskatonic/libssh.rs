#![allow(unused_imports)]

extern crate libc;

use crate::sftp::*;
use crate::libssh;
use crate::libssh_server;
use crate::ssh_session::SSHSession;
use crate::sftp_session::SFTPSession;

use std::mem;
use std::ptr;
use std::ffi::*;

pub struct SFTPFile {
	_file: *mut sftp_file_struct
}

impl SFTPFile {
	pub fn open(sftp: &SFTPSession, path: &str, accesstype: i32, mode: u32) -> Result<SFTPFile, ()> {
		let f = unsafe { sftp_open(sftp.raw(), CString::new(path).unwrap().as_ptr(), accesstype as libc::c_int, mode as u32) };
		if f.is_null() {
			let e = sftp.get_error();
			panic!("Could not open file: {:?}", e);
		}
		Ok(SFTPFile { _file: f } )
	}		

	pub fn raw(self: &Self) -> *mut sftp_file_struct {
		self._file
	}

	pub fn set_blocking(self: &Self){
		unsafe { sftp_file_set_blocking(self._file); }
	}

	pub fn set_nonblocking(self: &Self){
		unsafe { sftp_file_set_nonblocking(self._file); }
	}

	pub fn read(self: &Self, buf: &mut [u8], count: usize) -> isize {
		assert!(!self._file.is_null());
		
		let pointer = std::ptr::NonNull::new(buf).unwrap().cast::<libc::c_void>();
		unsafe { sftp_read(self._file, pointer.as_ptr(), count as libc::size_t) as isize }
	}

	pub fn write(self: &Self, data: &mut [u8], count: usize) -> isize {
		assert!(!self._file.is_null());
		
		let pointer = std::ptr::NonNull::new(data).unwrap().cast::<libc::c_void>();
		unsafe { sftp_write(self._file, pointer.as_ptr(), count as libc::size_t) as isize }
	}

}

impl Drop for SFTPFile {
	fn drop(self: &mut Self) {
		unsafe { sftp_close(self._file); }
	}
}
