use eframe::{egui::{self, Color32}, epaint::pos2};
use image;
use egui_extras;
use std::{path::Path, fs::OpenOptions, io::Write};

#[derive(Debug, PartialEq)]
enum Points { 
    First, 
    Second, 
    Third 
}

pub struct TaskApp {
    first_win: bool,
    second_win: bool,
    input: String,
    output: String,
    texture: Option<egui::TextureHandle>,
    write_file: bool,
    output_file_path: String,
    option: Points,
}

impl Default for TaskApp {
    fn default() -> Self {
        Self {
            first_win: true,
            second_win: false,
            input: String::new(),
            output: String::new(),
            texture: None,
            write_file: false,
            output_file_path: String::new(),
            option: Points::First,
        }
    }
}

impl eframe::App for TaskApp {
    fn update(&mut self, ctx: &eframe::egui::Context, frame: &mut eframe::Frame) {
        let button_size = egui::vec2(ctx.available_rect().width() * 0.2, 4.0);
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.horizontal(|ui| {
                ui.style_mut().spacing.button_padding = button_size;
                if ui.button("first button").clicked() {
                    self.first_win = true;
                    self.second_win = false; 
                }
                
                if ui.button("second button").clicked() {
                    self.first_win = false;
                    self.second_win = true;
                }
            });
            if self.first_win {
                self.first_window(ctx, ui);
            }
            if self.second_win {
                self.second_window(ctx, ui);
            }
        });
    }
}

impl TaskApp {
    fn save_output(&mut self, ui: &mut egui::Ui){
        ui.add(egui::TextEdit::singleline(&mut self.output_file_path));
        if ui.button("Save name").clicked() && 
                        self.output_file_path.ends_with(".txt") {
            let mut output_file = OpenOptions::new()
                                                .write(true)
                                                .append(true)
                                                .create(true)
                                                .open(&self.output_file_path)
                                                .unwrap();
            output_file.write_all(self.output.as_bytes()).unwrap();
            self.output.clear();
            self.write_file = false;
        }
    }
    fn first_window(&mut self, ctx: &eframe::egui::Context, ui: &mut egui::Ui){
        // создаёт прокручиваемую панель для текста
        // (max_width устанавливает длину для виджетов)
        let scroll_area =
            egui::ScrollArea::new([true; 2]).max_width(ctx.available_rect().width() * 0.485);

        ui.horizontal(|ui| {
            ui.vertical(|ui| {
                ui.set_min_size(egui::vec2(
                    ctx.available_rect().width() * 0.50,
                    ctx.available_rect().height(),
                ));
                scroll_area.show(ui, |ui| {
                    ui.label(format!("{}", self.output));
                });
            });
            ui.vertical(|ui| {
                ui.add(egui::TextEdit::multiline(&mut self.input));
                if ui.button("Send").clicked() {
                    self.output.push_str(&self.input);
                    self.input.clear();
                }

                if ui.button("Clear").clicked() {
                    self.output.clear();
                }

                if ui.button("Save").clicked() {
                    self.write_file = true;
                }

                if self.write_file{
                    self.save_output(ui);
                }
                
                let texture = &mut self.texture;
                let texture: &egui::TextureHandle = texture.get_or_insert_with(|| {
                ui.ctx()
                        .load_texture("kotik:3",
                                load_image_from_path(Path::new(r"image\cat.jpg")).unwrap(),
                                Default::default())
                        });

                ui.add(egui::Image::new(texture,
                            egui::vec2(ctx.available_rect().width()*0.5, 300.0)));
            })
        });
    }

    fn second_window(&mut self, ctx: &eframe::egui::Context, ui: &mut egui::Ui){
        egui_extras::StripBuilder::new(ui)
        .size(egui_extras::Size::exact(100.0))
        .vertical(|mut strip| {
            strip.cell(|ui| {

            let radious_circle = 20.0;
            let rectangle_width = 100.0;
            let rectangle_height = 50.0;

            ui.painter().circle_filled(egui::pos2(100.0, 100.0), radious_circle, Color32::from_rgb(255, 0, 0));
            let rect = egui::Rect{
                min: pos2(200.0, 50.0),
                max: pos2(200.0 + rectangle_width, 50.0 + rectangle_height),
            };

            ui.painter().rect_filled(rect, 10.0, Color32::from_rgb(255, 0, 0));
            })
        });
        ui.horizontal(|ui| {
            ui.radio_value(&mut self.option, Points::First, "First");
            ui.radio_value(&mut self.option, Points::Second, "Second");
            ui.radio_value(&mut self.option, Points::Third, "Third");
            egui::ComboBox::from_label("")
            .selected_text(format!("{:?}", self.option))
            .show_ui(ui, |ui| {
                ui.selectable_value(&mut self.option, Points::First, "First");
                ui.selectable_value(&mut self.option, Points::Second, "Second");
                ui.selectable_value(&mut self.option, Points::Third, "Third");
            })
        });

    }    
}

fn load_image_from_path(path: &std::path::Path) -> Result<egui::ColorImage, image::ImageError> {
    let image = image::io::Reader::open(path)?.decode()?;
    let size = [image.width() as _, image.height() as _];
    let image_buffer = image.to_rgba8();
    let pixels = image_buffer.as_flat_samples();
    Ok(egui::ColorImage::from_rgba_unmultiplied(
        size,
        pixels.as_slice(),
    ))
}

