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
pub struct SessionError;

impl Display for SessionError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str("Session error")
    }
}

impl Error for SessionError {}
