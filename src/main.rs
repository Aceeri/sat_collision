#[macro_use]
extern crate glium;
extern crate cgmath;
extern crate time;
extern crate image;

mod entities;
mod rendering;

use cgmath::{ Vector2, Matrix4 };
use entities::object::{ Actor, Hitbox };
use rendering::rendering::{ Vertex };
use glium::{ Surface };
use time::{ PreciseTime };

fn main() {
	let mut actor : Actor = Actor::new();
	actor.position = Vector2 { x: 0.0, y: 0.0 };
	let hitbox : Hitbox = Hitbox::Polygon { 
		offset: Vector2 { x: 0.0, y: 0.0 }, 
		vertices: vec![
			Vector2 { x: -1.0, y: 1.0 },
			Vector2 { x: 1.0, y: 1.0 },
			Vector2 { x: 1.0, y: -1.0 },
			Vector2 { x: -1.0, y: -1.0 }
		]
	};
	actor.add_hitbox(hitbox);
	println!("Actor 1: {:?}", actor);

	let mut actor2 : Actor = Actor::new();
	//let hitbox2 : Hitbox = Hitbox::Circle { offset: Vector2 { x: 0.0, y: 0.0 }, radius: 3.0f32 };
	let hitbox2 : Hitbox = Hitbox::Polygon {
		offset: Vector2 { x: 3.9, y: 3.9 },
		vertices: vec![
			Vector2 { x: -1.0, y: 1.0 },
			Vector2 { x: 1.0, y: 1.0 },
			Vector2 { x: 1.0, y: -1.0 },
			Vector2 { x: -1.0, y: -1.0 }
		]
	};
	actor2.add_hitbox(hitbox2);
	println!("Actor 2: {:?}", actor2);

    let mut events_loop = glium::glutin::EventsLoop::new();
    let primary_monitor = events_loop.get_primary_monitor();
    let (dim_x, dim_y) = (800.0, 800.0);
    //let (dim_x, dim_y) = glium::glutin::get_primary_monitor().get_dimensions();
    let fullscreen = false;
    let title = "Test Window".to_string();

    let mut window_builder = glium::glutin::WindowBuilder::new();
    window_builder = window_builder
        .with_dimensions(glutin::dpi::LogicalSize::new(dim_x, dim_y))
        .with_title(title);

    if fullscreen {
        window_builder = window_builder.with_fullscreen(Some(primary_monitor));
    }

    let context_builder = glium::glutin::ContextBuilder::new();
    let display = glium::Display::new(window_builder, context_builder, &events_loop).unwrap();


    let vertex1 = Vertex { position : [ -0.5, 0.5 ], tex_coords: [ 0.0, 0.0 ] };
    let vertex2 = Vertex { position : [ 0.5, -0.5 ], tex_coords: [ 0.0, 1.0 ] };
    let vertex3 = Vertex { position : [ -0.5, -0.5 ], tex_coords: [ 1.0, 0.0 ] };
    let shape = vec![ vertex1, vertex2, vertex3 ];

    let vertex_buffer = glium::VertexBuffer::new(&display, &shape).unwrap();
    let indices = glium::index::NoIndices(glium::index::PrimitiveType::TrianglesList);

    //let image = image::load(Cursor::new(&include_bytes!("../fsm.png")[..]), image::PNG).unwrap();
    //let texture = glium::texture::Texture2d::new(&display, image).unwrap();

    let vertex_shader_src = rendering::vertex_src();
    let fragment_shader_src = rendering::fragment_src();

    let program = glium::Program::from_source(&display, vertex_shader_src, fragment_shader_src, None).unwrap();

    let mut ratio : f32 = dim_y as f32 / dim_x as f32;
    println!("ratio: {}", ratio);

    let mut now: PreciseTime = PreciseTime::now();
    let mut delta: f32 = 0.0;
    let mut elapsed: f64 = 0.0;

    let mut t: f32 = 0.0;

    loop {
        ::std::thread::sleep(::std::time::Duration::from_millis(0));
        t += 0.16f32 * delta;

        /*
        Identity Matrix4
        1.0, 0.0, 0.0, 0.0
        0.0, 1.0, 0.0, 0.0
        0.0, 0.0, 1.0, 0.0
        0.0, 0.0, 0.0, 1.0
        */
        let scaled_matrix : Matrix4<f32>  = Matrix4::new(ratio, 0.0, 0.0, 0.0, 0.0, 1.0, 0.0, 0.0, 0.0, 0.0, 1.0, 0.0, 0.0, 0.0, 0.0, 1.0f32);
        let rotated_matrix : Matrix4<f32> = Matrix4::new(t.cos(), t.sin(), 0.0, 0.0, -t.sin(), t.cos(), 0.0, 0.0, 0.0, 0.0, 1.0, 0.0, 0.0, 0.0, 0.0, 1.0f32);
        let translation_matrix : Matrix4<f32> = Matrix4::new(1.0, 0.0, 0.0, 0.0, 0.0, 1.0, 0.0, 0.0, 0.0, 0.0, 1.0, 0.0, 0.5, -0.5, 0.0, 1.0f32);
        let position_matrix : Matrix4<f32> = Matrix4::new(1.0, 0.0, 0.0, 0.0, 0.0, 1.0, 0.0, 0.0, 0.0, 0.0, 1.0, 0.0, 0.0, 0.0, 0.0, 1.0f32);
        let zoom_matrix : Matrix4<f32> = Matrix4::new(1.0, 0.0, 0.0, 0.0, 0.0, 1.0, 0.0, 0.0, 0.0, 0.0, 1.0, 0.0, 0.0, 0.0, 0.0, 1.0f32);
        let result_matrix : Matrix4<f32> = position_matrix * scaled_matrix * zoom_matrix * rotated_matrix * translation_matrix;

        let f32matrix: [[f32; 4]; 4] = result_matrix.into();
        let uniforms = uniform! {
            /*matrix: [
                [ 1.0,  0.0,  0.0,  0.0 ], // x scale, y skew, ?
                [ 0.0,  1.0,  0.0,  0.0 ], // x skew, y scale, ?, ?
                [ 0.0,  0.0,  1.0,  0.0 ], // ?, ?, z scale, ?
                [ 0.0,  0.0,  0.0,  3f32 ], // x translation, y translation, ?, zoom
            ]*/
            matrix: f32matrix,
            //test: 0.5f32
        };
        // rendering
        let mut target = display.draw();
        target.clear_color(108.0/255.0, 122.0/255.0, 137.0/255.0, 1.0);

        target.draw(&vertex_buffer, &indices, &program, &uniforms, &Default::default()).unwrap();

        target.finish().unwrap();

        // delta time
        delta = (now.to(PreciseTime::now()).num_milliseconds() as f32) / 1000.0f32;
        elapsed += delta as f64;
        now = PreciseTime::now();

        let mut end = false;
        events_loop.poll_events(|e| {
            match e { 
                glutin::Event::WindowEvent { window_id, event } => match event {
                    glutin::WindowEvent::Resized(size) => {
                        println!("Window Resized to {}x{}", size.width, size.height);
                        ratio = size.height as f32 / size.width as f32
                    }
                    glutin::WindowEvent::CloseRequested => end = true,
                    _ => {},
                },
                glutin::Event::DeviceEvent { device_id, event } => match event {
                    _ => {},
                },
                glutin::Event::Awakened => {},
                glutin::Event::Suspended(_) => {},

                //glium::glutin::Event::Closed => return,
                //glium::glutin::Event::Resized(width, height) => { 
                //},
            }
        });

        if end {
            break;
        }
    }

    entities::colliding(&mut actor, &mut actor2);
}
