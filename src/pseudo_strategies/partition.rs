pub fn get_partitions(window: usize, series_len: usize) -> [usize; 7] {
    let step = window / 6;
    return [
        series_len - 6 * step,
        series_len - 5 * step,
        series_len - 4 * step,
        series_len - 3 * step,
        series_len - 2 * step,
        series_len - step,
        series_len
    ];
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn get_partitions_return_exact_partitions_when_window_equals_series_len() {
        let window = 36 as usize;
        let series_len = 36 as usize;

        let result = get_partitions(window, series_len);
        let expected = [0, 6, 12, 18, 24, 30, 36];

        assert_eq!(result, expected);
    }

    #[test]
    fn get_partitions_return_partitions_that_cover_only_last_series_indices() {
        let window = 36 as usize;
        let series_len = 72 as usize;

        let result = get_partitions(window, series_len);
        let expected = [36, 42, 48, 54, 60, 66, 72];

        assert_eq!(result, expected);
    }
}
