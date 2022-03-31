use orbits::ode::{OdeSolver, SolverType};
use orbits::physics::sho;



fn main() {
    const MAX: usize = 1000;
    const DT: f64 = 0.01;
    let initial: Vec<f64> = vec![1000_f64, 0_f64];

    let mut x: Vec<Vec<f64>> = Vec::new();
    let mut t: Vec<f64> = Vec::with_capacity(MAX);
    let dxdt: Vec<fn(&Vec<f64>, f64, &Vec<f64>) -> f64> = vec![sho::ftheta, sho::fomega];
    let params: Vec<f64> = vec![
        9.8, //g
        1.0,
    ]; // l
    x.push(initial);
    t.push(0_f64);
    let mut solver = OdeSolver::new(&mut x, &mut t, &dxdt, &params, DT, MAX, SolverType::Rk4);
    solver.solve();

    for (i, v) in x.iter().enumerate() {
        print!("t: {}\t", t[i]);
        for (j, _) in v.iter().enumerate() {
            print!("x_{}: {}\t", j, v[j]);
        }
        println!();
    }
}
