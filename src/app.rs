use eframe::{
    egui::{
        self,
        plot::{self},
        Color32, ScrollArea,
    },
    epaint::pos2,
};
use egui_extras;
use image;
use std::{fs::OpenOptions, io::Write, path::Path};

#[derive(Debug, PartialEq)]
enum Points {
    First,
    Second,
    Third,
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
    fn update(&mut self, ctx: &eframe::egui::Context, _frame: &mut eframe::Frame) {
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
    fn first_window(&mut self, ctx: &eframe::egui::Context, ui: &mut egui::Ui) {
        let scroll_area = egui::ScrollArea::new([true; 2])
            .max_width(ctx.available_rect().width() * 0.485)
            .max_height(ctx.available_rect().height() * 0.9);

        ui.horizontal(|ui| {
            ui.vertical(|ui| {
                //set_min_size needed to make fixed size for
                //scroll area to to ensure that the widgets
                //on the left side are always a certain length
                //from the beginning of the screen
                ui.set_min_size(egui::vec2(
                    ctx.available_rect().width() * 0.485,
                    ctx.available_rect().height(),
                ));
                scroll_area.show(ui, |ui| {
                    ui.label(&self.output);
                });
            });
            ui.vertical(|ui| {
                ui.add(
                    egui::TextEdit::multiline(&mut self.input)
                        .min_size(egui::vec2(ctx.available_rect().width() * 0.48, 100.0)),
                );

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

                if self.write_file {
                    self.save_output(ui);
                }

                let texture = &mut self.texture;
                //load image texture with default settings
                let texture: &egui::TextureHandle = texture.get_or_insert_with(|| {
                    ui.ctx().load_texture(
                        "kotik:3",
                        load_image_from_path(Path::new("image/cat.jpg")).unwrap(),
                        Default::default(),
                    )
                });

                //add Image from TextureHandle
                ui.add(egui::Image::new(
                    texture,
                    egui::vec2(ctx.available_rect().width() * 0.5, 400.0),
                ));
            })
        });
    }

    fn save_output(&mut self, ui: &mut egui::Ui) {
        ui.add(egui::TextEdit::singleline(&mut self.output_file_path));
        if ui.button("Save name").clicked() {
            if !self.output_file_path.ends_with(".txt") {
                self.output_file_path.push_str(".txt");
            }
            //open file for output text
            //if text with name from let output_file_path
            //is exist we write text to the end of this file
            //else we create them
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

    fn second_window(&mut self, ctx: &eframe::egui::Context, ui: &mut egui::Ui) {
        //egui_extras is a addon to egui
        //StripBuilder need to make cells with
        //fixed size, more info: https://docs.rs/egui_extras/latest/egui_extras/struct.StripBuilder.html
        egui_extras::StripBuilder::new(ui)
            .size(egui_extras::Size::exact(100.0))
            .vertical(|mut strip| {
                strip.cell(|ui| {
                    let radious_circle = 35.0;
                    let rectangle_width = 100.0;
                    let rectangle_height = 50.0;

                    ui.painter().circle_filled(
                        egui::pos2(100.0, 75.0),
                        radious_circle,
                        Color32::from_rgb(255, 0, 0),
                    );

                    let rect = egui::Rect {
                        min: pos2(200.0, 50.0),
                        max: pos2(200.0 + rectangle_width, 50.0 + rectangle_height),
                    };

                    ui.painter()
                        .rect_filled(rect, 10.0, Color32::from_rgb(255, 0, 0));
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

        let sin: plot::PlotPoints = (-1000..1000)
            .map(|i| {
                let x = i as f64 * 0.01;
                [x, x.sin()]
            })
            .collect();

        let line = plot::Line::new(sin);
        ui.set_min_size(egui::vec2(400.0, 300.0));
        plot::Plot::new("")
            .width(ctx.available_rect().width())
            .height(200.0)
            .view_aspect(2.0) // width/height ratio
            .show(ui, |plot_ui| plot_ui.line(line));

        ScrollArea::new([true; 2])
            .max_width(ctx.available_rect().width() * 0.98)
            .max_height(300.0)
            .show(ui, |ui| {
                if !self.output.is_empty() {
                    ui.label("Hey, im here too >///<");
                    ui.label(&self.output);
                }
            });
    }
}

fn load_image_from_path(path: &std::path::Path) -> Result<egui::ColorImage, image::ImageError> {
    //open image from path and decode them
    let image = image::io::Reader::open(path)?.decode()?;
    //get width and height of image
    let size = [image.width() as _, image.height() as _];
    //transform image into image::RgbaImage
    let image_buffer = image.to_rgba8();
    //get the image pixels as a flat slice of samples
    let pixels = image_buffer.as_flat_samples();
    //create a new egui::ColorImage from the rgba8 deta
    Ok(egui::ColorImage::from_rgba_unmultiplied(
        size,
        pixels.as_slice(),
    ))
}
