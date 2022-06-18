use regex::Regex;
use pwhash::bcrypt;
extern crate zxcvbn;
use zxcvbn::{zxcvbn, time_estimates::CrackTimes};

pub enum Email {
    Raw(String),
    Validated(String),
    Verified(String),
    Invalid(String)
}

impl Email {
    pub fn new(email: String) -> Self {
        Self::Raw(email)                 
    }
    
    pub fn validate(self) -> Self {
        let email_regex = Regex::new(r"^([a-z0-9_+]([a-z0-9_+.]*[a-z0-9_+])?)@([a-z0-9]+([\-\.]{1}[a-z0-9]+)*\.[a-z]{2,6})").unwrap();
        match self {
            Self::Raw(em) => match email_regex.is_match(&em) {
                true => Self::Validated(em),
                false => Self::Invalid(em)
            },
            _ => self
        }

    }

        /// todo: need to actually verify token
        pub fn verify(self, token: i32) -> Self {
        match self {
            Self::Validated(em) => Self::Verified(em),
            _ => self
        }
    }

    pub fn process(email: String) -> Self {
        Self::new(email).validate()
    }

}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn email_returns_validated_email_address() {
        let address = "someone@someplace.com".to_string();
        let email = Email::process(address);
        match email {
            Email::Validated(addr) => assert!(true),
            _ => assert!(false, "email was invalid")
        }
    }

    #[test]
    fn email_returns_invalid_email_address() {
        let address = "someone_someplace.com".to_string();
        let email = Email::process(address);
        match email {
            Email::Invalid(addr) => assert!(true),
            _ => assert!(false, "email was invalid")
        }
    }


}