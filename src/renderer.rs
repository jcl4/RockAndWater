use crate::{
    objects::{Cube, Object},
    Result,
};
use std::path::Path;
use winit::{dpi::PhysicalSize, window::Window};

pub mod pipeline;
pub mod texture;
pub use texture::Texture;

pub struct Renderer {
    surface: wgpu::Surface,
    _adapter: wgpu::Adapter,
    pub device: wgpu::Device,
    pub queue: wgpu::Queue,
    sc_desc: wgpu::SwapChainDescriptor,
    swap_chain: wgpu::SwapChain,
    size: PhysicalSize<u32>,
    bg_color: wgpu::Color,
}

impl Renderer {
    pub async fn new(window: &Window, bg_color: [f32; 4]) -> Self {
        let size = window.inner_size();

        let surface = wgpu::Surface::create(window);
        let adapter = wgpu::Adapter::request(&wgpu::RequestAdapterOptions {
            power_preference: wgpu::PowerPreference::HighPerformance,
            compatible_surface: Some(&surface),
        }, wgpu::BackendBit::VULKAN)
        .await.unwrap();

        let (device, queue) = adapter.request_device(&wgpu::DeviceDescriptor {
            extensions: wgpu::Extensions {
                anisotropic_filtering: false,
            },
            limits: Default::default(),
        }).await;

        let sc_desc = wgpu::SwapChainDescriptor {
            usage: wgpu::TextureUsage::OUTPUT_ATTACHMENT,
            format: wgpu::TextureFormat::Bgra8UnormSrgb,
            width: size.width,
            height: size.height,
            present_mode: wgpu::PresentMode::Immediate,
        };
        let swap_chain = device.create_swap_chain(&surface, &sc_desc);
        let bg_color = wgpu::Color {
            r: bg_color[0] as f64,
            g: bg_color[1] as f64,
            b: bg_color[2] as f64,
            a: bg_color[3] as f64,
        };

        Self {
            surface,
            _adapter: adapter,
            device,
            queue,
            sc_desc,
            swap_chain,
            size,
            bg_color,
        }
    }

    pub fn create_pipeline(
        &self,
        vert_file: &Path,
        frag_file: &Path,
        vertex_buffer_descriptor: wgpu::VertexBufferDescriptor,
    ) -> Result<wgpu::RenderPipeline> {
        pipeline::create_pipeline(
            vert_file,
            frag_file,
            vertex_buffer_descriptor,
            self.sc_desc.format,
            &self.device,
        )
    }

    pub fn resize(&mut self, new_size: winit::dpi::PhysicalSize<u32>) {
        self.size = new_size;
        self.sc_desc.width = new_size.width;
        self.sc_desc.height = new_size.height;
        self.swap_chain = self.device.create_swap_chain(&self.surface, &self.sc_desc);
    }

    pub fn init_clear_screen(&mut self) {
        // TODO: fix unwrap
        let frame = self.swap_chain.get_next_texture().unwrap();

        let mut encoder = get_command_encoder(&self.device);
        {
            let _render_pass = begin_render_pass(&mut encoder, &frame, self.bg_color);
        }
        submit_frame(&mut self.queue, encoder);
    }

    pub fn render(&mut self, cube: &Cube) {
        // TODO: fix unwrap
        let frame = self.swap_chain.get_next_texture().unwrap();
        let mut encoder = get_command_encoder(&self.device);
        {
            let mut render_pass = begin_render_pass(&mut encoder, &frame, self.bg_color);
            // cube.render(&mut render_pass);
            render_pass.set_pipeline(&cube.pipeline);
            render_pass.set_vertex_buffer(0, &cube.vertex_buffer, 0, 0);
            render_pass.set_index_buffer(&cube.index_buffer, 0, 0);
            render_pass.draw_indexed(0..cube.num_indices, 0, 0..1);
        }
        submit_frame(&mut self.queue, encoder);
    }

    // pub fn get_command_encoder(&self) -> wgpu::CommandEncoder {

    //     // render_pass.set_pipeline(&model.pipeline.render_pipeline);
    //     // render_pass.set_bind_group(0, &model.texture.diffuse_bind_group, &[]);
    //     // render_pass.set_vertex_buffers(0, &[(&model.vertex_buffer, 0)]);
    //     // render_pass.set_index_buffer(&model.index_buffer, 0);
    //     // render_pass.draw_indexed(0..model.num_indices, 0, 0..1);
    // }

    // pub fn begin_render_pass<'a>(
    //     &self,
    //     encoder: &'a mut wgpu::CommandEncoder,
    //     frame: &wgpu::SwapChainOutput,
    // ) -> wgpu::RenderPass<'a> {
    //     let render_pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
    //         color_attachments: &[wgpu::RenderPassColorAttachmentDescriptor {
    //             attachment: &frame.view,
    //             resolve_target: None,
    //             load_op: wgpu::LoadOp::Clear,
    //             store_op: wgpu::StoreOp::Store,
    //             clear_color: wgpu::Color {
    //                 r: 0.1,
    //                 g: 0.2,
    //                 b: 0.3,
    //                 a: 1.0,
    //             },
    //         }],
    //         depth_stencil_attachment: None,
    //     });
    //     render_pass
    // }

    // pub fn submit_frame(&mut self, encoder: wgpu::CommandEncoder) {
    //     self.queue.submit(&[encoder.finish()]);
    // }
}

fn get_command_encoder(device: &wgpu::Device) -> wgpu::CommandEncoder {
    device.create_command_encoder(&wgpu::CommandEncoderDescriptor { label: Some("comand_encoder")})
}

fn begin_render_pass<'a>(
    encoder: &'a mut wgpu::CommandEncoder,
    frame: &'a wgpu::SwapChainOutput,
    bg_color: wgpu::Color,
) -> wgpu::RenderPass<'a> {
    encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
        color_attachments: &[wgpu::RenderPassColorAttachmentDescriptor {
            attachment: &frame.view,
            resolve_target: None,
            load_op: wgpu::LoadOp::Clear,
            store_op: wgpu::StoreOp::Store,
            clear_color: bg_color,
        }],
        depth_stencil_attachment: None,
    })
}

fn submit_frame(queue: &mut wgpu::Queue, encoder: wgpu::CommandEncoder) {
    queue.submit(&[encoder.finish()]);
}
