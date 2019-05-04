use log::{error, trace};
use std::fs;
use std::option::Option;
use regex::Regex;

pub struct FlowAnalysis {
    input_folder: String,
    input_pattern: Regex,
}

impl FlowAnalysis {
    pub fn new(folder: &str, pattern: &str) -> Option<FlowAnalysis> {
        // validate the input parameters
        if !fs::metadata(folder).is_ok() {
            error!("The provided input folder seems not to exist. Cannot proceed.");
            None
        } else {
            // everything validated, we can return the corresponding object
            trace!("New instance of a flow-analysis sub-command created.");
            Some(FlowAnalysis {
                input_folder: folder.to_string(),
                input_pattern: Regex::new(pattern).unwrap(),
            })
        }
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn creating_with_invalid_folder_fails() {
        assert_eq!(2 + 2, 4);
    }

    #[test]
    fn creating_with_valid_folder_works() {
        assert_eq!(2 + 2, 4);
    }
}
