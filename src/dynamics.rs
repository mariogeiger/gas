use core;

use crate::vec3::{dot, V};

#[derive(Debug)]
pub struct Ball {
    pub x: V,
    pub v: V,
    pub m: f64,
    pub r: f64,
}

#[derive(Debug)]
pub struct Wall {
    pub x: V,
    pub v: V,
    pub n: V,
    pub m: f64,
}

fn ball_ball_collision_time(a: &Ball, b: &Ball) -> f64 {
    // go in frame of `a`
    let x = b.x - a.x;
    let v = b.v - a.v;
    let r = a.r + b.r;
    if dot(x, v) >= 0.0 {
        core::f64::INFINITY
    } else {
        // println!("x = {:?}\nv = {:?}", x, v);
        let dis = dot(x, v).powi(2) - dot(v, v) * (dot(x, x) - r * r);
        if dis > 0.0 {
            (-dot(x, v) - dis.sqrt()) / dot(v, v)
        } else {
            core::f64::INFINITY
        }
    }
}

fn ball_wall_collision_time(a: &Ball, w: &Wall) -> f64 {
    // do in frame of `w`
    let x = a.x - w.x;
    let v = a.v - w.v;
    let vn = dot(v, w.n);
    let xn = dot(x, w.n);
    if xn * vn < 0.0 {
        -xn / vn - (a.r / vn).abs()
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
    va -= 2.0 * dot(n, va) * n;
    vb = -ma / mb * va;
    // go back in original frame
    va += fv;
    vb += fv;
    (va, vb)
}

fn ball_ball_collision(a: &Ball, b: &Ball) -> (V, V) {
    let mut n = b.x - a.x;
    n = n / n.norm();
    collision(n, a.v, a.m, b.v, b.m)
}

fn ball_wall_collision(a: &Ball, w: &Wall) -> (V, V) {
    collision(w.n, a.v, a.m, w.v, w.m)
}

pub fn evolve(balls: &mut Vec<Ball>, walls: &mut Vec<Wall>) -> f64 {
    let mut dt = core::f64::INFINITY;
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

    for i in 0..balls.len() {
        let v = balls[i].v;
        balls[i].x += v * dt;
    }
    for i in 0..walls.len() {
        let v = walls[i].v;
        walls[i].x += v * dt;
    }

    if bj >= 0 {
        let (va, vb) = ball_ball_collision(&balls[bi as usize], &balls[bj as usize]);
        balls[bi as usize].v = va;
        balls[bj as usize].v = vb;
    }
    if bj == -1 {
        let (va, vw) = ball_wall_collision(&balls[bi as usize], &walls[w as usize]);
        balls[bi as usize].v = va;
        walls[w as usize].v = vw;
    }

    dt
}
