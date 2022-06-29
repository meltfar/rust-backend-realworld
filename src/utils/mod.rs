pub use errors::errors as inner_error;
pub use errors::errors::MyError;

pub mod errors;
#[macro_export]
macro_rules! error {
    ($msg: ident) => {
        $crate::utils::MyError::from_string($msg)
    };
    ($msg: literal) => {
        $crate::utils::MyError::from_string($msg)
    };
}