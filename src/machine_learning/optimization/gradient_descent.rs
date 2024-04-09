/// 梯度下降优化
///
/// # Arguments
///
/// * `derivative_fn` - 计算梯度函数
/// * `x` - 要优化的初始化向量参数
/// * `learning_rate` - 学习率
/// * `num_iterations` - 迭代轮数
///
/// # Returns
///
/// 对优化参数向量`x`的引用

pub fn gradient_descent(derivative_fn: fn(&[f64]) -> Vec<f64>, x: &mut Vec<f64>, learning_rate: f64, num_iterations: i32) -> &mut Vec<f64> {
    for _ in 0..num_iterations {
        let gradient = derivative_fn(x);
        for (x_i, grad_i) in x.iter_mut().zip(gradient.iter()) {
            *x_i -= learning_rate * grad_i;
        }
    }
    x
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_gradient_descent_optimized() {
        fn derivative_of_square(params: &[f64]) -> Vec<f64> {
            params.iter().map(|x| 2. * x).collect()
        }

        let mut x: Vec<f64> = vec![5.0, 6.0];
        let learning_rate: f64 = 0.03;
        let num_iterations: i32 = 1000;

        let minimized_vector =
            gradient_descent(derivative_of_square, &mut x, learning_rate, num_iterations);

        let test_vector = [0.0, 0.0];

        let tolerance = 1e-6;
        for (minimized_value, test_value) in minimized_vector.iter().zip(test_vector.iter()) {
            assert!((minimized_value - test_value).abs() < tolerance);
        }
    }

    #[test]
    fn test_gradient_descent_unoptimized() {
        fn derivative_of_square(params: &[f64]) -> Vec<f64> {
            params.iter().map(|x| 2. * x).collect()
        }

        let mut x: Vec<f64> = vec![5.0, 6.0];
        let learning_rate: f64 = 0.03;
        let num_iterations: i32 = 10;

        let minimized_vector =
            gradient_descent(derivative_of_square, &mut x, learning_rate, num_iterations);

        let test_vector = [0.0, 0.0];

        let tolerance = 1e-6;
        for (minimized_value, test_value) in minimized_vector.iter().zip(test_vector.iter()) {
            assert!((minimized_value - test_value).abs() >= tolerance);
        }
    }
}