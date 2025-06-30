use bytemuck;
use winit::{
    dpi::PhysicalPosition,
    event::{ElementState, Event, MouseButton, WindowEvent},
    event_loop::{ControlFlow, EventLoop},
    window::WindowBuilder,
};

#[repr(C)]
#[derive(Debug, Clone, Copy, bytemuck::Pod, bytemuck::Zeroable)]
struct Uniforms {
    center: [f32; 2],
    range: [f32; 2],
    max_iter: i32,
    _padding: [u32; 3],
}

const INITIAL_ITERATIONS: i32 = 500;

pub async fn run_window() {
    let event_loop = EventLoop::new();
    let window = WindowBuilder::new()
        .with_title("Mandelbrot - True High-Quality Zoom")
        .build(&event_loop)
        .unwrap();

    let instance = wgpu::Instance::new(wgpu::InstanceDescriptor::default());
    let surface = unsafe { instance.create_surface(&window) }.unwrap();
    let adapter = instance
        .request_adapter(&wgpu::RequestAdapterOptions::default())
        .await
        .unwrap();
    let (device, queue) = adapter
        .request_device(&wgpu::DeviceDescriptor::default(), None)
        .await
        .unwrap();

    let size = window.inner_size();
    let surface_caps = surface.get_capabilities(&adapter);
    let surface_format = surface_caps
        .formats
        .iter()
        .copied()
        .find(|f| f.is_srgb())
        .unwrap_or(surface_caps.formats[0]);

    let mut config = wgpu::SurfaceConfiguration {
        usage: wgpu::TextureUsages::RENDER_ATTACHMENT,
        format: surface_format,
        width: size.width,
        height: size.height,
        present_mode: wgpu::PresentMode::Fifo,
        alpha_mode: surface_caps.alpha_modes[0],
        view_formats: vec![],
    };
    surface.configure(&device, &config);

    let mut uniforms = Uniforms {
        center: [-0.75, 0.0],
        range: [3.5, 2.0],
        max_iter: INITIAL_ITERATIONS,
        _padding: [0; 3],
    };
    let uniform_buffer = device.create_buffer(&wgpu::BufferDescriptor {
        label: Some("Uniform Buffer"),
        size: std::mem::size_of::<Uniforms>() as wgpu::BufferAddress,
        usage: wgpu::BufferUsages::UNIFORM | wgpu::BufferUsages::COPY_DST,
        mapped_at_creation: false,
    });
    queue.write_buffer(&uniform_buffer, 0, bytemuck::bytes_of(&uniforms));

    let mut mouse_pos = PhysicalPosition::new(0.0, 0.0);

    let mut storage_texture = device.create_texture(&wgpu::TextureDescriptor {
        label: Some("Mandelbrot Storage Texture"),
        size: wgpu::Extent3d {
            width: config.width,
            height: config.height,
            depth_or_array_layers: 1,
        },
        mip_level_count: 1,
        sample_count: 1,
        dimension: wgpu::TextureDimension::D2,
        format: wgpu::TextureFormat::Rgba8Unorm,
        usage: wgpu::TextureUsages::STORAGE_BINDING | wgpu::TextureUsages::TEXTURE_BINDING,
        view_formats: &[],
    });

    let shader_module = device.create_shader_module(wgpu::ShaderModuleDescriptor {
        label: Some("Mandelbrot Shader Module"),
        source: wgpu::ShaderSource::Wgsl(include_str!("shader.wgsl").into()),
    });

    let compute_uniform_bgl = device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
        label: Some("Compute Uniform BGL"),
        entries: &[wgpu::BindGroupLayoutEntry {
            binding: 0,
            visibility: wgpu::ShaderStages::COMPUTE,
            ty: wgpu::BindingType::Buffer {
                ty: wgpu::BufferBindingType::Uniform,
                has_dynamic_offset: false,
                min_binding_size: None,
            },
            count: None,
        }],
    });
    let compute_storage_bgl = device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
        label: Some("Compute Storage BGL"),
        entries: &[wgpu::BindGroupLayoutEntry {
            binding: 0,
            visibility: wgpu::ShaderStages::COMPUTE,
            ty: wgpu::BindingType::StorageTexture {
                access: wgpu::StorageTextureAccess::WriteOnly,
                format: wgpu::TextureFormat::Rgba8Unorm,
                view_dimension: wgpu::TextureViewDimension::D2,
            },
            count: None,
        }],
    });
    let compute_pipeline_layout = device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
        label: Some("Compute Pipeline Layout"),
        bind_group_layouts: &[&compute_uniform_bgl, &compute_storage_bgl],
        push_constant_ranges: &[],
    });
    let compute_pipeline = device.create_compute_pipeline(&wgpu::ComputePipelineDescriptor {
        label: Some("Compute Pipeline"),
        layout: Some(&compute_pipeline_layout),
        module: &shader_module,
        entry_point: "main_compute",
    });

    let render_bind_group_layout =
        device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
            label: Some("Render BGL"),
            entries: &[
                wgpu::BindGroupLayoutEntry {
                    binding: 0,
                    visibility: wgpu::ShaderStages::FRAGMENT,
                    ty: wgpu::BindingType::Texture {
                        sample_type: wgpu::TextureSampleType::Float { filterable: true },
                        view_dimension: wgpu::TextureViewDimension::D2,
                        multisampled: false,
                    },
                    count: None,
                },
                wgpu::BindGroupLayoutEntry {
                    binding: 1,
                    visibility: wgpu::ShaderStages::FRAGMENT,
                    ty: wgpu::BindingType::Sampler(wgpu::SamplerBindingType::Filtering),
                    count: None,
                },
            ],
        });
    let render_pipeline = device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
        label: Some("Render Pipeline"),
        layout: Some(
            &device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
                label: None,
                bind_group_layouts: &[&render_bind_group_layout],
                push_constant_ranges: &[],
            }),
        ),
        vertex: wgpu::VertexState {
            module: &shader_module,
            entry_point: "main_vertex",
            buffers: &[],
        },
        fragment: Some(wgpu::FragmentState {
            module: &shader_module,
            entry_point: "main_fragment",
            targets: &[Some(wgpu::ColorTargetState {
                format: config.format,
                blend: Some(wgpu::BlendState::REPLACE),
                write_mask: wgpu::ColorWrites::ALL,
            })],
        }),
        primitive: wgpu::PrimitiveState {
            topology: wgpu::PrimitiveTopology::TriangleStrip,
            ..wgpu::PrimitiveState::default()
        },
        depth_stencil: None,
        multisample: wgpu::MultisampleState::default(),
        multiview: None,
    });
    let sampler = device.create_sampler(&wgpu::SamplerDescriptor::default());

    event_loop.run(move |event, _, control_flow| match event {
        Event::WindowEvent { event, .. } => match event {
            WindowEvent::CloseRequested => *control_flow = ControlFlow::Exit,
            WindowEvent::Resized(new_size) => {
                if new_size.width > 0 && new_size.height > 0 {
                    config.width = new_size.width;
                    config.height = new_size.height;
                    surface.configure(&device, &config);

                    storage_texture = device.create_texture(&wgpu::TextureDescriptor {
                        label: Some("Mandelbrot Storage Texture"),
                        size: wgpu::Extent3d {
                            width: config.width,
                            height: config.height,
                            depth_or_array_layers: 1,
                        },
                        mip_level_count: 1,
                        sample_count: 1,
                        dimension: wgpu::TextureDimension::D2,
                        format: wgpu::TextureFormat::Rgba8Unorm,
                        usage: wgpu::TextureUsages::STORAGE_BINDING
                            | wgpu::TextureUsages::TEXTURE_BINDING,
                        view_formats: &[],
                    });

                    let aspect_ratio = config.width as f32 / config.height as f32;
                    uniforms.range[1] = uniforms.range[0] / aspect_ratio;
                    queue.write_buffer(&uniform_buffer, 0, bytemuck::bytes_of(&uniforms));

                    window.request_redraw();
                }
            }
            WindowEvent::CursorMoved { position, .. } => {
                mouse_pos = position;
            }

            WindowEvent::MouseInput {
                state: ElementState::Pressed,
                button,
                ..
            } => {
                let zoom_factor = match button {
                    MouseButton::Left => 0.5,
                    MouseButton::Right => 2.0,
                    _ => return,
                };
                let min_zoom_range = 6.2e-5;
                if zoom_factor < 1.0 && uniforms.range[0] < min_zoom_range {
                    println!("Zoom limit reached.");
                    return;
                }

                let norm_x = (mouse_pos.x as f32 / config.width as f32) - 0.5;
                let norm_y = (mouse_pos.y as f32 / config.height as f32) - 0.5;

                let mouse_complex_x = uniforms.center[0] + norm_x * uniforms.range[0];
                let mouse_complex_y = uniforms.center[1] - norm_y * uniforms.range[1];

                uniforms.range[0] *= zoom_factor;
                uniforms.range[1] *= zoom_factor;

                uniforms.center[0] = mouse_complex_x - norm_x * uniforms.range[0];
                uniforms.center[1] = mouse_complex_y + norm_y * uniforms.range[1];

                uniforms.max_iter = (INITIAL_ITERATIONS as f32
                    * (3.5 / uniforms.range[0]).powf(0.3))
                .clamp(128.0, 5000.0) as i32;

                println!(
                    "Zooming to center: {:?}, range: {:?}, iterations: {}",
                    uniforms.center, uniforms.range, uniforms.max_iter,
                );

                queue.write_buffer(&uniform_buffer, 0, bytemuck::bytes_of(&uniforms));
                window.request_redraw();
            }
            _ => {}
        },
        Event::RedrawRequested(_) => {
            let frame = surface
                .get_current_texture()
                .expect("Failed to get texture");
            let surface_view = frame
                .texture
                .create_view(&wgpu::TextureViewDescriptor::default());
            let storage_texture_view =
                storage_texture.create_view(&wgpu::TextureViewDescriptor::default());

            let uniform_bind_group = device.create_bind_group(&wgpu::BindGroupDescriptor {
                label: Some("Uniform BG"),
                layout: &compute_uniform_bgl,
                entries: &[wgpu::BindGroupEntry {
                    binding: 0,
                    resource: uniform_buffer.as_entire_binding(),
                }],
            });
            let storage_bind_group = device.create_bind_group(&wgpu::BindGroupDescriptor {
                label: Some("Storage BG"),
                layout: &compute_storage_bgl,
                entries: &[wgpu::BindGroupEntry {
                    binding: 0,
                    resource: wgpu::BindingResource::TextureView(&storage_texture_view),
                }],
            });
            let render_bind_group = device.create_bind_group(&wgpu::BindGroupDescriptor {
                label: Some("Render BG"),
                layout: &render_bind_group_layout,
                entries: &[
                    wgpu::BindGroupEntry {
                        binding: 0,
                        resource: wgpu::BindingResource::TextureView(&storage_texture_view),
                    },
                    wgpu::BindGroupEntry {
                        binding: 1,
                        resource: wgpu::BindingResource::Sampler(&sampler),
                    },
                ],
            });

            let mut encoder =
                device.create_command_encoder(&wgpu::CommandEncoderDescriptor::default());

            {
                let mut compute_pass =
                    encoder.begin_compute_pass(&wgpu::ComputePassDescriptor::default());

                compute_pass.set_pipeline(&compute_pipeline);
                compute_pass.set_bind_group(0, &uniform_bind_group, &[]);
                compute_pass.set_bind_group(1, &storage_bind_group, &[]);
                compute_pass.dispatch_workgroups(
                    (config.width as f32 / 8.0).ceil() as u32,
                    (config.height as f32 / 8.0).ceil() as u32,
                    1,
                );
            }

            {
                let mut render_pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
                    label: Some("Render Pass"),
                    color_attachments: &[Some(wgpu::RenderPassColorAttachment {
                        view: &surface_view,
                        resolve_target: None,
                        ops: wgpu::Operations {
                            load: wgpu::LoadOp::Clear(wgpu::Color::BLACK),
                            store: true,
                        },
                    })],
                    depth_stencil_attachment: None,
                });
                render_pass.set_pipeline(&render_pipeline);
                render_pass.set_bind_group(0, &render_bind_group, &[]);
                render_pass.draw(0..4, 0..1);
            }

            queue.submit(Some(encoder.finish()));
            frame.present();
        }
        _ => {}
    });
}
