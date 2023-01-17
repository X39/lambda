use std::error::Error;
use std::process::Command;
use uuid::Uuid;
use crate::controllers::VmController;
use crate::machine::{VmState, VmValue};

pub struct VmLocalController();

impl VmLocalController {
    pub fn new() -> VmLocalController {
        VmLocalController{}
    }
}

impl VmController for VmLocalController {
    fn call(&self, function: String, arg: Option<VmValue>) -> Result<Uuid, Box<dyn Error>> {
        let output = if cfg!(target_os = "windows") {
            Command::new("cmd")
                .args(["/C", "echo hello"])
                .output()
                .expect("failed to execute process")
        } else {
            Command::new("sh")
                .arg("-c")
                .arg("echo hello")
                .output()
                .expect("failed to execute process")
        };
        let data = output.stdout;
        todo!()
    }

    fn get_and_remove_result_of(&self, job: Uuid) -> Result<Option<VmValue>, Box<dyn Error>> {
        todo!()
    }

    fn suspend_until_all(&self, state: &VmState, jobs: Vec<Uuid>) -> Result<(), Box<dyn Error>> {
        todo!()
    }

    fn suspend_until_any(&self, state: &VmState, jobs: Vec<Uuid>) -> Result<(), Box<dyn Error>> {
        todo!()
    }

    fn abort(&self, jobs: Vec<Uuid>) -> Result<(), Box<dyn Error>> {
        todo!()
    }
}