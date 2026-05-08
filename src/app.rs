use egui::{CentralPanel, ComboBox, MenuBar, Panel, widgets};
use egui_plot::{Legend, Line, Plot, PlotPoints, Points};
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
                    for algo in Algorithm::VARIANTS {
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

                    let slices = self.selected.eval(self.a, self.b, func);

                    let func = expr.clone().bind("x").unwrap();

                    ui.line(
                        Line::new("func", PlotPoints::from_explicit_callback(func, .., 100)).name(
                            format!("f = {:.17}", slices.iter().map(|e| e[1]).sum::<f64>()),
                        ),
                    );

                    let func = expr.clone().bind("x").unwrap();

                    for (i, &[x, a]) in slices.iter().enumerate() {
                        ui.points(
                            Points::new(format!("marker_{}", i), [x, func(x)])
                                .name(format!("p{i} = {a:.17}"))
                                .filled(true)
                                .radius(10.0),
                        );
                    }
                });
        });
    }
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
enum Algorithm {
    Trapezoidal,
    Simpson,
    NewtonCotes3,
    NewtonCotes4,
    NewtonCotes5,
    NewtonCotes6,
}

impl Algorithm {
    const VARIANTS: [Algorithm; 6] = [
        Algorithm::Trapezoidal,
        Algorithm::Simpson,
        Algorithm::NewtonCotes3,
        Algorithm::NewtonCotes4,
        Algorithm::NewtonCotes5,
        Algorithm::NewtonCotes6,
    ];

    fn text(&self) -> &'static str {
        match self {
            Algorithm::Trapezoidal => "Trapezoidal",
            Algorithm::Simpson => "Simpson",
            Algorithm::NewtonCotes3 => "Newton-Cotes n = 3",
            Algorithm::NewtonCotes4 => "Newton-Cotes n = 4",
            Algorithm::NewtonCotes5 => "Newton-Cotes n = 5",
            Algorithm::NewtonCotes6 => "Newton-Cotes n = 6",
        }
    }

    fn eval(&self, a: f64, b: f64, f: impl Fn(f64) -> f64) -> Vec<[f64; 2]> {
        let newton_cotes = |co: &[f64], all_co: f64| -> Vec<_> {
            let n = co.len() as f64 - 1.0;
            co.iter()
                .enumerate()
                .map(|(i, &co)| {
                    let i = i as f64;
                    let h = (b - a) / n;
                    [a + i * h, all_co * h * f(a + i * h) * co]
                })
                .collect()
        };

        match self {
            Algorithm::Trapezoidal => newton_cotes(&[1.0, 1.0], 0.5),
            Algorithm::Simpson => newton_cotes(&[1.0, 4.0, 1.0], 1.0 / 1.3),
            Algorithm::NewtonCotes3 => newton_cotes(&[1.0, 3.0, 3.0, 1.0], 3.0 / 8.0),
            Algorithm::NewtonCotes4 => newton_cotes(&[7.0, 32.0, 12.0, 32.0, 7.0], 2.0 / 45.0),
            Algorithm::NewtonCotes5 => {
                newton_cotes(&[19.0, 75.0, 50.0, 50.0, 75.0, 19.0], 5.0 / 288.0)
            }
            Algorithm::NewtonCotes6 => {
                newton_cotes(&[41.0, 216.0, 27.0, 272.0, 27.0, 216.0, 41.0], 1.0 / 140.0)
            }
        }
    }
}
