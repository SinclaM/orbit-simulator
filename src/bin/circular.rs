use orbits::ode::{OdeSolver, SolverType};
use orbits::physics::celestial::CelestialBody;
use std::path::Path;

fn main() {
    const NUM_BODIES: usize = 100;
    const NUM_STEPS: usize = 2;
    const DT: f64 = 0.005;
    let params: Vec<f64> = vec![39.478 // G
                                ];
    let fname = String::from("data/bodies.dat");
    let mut bodies: Vec<CelestialBody>;
    let t_initial: f64;

    if Path::new(&fname).exists() {
        (bodies, t_initial) = CelestialBody::read_bodies(fname);
    } else {
        t_initial = 0.0;
        bodies = Vec::with_capacity(NUM_BODIES);
        for _ in 0..NUM_BODIES {
            bodies.push(CelestialBody::new_random(10.0, 10.0, 50.0, 50.0, 5.0));
        }
    }

    // Add a black hole
    bodies.pop();
    bodies.push(CelestialBody::new(0.0, 0.0, 0.0, 0.0, 1000.0));

    // Load Ode Solver
    let mut x: Vec<Vec<f64>> = Vec::with_capacity(2);
    x.push(CelestialBody::gen_vars(&bodies));

    let mut t: Vec<f64> = Vec::with_capacity(2);
    t.push(t_initial);

    let dxdt = CelestialBody::gen_derivates(&bodies);

    let mut solver = OdeSolver::new(&mut x, &mut t, &dxdt, &params, DT, NUM_STEPS,
                               SolverType::Rk4);

    solver.solve();

    let bodies = CelestialBody::regen_bodies(&x[NUM_STEPS - 1], &bodies);
    CelestialBody::print_bodies(&bodies, t[NUM_STEPS - 1]);
}
