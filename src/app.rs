use egui::{CentralPanel, ComboBox, MenuBar, Panel, widgets};
use egui_plot::{Legend, Line, Plot, PlotPoints, Points, Span};
use meval::Expr;
use serde::{Deserialize, Serialize};

/// We derive Deserialize/Serialize so we can persist app state on shutdown.
#[derive(Deserialize, Serialize)]
#[serde(default)] // if we add new fields, give them default values when deserializing old state
pub struct App {
    expr: String,
    a: f64,
    b: f64,
    selected: Algorithm,
}

impl Default for App {
    fn default() -> Self {
        Self {
            expr: "x^2".to_owned(),
            a: -1.0,
            b: 1.0,
            selected: Algorithm::Trapezoidal,
        }
    }
}

impl App {
    /// Called once before the first frame.
    pub fn new(cc: &eframe::CreationContext<'_>) -> Self {
        // This is also where you can customize the look and feel of egui using
        // `cc.egui_ctx.set_visuals` and `cc.egui_ctx.set_fonts`.

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

        Panel::top("top_panel").show_inside(ui, |ui| {
            MenuBar::new().ui(ui, |ui| {
                widgets::global_theme_preference_buttons(ui);
            });
        });

        Panel::left("left_panel").show_inside(ui, |ui| {
            ui.heading("Interactive Quadrature");
            ui.spacing();

            ui.horizontal(|ui| {
                ui.label("Expr: ");
                ui.text_edit_singleline(&mut self.expr);
            });

            ui.horizontal(|ui| {
                ui.label("a: ");
                ui.add(egui::DragValue::new(&mut self.a).speed(0.1));
                ui.label("b: ");
                ui.add(egui::DragValue::new(&mut self.b).speed(0.1));
            });

            ComboBox::from_label("Algorithm")
                .selected_text(self.selected.text())
                .show_ui(ui, |ui| {
                    use Algorithm::*;
                    for algo in [Trapezoidal, Simpson] {
                        let text = algo.text();
                        ui.selectable_value(&mut self.selected, algo, text);
                    }
                });
        });

        CentralPanel::default().show_inside(ui, |ui| {
            Plot::new("central_plot")
                .legend(Legend::default())
                .data_aspect(1.0)
                .show(ui, |ui| {
                    let Ok(expr) = self.expr.parse::<Expr>() else {
                        return;
                    };

                    let Ok(func) = expr.clone().bind("x") else {
                        return;
                    };

                    let slices =
                        self.selected
                            .eval(self.a, self.b, expr.clone().bind("x").unwrap());

                    for (i, slice) in slices.iter().enumerate() {
                        // ui.line(
                        //     Line::new(
                        //         format!("{i}",),
                        //         PlotPoints::new(vec![
                        //             [slice.0, 1.0],
                        //             [slice.0, 0.0],
                        //             [slice.0, -1.0],
                        //         ]),
                        //     )
                        //     .name(format!("{:.4}", slice.1)),
                        // );

                        let start = slice.0;
                        let end = slices.get(i + 1).map(|e| e.0).unwrap_or(self.b);

                        ui.span(Span::new(format!("{:.4}", slice.1), start..=end));
                    }

                    ui.line(
                        Line::new("func", PlotPoints::from_explicit_callback(func, .., 100))
                            .name(format!("{:.4}", slices.iter().map(|e| e.1).sum::<f64>())),
                    );
                });
        });
    }
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
enum Algorithm {
    Trapezoidal,
    Simpson,
}

impl Algorithm {
    fn text(&self) -> &'static str {
        match self {
            Algorithm::Trapezoidal => "Trapezoidal",
            Algorithm::Simpson => "Simpson",
        }
    }

    fn eval(&self, a: f64, b: f64, f: impl Fn(f64) -> f64) -> Vec<(f64, f64)> {
        let woof = |co: &[f64], all_co: f64| -> Vec<_> {
            let n = co.len() as f64;
            co.iter()
                .enumerate()
                .map(|(i, &co)| {
                    let i = i as f64;
                    let h = (b - a) / n;
                    (a + i * h, all_co * h * f(a + i * h) * co)
                })
                .collect()
        };

        match self {
            Algorithm::Trapezoidal => woof(&[1.0, 1.0], 0.5),

            Algorithm::Simpson => woof(&[1.0, 4.0, 1.0], 1.0 / 1.3),
        }
    }
}
