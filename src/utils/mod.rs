pub use errors::errors as inner_error;
pub use errors::errors::MyError;

pub mod errors;
#[macro_export]
macro_rules! error {
    ($msg: expr) => {
        $crate::utils::MyError::from_string($msg)
    };
    // ($msg: literal) => {
    //     $crate::utils::MyError::from_string($msg)
    // };
}

pub trait LogExt {
    fn log(self) -> Self;
}

impl<T, E> LogExt for Result<T, E>
where
    E: std::fmt::Display,
{
    fn log(self) -> Self {
        if let Err(e) = &self {
            log::error!("{}", e);
            // eprintln!("An error happened [{}:{}] {}", file!(), line!(), e);
        }
        self
    }
}