mod ode;
mod common;
use ode::euler;
use common::TwoComponent;


fn main() {
   let mut x: Vec<TwoComponent> = Vec::with_capacity(100);
   let mut t: Vec<f64>          = Vec::with_capacity(100);
   let dt: f64 = 0.01;
   let params: Vec<f64> = Vec::new();
   x.push(TwoComponent{A: 1000_f64, B: 0_f64});
   t.push(0_f64);
   euler::euler_solver_2component(&mut x, &mut t, f1, f1, &params, dt, 100);
   for (i, x) in x.iter().enumerate(){
       println!("t: {}\tA: {}\tB: {}", t[i], x.A, x.B);
   }
}


fn f1(x: &TwoComponent, t: f64, params: &Vec<f64>) -> f64{
    x.A
}
