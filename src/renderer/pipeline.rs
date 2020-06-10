use crate::Result;
use std::{fs::File, io::Read, path::Path};

pub fn create_pipeline(
    vert_file: &Path,
    frag_file: &Path,
    vertex_buffer_descriptor: wgpu::VertexBufferDescriptor,
    format: wgpu::TextureFormat,
    device: &wgpu::Device,
) -> Result<wgpu::RenderPipeline> {
    let mut vs_src = String::new();
    File::open(vert_file)?.read_to_string(&mut vs_src)?;

    let mut fs_src = String::new();
    File::open(frag_file)?.read_to_string(&mut fs_src)?;

    let vs_spriv = glsl_to_spirv::compile(&vs_src, glsl_to_spirv::ShaderType::Vertex)?;
    let fs_spriv = glsl_to_spirv::compile(&fs_src, glsl_to_spirv::ShaderType::Fragment)?;

    let vs_data = wgpu::read_spirv(vs_spriv)?;
    let fs_data = wgpu::read_spirv(fs_spriv)?;

    let vs_module = device.create_shader_module(&vs_data);
    let fs_module = device.create_shader_module(&fs_data);
    let layout = device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
        bind_group_layouts: &[],
    });

    let render_pipeline = device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
        layout: &layout,
        vertex_stage: wgpu::ProgrammableStageDescriptor {
            module: &vs_module,
            entry_point: "main",
        },
        fragment_stage: Some(wgpu::ProgrammableStageDescriptor {
            module: &fs_module,
            entry_point: "main",
        }),
        rasterization_state: Some(wgpu::RasterizationStateDescriptor {
            front_face: wgpu::FrontFace::Ccw,
            cull_mode: wgpu::CullMode::Back,
            depth_bias: 0,
            depth_bias_slope_scale: 0.0,
            depth_bias_clamp: 0.0,
        }),
        color_states: &[wgpu::ColorStateDescriptor {
            format: format,
            color_blend: wgpu::BlendDescriptor::REPLACE,
            alpha_blend: wgpu::BlendDescriptor::REPLACE,
            write_mask: wgpu::ColorWrite::ALL,
        }],
        vertex_state: wgpu::VertexStateDescriptor {
            index_format: wgpu::IndexFormat::Uint16,
            vertex_buffers: &[vertex_buffer_descriptor],
        },
        primitive_topology: wgpu::PrimitiveTopology::TriangleList,
        depth_stencil_state: None,
        sample_count: 1,
        sample_mask: !0,
        alpha_to_coverage_enabled: false,
    });

    Ok(render_pipeline)
}
