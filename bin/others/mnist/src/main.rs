#![cfg_attr(not(test), no_std, no_main)]

#[cfg(not(test))]
runtime::binInit!();
#[cfg(not(test))]
runtime::entry!(main);

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

    #[test]
    fn test_main() {
        main();
    }

    use eframe::egui;

    const WIDTH: usize = 28;
    const HEIGHT: usize = 28;

    #[test]
    fn mnist_get() -> Result<(), eframe::Error> {
        let options = eframe::NativeOptions {
            viewport: egui::ViewportBuilder::default()
                .with_inner_size([800.0, 600.0]) // A good default size.
                .with_min_inner_size([450.0, 350.0]) // A sensible minimum size.
                .with_resizable(true),
            ..Default::default()
        };

        // Run the egui application.
        eframe::run_native(
            "MNIST Drawer",
            options,
            Box::new(|_cc| Ok(Box::<MnistAPP>::default())),
        )
    }

    struct MnistAPP {
        pixels: Vec<u8>,
        texture: Option<egui::TextureHandle>,
        pixels_dirty: bool,
        brush_radius: f32,
        recognized_digit: usize,

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

                infer: Inference::new(),
            }
        }
    }

    impl eframe::App for MnistAPP {
        fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
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

                    if ui.button("Save (dummy)").clicked() {
                        println!("'Save' button clicked. Implement saving logic here.");
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

        fn recognize(&self) -> usize {
            self.infer.mnist_inference_pure_int8(&self.pixels)
        }
    }

    // #[test]
    // #[ignore]
    // fn mnist_gui() {
    //     use fltk::{
    //         app, button::Button, draw, prelude::*, text::TextDisplay, text::TextEditor,
    //         window::Window,
    //     };
    //     use std::cell::RefCell;
    //     use std::rc::Rc;

    //     const GRID_WIDTH: usize = 28;
    //     const GRID_HEIGHT: usize = 28;
    //     const PIXEL_SIZE: i32 = 20;
    //     const CANVAS_SIZE: i32 = PIXEL_SIZE as i32 * GRID_WIDTH as i32;

    //     let app = app::App::default();
    //     let mut wind = Window::default()
    //         .with_size(900, 650)
    //         .with_label("MNIST Digit Recognizer - Draw a Digit!");

    //     // Canvas area (left side)
    //     let mut canvas = fltk::frame::Frame::default()
    //         .with_size(CANVAS_SIZE, CANVAS_SIZE)
    //         .with_pos(10, 10);

    //     let mut result_box = TextDisplay::default()
    //         .with_size(280, 150)
    //         .with_pos(CANVAS_SIZE + 20, 10);
    //     result_box.set_buffer(fltk::text::TextBuffer::default());

    //     let mut recognize_btn = Button::default()
    //         .with_size(100, 40)
    //         .with_pos(CANVAS_SIZE + 20, 170)
    //         .with_label("Recognize");

    //     let mut clear_btn = Button::default()
    //         .with_size(100, 40)
    //         .with_pos(CANVAS_SIZE + 130, 170)
    //         .with_label("Clear");

    //     let mut save_btn = Button::default()
    //         .with_size(100, 40)
    //         .with_pos(CANVAS_SIZE + 20, 220)
    //         .with_label("Save Image");

    //     let mut label_input = TextEditor::default()
    //         .with_size(100, 30)
    //         .with_pos(CANVAS_SIZE + 20, 270);
    //     label_input.set_buffer(fltk::text::TextBuffer::default());
    //     label_input.buffer().unwrap().set_text("0");

    //     let mut instr = TextDisplay::default()
    //         .with_size(260, 150)
    //         .with_pos(CANVAS_SIZE + 20, 320);
    //     let mut instr_buf = fltk::text::TextBuffer::default();
    //     instr_buf.set_text(
    //         "Instructions:\n1. Draw digit on canvas\n2. Click Recognize\n3. Set label (0-9)\n4. Click Save Image",
    //     );
    //     instr.set_buffer(instr_buf);

    //     wind.end();
    //     wind.show();

    //     const FC1_WEIGHT_DATA: &[u8] = include_bytes!("../binarys/fc1_weight.bin");
    //     const FC2_WEIGHT_DATA: &[u8] = include_bytes!("../binarys/fc2_weight.bin");
    //     const FC3_WEIGHT_DATA: &[u8] = include_bytes!("../binarys/fc3_weight.bin");

    //     const FC1_PARSED: ([[i8; 784]; 256], f32) =
    //         parse_weight_binary_const::<256, 784>(FC1_WEIGHT_DATA);
    //     const FC2_PARSED: ([[i8; 256]; 128], f32) =
    //         parse_weight_binary_const::<128, 256>(FC2_WEIGHT_DATA);
    //     const FC3_PARSED: ([[i8; 128]; 10], f32) =
    //         parse_weight_binary_const::<10, 128>(FC3_WEIGHT_DATA);

    //     let (fc1_weights, fc1_scale) = FC1_PARSED;
    //     let (fc2_weights, fc2_scale) = FC2_PARSED;
    //     let (fc3_weights, fc3_scale) = FC3_PARSED;

    //     let fc1_scale_q16 = scale_to_q16!(fc1_scale);
    //     let fc2_scale_q16 = scale_to_q16!(fc2_scale);
    //     let fc3_scale_q16 = scale_to_q16!(fc3_scale);

    //     println!("MNIST model loaded!");

    //     let pixel_buffer = Rc::new(RefCell::new(vec![0u8; GRID_WIDTH * GRID_HEIGHT]));
    //     let drawing = Rc::new(RefCell::new(false));

    //     // Drawing callback
    //     let pb_draw = pixel_buffer.clone();
    //     canvas.draw({
    //         let pb = pb_draw.clone();
    //         move |_| {
    //             draw::set_draw_color(fltk::enums::Color::White);
    //             draw::draw_rectf(10, 10, CANVAS_SIZE, CANVAS_SIZE);

    //             let pixels = pb.borrow();
    //             for y in 0..GRID_HEIGHT {
    //                 for x in 0..GRID_WIDTH {
    //                     let pixel = pixels[y * GRID_WIDTH + x];
    //                     let gray = 255u32.saturating_sub(pixel as u32) as u8;
    //                     draw::set_draw_color(fltk::enums::Color::from_rgb(gray, gray, gray));
    //                     draw::draw_rectf(
    //                         10 + (x as i32 * PIXEL_SIZE),
    //                         10 + (y as i32 * PIXEL_SIZE),
    //                         PIXEL_SIZE,
    //                         PIXEL_SIZE,
    //                     );

    //                     draw::set_draw_color(fltk::enums::Color::Light1);
    //                     draw::draw_rect(
    //                         10 + (x as i32 * PIXEL_SIZE),
    //                         10 + (y as i32 * PIXEL_SIZE),
    //                         PIXEL_SIZE,
    //                         PIXEL_SIZE,
    //                     );
    //                 }
    //             }
    //         }
    //     });

    //     // Mouse event handling
    //     let pb_handle = pixel_buffer.clone();
    //     let dr_handle = drawing.clone();
    //     canvas.handle({
    //         let pb = pb_handle.clone();
    //         let dr = dr_handle.clone();
    //         move |w, ev| match ev {
    //             fltk::enums::Event::Push => {
    //                 *dr.borrow_mut() = true;
    //                 true
    //             }
    //             fltk::enums::Event::Released => {
    //                 *dr.borrow_mut() = false;
    //                 true
    //             }
    //             fltk::enums::Event::Drag => {
    //                 if *dr.borrow() {
    //                     let (mx, my) = app::event_coords();
    //                     let x = ((mx - 10) / PIXEL_SIZE) as usize;
    //                     let y = ((my - 10) / PIXEL_SIZE) as usize;

    //                     if x < GRID_WIDTH && y < GRID_HEIGHT {
    //                         let mut pixels = pb.borrow_mut();
    //                         // Draw with brush (3x3 area)
    //                         for dy in -1..=1 {
    //                             for dx in -1..=1 {
    //                                 let nx = (x as i32 + dx) as usize;
    //                                 let ny = (y as i32 + dy) as usize;
    //                                 if nx < GRID_WIDTH && ny < GRID_HEIGHT {
    //                                     let idx = ny * GRID_WIDTH + nx;
    //                                     let dist_sq = (dx * dx + dy * dy) as f32;
    //                                     let intensity =
    //                                         ((180.0 * (1.0 - dist_sq / 2.0)).max(0.0)) as u8;
    //                                     pixels[idx] = pixels[idx].saturating_add(intensity);
    //                                 }
    //                             }
    //                         }
    //                         drop(pixels);
    //                         w.redraw();
    //                     }
    //                 }
    //                 true
    //             }
    //             _ => false,
    //         }
    //     });

    //     let pb_rec = pixel_buffer.clone();
    //     recognize_btn.set_callback({
    //         let result_box = result_box.clone();
    //         move |_| {
    //             let pixels = pb_rec.borrow().clone();
    //             let prediction = mnist_inference_pure_int8(
    //                 &fc1_weights,
    //                 &fc2_weights,
    //                 &fc3_weights,
    //                 &pixels,
    //                 fc1_scale_q16,
    //                 fc2_scale_q16,
    //                 fc3_scale_q16,
    //             );

    //             let mut buf = result_box.buffer().unwrap();
    //             buf.set_text(&format!(
    //                 "Predicted Digit:\n\n    {}\n\nConfident!",
    //                 prediction
    //             ));
    //         }
    //     });

    //     let pb_clr = pixel_buffer.clone();
    //     clear_btn.set_callback({
    //         let mut canvas = canvas.clone();
    //         move |_| {
    //             let mut pixels = pb_clr.borrow_mut();
    //             for p in pixels.iter_mut() {
    //                 *p = 0;
    //             }
    //             drop(pixels);
    //             canvas.redraw();
    //         }
    //     });

    //     let pb_save = pixel_buffer.clone();
    //     save_btn.set_callback({
    //         let label_input = label_input.clone();
    //         move |_| {
    //             let label_text = label_input.buffer().unwrap().text();
    //             let label = label_text.trim().parse::<u8>().unwrap_or(0);

    //             let pixels = pb_save.borrow().clone();
    //             save_mnist_image(&pixels, label);

    //             let mut buf = label_input.buffer().unwrap();
    //             buf.set_text("Saved!");
    //         }
    //     });

    //     app.run().unwrap();
    // }

    // fn save_mnist_image(pixels: &[u8], label: u8) {
    //     use std::fs::{create_dir_all, File};
    //     use std::io::Write;
    //     use std::path::Path;

    //     let _ = create_dir_all("test_images");

    //     let mut index = 0;
    //     loop {
    //         let bin_path = format!("test_images/saved_image_{:05}.bin", index);
    //         let txt_path = format!("test_images/saved_image_{:05}.txt", index);

    //         if !Path::new(&bin_path).exists() && !Path::new(&txt_path).exists() {
    //             let mut bin_data = vec![0u8; pixels.len() + 9];

    //             bin_data[0..4].copy_from_slice(&(28u32).to_le_bytes());
    //             bin_data[4..8].copy_from_slice(&(28u32).to_le_bytes());
    //             bin_data[8] = label;
    //             bin_data[9..].copy_from_slice(pixels);

    //             if let Ok(mut file) = File::create(&bin_path) {
    //                 let _ = file.write_all(&bin_data);
    //                 println!("Saved binary: {}", bin_path);
    //             }

    //             let mut txt_content = format!(
    //                 "Image Index: {}\nTrue Label: {}\nImage Data (28x28):\n",
    //                 index, label
    //             );

    //             for y in 0..28 {
    //                 for x in 0..28 {
    //                     let pixel = pixels[y * 28 + x];
    //                     txt_content.push_str(&format!("{:3} ", pixel));
    //                 }
    //                 txt_content.push('\n');
    //             }

    //             if let Ok(mut file) = File::create(&txt_path) {
    //                 let _ = file.write_all(txt_content.as_bytes());
    //                 println!("Saved text: {}", txt_path);
    //             }

    //             break;
    //         }
    //         index += 1;
    //     }
    // }
}
