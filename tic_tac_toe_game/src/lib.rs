//using statements
use eframe::{egui::{self, CentralPanel, TopBottomPanel, Ui, Context, FontDefinitions, FontData}, App, epaint::FontFamily};

//constants
///Represents the symbols in a tiik tak toe game
const SYMBOLIDS: [&'static str; 2] = ["O", "X"];
//struct app

///Game instance
///Stores all necessary information for a game
#[derive(Default)]
pub struct Game{
    round: i8,
    finished: bool,
    is_there_awinner: bool,
    field: Vec<Vec<String>>,
}

impl Game{
    //for settig the default values
    ///Sets the default values for a game
    pub fn new() -> Game{
        Game{
            round: 0,
            finished: false,
            is_there_awinner: false,
            //that looks like this:
            //_ _ _ 
            //_ _ _
            //_ _ _
            field: vec![vec![
                " ".to_string(),
                " ".to_string(),
                " ".to_string(),
            ],
            vec![
                " ".to_string(),
                " ".to_string(),
                " ".to_string(),
            ],
            vec![
                " ".to_string(),
                " ".to_string(),
                " ".to_string(),
            ]]
        }
    }

    ///Called when the user wants to play another game
    /// 
    ///Fails when the game is null
    fn restart(&mut self){
        *self = Game::new();
    }

    //ask for a new game
    ///A component that asks for a new game
    /// 
    ///toggles popup when player presses button
    fn play_again(&mut self, ui: &mut Ui) {
        let answer = ui.add_sized((45., 35.), egui::Button::new("Play Again"));
        let popup_id = ui.make_persistent_id("popup");
        if answer.clicked(){
            ui.memory_mut(|mem| mem.toggle_popup(popup_id));
        }
        let above = egui::AboveOrBelow::Above;
        //thats the popup for a new game
        egui::popup::popup_above_or_below_widget(ui, popup_id, &answer, above, |ui| {
            ui.add_sized((100., 70.), egui::Label::new("Do you want to play again?"));
            ui.add_space(15.);
            if ui.add_sized((50., 30.), egui::Button::new("Yes")).clicked(){
                self.restart();
            }
        });
    }

    //check if the game is finished
    ///Main endpoint for game validation
    /// 
    ///Fails when the game is null
    fn check(&mut self){
        for s in SYMBOLIDS{
            diagonal(self, s);
            vertical(self, s);
            horizontal(self, s)
        }
        all_fields_full(self);
    }

    //set the font size to see the buttons better
    ///Sets the font of the application
    /// 
    ///Panics when the ttf file isn't valid
    fn set_font_size(&self, ctx: &Context){
        let mut font_def = FontDefinitions::default();
        
        //add arial font
        font_def.font_data.insert(
            "Arial".to_string(),
            FontData::from_static(include_bytes!("../fonts/ARIALBD.TTF")),
        );

        font_def.families.get_mut(&FontFamily::Proportional).unwrap().insert(0, "Arial".to_string());

        ctx.set_fonts(font_def);
    }

    ///The gaming field is split into three parts
    /// 
    /// A component functions
    /// 
    /// Thats line one
    /// 
    /// Three buttons are rendered where the content of the button: X OR O depends on the round number
    fn render_first_line(&mut self, ui: &mut Ui){
        ui.horizontal(|ui| {

            // }
            //button 1
            if ui.add_sized((60., 60.), egui::Button::new(&self.field[0][0])).clicked(){
                if !self.finished{
                    if self.field[0][0] == " ".to_string(){
                        if &self.round % 2 == 1 {
                            self.field[0][0] = "O".to_string();
                        }else{
                            self.field[0][0] = "X".to_string();
                        }
                        self.round += 1;
                        self.check();
                    }
                }
            }
            //button 2
            if ui.add_sized((60., 60.), egui::Button::new(&self.field[0][1])).clicked(){
                if !self.finished{
                    if self.field[0][1] == " ".to_string(){
                        if &self.round % 2 == 1 {
                            self.field[0][1] = "O".to_string();
                        }else{
                            self.field[0][1] = "X".to_string();
                        }
                        self.round += 1;
                        self.check();
                    }
                }
            }
            //button 3
            if ui.add_sized((60., 60.), egui::Button::new(&self.field[0][2])).clicked(){
                if !self.finished{
                    if self.field[0][2] == " ".to_string(){
                        
                        if &self.round % 2 == 1{
                            self.field[0][2] = "O".to_string();
                        }else{
                            self.field[0][2] = "X".to_string();
                        }
                        self.round += 1;
                        self.check();   
                    }
                }
            }
        }); 
    }

    ///A component function
    /// 
    /// Second line of the gamefield
    /// 
    /// A horizontal container widget contains the buttons
    /// 
    /// Every button represents one symbol of the game field property
    /// 
    /// When the button is clicked, it checks if the game is finished
    /// if not it looks with a modulo of 2 what round we have currently and asigns it to the button 
    fn render_second_line(&mut self, ui: &mut Ui){
        ui.horizontal(|ui|{
                //button 1
            if ui.add_sized((60., 60.), egui::Button::new(&self.field[1][0])).clicked(){
                if !self.finished{
                    if self.field[1][0] == " ".to_string(){
                        if &self.round % 2 == 1 {
                            self.field[1][0] = "O".to_string();
                        }else{
                            self.field[1][0] = "X".to_string();
                        }
                        self.round += 1;
                        self.check();
                    }
                }
            }
            //button 2
            if ui.add_sized((60., 60.), egui::Button::new(&self.field[1][1])).clicked(){
                if !self.finished{
                    if self.field[1][1] == " ".to_string(){
                        if &self.round % 2 == 1 {
                            self.field[1][1] = "O".to_string();
                        }else{
                            self.field[1][1] = "X".to_string();
                        }
                        self.round += 1;
                        self.check();
                    }
                }
            }
            //button 3
            if ui.add_sized((60., 60.), egui::Button::new(&self.field[1][2])).clicked(){
                if !self.finished{
                    if self.field[1][2] == " ".to_string(){
                        if &self.round % 2 == 1 {
                            self.field[1][2] = "O".to_string();
                        }else{
                            self.field[1][2] = "X".to_string();
                        }
                        self.round += 1;
                        self.check();
                    }
                }
            }
        });
    }

    ///A component function
    /// 
    /// The third line of the gamefield
    /// 
    /// 
    fn render_third_line(&mut self, ui: &mut Ui){
        ui.horizontal(|ui|{
                //button 1
            if ui.add_sized((60., 60.), egui::Button::new(&self.field[2][0])).clicked(){
                if !self.finished{
                    if self.field[2][0] == " ".to_string(){
                        if &self.round % 2 == 1 {
                            self.field[2][0] = "O".to_string();
                        }else{
                            self.field[2][0] = "X".to_string();
                        }
                        self.round += 1;
                        self.check();
                    }
                }
            }
            //button 2
            if ui.add_sized((60., 60.), egui::Button::new(&self.field[2][1])).clicked(){
                if !self.finished{
                    if self.field[2][1] == " ".to_string(){
                        if &self.round % 2 == 1 {
                            self.field[2][1] = "O".to_string();
                        }else{
                            self.field[2][1] = "X".to_string();
                        }
                        self.round += 1;
                        self.check();
                    }
                }
            }
            //button 3
            if ui.add_sized((60., 60.), egui::Button::new(&self.field[2][2])).clicked(){
                if !self.finished{
                    if self.field[2][2] == " ".to_string(){
                        if &self.round % 2 == 1 {
                            self.field[2][2] = "O".to_string();
                        }else{
                            self.field[2][2] = "X".to_string();
                        }
                        self.round += 1;
                        self.check();
                    }    
                }                
            }
        });
    }
}

//all field are used?
///Checker function
/// 
/// loops over all fields and checks if the field isn't empty
/// 
/// counter is the number of fields that are not empty -> when all fields aren't empty it will stop the game
/// 
/// Fails when the game is null OR the field is null
fn all_fields_full(game: &mut Game ){
    //counts the number of used fields
    let mut counter = 0;

    for line in &game.field{
        for f in line{
            if f != &' '.to_string(){
                counter += 1;
            }
        }
    }

    if counter == 9{
        game.finished = true;
    }
}

//checking functions
///There are several ways to win the game
/// 
/// Here are functions if any of the two players won the game with any of the
/// winning cases
/// 
/// These is the first function to check the possibility of winning on a diagonal
/// 
/// Fails when the symbol argument is null OR the game is null
/// 
/// Changes the symbols of the winning three combination to a more bolder version of the button contents
fn diagonal(game: &mut Game, symbol: &str){
    //first possability
    //layout:
    //X _ _
    //_ X _
    //_ _ X
    if game.field[0][0] == symbol && game.field[1][1] == symbol && game.field[2][2] == symbol{
        if symbol == "X"{
            game.field[0][0] = "❌".to_string();
            game.field[1][1] = "❌".to_string();
            game.field[2][2] = "❌".to_string();
        }else{
            game.field[0][0] = "⭕".to_string();
            game.field[1][1] = "⭕".to_string();
            game.field[2][2] = "⭕".to_string();
        }
        game.finished = true;
        game.is_there_awinner = true;
    }

    //second possability
    //layout: 
    //_ _ X
    //_ X _ 
    //X _ _ 
    if game.field[0][2] == symbol && game.field[1][1] == symbol && game.field[2][0] == symbol{
        if symbol == "X"{
            game.field[0][2] = "❌".to_string();
            game.field[1][1] = "❌".to_string();
            game.field[2][0] = "❌".to_string();
        }else{
            game.field[0][2] = "⭕".to_string();
            game.field[1][1] = "⭕".to_string();
            game.field[2][0] = "⭕".to_string();
        }
        game.finished = true;
        game.is_there_awinner = true;
    }
}

/// A checking function 
/// 
/// Looks over all vertical lines if a user won the game
/// 
/// Fails if one of the arguments are null
/// 
/// Replaces the current line with the special characters
fn vertical(game: &mut Game, symbol: &str){
    //first case
    //layout:
    //X _ _
    //X _ _
    //X _ _
    if game.field[0][0] == symbol && game.field[1][0] == symbol && game.field[2][0] == symbol{
        if symbol == "X"{
            game.field[0][0] = "❌".to_string();
            game.field[1][0] = "❌".to_string();
            game.field[2][0] = "❌".to_string();
        }else{
            game.field[0][0] = "⭕".to_string();
            game.field[1][0] = "⭕".to_string();
            game.field[2][0] = "⭕".to_string();
        }
        game.finished = true;
        game.is_there_awinner = true;
    }

    //second case
    //layout:
    //_ X _
    //_ X _
    //_ X _ 
    if game.field[0][1] == symbol && game.field[1][1] == symbol && game.field[2][1] == symbol{
        if symbol == "X"{
            game.field[0][1] = "❌".to_string();
            game.field[1][1] = "❌".to_string();
            game.field[2][1] = "❌".to_string();
        }else{
            game.field[0][1] = "⭕".to_string();
            game.field[1][1] = "⭕".to_string();
            game.field[2][1] = "⭕".to_string();
        }
        game.finished = true;
        game.is_there_awinner = true;
    }

    //third case
    //layout:
    //_ _ X
    //_ _ X
    //_ _ X
    if game.field[0][2] == symbol && game.field[1][2] == symbol && game.field[2][2] == symbol{
        if symbol == "X"{
            game.field[0][2] = "❌".to_string();
            game.field[1][2] = "❌".to_string();
            game.field[2][2] = "❌".to_string();
        }else{
            game.field[0][2] = "⭕".to_string();
            game.field[1][2] = "⭕".to_string();
            game.field[2][2] = "⭕".to_string();
        }
        game.finished = true;
        game.is_there_awinner = true;
    }
}

/// A checking function 
/// 
/// Makes sure that maybe somebody won on a horizontal
/// 
/// Fails if the arguments are null
/// 
/// Sets the winner property of true so that the checker knows that the game has ended successfully
fn horizontal(game: &mut Game, symbol: &str){
    //first line
    //layout:
    //X X X
    //_ _ _ 
    //_ _ _
    if game.field[0][0] == symbol && game.field[0][1] == symbol && game.field[0][2] == symbol{
        if symbol == "X"{
            game.field[0][0] = "❌".to_string();
            game.field[0][1] = "❌".to_string();
            game.field[0][2] = "❌".to_string();
        }else{
            game.field[0][0] = "⭕".to_string();
            game.field[0][1] = "⭕".to_string();
            game.field[0][2] = "⭕".to_string();
        }
        game.finished = true;
        game.is_there_awinner = true;
    }

    //second line 
    //layout:
    //_ _ _
    //X X X
    //_ _ _
    if game.field[1][0] == symbol && game.field[1][1] == symbol && game.field[1][2] == symbol{
        if symbol == "X"{
            game.field[1][0] = "❌".to_string();
            game.field[1][1] = "❌".to_string();
            game.field[1][2] = "❌".to_string();
        }else{
            game.field[1][0] = "⭕".to_string();
            game.field[1][1] = "⭕".to_string();
            game.field[1][2] = "⭕".to_string();
        }
        game.finished = true;
        game.is_there_awinner = true;
    }

    //third line
    //layout:
    //_ _ _
    //_ _ _
    //X X X
    if game.field[2][0] == symbol && game.field[2][1] == symbol && game.field[2][2] == symbol{
        if symbol == "X"{
            game.field[2][0] = "❌".to_string();
            game.field[2][1] = "❌".to_string();
            game.field[2][2] = "❌".to_string();
        }else{
            game.field[2][0] = "⭕".to_string();
            game.field[2][1] = "⭕".to_string();
            game.field[2][2] = "⭕".to_string();
        }
        game.finished = true;
        game.is_there_awinner = true;
    } 
}

impl App for Game{
    fn save(&mut self, _storage: &mut dyn eframe::Storage) {}

    fn on_close_event(&mut self) -> bool {
        true
    }

    

    fn on_exit(&mut self, _gl: Option<&eframe::glow::Context>) {}

    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        self.set_font_size(ctx);
        
        //header part
        TopBottomPanel::top("top").show(ctx, |ui| {
            ui.label("TikTakToe");
        });
        
        CentralPanel::default().show(ctx, |ui| {
            ctx.set_visuals(egui::Visuals::light());
            //check if game is finished
            //buttons which contain game symbols
            //-> store game symbols in field property
            //-> check if game is finished
            //-> when a plauer already has a symbol in the field in cant be removed
            if self.finished{
                ui.label("Game over!");
                ui.add_space(15.);
                
                if self.is_there_awinner {
                    if self.round % 2 == 0{
                        ui.label("Player two won!");
                    }else{
                        ui.label("Player one won!");
                    }
                }
                ui.add_space(15.);
                self.play_again(ui);
                //return;
            }
            ui.add_space(15.);
            //first row of the game fields
            // ui.horizontal_centered(|ui| {
            self.render_first_line(ui);
            // });

            //row 2 
            // ui.horizontal_centered(|ui|{
            self.render_second_line(ui);
            // });

            //row 3 
            // ui.horizontal_centered(|ui|{
            self.render_third_line(ui);
            // });

        });
    }

    fn auto_save_interval(&self) -> std::time::Duration {
        std::time::Duration::from_secs(30)
    }

    fn max_size_points(&self) -> egui::Vec2 {
        egui::Vec2::INFINITY
    }

    fn clear_color(&self, _visuals: &egui::Visuals) -> [f32; 4] {
        // NOTE: a bright gray makes the shadows of the windows look weird.
        // We use a bit of transparency so that if the user switches on the
        // `transparent()` option they get immediate results.
        egui::Color32::from_rgba_unmultiplied(12, 12, 12, 180).to_normalized_gamma_f32()

        // _visuals.window_fill() would also be a natural choice
    }

    fn persist_native_window(&self) -> bool {
        true
    }

    fn persist_egui_memory(&self) -> bool {
        true
    }

    fn warm_up_enabled(&self) -> bool {
        false
    }

    fn post_rendering(&mut self, _window_size_px: [u32; 2], _frame: &eframe::Frame) {}


}


