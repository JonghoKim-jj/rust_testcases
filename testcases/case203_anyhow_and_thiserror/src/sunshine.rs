use anyhow::{Result, anyhow, bail};

// Returns Ok(val) if val is one digit (0-9)
// Else returns Err
fn success_if_one_digit(val: u32) -> Result<u32> {
    if (0..=9).contains(&val) {
        Ok(val)
    } else {
        Err(anyhow!("Fail - Not one digit"))
    }
}

fn change_config(old_str: &str, new_str: &str) -> Result<u32> {
    if old_str.is_empty() {
        bail!("old string is empty");
    }

    let old: u32 = old_str.parse().or(Err(anyhow!("old string is invalid")))?;
    let new: u32 = new_str.parse().or(Err(anyhow!("new string is invalid")))?;

    if old == new {
        return Err(anyhow!("new value is same as old value"));
    }

    Ok(new)
}

mod tests {
    use super::*;

    #[test]
    fn test_1() {
        assert!(success_if_one_digit(1).is_ok_and(|val| val == 1));
        assert!(
            success_if_one_digit(10).is_err_and(|err| err.to_string() == "Fail - Not one digit")
        );
    }

    #[test]
    fn test_2() {
        assert!(change_config("1", "2").is_ok_and(|val| val == 2));
        assert!(
            change_config("1", "1")
                .is_err_and(|err| err.to_string() == "new value is same as old value")
        );
    }
}
