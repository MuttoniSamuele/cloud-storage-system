use std::{error::Error, fmt::Display};

#[derive(Debug)]
pub enum SignupError {
    UsernameExists,
    EmailExists,
    InvalidUsername,
    InvalidEmail,
    ShortPassword,
    InternalError,
}

impl Display for SignupError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            SignupError::UsernameExists => f.write_str("Username already exists"),
            SignupError::EmailExists => f.write_str("Email already exists"),
            SignupError::InvalidUsername => f.write_str("Invalid username"),
            SignupError::InvalidEmail => f.write_str("Invalid email"),
            SignupError::ShortPassword => f.write_str("Password too short"),
            SignupError::InternalError => f.write_str("Internal error"),
        }
    }
}

impl Error for SignupError {}

#[derive(Debug)]
pub enum LoginError {
    EmailDoesNotExists,
    WrongPassword,
    InternalError,
}

impl Display for LoginError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            LoginError::EmailDoesNotExists => f.write_str("Email does not exist"),
            LoginError::WrongPassword => f.write_str("Wrong password"),
            LoginError::InternalError => f.write_str("Internal error"),
        }
    }
}

impl Error for LoginError {}

#[derive(Debug)]
pub enum FileError {
    NameError,
    InternalError,
}

impl Display for FileError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            FileError::NameError => f.write_str("Invalid file name"),
            FileError::InternalError => f.write_str("Internal error"),
        }
    }
}

impl Error for FileError {}

#[derive(Debug)]
pub struct InternalError(pub String);

impl Display for InternalError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(&format!("Internal error: {}", self.0))
    }
}

impl Error for InternalError {}
