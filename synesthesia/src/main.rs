//! SYNESTHESIA - AI-Driven Immersive Music Visualization
//!
//! A breakthrough entertainment platform that transforms music into
//! meaningful visual experiences using AI understanding of lyrics,
//! emotional arcs, and semantic content.

mod app;
mod audio;
mod ai;
mod render;
mod ui;

#[cfg(feature = "omega")]
mod omega;

use anyhow::Result;
use log::info;
use winit::{
    event::{Event, WindowEvent},
    event_loop::{ControlFlow, EventLoop},
    window::WindowBuilder,
};

fn main() -> Result<()> {
    // Initialize logging
    env_logger::Builder::from_env(env_logger::Env::default().default_filter_or("info")).init();

    info!("Starting SYNESTHESIA v{}", env!("CARGO_PKG_VERSION"));
    info!("═══════════════════════════════════════════════════════════");
    info!("  AI-Driven Immersive Music Visualization Platform");
    info!("═══════════════════════════════════════════════════════════");

    // Parse command line arguments
    let args: Vec<String> = std::env::args().collect();
    let song_path = args.get(1).cloned();

    // Create window
    let event_loop = EventLoop::new()?;
    let window = WindowBuilder::new()
        .with_title("SYNESTHESIA")
        .with_inner_size(winit::dpi::LogicalSize::new(1920, 1080))
        .with_resizable(true)
        .build(&event_loop)?;

    // Initialize application
    let mut app = pollster::block_on(app::App::new(&window, song_path))?;

    info!("Application initialized. Ready for music.");

    // Main event loop
    event_loop.run(move |event, elwt| {
        elwt.set_control_flow(ControlFlow::Poll);

        match event {
            Event::WindowEvent { event, window_id } if window_id == window.id() => {
                // Let egui handle events first
                if app.handle_input(&event) {
                    return;
                }

                match event {
                    WindowEvent::CloseRequested => {
                        info!("Closing SYNESTHESIA...");
                        elwt.exit();
                    }
                    WindowEvent::Resized(new_size) => {
                        app.resize(new_size);
                    }
                    WindowEvent::RedrawRequested => {
                        app.update();
                        match app.render() {
                            Ok(_) => {}
                            Err(e) => {
                                log::error!("Render error: {:?}", e);
                            }
                        }
                    }
                    WindowEvent::KeyboardInput { event, .. } => {
                        app.handle_keyboard(&event);
                    }
                    _ => {}
                }
            }
            Event::AboutToWait => {
                window.request_redraw();
            }
            _ => {}
        }
    })?;

    Ok(())
}
