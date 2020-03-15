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

#[derive(Clone, Copy)]
struct Vertex {
    position: [f32; 4],
    normal: [f32; 3],
}

implement_vertex!(Vertex, position, normal);

fn main() {
    let mut balls = Vec::new();
    balls.push(Ball {
        x: V::new(-0.9, 0.0, 0.0),
        v: V::new(5.0, 1.0, 1.1),
        m: 1.0,
        r: 0.1,
    });
    balls.push(Ball {
        x: V::new(0.9, 0.0, 0.0),
        v: V::new(1.0, 1.0, 1.1),
        m: 3.0,
        r: 0.2,
    });
    // for &x in &[-2.0, -1.0, 0.0, 1.0, 2.0] {
    //     for &y in &[-2.0, -1.0, 0.0, 1.0, 2.0] {
    //         for &z in &[-2.0, -1.0, 0.0, 1.0, 2.0] {
    //             balls.push(Ball {
    //                 x: V::new(x * 0.2, y * 0.2, z * 0.2),
    //                 v: V::new(x * 0.2, y * 0.2, z * 0.2),
    //                 m: 1.0,
    //                 r: 0.1,
    //             });
    //         }
    //     }
    // }

    let mut walls = vec![
        Wall {
            x: V::new(0.0, 0.0, 0.0),
            v: V::new(0.5, 0.0, 0.0),
            j: V::new(0.0, 1.0, 0.0),
            k: V::new(0.0, 0.0, 1.0),
            m: core::f64::INFINITY,
        },
        Wall {
            x: V::new(1.0, -1.0, -1.0),
            v: V::new(0.0, 0.0, 0.0),
            j: V::new(0.0, 2.0, 0.0),
            k: V::new(0.0, 0.0, 2.0),
            m: core::f64::INFINITY,
        },
        Wall {
            x: V::new(-1.0, -1.0, -1.0),
            v: V::new(0.0, 0.0, 0.0),
            j: V::new(0.0, 2.0, 0.0),
            k: V::new(0.0, 0.0, 2.0),
            m: core::f64::INFINITY,
        },
        Wall {
            x: V::new(-1.0, -1.0, -1.0),
            v: V::new(0.0, 0.0, 0.0),
            j: V::new(2.0, 0.0, 0.0),
            k: V::new(0.0, 0.0, 2.0),
            m: core::f64::INFINITY,
        },
        Wall {
            x: V::new(-1.0, 1.0, -1.0),
            v: V::new(0.0, 0.0, 0.0),
            j: V::new(2.0, 0.0, 0.0),
            k: V::new(0.0, 0.0, 2.0),
            m: core::f64::INFINITY,
        },
        Wall {
            x: V::new(-1.0, -1.0, -1.0),
            v: V::new(0.0, 0.0, 0.0),
            j: V::new(2.0, 0.0, 0.0),
            k: V::new(0.0, 2.0, 0.0),
            m: core::f64::INFINITY,
        },
        Wall {
            x: V::new(-1.0, -1.0, 1.0),
            v: V::new(0.0, 0.0, 0.0),
            j: V::new(2.0, 0.0, 0.0),
            k: V::new(0.0, 2.0, 0.0),
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

    let wall_verticies = vec![
        Vertex {
            position: [0.0, 0.0, 0.0, 1.0],
            normal: [0.0, 0.0, 1.0],
        },
        Vertex {
            position: [1.0, 0.0, 0.0, 1.0],
            normal: [0.0, 0.0, 1.0],
        },
        Vertex {
            position: [1.0, 1.0, 0.0, 1.0],
            normal: [0.0, 0.0, 1.0],
        },
        Vertex {
            position: [0.0, 1.0, 0.0, 1.0],
            normal: [0.0, 0.0, 1.0],
        },
    ];
    let wall_verticies = glium::VertexBuffer::new(&display, &wall_verticies).unwrap();
    let wall_indices = glium::index::NoIndices(glium::index::PrimitiveType::TriangleFan);

    let vertex_sphere = r#"
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
    let fragment_sphere = r#"
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
        color = vec4(mix(dc, hc, brightness) + specular * vec3(1,1,1), 1);
    }
    "#;

    let vertex_wall = r#"
    #version 150

    in vec4 position;
    in vec3 normal;

    uniform mat4 model;
    uniform mat4 view;
    uniform mat4 perspective;

    smooth out vec3 n;
    smooth out vec3 p;

    void main() {
        mat4 modelview = view * model;
        n = transpose(inverse(mat3(modelview))) * normal;
        p = position.xyz;
        gl_Position = perspective * modelview * position;
    }
    "#;
    let fragment_wall = r#"
    #version 150

    uniform vec4 uniform_color;

    smooth in vec3 n;
    smooth in vec3 p;

    out vec4 color;

    void main() {
        color = uniform_color;
    }
    "#;

    let program_sphere =
        glium::Program::from_source(&display, vertex_sphere, fragment_sphere, None).unwrap();
    let program_wall =
        glium::Program::from_source(&display, vertex_wall, fragment_wall, None).unwrap();

    let mut new_balls = balls.clone();
    let mut new_walls = walls.clone();
    let mut dt = 0.0;
    let start = std::time::Instant::now();
    let mut last = start;

    event_loop.run(move |event, _, control_flow| {
        match &event {
            glutin::event::Event::WindowEvent { event, .. } => match event {
                glutin::event::WindowEvent::CloseRequested => {
                    *control_flow = glutin::event_loop::ControlFlow::Exit;
                    return;
                }
                _ => (),
            },
            glutin::event::Event::NewEvents(cause) => match cause {
                glutin::event::StartCause::ResumeTimeReached { .. } => (),
                glutin::event::StartCause::Init => (),
                glutin::event::StartCause::WaitCancelled { .. } => (),
                _ => (),
            },
            _ => (),
        }

        if (std::time::Instant::now() - last).as_secs_f64() < 1.0 / 120.0 {
            // println!("{:?}", event);
            return;
        }
        last = std::time::Instant::now();
        t = (std::time::Instant::now() - start).as_secs_f64();

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

            let (new_balls_, new_walls_, dt_s, _work) = evolve(balls.clone(), walls.clone(), 10.0);
            new_balls = new_balls_;
            new_walls = new_walls_;
            dt = dt_s;

            // println!("{} {}", new_walls[1].x, new_walls[1].v);
            if new_walls[0].x.0 > 0.2 && new_walls[0].v.0 > 0.0 {
                new_walls[0].v = -new_walls[0].v;
            }
            if new_walls[0].x.0 < -0.2 && new_walls[0].v.0 < 0.0 {
                new_walls[0].v = -new_walls[0].v;
            }
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
            * Mat4::rotation(std::f32::consts::FRAC_PI_4, 1.0, 0.0, 1.0)
            * Mat4::scale(1.0);

        // draw balls
        let params = glium::DrawParameters {
            depth: glium::Depth {
                test: glium::draw_parameters::DepthTest::IfLess,
                write: true,
                ..Default::default()
            },
            // polygon_mode: glium::draw_parameters::PolygonMode::Line,
            backface_culling: glium::draw_parameters::BackfaceCullingMode::CullClockwise,
            // blend: glium::Blend::alpha_blending(),
            ..Default::default()
        };

        let ddt = t - t_checkpoint;

        for i in 0..balls.len() {
            let a = &balls[i];
            let r = a.r;
            let x = a.x + ddt * a.v;

            let model =
                Mat4::translation(x.0 as f32, x.1 as f32, x.2 as f32) * Mat4::scale(r as f32);

            let uniform = uniform! {
                model: model.as_array(),
                view: view.as_array(),
                perspective: pers.as_array(),
                light: [0., 0., -3f32],
                high_color: if i == 0 {[1.0, 0.0, 0.0f32]} else {[0.5, 0.5, 1.0f32]},
                dark_color: if i == 0 {[0.5, 0.0, 0.0f32]} else {[0.4, 0.4, 0.5f32]},
            };

            target
                .draw(
                    sphere.get_positions(),
                    sphere.get_indices(),
                    &program_sphere,
                    &uniform,
                    &params,
                )
                .unwrap();
        }

        // draw walls
        for (i, w) in walls.iter().enumerate() {
            let x = w.x + ddt * w.v;
            let m = Mat4::from_array([
                [w.j.0 as f32, w.j.1 as f32, w.j.2 as f32, 0.0],
                [w.k.0 as f32, w.k.1 as f32, w.k.2 as f32, 0.0],
                [0.0, 0.0, 0.0, 0.0],
                [0.0, 0.0, 0.0, 1.0],
            ]);

            let uniform = uniform! {
                model: (Mat4::translation(x.0 as f32, x.1 as f32, x.2 as f32) * m).as_array(),
                view: view.as_array(),
                perspective: pers.as_array(),
                uniform_color: [
                    [0.5, 0., 0., 0.1f32],
                    [0., 0.5, 0., 0.1f32],
                    [0., 0., 0.5, 0.1f32],
                    [0., 0.5, 0.5, 0.1f32],
                    [0.5, 0., 0.5, 0.1f32],
                    [0.5, 0.5, 0., 0.1f32],
                ][i % 6],
            };

            let params = glium::DrawParameters {
                depth: glium::Depth {
                    test: glium::draw_parameters::DepthTest::IfLess,
                    write: false,
                    ..Default::default()
                },
                blend: glium::Blend {
                    color: glium::BlendingFunction::Addition {
                        source: glium::LinearBlendingFactor::SourceAlpha,
                        destination: glium::LinearBlendingFactor::OneMinusSourceAlpha,
                    },
                    alpha: glium::BlendingFunction::Addition {
                        source: glium::LinearBlendingFactor::Zero,
                        destination: glium::LinearBlendingFactor::One,
                    },
                    constant_value: (0.0, 0.0, 0.0, 0.0),
                },
                ..Default::default()
            };

            target
                .draw(
                    &wall_verticies,
                    &wall_indices,
                    &program_wall,
                    &uniform,
                    &params,
                )
                .unwrap();
        }

        target.finish().unwrap();
    });
}
