use std::collections::{HashMap};
use speedy2d::window::WindowHelper;
use speedy2d::{window::VirtualKeyCode, Graphics2D};
use std::time::Instant;

use super::game_manager::{GameManager, UIState};
use super::timer::Timer;

const ACTION_KEYS : [VirtualKeyCode;4]= [
    VirtualKeyCode::A,
    VirtualKeyCode::D,
    VirtualKeyCode::Left,
    VirtualKeyCode::Right
];
pub struct  InputManager {
    pressed_keys : HashMap<VirtualKeyCode,bool>,
    released_keys : HashMap<VirtualKeyCode,bool>,
    pressed_keys_last_frame : HashMap<VirtualKeyCode,bool>,
    game_manager : GameManager, 
    timer : Timer,
 
}

impl InputManager {
    
    pub fn new(game_manager : GameManager) -> InputManager{
        let mut  keymap = HashMap::new();
        for key in ACTION_KEYS{
            keymap.insert(key, false);
        }

        InputManager { 
            pressed_keys: keymap.clone(), 
            released_keys : keymap.clone(),
            pressed_keys_last_frame : keymap,
            game_manager: game_manager, 
            timer : Timer { start_time: Instant::now(), elapsed_secs: 0.0, sec_counter : 0.0  }
        }

    }

    pub fn event_on_key_down(&mut self, key_code : Option<VirtualKeyCode>,helper : &mut WindowHelper){

        match key_code {
            Some(VirtualKeyCode::R) => {
                if self.game_manager.state == UIState::GameOver{
                    self.game_manager.reset_game();
                    self.timer.reset_timer();
                }

            },
            Some(VirtualKeyCode::Space) => {
                if self.game_manager.state == UIState::StartScreen{
                    self.game_manager.start_game();
                    self.timer.reset_timer();    
                }

            },
            Some(VirtualKeyCode::Q) => {
                helper.terminate_loop();
            },
            Some(key_code) => {
                if ACTION_KEYS.contains(&key_code){
                    self.pressed_keys.insert(key_code, true);
                }
            },
            None => {}
        }
    
    }

    pub fn event_on_key_up(&mut self, key_code : Option<VirtualKeyCode>){
        match key_code {
            Some(key_code) => {
                if ACTION_KEYS.contains(&key_code){
                    self.released_keys.insert(key_code, true);
                }
            },
            None => ()
            
        }
    }

    pub fn update_game_state(&mut self, graphics : &mut Graphics2D){
        if self.timer.has_frame_passed(){

            let mut user_action : i32 = 1;
            
            //this might be extremly confusing be here goes

            for key in ACTION_KEYS{
                //for every key we check this insane condition
                if match (self.pressed_keys_last_frame[&key],self.pressed_keys[&key],self.released_keys[&key]) {
                    (_,false,_) => false, //if the key was no pressed this frame we take no action
                    (false,true,_) => true,//it the key was pressed this frame and not on the previous frame we always take action
                    (true,true,true) => false,//if the key was pressed on both this and previous frame and released we take no action
                    (true,true,false) => true,// and if the key was held on both this and previous frame and not released we take action
                    //this aims to remove the the tradoff that we had previously
                    //either update the pressed keys before action and thus we would miss quick taps,
                    //or update after and let action prolong for one extra frame
                }{
                    if key == VirtualKeyCode::A || key == VirtualKeyCode::Left {
                        user_action += 1;
                    }
                    if key == VirtualKeyCode::D || key == VirtualKeyCode::Right {
                        user_action += -1;
                    }
                }
            }
            
            
            //pressed keys that were not released are marked as pressed last frame
            for key in ACTION_KEYS{
                if self.pressed_keys[&key] && !self.released_keys[&key]{
                    self.pressed_keys_last_frame.insert(key, true);
                }else {
                    self.pressed_keys_last_frame.insert(key, false);

                }
            }
            //this might seem strange but it has a reason
            //basically the way windows gets key press events if the key is held is annoying for us
            //When the key is pressed for the first time we get an event, then there is a short break and
            //then we get constant events on even intervals
            //This was a problem because the game might advanece through few frames during that break
            //so we make sure to only un-press those keys, which were released 
            for key in ACTION_KEYS{
                if self.released_keys[&key]{
                    self.pressed_keys.insert(key, false);
                }

            }
            //all keys are marked as not released for the next frame
            for key in ACTION_KEYS{
                self.released_keys.insert(key, false);
            }
            let user_action = user_action as usize;
            self.game_manager.user_action(user_action, graphics);
    
        }
        
        
    }

}