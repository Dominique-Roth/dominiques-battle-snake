// Welcome to
// __________         __    __  .__                               __
// \______   \_____ _/  |__/  |_|  |   ____   ______ ____ _____  |  | __ ____
//  |    |  _/\__  \\   __\   __\  | _/ __ \ /  ___//    \\__  \ |  |/ // __ \
//  |    |   \ / __ \|  |  |  | |  |_\  ___/ \___ \|   |  \/ __ \|    <\  ___/
//  |________/(______/__|  |__| |____/\_____>______>___|__(______/__|__\\_____>
//
// This file can be a nice home for your Battlesnake logic and helper functions.
//
// To get you started we've included code to prevent your Battlesnake from moving backwards.
// For more info see docs.battlesnake.com

use std::arch::aarch64::vaba_s8;
use std::collections::HashMap;
use std::fmt::Error;

use log::info;
use serde_json::{json, Value};

use crate::{Battlesnake, Board, Game};

// info is called when you create your Battlesnake on play.battlesnake.com
// and controls your Battlesnake's appearance
// TIP: If you open your Battlesnake URL in a browser you should see this data
pub fn info() -> Value {
    info!("INFO");

    return json!({
        "apiversion": "1",
        "author": "MissDominique", // TODO: Your Battlesnake Username
        "color": "#888888", // TODO: Choose color
        "head": "default", // TODO: Choose head
        "tail": "default", // TODO: Choose tail
    });
}

// start is called when your Battlesnake begins a game
pub fn start(_game: &Game, _turn: &u32, _board: &Board, _you: &Battlesnake) {
    info!("GAME START");
}

// end is called when your Battlesnake finishes a game
pub fn end(_game: &Game, _turn: &u32, _board: &Board, _you: &Battlesnake) {
    info!("GAME OVER");
}

// move is called on every turn and returns your next move
// Valid moves are "up", "down", "left", or "right"
// See https://docs.battlesnake.com/api/example-move for available data
pub fn get_move(_game: &Game, turn: &u32, board: &Board, you: &Battlesnake) -> Value {
    let mut is_move_safe: HashMap<_, _> = vec![
        ("up", true),
        ("down", true),
        ("left", true),
        ("right", true),
    ]
        .into_iter()
        .collect();

    // We've included code to prevent your Battlesnake from moving backwards
    let my_head = &you.body[0]; // Coordinates of your head
    let my_neck = &you.body[1]; // Coordinates of your "neck"

    if my_neck.x < my_head.x { // Neck is left of head, don't move left
        is_move_safe.insert("left", false);
    } else if my_neck.x > my_head.x { // Neck is right of head, don't move right
        is_move_safe.insert("right", false);
    } else if my_neck.y < my_head.y { // Neck is below head, don't move down
        is_move_safe.insert("down", false);
    } else if my_neck.y > my_head.y { // Neck is above head, don't move up
        is_move_safe.insert("up", false);
    }

    // TODO: Step 1 - Prevent your Battlesnake from moving out of bounds
    let board_width = &board.width;
    let board_height = &board.height;
    info!("{} {}", you.body[0].x, board_width);
    info!("{} {}", you.body[0].y, board_height);
    if you.body[0].x == board_width - 1 {
        info!("No right");
        is_move_safe.insert("right", false);
    } else if you.body[0].x == 0 {
        info!("No left");
        is_move_safe.insert("left", false);
    }
    if you.body[0].y == board_height - 1 {
        info!("No up");
        is_move_safe.insert("up", false);
    } else if you.body[0].y == 0 {
        info!("No down");
        is_move_safe.insert("down", false);
    }

    // TODO: Step 2 - Prevent your Battlesnake from colliding with itself
    let my_body = &you.body;
    info!("{:?}", my_body);
    if *is_move_safe.get("right").unwrap() &&
        my_body.iter()
            .any(|body_part|
                body_part.x == my_body[0].x + 1 && body_part.y == my_body[0].y
            ) {
        info!("Colliding at right {:?}", my_body[0].x + 1);
        is_move_safe.insert("right", false);
    }
    if *is_move_safe.get("left").unwrap() &&
        my_body.iter()
            .any(|body_part| body_part.x == my_body[0].x - 1 && body_part.y == my_body[0].y
            ) {
        info!("Colliding at left {:?}", my_body[0].x - 1);
        is_move_safe.insert("left", false);
    }
    if *is_move_safe.get("up").unwrap() &&
        my_body.iter()
            .any(|body_part| body_part.y == my_body[0].y + 1 && body_part.x == my_body[0].x
            ) {
        info!("Colliding at up {:?}", my_body[0].y + 1);
        is_move_safe.insert("up", false);
    }
    if *is_move_safe.get("down").unwrap() &&
        my_body.iter()
            .any(|body_part| body_part.y == my_body[0].y - 1 && body_part.x == my_body[0].x
            ) {
        info!("Colliding at down {:?}", my_body[0].y - 1);
        is_move_safe.insert("down", false);
    }

    // TODO: Step 3 - Prevent your Battlesnake from colliding with other Battlesnakes
    let opponents = &board.snakes;
    for opponent in opponents {
        if *is_move_safe.get("right").unwrap() &&
            opponent.body.iter()
                .any(|opponent_body|
                    opponent_body.x == my_body[0].x + 1 && opponent_body.y == my_body[0].y
                ) {
            info!("Colliding at right {:?}", my_body[0].x + 1);
            is_move_safe.insert("right", false);
        }
        if *is_move_safe.get("left").unwrap() &&
            my_body.iter()
                .any(|opponent_body| opponent_body.x == my_body[0].x - 1 && opponent_body.y == my_body[0].y
                ) {
            info!("Colliding at left {:?}", my_body[0].x - 1);
            is_move_safe.insert("left", false);
        }
        if *is_move_safe.get("up").unwrap() &&
            my_body.iter()
                .any(|opponent_body| opponent_body.y == my_body[0].y + 1 && opponent_body.x == my_body[0].x
                ) {
            info!("Colliding at up {:?}", my_body[0].y + 1);
            is_move_safe.insert("up", false);
        }
        if *is_move_safe.get("down").unwrap() &&
            my_body.iter()
                .any(|opponent_body| opponent_body.y == my_body[0].y - 1 && opponent_body.x == my_body[0].x
                ) {
            info!("Colliding at down {:?}", my_body[0].y - 1);
            is_move_safe.insert("down", false);
        }
    }


    // Are there any safe moves left?
    info!("Options {:?}", is_move_safe);
    let safe_moves = is_move_safe
        .into_iter()
        .filter(|&(_, v)| v)
        .map(|(k, _)| k)
        .collect::<Vec<_>>();

    // Choose a random move from the safe ones
    // let chosen = safe_moves.choose(&mut rand::thread_rng()).unwrap();

    // TODO: Step 4 - Move towards food instead of random, to regain health and survive longer
    let food = &board.food;
    let chosen = get_closest_food(
        my_body.iter().nth(0).unwrap(),
        food,
        Direction::Up,
        &mut vec![],
        board_width,
        board_height
    );


    info!("MOVE {}: {}", turn, &chosen.clone().unwrap());
    return json!({ "move": chosen.unwrap() });
}

enum Direction {
    Up,
    Down,
    Left,
    Right
}

fn get_closest_food<'a>(
    current_coord: &'a crate::Coord,
    food: &Vec<crate::Coord>,
    last_direction: Direction,
    visited: &mut Vec<&'a crate::Coord>,
    board_width: &u32,
    board_height: &u32
) -> Result<String, Error> {
    if visited.iter().any(|x| x.x == current_coord.x && x.y == current_coord.y) { return Err(Error); }
    info!("Checking {:?}", current_coord);
    if food.iter().any(|food_coord|
        food_coord.x == current_coord.x+1 &&
            food_coord.y == current_coord.y &&
            &(current_coord.x + 1) < board_width
    ) {
        return Ok(String::from("right"));
    } else if food.iter().any(|food_coord|
        food_coord.x == current_coord.x-1 &&
            food_coord.y == current_coord.y &&
            current_coord.x-1 > 0u32
    ) {
        return Ok(String::from("left"));
    } else if food.iter().any(|food_coord|
        food_coord.y == current_coord.y+1 &&
            food_coord.x == current_coord.x &&
            &(current_coord.y+1) < board_height
    ) {
        return Ok(String::from("up"));
    } else if food.iter().any(|food_coord|
        food_coord.y == current_coord.y-1 &&
            food_coord.x == current_coord.x &&
            food_coord.y-1 > 0
    ) {
        return Ok(String::from("down"));
    }
    visited.append(&mut vec![current_coord]);
    match last_direction {
        Direction::Up => {
            get_closest_food(
                &crate::Coord {
                    x: &current_coord.x+1,
                    y: &current_coord.y,
                },
                food,
                Direction::Right,
                visited,
                board_width,
                board_height
            )
        }
        Direction::Down => {
            get_closest_food(
                &crate::Coord {
                    x: current_coord.x-1,
                    y: current_coord.y,
                },
                food,
                Direction::Left,
                visited,
                board_width,
                board_height
            )
        }
        Direction::Left => {
            get_closest_food(
                &crate::Coord {
                    x: current_coord.x,
                    y: current_coord.y-1,
                },
                food,
                Direction::Up,
                visited,
                board_width,
                board_height
            )
        }
        Direction::Right => {
            get_closest_food(
                &crate::Coord {
                    x: current_coord.x,
                    y: current_coord.y+1,
                },
                food,
                Direction::Down,
                visited,
                board_width,
                board_height
            )
        }
    }
}
