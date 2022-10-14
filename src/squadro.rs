// Change since last commit :

// Changed is_upward to is_coming_back
use std::{vec, array};

struct Board {
    game_in_progress : bool,
    upward_position : Vec<i32>,
    sideward_position : Vec<i32> 
}
struct Pion {
    index : u8,
    is_coming_back : bool,
    position : u8,
    skipligne : array,
    skipcol : array
}
struct Board {
    board : vec
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

fn place_un_charactere(&mut tableau: vec, icone : String, hor: u8, vert: u8){
    tableau[(vert*41) + hor] = icone;
}

fn place_un_pion_h(){
    let chaine1= String::from("□□ ○");
    let chaine2=String::from("○ □□");
}

fn place_un_pion_v(){}

fn formatter_les_parties(){}