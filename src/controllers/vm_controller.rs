use uuid::Uuid;
use crate::machine::{VmState, VmValue};

pub trait VmController {
    fn call(&self, function: String, arg: Option<VmValue>) -> Result<Uuid, Box<dyn std::error::Error>>;
    fn get_and_remove_result_of(&self, job: Uuid) -> Result<Option<VmValue>, Box<dyn std::error::Error>>;
    fn suspend_until_all(&self, state: &VmState, jobs: Vec<Uuid>) -> Result<(), Box<dyn std::error::Error>>;
    fn suspend_until_any(&self, state: &VmState, jobs: Vec<Uuid>) -> Result<(), Box<dyn std::error::Error>>;
    fn abort(&self, jobs: Vec<Uuid>) -> Result<(), Box<dyn std::error::Error>>;
}