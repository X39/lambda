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

The quit message is either sent when lambda decides to kill a process due to lack of
usage or after the [version package](#0--version-package) was read and the server is
not supporting the protocol version desired by the client.

The byte can be one of the following values:

| sender | value | purpose           | description                                                                                           |
|--------|-------|-------------------|-------------------------------------------------------------------------------------------------------|
| server | 0x00  | terminate-request | The server requested that the client closes. The client is given 1s to quit unless more is requested. |
| client | 0x01  | additional-1s     | Request one additional second of time for exiting the process. Works up to 60s.                       |
| client | 0x02  | additional-2s     | Request two additional seconds of time for exiting the process. Works up to 60s.                      |
| client | 0x03  | additional-3s     | Request three additional seconds of time for exiting the process. Works up to 60s.                    |
| ...    | ...   | ...               | ...                                                                                                   |
| client | 0x39  | additional-57s    | Request 57 additional seconds of time for exiting the process. Works up to 60s.                       |
| client | 0x3A  | additional-58s    | Request 58 additional seconds of time for exiting the process. Works up to 60s.                       |
| client | 0x3B  | additional-59s    | Request 59 additional seconds of time for exiting the process. Works up to 60s.                       |

##### Client Receives

The client must immediately start shutdown procedure.
If it has not quit in a finite amount of time, the server will force-kill the process.

##### Server Receives

The client is given additional seconds up to 60.

-----

#### 2: Capabilities Package

Send by lambda to receive the capabilities of a client.

The package has no payload.

##### Client Receives

The client must respond with a [capabilities-response package](#3--capabilities-response-package).

##### Server Receives

Not Applicable

-----

#### 3: Capabilities-Response Package

Send by the client if a [capabilities package](#2--capabilities-package) is received.

The bytes are read as follows (Indexes are not zero based and always inclusive):

| from |  to | purpose         | description                                     |
|-----:|----:|-----------------|-------------------------------------------------|
|    1 |   4 | functions-count | The number of functions provided by the client. |

##### Client Receives

Not Applicable

##### Server Receives

The server will ask for every individual function sending a
[function-capabilities package](#4--function-capabilities-package) for every
function reported.

-----

#### 4: Function-Capabilities Package

Send by the server to receive individual function information of the client.

The bytes are read as follows (Indexes are not zero based and always inclusive):

| from |  to | purpose            | description                                          |
|-----:|----:|--------------------|------------------------------------------------------|
|    1 |   4 | function-requested | The function that should be send back to the server. |

##### Client Receives

The client must respond with a
[function-capabilities-response package](#5--function-capabilities-response-package).

##### Server Receives

Not Applicable

-----

#### 5: Function-Capabilities-Response Package

Send by the client once a
[function-capabilities package](#4--function-capabilities-package) is received.

The bytes are read as follows (Indexes are not zero based and always inclusive):

| from |  to | purpose            | description                                                                                                                         |
|-----:|----:|--------------------|-------------------------------------------------------------------------------------------------------------------------------------|
|    1 |   4 | function-index     | The function index.                                                                                                                 |
|    5 |   6 | name-length        | The length of the function name. Even tho this allows 2^16 characters, the maximum length is capped to 10000.                       |
|    7 |   7 | arguments-required | The number of arguments expected for this function.                                                                                 |
|    8 |   8 | arguments-count    | The number of arguments expected plus optional for this function. Optional arguments must occur in the end and are not transmitted. |
|    9 |   9 | results-count      | The number of results produced by this function.                                                                                    |
|   10 |   * | function-name      | The function name. The "to" field is as long as "name-length" was provided.                                                         |

##### Client Receives
Not Applicable

##### Server Receives
No response is returned. Server is supposed to store the information and only
send valid requests to the client. The client may expect that the server
is always sending valid requests for the [call package](#6--call-package)

-----

#### 6: Call Package

Send by lambda to start a function. The package contains information about the function
to be called and the expected number of values.

The bytes are read as follows (Indexes are not zero based and always inclusive):

| from |  to | purpose         | description                        |
|-----:|----:|-----------------|------------------------------------|
|    1 |   4 | function-index  | The function to be called.         |
|    5 |   5 | arguments-count | The number of arguments available. |
|    6 |   9 | request-id      | The id for the call request.       |

##### Client Receives
The client should start working on the function immediately and may receive the
arguments using the [value-request package](#7--value-request-package). Once the
functions results are available, a [call-response package](#9--call-response-package)
has to be sent. The resources of the result must be held until the server sends a
[close-call package](#10--close-call-package).

##### Server Receives
Not Applicable

-----

#### 7: Value-Request Package

Requests a value from the given slot.

#### 8: GetValueResponse

Sends the value of a given slot.

#### 9: Call-Response Package

Function call has completed.

#### 10: Close-Call Package

Closes a function call, allowing to release the resources
