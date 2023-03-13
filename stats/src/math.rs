pub fn round_to_precision(num: u32, p: u32) -> u32 {
    let rounded = (num / p) * p;
    return rounded;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn round_to_precision_rounds_correctly_for_500() {
        let cases = [(0, 0), (1, 0), (999, 500), (1000, 1000), (1001, 1000)];
        for case in cases {
            assert_eq!(round_to_precision(case.0, 500), case.1)
        }
    }
}
