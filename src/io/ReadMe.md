# Protocol Documentation

### Transfer Medium

The transfer medium for the protocol is the STDIO of the various processes.
This means that any means of logging cannot be done via console as lambda is
actively reading and writing to the stdio of the target program. All bytes are
transferred in *Little Endian* with floating point numbers being in the
*IEEE 754-2008* format.

### Format

#### Header

The header consists of usually 1 byte, containing the package id.
If the **last bit** of the package id is `1`, another byte is
required to parse the package id. In that case, another byte is emitted
for the package id. The actual package id then gets calculated by shifting
following bits by 1.

Samples (bytes received left to right):

* `0b0000_0000` is the package id `0`
* `0b0111_1111` is the package id `127`
* `0b1000_0000 0b0000_0001` is the package id `128`
* `0b1111_1111 0b0000_0001` is the package id `255`
* `0b1000_0000 0b0000_0010` is the package id `256`
* `0b1111_1111 0b0111_1111` is the package id `16383`

#### Body

The length of the *body* is depending on the package id received in the *header*.
See [packages](#Packages) for more info.

### Packages

#### 0: Version Package

The version package is the first package send by lambda and consists of
16 (4 + 4 + 4 + 4 + 4) bytes.
The bytes are read as follows (Indexes are not zero based and always inclusive):

| from |  to | purpose  | description                                          |
|-----:|----:|:--------:|:-----------------------------------------------------|
|    1 |   4 |  major   | The major version of the server/client               |
|    5 |   8 |  minor   | The minor version of the server/client               |
|    9 |  12 |  build   | The build version of the server/client               |
|   13 |  16 | revision | The revision version of the server/client            |
|   17 |  20 | protocol | The preferred protocol version for the server/client |

##### Client Receives

The client must immediately respond to this package with its own version information.
The proceeding protocol version is the one the client provided.

##### Server Receives

Once the server received the client package, the server will attempt to find a
matching protocol. If no matching protocol can be located, the client process will be
asked to quit via the [quit package](#1--quit-package).

#### 1: Quit Package

The protocol confirmation is sent after the [version package](#0--version-package)
was processed by the server and consists of 1 byte.

The byte can be one of the following values:

| sender | value | purpose           | description                                                                                           |
|--------|-------|-------------------|-------------------------------------------------------------------------------------------------------|
| server | 0x00  | terminate request | The server requested that the client closes. The client is given 1s to quit unless more is requested. |
| client | 0x01  | additional-1s     | Request one additional second of time for exiting the process. Works up to 60s.                       |
| client | 0x02  | additional-2s     | Request two additional seconds of time for exiting the process. Works up to 60s.                      |
| client | 0x03  | additional-3s     | Request three additional seconds of time for exiting the process. Works up to 60s.                    |
| ...    | ...   | ...               | ...                                                                                                   |
| client | 0x39  | additional-57s    | Request 57 additional seconds of time for exiting the process. Works up to 60s.                       |
| client | 0x3A  | additional-58s    | Request 58 additional seconds of time for exiting the process. Works up to 60s.                       |
| client | 0x3B  | additional-59s    | Request 59 additional seconds of time for exiting the process. Works up to 60s.                       |

#### Client Receives
The client must immediately start shutdown procedure.
If it has not quit in a finite amount of time, the server will force-kill the process.

#### Server Receives
The client is given additional seconds up to 60.

#### 2: CallRequest
Function call is requested.
#### 3: CallResponse
Function call has completed.
#### 4: CallClosed
Closes a function call, allowing to release the resources
#### 5: GetValueRequest
Requests a value from the given slot.
#### 6: GetValueResponse
Sends the value of a given slot.