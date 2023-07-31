use eframe::egui;
use image;
use std::path::Path;

pub struct TaskApp {
    input: String,
    output: String,
    texture: Option<egui::TextureHandle>
}

impl Default for TaskApp {
    fn default() -> Self {
        Self {
            input: String::new(),
            output: String::new(),
            texture: None,
        }
    }
}

impl eframe::App for TaskApp {
    fn update(&mut self, ctx: &eframe::egui::Context, frame: &mut eframe::Frame) {


        let button_size = egui::vec2(ctx.available_rect().width() * 0.2, 4.0);
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.horizontal(|ui| {
                ui.style_mut().spacing.button_padding = button_size;
                if ui.button("first button").clicked() {}
                if ui.button("second button").clicked() {}
            });
            // создаёт прокручиваемую панель для текста
            // (max_width устанавливает длину для виджетов)
            let scroll_area =
                egui::ScrollArea::new([true; 2]).max_width(ctx.available_rect().width() * 0.485);

            ui.horizontal(|ui| {
                ui.vertical(|ui| {
                    ui.set_min_size(egui::vec2(
                        ctx.available_rect().width() * 0.48,
                        ctx.available_rect().height(),
                    ));
                    scroll_area.show(ui, |ui| {
                        ui.label(format!("{}", self.output));
                    });
                });
                ui.vertical(|ui| {
                    ui.add(egui::TextEdit::multiline(&mut self.input));
                    if ui.button("Send").clicked() {}
                    if ui.button("Clear").clicked() {}
                    if ui.button("Save").clicked() {}
                    let texture: &egui::TextureHandle = 
                        // Load the texture only once.
                        &ui.ctx().load_texture(
                            "../image/cat.jpg",
                            egui::ColorImage::example(),
                            Default::default(),
                        );
                    let texture = &mut self.texture;
                    let texture: &egui::TextureHandle = texture.get_or_insert_with(|| {
                    ui.ctx()
                            .load_texture("example",
                                          load_image_from_path(Path::new("/home/nemoved/Proga/rust/task_egui/image/cat.jpg")).unwrap(),
                                          Default::default())
                            });

                    ui.add(egui::Image::new(texture, egui::vec2(ctx.available_rect().width()*0.5, 300.0)));
                })
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
