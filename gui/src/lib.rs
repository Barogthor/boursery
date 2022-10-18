use std::sync::Arc;
pub use eframe;
pub use core::portfolio::PortfolioRepository;
use eframe::egui;
use eframe::egui::{ScrollArea, Ui};
use eframe::egui::TextStyle;

struct NewPortfolioState {
    name: String,
}
impl Default for NewPortfolioState {
    fn default() -> Self {
        Self {
            name: "Double momentum".to_owned()
        }
    }
}

struct AppState{
    new_porfolio: NewPortfolioState,
    portfolios: Vec<String>
}
impl Default for AppState {
    fn default() -> Self {
        Self {
            new_porfolio: Default::default(),
            portfolios: vec![]
        }
    }
}

pub struct AppGUI {
    state: AppState,
    portfolio_controller: PortfolioController
}

impl AppGUI {
    pub fn new(portfolio_repository: Arc<dyn PortfolioRepository>) -> Self {
        Self {
            state: AppState::default(),
            portfolio_controller: PortfolioController{repo: portfolio_repository}
        }
    }
}

impl eframe::epi::App for AppGUI {
    fn update(&mut self, ctx: &egui::Context, _frame: &eframe::epi::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            // ui.heading("My egui Application");
            ui.horizontal(|ui| {
                ui.label("Nom portefeuille: ");
                ui.text_edit_singleline(&mut self.state.new_porfolio.name);
                if ui.button("Créer portefeuille").clicked() && self.state.new_porfolio.name.len() > 0 {
                    let new_portfolio = std::mem::replace(&mut self.state.new_porfolio.name, String::new());
                    self.portfolio_controller.create_portfolio(&mut self.state, new_portfolio);
                }
            });
            ui.separator();
            let text_style = TextStyle::Body;
            let row_height = ui.text_style_height(&text_style);
            ScrollArea::vertical().auto_shrink([false;2])
                .show_rows(ui,
                           row_height,
                           self.state.portfolios.len(),
                           |ui, row_range| {
                               for row in row_range {
                                   ui.label(&self.state.portfolios[row]);
                               }
                           }
                );

        });
    }

    fn name(&self) -> &str {
        "Boursery advisor"
    }
}

pub struct PortfolioController {
    repo: Arc<dyn PortfolioRepository>
}

impl PortfolioController {
    fn create_portfolio(&self, state: &mut AppState, portfolio_name: String) {
        let request = core::portfolio::create_portfolio::Request { name: portfolio_name };
        let result = core::portfolio::create_portfolio::execute(request, self.repo.clone());
        if let Ok(portfolio) = result {
            state.portfolios.push(portfolio.name);
            state.new_porfolio.name = "".to_string();
            println!("portfolios: {:?}", state.portfolios);
        }
    }
}

