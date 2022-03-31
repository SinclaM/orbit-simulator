use rand::Rng;
use std::fs::File; use std::io::{BufRead, BufReader}; pub fn dist(x1: f64, x2: f64, y1: f64, y2: f64) -> f64 { ((x1 - x2).powf(2.0) + (y1 - y2).powf(2.0)).powf(0.5) }

pub struct CelestialBody{
    pub x_pos: f64, // AU
    pub y_pos: f64, // AU
    pub v_x: f64,   // AU / yr
    pub v_y: f64,   // AU / yr
    pub mass: f64   // Solar masses
}

impl CelestialBody{
    pub fn new(x_pos: f64, y_pos: f64, v_x: f64, v_y: f64, mass: f64)
        -> CelestialBody {
            CelestialBody{
                x_pos, y_pos, v_x, v_y, mass
            } }
    pub fn new_random(x_lim: f64, y_lim: f64, v_x_lim: f64, v_y_lim: f64, 
                      mass_lim: f64) 
        -> CelestialBody {
            let mut rng = rand::thread_rng();
            CelestialBody{
                x_pos: rng.gen_range(-x_lim..=x_lim),
                y_pos: rng.gen_range(-y_lim..=y_lim),
                v_x: rng.gen_range(-v_x_lim..=v_x_lim),
                v_y: rng.gen_range(-v_y_lim..=v_y_lim),
                mass: rng.gen_range(0.0..=mass_lim),
            }
    }

    pub fn dist_to(&self, other: &CelestialBody) -> f64 {
        ((self.x_pos - other.x_pos).powf(2.0) + (self.y_pos - other.y_pos).powf(2.0))
        .sqrt()
    }

    pub fn gen_vars(bodies: &Vec<CelestialBody>) -> Vec<f64>{
        let mut vars: Vec<f64> = Vec::with_capacity(4 * bodies.len());
        for body in bodies{
            vars.push(body.x_pos);
            vars.push(body.y_pos);
            vars.push(body.v_x);
            vars.push(body.v_y);
        }
        vars
    }

    pub fn gen_derivates<'a>(bodies: &'a Vec<CelestialBody>)
        -> Vec<impl Fn(&Vec<f64>, f64, &Vec<f64>) -> f64 + 'a> 
    {
        let mut dxdt: Vec<Box<dyn Fn(&Vec<f64>, f64, &Vec<f64>) -> f64>> 
            = Vec::with_capacity(bodies.len() * 4);

        let mut i: usize = 0;
        for body_i in bodies{
            // x
            dxdt.push(
                Box::new(
                    move |vars, t, params| {
                        vars[4 * i + 2]
                    }
                )
            );
            // y
            dxdt.push(
                Box::new(
                    move |vars, t, params| {
                        vars[4 * i + 3]
                    }
                )
            );
            // v_x
            dxdt.push(
                Box::new(
                    move |vars, t, params| {
                        let mut j: usize = 0;
                        let mut a: f64 = 0.0;
                        for body_j in bodies{
                            if i != j {
                                a += - params[0] * body_j.mass 
                                    * (vars[4 * i] - vars[4 * j])
                                    / (dist(vars[4 * i], vars[4 * j], 
                                            vars[4 * i + 1], vars[4 * j + 1]))
                                    .powf(3.0);
                            }
                            j += 1;
                        }
                        a
                    }
                )
            );

            // v_y
            dxdt.push(
                Box::new(
                    move |vars, t, params| {
                        let mut j: usize = 0;
                        let mut a: f64 = 0.0;
                        for body_j in bodies{
                            if i != j {
                                a += - params[0] * body_j.mass 
                                    * (vars[4 * i + 1] - vars[4 * j + 1])
                                    / (dist(vars[4 * i], vars[4 * j], 
                                            vars[4 * i + 1], vars[4 * j + 1]))
                                    .powf(3.0);
                            }
                            j += 1;
                        }
                        a
                    }
                )
            );

            i += 1;
        }

        dxdt
    }

    // second argument is vector of bodies with initial positions and
    // velocities but the same masses to refer to
    pub fn regen_bodies(vars: &Vec<f64>, initial_bodies: &Vec<CelestialBody>) 
        -> Vec<CelestialBody> {
            let mut bodies: Vec<CelestialBody> = Vec::with_capacity(vars.len() / 4);
            for i in (0..(vars.len())).step_by(4){
                bodies.push(CelestialBody::new(vars[i],                      // x_pos
                                               vars[i + 1],                  // y_pos
                                               vars[i + 2],                  // v_x
                                               vars[i + 3],                  // v_y
                                               initial_bodies[i / 4].mass)); // mass
            }
            bodies
    }

    pub fn print_bodies(bodies: &Vec<CelestialBody>, t: f64){
        for body in bodies{
            println!("t: {} x: {} y: {} v_x: {} v_y: {} mass: {}", 
                t, body.x_pos, body.y_pos, body.v_x, body.v_y, body.mass);
        }
    }

    pub fn read_bodies(fname: String) -> (Vec<CelestialBody>, f64){
        let file = File::open(fname).unwrap();
        let reader = BufReader::new(file);

        let mut bodies: Vec<CelestialBody> = Vec::new();
        let mut t = 0.0;
        for (_, line) in reader.lines().enumerate() {
            let line = line.unwrap(); // Ignore errors.
            let (tmp_t, x, y, v_x, v_y, mass) = 
                scan_fmt!(&line, 
                          "t: {} x: {} y: {} v_x: {} v_y: {} mass: {}", 
                          f64, f64, f64, f64, f64, f64)
                .expect("Failed to scan");
            bodies.push(CelestialBody::new(x, y, v_x, v_y, mass));
            t = tmp_t;
        }
        (bodies, t)
    }
}
