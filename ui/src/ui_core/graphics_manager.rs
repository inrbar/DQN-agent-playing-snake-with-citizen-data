use snake_game_rust::snake::data_types::Matrix;
use speedy2d::dimen::Vec2;
use speedy2d::error::{BacktraceError, ErrorMessage};
use speedy2d::shape::Rect;
use speedy2d::{color::Color, Graphics2D};
use speedy2d::font::{TextLayout, TextOptions, Font};


const SCREEN_CLEAR_COLOR : Color = Color::BLACK;
const _ASSETS_FOLDER_PATH : &str = "../../assets/";


const COLORS : [Color;4] = [
    Color::GRAY,
    Color::BLUE,
    Color::CYAN,
    Color::RED

];
// TODO : switch to Path module
pub struct GraphicManager{
    windows_size : (u32,u32),
    game_size : (f32, f32)

}

impl GraphicManager{

    pub fn new(windows_size : (u32, u32), game_size : (f32, f32)) -> GraphicManager{
        GraphicManager { windows_size: windows_size, game_size: game_size }   
    }

    fn load_font(&self, _font_name : &str) -> Result<Font,BacktraceError<ErrorMessage>>{
        let byte_font = include_bytes!("../../assets/fonts/Roboto-Black.ttf");
        Font::new(byte_font)

        // TODO : fix custom paths
    }
    // maybe add support for rectangles? 
    fn construct_simple_square(&self, left_corner : Vec2, side_length : f32) -> [Vec2;4]{
        [
            Vec2::new(left_corner.x,left_corner.y),
            Vec2::new(left_corner.x, left_corner.y+side_length),
            Vec2::new(left_corner.x+side_length, left_corner.y+side_length),
            Vec2::new(left_corner.x+side_length, left_corner.y)
        ]
    }

    pub fn clear_screen(&self, graphics : &mut Graphics2D){
        graphics.clear_screen(SCREEN_CLEAR_COLOR);
    }


    pub fn start_screen(&self, graphics : &mut Graphics2D){
        
        let font = self.load_font("").unwrap();
        let text_block = font.layout_text("Press Space to Start Q to quit\n Resets can take a moment due\n while waiting for Db response", 32.0, TextOptions::new());

        graphics.draw_text((self.windows_size.0 as f32/3.0,self.windows_size.1 as f32/5.0), Color::WHITE, &text_block);

    }

    pub fn game_over_screen(&self,score : usize, graphics : &mut Graphics2D){
        let font = self.load_font("").unwrap();
        let text = format!("Game Over \n Score : {}\n Press R to restart", score);
        let text_block = font.layout_text(&text, 32.0, TextOptions::new());

        graphics.draw_text((self.windows_size.0 as f32/3.0,self.windows_size.1 as f32/5.0), Color::WHITE, &text_block);

    }
 
    pub fn draw_game(&self, board: Matrix, graphics : &mut Graphics2D){
        let board_size_x = board.size.0 as f32;
        let board_size_y = board.size.1 as f32;
        let square_size = (self.game_size.0/board_size_x, self.game_size.1/board_size_y);
        let windows_size_x = self.windows_size.0 as f32;
        let windows_size_y = self.windows_size.1 as f32;
        let rect = Rect::new(
            Vec2::new((windows_size_x-self.game_size.0)/(2.0), (windows_size_y+self.game_size.1)/2.0),
            Vec2::new((windows_size_x+self.game_size.0)/2.0, (windows_size_y-self.game_size.1)/(2.0)));
        graphics.draw_rectangle(rect, COLORS[0]);

        for x in 0..board.size.0{
            let x_float = x as f32;
            
            for y in 0..board.size.1{
            let y_float = y as f32;

            let cell = board[(x,y)];
            
            match cell {
                1 | 2 | 3 => {
                    let verticies = self.construct_simple_square(
                        Vec2::new(x_float*square_size.0+(windows_size_x-self.game_size.0)/(2.0), y_float*square_size.1 + (windows_size_y-self.game_size.1)/(2.0)), 
                        square_size.0
                    );
                    let cell_as_index = cell as usize;
                    let color = COLORS[cell_as_index];
                    graphics.draw_quad(verticies, color);
                }

                _ => ()
            }
            
            }

        }
    }   


}