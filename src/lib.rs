#[macro_use]
extern crate glium;

mod dynamics;
mod gl;
mod vec3;

use core;
use dynamics::{evolve, Ball, Wall};
use vec3::V;

use pyo3::prelude::{
    pyclass, pyfunction, pymethods, pymodule, PyModule, PyResult, Python,
};
use pyo3::wrap_pyfunction;

#[pymodule]
fn gas(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_wrapped(wrap_pyfunction!(visualize_example))?;
    m.add_class::<State>()?;
    Ok(())
}

#[pyclass]
#[derive(Default)]
pub struct State {
    balls: Vec<Ball>,
    walls: Vec<Wall>,
}

#[pymethods]
impl State {
    #[new]
    fn new() -> Self {
        State::default()
    }

    fn add_ball(&mut self, position: (f64, f64, f64), speed: (f64, f64, f64), m: f64, r: f64) {
        self.balls.push(Ball {
            x: V::new(position.0, position.1, position.2),
            v: V::new(speed.0, speed.1, speed.2),
            m: m,
            r: r,
        })
    }

    fn add_wall(
        &mut self,
        position: (f64, f64, f64),
        speed: (f64, f64, f64),
        j: (f64, f64, f64),
        k: (f64, f64, f64),
        m: f64,
    ) {
        self.walls.push(Wall {
            x: V::new(position.0, position.1, position.2),
            v: V::new(speed.0, speed.1, speed.2),
            j: V::new(j.0, j.1, j.2),
            k: V::new(k.0, k.1, k.2),
            m: m,
        })
    }

    fn visualize(&self) {
        let mut balls = self.balls.clone();
        let mut walls = self.walls.clone();

        gl::window::visualize(move || {
            let old_balls = balls.clone();
            let old_walls = walls.clone();

            let (dt, _work) = evolve(&mut balls, &mut walls, 10.0);
            (dt, old_balls, old_walls)
        });
    }
}

#[pyfunction]
fn visualize_example() {
    let mut balls = Vec::new();

    for &x in &[-2.0, -1.0, 0.0, 1.0, 2.0, 3.0] {
        for &y in &[-2.0, -1.0, 0.0, 1.0, 2.0, 3.0] {
            for &z in &[-2.0, -1.0, 0.0, 1.0, 2.0] {
                balls.push(Ball {
                    x: V::new(x * 0.201, y * 0.201, z * 0.201),
                    v: V::new(y * 0.1, z * 0.1, x * 0.1),
                    m: 1.0,
                    r: 0.1,
                });
            }
        }
    }

    let a = 2.0;
    let mut walls = vec![
        Wall {
            x: V::new(a, -a, -a),
            v: V::new(0.0, 0.0, 0.0),
            j: V::new(0.0, 2.0 * a, 0.0),
            k: V::new(0.0, 0.0, 2.0 * a),
            m: core::f64::INFINITY,
        },
        Wall {
            x: V::new(-a, -a, -a),
            v: V::new(0.0, 0.0, 0.0),
            j: V::new(0.0, 2.0 * a, 0.0),
            k: V::new(0.0, 0.0, 2.0 * a),
            m: core::f64::INFINITY,
        },
        Wall {
            x: V::new(-a, -a, -a),
            v: V::new(0.0, 0.0, 0.0),
            j: V::new(2.0 * a, 0.0, 0.0),
            k: V::new(0.0, 0.0, 2.0 * a),
            m: core::f64::INFINITY,
        },
        Wall {
            x: V::new(-a, a, -a),
            v: V::new(0.0, 0.0, 0.0),
            j: V::new(2.0 * a, 0.0, 0.0),
            k: V::new(0.0, 0.0, 2.0 * a),
            m: core::f64::INFINITY,
        },
        Wall {
            x: V::new(-a, -a, -a),
            v: V::new(0.0, 0.0, 0.0),
            j: V::new(2.0 * a, 0.0, 0.0),
            k: V::new(0.0, 2.0 * a, 0.0),
            m: core::f64::INFINITY,
        },
        Wall {
            x: V::new(-a, -a, a),
            v: V::new(0.0, 0.0, 0.0),
            j: V::new(2.0 * a, 0.0, 0.0),
            k: V::new(0.0, 2.0 * a, 0.0),
            m: core::f64::INFINITY,
        },
    ];

    gl::window::visualize(move || {
        // let mut e = 0.0;
        // for a in &balls {
        //     e += 0.5 * a.m * a.v.norm().powi(2);
        // }
        // println!("energy {:.6}", e);

        // balls = new_balls.clone();
        // walls = new_walls.clone();

        let old_balls = balls.clone();
        let old_walls = walls.clone();

        let (dt, _work) = evolve(&mut balls, &mut walls, 10.0);

        // println!("{:.4}", dt);

        (dt, old_balls, old_walls)
    });
}
