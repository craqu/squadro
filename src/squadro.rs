
use std::{vec, array};

struct Board {
    game_in_progress : bool,
    upward_position : Vec<i32>,
    sideward_position : Vec<i32> 
}
struct Pion {
    index : u8,
    is_upward : bool,
    position : u8
}
impl Pion {
    fn bouge(&mut self, position: u8) {
        self.position = position;
    }
    fn position(self) -> u8 {
        self.position
    }
}

fn print_board(upward_position : [Pion ; 5], sideward_position : [Pion ; 5]) {
    let board = vec![
            "       . | . : | : : | : : | : . | .     ",
            "         |   . | .   |   . | .   |       ",
            "  ...    |     |     |     |     |      .",
            "1 ───────┼─────┼─────┼─────┼─────┼───────",
            "  ...    |     |     |     |     |      .",
            "  .      |     |     |     |     |    ...",
            "2 ───────┼─────┼─────┼─────┼─────┼───────",
            "  .      |     |     |     |     |    ...",
            "  ..     |     |     |     |     |     ..",
            "3 ───────┼─────┼─────┼─────┼─────┼───────",
            "  ..     |     |     |     |     |     ..",
            "  .      |     |     |     |     |    ...",
            "4 ───────┼─────┼─────┼─────┼─────┼───────",
            "  .      |     |     |     |     |    ...",
            "  ...    |     |     |     |     |      .",
            "5 ───────┼─────┼─────┼─────┼─────┼───────",
            "  ...    |     |     |     |     |      .",
            "       . | .   |     |     |   . | .     ",
            "       : | : . | . : | : . | . : | .     "];
    // place les pions des joueurs sur le board

    for line in board{
        println!{"{}",line};
    }
    enum Position{
        One,
        Two,
        Tree,
        Four,
        Five
    }
}
fn place_joueur(){}

fn place_un_charactere(){}

fn place_un_pion_h(){}

fn place_un_pion_v(){}

fn formatter_les_parties(){}