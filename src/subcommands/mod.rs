pub mod dummy;
pub mod flowanalysis;

#[derive(Clone, Debug, PartialEq)]
pub enum SubCommandError {
    RegularExpressionInvalid,
    InputFolderDoesNotExist,
}

pub trait SubCommand {
    fn execute(&self) -> bool;
}