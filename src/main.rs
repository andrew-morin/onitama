use rand::seq::IteratorRandom;
use strum::IntoEnumIterator;
use strum_macros::EnumIter;

#[derive(EnumIter, Clone, Copy, Debug, PartialEq)]
enum Card {
    Tiger,
    Crab,
    Monkey,
    Crane,
    Dragon,
    Elephant,
    Mantis,
    Boar,
    Frog,
    Goose,
    Horse,
    Eel,
    Rabbit,
    Rooster,
    Ox,
    Cobra,
}

#[derive(Debug)]
enum Move {
    Left,
    Up,
    Right,
    Down,

    UpLeft,
    UpRight,
    DownLeft,
    DownRight,

    DragonLeft,
    DragonRight,

    LeftTwo,
    UpTwo,
    RightTwo,
}

#[derive(Clone)]
enum Color {
    Blue,
    Red,
}

#[derive(Clone)]
enum PawnType {
    Master,
    Student,
}

#[derive(Clone)]
struct Pawn {
    color: Color,
    pawn_type: PawnType,
}

#[derive(Clone)]
enum Square {
    Pawn(Pawn),
    Empty,
}

impl Card {
    fn get_movement(&self) -> &[Move] {
        match self {
            Card::Tiger => &[Move::UpTwo, Move::Down],
            Card::Crab => &[Move::Up, Move::LeftTwo, Move::RightTwo],
            Card::Monkey => &[Move::UpLeft, Move::UpRight, Move::DownLeft, Move::DownRight],
            Card::Crane => &[Move::Up, Move::DownLeft, Move::DownRight],
            Card::Dragon => &[
                Move::DragonLeft,
                Move::DragonRight,
                Move::DownLeft,
                Move::DownRight,
            ],
            Card::Elephant => &[Move::UpLeft, Move::UpRight, Move::Left, Move::Right],
            Card::Mantis => &[Move::UpLeft, Move::UpRight, Move::Down],
            Card::Boar => &[Move::Up, Move::Left, Move::Right],
            Card::Frog => &[Move::UpLeft, Move::LeftTwo, Move::DownRight],
            Card::Goose => &[Move::UpLeft, Move::Left, Move::Right, Move::DownRight],
            Card::Horse => &[Move::Up, Move::Left, Move::Down],
            Card::Eel => &[Move::UpLeft, Move::Right, Move::DownLeft],
            Card::Rabbit => &[Move::UpRight, Move::RightTwo, Move::DownLeft],
            Card::Rooster => &[Move::UpRight, Move::Left, Move::Right, Move::DownLeft],
            Card::Ox => &[Move::Up, Move::Right, Move::Down],
            Card::Cobra => &[Move::UpRight, Move::Left, Move::DownRight],
        }
    }
}

fn deal_five_cards() -> Vec<Card> {
    let mut rng = rand::thread_rng();
    Card::iter().choose_multiple(&mut rng, 5)
}

fn generate_start_board() -> [[Square; 5]; 5] {
    let blue_master = Square::Pawn(Pawn {
        color: Color::Blue,
        pawn_type: PawnType::Master,
    });
    let blue_student = Square::Pawn(Pawn {
        color: Color::Blue,
        pawn_type: PawnType::Student,
    });
    let red_master = Square::Pawn(Pawn {
        color: Color::Red,
        pawn_type: PawnType::Master,
    });
    let red_student = Square::Pawn(Pawn {
        color: Color::Red,
        pawn_type: PawnType::Student,
    });

    [
        [
            blue_student.clone(),
            blue_student.clone(),
            blue_master,
            blue_student.clone(),
            blue_student,
        ],
        [
            Square::Empty,
            Square::Empty,
            Square::Empty,
            Square::Empty,
            Square::Empty,
        ],
        [
            Square::Empty,
            Square::Empty,
            Square::Empty,
            Square::Empty,
            Square::Empty,
        ],
        [
            Square::Empty,
            Square::Empty,
            Square::Empty,
            Square::Empty,
            Square::Empty,
        ],
        [
            red_student.clone(),
            red_student.clone(),
            red_master,
            red_student.clone(),
            red_student,
        ],
    ]
}

fn main() {
    let cards = deal_five_cards();
    println!("Cards: {:#?}", cards);
    for card in cards {
        println!("Moves for {:?} are: {:?}", card, card.get_movement());
    }
    let board: [[Square; 5]; 5] = generate_start_board();
    for row in board {
        for square in row {
            if let Square::Pawn(pawn) = square {
                let color_str = match pawn.color {
                    Color::Blue => "b",
                    Color::Red => "r",
                };
                let type_str = match pawn.pawn_type {
                    PawnType::Student => "s",
                    PawnType::Master => "M",
                };
                print!(" {}{}", color_str, type_str)
            } else {
                print!(" []");
            }
        }
        println!()
    }
}
