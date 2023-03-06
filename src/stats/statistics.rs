pub fn mean(nums: &Vec<f64>) -> f64 {
    let s: f64 = nums.iter().sum();
    return s / nums.len() as f64;
}

pub fn std(nums: &Vec<f64>) -> f64 {
    let nums_clone = nums.clone();
    let m = mean(nums);
    let diff_square: Vec<f64> = nums_clone.iter().map(|x| f64::powi(x - m, 2)).collect();
    return f64::sqrt(diff_square.iter().sum::<f64>() / nums_clone.len() as f64);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn mean_calculates_mean_correctly() {
        let vec = vec![0.0, 1.0];
        let m = mean(&vec);
        assert_eq!(m, 0.5);
    }

    #[test]
    fn std_calculates_std_correctly() {
        let vec = vec![-2.0, 2.0];
        let s = std(&vec);
        assert_eq!(s, 2.0);
    }
}
