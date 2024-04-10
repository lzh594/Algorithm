use std::f64::consts::E;

pub fn sigmoid(x: &f64) -> f64 {
    1. / (1. + E.powf(-1. * x))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sigmoid() {
        let test = Vec::from([1.0, 0.5, -1.0, 0.0, 0.3]);
        assert_eq!(
            test.iter().map(|x| { sigmoid(x) }).collect::<Vec<f64>>(),
            vec![0.7310585786300049, 0.6224593312018546, 0.2689414213699951, 0.5, 0.574442516811659]
        );
    }
}