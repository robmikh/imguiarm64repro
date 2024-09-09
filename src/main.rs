use std::time::Duration;

use imgui::{Condition, FontSource};
use imgui_wgpu::RendererConfig;
use winit::{dpi::LogicalSize, event_loop::EventLoop, window::Window};

fn main() {
    let event_loop = EventLoop::new().unwrap();

    let window = Window::new(&event_loop).unwrap();
    let _ = window.request_inner_size(LogicalSize::<f32>::new(1447.0, 867.0));
    let size = window.inner_size();

    let wgpu_backend = if cfg!(target_os = "windows") {
        wgpu::Backends::DX12
    } else {
        wgpu::Backends::all()
    };
    let instance = wgpu::Instance::new(wgpu::InstanceDescriptor {
        //backends: wgpu::Backends::all(),
        backends: wgpu_backend,
        ..Default::default()
    });
    let surface = instance.create_surface(&window).unwrap();

    let adapter = pollster::block_on(instance.request_adapter(&wgpu::RequestAdapterOptions {
        power_preference: wgpu::PowerPreference::HighPerformance,
        compatible_surface: None,
        force_fallback_adapter: false,
    }))
    .unwrap();
    let adapter_info = adapter.get_info();
    println!("Adapter: {:#?}", adapter_info);
    let (device, queue) =
        pollster::block_on(adapter.request_device(&wgpu::DeviceDescriptor::default(), None))
            .unwrap();

    let surface_config = wgpu::SurfaceConfiguration {
        usage: wgpu::TextureUsages::RENDER_ATTACHMENT,
        format: wgpu::TextureFormat::Rgba8Unorm,
        width: size.width as u32,
        height: size.height as u32,
        present_mode: wgpu::PresentMode::Mailbox,
        alpha_mode: wgpu::CompositeAlphaMode::Auto,
        view_formats: vec![wgpu::TextureFormat::Rgba8Unorm],
        desired_maximum_frame_latency: 0,
    };
    surface.configure(&device, &surface_config);

    let mut imgui = imgui::Context::create();
    let mut platform = imgui_winit_support::WinitPlatform::init(&mut imgui);
    platform.attach_window(
        imgui.io_mut(),
        &window,
        imgui_winit_support::HiDpiMode::Default,
    );
    imgui.set_ini_filename(None);

    let hidpi_factor = window.scale_factor();
    let font_size = (13.0 * hidpi_factor) as f32;
    imgui.io_mut().font_global_scale = (1.0 / hidpi_factor) as f32;

    imgui.fonts().add_font(&[FontSource::DefaultFontData {
        config: Some(imgui::FontConfig {
            oversample_h: 1,
            pixel_snap_h: true,
            size_pixels: font_size,
            ..Default::default()
        }),
    }]);

    let renderer_config = RendererConfig {
        texture_format: surface_config.format,
        ..Default::default()
    };

    let _imgui_renderer = imgui_wgpu::Renderer::new(&mut imgui, &device, &queue, renderer_config);

    let delta = Duration::from_millis(0);
    imgui.io_mut().update_delta_time(delta);
    platform
        .prepare_frame(imgui.io_mut(), &window)
        .expect("Failed to prepare frame");
    let ui = imgui.frame();

    // Time to draw
    println!("Press ENTER to continue...");
    let mut line = String::new();
    let _ = std::io::stdin().read_line(&mut line);

    ui.window("Map Info")
        .position([25.0, 25.0], Condition::FirstUseEver)
        .size([300.0, 400.0], Condition::FirstUseEver)
        .build(|| {
            ui.text(format!("Path: {}", "fake path"));
        });

    println!("Passed!");
}
