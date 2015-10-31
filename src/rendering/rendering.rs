
#[derive(Copy, Clone)]
pub struct Vertex {
    pub position: [f32; 2],
    pub texture: [f32; 2]
}

implement_vertex!(Vertex, position, texture);

pub trait Renderable {
	fn draw();
}