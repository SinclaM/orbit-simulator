pub fn euler_step_2component(x: &mut Vec<Vec<f64>>, t: &mut Vec<f64>, f: &Vec<f64>, dt: f64) {
    let mut updated = Vec::with_capacity(x[0].capacity());
    for i in 0..updated.capacity() {
        updated.push(x.last().unwrap()[i] + f[i] * dt);
    }
    x.push(updated);
    t.push(t.last().unwrap() + dt);
}

pub fn euler_solver_2component(
    x: &mut Vec<Vec<f64>>,
    t: &mut Vec<f64>,
    dxdt: &Vec<fn(&Vec<f64>, f64, &Vec<f64>) -> f64>,
    params: &Vec<f64>,
    dt: f64,
    steps: usize,
) {
    for i in 0..(steps - 1) {
        let mut f = Vec::with_capacity(dxdt.capacity());
        for j in 0..(f.capacity()) {
            f.push(dxdt[j](&x[i], t[i], &params));
        }
        euler_step_2component(x, t, &f, dt);
    }
}
