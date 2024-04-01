use flip_flop::flip_flop;

fn main() {
    let xs = [0, 1, 2, 0, 0, 1, 0, 2, 0, 0, 0, 0, 2, 2, 2, 0, 1, 1, 2, 0];

    let ys = xs
        .into_iter()
        .filter(|&x| flip_flop!((x == 1)..(x == 2)))
        .collect::<Vec<_>>();

    println!("Original: {:?}", xs);
    println!("Filtered: {:?}", ys);
}
