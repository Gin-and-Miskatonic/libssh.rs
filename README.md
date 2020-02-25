# Libssh.rs: A Rust Wrapper for Libssh
---
This package provides foreign function interfaces for all functions in the libssh API, including server and SFTP functionality.
Beyond providing access to these functions, the package provides an object-oriented model for handling ssh sessions and structs.
For more information about the original library, see the documentation at <https://libssh.org>, especially <https://api.libssh.org> and the [libssh tutorial](http://api.libssh.org/master/libssh\_tutorial.html).

## The SSH Model
---
The Secure Shell protocol is defined by RFCs [4251](https://tools.ietf.org/html/rfc4251), [4252](https://tools.ietf.org/html/rfc4252) [4253](https://tools.ietf.org/html/rfc4253) and [4254](https://tools.ietf.org/html/rfc4254). These documents define the expected communications between the client and server in a typical ssh session. From this basis, libssh uses several data structures to track information in active sessions and simplify the process of maintaining a secure connection. Unlike in the original C code, the Rust implementation of libssh allows for an object-oriented model where data structures have methods for their associated functions.

## Structs and Methods
---
The following data structures are included with method implementations to simplify the tracking and use of related functions.
For example, creating an ssh session returns a session object. You can then call session.connect to make a connection to a server over that session.

## SSHSession
The session object allows the user to keep track of conversation with a particular client or server.
---

#### new
(user: Option<&str\>, host: Option<&str\>) -\> Result<SSHSession, ()\>

This method can be called statically to create a new SSHSession object. Optionally, you may pass a username and a host to connect to so that the following two methods are handled automatically. The host should be a string slice describing an IP address. If you pass None at this time, at least the host should be set before calling connect(). 

#### raw
-\> \*mut ssh\_session\_struct

Used to expose a copy of the pointer to the struct. You should probably (hopefully) never need/use this.

#### set\_host
(user: &str) -\> Result<(),()\>

Sets the hostname for the session to connect to.

#### set\_user
(host: &str) -\> Result<(),()\>

The preferred method for authenticating as a particular user. The default is your local username, so this function should be set before attempting to connect if you need to use a different name.

#### set\_port
(port: i32) -\> Result<(),()\>

Used to change the port that the client will connect on. Important to note that the default is not necessarily 22, despite that being the protocol-wide default, so you should set this before connecting no matter what.

#### connect <F\>
(verify\_public\_key: F) -\> Result<(), String\> where F: Fn(&SSHKey) -\> bool

Establish a connection to an ssh server. You're not done just by calling this.
If you aren't familiar with the format, the above method declaration is saying that this method needs to be passed a closure that takes an SSHKey and returns a boolean value. For example, default acceptance of the ssh server would look like this:
```rust
mysession.connect(|public\_key| { true });
```
It actually returns a Result with an empty object on success and a string describing the error on failure.

#### disconnect
You shouldn't need to worry about this, because it will be called when the session leaves scope, but if you want to end a session early this is the method to call.

#### auth\_by\_public\_key
(username: Option<&str\>, pubkey: &SSHKey) -\> Result<(), ssh\_auth\_e\>

This is a method for client sessions to authenticate themselves to the server over pki. If the username was set using the set\_user method as recommended, the first parameter here should be none. The second should be a private SSHKey boject, despite the name (kept for historical reasons).

#### auth\_with\_public\_key <'a, F\>
(verify\_public\_key: F) -\> Result<(),&'a str\> where F: Fn(&SSHKey) -\> bool

A method for servers to verify clients are authorized. Like the connect method above, the parameter here accepts a closure that takes a key and returns a boolean. If this function returns an error, do not proceed with logging the user into the server.

#### set\_log\_level
(level: i32)
Change the verbosity of log output. You can find more info about ssh logging in the official libssh documentation.

## SSHChannel
Within a session, almost all data, including commands, is passed between client and server via channels. This object allows the user to allocate new channels for communication with the server (or client, depending on your role).

####new
(session: &SSHSession) -\> Result<SSHCHannel, ()\>

Used to establish a new channel attached to a given session. This only creates the channel locally. It must be opened to connect to the server/client.

####raw
-\> \*mut ssh\_channel\_struct

This returns a pointer to the underlying channel object. You should hopefully not need to use this.

####send\_eof
-\> i32

Use this function to tell the other side you are done sending data. You will still be able to read data from the channel but can no longer write to it.

####is\_eof
-\> bool

Will return true if the other side has sent an EOF across the channel. Once both of you have sent an EOF, the channel no longer has a use and can be closed and freed (handled automatically when the object leaves scope).

####is\_open
-\> bool

Returns true if the channel is open. In other words, this method can be used to test whether the channel has successfully connected to the other side.

####open\_session
-\> i32

This method opens the created channel as a session channel. Session channels are suitable for passing commands and creating shells, not for passing TCP data.

####open\_reverse\_forward
(remotehost: &str, remoteport: i32, sourcehost: &str, localport: i32) -\> i32

This method opens a channel from the server to the client that can pass TCP data back and forth. This does not implement the tunnel in its entirety. You still need to read from and write to the channel. The client must request a reverse tunnel first, and any unsolicited tunnel requests will be rejected by a correct implementation.
####read\_nonblocking
(dest: &mut [u8], count: usize, is\_stderr: bool) -\> i32

Reads data from the channel and places it into the dest buffer. Will read up to count number of bytes, and returns the number of bytes actually read (should be less than count) or a negative number if the read failed. Returns 0 if there was nothing to read or if EOF is reached. The is\_stderr option decides whether you read from stdout or stderr (in the case of a session channel).

####write
(buf: &mut [u8], count: usize) -> i32

Writes the data from buf into the channel. Starts at the beginning of buf and writes count bytes. Returns the number of bytes actually written or a negative number on error.

## SSHBind
The SSHBind module is only useful for those who need a server-side implementation. It provides the object that listens for new ssh connections and can attach them to newly created sessions.

####new
(priv\_key\_file: &str, host: Option<&str>, port: Option<u32\>) -\> Result<SSHBind, &static str\>

The method creates a new bind object. Passing the host and port are optional at this stage, but the key file is required.

####set\_host
(host: &str) -\> Result<(),&static str\>

Sets the option for the hostname or IP (and by extension what interface) on which the bind will listen for connections. 0.0.0.0 will listen on all interfaces.

####set\_port
(port: u32) -\> Result<(), &static str\>

Sets the option for the port on which the bind will listen for connections. The default is seemingly arbitrary, so it is best to set this.

####set\_private\_key\_file
(key\_file: &str) -\> Result <(), &static str\>

This method sets the host key file for the server. The parameter should be a path to a valid private key file.

####listen
-\> Result<(), &static str\>

Like many other listeners, this causes the bind object to begin listening on the specified host/port for connections. It is a non-blocking call.
####accept
(session: &SSHSession) -\> Result<(), &static str\>

This is a blocking call that will wait for a connecion on the listening bind. Once a connection is established an accepted, it will be bound to the session given as a parameter, and the session can then be used to communicate with the client.

####set\_log\_level
(level: i32)
Change the verbosity of log output. You can find more info about ssh logging in the official libssh documentation.

## SSHMessage
Administrative information and requests are passed between client and server via messages. These operate outside of channels but within a session. Examples of messages include requests to open a new channel, or to close the server, as well as sending success/error responses. Please follow the RFCs listed above, especially 4250 and 4254, if you need to craft any messages.

####from\_session
(session: &SSHSession) -\> Result<SSHMessage, &static str\>
####raw
-\> \*mut ssh\_message\_struct

Returns a pointer to the raw message object for libssh.

####get\_type
-\> ssh\_requests\_e

Returns an instance of the ssh\_requests\_e enum, which can be one of the following:
SSH\_CHANNEL\_REQUEST\_UNKNOWN =0, 
SSH\_CHANNEL\_REQUEST\_PTY =2,
SSH\_CHANNEL\_REQUEST\_EXEC =3,
SSH\_CHANNEL\_REQUEST\_SHELL =4,
SSH\_CHANNEL\_REQUEST\_ENV =5,
SSH\_CHANNEL\_REQUEST\_SUBSYSTEM =6,
SSH\_CHANNEL\_REQUEST\_WINDOW\_CHANGE =7,
SSH_CHANNEL_REQUEST_X11 =8,

####get\_subtype
-\> i32

Returns an integer relating to the subtype of method. The exact enum this integer applies to depends on the primary type of the message. Check the libssh documentation for more information. May also return -1 in case of an error.

####get\_global\_request\_port
-\> i32

Returns the port associated with a global request such as a tunnel request, or -1 on an error.

####get\_global\_request\_address
-\> String

Returns the address associated with a global request, or -1 on an error.

####global\_reply\_success
-\> i32

Sends a reply message indicating that the request in the last message was successfully completed.

####reply\_default
-\> Result<(), &static str\>

Sends a generic failure message as a reply to the last request message received. NOTE THAT THIS IS NOT A DEFAULT SUCCESS MESSAGE. THIS SHOULD BE USED IN CASE OF REJECTION OR FAILURE.

## The SFTP Sub-System
---

## SFTPSession

####new
(session: &SSHSession) -\> Result<SFTPSession, ()\>
####raw
-\> \*mut sftp\_session\_struct
####init
-\> i32
####get\_error
-\> sftp\_server\_responses\_e

## SFTPFile

####open
(sftp: &SFTPSession, path: &str, accesstype: i32, mode: u32) -\> Result<SFTPFile, ()\>

This will open a file on the remote machine at the given path. Accesstype correlates to the *flags* parameter as specified in the [open(2) man page for C](http://man7.org/linux/man-pages/man2/open.2.html), and mode is an unsigned int correlating with linux permissions in octal (i.e. a mode of 777 allows all users to read, right, and execute the file, while a mode of 600 allows only the owner to read and write but not execute, and so on). The mode parameter should only matter in the case of a new file being created. If the file is being opened read only, this parameter may be left as a 0.

####raw
-\> \*mut sftp\_file\_struct

Returns the underlying raw C structure for sftp files.

####set\_blocking

Changes the file operations to operate in a blocking manner: reads and writes will complete before passing control to the next instruction.

####set\_nonblocking

Changes file operations to operate in a nonblocking manner: if the file has nothing to read or the write cannot immediately complete, the program will move to the next instruction rather than halting.

####read
(buf: &mut [u8], count: usize) -\> isize

Reads at most *count* bytes from the file into the specified buffer, and returns the total number of bytes actually read, or -1 on an error.

####write
(data: &mut [u8], count: usize) -\> isize

Writes *count* bytes from *data* to the file, and returns the number of bytes written. Ideally, count == return.

## SFTPDir

####open
(sftp: &SFTPSession, path: &str) -\> Result<SFTPDir, ()\>

Creates a new directory object from an active SFTP session and a path to the desired directory on the remote machine. This can be referenced to list contained files.

####raw
-\> \*mut sftp\_dir\_struct

Returns a raw pointer to the libssh struct for remote directories, in case you need to do something manual with it.

####is\_eof
-\> bool

This method returns whether all of the files contained in the directory have been read as attribute objects. If read().is\_err()==true, this should also be true or else a real error occurred.

####read
(sftp: SFTPSession) -\> Result<SFTPAttributes, ()\>

Every call to this method returns the attribute object for one file in the directory. To read all files in the directory, this function should be looped until read().is\_err()==true. At that point, the directory has either listed all of its contents, or an error has actually occurred. Check is\_eof() to find out.

## SFTPAttributes

####new
(att: \*mut stfp\_attributes\_struct) -\> Result<SFTPAttributes, ()\>

This method is mostly used by SFTPDir.read to create new Attribute objects, but if you find yourself in posession of a raw pointer, feel free to wrap it with this.

####raw
-\> \*mut sftp\_attributes\_struct

Unwraps the underlying libssh pointer, in case you want to ask it for any other attributes I guess.

####name
-\> &str

Returns a string slice of the filename of the remote object this points to. Does not include a whole filepath.

####longname
-\> &str

Returns the entire listing of the file as it would appear if `ls -l` were used locally. Important to note that this does *NOT* return the absolute filepath.

####owner
-\> &str

Returns a string slice describing the username that owns the permissions to the remote file.

#Future Work
---
This library has all of the library functions mapped, but could still use a lot of work implementing new and better helper functions within the object oriented framework. Please contact gin-and-miskatonic on github or fork the repository and submit a pull request if you would like to contribute! You can also contact me or submit an issue if there is a particular feature you would like to see added and I will look into it.
