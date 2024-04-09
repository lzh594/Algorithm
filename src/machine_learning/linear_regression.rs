pub fn linear_regression(data_points: Vec<(f64, f64)>) -> Option<(f64, f64)> {
    if data_points.is_empty() {
        return None;
    }
    let cnt = data_points.len() as f64;
    let x_mean = data_points.iter().fold(0.0, |sum, p| { sum + p.0 }) / cnt;
    let y_mean = data_points.iter().fold(0.0, |sum, p| { sum + p.1 }) / cnt;
    let mut covariance = 0.0;
    let mut std_dev_sqr_x = 0.0;
    let mut std_dev_sqr_y = 0.0;
    for data_point in data_points {
        covariance += (data_point.0 - x_mean) * (data_point.0 - y_mean);
        std_dev_sqr_x += (data_point.0 - x_mean).powi(2);
        std_dev_sqr_y += (data_point.1 - y_mean).powi(2);
    };
    let std_dev_x = std_dev_sqr_x.sqrt();
    let std_dev_y = std_dev_sqr_y.sqrt();
    let std_dev_prod = std_dev_x * std_dev_y;
    // Pearson's correlation constant
    let r = covariance / std_dev_prod;
    let slope = r * (std_dev_y / std_dev_x);
    let b = y_mean - slope * x_mean;
    Some((slope, b))
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_linear_regression() {
        assert_eq!(
            linear_regression(vec![(0.0, 0.0), (1.0, 1.0), (2.0, 2.0)]),
            Some((0.9999999999999998, 2.220446049250313e-16))
        );
    }

    #[test]
    fn test_empty_list_linear_regression() {
        assert_eq!(linear_regression(vec![]), None);
    }
}