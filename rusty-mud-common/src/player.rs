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
    pub fn new(email: String) -> Email {
        Email::Raw(email)
    }

    pub fn validate(self) -> Email {
        let email_regex = Regex::new(r"^([a-z0-9_+]([a-z0-9_+.]*[a-z0-9_+])?)@([a-z0-9]+([\-\.]{1}[a-z0-9]+)*\.[a-z]{2,6})").unwrap();
        match self {
            Email::Raw(em) => {
                match email_regex.is_match(&em) {
                    true => Email::Validated(em.to_owned()),
                    false => Email::Invalid(em.to_owned())
                }
            },
            _ => self
        }
    }

    pub fn process(email: String) -> Email {
        Email::new(email).validate()
    }

}

#[derive(Debug)]
pub enum Password {
    Hashed(String, u8, CrackTimes),
    Invalid(String, u8),
}

impl Password {
    pub fn new(password: String) -> Password {
        let estimate = zxcvbn(&password, &[]).unwrap();
        if estimate.score() > 2{
            Password::Hashed(bcrypt::hash(password).unwrap(), estimate.score(), estimate.crack_times())
        } else {
            Password::Invalid(password, estimate.score())
        }
    }
}


pub struct PlayerAccount {
    id: i32,
    email: Email,
    password: Password
}



#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn password_returns_hashed_password() {
        let pw = Password::new("ReasonablyGoodPassword!!$$".to_string());
        match pw {
            Password::Hashed(_) => assert!(true),
            _ => assert!(false, "{}", std::format!("{:#?}", pw)),
        }
    }
    
    #[test]
    fn password_returns_invalid_password() {
        let pw = Password::new("bill".to_string());
        match pw {
            Password::Hashed(_) => assert!(false, "{}", std::format!("{:#?}", pw)),
            _ => assert!(true),
        }
    }

    #[test]
    fn email_returns_validated_email_address() {
        let address = "someone@someplace.com".to_string();
        let email = Email::process(address);
        match email {
            Email::Validated(_) => assert!(true),
            _ => assert!(false)
        }
    }

    #[test]
    fn email_returns_invalid_email_address() {
        let address = "someone@someplace/com".to_string();
        let email = Email::process(address);
        match email {
            Email::Invalid(_) => assert!(true),
            _ => assert!(false)
        }
    }
}
