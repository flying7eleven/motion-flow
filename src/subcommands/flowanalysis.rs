use crate::subcommands::{SubCommand, SubCommandError};
use log::{error, trace};
use regex::Regex;
use std::fs;

pub struct FlowAnalysis {
    _input_folder: String,
    _input_pattern: Regex,
}

impl FlowAnalysis {
    pub fn get_instance(
        folder: &str,
        pattern: &str,
    ) -> Result<Box<dyn SubCommand>, SubCommandError> {
        if fs::metadata(folder).is_err() {
            error!("The provided input folder seems not to exist. Cannot proceed.");
            return Err(SubCommandError::InputFolderDoesNotExist);
        }

        let compiled_pattern = Regex::new(pattern);
        if compiled_pattern.is_err() {
            error!("Could not compile pattern to a regular expression.");
            return Err(SubCommandError::RegularExpressionInvalid);
        }

        trace!("New instance of a flow-analysis sub-command created.");
        Ok(Box::new(FlowAnalysis {
            _input_folder: folder.to_string(),
            _input_pattern: compiled_pattern.unwrap(),
        }))
    }
}

impl SubCommand for FlowAnalysis {
    fn execute(&self) -> bool {
        unimplemented!()
    }
}

#[cfg(test)]
mod tests {
    use crate::subcommands::flowanalysis::FlowAnalysis;
    use crate::subcommands::SubCommandError;

    #[test]
    fn creating_with_invalid_folder_and_valid_pattern_fails() {
        let instance = FlowAnalysis::get_instance("/this/folder/does/not/exist", ".*");
        assert_eq!(instance.is_err(), true);
        assert_eq!(
            instance.err().unwrap(),
            SubCommandError::InputFolderDoesNotExist
        );
    }

    #[test]
    fn creating_with_valid_folder_and_invalid_pattern_fails() {
        let instance = FlowAnalysis::get_instance(".", r"(?m)^([0-9]+$");
        assert_eq!(instance.is_err(), true);
        assert_eq!(
            instance.err().unwrap(),
            SubCommandError::RegularExpressionInvalid
        );
    }

    #[test]
    fn creating_with_valid_folder_and_valid_pattern_works() {
        let instance = FlowAnalysis::get_instance(".", ".*");
        assert_eq!(instance.is_err(), false);
    }
}
