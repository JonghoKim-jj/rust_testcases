mod multiplier {
    pub fn run_mult(a: u32, b: u32) -> u64 {
        (a * b).into()
    }
}

#[cfg(test)]
mod tests {
    use crate::mult::multiplier::run_mult;
    use rstest::{fixture, rstest};

    const A: u32 = 4;
    const B: u32 = 5;
    const C: u64 = 20;

    const DIRTY_THING: Option<u64> = Some(56);
    const P: u32 = 7;
    const Q: u32 = (1 + 2) * 3 + 4 - 5;

    // Rust can not use unwrap() in const context.
    // Thus, if you really want to unwrap,
    // you may use match statement and gives un-evaluatable value such as [][0]
    const R: u64 = match DIRTY_THING {
        Some(v) => v,
        None => [][0],
    };

    #[test]
    fn test_run_mult1() {
        let a: u32 = 4;
        let b: u32 = 5;
        let c: u64 = 20;

        assert_eq!(run_mult(a, b), c)
    }

    #[test]
    fn test_run_mult2() {
        assert_eq!(run_mult(A, B), C)
    }

    #[test]
    fn test_run_mult3() {
        assert_eq!(run_mult(P, Q), R)
    }

    // Tests using *rstest*.

    #[fixture]
    fn p_as_fixture() -> u32 {
        7
    }

    #[fixture]
    fn q_as_fixture() -> u32 {
        (1 + 2) * 3 + 4 - 5
    }

    // Rust const context is hard to code.
    // for example, you can not use unwrap() in const context.
    // With *rstest*, you can easily build fixtures.
    #[fixture]
    fn r_as_fixture() -> u64 {
        DIRTY_THING.unwrap()
    }

    #[rstest]
    #[case(4, 5, 20)]
    #[case(A, B, C)]
    fn test_with_rstest(#[case] a: u32, #[case] b: u32, #[case] c: u64) {
        assert_eq!(run_mult(a, b), c)
    }

    #[rstest]
    fn test_with_rstest_fixture(
        #[from(p_as_fixture)] p: u32,
        #[from(q_as_fixture)] q: u32,
        #[from(r_as_fixture)] r: u64,
    ) {
        assert_eq!(run_mult(p, q), r)
    }
}
