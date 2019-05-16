use crate::subcommands::{SubCommand, SubCommandError};
use log::{error, trace};
use std::fs;

pub struct Show {
    _image_file: String,
}

impl Show {
    pub fn get_instance(image_path: &str) -> Result<Box<dyn SubCommand>, SubCommandError> {
        if fs::metadata(image_path).is_err() {
            error!("The provided input image seems not to exist. Cannot proceed.");
            return Err(SubCommandError::InputFileDoesNotExist);
        }

        trace!("New instance of a show sub-command created.");
        Ok(Box::new(Show {
            _image_file: image_path.to_string(),
        }))
    }
}

impl SubCommand for Show {
    fn execute(&self) -> bool {
        true
    }
}

#[cfg(test)]
mod tests {
    use crate::subcommands::show::Show;
    use crate::subcommands::SubCommandError;

    #[test]
    fn creating_an_instance_with_nonexisting_image_fails() {
        let instance = Show::get_instance("/this/file/will/not/exist.jpg");
        assert_eq!(instance.is_err(), true);
        assert_eq!(
            instance.err().unwrap(),
            SubCommandError::InputFileDoesNotExist
        );
    }
}
