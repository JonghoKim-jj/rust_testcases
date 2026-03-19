use anyhow::{Result, bail};

fn safe_addition(a: u32, b: u32) -> Result<u32> {
    match a.checked_add(b) {
        Some(c) => Ok(c),
        None => bail!("Overflow"),
    }
}

#[cfg(test)]
mod tests {
    use rstest::*;

    use super::*;

    #[fixture]
    fn sample_a() -> u32 {
        7
    }

    #[fixture]
    fn sample_b() -> u32 {
        8
    }

    #[rstest]
    #[case::basic(1, 2, 3)]
    #[case::another_one(4, 5, 9)]
    #[case::sample(sample_a(), sample_b(), 15)]
    fn test_safe_addition(#[case] a: u32, #[case] b: u32, #[case] expected: u32) {
        let result = safe_addition(a, b).expect("Should success");
        assert_eq!(result, expected);
    }

    #[rstest]
    #[case::overflow1(u32::MAX, u32::MAX, "Overflow")]
    #[case::overflow2(u32::MAX, 1, "Overflow")]
    fn fail_safe_addition(#[case] a: u32, #[case] b: u32, #[case] expected_msg: &str) {
        let result = safe_addition(a, b).expect_err("Should fail");
        assert_eq!(result.to_string(), expected_msg);
    }
}
