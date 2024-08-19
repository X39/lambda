pub mod protocol_v1 {
    use std::io::{ErrorKind, Read, Write};
    use crate::io::FunctionHost;
    use crate::io::protocol_v1::protocol_v1::data::{MessageKind, Message, VersionMessage, MessageEnum};
    use std::{
        sync::mpsc::channel,
        thread,
        time::Duration,
    };
    use std::sync::mpsc::{Receiver, Sender};
    use queues::{IsQueue, Queue, queue};


    mod data {
        use std::io::{Error, Read, Write};

        #[derive(PartialEq, Clone)]
        #[repr(u16)]
        pub enum MessageEnum {
            Version(VersionMessage),
            Quit(QuitMessage),
            CapabilitiesRequest(CapabilitiesRequestMessage),
            CapabilitiesResponse(CapabilitiesResponseMessage),
            FunctionCapabilitiesRequest(FunctionCapabilitiesRequestMessage),
            FunctionCapabilitiesResponse(FunctionCapabilitiesResponseMessage),
            Call(CallMessage),
            ArgumentRequest(ArgumentRequestMessage),
            ArgumentResponse(ArgumentResponseMessage),
            CallCompleted(CallCompletedMessage),
            ResultRequest(ResultRequestMessage),
            ResultResponse(ResultResponseMessage),
            CloseCall(CloseCallMessage),
        }

        #[derive(PartialEq, Clone)]
        #[repr(u16)]
        pub enum MessageKind {
            Version = 0,
            Quit = 1,
            CapabilitiesRequest = 2,
            CapabilitiesResponse = 3,
            FunctionCapabilitiesRequest = 4,
            FunctionCapabilitiesResponse = 5,
            Call = 6,
            ArgumentRequest = 7,
            ArgumentResponse = 8,
            CallCompleted = 9,
            ResultRequest = 10,
            ResultResponse = 11,
            CloseCall = 12,
        }

        impl Into<u16> for MessageKind {
            fn into(self) -> u16 {
                match self {
                    MessageKind::Version => 0,
                    MessageKind::Quit => 1,
                    MessageKind::CapabilitiesRequest => 2,
                    MessageKind::CapabilitiesResponse => 3,
                    MessageKind::FunctionCapabilitiesRequest => 4,
                    MessageKind::FunctionCapabilitiesResponse => 5,
                    MessageKind::Call => 6,
                    MessageKind::ArgumentRequest => 7,
                    MessageKind::ArgumentResponse => 8,
                    MessageKind::CallCompleted => 9,
                    MessageKind::ResultRequest => 10,
                    MessageKind::ResultResponse => 11,
                    MessageKind::CloseCall => 12,
                }
            }
        }

        pub trait Message {
            const KIND: MessageKind;
            fn new() -> Self;
            fn length(&self) -> usize;
            fn serialize(&self, writer: &mut dyn Write) -> Result<(), Error>;
            fn deserialize(&mut self, reader: &mut dyn Read) -> Result<(), Error>;
        }

        #[derive(PartialEq, Clone)]
        pub struct VersionMessage {
            pub major: u32,
            pub minor: u32,
            pub build: u32,
            pub revision: u32,
            pub protocol: u32,
        }

        impl Message for VersionMessage {
            const KIND: MessageKind = MessageKind::Version;

            fn new() -> Self {
                VersionMessage {
                    major: 0,
                    minor: 0,
                    build: 0,
                    revision: 0,
                    protocol: 0,
                }
            }

            fn length(&self) -> usize {
                20
            }

            fn serialize(&self, writer: &mut dyn Write) -> Result<(), Error> {
                crate::io::ext::write_u32(writer, self.major)?;
                crate::io::ext::write_u32(writer, self.minor)?;
                crate::io::ext::write_u32(writer, self.build)?;
                crate::io::ext::write_u32(writer, self.revision)?;
                crate::io::ext::write_u32(writer, self.protocol)?;
                Ok(())
            }

            fn deserialize(&mut self, reader: &mut dyn Read) -> Result<(), Error> {
                self.major = crate::io::ext::read_u32(reader)?;
                self.minor = crate::io::ext::read_u32(reader)?;
                self.build = crate::io::ext::read_u32(reader)?;
                self.revision = crate::io::ext::read_u32(reader)?;
                self.protocol = crate::io::ext::read_u32(reader)?;
                Ok(())
            }
        }

        #[derive(PartialEq, Clone)]
        #[repr(u8)]
        pub enum QuitMessage {
            Terminate = 0,
            Additional1s = 1,
            Additional2s = 2,
            Additional3s = 3,
            Additional4s = 4,
            Additional5s = 5,
            Additional6s = 6,
            Additional7s = 7,
            Additional8s = 8,
            Additional9s = 9,
            Additional10s = 10,
            Additional11s = 11,
            Additional12s = 12,
            Additional13s = 13,
            Additional14s = 14,
            Additional15s = 15,
            Additional16s = 16,
            Additional17s = 17,
            Additional18s = 18,
            Additional19s = 19,
            Additional20s = 20,
            Additional21s = 21,
            Additional22s = 22,
            Additional23s = 23,
            Additional24s = 24,
            Additional25s = 25,
            Additional26s = 26,
            Additional27s = 27,
            Additional28s = 28,
            Additional29s = 29,
            Additional30s = 30,
            Additional31s = 31,
            Additional32s = 32,
            Additional33s = 33,
            Additional34s = 34,
            Additional35s = 35,
            Additional36s = 36,
            Additional37s = 37,
            Additional38s = 38,
            Additional39s = 39,
            Additional40s = 40,
            Additional41s = 41,
            Additional42s = 42,
            Additional43s = 43,
            Additional44s = 44,
            Additional45s = 45,
            Additional46s = 46,
            Additional47s = 47,
            Additional48s = 48,
            Additional49s = 49,
            Additional50s = 50,
            Additional51s = 51,
            Additional52s = 52,
            Additional53s = 53,
            Additional54s = 54,
            Additional55s = 55,
            Additional56s = 56,
            Additional57s = 57,
            Additional58s = 58,
            Additional59s = 59,
        }

        impl Message for QuitMessage {
            const KIND: MessageKind = MessageKind::Quit;

            fn new() -> Self {
                QuitMessage::Terminate
            }

            fn length(&self) -> usize {
                20
            }

            fn serialize(&self, writer: &mut dyn Write) -> Result<(), Error> {
                let val = match self {
                    QuitMessage::Terminate => 0,
                    QuitMessage::Additional1s => 1,
                    QuitMessage::Additional2s => 2,
                    QuitMessage::Additional3s => 3,
                    QuitMessage::Additional4s => 4,
                    QuitMessage::Additional5s => 5,
                    QuitMessage::Additional6s => 6,
                    QuitMessage::Additional7s => 7,
                    QuitMessage::Additional8s => 8,
                    QuitMessage::Additional9s => 9,
                    QuitMessage::Additional10s => 10,
                    QuitMessage::Additional11s => 11,
                    QuitMessage::Additional12s => 12,
                    QuitMessage::Additional13s => 13,
                    QuitMessage::Additional14s => 14,
                    QuitMessage::Additional15s => 15,
                    QuitMessage::Additional16s => 16,
                    QuitMessage::Additional17s => 17,
                    QuitMessage::Additional18s => 18,
                    QuitMessage::Additional19s => 19,
                    QuitMessage::Additional20s => 20,
                    QuitMessage::Additional21s => 21,
                    QuitMessage::Additional22s => 22,
                    QuitMessage::Additional23s => 23,
                    QuitMessage::Additional24s => 24,
                    QuitMessage::Additional25s => 25,
                    QuitMessage::Additional26s => 26,
                    QuitMessage::Additional27s => 27,
                    QuitMessage::Additional28s => 28,
                    QuitMessage::Additional29s => 29,
                    QuitMessage::Additional30s => 30,
                    QuitMessage::Additional31s => 31,
                    QuitMessage::Additional32s => 32,
                    QuitMessage::Additional33s => 33,
                    QuitMessage::Additional34s => 34,
                    QuitMessage::Additional35s => 35,
                    QuitMessage::Additional36s => 36,
                    QuitMessage::Additional37s => 37,
                    QuitMessage::Additional38s => 38,
                    QuitMessage::Additional39s => 39,
                    QuitMessage::Additional40s => 40,
                    QuitMessage::Additional41s => 41,
                    QuitMessage::Additional42s => 42,
                    QuitMessage::Additional43s => 43,
                    QuitMessage::Additional44s => 44,
                    QuitMessage::Additional45s => 45,
                    QuitMessage::Additional46s => 46,
                    QuitMessage::Additional47s => 47,
                    QuitMessage::Additional48s => 48,
                    QuitMessage::Additional49s => 49,
                    QuitMessage::Additional50s => 50,
                    QuitMessage::Additional51s => 51,
                    QuitMessage::Additional52s => 52,
                    QuitMessage::Additional53s => 53,
                    QuitMessage::Additional54s => 54,
                    QuitMessage::Additional55s => 55,
                    QuitMessage::Additional56s => 56,
                    QuitMessage::Additional57s => 57,
                    QuitMessage::Additional58s => 58,
                    QuitMessage::Additional59s => 59,
                };
                crate::io::ext::write_u8(writer, val)?;
                Ok(())
            }

            fn deserialize(&mut self, reader: &mut dyn Read) -> Result<(), Error> {
                let val = crate::io::ext::read_u8(reader)?;
                *self = match val {
                    0 => QuitMessage::Terminate,
                    1 => QuitMessage::Additional1s,
                    2 => QuitMessage::Additional2s,
                    3 => QuitMessage::Additional3s,
                    4 => QuitMessage::Additional4s,
                    5 => QuitMessage::Additional5s,
                    6 => QuitMessage::Additional6s,
                    7 => QuitMessage::Additional7s,
                    8 => QuitMessage::Additional8s,
                    9 => QuitMessage::Additional9s,
                    10 => QuitMessage::Additional10s,
                    11 => QuitMessage::Additional11s,
                    12 => QuitMessage::Additional12s,
                    13 => QuitMessage::Additional13s,
                    14 => QuitMessage::Additional14s,
                    15 => QuitMessage::Additional15s,
                    16 => QuitMessage::Additional16s,
                    17 => QuitMessage::Additional17s,
                    18 => QuitMessage::Additional18s,
                    19 => QuitMessage::Additional19s,
                    20 => QuitMessage::Additional20s,
                    21 => QuitMessage::Additional21s,
                    22 => QuitMessage::Additional22s,
                    23 => QuitMessage::Additional23s,
                    24 => QuitMessage::Additional24s,
                    25 => QuitMessage::Additional25s,
                    26 => QuitMessage::Additional26s,
                    27 => QuitMessage::Additional27s,
                    28 => QuitMessage::Additional28s,
                    29 => QuitMessage::Additional29s,
                    30 => QuitMessage::Additional30s,
                    31 => QuitMessage::Additional31s,
                    32 => QuitMessage::Additional32s,
                    33 => QuitMessage::Additional33s,
                    34 => QuitMessage::Additional34s,
                    35 => QuitMessage::Additional35s,
                    36 => QuitMessage::Additional36s,
                    37 => QuitMessage::Additional37s,
                    38 => QuitMessage::Additional38s,
                    39 => QuitMessage::Additional39s,
                    40 => QuitMessage::Additional40s,
                    41 => QuitMessage::Additional41s,
                    42 => QuitMessage::Additional42s,
                    43 => QuitMessage::Additional43s,
                    44 => QuitMessage::Additional44s,
                    45 => QuitMessage::Additional45s,
                    46 => QuitMessage::Additional46s,
                    47 => QuitMessage::Additional47s,
                    48 => QuitMessage::Additional48s,
                    49 => QuitMessage::Additional49s,
                    50 => QuitMessage::Additional50s,
                    51 => QuitMessage::Additional51s,
                    52 => QuitMessage::Additional52s,
                    53 => QuitMessage::Additional53s,
                    54 => QuitMessage::Additional54s,
                    55 => QuitMessage::Additional55s,
                    56 => QuitMessage::Additional56s,
                    57 => QuitMessage::Additional57s,
                    58 => QuitMessage::Additional58s,
                    59 => QuitMessage::Additional59s,
                    _ => return Err(Error::new(std::io::ErrorKind::InvalidData, "Quit message received is out of the valid value range.")),
                };
                Ok(())
            }
        }

        #[derive(PartialEq, Clone)]
        pub struct CapabilitiesRequestMessage {}

        impl Message for CapabilitiesRequestMessage {
            const KIND: MessageKind = MessageKind::CapabilitiesRequest;

            fn new() -> Self {
                CapabilitiesRequestMessage {}
            }

            fn length(&self) -> usize {
                0
            }

            fn serialize(&self, _writer: &mut dyn Write) -> Result<(), Error> {
                Ok(())
            }

            fn deserialize(&mut self, _reader: &mut dyn Read) -> Result<(), Error> {
                Ok(())
            }
        }

        #[derive(PartialEq, Clone)]
        pub struct CapabilitiesResponseMessage {
            pub functions_count: u32,
        }

        impl Message for CapabilitiesResponseMessage {
            const KIND: MessageKind = MessageKind::CapabilitiesResponse;

            fn new() -> Self {
                CapabilitiesResponseMessage {
                    functions_count: 0
                }
            }

            fn length(&self) -> usize {
                4
            }

            fn serialize(&self, writer: &mut dyn Write) -> Result<(), Error> {
                crate::io::ext::write_u32(writer, self.functions_count)?;
                Ok(())
            }

            fn deserialize(&mut self, reader: &mut dyn Read) -> Result<(), Error> {
                self.functions_count = crate::io::ext::read_u32(reader)?;
                Ok(())
            }
        }

        #[derive(PartialEq, Clone)]
        pub struct FunctionCapabilitiesRequestMessage {
            pub function_requested: u32,
        }

        impl Message for FunctionCapabilitiesRequestMessage {
            const KIND: MessageKind = MessageKind::FunctionCapabilitiesRequest;

            fn new() -> Self {
                FunctionCapabilitiesRequestMessage {
                    function_requested: 0
                }
            }

            fn length(&self) -> usize {
                4
            }

            fn serialize(&self, writer: &mut dyn Write) -> Result<(), Error> {
                crate::io::ext::write_u32(writer, self.function_requested)?;
                Ok(())
            }

            fn deserialize(&mut self, reader: &mut dyn Read) -> Result<(), Error> {
                self.function_requested = crate::io::ext::read_u32(reader)?;
                Ok(())
            }
        }

        #[derive(PartialEq, Clone)]
        pub struct FunctionCapabilitiesResponseMessage {
            pub function_index: u32,
            pub arguments_required: u8,
            pub arguments_count: u8,
            pub results_count: u8,
            pub function_name: String,
        }

        impl Message for FunctionCapabilitiesResponseMessage {
            const KIND: MessageKind = MessageKind::FunctionCapabilitiesRequest;

            fn new() -> Self {
                FunctionCapabilitiesResponseMessage {
                    function_index: 0,
                    arguments_required: 0,
                    arguments_count: 0,
                    results_count: 0,
                    function_name: "".to_string(),
                }
            }

            fn length(&self) -> usize {
                9 + self.function_name.len()
            }

            fn serialize(&self, writer: &mut dyn Write) -> Result<(), Error> {
                let name_length = self.function_name.len() as u16;
                crate::io::ext::write_u32(writer, self.function_index)?;
                crate::io::ext::write_u8(writer, self.arguments_required)?;
                crate::io::ext::write_u8(writer, self.arguments_count)?;
                crate::io::ext::write_u8(writer, self.results_count)?;
                crate::io::ext::write_u16(writer, name_length)?;
                crate::io::ext::write_string(writer, &self.function_name, 0, name_length as usize)?;
                Ok(())
            }

            fn deserialize(&mut self, reader: &mut dyn Read) -> Result<(), Error> {
                self.function_index = crate::io::ext::read_u32(reader)?;
                self.arguments_required = crate::io::ext::read_u8(reader)?;
                self.arguments_count = crate::io::ext::read_u8(reader)?;
                self.results_count = crate::io::ext::read_u8(reader)?;
                let name_length = crate::io::ext::read_u16(reader)?;
                self.function_name = crate::io::ext::read_string(reader, name_length as usize)?;
                Ok(())
            }
        }

        #[derive(PartialEq, Clone)]
        pub struct CallMessage {
            pub function_index: u32,
            pub arguments_count: u8,
            pub call_request_id: u32,
        }

        impl Message for CallMessage {
            const KIND: MessageKind = MessageKind::Call;

            fn new() -> Self {
                CallMessage {
                    function_index: 0,
                    arguments_count: 0,
                    call_request_id: 0,
                }
            }

            fn length(&self) -> usize {
                9
            }

            fn serialize(&self, writer: &mut dyn Write) -> Result<(), Error> {
                crate::io::ext::write_u32(writer, self.function_index)?;
                crate::io::ext::write_u8(writer, self.arguments_count)?;
                crate::io::ext::write_u32(writer, self.call_request_id)?;
                Ok(())
            }

            fn deserialize(&mut self, reader: &mut dyn Read) -> Result<(), Error> {
                self.function_index = crate::io::ext::read_u32(reader)?;
                self.arguments_count = crate::io::ext::read_u8(reader)?;
                self.call_request_id = crate::io::ext::read_u32(reader)?;
                Ok(())
            }
        }

        #[derive(PartialEq, Clone)]
        pub struct ArgumentRequestMessage {
            pub call_request_id: u32,
            pub argument_index: u8,
        }

        impl Message for ArgumentRequestMessage {
            const KIND: MessageKind = MessageKind::ArgumentRequest;

            fn new() -> Self {
                ArgumentRequestMessage {
                    call_request_id: 0,
                    argument_index: 0,
                }
            }

            fn length(&self) -> usize {
                5
            }

            fn serialize(&self, writer: &mut dyn Write) -> Result<(), Error> {
                crate::io::ext::write_u32(writer, self.call_request_id)?;
                crate::io::ext::write_u8(writer, self.argument_index)?;
                Ok(())
            }

            fn deserialize(&mut self, reader: &mut dyn Read) -> Result<(), Error> {
                self.call_request_id = crate::io::ext::read_u32(reader)?;
                self.argument_index = crate::io::ext::read_u8(reader)?;
                Ok(())
            }
        }

        #[derive(PartialEq, Clone)]
        pub struct ArgumentResponseMessage {
            pub json: String,
        }

        impl Message for ArgumentResponseMessage {
            const KIND: MessageKind = MessageKind::ArgumentResponse;

            fn new() -> Self {
                ArgumentResponseMessage {
                    json: "".to_string(),
                }
            }

            fn length(&self) -> usize {
                4 + self.json.len()
            }

            fn serialize(&self, writer: &mut dyn Write) -> Result<(), Error> {
                let json_length = self.json.len() as u32;
                crate::io::ext::write_u32(writer, json_length)?;
                crate::io::ext::write_string(writer, &self.json, 0, json_length as usize)?;
                Ok(())
            }

            fn deserialize(&mut self, reader: &mut dyn Read) -> Result<(), Error> {
                let json_length = crate::io::ext::read_u32(reader)?;
                self.json = crate::io::ext::read_string(reader, json_length as usize)?;
                Ok(())
            }
        }

        #[derive(PartialEq, Clone)]
        pub struct CallCompletedMessage {
            pub call_request_id: u32,
            pub success: bool,
            pub results_count: u8,
        }

        impl Message for CallCompletedMessage {
            const KIND: MessageKind = MessageKind::CallCompleted;

            fn new() -> Self {
                CallCompletedMessage {
                    call_request_id: 0,
                    success: false,
                    results_count: 0,
                }
            }

            fn length(&self) -> usize {
                6
            }

            fn serialize(&self, writer: &mut dyn Write) -> Result<(), Error> {
                crate::io::ext::write_u32(writer, self.call_request_id)?;
                crate::io::ext::write_bool(writer, self.success)?;
                crate::io::ext::write_u8(writer, self.results_count)?;
                Ok(())
            }

            fn deserialize(&mut self, reader: &mut dyn Read) -> Result<(), Error> {
                self.call_request_id = crate::io::ext::read_u32(reader)?;
                self.success = crate::io::ext::read_bool(reader)?;
                self.results_count = crate::io::ext::read_u8(reader)?;
                Ok(())
            }
        }

        #[derive(PartialEq, Clone)]
        pub struct ResultRequestMessage {
            pub call_request_id: u32,
            pub result_index: u8,
        }

        impl Message for ResultRequestMessage {
            const KIND: MessageKind = MessageKind::ResultRequest;

            fn new() -> Self {
                ResultRequestMessage {
                    call_request_id: 0,
                    result_index: 0,
                }
            }

            fn length(&self) -> usize {
                5
            }

            fn serialize(&self, writer: &mut dyn Write) -> Result<(), Error> {
                crate::io::ext::write_u32(writer, self.call_request_id)?;
                crate::io::ext::write_u8(writer, self.result_index)?;
                Ok(())
            }

            fn deserialize(&mut self, reader: &mut dyn Read) -> Result<(), Error> {
                self.call_request_id = crate::io::ext::read_u32(reader)?;
                self.result_index = crate::io::ext::read_u8(reader)?;
                Ok(())
            }
        }

        #[derive(PartialEq, Clone)]
        pub struct ResultResponseMessage {
            pub json: String,
        }

        impl Message for ResultResponseMessage {
            const KIND: MessageKind = MessageKind::ResultResponse;

            fn new() -> Self {
                ResultResponseMessage {
                    json: "".to_string(),
                }
            }

            fn length(&self) -> usize {
                4 + self.json.len()
            }

            fn serialize(&self, writer: &mut dyn Write) -> Result<(), Error> {
                let json_length = self.json.len() as u32;
                crate::io::ext::write_u32(writer, json_length)?;
                crate::io::ext::write_string(writer, &self.json, 0, json_length as usize)?;
                Ok(())
            }

            fn deserialize(&mut self, reader: &mut dyn Read) -> Result<(), Error> {
                let json_length = crate::io::ext::read_u32(reader)?;
                self.json = crate::io::ext::read_string(reader, json_length as usize)?;
                Ok(())
            }
        }

        #[derive(PartialEq, Clone)]
        pub struct CloseCallMessage {
            pub call_request_id: u32,
        }

        impl Message for CloseCallMessage {
            const KIND: MessageKind = MessageKind::CloseCall;

            fn new() -> Self {
                CloseCallMessage {
                    call_request_id: 0,
                }
            }

            fn length(&self) -> usize {
                4
            }

            fn serialize(&self, writer: &mut dyn Write) -> Result<(), Error> {
                crate::io::ext::write_u32(writer, self.call_request_id)?;
                Ok(())
            }

            fn deserialize(&mut self, reader: &mut dyn Read) -> Result<(), Error> {
                self.call_request_id = crate::io::ext::read_u32(reader)?;
                Ok(())
            }
        }
    }

    pub enum Error {
        ProtocolError(&'static str),
        IoError(std::io::Error),
    }

    impl From<std::io::Error> for Error {
        fn from(value: std::io::Error) -> Self {
            Error::IoError(value)
        }
    }

    impl From<&'static str> for Error {
        fn from(value: &'static str) -> Self {
            Error::ProtocolError(value)
        }
    }

    pub struct ProtocolHost<'a> {
        host: &'a mut dyn FunctionHost<'a>,
        client_version: VersionMessage,
        thread: Option<thread::JoinHandle<()>>,
        received_messages: Vec<data::MessageEnum>,
        sender: Sender<Result<data::MessageEnum, Error>>,
        receiver: Receiver<Result<data::MessageEnum, Error>>,
        thread_active: bool,
        pub messages: MessageVectors,
    }

    pub struct MessageVectors {
        version: Queue<data::VersionMessage>,
        quit: Queue<data::QuitMessage>,
        capabilities_request: Queue<data::CapabilitiesRequestMessage>,
        capabilities_response: Queue<data::CapabilitiesResponseMessage>,
        function_capabilities_request: Queue<data::FunctionCapabilitiesRequestMessage>,
        function_capabilities_response: Queue<data::FunctionCapabilitiesResponseMessage>,
        call: Queue<data::CallMessage>,
        argument_request: Queue<data::ArgumentRequestMessage>,
        argument_response: Queue<data::ArgumentResponseMessage>,
        call_completed: Queue<data::CallCompletedMessage>,
        result_request: Queue<data::ResultRequestMessage>,
        result_response: Queue<data::ResultResponseMessage>,
        close_call: Queue<data::CloseCallMessage>,
    }

    impl<'a> ProtocolHost<'a> {
        const FRAME_VERSION: u8 = 1;
        const PROTOCOL_VERSION: u32 = 1;
        fn write<MESSAGE>(&mut self, message: MESSAGE)
                          -> Result<(), std::io::Error>
            where MESSAGE: Message
        {
            let msg_length = message.length();
            if msg_length > u32::MAX as usize {
                return Err(std::io::Error::new(ErrorKind::InvalidData, "The message size exceeds the maximum size of u32"));
            }
            crate::io::ext::write_u8(self.host.writer(), 0)?;
            crate::io::ext::write_u8(self.host.writer(), ProtocolHost::FRAME_VERSION)?;
            crate::io::ext::write_u16(self.host.writer(), MESSAGE::KIND.into())?;
            crate::io::ext::write_u32(self.host.writer(), msg_length as u32)?;
            message.serialize(self.host.writer())?;
            Ok(())
        }
        fn read_header(&mut self) -> Result<(data::MessageKind, u32), std::io::Error>
        {
            let _ = crate::io::ext::read_u8(self.host.reader())?;
            let frame_version = crate::io::ext::read_u8(self.host.reader())?;
            if frame_version != ProtocolHost::FRAME_VERSION {
                return Err(std::io::Error::new(ErrorKind::InvalidData, "Unsupported frame version"));
            }
            let message_id = crate::io::ext::read_u16(self.host.reader())?;
            let message_length = crate::io::ext::read_u32(self.host.reader())?;
            let message = match message_id {
                0 => MessageKind::Version,
                1 => MessageKind::Quit,
                2 => MessageKind::CapabilitiesRequest,
                3 => MessageKind::CapabilitiesResponse,
                4 => MessageKind::FunctionCapabilitiesRequest,
                5 => MessageKind::FunctionCapabilitiesResponse,
                6 => MessageKind::Call,
                7 => MessageKind::ArgumentRequest,
                8 => MessageKind::ArgumentResponse,
                9 => MessageKind::CallCompleted,
                10 => MessageKind::ResultRequest,
                11 => MessageKind::ResultResponse,
                12 => MessageKind::CloseCall,
                _ => return Err(std::io::Error::new(ErrorKind::InvalidData, "Unknown message id")),
            };
            Ok((message, message_length))
        }
        fn read_full<MESSAGE>(&mut self) -> Result<MESSAGE, Error>
            where MESSAGE: Message {
            let (id, _) = self.read_header()?;
            self.read(id)
        }
        fn read<MESSAGE>(&mut self, id: MessageKind) -> Result<MESSAGE, Error>
            where MESSAGE: Message {
            if id != MESSAGE::KIND {
                return Err("Different message was expected at this point".into());
            }
            let mut message = MESSAGE::new();
            message.deserialize(self.host.reader())?;
            return Ok(message);
        }

        fn function_host_reader(&mut self) {}

        pub fn connect(function_host: &'a mut dyn FunctionHost) -> Result<ProtocolHost<'a>, Error> {
            let (sender, receiver) = channel::<Result<data::MessageEnum, Error>>();
            let mut protocol_host = ProtocolHost {
                client_version: VersionMessage::new(),
                host: function_host,
                received_messages: vec!(),
                thread: None,
                thread_active: false,
                sender,
                receiver,
                messages: MessageVectors {
                    version: queue!(),
                    quit: queue!(),
                    capabilities_request: queue!(),
                    capabilities_response: queue!(),
                    function_capabilities_request: queue!(),
                    function_capabilities_response: queue!(),
                    call: queue!(),
                    argument_request: queue!(),
                    argument_response: queue!(),
                    call_completed: queue!(),
                    result_request: queue!(),
                    result_response: queue!(),
                    close_call: queue!(),
                },
            };
            let thread = Some(thread::spawn(move || {
                protocol_host.thread_active = true;
                while protocol_host.thread_active {
                    let result = protocol_host.read_header();
                    if let Ok(header) = result {
                        let message: Result<data::MessageEnum, Error> = match header.0 {
                            MessageKind::Version => match protocol_host.read::<data::VersionMessage>(header.0) {
                                Ok(ok) => Ok(data::MessageEnum::Version(ok)),
                                Err(err) => Err(err)
                            },
                            MessageKind::Quit => match protocol_host.read::<data::QuitMessage>(header.0) {
                                Ok(ok) => Ok(data::MessageEnum::Quit(ok)),
                                Err(err) => Err(err)
                            },
                            MessageKind::CapabilitiesRequest => match protocol_host.read::<data::CapabilitiesRequestMessage>(header.0) {
                                Ok(ok) => Ok(data::MessageEnum::CapabilitiesRequest(ok)),
                                Err(err) => Err(err)
                            },
                            MessageKind::CapabilitiesResponse => match protocol_host.read::<data::CapabilitiesResponseMessage>(header.0) {
                                Ok(ok) => Ok(data::MessageEnum::CapabilitiesResponse(ok)),
                                Err(err) => Err(err)
                            },
                            MessageKind::FunctionCapabilitiesRequest => match protocol_host.read::<data::FunctionCapabilitiesRequestMessage>(header.0) {
                                Ok(ok) => Ok(data::MessageEnum::FunctionCapabilitiesRequest(ok)),
                                Err(err) => Err(err)
                            },
                            MessageKind::FunctionCapabilitiesResponse => match protocol_host.read::<data::FunctionCapabilitiesResponseMessage>(header.0) {
                                Ok(ok) => Ok(data::MessageEnum::FunctionCapabilitiesResponse(ok)),
                                Err(err) => Err(err)
                            },
                            MessageKind::Call => match protocol_host.read::<data::CallMessage>(header.0) {
                                Ok(ok) => Ok(data::MessageEnum::Call(ok)),
                                Err(err) => Err(err)
                            },
                            MessageKind::ArgumentRequest => match protocol_host.read::<data::ArgumentRequestMessage>(header.0) {
                                Ok(ok) => Ok(data::MessageEnum::ArgumentRequest(ok)),
                                Err(err) => Err(err)
                            },
                            MessageKind::ArgumentResponse => match protocol_host.read::<data::ArgumentResponseMessage>(header.0) {
                                Ok(ok) => Ok(data::MessageEnum::ArgumentResponse(ok)),
                                Err(err) => Err(err)
                            },
                            MessageKind::CallCompleted => match protocol_host.read::<data::CallCompletedMessage>(header.0) {
                                Ok(ok) => Ok(data::MessageEnum::CallCompleted(ok)),
                                Err(err) => Err(err)
                            },
                            MessageKind::ResultRequest => match protocol_host.read::<data::ResultRequestMessage>(header.0) {
                                Ok(ok) => Ok(data::MessageEnum::ResultRequest(ok)),
                                Err(err) => Err(err)
                            },
                            MessageKind::ResultResponse => match protocol_host.read::<data::ResultResponseMessage>(header.0) {
                                Ok(ok) => Ok(data::MessageEnum::ResultResponse(ok)),
                                Err(err) => Err(err)
                            },
                            MessageKind::CloseCall => match protocol_host.read::<data::CloseCallMessage>(header.0) {
                                Ok(ok) => Ok(data::MessageEnum::CloseCall(ok)),
                                Err(err) => Err(err)
                            },
                        };
                        let send_result = sender.send(message);
                        if let Err(error) = send_result {
                            panic!("Failed to send data using channel");
                        }
                    }
                }
                ()
            }));
            protocol_host.thread = thread;
            protocol_host.write(VersionMessage {
                major: 0,
                minor: 1,
                build: 0,
                revision: 0,
                protocol: ProtocolHost::PROTOCOL_VERSION,
            })?;
            protocol_host.client_version = protocol_host.read_full()?;
            if protocol_host.client_version.protocol != ProtocolHost::PROTOCOL_VERSION {
                protocol_host.quit()?;
                return Err("Client-Protocol version mismatch".into());
            }
            Ok(protocol_host)
        }
        pub fn quit(&mut self) -> Result<(), Error> {
            self.write(data::QuitMessage::Terminate)?;
            let mut remaining = 60;
            while remaining > 0 {
                thread::sleep(Duration::from_secs(1));
                let dequeue_result = self.messages.quit.remove();
                match dequeue_result {
                    Ok(quit_message) => {
                        let mut value = quit_message as u8;
                        if remaining < value { value = remaining; }
                        remaining = remaining - value;
                    }
                    Err(s) => break,
                }
            }
            self.host.terminate();
            let _ = self.thread.unwrap().join();
            Ok(())
        }
        fn poll(&mut self) {
            while let Ok(result) = self.receiver.try_recv() {
                match result {
                    Ok(ok) => {
                        match ok {
                            MessageEnum::Version(msg) => match self.messages.version.add(msg) {
                                Ok(_) => {}
                                Err(s) => panic!("{}", s),
                            },
                            MessageEnum::Quit(msg) => match self.messages.quit.add(msg) {
                                Ok(_) => {}
                                Err(s) => panic!("{}", s),
                            },
                            MessageEnum::CapabilitiesRequest(msg) => match self.messages.capabilities_request.add(msg) {
                                Ok(_) => {}
                                Err(s) => panic!("{}", s),
                            },
                            MessageEnum::CapabilitiesResponse(msg) => match self.messages.capabilities_response.add(msg) {
                                Ok(_) => {}
                                Err(s) => panic!("{}", s),
                            },
                            MessageEnum::FunctionCapabilitiesRequest(msg) => match self.messages.function_capabilities_request.add(msg) {
                                Ok(_) => {}
                                Err(s) => panic!("{}", s),
                            },
                            MessageEnum::FunctionCapabilitiesResponse(msg) => match self.messages.function_capabilities_response.add(msg) {
                                Ok(_) => {}
                                Err(s) => panic!("{}", s),
                            },
                            MessageEnum::Call(msg) => match self.messages.call.add(msg) {
                                Ok(_) => {}
                                Err(s) => panic!("{}", s),
                            },
                            MessageEnum::ArgumentRequest(msg) => match self.messages.argument_request.add(msg) {
                                Ok(_) => {}
                                Err(s) => panic!("{}", s),
                            },
                            MessageEnum::ArgumentResponse(msg) => match self.messages.argument_response.add(msg) {
                                Ok(_) => {}
                                Err(s) => panic!("{}", s),
                            },
                            MessageEnum::CallCompleted(msg) => match self.messages.call_completed.add(msg) {
                                Ok(_) => {}
                                Err(s) => panic!("{}", s),
                            },
                            MessageEnum::ResultRequest(msg) => match self.messages.result_request.add(msg) {
                                Ok(_) => {}
                                Err(s) => panic!("{}", s),
                            },
                            MessageEnum::ResultResponse(msg) => match self.messages.result_response.add(msg) {
                                Ok(_) => {}
                                Err(s) => panic!("{}", s),
                            },
                            MessageEnum::CloseCall(msg) => match self.messages.close_call.add(msg) {
                                Ok(_) => {}
                                Err(s) => panic!("{}", s),
                            },
                        }
                    }
                    Err(err) => {
                        match err {
                            Error::ProtocolError(s) => panic!("{}", s),
                            Error::IoError(io_error) => panic!("{}", io_error),
                        }
                    }
                }
            }
        }
    }
}