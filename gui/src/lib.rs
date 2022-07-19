pub use eframe;
use eframe::egui;
use eframe::egui::ScrollArea;
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
    state: AppState
}

impl Default for AppGUI {
    fn default() -> Self {
        Self {
            state: AppState::default()
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
            });
            if ui.button("Créer portefeuille").clicked() && self.state.new_porfolio.name.len() > 0 {
                let new_portfolio = std::mem::replace(&mut self.state.new_porfolio.name, String::new());
                self.state.portfolios.push(new_portfolio);
                println!("portfolios: {:?}", self.state.portfolios);
            }

            let text_style = TextStyle::Body;
            let row_height = ui.text_style_height(&text_style);
            ScrollArea::vertical().auto_shrink([false;2])
                .show_rows(ui,
                    row_height,
                   self.state.portfolios.len(),
                   |ui, row_range| {
                           for row in row_range {
                               ui.text_edit_singleline(&mut self.state.portfolios[row]);
                           }
                       }
                );
        });
    }

    fn name(&self) -> &str {
        "TKL Boursery"
    }
}