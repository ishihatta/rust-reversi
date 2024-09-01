mod ai;
mod banmen;

fn input_player_turn() -> u8 {
    println!("Please input your turn (1=O, 2=X): ");
    let mut text = String::new();
    std::io::stdin().read_line(&mut text).ok();
    match text.trim().parse::<u8>() {
        Ok(n) => n,
        Err(_) => 0
    }
}

fn input_player_action(banmen: &banmen::Banmen, turn: u8) -> Option<(usize, usize)> {
    println!("Please input your action (XY or just enter to PASS)> ");
    loop {
        let mut text = String::new();
        std::io::stdin().read_line(&mut text).ok();
        if text.len() == 0 {
            return None;
        }
        match text.trim().parse::<usize>() {
            Ok(number) => {
                let x = number / 10;
                let y = number % 10;
                if x < banmen::BANMEN_SIZE && y < banmen::BANMEN_SIZE {
                    if let Some(_) = banmen.put_piece(x, y, turn) {
                        return Some((x, y));
                    } else {
                        println!("You cannot put there");
                    }
                } else {
                    println!("Invalid input")
                }
            },
            Err(_) => println!("Invalid input")
        }
    }
}

fn main() {
    let player_turn = input_player_turn();

    let mut banmen = banmen::Banmen::new();
    banmen.print();

    let mut ban = 1;
    loop {
        let place_opt = if ban == player_turn {
            input_player_action(&banmen, ban)
        } else {
            println!("Computer thinking...");
            ai::calc_next_place(&banmen, ban)
        };
        match ban {
            1 => print!("O> "),
            _ => print!("X> ")
        }
        match place_opt {
            Some(place) => {
                println!("PUT {},{}", place.0, place.1);
                banmen = banmen.put_piece(place.0, place.1, ban).unwrap();
            },
            None => println!("PASS")
        }
        banmen.print();
        if banmen.remain == 0 {
            break;
        }
        println!();
        ban = 3 - ban;
    }
}
