use crate::{model::Model, model::Vertex, Result};
use std::path::Path;
use winit::{dpi::PhysicalSize, window::Window};

pub mod pipeline;
pub mod texture;
pub use pipeline::Pipeline;
pub use texture::Texture;

pub struct Renderer {
    surface: wgpu::Surface,
    adapter: wgpu::Adapter,
    pub device: wgpu::Device,
    pub queue: wgpu::Queue,
    sc_desc: wgpu::SwapChainDescriptor,
    swap_chain: wgpu::SwapChain,
    size: PhysicalSize<u32>,
}

impl Renderer {
    pub fn new(window: &Window) -> Self {
        let size = window.inner_size();

        let surface = wgpu::Surface::create(window);
        let adapter = wgpu::Adapter::request(&wgpu::RequestAdapterOptions {
            power_preference: wgpu::PowerPreference::HighPerformance,
            backends: wgpu::BackendBit::VULKAN,
        })
        .unwrap();

        let (device, queue) = adapter.request_device(&wgpu::DeviceDescriptor {
            extensions: wgpu::Extensions {
                anisotropic_filtering: false,
            },
            limits: Default::default(),
        });

        let sc_desc = wgpu::SwapChainDescriptor {
            usage: wgpu::TextureUsage::OUTPUT_ATTACHMENT,
            format: wgpu::TextureFormat::Bgra8UnormSrgb,
            width: size.width,
            height: size.height,
            present_mode: wgpu::PresentMode::NoVsync,
        };
        let swap_chain = device.create_swap_chain(&surface, &sc_desc);

        Self {
            surface,
            adapter,
            device,
            queue,
            sc_desc,
            swap_chain,
            size,
        }
    }

    pub fn create_pipeline(
        &self,
        vert_file: &Path,
        frag_file: &Path,
        bind_group_layout: &wgpu::BindGroupLayout,
    ) -> Result<Pipeline> {
        let pipeline = Pipeline::new(
            vert_file,
            frag_file,
            &self.device,
            self.sc_desc.format,
            bind_group_layout,
        )?;
        Ok(pipeline)
    }

    pub fn resize(&mut self, new_size: winit::dpi::PhysicalSize<u32>) {
        self.size = new_size;
        self.sc_desc.width = new_size.width;
        self.sc_desc.height = new_size.height;
        self.swap_chain = self.device.create_swap_chain(&self.surface, &self.sc_desc);
    }

    pub fn render(&mut self, model: &Model) {
        let frame = self.swap_chain.get_next_texture();
        let mut encoder = self
            .device
            .create_command_encoder(&wgpu::CommandEncoderDescriptor { todo: 0 });

        {
            let mut render_pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
                color_attachments: &[wgpu::RenderPassColorAttachmentDescriptor {
                    attachment: &frame.view,
                    resolve_target: None,
                    load_op: wgpu::LoadOp::Clear,
                    store_op: wgpu::StoreOp::Store,
                    clear_color: wgpu::Color {
                        r: 0.1,
                        g: 0.2,
                        b: 0.3,
                        a: 1.0,
                    },
                }],
                depth_stencil_attachment: None,
            });

            render_pass.set_pipeline(&model.pipeline.render_pipeline);
            render_pass.set_bind_group(0, &model.texture.diffuse_bind_group, &[]);
            render_pass.set_vertex_buffers(0, &[(&model.vertex_buffer, 0)]);
            render_pass.set_index_buffer(&model.index_buffer, 0);
            render_pass.draw_indexed(0..model.num_indices, 0, 0..1);
        }

        self.queue.submit(&[encoder.finish()]);
    }
}
