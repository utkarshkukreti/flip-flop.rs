use flip_flop::flip_flop;

fn main() {
    for i in 0..20 {
        // Prints 5, 6, 7, 8, 9, 10.
        if flip_flop!((i == 5)..(i == 10)) {
            println!("{i}")
        }
    }
}
