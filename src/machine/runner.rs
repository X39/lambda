use crate::machine::{InstructionArg, VmValue, VmValueType};

impl VmValue {
    pub fn is_job(&self) -> bool {
        match self {
            VmValue::Null => false,
            VmValue::String(_) => false,
            VmValue::Number(_) => false,
            VmValue::Array(_) => false,
            VmValue::Boolean(_) => false,
            VmValue::Object(_) => false,
            VmValue::Job(_) => true,
        }
    }
    pub fn is_array(&self) -> bool {
        match self {
            VmValue::Null => false,
            VmValue::String(_) => false,
            VmValue::Number(_) => false,
            VmValue::Array(_) => true,
            VmValue::Boolean(_) => false,
            VmValue::Object(_) => false,
            VmValue::Job(_) => false,
        }
    }
    fn is_string(&self) -> bool {
        match self {
            VmValue::Null => false,
            VmValue::String(_) => true,
            VmValue::Number(_) => false,
            VmValue::Array(_) => false,
            VmValue::Boolean(_) => false,
            VmValue::Object(_) => false,
            VmValue::Job(_) => false,
        }
    }
    pub fn is_object(&self) -> bool {
        match self {
            VmValue::Null => false,
            VmValue::String(_) => false,
            VmValue::Number(_) => false,
            VmValue::Array(_) => false,
            VmValue::Boolean(_) => false,
            VmValue::Object(_) => true,
            VmValue::Job(_) => false,
        }
    }
    pub fn is_null(&self) -> bool {
        match self {
            VmValue::Null => true,
            VmValue::String(_) => false,
            VmValue::Number(_) => false,
            VmValue::Array(_) => false,
            VmValue::Boolean(_) => false,
            VmValue::Object(_) => false,
            VmValue::Job(_) => false,
        }
    }
    pub fn is_array_of_jobs(&self) -> bool {
        match self {
            VmValue::Null => false,
            VmValue::String(_) => false,
            VmValue::Number(_) => false,
            VmValue::Array(arr) => {
                let mut all_jobs = true;
                for arr_value in arr {
                    if !arr_value.is_job()
                    {
                        all_jobs = false;
                        break;
                    }
                }
                all_jobs
            }
            VmValue::Boolean(_) => false,
            VmValue::Object(_) => false,
            VmValue::Job(_) => false,
        }
    }
}

impl InstructionArg {
    pub fn get_vm_type(self) -> Result<VmValueType, &'static str> {
        match self {
            InstructionArg::Empty => Err("Instruction arg was expected to be Type arg but arg was Empty"),
            InstructionArg::Unsigned(_) => Err("Instruction arg was expected to be Type arg but arg was Unsigned"),
            InstructionArg::Signed(_) => Err("Instruction arg was expected to be Type arg but arg was Signed"),
            InstructionArg::Type(t) => Ok(t),
        }
    }
    pub fn get_signed(self) -> Result<i16, &'static str> {
        match self {
            InstructionArg::Empty => Err("Instruction arg was expected to be Signed arg but arg was Empty"),
            InstructionArg::Unsigned(_) => Err("Instruction arg was expected to be Signed arg but arg was Unsigned"),
            InstructionArg::Signed(signed) => Ok(signed),
            InstructionArg::Type(_) => Err("Instruction arg was expected to be Signed arg but arg was Type"),
        }
    }
    pub fn get_unsigned(self) -> Result<u16, &'static str> {
        match self {
            InstructionArg::Empty => Err("Instruction arg was expected to be Signed arg but arg was Empty"),
            InstructionArg::Unsigned(unsigned) => Ok(unsigned),
            InstructionArg::Signed(_) => Err("Instruction arg was expected to be Signed arg but arg was Signed"),
            InstructionArg::Type(_) => Err("Instruction arg was expected to be Signed arg but arg was Type"),
        }
    }
}
