
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

const RECT_INDICES: &[u16] = &[0, 1, 2, 0, 2, 3];

pub fn create_bar_buffers(device: &wgpu::Device, values: &[f32]) -> 
(wgpu::Buffer, wgpu::Buffer, u32) {
    let mut vertices = Vec::with_capacity(values.len() * 4);
    let mut indices = Vec::with_capacity(values.len() * 6);

    let count = values.len() as f32;
    let bar_width = 2.0 / count;

    let bar_gap = bar_width * 0.10;
    

    for (i, &val) in values.iter().enumerate(){
        let min_x = -1.0 + (i as f32 * bar_width) + bar_gap;
        let max_x = min_x + bar_width - bar_gap;
        let min_y = -1.0;
        let max_y = min_y + (val * 2.0);

        vertices.extend_from_slice(&[
            Vertex { position: [min_x, max_y, 0.0], color: [0.0, 0.2, 0.1] },
            Vertex { position: [min_x, min_y, 0.0], color: [0.0, 0.2, 0.1] },
            Vertex { position: [max_x, min_y, 0.0], color: [0.0, 0.2, 0.1] },
            Vertex { position: [max_x, max_y, 0.0], color: [0.0, 0.2, 0.1] },
        ]);

        let vertex_offset = (i * 4) as u16;
        
        for &idx in RECT_INDICES {
           indices.push(vertex_offset + idx); 
        }
    }

    let vertex_buffer = device.create_buffer_init(
            &wgpu::util::BufferInitDescriptor {
                label: Some("Vertex Buffer"),
                contents: bytemuck::cast_slice(&vertices),
                usage: wgpu::BufferUsages::VERTEX,
            }
        );

        let index_buffer = device.create_buffer_init(
            &wgpu::util::BufferInitDescriptor {
                label: Some("Test Index Buffer"),
                contents: bytemuck::cast_slice(&indices),
                usage: wgpu::BufferUsages::INDEX
            }
        );
        
        (vertex_buffer, index_buffer, indices.len() as u32)


}
