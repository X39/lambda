use crate::machine::{VmValue, VmValueType};

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
    pub fn is_boolean(&self) -> bool {
        match self {
            VmValue::Null => false,
            VmValue::String(_) => false,
            VmValue::Number(_) => false,
            VmValue::Array(_) => false,
            VmValue::Boolean(_) => true,
            VmValue::Object(_) => false,
            VmValue::Job(_) => false,
        }
    }
    pub fn is_type(&self, value_type: VmValueType) -> bool {
        match value_type {
            VmValueType::Null => self.is_null(),
            VmValueType::Array => self.is_array(),
            VmValueType::ArrayOfJobs => self.is_array_of_jobs(),
            VmValueType::Job => self.is_job(),
        }
    }
}