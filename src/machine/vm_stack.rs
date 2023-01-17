use uuid::Uuid;
use crate::machine::{VmPair, VmValue};

pub struct VmStack {
    data: Vec<VmValue>,
    variables: Vec<VmPair>,
}

impl VmStack {
    pub fn new() -> VmStack {
        return VmStack {
            data: vec!(),
            variables: vec!(),
        };
    }
    pub fn push_value(&mut self, value: VmValue) {
        self.data.push(value);
    }

    pub fn pop_value(&mut self) -> Result<VmValue, &'static str> {
        let value_opt = self.data.pop();
        match value_opt {
            None => Err("Failed to pop value from an empty stack"),
            Some(value) => Ok(value),
        }
    }

    pub fn pop_object(&mut self) -> Result<Vec<VmPair>, &'static str> {
        let candidate = self.pop_value()?;
        match candidate {
            VmValue::Object(object) => Ok(object),
            _ => Err("Failed to pop OBJECT value from stack"),
        }
    }

    pub fn pop_array(&mut self) -> Result<Vec<VmValue>, &'static str> {
        let candidate = self.pop_value()?;
        match candidate {
            VmValue::Array(array) => Ok(array),
            _ => Err("Failed to pop ARRAY value from stack"),
        }
    }

    pub fn pop_string(&mut self) -> Result<String, &'static str> {
        let candidate = self.pop_value()?;
        match candidate {
            VmValue::String(string) => Ok(string),
            _ => Err("Failed to pop STRING value from stack"),
        }
    }

    pub fn pop_number(&mut self) -> Result<f64, &'static str> {
        let candidate = self.pop_value()?;
        match candidate {
            VmValue::Number(f) => Ok(f),
            _ => Err("Failed to pop NUMBER value from stack"),
        }
    }

    pub fn pop_bool(&mut self) -> Result<bool, &'static str> {
        let candidate = self.pop_value()?;
        match candidate {
            VmValue::Boolean(flag) => Ok(flag),
            _ => Err("Failed to pop BOOLEAN value from stack"),
        }
    }

    pub fn get_variable<S>(&self, name: S) -> Option<VmValue> where S: Into<String> {
        let key = name.into();
        for vm_pair in self.variables.iter() {
            if vm_pair.key == key {
                return Some(vm_pair.value.clone());
            }
        }
        return None;
    }
    pub fn set_variable<S>(&mut self, name: S, value: VmValue) where S: Into<String> {
        let key = name.into();
        for mut vm_pair in self.variables.iter_mut() {
            if vm_pair.key == key {
                vm_pair.value = value;
                return;
            }
        }
        self.variables.push(VmPair {
            key,
            value,
        });
    }

    pub fn pop_job(&mut self) -> Result<Uuid, &'static str> {
        let candidate = self.pop_value()?;
        match candidate {
            VmValue::Job(uuid) => Ok(uuid),
            _ => Err("Failed to pop JOB value from stack"),
        }
    }

    pub fn pop_array_of_jobs(&mut self) -> Result<Vec<Uuid>, &'static str> {
        let array_value = self.pop_value()?;
        if !array_value.is_array_of_jobs() {
            return Err("AbortAll expected array of jobs");
        }
        let mut jobs: Vec<Uuid> = vec!();
        match array_value {
            VmValue::Array(array) => {
                for value in array {
                    match value {
                        VmValue::Job(uuid) => jobs.push(uuid),
                        _ => return Err("Not all elements in array are of type job.")
                    }
                }
            }
            _ => return Err("Value is not of an array type.")
        }
        Ok(jobs)
    }
}


#[cfg(test)]
mod tests {
    use tracing_test::traced_test;
    use uuid::Uuid;
    use crate::machine::*;

    #[test]
    #[traced_test]
    fn new_creates_empty_stack() -> Result<(), Box<dyn std::error::Error>> {
        let stack = VmStack::new();
        match stack.data.len() == 0 && stack.variables.len() == 0 {
            false => Err("VmStack::new() creates non-empty stack".into()),
            true => Ok(()),
        }
    }

    #[test]
    #[traced_test]
    fn push_value_has_value() -> Result<(), Box<dyn std::error::Error>> {
        let mut stack = VmStack::new();
        stack.push_value(VmValue::Null);
        match stack.data.len() {
            1 => Ok(()),
            _ => Err("push_value did not increase data size".into()),
        }
    }

    #[test]
    #[traced_test]
    fn pop_value_empty_errors() -> Result<(), Box<dyn std::error::Error>> {
        let mut stack = VmStack::new();
        match stack.pop_value() {
            Ok(_) => Err("Empty stack yields value instead of error".into()),
            Err(_) => Ok(()),
        }
    }

    #[test]
    #[traced_test]
    fn pop_value_nonempty_no_error() -> Result<(), Box<dyn std::error::Error>> {
        let mut stack = VmStack::new();
        stack.data.push(VmValue::Null);
        match stack.pop_value() {
            Ok(_) => Ok(()),
            Err(_) => Err("Non empty stack yields an error instead of a value".into()),
        }
    }

    #[test]
    #[traced_test]
    fn pop_object_wrong_type_errors() -> Result<(), Box<dyn std::error::Error>> {
        let mut stack = VmStack::new();
        stack.push_value(VmValue::Null);
        match stack.pop_object() {
            Ok(_) => Err("pop_object with VmValue::Null returned valid object".into()),
            Err(_) => Ok(()),
        }
    }

    #[test]
    #[traced_test]
    fn pop_object_correct_type_no_error() -> Result<(), Box<dyn std::error::Error>> {
        let mut stack = VmStack::new();
        stack.push_value(VmValue::Object(vec!(VmPair {
            key: "abc".into(),
            value: VmValue::Null,
        })));
        match stack.pop_object() {
            Ok(v) => if v.len() == 1 && v[0].key == "abc" && v[0].value.is_null() {
                Ok(())
            } else {
                Err("pop_object with VmValue::Object returned a value \
                but the value does not hold the expected object".into())
            },
            Err(_) => Err("pop_object with VmValue::Object returned an error".into()),
        }
    }

    #[test]
    #[traced_test]
    fn pop_array_wrong_type_errors() -> Result<(), Box<dyn std::error::Error>> {
        let mut stack = VmStack::new();
        stack.push_value(VmValue::Null);
        match stack.pop_array() {
            Ok(_) => Err("pop_array with VmValue::Null returned valid array".into()),
            Err(_) => Ok(()),
        }
    }

    #[test]
    #[traced_test]
    fn pop_array_correct_type_no_error() -> Result<(), Box<dyn std::error::Error>> {
        let mut stack = VmStack::new();
        stack.push_value(VmValue::Array(vec!(VmValue::Null)));
        match stack.pop_array() {
            Ok(v) => if v.len() == 1 && v[0].is_null() {
                Ok(())
            } else {
                Err("pop_array with VmValue::Array returned a value \
                but the value does not hold the expected array".into())
            },
            Err(_) => Err("pop_array with VmValue::Array returned an error".into()),
        }
    }

    #[test]
    #[traced_test]
    fn pop_string_wrong_type_errors() -> Result<(), Box<dyn std::error::Error>> {
        let mut stack = VmStack::new();
        stack.push_value(VmValue::Null);
        match stack.pop_string() {
            Ok(_) => Err("pop_string with VmValue::Null returned valid string".into()),
            Err(_) => Ok(()),
        }
    }

    #[test]
    #[traced_test]
    fn pop_string_correct_type_no_error() -> Result<(), Box<dyn std::error::Error>> {
        let mut stack = VmStack::new();
        stack.push_value(VmValue::String("FooBar".into()));
        match stack.pop_string() {
            Ok(v) => if v == "FooBar" {
                Ok(())
            } else {
                Err("pop_string with VmValue::String returned a value \
                but the value does not hold the expected string".into())
            },
            Err(_) => Err("pop_string with VmValue::String returned an error".into()),
        }
    }

    #[test]
    #[traced_test]
    fn pop_number_wrong_type_errors() -> Result<(), Box<dyn std::error::Error>> {
        let mut stack = VmStack::new();
        stack.push_value(VmValue::Null);
        match stack.pop_number() {
            Ok(_) => Err("pop_number with VmValue::Null returned valid number".into()),
            Err(_) => Ok(()),
        }
    }

    #[test]
    #[traced_test]
    fn pop_number_correct_type_no_error() -> Result<(), Box<dyn std::error::Error>> {
        let mut stack = VmStack::new();
        stack.push_value(VmValue::Number(123.5));
        match stack.pop_number() {
            Ok(v) => if v == 123.5 {
                Ok(())
            } else {
                Err("pop_number with VmValue::Number returned a value \
                but the value does not hold the expected number".into())
            },
            Err(_) => Err("pop_number with VmValue::Number returned an error".into()),
        }
    }

    #[test]
    #[traced_test]
    fn pop_bool_wrong_type_errors() -> Result<(), Box<dyn std::error::Error>> {
        let mut stack = VmStack::new();
        stack.push_value(VmValue::Null);
        match stack.pop_bool() {
            Ok(_) => Err("pop_bool with VmValue::Null returned valid bool".into()),
            Err(_) => Ok(()),
        }
    }

    #[test]
    #[traced_test]
    fn pop_bool_correct_type_no_error() -> Result<(), Box<dyn std::error::Error>> {
        let mut stack = VmStack::new();
        stack.push_value(VmValue::Boolean(true));
        match stack.pop_bool() {
            Ok(v) => if v == true {
                Ok(())
            } else {
                Err("pop_bool with VmValue::Boolean returned a value \
                but the value does not hold the expected bool".into())
            },
            Err(_) => Err("pop_bool with VmValue::Boolean returned an error".into()),
        }
    }

    #[test]
    #[traced_test]
    fn pop_job_wrong_type_errors() -> Result<(), Box<dyn std::error::Error>> {
        let mut stack = VmStack::new();
        stack.push_value(VmValue::Null);
        match stack.pop_job() {
            Ok(_) => Err("pop_job with VmValue::Null returned valid job".into()),
            Err(_) => Ok(()),
        }
    }

    #[test]
    #[traced_test]
    fn pop_job_correct_type_no_error() -> Result<(), Box<dyn std::error::Error>> {
        let mut stack = VmStack::new();
        let uuid = Uuid::new_v4();
        stack.push_value(VmValue::Job(uuid));
        match stack.pop_job() {
            Ok(v) => if v == uuid {
                Ok(())
            } else {
                Err("pop_job with VmValue::job returned a value \
                but the value does not hold the expected job".into())
            },
            Err(_) => Err("pop_job with VmValue::job returned an error".into()),
        }
    }

    #[test]
    #[traced_test]
    fn get_variable_empty_not_existing() -> Result<(), Box<dyn std::error::Error>> {
        let stack = VmStack::new();
        match stack.get_variable("foobar") {
            Some(_) => Err("get_variable returned a value for foobar \
            but no variable exists on the stack.".into()),
            None => Ok(()),
        }
    }

    #[test]
    #[traced_test]
    fn get_variable_filled_not_existing() -> Result<(), Box<dyn std::error::Error>> {
        let mut stack = VmStack::new();
        stack.variables.push(VmPair {
            key: "barfoo".into(),
            value: VmValue::Null,
        });
        match stack.get_variable("foobar") {
            Some(_) => Err("get_variable returned a value for foobar \
            but no variable exists on the stack.".into()),
            None => Ok(()),
        }
    }

    #[test]
    #[traced_test]
    fn get_variable_existing() -> Result<(), Box<dyn std::error::Error>> {
        let mut stack = VmStack::new();
        stack.variables.push(VmPair {
            key: "foobar".into(),
            value: VmValue::Boolean(true),
        });
        match stack.get_variable("foobar") {
            Some(v) => match v {
                VmValue::Boolean(flag) => if flag == true {
                    Ok(())
                } else {
                    Err("get_variable returned a value for foobar \
                but the value does not hold the expected true bool".into())
                },
                _ => Err("get_variable returned a value for foobar \
                but the value does not hold the expected type".into())
            },
            None => Err("get_variable returned no value for foobar \
            but should have as the variable exists.".into()),
        }
    }

    #[test]
    #[traced_test]
    fn get_variable_empty_not_creating_variable() -> Result<(), Box<dyn std::error::Error>> {
        let stack = VmStack::new();
        let len = stack.variables.len();
        stack.get_variable("foobar");
        if stack.variables.len() != len {
            Err("get_variable created a variable when it should not have been able to.".into())
        } else {
            Ok(())
        }
    }

    #[test]
    #[traced_test]
    fn set_variable_empty_not_existing_creates_new_pair() -> Result<(), Box<dyn std::error::Error>> {
        let mut stack = VmStack::new();
        let len = stack.variables.len();
        stack.set_variable("foobar", VmValue::Null);
        if stack.variables.len() != len + 1 {
            Err("set_variable did not create a new VmPair in variables section of stack.".into())
        } else {
            Ok(())
        }
    }

    #[test]
    #[traced_test]
    fn set_variable_filled_not_existing_creates_new_pair() -> Result<(), Box<dyn std::error::Error>> {
        let mut stack = VmStack::new();
        stack.variables.push(VmPair {
            key: "barfoo".into(),
            value: VmValue::Boolean(true),
        });
        let len = stack.variables.len();
        stack.set_variable("foobar", VmValue::Null);
        if stack.variables.len() != len + 1 {
            Err("set_variable did not create a new VmPair in variables section of stack.".into())
        } else {
            Ok(())
        }
    }

    #[test]
    #[traced_test]
    fn set_variable_existing_no_new_pair() -> Result<(), Box<dyn std::error::Error>> {
        let mut stack = VmStack::new();
        stack.variables.push(VmPair {
            key: "foobar".into(),
            value: VmValue::Boolean(true),
        });
        let len = stack.variables.len();
        stack.set_variable("foobar", VmValue::Null);
        if stack.variables.len() != len {
            Err("set_variable created a new VmPair in variables section of stack even \
            though a matching pair existed.".into())
        } else {
            Ok(())
        }
    }

    #[test]
    #[traced_test]
    fn set_variable_existing_value_matches_expected() -> Result<(), Box<dyn std::error::Error>> {
        let mut stack = VmStack::new();
        stack.variables.push(VmPair {
            key: "foobar".into(),
            value: VmValue::Boolean(true),
        });
        stack.set_variable("foobar", VmValue::Null);
        match stack.get_variable("foobar") {
            Some(v) => match v {
                VmValue::Null => {}
                _ => return Err("set_variable did not update the value to the expected value.".into()),
            },
            _ => return Err("set_variable erased variable instead of setting it.".into())
        };
        stack.set_variable("foobar", VmValue::Boolean(true));
        match stack.get_variable("foobar") {
            Some(v) => match v {
                VmValue::Boolean(flag) => if flag == true {
                    Ok(())
                } else {
                    Err("set_variable did update the value to the expected type but it does not contain the expected value.".into())
                },
                _ => Err("set_variable did not update the value to the expected value.".into()),
            },
            _ => Err("set_variable erased variable instead of setting it.".into())
        }
    }
}