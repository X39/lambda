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
}
impl std::ops::Index<String> for VmStack {
    type Output = VmValue;

    fn index(&self, index: String) -> &Self::Output {
        todo!()
    }
}
impl std::ops::IndexMut<String> for VmStack {
    fn index_mut(&mut self, index: String) -> &mut Self::Output {
        todo!()
    }
}
impl std::ops::Index<usize> for VmStack {
    type Output = VmValue;

    fn index(&self, index: usize) -> &Self::Output {
        todo!()
    }
}
impl std::ops::IndexMut<usize> for VmStack {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        todo!()
    }
}


#[cfg(test)]
mod tests {
    use tracing_test::traced_test;
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
            Ok(_) => Err("pop_number with VmValue::Null returned valid string".into()),
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
            Ok(_) => Err("pop_bool with VmValue::Null returned valid string".into()),
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
}