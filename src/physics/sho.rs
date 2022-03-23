pub fn ftheta(x: &Vec<f64>, t: f64, params: &Vec<f64>) -> f64 {
    x[1]
}

pub fn fomega(x: &Vec<f64>, t: f64, params: &Vec<f64>) -> f64 {
    - params[0] / params[1] * x[0]
}
