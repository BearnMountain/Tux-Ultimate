use std::sync::Arc;

use winit::{dpi::PhysicalSize, window::Window};

use crate::util::config::Config;

// context of how graphics are rendered on the main window
pub struct RenderContext {
    pub window: Arc<Window>,
    pub instance: wgpu::Instance,
    pub surface: wgpu::Surface<'static>,
    pub device: wgpu::Device,
    pub queue: wgpu::Queue,
    pub config: wgpu::SurfaceConfiguration,
    pub size: PhysicalSize<u32>,
}

impl RenderContext {
    pub async fn new(window: Arc<Window>) -> Self {
        let global_config = Config::get().read().unwrap();

        let size = window.inner_size();

        // descriptor to handle gpu
        let instance = wgpu::Instance::new(wgpu::InstanceDescriptor {
            backends: match global_config.graphics.backend.as_str() {
                "Metal" => wgpu::Backends::METAL,
                "Vulkan" => wgpu::Backends::VULKAN,
                "DX12" => wgpu::Backends::DX12,
                _ => {
                    log::error!("Failed to instance a backend: {}, only supports 
                        'Metal', 'Vulkan', or 'DX12', format is enforced", 
                        global_config.graphics.backend);
                    log::error!("Instance backend falling back to default");
                    wgpu::Backends::all()
                }
            },
            flags: Default::default(),
            memory_budget_thresholds: Default::default(),
            backend_options: Default::default(),
            display: None,
        });

        // creates winit surface for graphical texture rendering
        // *** FURTURE TEXTURE CAN BE USED WITH ImGui::Image
        // *** TO GET GAME VIEW ONTOP OF UI
        let surface = instance.create_surface(window.clone()).unwrap();

        // select physical gpu(igpu or main graphics card)
        let adapter = instance.request_adapter(&wgpu::RequestAdapterOptionsBase {
            power_preference: wgpu::PowerPreference::default(),
            force_fallback_adapter: false,
            compatible_surface: Some(&surface),
            apply_limit_buckets: Default::default(),
        }).await.unwrap();

        // logical device and command queue for graphic calls
        let (device, queue) = adapter.request_device(&wgpu::DeviceDescriptor {
            label: Some("ContextDevice"),
            required_features: wgpu::Features::empty(),
            required_limits: wgpu::Limits::default(),
            experimental_features: wgpu::ExperimentalFeatures::disabled(),
            memory_hints: Default::default(),
            trace: wgpu::Trace::Off,
        }).await.unwrap();

        // format for surface
        let surface_capabilities = surface.get_capabilities(&adapter);
        let surface_format = surface_capabilities.formats.iter()
            .copied().filter(|f| f.is_srgb())
            .next().unwrap_or(surface_capabilities.formats[0]);

        let config = wgpu::SurfaceConfiguration {
            usage: wgpu::TextureUsages::RENDER_ATTACHMENT,
            format: surface_format,
            color_space: wgpu::SurfaceColorSpace::default(),
            width: size.width,
            height: size.height,
            present_mode: surface_capabilities.present_modes[0],
            desired_maximum_frame_latency: 2,
            alpha_mode: surface_capabilities.alpha_modes[0],
            view_formats: vec![],
        };
        surface.configure(&device, &config);
   
        return Self {
            window,
            instance,
            surface,
            device,
            queue,
            config,
            size,
        };
    }

    pub fn resize(&mut self, width: u32, height: u32) {
        if width > 0 && height > 0 {
            let max = 2048;
            self.config.width = width.min(max);
            self.config.height = height.min(max);
            self.surface.configure(&self.device, &self.config);
        }
    }

    pub fn update_surface(&mut self) {
        self.surface = self.instance.create_surface(self.window.clone()).unwrap();
        self.surface.configure(&self.device, &self.config);
    }
}
