use anyhow::Context;

pub struct Renderer {
    pub adapter_name: String,
    pub _device: wgpu::Device,
    pub _queue: wgpu::Queue,
}

impl Renderer {
    pub async fn bootstrap() -> anyhow::Result<Self> {
        let instance = wgpu::Instance::default();
        let adapter = instance
            .request_adapter(&wgpu::RequestAdapterOptions {
                power_preference: wgpu::PowerPreference::HighPerformance,
                force_fallback_adapter: false,
                compatible_surface: None,
            })
            .await
            .context("no suitable GPU adapter")?;

        let adapter_name = adapter.get_info().name;
        let (device, queue) = adapter
            .request_device(
                &wgpu::DeviceDescriptor {
                    label: Some("fps-device"),
                    required_features: wgpu::Features::empty(),
                    required_limits: wgpu::Limits::default(),
                },
                None,
            )
            .await?;

        Ok(Self {
            adapter_name,
            _device: device,
            _queue: queue,
        })
    }
}
