use eframe::egui;
use eframe::run_native;
use eframe::NativeOptions;

fn main() -> Result<(), eframe::Error> {
    let options = NativeOptions {
        ..Default::default()
    };

    run_native(
        "rust_egui",
        options,
        Box::new(|_cc| Box::<MyApp>::default()),
    )
}

#[derive(Debug, PartialEq)]
enum Panel {
    Read,
    Create,
}

impl Panel {
    fn ui(&self, ui: &mut egui::Ui) -> egui::Response {
        match self {
            Self::Read => ui.label("Read"),
            Self::Create => ui.label("Create"),
        }
    }
}

impl Default for Panel {
    fn default() -> Self {
        Self::Read
    }
}

struct MyApp {
    panel: Panel,
}

impl Default for MyApp {
    fn default() -> Self {
        Self {
            panel: Panel::default(),
        }
    }
}

impl eframe::App for MyApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        // TopBottomPanel::top("top_panel")
        //     .resizable(false)
        //     .min_height(0.0)
        //     .show(ctx, |ui| {
        //         ui.horizontal(|ui| {
        //             ui.with_layout(Layout::left_to_right(Align::Center), |ui| {
        //                 ui.label("Users CRUD");
        //             });
        //
        //             ui.with_layout(Layout::right_to_left(Align::Center), |ui| {
        //                 eframe::egui::widgets::global_dark_light_mode_switch(ui);
        //             });
        //         });
        //     });

        egui::SidePanel::left("left_panel")
            .resizable(true)
            .default_width(100.0)
            .width_range(50.0..=200.0)
            .frame(egui::Frame {
                inner_margin: egui::Margin {
                    left: 5.0,
                    right: 7.0,
                    top: 5.0,
                    bottom: 5.0,
                },
                fill: ctx.style().noninteractive().bg_fill,
                ..Default::default()
            })
            .show(ctx, |ui| {
                ui.vertical_centered_justified(|ui| {
                    ui.selectable_value(&mut self.panel, Panel::Read, "Read");
                    ui.selectable_value(&mut self.panel, Panel::Create, "Create");
                });
                ui.with_layout(egui::Layout::bottom_up(egui::Align::LEFT), |ui| {
                    eframe::egui::widgets::global_dark_light_mode_switch(ui);
                });
            });

        egui::CentralPanel::default().show(ctx, |ui| {
            self.panel.ui(ui);
        });
    }
}
