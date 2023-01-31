pub mod protocol_v1 {
    use std::io::{ErrorKind, Read, Write};
    use crate::io::protocol_v1::protocol_v1::data::{MessageKind, Message, VersionMessage, Quit};
    use crate::io::protocol_v1::protocol_v1::data::MessageKind::Version;

    mod io {
        use std::io::{Write, Read};
        use std::mem::size_of;

        pub fn write_bool(writer: &mut dyn Write, value: bool) -> Result<usize, std::io::Error> {
            if value {
                write_u8(writer, 1)
            } else {
                write_u8(writer, 0)
            }
        }

        pub fn write_u8(writer: &mut dyn Write, value: u8) -> Result<usize, std::io::Error> {
            let para = value.to_le_bytes();
            writer.write(para.as_slice())
        }

        pub fn write_u16(writer: &mut dyn Write, value: u16) -> Result<usize, std::io::Error> {
            let para = value.to_le_bytes();
            writer.write(para.as_slice())
        }

        pub fn write_u32(writer: &mut dyn Write, value: u32) -> Result<usize, std::io::Error> {
            let para = value.to_le_bytes();
            writer.write(para.as_slice())
        }

        pub fn write_u64(writer: &mut dyn Write, value: u64) -> Result<usize, std::io::Error> {
            let para = value.to_le_bytes();
            writer.write(para.as_slice())
        }

        pub fn write_i8(writer: &mut dyn Write, value: i8) -> Result<usize, std::io::Error> {
            let para = value.to_le_bytes();
            writer.write(para.as_slice())
        }

        pub fn write_i16(writer: &mut dyn Write, value: i16) -> Result<usize, std::io::Error> {
            let para = value.to_le_bytes();
            writer.write(para.as_slice())
        }

        pub fn write_i32(writer: &mut dyn Write, value: i32) -> Result<usize, std::io::Error> {
            let para = value.to_le_bytes();
            writer.write(para.as_slice())
        }

        pub fn write_i64(writer: &mut dyn Write, value: i64) -> Result<usize, std::io::Error> {
            let para = value.to_le_bytes();
            writer.write(para.as_slice())
        }

        pub fn write_string(writer: &mut dyn Write, text: &String, index: usize, length: usize) -> Result<usize, std::io::Error> {
            let para = text[index..length].as_bytes();
            writer.write(para)
        }


        pub fn read_bool(reader: &mut dyn Read) -> Result<bool, std::io::Error> {
            let result = read_u8(reader)?;
            Ok(result > 0)
        }

        pub fn read_u8(reader: &mut dyn Read) -> Result<u8, std::io::Error> {
            let mut buff: [u8; size_of::<u8>()] = [0; size_of::<u8>()];
            reader.read_exact(&mut buff)?;
            Ok(u8::from_le_bytes(buff))
        }

        pub fn read_u16(reader: &mut dyn Read) -> Result<u16, std::io::Error> {
            let mut buff: [u8; size_of::<u16>()] = [0; size_of::<u16>()];
            reader.read_exact(&mut buff)?;
            Ok(u16::from_le_bytes(buff))
        }

        pub fn read_u32(reader: &mut dyn Read) -> Result<u32, std::io::Error> {
            let mut buff: [u8; size_of::<u32>()] = [0; size_of::<u32>()];
            reader.read_exact(&mut buff)?;
            Ok(u32::from_le_bytes(buff))
        }

        pub fn read_u64(reader: &mut dyn Read) -> Result<u64, std::io::Error> {
            let mut buff: [u8; size_of::<u64>()] = [0; size_of::<u64>()];
            reader.read_exact(&mut buff)?;
            Ok(u64::from_le_bytes(buff))
        }

        pub fn read_i8(reader: &mut dyn Read) -> Result<i8, std::io::Error> {
            let mut buff: [u8; size_of::<i8>()] = [0; size_of::<i8>()];
            reader.read_exact(&mut buff)?;
            Ok(i8::from_le_bytes(buff))
        }

        pub fn read_i16(reader: &mut dyn Read) -> Result<i16, std::io::Error> {
            let mut buff: [u8; size_of::<i16>()] = [0; size_of::<i16>()];
            reader.read_exact(&mut buff)?;
            Ok(i16::from_le_bytes(buff))
        }

        pub fn read_i32(reader: &mut dyn Read) -> Result<i32, std::io::Error> {
            let mut buff: [u8; size_of::<i32>()] = [0; size_of::<i32>()];
            reader.read_exact(&mut buff)?;
            Ok(i32::from_le_bytes(buff))
        }

        pub fn read_i64(reader: &mut dyn Read) -> Result<i64, std::io::Error> {
            let mut buff: [u8; size_of::<i64>()] = [0; size_of::<i64>()];
            reader.read_exact(&mut buff)?;
            Ok(i64::from_le_bytes(buff))
        }

        pub fn read_string(reader: &mut dyn Read, length: usize) -> Result<String, std::io::Error> {
            let mut text: String = String::with_capacity(length);
            // ToDo: Find a better way to convert unknown encoding text with as little overhead as possible into rust String to remove unsafe block
            let buff = unsafe { text.as_bytes_mut() };
            reader.read_exact(buff)?;
            Ok(text)
        }
    }

    mod data {
        use std::io::{Error, Read, Write};
        use crate::io::protocol_v1::protocol_v1;

        #[derive(PartialEq)]
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
                protocol_v1::io::write_u32(writer, self.major)?;
                protocol_v1::io::write_u32(writer, self.minor)?;
                protocol_v1::io::write_u32(writer, self.build)?;
                protocol_v1::io::write_u32(writer, self.revision)?;
                protocol_v1::io::write_u32(writer, self.protocol)?;
                Ok(())
            }

            fn deserialize(&mut self, reader: &mut dyn Read) -> Result<(), Error> {
                self.major = protocol_v1::io::read_u32(reader)?;
                self.minor = protocol_v1::io::read_u32(reader)?;
                self.build = protocol_v1::io::read_u32(reader)?;
                self.revision = protocol_v1::io::read_u32(reader)?;
                self.protocol = protocol_v1::io::read_u32(reader)?;
                Ok(())
            }
        }

        #[repr(u8)]
        pub enum Quit {
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

        impl Message for Quit {
            const KIND: MessageKind = MessageKind::Quit;

            fn new() -> Self {
                Quit::Terminate
            }

            fn length(&self) -> usize {
                20
            }

            fn serialize(&self, writer: &mut dyn Write) -> Result<(), Error> {
                let val = match self {
                    Quit::Terminate => 0,
                    Quit::Additional1s => 1,
                    Quit::Additional2s => 2,
                    Quit::Additional3s => 3,
                    Quit::Additional4s => 4,
                    Quit::Additional5s => 5,
                    Quit::Additional6s => 6,
                    Quit::Additional7s => 7,
                    Quit::Additional8s => 8,
                    Quit::Additional9s => 9,
                    Quit::Additional10s => 10,
                    Quit::Additional11s => 11,
                    Quit::Additional12s => 12,
                    Quit::Additional13s => 13,
                    Quit::Additional14s => 14,
                    Quit::Additional15s => 15,
                    Quit::Additional16s => 16,
                    Quit::Additional17s => 17,
                    Quit::Additional18s => 18,
                    Quit::Additional19s => 19,
                    Quit::Additional20s => 20,
                    Quit::Additional21s => 21,
                    Quit::Additional22s => 22,
                    Quit::Additional23s => 23,
                    Quit::Additional24s => 24,
                    Quit::Additional25s => 25,
                    Quit::Additional26s => 26,
                    Quit::Additional27s => 27,
                    Quit::Additional28s => 28,
                    Quit::Additional29s => 29,
                    Quit::Additional30s => 30,
                    Quit::Additional31s => 31,
                    Quit::Additional32s => 32,
                    Quit::Additional33s => 33,
                    Quit::Additional34s => 34,
                    Quit::Additional35s => 35,
                    Quit::Additional36s => 36,
                    Quit::Additional37s => 37,
                    Quit::Additional38s => 38,
                    Quit::Additional39s => 39,
                    Quit::Additional40s => 40,
                    Quit::Additional41s => 41,
                    Quit::Additional42s => 42,
                    Quit::Additional43s => 43,
                    Quit::Additional44s => 44,
                    Quit::Additional45s => 45,
                    Quit::Additional46s => 46,
                    Quit::Additional47s => 47,
                    Quit::Additional48s => 48,
                    Quit::Additional49s => 49,
                    Quit::Additional50s => 50,
                    Quit::Additional51s => 51,
                    Quit::Additional52s => 52,
                    Quit::Additional53s => 53,
                    Quit::Additional54s => 54,
                    Quit::Additional55s => 55,
                    Quit::Additional56s => 56,
                    Quit::Additional57s => 57,
                    Quit::Additional58s => 58,
                    Quit::Additional59s => 59,
                };
                protocol_v1::io::write_u8(writer, val)?;
                Ok(())
            }

            fn deserialize(&mut self, reader: &mut dyn Read) -> Result<(), Error> {
                let val = protocol_v1::io::read_u8(reader)?;
                *self = match val {
                    0 => Quit::Terminate,
                    1 => Quit::Additional1s,
                    2 => Quit::Additional2s,
                    3 => Quit::Additional3s,
                    4 => Quit::Additional4s,
                    5 => Quit::Additional5s,
                    6 => Quit::Additional6s,
                    7 => Quit::Additional7s,
                    8 => Quit::Additional8s,
                    9 => Quit::Additional9s,
                    10 => Quit::Additional10s,
                    11 => Quit::Additional11s,
                    12 => Quit::Additional12s,
                    13 => Quit::Additional13s,
                    14 => Quit::Additional14s,
                    15 => Quit::Additional15s,
                    16 => Quit::Additional16s,
                    17 => Quit::Additional17s,
                    18 => Quit::Additional18s,
                    19 => Quit::Additional19s,
                    20 => Quit::Additional20s,
                    21 => Quit::Additional21s,
                    22 => Quit::Additional22s,
                    23 => Quit::Additional23s,
                    24 => Quit::Additional24s,
                    25 => Quit::Additional25s,
                    26 => Quit::Additional26s,
                    27 => Quit::Additional27s,
                    28 => Quit::Additional28s,
                    29 => Quit::Additional29s,
                    30 => Quit::Additional30s,
                    31 => Quit::Additional31s,
                    32 => Quit::Additional32s,
                    33 => Quit::Additional33s,
                    34 => Quit::Additional34s,
                    35 => Quit::Additional35s,
                    36 => Quit::Additional36s,
                    37 => Quit::Additional37s,
                    38 => Quit::Additional38s,
                    39 => Quit::Additional39s,
                    40 => Quit::Additional40s,
                    41 => Quit::Additional41s,
                    42 => Quit::Additional42s,
                    43 => Quit::Additional43s,
                    44 => Quit::Additional44s,
                    45 => Quit::Additional45s,
                    46 => Quit::Additional46s,
                    47 => Quit::Additional47s,
                    48 => Quit::Additional48s,
                    49 => Quit::Additional49s,
                    50 => Quit::Additional50s,
                    51 => Quit::Additional51s,
                    52 => Quit::Additional52s,
                    53 => Quit::Additional53s,
                    54 => Quit::Additional54s,
                    55 => Quit::Additional55s,
                    56 => Quit::Additional56s,
                    57 => Quit::Additional57s,
                    58 => Quit::Additional58s,
                    59 => Quit::Additional59s,
                    _ => return Err(Error::new(std::io::ErrorKind::InvalidData, "Quit message received is out of the valid value range.")),
                };
                Ok(())
            }
        }

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
                protocol_v1::io::write_u32(writer, self.functions_count)?;
                Ok(())
            }

            fn deserialize(&mut self, reader: &mut dyn Read) -> Result<(), Error> {
                self.functions_count = protocol_v1::io::read_u32(reader)?;
                Ok(())
            }
        }

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
                protocol_v1::io::write_u32(writer, self.function_requested)?;
                Ok(())
            }

            fn deserialize(&mut self, reader: &mut dyn Read) -> Result<(), Error> {
                self.function_requested = protocol_v1::io::read_u32(reader)?;
                Ok(())
            }
        }

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
                protocol_v1::io::write_u32(writer, self.function_index)?;
                protocol_v1::io::write_u8(writer, self.arguments_required)?;
                protocol_v1::io::write_u8(writer, self.arguments_count)?;
                protocol_v1::io::write_u8(writer, self.results_count)?;
                protocol_v1::io::write_u16(writer, name_length)?;
                protocol_v1::io::write_string(writer, &self.function_name, 0, name_length as usize)?;
                Ok(())
            }

            fn deserialize(&mut self, reader: &mut dyn Read) -> Result<(), Error> {
                self.function_index = protocol_v1::io::read_u32(reader)?;
                self.arguments_required = protocol_v1::io::read_u8(reader)?;
                self.arguments_count = protocol_v1::io::read_u8(reader)?;
                self.results_count = protocol_v1::io::read_u8(reader)?;
                let name_length = protocol_v1::io::read_u16(reader)?;
                self.function_name = protocol_v1::io::read_string(reader, name_length as usize)?;
                Ok(())
            }
        }

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
                protocol_v1::io::write_u32(writer, self.function_index)?;
                protocol_v1::io::write_u8(writer, self.arguments_count)?;
                protocol_v1::io::write_u32(writer, self.call_request_id)?;
                Ok(())
            }

            fn deserialize(&mut self, reader: &mut dyn Read) -> Result<(), Error> {
                self.function_index = protocol_v1::io::read_u32(reader)?;
                self.arguments_count = protocol_v1::io::read_u8(reader)?;
                self.call_request_id = protocol_v1::io::read_u32(reader)?;
                Ok(())
            }
        }

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
                protocol_v1::io::write_u32(writer, self.call_request_id)?;
                protocol_v1::io::write_u8(writer, self.argument_index)?;
                Ok(())
            }

            fn deserialize(&mut self, reader: &mut dyn Read) -> Result<(), Error> {
                self.call_request_id = protocol_v1::io::read_u32(reader)?;
                self.argument_index = protocol_v1::io::read_u8(reader)?;
                Ok(())
            }
        }

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
                protocol_v1::io::write_u32(writer, json_length)?;
                protocol_v1::io::write_string(writer, &self.json, 0, json_length as usize)?;
                Ok(())
            }

            fn deserialize(&mut self, reader: &mut dyn Read) -> Result<(), Error> {
                let json_length = protocol_v1::io::read_u32(reader)?;
                self.json = protocol_v1::io::read_string(reader, json_length as usize)?;
                Ok(())
            }
        }

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
                protocol_v1::io::write_u32(writer, self.call_request_id)?;
                protocol_v1::io::write_bool(writer, self.success)?;
                protocol_v1::io::write_u8(writer, self.results_count)?;
                Ok(())
            }

            fn deserialize(&mut self, reader: &mut dyn Read) -> Result<(), Error> {
                self.call_request_id = protocol_v1::io::read_u32(reader)?;
                self.success = protocol_v1::io::read_bool(reader)?;
                self.results_count = protocol_v1::io::read_u8(reader)?;
                Ok(())
            }
        }

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
                protocol_v1::io::write_u32(writer, self.call_request_id)?;
                protocol_v1::io::write_u8(writer, self.result_index)?;
                Ok(())
            }

            fn deserialize(&mut self, reader: &mut dyn Read) -> Result<(), Error> {
                self.call_request_id = protocol_v1::io::read_u32(reader)?;
                self.result_index = protocol_v1::io::read_u8(reader)?;
                Ok(())
            }
        }

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
                protocol_v1::io::write_u32(writer, json_length)?;
                protocol_v1::io::write_string(writer, &self.json, 0, json_length as usize)?;
                Ok(())
            }

            fn deserialize(&mut self, reader: &mut dyn Read) -> Result<(), Error> {
                let json_length = protocol_v1::io::read_u32(reader)?;
                self.json = protocol_v1::io::read_string(reader, json_length as usize)?;
                Ok(())
            }
        }

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
                protocol_v1::io::write_u32(writer, self.call_request_id)?;
                Ok(())
            }

            fn deserialize(&mut self, reader: &mut dyn Read) -> Result<(), Error> {
                self.call_request_id = protocol_v1::io::read_u32(reader)?;
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
        writer: &'a mut dyn Write,
        reader: &'a mut dyn Read,
        client_version: VersionMessage,
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
            io::write_u8(self.writer, 0)?;
            io::write_u8(self.writer, ProtocolHost::FRAME_VERSION)?;
            io::write_u16(self.writer, MESSAGE::KIND.into())?;
            io::write_u32(self.writer, msg_length as u32)?;
            message.serialize(self.writer)?;
            Ok(())
        }
        fn read_header(&mut self) -> Result<(data::MessageKind, u32), std::io::Error>
        {
            let _ = io::read_u8(self.reader)?;
            let frame_version = io::read_u8(self.reader)?;
            if frame_version != ProtocolHost::FRAME_VERSION {
                return Err(std::io::Error::new(ErrorKind::InvalidData, "Unsupported frame version"));
            }
            let message_id = io::read_u16(self.reader)?;
            let message_length = io::read_u32(self.reader)?;
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
            message.deserialize(self.reader)?;
            return Ok(message);
        }


        pub fn connect(writer: &'a mut dyn Write, reader: &'a mut dyn Read) -> Result<ProtocolHost<'a>, Error> {
            let mut host = ProtocolHost { reader, writer, client_version: VersionMessage::new() };

            host.write(VersionMessage {
                major: 0,
                minor: 1,
                build: 0,
                revision: 0,
                protocol: ProtocolHost::PROTOCOL_VERSION,
            })?;
            host.client_version = host.read_full()?;
            if host.client_version.protocol != ProtocolHost::PROTOCOL_VERSION {
                host.write(Quit::Terminate)?;
                // ToDo: implement proper termination protocol, awaiting termination of the process attached by inducing the quit message from the caller
                return Err("Client-Protocol version mismatch".into());
            }
            Ok(host)
        }
        // ToDo: Write async-based threading model in here to "host" the actual protocol read/writes and decide whether to notify (event-based) or get polled from the outside.
    }
}