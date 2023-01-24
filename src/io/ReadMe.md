# Protocol v0.1.0 Documentation

This protocol defines a message-based system where every message is packed inside
of a frame, containing meta-information for every message. The goal here is to
allow lambda to transfer data between function-hosting worker processes

### Transfer Medium

The transfer medium for the protocol is the STDIO of the various processes.
This means that any means of logging cannot be done via console as lambda is
actively reading and writing to the stdio of the target program. All bytes are
transferred in *Little Endian* with floating point numbers being in the
*IEEE 754-2008* format.

### Protocol error handling

If a protocol error occurs, the ...

- **Server** will terminate process immediately
- **Client** should terminate itself immediately

### Frame-Format

#### Header

The header contains meta information of a frame, namely the message id contained
and the length of the message. It is always 8 bytes long.

| from |  to |    purpose     | description                                                                                                                                                                                                  |
|-----:|----:|:--------------:|:-------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------|
|    1 |   1 |       0        | Zero-Byte.                                                                                                                                                                                                   |
|    2 |   2 | frame-version  | Frame version. For this protocol version, this is always 0. If this is not 0, you may either switch to the newer protocol (given you have an implementation) or quit. The frame-version should never change. |
|    3 |   4 |   message-id   | Denotes the message contained in this frame.                                                                                                                                                                 |
|    5 |   8 | message-length | The length of the message.                                                                                                                                                                                   |

#### Body

The length of the *body* is depending on the message id received in the *header*.
See [messages](#messages) for more info.

### Messages

#### 0: Version Message

The version message is the first message send by lambda and consists of
20 (4 + 4 + 4 + 4 + 4) bytes.
The bytes are read as follows (Indexes are not zero based and always inclusive):

| from |  to | purpose  | description                                          |
|-----:|----:|:--------:|:-----------------------------------------------------|
|    1 |   4 |  major   | The major version of the server/client               |
|    5 |   8 |  minor   | The minor version of the server/client               |
|    9 |  12 |  build   | The build version of the server/client               |
|   13 |  16 | revision | The revision version of the server/client            |
|   17 |  20 | protocol | The preferred protocol version for the server/client |

##### Client Receives

The client must immediately respond to this message with its own version information.
The proceeding protocol version is the one the client provided.

##### Server Receives

Once the server received the client message, the server will attempt to find a
matching protocol. If no matching protocol can be located, the client process will be
asked to quit via the [quit message](#1--quit-message).

#### 1: Quit Message

The quit message is either sent when lambda decides to kill a process due to lack of
usage or after the [version message](#0--version-message) was read and the server is
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

The client must immediately start shutdown procedure or ask for additional time by sending
this [quit message](#1--quit-message) to the server with the corresponding time.
If it has not quit in a finite amount of time, the server will force-kill the process.

##### Server Receives

The client is given additional seconds up to 60.

-----

#### 2: Capabilities-Request Message

Send by lambda to receive the capabilities of a client.

The message has no payload.

##### Client Receives

The client must respond with a [capabilities-response message](#3--capabilities-response-message).

##### Server Receives

Not Applicable

-----

#### 3: Capabilities-Response Message

Send by the client if a [capabilities message](#2--capabilities-request-message) is received.

The bytes are read as follows (Indexes are not zero based and always inclusive):

| from |  to | purpose         | description                                     |
|-----:|----:|-----------------|-------------------------------------------------|
|    1 |   4 | functions-count | The number of functions provided by the client. |

##### Client Receives

Not Applicable

##### Server Receives

The server will ask for every individual function sending a
[function-capabilities message](#4--function-capabilities-message) for every
function reported.

-----

#### 4: Function-Capabilities Message

Send by the server to receive individual function information of the client.

The bytes are read as follows (Indexes are not zero based and always inclusive):

| from |  to | purpose            | description                                          |
|-----:|----:|--------------------|------------------------------------------------------|
|    1 |   4 | function-requested | The function that should be send back to the server. |

##### Client Receives

The client must respond with a
[function-capabilities-response message](#5--function-capabilities-response-message).

##### Server Receives

Not Applicable

-----

#### 5: Function-Capabilities-Response Message

Send by the client once a
[function-capabilities message](#4--function-capabilities-message) is received.
The values for "arguments-required" and "arguments-count" can be received once a
[call message](#6--call-message) initiated a function execution by using the
[value-request message](#7--value-request-message).

The bytes are read as follows (Indexes are not zero based and always inclusive):

| from |  to | purpose            | description                                                                                                                                                                |
|-----:|----:|--------------------|----------------------------------------------------------------------------------------------------------------------------------------------------------------------------|
|    1 |   4 | function-index     | The function index.                                                                                                                                                        |
|    5 |   6 | name-length        | The length of the function name. Even tho this allows 2^16 characters, the maximum length is capped to 10000.                                                              |
|    7 |   7 | arguments-required | The number of arguments expected for this function.                                                                                                                        |
|    8 |   8 | arguments-count    | The number of arguments expected plus optional for this function. Optional arguments are always positioned in the end and are not transmitted unless a value is available. |
|    9 |   9 | results-count      | The number of results produced by this function.                                                                                                                           |
|   10 |   * | function-name      | The function name. The "to" field is as long as "name-length" was provided.                                                                                                |

##### Client Receives

Not Applicable

##### Server Receives

No response is returned. Server is supposed to store the information and only
send valid requests to the client. The client may expect that the server
is always sending valid requests for the [call message](#6--call-message)

-----

#### 6: Call Message

Send by lambda to start a function. The message contains information about the function
to be called and the expected number of values.

The bytes are read as follows (Indexes are not zero based and always inclusive):

| from |  to | purpose         | description                                                                                                                     |
|-----:|----:|-----------------|---------------------------------------------------------------------------------------------------------------------------------|
|    1 |   4 | function-index  | The function to be called.                                                                                                      |
|    5 |   5 | arguments-count | The number of arguments available.                                                                                              |
|    6 |   9 | call-request-id | A number identifying the call. It is used in other conversations to refer to the function execution started with this message.. |

##### Client Receives

The client should start working on the function immediately and may receive the
arguments using the [value-request message](#7--value-request-message). Once the
functions results are available, a [call-response message](#9--call-response-message)
has to be sent. The resources of the result must be held until the server sends a
[close-call message](#10--close-call-message).

##### Server Receives

Not Applicable

-----

#### 7: Value-Request Message

Send by one of the parties to receive a value from the other party.

The bytes are read as follows (Indexes are not zero based and always inclusive):

| from |  to | purpose         | description                         |
|-----:|----:|-----------------|-------------------------------------|
|    1 |   4 | call-request-id | The request which holds the values. |
|    5 |   5 | argument-index  | The index of the value.             |

##### Client Receives

Immediately (the next message send) the client must respond using the
[value-response message](#8--value-response-message) with the value held
in the given call-request-id at argument-index.

##### Server Receives

Immediately (the next message send) the server must respond using the
[value-response message](#8--value-response-message) with the value held
in the given call-request-id at argument-index.

-----

#### 8: Value-Response Message

Send by one of the parties the moment it received the
[value-request message](#7--value-request-message)

The bytes are read as follows (Indexes are not zero based and always inclusive):

| from |  to | purpose     | description                                    |
|-----:|----:|-------------|------------------------------------------------|
|    1 |   4 | json-length | The length of the "json" payload.              |
|    5 |   * | json        | The json value with a length of "json-length". |

##### Client Receives

Client continues processing.

##### Server Receives

Server continues processing

-----

#### 9: Call-Response Message

Send once a call started with the [call message](#6--call-message) has completed or errored.
It is important that the client holds the data for lambda to receive until a
[close-call message](#10--close-call-message) is received.

The bytes are read as follows (Indexes are not zero based and always inclusive):

| from |  to | purpose         | description                                                                                                                                           |
|-----:|----:|-----------------|-------------------------------------------------------------------------------------------------------------------------------------------------------|
|    1 |   4 | call-request-id | The id for the call request.                                                                                                                          |
|    5 |   5 | success         | Boolean value (0 = false; 1 = true) indicating whether the call ended successfully or not. This is mostly for languages supporting exceptions.        |
|    6 |   6 | results-count   | The amount of results available. If "success" is false, this must be 1 as a single exception result is enforced on client-failures at protocol layer. |

##### Client Receives

Not Applicable

##### Server Receives

The server will start receiving the results using the
[value-request message](#7--value-request-message) and close the call using the
[close-call message](#10--close-call-message) after it is done.

-----

#### 10: Close-Call Message

Closes a function call, allowing to release the resources

The bytes are read as follows (Indexes are not zero based and always inclusive):

| from |  to | purpose         | description                                                                                                                                    |
|-----:|----:|-----------------|------------------------------------------------------------------------------------------------------------------------------------------------|
|    1 |   4 | call-request-id | The id for the call request.                                                                                                                   |
|    5 |   5 | success         | Boolean value (0 = false; 1 = true) indicating whether the call ended successfully or not. This is mostly for languages supporting exceptions. |
|    6 |   6 | results-count   | The amount of results available. If "success" is false, this must be 1.                                                                        |

##### Client Receives

The client should release any resources left over for the provided "call-request-id"

##### Server Receives

Not Applicable

