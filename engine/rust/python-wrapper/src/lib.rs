//! A python wrapper around the snake-game-rust package, allows
//! for usage in python projects, however there seems to be a issue
//! with `pylanace` recognizing imports and applying highlighting
//! 
//! # Installation
//! 
//! To install this package inside a python you will need to use the `maturin` 
//! python package. The maturin webpage does not clearly state that you need rust
//! installed, it probably is needed. You will probably need it for this project
//! anyway, so go to `https://www.rust-lang.org/tools/install` for a simple installation
//! 
//!  Assuming that you are using a Anaconda enviroment, you should
//! follow these steps
//! * activate you conda enviroment with `conda activate your-env-name`
//! * add the conda-forge channel `conda config --add channels conda-forge`
//! * set it as the default channel `conda config --set channel_priority strict`. 
//! Since this can affect other installations, you can either revert it afterwards 
//! with `conda config --set channel_priority flexible`
//! * and finally install `maturin` with `conda install maturin`
//! * then cd to the package directory so that the terminal path is something like
//! `(your env-name)C:\....\snake-nn\engine\rust\python-wrapper>`
//! * finally install the package locally with `maturin develop`
use pyo3::prelude::*;
use snake_game_rust::snake::*;


/// A wrapper around the `GameEngine` struct
#[pyclass]
pub struct EngineWrapper{
    engine : GameEngine
}


#[pymethods]
impl EngineWrapper{

    /// A python wrapper around the `GameEngine::new` function
    /// 
    /// Note that we do not have to call this function explicitly, since
    /// it is mapped to the `__init__` method.
    /// 
    /// Takes 3 inputs
    /// * `size` - a tuple of unsigned integers representing the
    /// size of the world grid on which the snake move
    /// * `starting_pos` - a tuple of unsigned integers representing the
    /// starting position of the snake head
    /// * `starting_direction` - an unsigned integer respresenting the starting
    /// direction of the snake.
    /// Currently  
    /// * 0 - left
    /// * 1 - up        
    /// * 2 - right
    /// * 3 - down
    /// 
    /// # Panics
    /// Panics if the either of the input is of incorrect type, either of 
    /// the size dimensions is 0, the snake would be spawned outside the grid or
    /// the direction is not mapped.
    /// 
    /// # Examples 
    /// 
    /// ```
    /// from python_wrapper import EngineWrapper
    /// 
    /// // creates an Engine with a world gird 10x10,
    /// // snake head on (5,5) and snake facing
    /// // 0 - left
    /// ew = EngineWrapper((10,10),(5,5),0)
    /// ```
    #[new]
    pub fn py_new(size : (usize,usize), starting_pos : (usize,usize), starting_direction : usize) -> Self{
        let mut ge = GameEngine::new(size, starting_pos,starting_direction);
        EngineWrapper{ engine : ge}
    }

    /// take an action as an input, forwards the game by one iteration,
    /// and returns information on the step.
    /// 
    /// Returns a tuple `(done, food_eaten, msg)`
    /// * `done` - a boolean, true if the game has ended in some way
    /// * `food_eaten` - a boolean, true if food has been eaten this iteration
    /// * `msg` - a string, a short message describing the iteration, mostly 
    /// used for learning enviroment 
    /// 
    /// For now `msg` can take on 4 values:
    /// 
    /// * `"alive"` - if snake is alive, 
    /// * `"body"` - if the snake has collided with its own body
    /// * `"wall"` - if the snake has collided with on of the walls
    /// * `"victory"` - if the snake body is taking up the entire world grid
    /// 
    /// # Panics
    /// 
    /// Will panic if supplied with an action that is not mapped
    /// 
    /// # Examples
    /// 
    /// ```
    /// from python_wrapper import EngineWrapper
    /// 
    /// // creates an Engine with a world gird 10x10,
    /// // snake head on (5,5) and snake facing
    /// // 0 - left
    /// ew = EngineWrapper((10,10),(5,5),0)
    /// 
    /// // Forwards the game with the move straight
    /// // action
    /// done, food_eaten, msg = game.step(1)
    /// 
    /// if done:
    ///     //some code to run after the game ends
    /// 
    /// ``` 
    pub fn py_step(&mut self, action : usize) -> (bool,bool,String){
        self.engine.step(action)
    }

    
    /// resets the world state given a seed 
    /// 
    /// # Panics
    /// 
    /// Should only panic if the type of the input seed is incorrect
    /// 
    /// # Examples
    /// 
    /// ```
    /// from python_wrapper import EngineWrapper
    /// 
    /// game = GameEngine((10,10), (5,5), 1)
    /// 
    /// // some code that advances the game
    /// 
    /// seed = 12
    /// game.reset(seed)
    /// 
    /// ``` 
    pub fn py_reset(&mut self, seed : u64){
        self.engine.reset(seed)
    }


    ///generic getter function for the game_world matrix
    /// 
    /// In python it returns a list of lists of integers,
    /// so it can be simply converted to a `ndarray`
    /// 
    pub fn py_get_world(&self) -> Vec<Vec<i32>>{
        self.engine.get_world().matrix
    }

    ///generic getter function for the position of the snake head
    /// 
    /// In python it returns a list of two coordinates
    pub fn py_get_snake_head(&self) -> Vec<i32>{
        let snake_head = self.engine.get_snake_head();

        vec![snake_head.x,snake_head.y]
    }

    ///gerneric getter function for the position of the food.
    /// 
    /// In python it returns a list of two coordinates
    pub fn py_get_food_pos(&self) -> Vec<i32>{
        let food_pos = self.engine.get_food_pos();

        vec![food_pos.x,food_pos.y]
    }


}

/// A Python module implemented in Rust.
#[pymodule]
fn python_wrapper(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_class::<EngineWrapper>()?;
    Ok(())
}