use iced::widget::{text, row, column, container};
use iced::{Alignment, Element, Length};

use crate::cell;

pub struct Game<'a> {
    matrix: Vec<&'a str>,
    player: &'a str,
    winning_row: Vec<i32>,
    game_result: String 
}



#[derive(Debug, Clone, Copy)]
pub enum Message {
    CellPressed11,
    CellPressed12,
    CellPressed13,
    CellPressed21,
    CellPressed22,
    CellPressed23,
    CellPressed31,
    CellPressed32,
    CellPressed33
}


impl<'a> Game<'a> {


    pub fn new() -> Self{
        let value: Vec<&str> = [" "].repeat(9);
        Self{
            matrix: value,
            player: "X",
            winning_row: Vec::new(),
            game_result: "".to_string()
        }
    }

    pub fn is_valid_index(&self, index: usize) -> bool {
        index < 9 && self.matrix[index] == " "  
    }

    fn row_win(&mut self) -> bool {

        if self.matrix[0] != " " && (self.matrix[0] == self.matrix[1] && 
            self.matrix[0] == self.matrix[2]) {
            self.winning_row = vec![0, 1, 2];
            true 
        } else if self.matrix[3] != " " && (self.matrix[3] == self.matrix[4] && 
            self.matrix[3] == self.matrix[5])  {
            self.winning_row = vec![3, 4, 5];
            true
        } else if  self.matrix[6] != " " && (self.matrix[6] == self.matrix[7] && 
            self.matrix[6] == self.matrix[8]) {
            self.winning_row = vec![6, 7, 8];
            true 
        } else {
            false
        }

    }

    fn column_win(&mut self) -> bool {

        if self.matrix[0] != " " && (self.matrix[0] == self.matrix[3] &&
                                     self.matrix[0] == self.matrix[6]) {
            self.winning_row = vec![0, 3, 6];
            true 
        } else if self.matrix[1] != " " && (self.matrix[1] == self.matrix[4] &&
                                     self.matrix[1] == self.matrix[7]) {
            self.winning_row = vec![1, 4, 7];
            true 
        } else if self.matrix[2] != " " && (self.matrix[2] == self.matrix[5] &&
                                     self.matrix[2] == self.matrix[8]) {
            self.winning_row = vec![2, 5, 8];
            true 
        } else {
            false 
        }

    }

    fn diagonal_win(&mut self) -> bool {
        if self.matrix[0] != " " && (self.matrix[0] == self.matrix[4] &&
                                     self.matrix[0] == self.matrix[8]) {
            self.winning_row = vec![0, 4, 8];
            true 
        } else if self.matrix[2] != " " && (self.matrix[2] == self.matrix[4] &&
                                            self.matrix[2] == self.matrix[6]) {
            self.winning_row = vec![2, 4, 6];
            true 
        } else {
            false 
        }

    }


    fn win(&mut self) -> bool {
        self.row_win() || self.column_win() || self.diagonal_win() 
    }

    fn draw(&self) -> bool {
       for val in &self.matrix {
           if val == &" " {
               return false
           }

       }
       true
    }


    // Check if current value of matrix results in win (0), draw (1) or
    // incomplete (2)
    //
    pub fn status(&mut self) -> u8 {
        if self.win() {
            0
        } else if self.draw() {
            1
        } else {
            2
        }
    }

    fn update_player(&mut self) {
        self.player = if self.player == "X" {"O"} else {"X"}
    }

    pub fn update(&mut self, message: Message) {
        match message {
            Message::CellPressed11 => {
                if self.is_valid_index(0) {
                    self.matrix[0] = self.player
                }
           },
           Message::CellPressed12 => {
               if self.is_valid_index(1) {
                    self.matrix[1] = self.player
                }
           },
           Message::CellPressed13 => {
               if self.is_valid_index(2) {
                    self.matrix[2] = self.player
                }
           },
           Message::CellPressed21 => {
               if self.is_valid_index(3) {
                    self.matrix[3] = self.player
                }
           },
           Message::CellPressed22 => {

               if self.is_valid_index(4) {
                    self.matrix[4] = self.player
                }
           },
           Message::CellPressed23 => {
               if self.is_valid_index(5) {
                    self.matrix[5] = self.player
                }
           },
           Message::CellPressed31 => {
               if self.is_valid_index(6) {
                    self.matrix[6] = self.player
                }
           }, 
           Message::CellPressed32 => {
               if self.is_valid_index(7) {
                    self.matrix[7] = self.player
                }
           },
           Message::CellPressed33 => {
               if self.is_valid_index(8) {
                    self.matrix[8] = self.player
                }
           }
        }

        match self.status() {
            0 => self.game_result = format!("Congratulations! {} Won.", self.player),
            1 => self.game_result = "Game Drew.".to_string(),
            _ => self.update_player()
        }
    }

    pub fn view(&self) -> Element<Message> {
        let content = column![
            text(format!("{}'s turn", self.player)).size(40),
            row![
                cell(self.matrix[0], 40.0, Message::CellPressed11),
                cell(self.matrix[1], 40.0, Message::CellPressed12),
                cell(self.matrix[2], 40.0, Message::CellPressed13)
            ].spacing(1),
            row![
                cell(self.matrix[3], 40.0, Message::CellPressed21),
                cell(self.matrix[4], 40.0, Message::CellPressed22),
                cell(self.matrix[5], 40.0, Message::CellPressed23)
            ].spacing(1),
            row![
                cell(self.matrix[6], 40.0, Message::CellPressed31),
                cell(self.matrix[7], 40.0, Message::CellPressed32),
                cell(self.matrix[8], 40.0, Message::CellPressed33)
            ].spacing(1),
            text(self.game_result.clone()).size(40)
        ]
        .padding(20)
        .spacing(1)
        .max_width(500)
        .align_items(Alignment::Center);

        container(content)
            .width(Length::Fill)
            .height(Length::Fill)
            .center_x()
            .center_y()
            .into()
    }

}

impl<'a> Default for Game<'a> {
    fn default() -> Self {
        Self::new()
    }
}
