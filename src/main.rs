#[macro_use]
extern crate glium;

mod dynamics;
mod gl;
mod vec3;

use core;
use dynamics::{evolve, Ball, Wall};
use vec3::V;

fn main() {
    let mut balls = Vec::new();
    // balls.push(Ball {
    //     x: V::new(-0.9, 0.0, 0.0),
    //     v: V::new(5.0, 1.0, 1.1),
    //     m: 1.0,
    //     r: 0.1,
    // });
    // balls.push(Ball {
    //     x: V::new(5.0, 0., 0.),
    //     v: V::new(-1.0, 0.1, 0.),
    //     m: 20.0,
    //     r: 0.15,
    // });

    for &x in &[-2.0, -1.0, 0.0, 1.0, 2.0, 3.0] {
        for &y in &[-2.0, -1.0, 0.0, 1.0, 2.0, 3.0] {
            for &z in &[-2.0, -1.0, 0.0, 1.0, 2.0] {
                balls.push(Ball {
                    x: V::new(x * 0.201, y * 0.201, z * 0.201),
                    v: V::new(y * 0.5, z * 0.5, x * 0.5),
                    m: 1.0,
                    r: 0.1,
                });
            }
        }
    }

    let a = 1.3;
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

        println!("{:.4}", dt);

        (dt, old_balls, old_walls)
    });
}
