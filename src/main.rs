const FPS: u8 = 60;

#[macro_use]
extern crate glium;
extern crate image;

use std::io::Cursor;
use glium::{glutin, Surface};
use std::time;

#[derive(Copy, Clone)]
struct Vertex {
	position: [f32; 2],
	texture: [f32; 2]
}
impl Vertex {
	#[inline]
	fn new(p: [f32; 2], c: [f32; 2]) -> Self {
		Vertex {position: p, texture: c}
	}
}

const VERT_S: &str = r#"
	#version 330 core
	uniform mat2 matrix;
	in vec2 position;
	in vec2 texture;
	out vec2 vCol;
	void main() {
		gl_Position = vec4(matrix * position, 0.0, 1.0);
		vCol = texture;
	}
"#;

const FRAG_S: &str = r#"
	#version 330 core

	in vec2 vCol;
	out vec4 fCol;
	uniform sampler2D tex;

	void main() {
		fCol = texture(tex, vCol);
	}
"#;

fn main() {
	let event_loop = glutin::event_loop::EventLoop::new();
	let wb = glutin::window::WindowBuilder::new();
	let cb = glutin::ContextBuilder::new();

	let display = glium::Display::new(wb, cb, &event_loop).unwrap();


	let image = image::load(Cursor::new(&include_bytes!("../Lotka.jpeg")), image::ImageFormat::Jpeg).unwrap().to_rgb8();
	let _image_dim = image.dimensions();
	let image = glium::texture::RawImage2d::from_raw_rgb_reversed(&image.into_raw(), _image_dim);
	let texture = glium::texture::SrgbTexture2d::new(&display, image).unwrap();

	let shape = [
		Vertex::new([0.5, 0.5], [1.0, 1.0]),
		Vertex::new([-0.5, 0.5], [0.0, 1.0]),
		Vertex::new([-0.5, -0.5], [0.0, -1.0]),
		Vertex::new([0.5, -0.5], [0.0, -1.0])
	];


	implement_vertex!(Vertex, position, texture);
	let vertex_buffer = glium::VertexBuffer::new(&display, &shape).unwrap();
	let indices = glium::index::NoIndices(glium::index::PrimitiveType::TriangleFan);
	let program = glium::Program::from_source(&display, VERT_S, FRAG_S, None).unwrap();

	let mut t: f32 = 0.0;
	event_loop.run(move |event, _, control_flow| {
		use glutin::event::WindowEvent;
		match event {
			glutin::event::Event::WindowEvent{event, ..} => {
				match event {
					WindowEvent::CloseRequested => std::process::exit(0),
					_ => return
				};
			},
			glutin::event::Event::NewEvents(cause) => match cause {
				glutin::event::StartCause::ResumeTimeReached{..} => (),
				glutin::event::StartCause::Init => (),
				_ => return
			},
			_ => return
		}

		*control_flow = glutin::event_loop::ControlFlow::WaitUntil(
			time::Instant::now() +
			time::Duration::from_nanos((1e9 / (FPS as f64)) as u64)
		);

		let uniforms = uniform! {
			tex: &texture,
			matrix: [
				[t.cos(), t.sin()],
				[-t.sin(), t.cos()]
			],
		};

		let mut target = display.draw();
		target.clear_color(1.0, 0.5, 0.5, 1f32);
		target.draw(&vertex_buffer, &indices, &program, &uniforms, &Default::default()).unwrap();
		target.finish().unwrap();
		t += 0.003;
	});
}