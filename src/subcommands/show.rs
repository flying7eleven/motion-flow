use crate::subcommands::{SubCommand, SubCommandError};
use image::DynamicImage;
use log::{error, trace};
use std::fs;

pub struct Show {
    _image_file: String,
    _image_handle: DynamicImage,
}

impl Show {
    pub fn get_instance(image_path: &str) -> Result<Box<dyn SubCommand>, SubCommandError> {
        if fs::metadata(image_path).is_err() {
            error!("The provided input image seems not to exist. Cannot proceed.");
            return Err(SubCommandError::InputFileDoesNotExist);
        }

        let maybe_image = image::open(&image_path);
        if maybe_image.is_err() {
            error!("Could not read the input image. Cannot proceed.");
            return Err(SubCommandError::CouldNotReadFile);
        }

        trace!("New instance of a show sub-command created.");
        Ok(Box::new(Show {
            _image_file: image_path.to_string(),
            _image_handle: maybe_image.unwrap(),
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

    #[test]
    fn creating_an_instance_with_existing_valid_image_works() {
        let instance = Show::get_instance("./data/ferret.jpg");
        assert_eq!(instance.is_err(), false);
    }

    #[test]
    fn creating_an_instance_with_existing_invalid_image_works() {
        let instance = Show::get_instance("./data/ferret_corrupt.jpg");
        assert_eq!(instance.is_err(), true);
        assert_eq!(instance.err().unwrap(), SubCommandError::CouldNotReadFile);
    }
}
