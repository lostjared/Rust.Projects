use tictactoe::tictactoe::Grid;

fn get_move(player: u32) -> (usize, usize, u32) {
    println!("Player {} Enter x y ", player);
    let mut input_text : String = String::new();
    std::io::stdin().read_line(&mut input_text).expect("on readline");
    input_text.pop();
    if !input_text.contains(" ") {
        println!("Needs to contain two numbers");
        return get_move(player);
    }
    let value : Vec<&str> = input_text.split(" ").collect();
    let x : usize = value[0].parse().unwrap();
    let y : usize = value[1].parse().unwrap();

    if !(x <= 2 && y <= 2) {
        println!("must be value 0-2");
        return get_move(player);
    }

    (x, y, player)
}

fn main() {
    let mut g : Grid = Grid::new();
    g.print_board();
    loop  {
        let rt_val = get_move(g.turn);
        if g.check_turn(rt_val.0, rt_val.1) == true {
            g.go_turn(rt_val.0, rt_val.1, rt_val.2);
        } else {
            println!("Space occupied...\n");
            continue;
        }
        g.print_board();
        if g.check_game_over() == -1 {
            g.switch_turn();        
        } else {
            println!("Player {} Won!\n", g.check_game_over());
            g.print_board();
            break;
        }          
    }
}