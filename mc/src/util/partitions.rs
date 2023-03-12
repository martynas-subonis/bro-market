pub fn get_partitions(window: usize, series_len: usize, partitions_num: usize) -> Vec<usize> {
    if window % partitions_num != 0 {
        panic!(
            "{}",
            format!("Window must be divisible by {}.", partitions_num)
        );
    }

    let step = window / partitions_num;
    let mut partitions: Vec<usize> = Vec::new();

    for part in (0..=partitions_num).rev() {
        partitions.push(series_len - part * step);
    }
    return partitions;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn get_partitions_return_exact_partitions_when_window_equals_series_len() {
        let partitions_num = 6;
        let window = 36;
        let series_len = 36;

        let result = get_partitions(window, series_len, partitions_num);
        let expected: Vec<usize> = vec![0, 6, 12, 18, 24, 30, 36];

        assert_eq!(result, expected);
    }

    #[test]
    fn get_partitions_return_exact_partitions_when_window_equals_series_len_with_four_partitions() {
        let partitions_num = 4;
        let window = 36;
        let series_len = 36;

        let result = get_partitions(window, series_len, partitions_num);
        let expected: Vec<usize> = vec![0, 9, 18, 27, 36];

        assert_eq!(result, expected);
    }

    #[test]
    fn get_partitions_return_partitions_that_cover_only_last_series_indices() {
        let partitions_num = 6;
        let window = 36;
        let series_len = 72;

        let result = get_partitions(window, series_len, partitions_num);
        let expected = [36, 42, 48, 54, 60, 66, 72];

        assert_eq!(result, expected);
    }

    #[test]
    fn get_partitions_return_partitions_that_cover_only_last_series_indices_with_four_partitions() {
        let partitions_num = 4;
        let window = 36;
        let series_len = 72;

        let result = get_partitions(window, series_len, partitions_num);
        let expected = [36, 45, 54, 63, 72];

        assert_eq!(result, expected);
    }

    #[test]
    #[should_panic]
    fn get_partitions_return_error_when_window_is_not_divisible_by_partitions_num() {
        let partitions_num = 6;
        let window = 37;
        let series_len = 72;

        get_partitions(window, series_len, partitions_num);
    }
}
