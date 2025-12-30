//! UI Module
//!
//! Provides immediate-mode UI using egui for controls and status display.

use egui::{Context, FontId, RichText, Color32, Ui, Slider, ProgressBar};
use egui_wgpu::Renderer as EguiRenderer;
use egui_winit::State as EguiState;
use winit::window::Window;

use crate::audio::AudioFeatures;
use crate::render::{Renderer, Scene, SceneMode};

/// UI state and rendering
pub struct UI {
    /// egui context
    context: Context,
    /// egui-winit state
    state: EguiState,
    /// egui-wgpu renderer
    renderer: EguiRenderer,
    /// Whether UI is visible
    pub visible: bool,
    /// Current FPS
    fps: f64,
    /// Frame time accumulator
    frame_time_acc: f64,
    /// Frame counter
    frame_count: u64,
}

impl UI {
    /// Create new UI system
    pub fn new(window: &Window, gpu_renderer: &Renderer) -> anyhow::Result<Self> {
        let context = Context::default();

        // Configure fonts
        let mut fonts = egui::FontDefinitions::default();
        // Use default fonts for now
        context.set_fonts(fonts);

        // Configure style
        let mut style = (*context.style()).clone();
        style.visuals.window_fill = Color32::from_rgba_unmultiplied(20, 20, 30, 220);
        style.visuals.panel_fill = Color32::from_rgba_unmultiplied(15, 15, 25, 240);
        context.set_style(style);

        // Create egui-winit state
        let state = EguiState::new(
            context.clone(),
            egui::ViewportId::ROOT,
            window,
            None,
            None,
            None,
        );

        // Create egui-wgpu renderer
        let renderer = EguiRenderer::new(
            gpu_renderer.device(),
            gpu_renderer.surface_format(),
            None,
            1,
            false,
        );

        Ok(Self {
            context,
            state,
            renderer,
            visible: true,
            fps: 0.0,
            frame_time_acc: 0.0,
            frame_count: 0,
        })
    }

    /// Handle window events
    pub fn handle_event(&mut self, event: &winit::event::WindowEvent) -> bool {
        let response = self.state.on_window_event(&self.context, event);
        response.consumed
    }

    /// Update UI state
    pub fn update(&mut self, delta: f64, _features: &AudioFeatures, _scene: &Scene) {
        // Update FPS counter
        self.frame_time_acc += delta;
        self.frame_count += 1;

        if self.frame_time_acc >= 1.0 {
            self.fps = self.frame_count as f64 / self.frame_time_acc;
            self.frame_time_acc = 0.0;
            self.frame_count = 0;
        }
    }

    /// Render UI
    pub fn render(
        &mut self,
        window: &Window,
        scene: &mut Scene,
        features: &AudioFeatures,
    ) -> Vec<egui::ClippedPrimitive> {
        if !self.visible {
            return Vec::new();
        }

        let raw_input = self.state.take_egui_input(window);

        let full_output = self.context.run(raw_input, |ctx| {
            self.draw_ui(ctx, scene, features);
        });

        self.state.handle_platform_output(window, full_output.platform_output);

        self.context.tessellate(full_output.shapes, full_output.pixels_per_point)
    }

    /// Draw UI elements
    fn draw_ui(&self, ctx: &Context, scene: &mut Scene, features: &AudioFeatures) {
        // Top status bar
        egui::TopBottomPanel::top("status_bar").show(ctx, |ui| {
            ui.horizontal(|ui| {
                ui.label(
                    RichText::new("◉ SYNESTHESIA")
                        .color(Color32::from_rgb(100, 200, 255))
                        .strong()
                );
                ui.separator();
                ui.label(format!("FPS: {:.0}", self.fps));
                ui.separator();
                ui.label(format!("Mode: {:?}", scene.mode));
                ui.separator();
                ui.label(format!("BPM: {:.0}", features.bpm));
            });
        });

        // Control panel
        egui::Window::new("Controls")
            .default_pos([10.0, 50.0])
            .default_width(280.0)
            .resizable(false)
            .show(ctx, |ui| {
                self.draw_controls(ui, scene, features);
            });

        // Audio visualization panel
        egui::Window::new("Audio Analysis")
            .default_pos([10.0, 300.0])
            .default_width(280.0)
            .resizable(false)
            .show(ctx, |ui| {
                self.draw_audio_panel(ui, features);
            });
    }

    /// Draw control panel contents
    fn draw_controls(&self, ui: &mut Ui, scene: &mut Scene, _features: &AudioFeatures) {
        ui.heading("Visualization Mode");
        ui.horizontal(|ui| {
            if ui.selectable_label(scene.mode == SceneMode::Abstract, "Abstract").clicked() {
                scene.mode = SceneMode::Abstract;
            }
            if ui.selectable_label(scene.mode == SceneMode::Narrative, "Narrative").clicked() {
                scene.mode = SceneMode::Narrative;
            }
        });

        ui.separator();
        ui.heading("Scene");

        ui.label(format!("Mood: {:?}", scene.semantic.mood.primary));
        ui.label(format!("Setting: {:?}", scene.semantic.setting.location));
        ui.label(format!("Time: {:?}", scene.semantic.setting.time_of_day));
        ui.label(format!("Weather: {:?}", scene.semantic.setting.weather));

        ui.separator();
        ui.heading("Colors");

        for (i, color) in scene.colors.iter().enumerate() {
            let label = match i {
                0 => "Primary",
                1 => "Secondary",
                _ => "Accent",
            };
            ui.horizontal(|ui| {
                let c = Color32::from_rgb(
                    (color.x * 255.0) as u8,
                    (color.y * 255.0) as u8,
                    (color.z * 255.0) as u8,
                );
                ui.colored_label(c, "■■■");
                ui.label(label);
            });
        }

        ui.separator();
        ui.label(
            RichText::new("Keyboard Shortcuts:")
                .small()
                .color(Color32::GRAY)
        );
        ui.label(RichText::new("Space - Play/Pause").small());
        ui.label(RichText::new("1 - Abstract Mode").small());
        ui.label(RichText::new("2 - Narrative Mode").small());
        ui.label(RichText::new("Esc - Stop").small());
    }

    /// Draw audio analysis panel
    fn draw_audio_panel(&self, ui: &mut Ui, features: &AudioFeatures) {
        ui.heading("Frequency Bands");

        ui.horizontal(|ui| {
            ui.label("Bass:");
            ui.add(ProgressBar::new(features.bass).show_percentage());
        });

        ui.horizontal(|ui| {
            ui.label("Mid:");
            ui.add(ProgressBar::new(features.mid).show_percentage());
        });

        ui.horizontal(|ui| {
            ui.label("High:");
            ui.add(ProgressBar::new(features.high).show_percentage());
        });

        ui.separator();
        ui.heading("Dynamics");

        ui.horizontal(|ui| {
            ui.label("RMS:");
            ui.add(ProgressBar::new(features.rms).show_percentage());
        });

        ui.horizontal(|ui| {
            ui.label("Beat:");
            let color = if features.is_beat {
                Color32::from_rgb(255, 100, 100)
            } else {
                Color32::GRAY
            };
            ui.colored_label(color, if features.is_beat { "●" } else { "○" });
            ui.add(ProgressBar::new(features.beat_intensity));
        });

        ui.separator();

        // Frequency band visualization
        ui.heading("Spectrum");
        let available_width = ui.available_width();
        let bar_width = available_width / features.frequency_bands.len() as f32;

        ui.horizontal(|ui| {
            for &band in &features.frequency_bands {
                let height = band * 40.0;
                let (rect, _) = ui.allocate_exact_size(
                    egui::vec2(bar_width - 1.0, 40.0),
                    egui::Sense::hover(),
                );

                let painter = ui.painter();
                let bar_rect = egui::Rect::from_min_size(
                    egui::pos2(rect.min.x, rect.max.y - height),
                    egui::vec2(bar_width - 2.0, height),
                );

                let color = Color32::from_rgb(
                    (100.0 + band * 155.0) as u8,
                    (200.0 - band * 100.0) as u8,
                    255,
                );
                painter.rect_filled(bar_rect, 0.0, color);
            }
        });
    }

    /// Get egui renderer for wgpu integration
    pub fn renderer(&self) -> &EguiRenderer {
        &self.renderer
    }

    /// Get mutable egui renderer
    pub fn renderer_mut(&mut self) -> &mut EguiRenderer {
        &mut self.renderer
    }

    /// Get egui context
    pub fn context(&self) -> &Context {
        &self.context
    }
}
