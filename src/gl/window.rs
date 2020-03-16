use crate::dynamics::{Ball, Wall};
use crate::gl::math::Mat4;
use crate::gl::sphere::Sphere;
use glium::glutin;
use glium::Surface;
use std::collections::HashMap;

#[derive(Clone, Copy)]
struct Vertex {
    position: [f32; 4],
    normal: [f32; 3],
}

implement_vertex!(Vertex, position, normal);

pub fn visualize<F>(mut evolution: F) -> !
where
    F: 'static + FnMut() -> (f64, Vec<Ball>, Vec<Wall>),
{
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
        float specular = pow(max(nr.z, 0), 60);
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

    let start = std::time::Instant::now();
    let mut t_next = 0.0;
    let mut dt = 0.0;
    let mut last = start;

    let mut balls = Vec::new();
    let mut walls = Vec::new();

    let mut view = Mat4::translation(0.0, 0.0, -4.0)
        * Mat4::rotation(0.0, 1.0, 0.0, 0.0)
        * Mat4::rotation(std::f32::consts::FRAC_PI_4, 1.0, 0.0, 1.0)
        * Mat4::scale(1.0);

    let mut last_cursor_position = glutin::dpi::PhysicalPosition::new(0.0, 0.0);
    let mut left_button_state = glutin::event::ElementState::Released;

    let mut modifiers_state = glutin::event::ModifiersState::empty();
    let mut key_state: HashMap<glutin::event::VirtualKeyCode, glium::glutin::event::ElementState> =
        HashMap::new();

    event_loop.run(move |event, _, control_flow| {
        let next_frame_time =
            std::time::Instant::now() + std::time::Duration::from_nanos(16_666_667);
        *control_flow = glutin::event_loop::ControlFlow::WaitUntil(next_frame_time);

        match &event {
            glutin::event::Event::WindowEvent { event, .. } => match event {
                glutin::event::WindowEvent::CloseRequested => {
                    *control_flow = glutin::event_loop::ControlFlow::Exit;
                    return;
                }
                glutin::event::WindowEvent::CursorMoved { position, .. } => {
                    if left_button_state == glutin::event::ElementState::Pressed {
                        let dx = (position.x - last_cursor_position.x) as f32;
                        let dy = (position.y - last_cursor_position.y) as f32;
                        view = Mat4::rotation(-0.01 * dy, 1.0, 0.0, 0.0) * view;
                        view = Mat4::rotation(-0.01 * dx, 0.0, 1.0, 0.0) * view;
                    }
                    last_cursor_position = *position;
                }
                glutin::event::WindowEvent::MouseInput {
                    state,
                    button: glutin::event::MouseButton::Left,
                    ..
                } => {
                    left_button_state = *state;
                }
                glutin::event::WindowEvent::KeyboardInput {
                    input:
                        glutin::event::KeyboardInput {
                            state,
                            virtual_keycode,
                            ..
                        },
                    ..
                } => {
                    if let Some(key) = virtual_keycode {
                        key_state.insert(*key, *state);
                    }
                }
                _ => (),
            },
            glutin::event::Event::DeviceEvent { event, .. } => match event {
                glutin::event::DeviceEvent::ModifiersChanged(state) => {
                    modifiers_state = *state;
                }
                _ => (),
            },
            _ => (),
        }

        if (std::time::Instant::now() - last).as_secs_f64() < 1.0 / 120.0 {
            return;
        }
        {
            let dt = (std::time::Instant::now() - last).as_secs_f32();
            let v = if modifiers_state.shift() { 10.0 } else { 1.0 };
            if key_state
                .entry(glutin::event::VirtualKeyCode::W)
                .or_insert(glutin::event::ElementState::Released)
                == &glutin::event::ElementState::Pressed
            {
                view = Mat4::translation(0.0, 0.0, v * dt) * view;
            }
            if key_state
                .entry(glutin::event::VirtualKeyCode::S)
                .or_insert(glutin::event::ElementState::Released)
                == &glutin::event::ElementState::Pressed
            {
                view = Mat4::translation(0.0, 0.0, -v * dt) * view;
            }
            if key_state
                .entry(glutin::event::VirtualKeyCode::A)
                .or_insert(glutin::event::ElementState::Released)
                == &glutin::event::ElementState::Pressed
            {
                view = Mat4::translation(-v * dt, 0.0, 0.0) * view;
            }
            if key_state
                .entry(glutin::event::VirtualKeyCode::D)
                .or_insert(glutin::event::ElementState::Released)
                == &glutin::event::ElementState::Pressed
            {
                view = Mat4::translation(v * dt, 0.0, 0.0) * view;
            }
        }

        last = std::time::Instant::now();

        let delta_t = {
            let t = (std::time::Instant::now() - start).as_secs_f64();

            while t > t_next {
                let (new_dt, new_balls, new_walls) = evolution();
                t_next += new_dt;
                dt = new_dt;
                balls = new_balls;
                walls = new_walls;
            }

            t - (t_next - dt)
        };

        let mut target = display.draw();
        target.clear_color_and_depth((0.0, 0.0, 0.0, 1.0), 1.0);

        let pers = {
            let (width, height) = target.get_dimensions();
            let aspect_ratio = width as f32 / height as f32;
            Mat4::perspective(aspect_ratio, 3.14f32 / 3.0, 0.1, 1024.0)
        };

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

        for i in 0..balls.len() {
            let a = &balls[i];
            let r = a.r;
            let x = a.x + delta_t * a.v;

            let model =
                Mat4::translation(x.0 as f32, x.1 as f32, x.2 as f32) * Mat4::scale(r as f32);

            let uniform = uniform! {
                model: model.as_array(),
                view: view.as_array(),
                perspective: pers.as_array(),
                light: [0., 0., -3f32],
                high_color: if i == 0 {[1.0, 0.0, 0.0f32]} else {[0.7, 0.7, 0.7f32]},
                dark_color: if i == 0 {[0.5, 0.0, 0.0f32]} else {[0.4, 0.4, 0.4f32]},
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
            let x = w.x + delta_t * w.v;
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
