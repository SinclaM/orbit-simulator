use crate::common::TwoComponent;

pub fn euler_step_2component(x: &mut Vec<TwoComponent>, t: &mut Vec<f64>,
                             fA: f64, fB: f64, dt: f64) {
    x.push(TwoComponent{
            A: x.last().unwrap().A + fA * dt, 
            B: x.last().unwrap().B + fB * dt
           });
    t.push(t.last().unwrap() + dt);
}

pub fn euler_solver_2component(x: &mut Vec<TwoComponent>, t: &mut Vec<f64>,
                               dxAdt: fn(&TwoComponent, f64, &Vec<f64>) -> f64,
                               dxBdt: fn(&TwoComponent, f64, &Vec<f64>) -> f64,
                               params: &Vec<f64>,
                               dt: f64, steps: usize){
    for i in 0..(steps - 1){
        let fA = dxAdt(&x[i], t[i], params);
        let fB = dxBdt(&x[i], t[i], params);
        euler_step_2component(x, t, fA, fB, dt);
    }
}
