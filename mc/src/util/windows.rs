const SCALE_FACTOR: usize = 36;

const SMALL_WINDOW: usize = SCALE_FACTOR;
const MEDIUM_WINDOW: usize = SMALL_WINDOW * SCALE_FACTOR;
const LARGE_WINDOW: usize = MEDIUM_WINDOW * SCALE_FACTOR;

pub fn create_available_windows(n: usize) -> Vec<usize> {
    vec![LARGE_WINDOW, MEDIUM_WINDOW, SMALL_WINDOW]
        .into_iter()
        .filter(|x| x <= &n)
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn create_available_windows_returns_filters_windows_that_are_to_long() {
        let available_windows = create_available_windows(1296);
        assert_eq!(available_windows, vec![MEDIUM_WINDOW, SMALL_WINDOW]);
    }

    #[test]
    fn create_available_windows_returns_empty_vec_when_no_windows_available() {
        let available_windows = create_available_windows(1);
        assert_eq!(available_windows, Vec::<usize>::new());
    }
}
