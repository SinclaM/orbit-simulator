pub enum SolverType {
    Euler,
    Rk2,
    Rk4,
}

pub struct OdeSolver<'a> {
    x: &'a mut Vec<Vec<f64>>,
    t: &'a mut Vec<f64>,
    dxdt: &'a Vec<fn(&Vec<f64>, f64, &Vec<f64>) -> f64>,
    params: &'a Vec<f64>,
    dt: f64,
    steps: usize,
    solver_type: SolverType,
}

impl<'a> OdeSolver<'a> {
    pub fn new(
        x: &'a mut Vec<Vec<f64>>,
        t: &'a mut Vec<f64>,
        dxdt: &'a Vec<fn(&Vec<f64>, f64, &Vec<f64>) -> f64>,
        params: &'a Vec<f64>,
        dt: f64,
        steps: usize,
        solver_type: SolverType,
    ) -> OdeSolver<'a> {
        OdeSolver {
            x,
            t,
            dxdt,
            params,
            dt,
            steps,
            solver_type,
        }
    }

    fn calc_f(&self) -> Vec<f64> {
        let mut f: Vec<f64> = Vec::with_capacity(self.dxdt.capacity());

        match self.solver_type {
            SolverType::Euler => {
                for i in 0..(f.capacity()) {
                    f.push(self.dxdt[i](
                        &self.x.last().unwrap(),
                        *self.t.last().unwrap(),
                        &self.params,
                    ));
                }
            }
            SolverType::Rk2 => {
                let t_prime: f64 = self.t.last().unwrap() + 0.5 * self.dt;

                let mut x_prime: Vec<f64> = Vec::with_capacity(f.capacity());
                for i in 0..(f.capacity()) {
                    x_prime.push(
                        self.x.last().unwrap()[i]
                            + 0.5
                                * self.dxdt[i](
                                    &self.x.last().unwrap(),
                                    *self.t.last().unwrap(),
                                    &self.params,
                                )
                                * self.dt,
                    );
                }

                for i in 0..(f.capacity()) {
                    f.push(self.dxdt[i](&x_prime, t_prime, &self.params));
                }
            }
            SolverType::Rk4 => {
                let t_2prime: f64 = self.t.last().unwrap() + 0.5 * self.dt;
                let t_3prime: f64 = t_2prime; 
                let t_4prime: f64 = self.t.last().unwrap() + self.dt;

                let mut x_2prime: Vec<f64> = Vec::with_capacity(f.capacity());
                for i in 0..(f.capacity()){
                    x_2prime.push(
                        self.x.last().unwrap()[i]
                            + 0.5
                                * self.dxdt[i](
                                    &self.x.last().unwrap(),
                                    *self.t.last().unwrap(),
                                    &self.params,
                                )
                                * self.dt,
                    );
                }

                let mut f_2prime: Vec<f64> = Vec::with_capacity(f.capacity());
                for i in 0..(f.capacity()){
                    f_2prime.push(self.dxdt[i](&x_2prime, t_2prime, &self.params));
                }

                let mut x_3prime: Vec<f64> = Vec::with_capacity(f.capacity());
                for i in 0..(f.capacity()){
                    x_3prime.push(
                        self.x.last().unwrap()[i] + 0.5 * f_2prime[i] * self.dt
                    );
                }

                let mut f_3prime: Vec<f64> = Vec::with_capacity(f.capacity());
                for i in 0..(f.capacity()){
                    f_3prime.push(self.dxdt[i](&x_3prime, t_3prime, &self.params));
                }
                
                let mut x_4prime: Vec<f64> = Vec::with_capacity(f.capacity());
                for i in 0..(f.capacity()){
                    x_4prime.push(
                        self.x.last().unwrap()[i] + f_3prime[i] * self.dt
                    );
                }


                let mut f_4prime: Vec<f64> = Vec::with_capacity(f.capacity());
                for i in 0..(f.capacity()){
                    f_4prime.push(self.dxdt[i](&x_4prime, t_4prime, &self.params));
                }

                for i in 0..(f.capacity()){
                    f.push(
                        1.0 / 6.0 * (self.dxdt[i](&self.x.last().unwrap(),
                                                  *self.t.last().unwrap(),
                                                  &self.params)
                                     + 2.0 * f_2prime[i]
                                     + 2.0 * f_3prime[i]
                                     + f_4prime[i]
                                    )
                    );
                } 
            }
        }

        f
    }

    fn step(&mut self, f: &Vec<f64>) {
        let mut updated = Vec::with_capacity(self.x[0].capacity());
        for i in 0..updated.capacity() {
            updated.push(self.x.last().unwrap()[i] + f[i] * self.dt);
        }
        self.x.push(updated);
        self.t.push(self.t.last().unwrap() + self.dt);
    }

    pub fn solve(&mut self) {
        for _ in 0..(self.steps - 1) {
            self.step(&self.calc_f());
        }
    }
}
