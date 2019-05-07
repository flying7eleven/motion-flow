use crate::subcommands::{SubCommand, SubCommandError};
use log::trace;

pub struct Dummy {}

impl Dummy {
    pub fn new() -> Result<Box<dyn SubCommand>, SubCommandError> {
        trace!("New instance of a dummy sub-command created.");
        Ok(Box::new(Dummy {}))
    }
}

impl SubCommand for Dummy {
    fn execute(&self) -> bool {
        println!("{}", "Hello, Dummy!");
        true
    }
}

#[cfg(test)]
mod tests {
    use crate::subcommands::dummy::Dummy;

    #[test]
    fn creating_an_instance_of_the_dummy_works() {
        let instance = Dummy::new();
        assert_eq!(instance.is_err(), false);
    }
}
