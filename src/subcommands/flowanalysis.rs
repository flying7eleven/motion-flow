use log::{error, trace};
use regex::Regex;
use std::fs;

pub struct FlowAnalysis {
    input_folder: String,
    input_pattern: Regex,
}

#[derive(Clone, Debug, PartialEq)]
pub enum Error {
    RegularExpressionInvalid,
    InputFolderDoesNotExist,
}

impl FlowAnalysis {
    pub fn new(folder: &str, pattern: &str) -> Result<Box<FlowAnalysis>, Error> {
        if fs::metadata(folder).is_err() {
            error!("The provided input folder seems not to exist. Cannot proceed.");
            return Err(Error::InputFolderDoesNotExist);
        }

        let compiled_pattern = Regex::new(pattern);
        if compiled_pattern.is_err() {
            error!("Could not compile pattern to a regular expression.");
            return Err(Error::RegularExpressionInvalid);
        }

        trace!("New instance of a flow-analysis sub-command created.");
        Ok(Box::new(FlowAnalysis {
            _input_folder: folder.to_string(),
            _input_pattern: compiled_pattern.unwrap(),
        }))
    }
}

#[cfg(test)]
mod tests {
    use crate::subcommands::flowanalysis::Error;
    use crate::subcommands::flowanalysis::FlowAnalysis;

    #[test]
    fn creating_with_invalid_folder_and_valid_pattern_fails() {
        let instance = FlowAnalysis::new("/this/folder/does/not/exist", ".*");
        assert_eq!(instance.is_err(), true);
        assert_eq!(instance.err().unwrap(), Error::InputFolderDoesNotExist);
    }

    #[test]
    fn creating_with_valid_folder_and_invalid_pattern_fails() {
        let instance = FlowAnalysis::new(".", r"(?m)^([0-9]+$");
        assert_eq!(instance.is_err(), true);
        assert_eq!(instance.err().unwrap(), Error::RegularExpressionInvalid);
    }

    #[test]
    fn creating_with_valid_folder_and_valid_pattern_works() {
        let instance = FlowAnalysis::new(".", ".*");
        assert_eq!(instance.is_err(), false);
    }
}
