use core;

use crate::vec3::{dot, cross, V};

#[derive(Debug, Clone)]
pub struct Ball {
    pub x: V,
    pub v: V,
    pub m: f64,
    pub r: f64,
}

#[derive(Debug, Clone)]
pub struct Wall {
    pub x: V,
    pub v: V,
    pub j: V,
    pub k: V,
    pub m: f64,
}

fn ball_ball_collision_time(a: &Ball, b: &Ball) -> f64 {
    // go in frame of `a`
    let x = b.x - a.x;
    let v = b.v - a.v;
    let r = a.r + b.r;
    let xv = dot(x, v);
    if xv >= 0.0 {
        core::f64::INFINITY
    } else {
        let vv = dot(v, v);
        let dis = xv * xv - vv * (dot(x, x) - r * r);
        if dis > 0.0 {
            (-xv - dis.sqrt()) / vv
        } else {
            core::f64::INFINITY
        }
    }
}

fn ball_wall_collision_time(a: &Ball, w: &Wall) -> f64 {
    // do in frame of `w`
    let mut x = a.x - w.x;
    let v = a.v - w.v;
    let mut n = cross(w.j, w.k);
    let njck = n.norm();
    n /= njck;
    let vn = dot(v, n);
    let xn = dot(x, n);
    if xn * vn < 0.0 {
        let t = -xn / vn - (a.r / vn).abs();
        x += t * v;
        let xcn = cross(x, n);
        let a = -dot(xcn, w.k) / njck;
        let b = dot(xcn, w.j) / njck;
        if a < 0.0 || a > 1.0 || b < 0.0 || b > 1.0 {
            core::f64::INFINITY
        } else {
            t
        }
    } else {
        core::f64::INFINITY
    }
}

fn collision(n: V, mut va: V, ma: f64, mut vb: V, mb: f64) -> (V, V) {
    if ma.is_infinite() {
        return collision(n, vb, mb, va, ma);
    }
    let fv = if mb.is_infinite() {
        vb
    } else {
        (ma * va + mb * vb) / (ma + mb)
    };
    // go in rest frame
    va -= fv;
    vb -= fv;
    va -= 2.0 * dot(n, va) * n / dot(n, n);
    vb = -ma / mb * va;
    // go back in original frame
    va += fv;
    vb += fv;
    (va, vb)
}

fn ball_ball_collision(a: &Ball, b: &Ball) -> (V, V) {
    let n = b.x - a.x;
    collision(n, a.v, a.m, b.v, b.m)
}

fn ball_wall_collision(a: &Ball, w: &Wall) -> (V, V) {
    let n = cross(w.j, w.k);
    collision(n, a.v, a.m, w.v, w.m)
}

pub fn evolve(
    mut balls: Vec<Ball>,
    mut walls: Vec<Wall>,
    mut dt: f64,
) -> (Vec<Ball>, Vec<Wall>, f64, f64) {
    let mut bi = -1;
    let mut bj = -1;
    let mut w = -1;

    for i in 0..balls.len() {
        for j in i + 1..balls.len() {
            let tmp = ball_ball_collision_time(&balls[i], &balls[j]);
            if tmp < dt {
                bi = i as isize;
                bj = j as isize;
                w = -1;
                dt = tmp;
            }
        }
        for j in 0..walls.len() {
            let tmp = ball_wall_collision_time(&balls[i], &walls[j]);
            if tmp < dt {
                bi = i as isize;
                bj = -1;
                w = j as isize;
                dt = tmp;
            }
        }
    }

    if dt > 0.0 {
        for a in &mut balls {
            a.x += a.v * dt;
        }
        for w in &mut walls {
            w.x += w.v * dt;
        }
    }

    let mut work = 0.0;
    if bj >= 0 {
        let (va, vb) = ball_ball_collision(&balls[bi as usize], &balls[bj as usize]);
        balls[bi as usize].v = va;
        balls[bj as usize].v = vb;
    }
    if w >= 0 {
        let a = &mut balls[bi as usize];
        let w = &mut walls[w as usize];
        let (va, vw) = ball_wall_collision(a, w);
        work = dot(a.m * (va - a.v), w.v);
        a.v = va;
        w.v = vw;
    }

    (balls, walls, dt, work)
}
