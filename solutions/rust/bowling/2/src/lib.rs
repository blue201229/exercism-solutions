#[derive(Debug, PartialEq, Eq)]
pub enum Error {
    NotEnoughPinsLeft,
    GameComplete,
}

pub struct BowlingGame {
    rolls: Vec<u16>,
}

impl BowlingGame {
    pub fn new() -> Self {
        BowlingGame { rolls: Vec::new() }
    }

    pub fn roll(&mut self, pins: u16) -> Result<(), Error> {
        if self.is_game_complete() {
            return Err(Error::GameComplete);
        }

        // Validate pins are in range [0, 10]
        if pins > 10 {
            return Err(Error::NotEnoughPinsLeft);
        }

        // Check for invalid roll in current frame
        let frame_info = self.get_frame_info();
        let (current_frame, rolls_in_frame, pin_in_frame) = frame_info;

        if (pin_in_frame as u16) < pins {
            return Err(Error::NotEnoughPinsLeft);
        }
        if current_frame > 9 || (current_frame == 9  && rolls_in_frame > 2){
            return Err(Error::GameComplete);
        } else if current_frame < 9 && rolls_in_frame > 1{
            return Err(Error::NotEnoughPinsLeft);
        }

        self.rolls.push(pins);
        Ok(())
    }

    pub fn score(&self) -> Option<u16> {
        if !self.is_game_complete() {
            return None;
        }

        let mut total_score: u16 = 0;
        let mut roll_index = 0;

        for frame in 0..10 {
            if frame < 9 {
                // Frames 1-9
                if self.rolls[roll_index] == 10 {
                    // Strike
                    total_score += 10 + self.rolls[roll_index + 1] + self.rolls[roll_index + 2];
                    roll_index += 1;
                } else if self.rolls[roll_index] + self.rolls[roll_index + 1] == 10 {
                    // Spare
                    total_score += 10 + self.rolls[roll_index + 2];
                    roll_index += 2;
                } else {
                    // Open frame
                    total_score += self.rolls[roll_index] + self.rolls[roll_index + 1];
                    roll_index += 2;
                }
            } else {
                // 10th frame: just sum all remaining rolls
                while roll_index < self.rolls.len() {
                    total_score += self.rolls[roll_index];
                    roll_index += 1;
                }
            }
        }

        Some(total_score)
    }
    
    fn get_frame_info(&self) -> (usize, usize, usize) {
        // Returns (current_frame, rolls_in_current_frame, pin_in_frame)
        let mut frame = 0;
        let mut roll_index = 0;
        let mut pin_in_frame = 10;
        let mut start_frame_roll = 0;
        while roll_index < self.rolls.len() {
            pin_in_frame -= self.rolls[roll_index];
            if pin_in_frame < 0 {
                panic!("Critical failure:Frame{} Roll{} Too Much Pins{}", frame, roll_index, self.rolls[roll_index]); 
            } else {
                roll_index += 1;
                if pin_in_frame == 0 {
                    // Strike or Spare
                        if frame < 9 {
                            frame += 1;
                            start_frame_roll = roll_index;
                        }
                    pin_in_frame = 10; 
                } else {
                    if frame < 9 && roll_index > start_frame_roll + 1{
                        // Second roll of the frame
                        pin_in_frame = 10;
                        frame += 1;
                        start_frame_roll = roll_index;
                    }
                }
            }
        }        
        // We've completed all frames so far, next roll will start frame
        (frame, self.rolls.len() - start_frame_roll as usize , pin_in_frame as usize)
    }

    fn is_game_complete(&self) -> bool {
        if self.rolls.is_empty() {
            return false;
        }

        let mut roll_index = 0;

        for frame in 0..9 {
            if roll_index >= self.rolls.len() {
                return false;
            }

            if self.rolls[roll_index] == 10 {
                // Strike
                roll_index += 1;
            } else {
                // Need 2 rolls for this frame
                if roll_index + 1 >= self.rolls.len() {
                    return false;
                }
                roll_index += 2;
            }
        }

        // 10th frame
        if roll_index >= self.rolls.len() {
            return false;
        }

        let first_10th = self.rolls[roll_index];
        roll_index += 1;

        if first_10th == 10 {
            // Strike in 10th: need 2 more rolls
            return roll_index + 1 < self.rolls.len();
        } else if roll_index >= self.rolls.len() {
            return false;
        }

        let second_10th = self.rolls[roll_index];
        roll_index += 1;

        if first_10th + second_10th == 10 {
            // Spare in 10th: need 1 more roll
            return roll_index < self.rolls.len();
        }

        // Open frame in 10th: we have all we need
        true
    }
}