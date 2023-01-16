extern crate nalgebra_glm as glm;

use std::time::Instant;

use egui::{FullOutput, ClippedPrimitive};
use winit::window::Window;
use wgpu::util::DeviceExt;

use egui_wgpu_backend::{RenderPass, ScreenDescriptor};

#[repr(C)]
#[derive(Copy, Clone, Debug, bytemuck::Pod, bytemuck::Zeroable)]
struct PushConstants{
    proj_view: [[f32; 4]; 4],
}

#[repr(C)]
#[derive(Copy, Clone, Debug, bytemuck::Pod, bytemuck::Zeroable)]
struct Vertex {
    position: [f32; 2]
}

const VERTICES: &[Vertex] = &[
    Vertex { position: [-0.5,  0.5] },
    Vertex { position: [-0.5, -0.5] },
    Vertex { position: [ 0.5, -0.5] },
    Vertex { position: [ 0.5,  0.5] },
];
const INDICES: &[u32] = &[
    0, 1, 3,
    3, 1, 2,
];

pub const MAX_INSTANCES: usize = 4096*4;//4096;
pub const MAX_COLORS: usize = 32;

impl Vertex {
    fn desc<'a>() -> wgpu::VertexBufferLayout<'a> {
        wgpu::VertexBufferLayout {
            array_stride: std::mem::size_of::<Vertex>() as wgpu::BufferAddress,
            step_mode: wgpu::VertexStepMode::Vertex,
            attributes: &[
                // Position
                wgpu::VertexAttribute {
                    offset: 0,
                    shader_location: 0,
                    format: wgpu::VertexFormat::Float32x2,
                },
            ]
        }
    }
}

#[repr(C)]
#[derive(Copy, Clone, bytemuck::Pod, bytemuck::Zeroable)]
struct ColorRaw {
    color: [f32; 4],
}

#[repr(C)]
#[derive(Copy, Clone, bytemuck::Pod, bytemuck::Zeroable)]
struct InstanceRaw {
    position: [f32; 2],
    radius: f32,
    color_id: u32,
}

impl InstanceRaw {
    fn desc<'a>() -> wgpu::VertexBufferLayout<'a> {
        use std::mem;
        wgpu::VertexBufferLayout {
            array_stride: mem::size_of::<InstanceRaw>() as wgpu::BufferAddress,

            step_mode: wgpu::VertexStepMode::Instance,
            attributes: &[
                wgpu::VertexAttribute {
                    offset: 0,
                    shader_location: 1,
                    format: wgpu::VertexFormat::Float32x2,
                },
                wgpu::VertexAttribute {
                    offset: mem::size_of::<[f32; 2]>() as wgpu::BufferAddress,
                    shader_location: 2,
                    format: wgpu::VertexFormat::Float32,
                },
                wgpu::VertexAttribute {
                    offset: mem::size_of::<[f32; 3]>() as wgpu::BufferAddress,
                    shader_location: 3,
                    format: wgpu::VertexFormat::Uint32,
                },
            ],
        }
    }
}

#[derive(Clone)]
pub struct Instance {
    pub position: glm::Vec2,
    pub radius: f32,
    pub color_id: u32
}

impl Instance {
    fn to_raw(&self) -> InstanceRaw {
        InstanceRaw {
            position: self.position.into(),
            radius: self.radius,
            color_id: self.color_id
        }
    }
}

pub struct Renderer {
    pub device:  wgpu::Device,
    pub queue:   wgpu::Queue,

    surface: wgpu::Surface,
    config: wgpu::SurfaceConfiguration,
    pub size: winit::dpi::PhysicalSize<u32>,
    pub scale_factor: f64,

    pipeline: wgpu::RenderPipeline,

    vertex_buffer: wgpu::Buffer,
    index_buffer:  wgpu::Buffer,

    instances: Vec<InstanceRaw>,
    instance_buffer: wgpu::Buffer,

    colors_buffer: wgpu::Buffer,
    colors_bind_group: wgpu::BindGroup,

    egui_render_pass: RenderPass,
}

impl Renderer {
    pub async fn new(window: &Window, colors: &Vec<glm::Vec3>) -> Self {
        let size = window.inner_size();
        let scale_factor = window.scale_factor();

        let instance = wgpu::Instance::new(wgpu::Backends::DX12 | wgpu::Backends::VULKAN | wgpu::Backends::METAL);

        let surface = unsafe { instance.create_surface(&window) };

        let adapter = instance.request_adapter(
            &wgpu::RequestAdapterOptions {
                power_preference: wgpu::PowerPreference::HighPerformance,
                compatible_surface: Some(&surface),
                force_fallback_adapter: false,
            },
        ).await.unwrap();

        let (device, queue) = adapter.request_device(
            &wgpu::DeviceDescriptor {
                features: 
                    wgpu::Features::PUSH_CONSTANTS | 
                    wgpu::Features::TEXTURE_BINDING_ARRAY | 
                    wgpu::Features::SAMPLED_TEXTURE_AND_STORAGE_BUFFER_ARRAY_NON_UNIFORM_INDEXING,

                limits: adapter.limits(),
                
                label: None,
            },
            None,
        ).await.unwrap();

        let config = wgpu::SurfaceConfiguration {
            usage: wgpu::TextureUsages::RENDER_ATTACHMENT,
            format: wgpu::TextureFormat::Bgra8UnormSrgb,
            width: size.width,
            height: size.height,
            present_mode: wgpu::PresentMode::Fifo,
        };

        surface.configure(&device, &config);

        let shader = device.create_shader_module(wgpu::include_wgsl!("shader.wgsl"));

        let bind_group_layout = device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
            entries: &[
                    wgpu::BindGroupLayoutEntry {
                        binding: 0,
                        visibility: wgpu::ShaderStages::VERTEX,
                        ty: wgpu::BindingType::Buffer { 
                            ty: wgpu::BufferBindingType::Uniform,
                            has_dynamic_offset: false,
                            min_binding_size: None
                        },
                        count: None,
                    },
                ],
                label: Some("texture_bind_group_layout"),
        });

        let push_constant_range = wgpu::PushConstantRange {
            stages: wgpu::ShaderStages::VERTEX,
            range: 0..std::mem::size_of::<PushConstants>() as u32,
        };

        let pipeline_layout = device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
            label: Some("Main Render Pipeline Layout"),
            bind_group_layouts: &[&bind_group_layout],
            push_constant_ranges: &[push_constant_range],
        });

        let pipeline = device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
            label: Some("Render Pipeline"),
            layout: Some(&pipeline_layout),

            vertex: wgpu::VertexState {
                module: &shader,
                entry_point: "vs_main", 
                buffers: &[Vertex::desc(), InstanceRaw::desc()],
            },

            fragment: Some(wgpu::FragmentState {
                module: &shader,
                entry_point: "fs_main",
                targets: &[Some(wgpu::ColorTargetState {
                    format: config.format,
                    blend: Some(wgpu::BlendState::ALPHA_BLENDING),
                    write_mask: wgpu::ColorWrites::ALL,
                })],
            }),

            primitive: wgpu::PrimitiveState {
                topology: wgpu::PrimitiveTopology::TriangleList,
                strip_index_format: None,
                front_face: wgpu::FrontFace::Ccw,
                cull_mode: Some(wgpu::Face::Back),
                polygon_mode: wgpu::PolygonMode::Fill,
                unclipped_depth: false,
                conservative: false,
            },

            depth_stencil: None,

            multisample: wgpu::MultisampleState {
                count: 1,
                mask: !0,
                alpha_to_coverage_enabled: false,
            },
            multiview: None,
        });

        let vertex_buffer = device.create_buffer_init(
            &wgpu::util::BufferInitDescriptor {
                label: Some("Vertex Buffer"),
                contents: bytemuck::cast_slice(VERTICES),
                usage: wgpu::BufferUsages::VERTEX,
            }
        );
        let index_buffer = device.create_buffer_init(
            &wgpu::util::BufferInitDescriptor {
                label: Some("Index Buffer"),
                contents: bytemuck::cast_slice(INDICES),
                usage: wgpu::BufferUsages::INDEX,
            }
        );

        let mut instances = Vec::new();
        instances.reserve(MAX_INSTANCES);
        
        let alloc_data = (0..MAX_INSTANCES).map(|_|{
            Instance {
               position: glm::Vec2::identity(), 
               radius: 0.0, 
               color_id: 0
            }
        }).collect::<Vec<_>>();

        let instance_data = alloc_data.iter().map(Instance::to_raw).collect::<Vec<_>>();
        let instance_buffer = device.create_buffer_init(
            &wgpu::util::BufferInitDescriptor {
                label: Some("Instance Buffer"),
                contents: bytemuck::cast_slice(&instance_data),

                usage: wgpu::BufferUsages::VERTEX | wgpu::BufferUsages::COPY_DST,
            }
        );

        let mut colors_data = vec![ColorRaw{color: [0.0; 4]}; MAX_COLORS];

        for i in 0..colors.len() {
            let c = colors[i];
            colors_data[i] = ColorRaw{color: [c.x, c.y, c.z, 0.0]};
        }

        let colors_buffer = device.create_buffer_init(
            &wgpu::util::BufferInitDescriptor {
                label: Some("Colors Buffer"),
                contents: bytemuck::cast_slice(&colors_data),

                usage: wgpu::BufferUsages::UNIFORM | wgpu::BufferUsages::COPY_DST,
            }
        );

        let colors_bind_group = device.create_bind_group(&wgpu::BindGroupDescriptor {
            layout: &bind_group_layout,
            entries: &[
                wgpu::BindGroupEntry {
                    binding: 0,
                    resource: colors_buffer.as_entire_binding(),
                }
            ],
            label: Some("Colors Bind Group"),
        });
    
        // We use the egui_wgpu_backend crate as the render backend.
        let egui_render_pass = RenderPass::new(&device, config.format, 1);

        Self {
            device,
            queue,

            surface,
            config,
            size,
            scale_factor,

            pipeline,

            vertex_buffer,
            index_buffer,

            instances,
            instance_buffer,

            colors_buffer,
            colors_bind_group,

            egui_render_pass
        }
    }

    pub fn reset_queue(&mut self) {
        self.instances.clear();
    }

    pub fn enqueue_instance(&mut self, instance: Instance) {
        self.instances.push(instance.to_raw());
    }
 
    pub fn update_colors(&mut self, colors: &Vec<glm::Vec3>) {
        let mut colors_data = vec![ColorRaw{color: [0.0; 4]}; MAX_COLORS];

        for i in 0..colors.len() {
            let c = colors[i];
            colors_data[i] = ColorRaw{color: [c.x, c.y, c.z, 0.0]};
        }

        let encoder = self.device.create_command_encoder(&wgpu::CommandEncoderDescriptor {
            label: Some("update_colors_encoder"),
        });

        self.queue.write_buffer(&self.colors_buffer, 0, bytemuck::cast_slice(&colors_data));
        self.queue.submit(std::iter::once(encoder.finish()));
    }

    pub fn resize(&mut self, new_size: winit::dpi::PhysicalSize<u32>, scale_factor: f64) {
        if new_size.width > 0 && new_size.height > 0 {
            self.size = new_size;
            self.scale_factor = scale_factor;
            self.config.width = new_size.width;
            self.config.height = new_size.height;
            self.surface.configure(&self.device, &self.config);
        }
    }

    pub fn render(&mut self, proj_view: glm::Mat4, frame_data: Option<(FullOutput, Vec<ClippedPrimitive>)>) -> Result<(), wgpu::SurfaceError> {
        self.queue.write_buffer(&self.instance_buffer, 0, bytemuck::cast_slice(&self.instances));

        let output = self.surface.get_current_texture()?;
        let view = output.texture.create_view(&wgpu::TextureViewDescriptor::default()); 

        let mut encoder = self.device.create_command_encoder(&wgpu::CommandEncoderDescriptor {
            label: Some("Render Encoder"),
        });

        {
            let mut render_pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
                label: Some("Render Pass"),
                color_attachments: &[Some(wgpu::RenderPassColorAttachment {
                    view: &view,
                    resolve_target: None,
                    ops: wgpu::Operations {
                        load: wgpu::LoadOp::Clear(wgpu::Color {
                            r: 0.0,
                            g: 0.0,
                            b: 0.0,
                            a: 1.0,
                        }),
                        store: true,
                    },
                })],
                depth_stencil_attachment: None,
            });

            render_pass.set_pipeline(&self.pipeline);

            render_pass.set_vertex_buffer(0, self.vertex_buffer.slice(..));
            render_pass.set_vertex_buffer(1, self.instance_buffer.slice(..));
            render_pass.set_bind_group(0, &self.colors_bind_group, &[]);

            render_pass.set_index_buffer(self.index_buffer.slice(..), wgpu::IndexFormat::Uint32); 
            
            render_pass.set_push_constants(wgpu::ShaderStages::VERTEX, 0, bytemuck::bytes_of(&PushConstants{proj_view: proj_view.into()}));

            render_pass.draw_indexed(0..INDICES.len() as u32, 0, 0..self.instances.len() as u32);
        }

        let start;

        if frame_data.is_some() {
            let screen_descriptor = ScreenDescriptor {
                physical_width: self.config.width,
                physical_height: self.config.height,
                scale_factor: self.scale_factor as f32,
            };
    
            let (full_output, paint_jobs) = frame_data.unwrap();
    
            self.egui_render_pass.add_textures(&self.device, &self.queue, &full_output.textures_delta).unwrap();
            self.egui_render_pass.update_buffers(&self.device, &self.queue, &paint_jobs, &screen_descriptor);

            self.egui_render_pass.execute(
                &mut encoder,
                &view,
                &paint_jobs,
                &screen_descriptor,
                None,
            ).unwrap();

            start = Instant::now();
            self.queue.submit(std::iter::once(encoder.finish()));
            output.present();

            self.egui_render_pass.remove_textures(full_output.textures_delta).unwrap(); 
        } else {
            start = Instant::now();
            self.queue.submit(std::iter::once(encoder.finish()));
            output.present();
        }

        let submit = start.elapsed().as_secs_f64()*1000.0;

        println!("GPU took: {:.2}ms", submit);

        self.reset_queue();

        Ok(())
    }
}