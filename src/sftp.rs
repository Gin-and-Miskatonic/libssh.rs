#![allow(missing_copy_implementations)]
#![allow(improper_ctypes)]
#![allow(non_camel_case_types)]

extern crate libc;
use std::mem;

use crate::libssh::*;

extern "C" {
	pub fn sftp_new(session: *mut ssh_session_struct) -> *mut sftp_session_struct;
}

extern "C" {
	pub fn sftp_new_channel(session: *mut ssh_session_struct, channel: *mut ssh_channel_struct) -> *mut sftp_session_struct;
}

extern "C" {
	pub fn sftp_free(sftp: *mut sftp_session_struct);
}

extern "C" {
	pub fn sftp_init(sftp: *mut sftp_session_struct) -> libc::c_int;
}

extern "C" {
	pub fn sftp_get_error(sftp: *mut sftp_session_struct) -> libc::c_int;
}

extern "C" {
	pub fn sftp_extensions_get_count(sftp: *mut sftp_session_struct) -> libc::c_uint;
}

extern "C" {
	pub fn sftp_extensions_get_name(sftp: *mut sftp_session_struct, indexn: libc::c_uint) -> *const libc::c_char;
}

extern "C" {
	pub fn sftp_extensions_get_data(sftp: *mut sftp_session_struct, indexn: libc::c_uint) -> *const libc::c_char;
}

extern "C" {
	pub fn sftp_extension_supported(sftp: *mut sftp_session_struct, name: *const libc::c_char, data: *const libc::c_char) -> libc::c_int;
}

extern "C" {
	pub fn sftp_opendir(session: *mut sftp_session_struct, path: *const libc::c_char) -> *mut sftp_dir_struct;
}

extern "C" {
	pub fn sftp_readdir(session: *mut sftp_session_struct, dir: *mut sftp_dir_struct) -> *mut sftp_attributes_struct;
}

extern "C" {
	pub fn sftp_dir_eof(dir: *mut sftp_dir_struct) -> libc::c_int;
}

extern "C" {
	pub fn sftp_stat(session: *mut sftp_session_struct, path: *const libc::c_char) -> *mut sftp_attributes_struct;
}

extern "C" {
	pub fn sftp_lstat(session: *mut sftp_session_struct, path: *const libc::c_char) -> *mut sftp_attributes_struct;
}

extern "C" {
	pub fn sftp_fstat(file: *mut sftp_file_struct) -> *mut sftp_attributes_struct;
}

extern "C" {
	pub fn sftp_attributes_free(file: *mut sftp_attributes_struct);
}

extern "C" {
	pub fn sftp_closedir(dir: *mut sftp_dir_struct) -> libc::c_int;
}

extern "C" {
	pub fn sftp_close(file: *mut sftp_file_struct) -> libc::c_int;
}

extern "C" {
	pub fn sftp_open(session: *mut sftp_session_struct, file: *const libc::c_char, accesstype: libc::c_int, mode: libc::mode_t) -> *mut sftp_file_struct;
}

extern "C" {
	pub fn sftp_file_set_nonblocking(handle: *mut sftp_file_struct);
}

extern "C" {
	pub fn sftp_file_set_blocking(handle: *mut sftp_file_struct);
}

extern "C" {
	pub fn sftp_read(file: *mut sftp_file_struct, buf: *mut libc::c_void, count: libc::size_t) -> libc::ssize_t;
}

extern "C" {
	pub fn sftp_async_read_begin(file: *mut sftp_file_struct, len: u32) -> libc::c_int;
}

extern "C" {
	pub fn sftp_async_read(file: *mut sftp_file_struct, data: *mut libc::c_void, len: u32, id: u32) -> libc::c_int;
}

extern "C" {
	pub fn sftp_write(file: *mut sftp_file_struct, buf: *mut libc::c_void, count: libc::size_t) -> libc::ssize_t;
}

extern "C" {
	pub fn sftp_seek(file: *mut sftp_file_struct, new_offset: u32) -> libc::c_int;
}

extern "C" {
	pub fn sftp_seek64(file: *mut sftp_file_struct, new_offset: u64) -> libc::c_int;
}

extern "C" {
	pub fn sftp_tell(file: *mut sftp_file_struct) -> libc::c_ulong;
}

extern "C" {
	pub fn sftp_tell64(file: *mut sftp_file_struct) -> u64;
}

extern "C" {
	pub fn sftp_rewind(file: *mut sftp_file_struct);
}

extern "C" {
	pub fn sftp_unlink(sftp: *mut sftp_session_struct, file: *const libc::c_char) -> libc::c_int;
}

extern "C" {
	pub fn sftp_rmdir(sftp: *mut sftp_session_struct, directory: *const libc::c_char) -> libc::c_int;
}

extern "C" {
	pub fn sftp_mkdir(sftp: *mut sftp_session_struct, directory: *const libc::c_char, mode: libc::mode_t) -> libc::c_int;
}

extern "C" {
	pub fn sftp_rename(sftp: *mut sftp_session_struct, original: *const libc::c_char, newname: *const libc::c_char) -> libc::c_int;
}

extern "C" {
	pub fn sftp_setstat(sftp: *mut sftp_session_struct, file: *const libc::c_char, attr: *mut sftp_attributes_struct) -> libc::c_int;
}

extern "C" {
	pub fn sftp_chown(sftp: *mut sftp_session_struct, file: *const libc::c_char, owner: libc::uid_t, group: libc::gid_t) -> libc::c_int;
}

extern "C" {
	pub fn sftp_chmod(sftp: *mut sftp_session_struct, file: *const libc::c_char, mode: libc::mode_t) -> libc::c_int;
}

extern "C" {
	pub fn sftp_utimes(sftp: *mut sftp_session_struct, file: *const libc::c_char, times: *const timeval) -> libc::c_int;
}

extern "C" {
	pub fn sftp_symlink(sftp: *mut sftp_session_struct, target: *const libc::c_char, dest: *const libc::c_char) -> libc::c_int;
}

extern "C" {
	pub fn sftp_readlink(sftp: *mut sftp_session_struct, file: *const libc::c_char) -> *mut libc::c_char;
}

extern "C" {
	pub fn sftp_statvfs(sftp: *mut sftp_session_struct, path: *const libc::c_char) -> *mut sftp_statvfs_struct;
}

extern "C" {
	pub fn sftp_fstatvfs(file: *mut sftp_file_struct) -> *mut sftp_statvfs_struct;
}

extern "C" {
	pub fn sftp_statvfs_free(statvfs_o: *mut sftp_statvfs_struct);
}

extern "C" {
	pub fn sftp_fsync(file: *mut sftp_file_struct) -> libc::c_int;
}

extern "C" {
	pub fn sftp_canonicalize_path(sftp: *mut sftp_session_struct, path: *const libc::c_char) -> *mut libc::c_char;
}

extern "C" {
	pub fn sftp_server_version(sftp: *mut sftp_session_struct) -> libc::c_int;
}

extern "C" {
	pub fn sftp_packet_read(sftp: *mut sftp_session_struct) -> *mut sftp_packet_struct;
}

extern "C" {
	pub fn sftp_packet_write(sftp: *mut sftp_session_struct, type_: u8, payload: *mut ssh_buffer_struct) -> libc::c_int;
}

extern "C" {
	pub fn sftp_packet_free(packet: *mut sftp_packet_struct);
}

extern "C" {
	pub fn buffer_add_attributes(buffer: *mut ssh_buffer_struct, attr: *mut sftp_attributes_struct) -> libc::c_int;
}

extern "C" {
	pub fn sftp_parse_attr(sftp: *mut sftp_session_struct, buf: *mut ssh_buffer_struct, expectname: libc::c_int) -> *mut sftp_attributes_struct;
}

extern "C" {
	pub fn sftp_get_client_message(sftp: *mut sftp_session_struct) -> *mut sftp_client_message_struct;
}

extern "C" {
	pub fn sftp_client_message_free(msg: *mut sftp_client_message_struct);
}

extern "C" {
	pub fn sftp_client_message_get_type(msg: *mut sftp_client_message_struct) -> u8;
}

extern "C" {
	pub fn sftp_client_message_get_filename(msg: *mut sftp_client_message_struct) -> *const libc::c_char;
}

extern "C" {
	pub fn sftp_client_message_set_filename(msg: *mut sftp_client_message_struct, newname: *const libc::c_char) -> *const libc::c_char;
}

extern "C" {
	pub fn sftp_client_message_get_data(msg: *mut sftp_client_message_struct) -> *const libc::c_char;
}

extern "C" {
	pub fn sftp_client_message_get_flags(msg: *mut sftp_client_message_struct) -> u32;
}

extern "C" {
	pub fn sftp_client_message_get_submessage(msg: *mut sftp_client_message_struct) -> *const libc::c_char;
}

extern "C" {
	pub fn sftp_send_client_message(sftp: *mut sftp_session_struct, msg: *mut sftp_client_message_struct) -> libc::c_int;
}

extern "C" {
	pub fn sftp_reply_name(msg: *mut sftp_client_message_struct, name: *const libc::c_char, attr: *mut sftp_attributes_struct) -> libc::c_int;
}

extern "C" {
	pub fn sftp_sftp_reply_handle(msg: *mut sftp_client_message_struct, handle: *mut ssh_string_struct) -> libc::c_int;
}

extern "C" {
	pub fn sftp_handle_alloc(sftp: *mut sftp_session_struct, info: *mut libc::c_void) -> *mut ssh_string_struct;
}

extern "C" {
	pub fn sftp_reply_attr(msg: *mut sftp_client_message_struct, attr: *mut sftp_attributes_struct) -> libc::c_int;
}

extern "C" {
	pub fn sftp_handle(sftp: *mut sftp_session_struct, handle: *mut ssh_string_struct) -> *mut libc::c_void;
}

extern "C" {
	pub fn sftp_reply_status(msg: *mut sftp_client_message_struct, status: u32, message: *const libc::c_char) -> libc::c_int;
}

extern "C" {
	pub fn sftp_reply_names_add(msg: *mut sftp_client_message_struct, file: *const libc::c_char, longname: *const libc::c_char, attr: *mut sftp_attributes_struct) -> libc::c_int;
}

extern "C" {
	pub fn sftp_reply_names(msg: *mut sftp_client_message_struct) -> libc::c_int;
}

extern "C" {
	pub fn sftp_reply_data(msg: *mut sftp_client_message_struct, data: *const libc::c_void, len: libc::c_int) -> libc::c_int;
}

extern "C" {
	pub fn sftp_handle_remove(sftp: *mut sftp_session_struct, handle: *mut libc::c_void);
}

#[repr(C)]
pub struct sftp_attributes_struct;

#[repr(C)]
pub struct sftp_client_message_struct;

#[repr(C)]
pub struct sftp_dir_struct;

#[repr(C)]
pub struct sftp_ext_struct;

#[repr(C)]
pub struct sftp_file_struct;

#[repr(C)]
pub struct sftp_message_struct;

#[repr(C)]
pub struct sftp_packet_struct;

#[repr(C)]
pub struct sftp_request_queue_struct;

#[repr(C)]
pub struct sftp_session_struct;

#[repr(C)]
pub struct sftp_status_message_struct;

#[repr(C)]
pub struct sftp_statvfs_struct;

#[derive(Clone, Copy, PartialEq, Debug)]
#[repr(u32)]
pub enum sftp_server_responses_e {
	SSH_FX_OK = 0,
	SSH_FX_EOF = 1,
	SSH_FX_NO_SUCH_FILE = 2,
	SSH_FX_PERMISSION_DENIED = 3,
	SSH_FX_FAILURE = 4,
	SSH_FX_BAD_MESSAGE = 5,
	SSH_FX_NO_CONNECTION = 6,
	SSH_FX_CONNECTION_LOST = 7,
	SSH_FX_OP_UNSUPPORTED = 8,
	SSH_FX_INVALID_HANDLE = 9,
	SSH_FX_NO_SUCH_PATH = 10,
	SSH_FX_FILE_ALREADY_EXISTS = 11,
	SSH_FX_WRITE_PROTECT = 12,
	SSH_FX_NO_MEDIA = 13,
}

impl sftp_server_responses_e {
	pub fn to_i32(&self) -> libc::c_int{
		*self as libc::c_int
	}
	pub fn from_i32(v: libc::c_int) -> sftp_server_responses_e {
		unsafe { mem::transmute(v) }
	}
}
