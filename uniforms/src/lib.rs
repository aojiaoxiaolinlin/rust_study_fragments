use std::path::Path;
use wgpu::util::BufferInitDescriptor;
use wgpu::{util::DeviceExt, SurfaceError};
use winit::window::Window;
pub struct State<'window> {
    surface: wgpu::Surface<'window>,
    config: wgpu::SurfaceConfiguration,
    device: wgpu::Device,
    queue: wgpu::Queue,
    render_pipeline: wgpu::RenderPipeline,
}

impl<'window> State<'window> {
    pub fn new(window: &Window) -> anyhow::Result<Self> {
        let (instance, backend) = Self::create_wgpu_instance().unwrap();
        let surface = unsafe {
            instance.create_surface_unsafe(wgpu::SurfaceTargetUnsafe::from_window(window)?)
        }?;
        let (adapter, device, queue) = futures::executor::block_on(request_adapter_and_device(
            backend,
            &instance,
            Some(&surface),
        ))
        .map_err(|e| anyhow::anyhow!("请求设备失败: {:?}", e.to_string()))?;

        let surface_format = surface
            .get_capabilities(&adapter)
            .formats
            .first()
            .cloned()
            .expect("没有可用的表面格式");

        let config = wgpu::SurfaceConfiguration {
            usage: wgpu::TextureUsages::RENDER_ATTACHMENT,
            format: surface_format,
            width: window.inner_size().width,
            height: window.inner_size().height,
            present_mode: Default::default(),
            alpha_mode: Default::default(),
            view_formats: Default::default(),
            desired_maximum_frame_latency: 2,
        };
        surface.configure(&device, &config);

        let shader = device.create_shader_module(wgpu::ShaderModuleDescriptor {
            label: Some("着色器"),
            source: wgpu::ShaderSource::Wgsl(include_str!("../wgsls/shader.wgsl").into()),
        });

        let render_pipeline_layout =
            device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
                label: Some("渲染管线布局"),
                bind_group_layouts: &[],
                push_constant_ranges: &[],
            });
        let uniform_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor{
            label: Some("Uniform Buffer"),
            usage: wgpu::BufferUsages::UNIFORM | wgpu::BufferUsages::COPY_DST,
            contents: bytemuck::cast_slice(&[0.5, 0.5, 0.5, 0.5]),
        });
        let render_pipeline = device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
            label: Some("渲染管线"),
            layout: Some(&render_pipeline_layout),
            vertex: wgpu::VertexState {
                module: &shader,
                entry_point: "v_main",
                compilation_options: Default::default(),
                buffers: &[],
            },
            primitive: wgpu::PrimitiveState {
                topology: wgpu::PrimitiveTopology::TriangleList,
                strip_index_format: None,
                front_face: wgpu::FrontFace::Ccw,
                cull_mode: Some(wgpu::Face::Back),
                unclipped_depth: false,
                polygon_mode: wgpu::PolygonMode::Fill,
                conservative: false,
            },
            depth_stencil: None,
            multisample: wgpu::MultisampleState {
                count: 1,
                mask: !0,
                alpha_to_coverage_enabled: false,
            },
            fragment: Some(wgpu::FragmentState {
                module: &shader,
                entry_point: "f_main",
                compilation_options: Default::default(),
                targets: &[Some(wgpu::ColorTargetState {
                    format: config.format,
                    blend: Some(wgpu::BlendState::REPLACE),
                    write_mask: wgpu::ColorWrites::ALL,
                })],
            }),
            multiview: None,
        });
        Ok(Self {
            surface,
            config,
            device,
            queue,
            render_pipeline
        })
    }

    pub fn resize(&mut self, new_size: winit::dpi::PhysicalSize<u32>) {
        if new_size.width > 0 && new_size.height > 0 {
            // 重新设置窗口大小
            self.config.width = new_size.width;
            self.config.height = new_size.height;
            self.surface.configure(&self.device, &self.config)
        }
    }
    pub fn render(&mut self) -> Result<(), SurfaceError> {
        let surface_texture = self.surface.get_current_texture()?;
        let view = surface_texture
            .texture
            .create_view(&wgpu::TextureViewDescriptor::default());
        let mut encoder = self
            .device
            .create_command_encoder(&wgpu::CommandEncoderDescriptor {
                label: Some("Command Encoder"),
            });
        {
            let mut render_pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
                label: Some("Render Pass"),
                color_attachments: &[Some(wgpu::RenderPassColorAttachment {
                    view: &view,
                    resolve_target: None,
                    ops: wgpu::Operations {
                        load: wgpu::LoadOp::Clear(wgpu::Color {
                            r: 0.1,
                            g: 0.2,
                            b: 0.3,
                            a: 1.0,
                        }),
                        store: wgpu::StoreOp::Store,
                    },
                })],
                ..Default::default()
            });
            render_pass.set_pipeline(&self.render_pipeline);
            
            render_pass.draw(0..3, 0..1);
        }
        self.queue.submit(std::iter::once(encoder.finish()));
        surface_texture.present();
        Ok(())
    }

    fn create_wgpu_instance() -> anyhow::Result<(wgpu::Instance, wgpu::Backends)> {
        for backend in wgpu::Backends::all() {
            if let Some(instance) = try_wgpu_backend(backend) {
                // 如果实例创建成功，返回实例和后端
                return Ok((instance, backend));
            }
        }
        Err(anyhow::anyhow!("没有找到可用的后端"))
    }
}

fn try_wgpu_backend(backend: wgpu::Backends) -> Option<wgpu::Instance> {
    let instance = wgpu::Instance::new(wgpu::InstanceDescriptor {
        backends: backend,
        flags: wgpu::InstanceFlags::default().with_env(),
        ..Default::default()
    });
    if instance.enumerate_adapters(backend).is_empty() {
        None
    } else {
        Some(instance)
    }
}

async fn request_adapter_and_device(
    _backend: wgpu::Backends,
    instance: &wgpu::Instance,
    surface: Option<&wgpu::Surface<'static>>,
) -> anyhow::Result<(wgpu::Adapter, wgpu::Device, wgpu::Queue)> {
    let adapter = instance
        .request_adapter(&wgpu::RequestAdapterOptions {
            power_preference: wgpu::PowerPreference::HighPerformance,
            compatible_surface: surface,
            force_fallback_adapter: false,
        })
        .await
        .ok_or_else(|| anyhow::anyhow!("没有找到可用的适配器"))?;

    let (device, queue) = request_device(&adapter, None).await?;
    dbg!(adapter.get_info());
    Ok((adapter, device, queue))
}

async fn request_device(
    adapter: &wgpu::Adapter,
    trace_path: Option<&Path>,
) -> Result<(wgpu::Device, wgpu::Queue), wgpu::RequestDeviceError> {
    adapter
        .request_device(
            &wgpu::DeviceDescriptor {
                label: Some("设备"),
                required_features: wgpu::Features::empty(),
                required_limits: wgpu::Limits::default(),
            },
            trace_path,
        )
        .await
}
