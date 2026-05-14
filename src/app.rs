use egui::{CentralPanel, ComboBox, Panel, Visuals};
use egui_plot::{Legend, Line, Plot, PlotPoints, Points};
use formulac::Builder;
use num_complex::Complex;
use serde::{Deserialize, Serialize};

use crate::{Algorithm, Endpoints};

/// We derive Deserialize/Serialize so we can persist app state on shutdown.
#[derive(Deserialize, Serialize)]
// #[serde(default)] // if we add new fields, give them default values when deserializing old state
pub struct App {
    pub expr: String,
    pub endpoints: Endpoints,
    pub selected_algo: Algorithm,

    pub closed_newton_cotes_n: usize,
    pub open_newton_cotes_n: usize,
    pub gauss_legendre_n: usize,
}

impl Default for App {
    fn default() -> Self {
        Self {
            expr: "x^2".to_owned(),
            endpoints: Endpoints {
                start: -1.0,
                end: 1.0,
            },
            selected_algo: Algorithm::ClosedNewtonCotes,

            closed_newton_cotes_n: *Self::CLOSED_NEWTON_COTES_N_RANGE.start(),
            open_newton_cotes_n: *Self::OPEN_NEWTON_COTES_N_RANGE.start(),
            gauss_legendre_n: 2,
        }
    }
}

impl App {
    /// Called once before the first frame.
    pub fn new(cc: &eframe::CreationContext<'_>) -> Self {
        // This is also where you can customize the look and feel of egui using
        // `cc.egui_ctx.set_visuals` and `cc.egui_ctx.set_fonts`.

        cc.egui_ctx.set_visuals(Visuals::dark());

        // Load previous app state (if any).
        // Note that you must enable the `persistence` feature for this to work.
        if let Some(storage) = cc.storage {
            eframe::get_value(storage, eframe::APP_KEY).unwrap_or_default()
        } else {
            Default::default()
        }
    }
}

impl eframe::App for App {
    /// Called by the framework to save state before shutdown.
    fn save(&mut self, storage: &mut dyn eframe::Storage) {
        eframe::set_value(storage, eframe::APP_KEY, self);
    }

    /// Called each time the UI needs repainting, which may be many times per second.
    fn ui(&mut self, ui: &mut egui::Ui, _frame: &mut eframe::Frame) {
        // Put your widgets into a `SidePanel`, `TopBottomPanel`, `CentralPanel`, `Window` or `Area`.
        // For inspiration and more examples, go to https://emilk.github.io/egui

        Panel::left("left_panel").show_inside(ui, |ui| {
            ui.heading("Interactive Quadrature");
            ui.spacing();

            ui.horizontal(|ui| {
                ui.label("Expr: ");
                ui.text_edit_singleline(&mut self.expr);
            });

            ui.horizontal(|ui| {
                ui.label("Start: ");
                ui.add(egui::DragValue::new(&mut self.endpoints.start).speed(0.1));
                ui.label("End: ");
                ui.add(egui::DragValue::new(&mut self.endpoints.end).speed(0.1));
            });

            ComboBox::from_label("Algorithm")
                .selected_text(self.selected_algo.text())
                .show_ui(ui, |ui| {
                    for &algo in Algorithm::VARIANTS {
                        ui.selectable_value(&mut self.selected_algo, algo, algo.text());
                    }
                });

            ui.horizontal(|ui| {
                ui.label("n: ");
                match self.selected_algo {
                    Algorithm::ClosedNewtonCotes => ui.add(egui::Slider::new(
                        &mut self.closed_newton_cotes_n,
                        Self::CLOSED_NEWTON_COTES_N_RANGE,
                    )),
                    Algorithm::OpenNewtonCotes => ui.add(egui::Slider::new(
                        &mut self.open_newton_cotes_n,
                        Self::OPEN_NEWTON_COTES_N_RANGE,
                    )),
                    Algorithm::GaussLegendre | Algorithm::GaussChebyshev => ui.add(
                        egui::Slider::new(&mut self.gauss_legendre_n, Self::GAUSS_N_RANGE),
                    ),
                }
            });
        });

        CentralPanel::default().show_inside(ui, |ui| {
            Plot::new("central_plot")
                .legend(Legend::default().text_style(egui::TextStyle::Monospace))
                .data_aspect(1.0)
                .show(ui, |ui| {
                    let Ok(slices) = self.eval() else {
                        return;
                    };

                    let sum = slices.iter().map(|point| point.area).sum::<f64>();

                    ui.line(
                        Line::new(
                            "func",
                            PlotPoints::from_explicit_callback(
                                {
                                    let func = Builder::<f64, 1>::new(&self.expr, ["x"])
                                        .compile()
                                        .expect("expr passed through eval earlier");
                                    move |x| func([Complex::new(x, 0.0)]).re
                                },
                                self.endpoints.start..=self.endpoints.end,
                                100,
                            ),
                        )
                        .name(format!("f = {:+.17}", sum))
                        .fill(0.0)
                        .fill_alpha(0.25),
                    );

                    for (i, point) in slices.iter().enumerate() {
                        ui.points(
                            Points::new(format!("marker_{}", i), [point.x, point.y])
                                .name(format!("p{i:02} = {:+.17}", point.area))
                                .radius(10.0),
                        );
                    }
                });
        });
    }
}
