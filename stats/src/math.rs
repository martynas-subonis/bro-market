pub fn round_up_to_1000(value: u32) -> u32 {
    ((value + 999) / 1000) * 1000
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn round_up_to_500_rounds_0_correctly() {
        let cases = [(0, 0), (1, 1000), (999, 1000), (1000, 1000), (1001, 2000)];
        for case in cases {
            assert_eq!(round_up_to_1000(case.0), case.1)
        }
    }
}
