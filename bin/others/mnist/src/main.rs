#![cfg_attr(not(test), no_std, no_main)]

#[cfg(not(test))]
runtime::binInit!();
#[cfg(test)]
runtime::addtest!();

macros::mod_flat!(inference);

// Command line arguments simulation (for benchmark mode)
// In a real embedded system, this would come from boot parameters or configuration
static BENCHMARK_MODE: bool = false; // Set to true for benchmark-only mode

fn main() {
    let infer = Inference::new();

    // Run benchmarks based on mode
    if BENCHMARK_MODE {
        // Benchmark-only mode - skip accuracy testing
        println!("=== BENCHMARK-ONLY MODE ===");

        infer.detailed_performance_analysis();
        println!();

        // Run full inference benchmark
        infer.run_benchmark();

        return; // Exit after benchmarks
    } else {
        // Normal mode - run quick benchmark then accuracy test
        println!("=== QUICK BENCHMARK ===");
        infer.run_benchmark();
        println!();
    }

    infer.test();
}

#[cfg(test)]
mod tests {
    use super::*;

    use eframe::egui;

    const WIDTH: usize = 28;
    const HEIGHT: usize = 28;

    #[test]
    fn mnist_get() -> Result<(), eframe::Error> {
        let mut options = eframe::NativeOptions::default();
        #[cfg(target_os = "windows")]
        {
            use winit::platform::windows::EventLoopBuilderExtWindows;
            options.event_loop_builder = Some(Box::new(|builder| {
                builder.with_any_thread(true);
            }));
        }
        #[cfg(target_os = "linux")]
        {
            use winit::platform::wayland::EventLoopBuilderExtWayland;
            options.event_loop_builder = Some(Box::new(|builder| {
                builder.with_any_thread(true);
            }));
        }

        eframe::run_native(
            "Test App",
            options,
            Box::new(|_cc| {
                let app = MnistAPP::default();
                Ok(Box::new(app))
            }),
        )
    }

    struct MnistAPP {
        pixels: Vec<u8>,
        texture: Option<egui::TextureHandle>,
        pixels_dirty: bool,
        brush_radius: f32,
        recognized_digit: u8,

        exit_after_frame: Option<u32>,
        frame_count: u32,

        infer: Inference,
    }

    impl Default for MnistAPP {
        fn default() -> Self {
            Self {
                pixels: vec![0; WIDTH * HEIGHT],
                texture: None,
                pixels_dirty: true,
                brush_radius: 1.5,
                recognized_digit: 0,

                exit_after_frame: None,
                frame_count: 0,

                infer: Inference::new(),
            }
        }
    }

    impl eframe::App for MnistAPP {
        fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
            if let Some(exit_after_frame) = self.exit_after_frame {
                if self.frame_count >= exit_after_frame {
                    ctx.send_viewport_cmd(egui::ViewportCommand::Close);
                }
            }
            self.frame_count += 1;

            self.ui_side_panel(ctx);
            self.ui_central_panel(ctx);
        }
    }

    impl MnistAPP {
        fn ui_side_panel(&mut self, ctx: &egui::Context) {
            egui::SidePanel::right("side_panel")
                .min_width(150.0)
                .show(ctx, |ui| {
                    ui.heading("Controls");
                    ui.separator();

                    if ui.button("Clear Canvas").clicked() {
                        self.clear_canvas();
                    }

                    if ui.button("Save").clicked() {
                        self.save();
                    }

                    ui.separator();
                    ui.add(
                        egui::Slider::new(&mut self.brush_radius, 0.5..=5.0).text("Brush Radius"),
                    );
                    ui.separator();

                    ui.heading("Recognition");
                    let mut digit_str = self.recognized_digit.to_string();
                    ui.label("Recognized as:");
                    ui.add(egui::TextEdit::singleline(&mut digit_str).interactive(false));
                });
        }

        fn ui_central_panel(&mut self, ctx: &egui::Context) {
            egui::CentralPanel::default().show(ctx, |ui| {
                ui.heading("MNIST Digit Drawer");
                ui.label("Draw a single digit (0-9) in the black box.");

                self.ui_canvas(ui);
            });
        }

        fn ui_canvas(&mut self, ui: &mut egui::Ui) {
            let canvas_size = egui::Vec2::splat(ui.available_size().min_elem());
            let (response, painter) = ui.allocate_painter(canvas_size, egui::Sense::drag());

            painter.rect_filled(response.rect, 0.0, egui::Color32::BLACK);

            let texture = self.get_texture(ui);
            painter.image(
                texture.id(),
                response.rect,
                egui::Rect::from_min_max(egui::pos2(0.0, 0.0), egui::pos2(1.0, 1.0)),
                egui::Color32::WHITE,
            );

            self.handle_canvas_interaction(&response);
        }

        fn get_texture(&mut self, ui: &mut egui::Ui) -> &egui::TextureHandle {
            let texture_options = egui::TextureOptions::NEAREST;

            if self.texture.is_none() {
                let image = egui::ColorImage::from_gray([WIDTH, HEIGHT], &self.pixels);
                self.texture = Some(ui.ctx().load_texture(
                    "canvas-texture",
                    image,
                    texture_options,
                ));
            }

            if self.pixels_dirty {
                if let Some(texture) = &mut self.texture {
                    let image = egui::ColorImage::from_gray([WIDTH, HEIGHT], &self.pixels);
                    texture.set(image, texture_options);
                    self.pixels_dirty = false;
                }
            }

            self.texture.as_ref().unwrap()
        }

        fn handle_canvas_interaction(&mut self, response: &egui::Response) {
            if response.dragged() {
                if let Some(pos) = response.interact_pointer_pos() {
                    self.paint_stroke(pos, response.rect);
                }
            }

            if response.drag_stopped() {
                self.recognized_digit = self.recognize();
            }
        }

        fn paint_stroke(&mut self, pointer_pos: egui::Pos2, canvas_rect: egui::Rect) {
            let rect_pos = pointer_pos - canvas_rect.min;
            let grid_x = rect_pos.x * (WIDTH as f32) / canvas_rect.width();
            let grid_y = rect_pos.y * (HEIGHT as f32) / canvas_rect.height();
            let radius_sq = self.brush_radius * self.brush_radius;

            for y in 0..HEIGHT {
                for x in 0..WIDTH {
                    let dx = x as f32 - grid_x;
                    let dy = y as f32 - grid_y;
                    let dist_sq = dx * dx + dy * dy;

                    if dist_sq < radius_sq {
                        let intensity_factor = 1.0 - (dist_sq / radius_sq);
                        let intensity_to_add = (255.0 * intensity_factor) as u16;

                        let idx = y * WIDTH + x;
                        let current_val = self.pixels[idx] as u16;
                        let new_val = (current_val + intensity_to_add).min(255) as u8;
                        if self.pixels[idx] != new_val {
                            self.pixels[idx] = new_val;
                            self.pixels_dirty = true;
                        }
                    }
                }
            }
        }

        fn clear_canvas(&mut self) {
            self.pixels.fill(0);
            self.pixels_dirty = true;
            self.recognized_digit = self.recognize();
        }

        fn recognize(&self) -> u8 {
            let image = &self.pixels;

            if image.iter().all(|pixel| *pixel == 0) {
                0
            } else {
                self.infer.mnist_inference_pure_int8(&self.pixels) as u8
            }
        }

        fn save(&self) {
            use std::fs::{create_dir_all, File};
            use std::io::Write;
            use std::path::Path;

            let _ = create_dir_all("test_images");

            let mut index = 0;
            loop {
                let bin_path = format!("test_images/saved_image_{:05}.bin", index);
                let txt_path = format!("test_images/saved_image_{:05}.txt", index);

                if !Path::new(&bin_path).exists() && !Path::new(&txt_path).exists() {
                    let mut bin_data = vec![0u8; self.pixels.len() + 9];

                    bin_data[0..4].copy_from_slice(&(28u32).to_le_bytes());
                    bin_data[4..8].copy_from_slice(&(28u32).to_le_bytes());
                    bin_data[8] = self.recognized_digit;
                    bin_data[9..].copy_from_slice(&self.pixels);

                    if let Ok(mut file) = File::create(&bin_path) {
                        let _ = file.write_all(&bin_data);
                        println!("Saved binary: {}", bin_path);
                    }

                    let mut txt_content = format!(
                        "Image Index: {}\nTrue Label: {}\nImage Data (28x28):\n",
                        index, self.recognized_digit
                    );

                    for y in 0..28 {
                        for x in 0..28 {
                            let pixel = self.pixels[y * 28 + x];
                            txt_content.push_str(&format!("{:3} ", pixel));
                        }
                        txt_content.push('\n');
                    }

                    if let Ok(mut file) = File::create(&txt_path) {
                        let _ = file.write_all(txt_content.as_bytes());
                        println!("Saved text: {}", txt_path);
                    }

                    break;
                }
                index += 1;
            }
        }
    }
}
