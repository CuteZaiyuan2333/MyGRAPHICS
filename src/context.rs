use winit::{
    event_loop::EventLoop,
    window::{Window, WindowBuilder},
};
use wgpu::{Instance, Surface, Adapter, Device, Queue, SurfaceConfiguration};

pub struct WgpuContext {
    // Drop order matters! Surface must be dropped before Window.
    // Fields are dropped in declaration order.
    pub surface: Surface<'static>,
    pub window: Window,
    pub event_loop: EventLoop<()>,
    
    pub instance: Instance,
    pub adapter: Adapter,
    pub device: Device,
    pub queue: Queue,
    pub config: SurfaceConfiguration,
}

impl WgpuContext {
    pub async fn new(title: &str, width: u32, height: u32) -> Self {
        let event_loop = EventLoop::new().unwrap();
        let window = WindowBuilder::new()
            .with_title(title)
            .with_inner_size(winit::dpi::LogicalSize::new(width, height))
            .build(&event_loop)
            .unwrap();

        let instance = Instance::new(wgpu::InstanceDescriptor::default());
        
        // Safety: We force the surface lifetime to 'static.
        // We ensure safety by placing `surface` before `window` in the struct,
        // so `surface` is dropped first.
        let surface = unsafe { 
            let surface = instance.create_surface(&window).unwrap();
            std::mem::transmute::<Surface<'_>, Surface<'static>>(surface)
        };

        let adapter = instance.request_adapter(&wgpu::RequestAdapterOptions {
            power_preference: wgpu::PowerPreference::default(),
            compatible_surface: Some(&surface),
            force_fallback_adapter: false,
        }).await.unwrap();

        let (device, queue) = adapter.request_device(
            &wgpu::DeviceDescriptor {
                label: None,
                required_features: wgpu::Features::empty(),
                required_limits: wgpu::Limits::default(),
            },
            None,
        ).await.unwrap();

        let size = window.inner_size();
        let config = surface.get_default_config(&adapter, size.width, size.height).unwrap();
        surface.configure(&device, &config);

        Self {
            surface,
            window,
            event_loop,
            instance,
            adapter,
            device,
            queue,
            config,
        }
    }
}
