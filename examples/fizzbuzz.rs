use flip_flop::flip_flop;

// https://juliansimioni.com/blog/deconstructing-fizz-buzz-with-flip-flops-in-ruby
fn main() {
    let (mut a, mut b, mut c) = (false, false, false);

    for i in 1..=100 {
        #[rustfmt::skip]
        println!(
            "{}\r{}{}",
            i,
            if flip_flop!(({a = !a; a})..({a = !a; a})) { "" } else { "Fizz" },
            if flip_flop!(({b = !b; b})...(!flip_flop!(({c = !c; c})..({c = !c; c})))) { "" } else { "Buzz" },
        );
    }
}
