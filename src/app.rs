use egui::{CentralPanel, ComboBox, MenuBar, Panel, widgets};
use egui_plot::{Legend, Line, Plot, PlotPoints, Points};
use meval::Expr;
use serde::{Deserialize, Serialize};

/// We derive Deserialize/Serialize so we can persist app state on shutdown.
#[derive(Deserialize, Serialize)]
#[serde(default)] // if we add new fields, give them default values when deserializing old state
pub struct App {
    expr: String,
    endpoints: Endpoints,
    selected_algo: Algorithm,
    newton_cotes_n: usize,
}

impl Default for App {
    fn default() -> Self {
        Self {
            expr: "x^2".to_owned(),
            endpoints: Endpoints {
                start: -1.0,
                end: 1.0,
            },
            selected_algo: Algorithm::NewtonCotes,
            newton_cotes_n: 1,
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

            if self.selected_algo == Algorithm::NewtonCotes {
                ui.horizontal(|ui| {
                    ui.label("n: ");
                    ui.add(egui::Slider::new(
                        &mut self.newton_cotes_n,
                        1..=Algorithm::NEWTON_COTES_COEFFICIENTS.len(),
                    ));
                });
            }
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

                    let slices = self.selected_algo.eval(self, func);
                    let func = expr.clone().bind("x").unwrap();
                    let sum = slices.iter().map(|point| point.area).sum::<f64>();

                    ui.line(
                        Line::new("func", PlotPoints::from_explicit_callback(func, .., 100))
                            .name(format!("f = {:.17}", sum)),
                    );

                    for (i, point) in slices.iter().enumerate() {
                        ui.points(
                            Points::new(format!("marker_{}", i), [point.x, point.y])
                                .name(format!("p{i} = {:.17}", point.area))
                                .filled(true)
                                .radius(10.0),
                        );
                    }
                });
        });
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
struct Endpoints {
    start: f64,
    end: f64,
}

#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
struct InterpolationPoint {
    x: f64,
    y: f64,
    area: f64,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
enum Algorithm {
    NewtonCotes,
}

impl Algorithm {
    const VARIANTS: &[Algorithm] = &[Algorithm::NewtonCotes];

    fn text(&self) -> &'static str {
        match self {
            Algorithm::NewtonCotes => "Newton-Cotes",
        }
    }

    const NEWTON_COTES_COEFFICIENTS: &[(f64, &[f64])] = &[
        (1.0 / 2.0, &[1.0, 1.0]),
        (1.0 / 1.3, &[1.0, 4.0, 1.0]),
        (3.0 / 8.0, &[1.0, 3.0, 3.0, 1.0]),
        (2.0 / 45.0, &[7.0, 32.0, 12.0, 32.0, 7.0]),
        (5.0 / 288.0, &[19.0, 75.0, 50.0, 50.0, 75.0, 19.0]),
        (1.0 / 140.0, &[41.0, 216.0, 27.0, 272.0, 27.0, 216.0, 41.0]),
    ];

    fn newton_cotes(
        app: &App,
        endpoints: Endpoints,
        func: impl Fn(f64) -> f64,
    ) -> Vec<InterpolationPoint> {
        let (factor, weights) = Self::NEWTON_COTES_COEFFICIENTS[app.newton_cotes_n - 1];
        let n = app.newton_cotes_n as f64;

        weights
            .iter()
            .enumerate()
            .map(|(i, &weight)| {
                let i = i as f64;
                let step_h = (endpoints.end - endpoints.start) / n;

                let x = endpoints.start + i * step_h;
                let y = func(x);
                let area = factor * step_h * y * weight;

                InterpolationPoint { x, y, area }
            })
            .collect()
    }

    fn eval(&self, app: &App, func: impl Fn(f64) -> f64) -> Vec<InterpolationPoint> {
        match self {
            Algorithm::NewtonCotes => Self::newton_cotes(app, app.endpoints, func),
        }
    }
}
