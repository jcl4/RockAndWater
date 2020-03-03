use crate::Result;
use image::GenericImageView;
use std::path::Path;

pub struct Texture {
    pub diffuse_texture: wgpu::Texture,
    pub diffuse_texture_view: wgpu::TextureView,
    pub diffuse_sampler: wgpu::Sampler,
    pub diffuse_bind_group: wgpu::BindGroup,
    pub diffuse_bind_group_layout: wgpu::BindGroupLayout,
}

impl Texture {
    pub fn new(file: &Path, device: &wgpu::Device, queue: &mut wgpu::Queue) -> Result<Self> {
        let texture = image::open(file)?;
        println!("Dimensions {:?}", texture.dimensions());
        println!("Color Type: {:?}", texture.color());
        let texture_size = texture.dimensions();
        let size = wgpu::Extent3d {
            width: texture_size.0,
            height: texture_size.1,
            depth: 1,
        };
        let texture_rgba = texture.into_rgba();

        let diffuse_texture = device.create_texture(&wgpu::TextureDescriptor {
            size,
            array_layer_count: 1,
            mip_level_count: 1,
            sample_count: 1,
            dimension: wgpu::TextureDimension::D2,
            format: wgpu::TextureFormat::Rgba8UnormSrgb,
            usage: wgpu::TextureUsage::SAMPLED | wgpu::TextureUsage::COPY_DST,
        });

        let diffuse_buffer = device
            .create_buffer_mapped(texture_rgba.len(), wgpu::BufferUsage::COPY_SRC)
            .fill_from_slice(&texture_rgba);

        let mut encoder =
            device.create_command_encoder(&wgpu::CommandEncoderDescriptor { todo: 0 });

        encoder.copy_buffer_to_texture(
            wgpu::BufferCopyView {
                buffer: &diffuse_buffer,
                offset: 0,
                row_pitch: 4 * texture_size.0,
                image_height: texture_size.1,
            },
            wgpu::TextureCopyView {
                texture: &diffuse_texture,
                mip_level: 0,
                array_layer: 0,
                origin: wgpu::Origin3d::ZERO,
            },
            size,
        );

        queue.submit(&[encoder.finish()]);

        let diffuse_texture_view = diffuse_texture.create_default_view();

        let diffuse_sampler = device.create_sampler(&wgpu::SamplerDescriptor {
            address_mode_u: wgpu::AddressMode::ClampToEdge,
            address_mode_v: wgpu::AddressMode::ClampToEdge,
            address_mode_w: wgpu::AddressMode::ClampToEdge,
            mag_filter: wgpu::FilterMode::Linear,
            min_filter: wgpu::FilterMode::Nearest,
            mipmap_filter: wgpu::FilterMode::Nearest,
            lod_min_clamp: -100.0,
            lod_max_clamp: 100.0,
            compare_function: wgpu::CompareFunction::Always,
        });

        let texture_bind_group_layout =
            device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
                bindings: &[
                    wgpu::BindGroupLayoutBinding {
                        binding: 0,
                        visibility: wgpu::ShaderStage::FRAGMENT,
                        ty: wgpu::BindingType::SampledTexture {
                            multisampled: false,
                            dimension: wgpu::TextureViewDimension::D2,
                        },
                    },
                    wgpu::BindGroupLayoutBinding {
                        binding: 1,
                        visibility: wgpu::ShaderStage::FRAGMENT,
                        ty: wgpu::BindingType::Sampler,
                    },
                ],
            });

        let diffuse_bind_group = device.create_bind_group(&wgpu::BindGroupDescriptor {
            layout: &texture_bind_group_layout,
            bindings: &[
                wgpu::Binding {
                    binding: 0,
                    resource: wgpu::BindingResource::TextureView(&diffuse_texture_view),
                },
                wgpu::Binding {
                    binding: 1,
                    resource: wgpu::BindingResource::Sampler(&diffuse_sampler),
                },
            ],
        });

        Ok(Texture {
            diffuse_texture,
            diffuse_texture_view,
            diffuse_sampler,
            diffuse_bind_group,
            diffuse_bind_group_layout: texture_bind_group_layout,
        })
    }
}
