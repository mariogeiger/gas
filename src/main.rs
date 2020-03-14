mod dynamics;
mod vec3;

use core;
use dynamics::{evolve, Ball, Wall};
use vec3::V;

fn main() {
    let mut balls = vec![
        Ball {
            x: V::new(-0.1, 0.0, 0.0),
            v: V::new(-1.0, 0.0, 0.0),
            m: 1.0,
            r: 0.1,
        },
        Ball {
            x: V::new(0.1, 0.0, 0.0),
            v: V::new(1.0, 0.0, 0.0),
            m: 1.0,
            r: 0.1,
        },
        Ball {
            x: V::new(0.0, 0.0, 0.0),
            v: V::new(0.0, 0.01, 0.0),
            m: 1.0,
            r: 0.1,
        },
    ];
    let mut walls = vec![
        Wall {
            x: V::new(1.0, 0.0, 0.0),
            v: V::new(0.0, 0.0, 0.0),
            n: V::new(1.0, 0.0, 0.0),
            m: core::f64::INFINITY,
        },
        Wall {
            x: V::new(-1.0, 0.0, 0.0),
            v: V::new(0.0, 0.0, 0.0),
            n: V::new(1.0, 0.0, 0.0),
            m: core::f64::INFINITY,
        },
        Wall {
            x: V::new(0.0, 1.0, 0.0),
            v: V::new(0.0, 0.0, 0.0),
            n: V::new(0.0, 1.0, 0.0),
            m: core::f64::INFINITY,
        },
        Wall {
            x: V::new(0.0, -1.0, 0.0),
            v: V::new(0.0, 0.0, 0.0),
            n: V::new(0.0, 1.0, 0.0),
            m: core::f64::INFINITY,
        },
        Wall {
            x: V::new(0.0, 0.0, 1.0),
            v: V::new(0.0, 0.0, 0.0),
            n: V::new(0.0, 0.0, 1.0),
            m: core::f64::INFINITY,
        },
        Wall {
            x: V::new(0.0, 0.0, -1.0),
            v: V::new(0.0, 0.0, 0.0),
            n: V::new(0.0, 0.0, 1.0),
            m: core::f64::INFINITY,
        },
    ];

    let mut t = 0.0;

    loop {
        let dt = evolve(&mut balls, &mut walls);
        t += dt;

        println!("{:.3}", dt);
        println!("{:?}", balls);

        if t > 10.0 {
            break;
        }
    }
}
