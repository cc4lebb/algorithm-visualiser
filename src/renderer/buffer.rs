use wgpu::util::DeviceExt;


#[repr(C)]
#[derive(Copy, Clone, Debug, bytemuck::Pod, bytemuck::Zeroable)]
pub struct Vertex {
    position: [f32; 3],
    color: [f32; 3],
}

impl Vertex {
    pub fn desc() -> wgpu::VertexBufferLayout<'static> {
        wgpu::VertexBufferLayout{
            array_stride: std::mem::size_of::<Vertex>() as wgpu::BufferAddress,
            step_mode: wgpu::VertexStepMode::Vertex,
            attributes: &[
                wgpu::VertexAttribute {
                    offset: 0,
                    shader_location: 0,
                    format: wgpu::VertexFormat::Float32x3,
                },
                wgpu::VertexAttribute {
                    offset: std::mem::size_of::<[f32; 3]>() as wgpu::BufferAddress,
                    shader_location: 1,
                    format: wgpu::VertexFormat::Float32x3,
                }
            ]
        }
    }
}
const VERTICES: &[Vertex] = &[
    Vertex { position: [-0.5,  0.5, 0.0], color: [0.8, 0.2, 0.2] }, // 0: Top-Left
    Vertex { position: [-0.5, -0.5, 0.0], color: [0.8, 0.2, 0.2] }, // 1: Bottom-Left
    Vertex { position: [ 0.5, -0.5, 0.0], color: [0.8, 0.2, 0.2] }, // 2: Bottom-Right
    Vertex { position: [ 0.5,  0.5, 0.0], color: [0.8, 0.2, 0.2] }, // 3: Top-Right
];

const RECT_INDICES: &[u16] = &[0, 1, 2, 0, 2, 3];

pub fn create_buffer(device: &wgpu::Device) -> (wgpu::Buffer, wgpu::Buffer, u32) {
    let vertex_buffer = device.create_buffer_init(
        &wgpu::util::BufferInitDescriptor {
            label: Some("Vertex Buffer"),
            contents: bytemuck::cast_slice(VERTICES),
            usage: wgpu::BufferUsages::VERTEX,
        }
    );

    let index_buffer = device.create_buffer_init(
        &wgpu::util::BufferInitDescriptor {
            label: Some("Test Index Buffer"),
            contents: bytemuck::cast_slice(RECT_INDICES),
            usage: wgpu::BufferUsages::INDEX
        }
    );
    
    (vertex_buffer, index_buffer, RECT_INDICES.len() as u32)

}

