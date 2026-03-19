use thiserror::Error;

#[derive(Error, Debug, PartialEq)]
enum OneDigiterError {
    #[error(
        "Invalid: Not one digit. Value should be between {0} and {1} to be valid.",
        0,
        9
    )]
    NotOneDigit,
}

fn success_if_one_digit(val: u32) -> Result<u32, OneDigiterError> {
    if (0..=9).contains(&val) {
        Ok(val)
    } else {
        Err(OneDigiterError::NotOneDigit)
    }
}

#[derive(Error, Debug, PartialEq)]
enum ChangeConfigError {
    #[error("old string is empty")]
    NewIsEmpty,
    #[error("old string is invalid")]
    OldIsInvalid,
    #[error("new string is invalid")]
    NewIsInvalid,
    #[error("new value is same as old value")]
    NewIsSameAsOld,
}

fn change_config(old_str: &str, new_str: &str) -> Result<u32, ChangeConfigError> {
    if old_str.is_empty() {
        return Err(ChangeConfigError::NewIsEmpty);
    }

    let old: u32 = old_str.parse().or(Err(ChangeConfigError::OldIsInvalid))?;
    let new: u32 = new_str.parse().or(Err(ChangeConfigError::NewIsInvalid))?;

    if old == new {
        return Err(ChangeConfigError::NewIsSameAsOld);
    }

    Ok(new)
}

mod tests {
    use super::*;

    #[test]
    fn test_1() {
        assert!(success_if_one_digit(1).is_ok_and(|val| val == 1));
        assert!(success_if_one_digit(10).is_err_and(|err| err == OneDigiterError::NotOneDigit));
    }

    #[test]
    fn test_2() {
        assert!(change_config("1", "2").is_ok_and(|val| val == 2));
        assert!(change_config("1", "1").is_err_and(|err| err == ChangeConfigError::NewIsSameAsOld));
    }
}
