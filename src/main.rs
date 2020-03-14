#[macro_use]
extern crate glium;

mod dynamics;
mod gl;
mod vec3;

use core;
use dynamics::{evolve, Ball, Wall};
use gl::math::Mat4;
use gl::sphere::Sphere;
use glium::glutin;
use glium::Surface;
use vec3::V;

fn main() {
    let mut balls = vec![
        Ball {
            x: V::new(-0.1, 0.0, 0.0),
            v: V::new(-10.0, 0.0, 0.0),
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
            x: V::new(0.0, 0.1, 0.0),
            v: V::new(0.0, 0.01, 0.0),
            m: 1.0,
            r: 0.1,
        },
        Ball {
            x: V::new(0.0, 0.3, 0.0),
            v: V::new(0.0, 0.02, 0.0),
            m: 1.0,
            r: 0.1,
        },
        Ball {
            x: V::new(0.0, -0.1, 0.0),
            v: V::new(0.0, -0.01, 0.0),
            m: 1.0,
            r: 0.1,
        },
        Ball {
            x: V::new(0.0, -0.3, 0.0),
            v: V::new(0.0, -0.02, 0.0),
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

    let mut t_checkpoint = 0.0;
    let mut t = 0.0;

    let event_loop = glutin::event_loop::EventLoop::new();
    let wb = glutin::window::WindowBuilder::new();
    let cb = glutin::ContextBuilder::new();
    let display = glium::Display::new(wb, cb, &event_loop).unwrap();

    let sphere = Sphere::new(&display, 30, 30);

    let vertex = r#"
    #version 150

    in vec3 position;
    in vec3 normal;

    uniform mat4 model;
    uniform mat4 view;
    uniform mat4 perspective;
    uniform vec3 light;

    smooth out vec3 l;
    smooth out vec3 n;
    smooth out vec3 p;

    void main() {
        mat4 modelview = view * model;
        l = transpose(inverse(mat3(view))) * light;
        n = transpose(inverse(mat3(modelview))) * normal;
        p = position;
        gl_Position = perspective * modelview * vec4(position, 1.0);
    }
    "#;
    let fragment = r#"
    #version 150

    uniform vec3 dark_color;
    uniform vec3 high_color;

    smooth in vec3 l;
    smooth in vec3 n;
    smooth in vec3 p;

    out vec4 color;

    void main() {
        vec3 nl = normalize(l);
        vec3 nn = normalize(n);
        vec3 nr = nl - 2 * nn * dot(nl,nn);
        float brightness = clamp(0, -dot(nn, nl), 1);
        float specular = pow(max(nr.z, 0), 16);
        vec3 hc = high_color;
        vec3 dc = dark_color;
        color = vec4(mix(dc, hc, brightness) + specular * vec3(1,1,1), 0.95);
    }
    "#;

    let program = glium::Program::from_source(&display, vertex, fragment, None).unwrap();

    let mut new_balls = balls.clone();
    let mut new_walls = walls.clone();
    let mut dt = 0.0;
    let start = std::time::Instant::now();

    event_loop.run(move |event, _, control_flow| {
        t += 1.0 / 120.0;
        *control_flow = glutin::event_loop::ControlFlow::WaitUntil(
            start + std::time::Duration::from_secs_f64(t),
        );

        match event {
            glutin::event::Event::WindowEvent { event, .. } => match event {
                glutin::event::WindowEvent::CloseRequested => {
                    *control_flow = glutin::event_loop::ControlFlow::Exit;
                    return;
                }
                _ => {}
            },
            glutin::event::Event::NewEvents(cause) => match cause {
                glutin::event::StartCause::ResumeTimeReached { .. } => (),
                glutin::event::StartCause::Init => (),
                _ => {}
            },
            _ => {}
        }

        while t > t_checkpoint + dt {
            println!("evolve {:.1} + {:.3}", t_checkpoint, dt);
            let mut e = 0.0;
            for a in &balls {
                e += 0.5 * a.m * a.v.norm().powi(2);
            }
            println!("energy {:.6}", e);

            t_checkpoint += dt;
            balls = new_balls.clone();
            walls = new_walls.clone();

            let (dt_s, new_balls_, new_walls_) = evolve(&balls, &walls);
            new_balls = new_balls_;
            new_walls = new_walls_;
            dt = dt_s;
        }

        let mut target = display.draw();
        target.clear_color_and_depth((0.0, 0.0, 0.0, 1.0), 1.0);

        let pers = {
            let (width, height) = target.get_dimensions();
            let aspect_ratio = width as f32 / height as f32;
            Mat4::perspective(aspect_ratio, 3.14f32 / 3.0, 0.1, 1024.0)
        };
        let view = Mat4::translation(0.0, 0.0, -4.0)
            * Mat4::rotation(0.0, 1.0, 0.0, 0.0)
            * Mat4::rotation(0.0, 0.0, 0.0, -1.0)
            * Mat4::scale(1.0);

        let params = glium::DrawParameters {
            depth: glium::Depth {
                test: glium::draw_parameters::DepthTest::IfLess,
                write: true,
                ..Default::default()
            },
            // polygon_mode: glium::draw_parameters::PolygonMode::Line,
            backface_culling: glium::draw_parameters::BackfaceCullingMode::CullClockwise,
            blend: glium::Blend::alpha_blending(),
            ..Default::default()
        };

        for i in 0..balls.len() {
            let r = balls[i].r;
            let ddt = t - t_checkpoint;
            let x = balls[i].x + ddt * balls[i].v;

            let model =
                Mat4::translation(x.0 as f32, x.1 as f32, x.2 as f32) * Mat4::scale(r as f32);

            let uniform = uniform! {
                model: model.0,
                view: view.0,
                perspective: pers.0,
                light: [0., 0., -3f32],
                high_color: if i == 0 {[1.0, 0.0, 0.0f32]} else {[0.5, 0.5, 1.0f32]},
                dark_color: if i == 0 {[0.5, 0.0, 0.0f32]} else {[0.4, 0.4, 0.5f32]},
            };

            target
                .draw(
                    sphere.get_positions(),
                    sphere.get_indices(),
                    &program,
                    &uniform,
                    &params,
                )
                .unwrap();
        }

        target.finish().unwrap();
    });
}
