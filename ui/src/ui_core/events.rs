use crate::MyWindowHandler;
use speedy2d::window::{WindowHandler, WindowHelper, KeyScancode, VirtualKeyCode};
use speedy2d::{Graphics2D};
use speedy2d::dimen::UVec2;




impl WindowHandler for MyWindowHandler {
    fn on_key_down(&mut self,helper: &mut WindowHelper, virtual_key_code: Option<VirtualKeyCode>,_scancode: KeyScancode) {
        self.input_manager.event_on_key_down(virtual_key_code, helper);
    }

    fn on_key_up(&mut self,_helper: &mut WindowHelper, virtual_key_code: Option<VirtualKeyCode>,_scancode: KeyScancode) {
       self.input_manager.event_on_key_up(virtual_key_code);
    }

    fn on_draw(&mut self, helper: &mut WindowHelper, graphics: &mut Graphics2D) {
        
        
        self.input_manager.update_game_state(graphics);
        helper.request_redraw()
       
    }

    fn on_resize(&mut self,helper: &mut WindowHelper, _size_pixels: UVec2) {
        WindowHelper::set_size_pixels(helper, (1000, 1000))
    }
}
