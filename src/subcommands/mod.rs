pub mod dummy;
pub mod flowanalysis;
pub mod show;

#[derive(Clone, Debug, PartialEq)]
pub enum SubCommandError {
    RegularExpressionInvalid,
    InputFolderDoesNotExist,
    InputFileDoesNotExist,
    CouldNotReadFile,
}

pub trait SubCommand {
    fn execute(&self) -> bool;
}
