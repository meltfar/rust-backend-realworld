pub mod errors {
    use std::fmt::Display;

    #[derive(serde::Serialize)]
    pub struct ErrorResponseEntity {
        pub code: u16,
        pub msg: String,
    }

    #[derive(Debug, thiserror::Error)]
    pub struct MyError {
        pub status_code: u16,
        pub message: String,
    }

    impl MyError {
        pub fn new(sc: u16, msg: String) -> MyError {
            MyError {
                status_code: sc,
                message: msg,
            }
        }
        pub fn new_result<T>(sc: u16, msg: &str) -> Result<T, MyError> {
            Err(MyError::new(sc, msg.to_string()))
        }
    }

    impl Display for MyError {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(
                f,
                "error: {}, status_code: {}",
                self.message, self.status_code
            )
        }
    }

    impl actix_web::error::ResponseError for MyError {
        fn error_response(&self) -> actix_web::HttpResponse<actix_web::body::BoxBody> {
            actix_web::HttpResponse::build(self.status_code()).json(ErrorResponseEntity {
                code: self.status_code,
                msg: self.message.to_owned(),
            })
        }

        fn status_code(&self) -> actix_web::http::StatusCode {
            actix_web::http::StatusCode::from_u16(self.status_code)
                .unwrap_or(actix_web::http::StatusCode::INTERNAL_SERVER_ERROR)
        }
    }

    impl From<sqlx::Error> for MyError {
        fn from(err: sqlx::Error) -> Self {
            MyError {
                status_code: 500,
                message: err.to_string(),
            }
        }
    }

    impl From<std::io::Error> for MyError {
        fn from(err: std::io::Error) -> Self {
            MyError {
                status_code: 500,
                message: err.to_string(),
            }
        }
    }

    impl From<reqwest::Error> for MyError {
        fn from(err: reqwest::Error) -> Self {
            Self {
                status_code: 500,
                message: err.to_string(),
            }
        }
    }

    impl From<String> for MyError {
        fn from(err: String) -> Self {
            Self {
                status_code: 500,
                message: err.to_owned(),
            }
        }
    }

    impl From<&str> for MyError {
        fn from(err: &str) -> Self {
            Self {
                status_code: 500,
                message: err.to_owned(),
            }
        }
    }

    impl From<serde_json::Error> for MyError {
        fn from(err: serde_json::Error) -> Self {
            Self {
                status_code: 500,
                message: err.to_string(),
            }
        }
    }

    impl From<Box<dyn std::error::Error + 'static>> for MyError {
        fn from(err: Box<dyn std::error::Error + 'static>) -> Self {
            Self {
                status_code: 500,
                message: err.to_string(),
            }
        }
    }
}
