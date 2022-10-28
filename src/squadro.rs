// Change since last commit :

// Changed is_upward to is_coming_back
use std::{vec, array};

struct Pion {
    index : u8,
    is_coming_back : bool,
    position : u8,
    skipligne : array,
    skipcol : array
}
struct Board {
    board : vec,
    game_in_progress : bool,
}
impl Board {
    fn init(&mut self) {
        self.board = vec![
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
            self.game_in_progress = true;

    }
    fn place_pion(&mut self, pion: Pion) {
        if pion.is_upward {

        }
        else {
            let forward : &str = "□□ ○";
            let backward: &str = "○ □□";
            let interval = 6;
            let positions_forward = [4, 7, 13, 19, 25, 31, 34];
            let positions_backward = [33, 30, 24, 18, 12, 6, 4];
            let position: usize= pion.position.clone().into();
            if position <= 5 {
                // l'index de pion commence a 1 et termine a 5
                self.board[pion.index * 3][positions_forward[position]..positions_forward[position] + forward.len()+1] = forward;
            }
            else {
                self.board[pion.index * 3][positions_backward[position]..positions_backward[position] + backward.len()+1] = backward;
            }


        }
    }
    fn print_board(self) {
        for line in self.board{
            println!("{}",line);
        }
    }
}


impl Pion {
    fn bouge(&mut self, position: u8) {
        self.position = position;
    }
    fn position(self) -> u8 {
        self.position
    }
}


let mut b = Board {
    board : vec![
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
"       : | : . | . : | : . | . : | .     "],
is_upward: true
};
