mod cleaner;
mod linker;
pub mod parser;
mod preprocessor;
pub mod types;

pub use self::preprocessor::preprocess;
pub use self::linker::link;
pub use self::parser::parse;
pub use self::cleaner::{clean, print_unused};

error_chain! {
    links {
        Parse(parser::Error, parser::ErrorKind);
        Preprocess(preprocessor::Error, preprocessor::ErrorKind);
        Link(linker::Error, linker::ErrorKind);
    }
}
