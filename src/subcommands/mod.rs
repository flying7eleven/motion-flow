pub mod dummy;
pub mod flowanalysis;
pub mod show;

#[derive(Clone, Debug, PartialEq)]
pub enum SubCommandError {
    RegularExpressionInvalid,
    InputFolderDoesNotExist,
    InputFileDoesNotExist,
}

pub trait SubCommand {
    fn execute(&self) -> bool;
}
