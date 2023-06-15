import numpy as np
import gym
from typing import Tuple,Optional
import python_wrapper
from gym import spaces


class SnakeEnv(gym.Env):

    def __init__(self,size : Tuple[int,int], starting_pos : Tuple[int,int]) -> None:
        
        self.ge = python_wrapper.EngineWrapper(size,starting_pos)
        self.action_space = spaces.Discrete(3)

        obs_high = np.zeros(size).flatten() + 2
        obs_low = np.zeros(size).flatten()
        self.observation_space = spaces.Box(low=obs_low,high=obs_high,dtype=np.int32)

    def reward(self) -> float:
        return 0
    
    def info(self) -> dict:
        return {}

    def step(self, action):
        
        done,msg = self.ge.step()

        observation = np.array(self.ge.py_get_world()).flatten()

        reward = self.reward()

        info = self.info()

        return observation,reward,done,info
    

    def reset(self,seed: Optional[int] = None) -> Tuple[np.ndarray, dict]:
        
        if seed != None:
            print("set seeding not yet implemented")
        
        self.ge.py_reset()

        observation = np.array(self.ge.py_get_world()).flatten()

        info = self.info()

        return observation,info