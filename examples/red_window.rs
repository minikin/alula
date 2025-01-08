use alula::framework::render::Render;
use std::sync::Arc;
use winit::{
    event::{Event, WindowEvent},
    event_loop::EventLoop,
    window::WindowBuilder,
};

async fn run() -> Result<(), Box<dyn std::error::Error>> {
    let event_loop = EventLoop::new().unwrap();
    let window = Arc::new(
        WindowBuilder::new()
            .with_title("Alula - Red Window")
            .with_inner_size(winit::dpi::LogicalSize::new(800.0, 600.0))
            .build(&event_loop)
            .unwrap(),
    );

    let window_ref = Arc::clone(&window);
    let mut render = Render::new(&window_ref)
        .await
        .expect("Failed to create renderer");

    event_loop.run(move |event, target| match event {
        Event::WindowEvent {
            window_id,
            event: WindowEvent::CloseRequested,
        } if window_id == window.id() => {
            target.exit();
        }
        Event::WindowEvent {
            window_id,
            event: WindowEvent::Resized(size),
        } if window_id == window.id() => {
            if let Err(e) = render.resize(size.width, size.height) {
                eprintln!("Error resizing: {:?}", e);
            }
        }
        Event::AboutToWait => {
            window.request_redraw();
        }
        Event::WindowEvent {
            window_id,
            event: WindowEvent::RedrawRequested,
        } if window_id == window.id() => {
            let red = wgpu::Color {
                r: 1.0,
                g: 0.0,
                b: 0.0,
                a: 1.0,
            };

            match render.render(red) {
                Ok(_) => {}
                Err(wgpu::SurfaceError::Lost) => {
                    let size = window.inner_size();
                    if let Err(e) = render.resize(size.width, size.height) {
                        eprintln!("Error recovering from surface lost: {:?}", e);
                    }
                }
                Err(wgpu::SurfaceError::OutOfMemory) => {
                    target.exit();
                }
                Err(e) => eprintln!("{:?}", e),
            }
        }
        _ => {}
    })?;

    Ok(())
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    env_logger::init();
    pollster::block_on(run())
}
