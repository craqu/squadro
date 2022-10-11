use std::{vec, array};

fn main() {

}
struct Board {
    in_progress : bool,
    upward_position : Vec<i32>,
    sideward_position : Vec<i32> 
}
struct Pion {
    index : u8,
    is_upward : bool,
    position : u8
}

fn print_board(upward_position : [Pion ; 5], sideward_position : [Pion ; 5]) {
    let board = vec![
            "       . | . : | : : | : : | : . | .     \n",
            "         |   . | .   |   . | .   |       \n",
            "  ...    |     |     |     |     |      .\n",
            "1 ───────┼─────┼─────┼─────┼─────┼───────\n",
            "  ...    |     |     |     |     |      .\n",
            "  .      |     |     |     |     |    ...\n",
            "2 ───────┼─────┼─────┼─────┼─────┼───────\n",
            "  .      |     |     |     |     |    ...\n",
            "  ..     |     |     |     |     |     ..\n",
            "3 ───────┼─────┼─────┼─────┼─────┼───────\n",
            "  ..     |     |     |     |     |     ..\n",
            "  .      |     |     |     |     |    ...\n",
            "4 ───────┼─────┼─────┼─────┼─────┼───────\n",
            "  .      |     |     |     |     |    ...\n",
            "  ...    |     |     |     |     |      .\n",
            "5 ───────┼─────┼─────┼─────┼─────┼───────\n",
            "  ...    |     |     |     |     |      .\n",
            "       . | .   |     |     |   . | .     \n",
            "       : | : . | . : | : . | . : | .     \n"];
    enum Position{
        One,
        Two,
        Tree,
        Four,
        Five
    }

}