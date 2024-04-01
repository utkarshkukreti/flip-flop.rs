# flip_flop.rs

> This library implements the [flip-flop operator from Perl and Ruby](https://en.wikipedia.org/wiki/Flip-flop_(programming)) as a Rust macro.

## Changelog

* April 1, 2024: Version 1.0.0.

## Usage

The `flip_flop!` macro accepts two boolean expressions wrapped in parentheses and separated by either 2 dots (`..`) or 3 dots (`...`):

```rust,ignore
flip_flop!((x == 5)..(x == 10))
// or
flip_flop!((x == 5)...(x == 10))
```

The macro returns true from when the left expression is true until the right expression is true by maintaining a hidden bool state.

The difference between `..` and `...` is that with `...`, the right expression is also evaluated when the left expression first becomes true, which makes it possible for the flip-flop internal state to be reset in the same iteration. This behavior matches that of [Perl](https://perldoc.perl.org/perlop#Range-Operators).

The internal state of the operator is stored in a static `AtomicBool` and is therefore shared among threads, mirroring the behavior of Perl.

## Example

The print statement in the following code is executed from when `i == 5` becomes true until `i == 10` becomes true.

```rust
use flip_flop::flip_flop;

fn main() {
    for i in 0..20 {
        // Prints 5, 6, 7, 8, 9, 10.
        if flip_flop!((i == 5)..(i == 10)) {
            println!("{i}")
        }
    }
}
```

Output:

```text
5
6
7
8
9
10
```

## Extracting Sequence Example

The following code extracts all sequences starting with `1` followed by any numbers until finding a `2`.

```rust
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
```

Output:

```text
Original: [0, 1, 2, 0, 0, 1, 0, 2, 0, 0, 0, 0, 2, 2, 2, 0, 1, 1, 2, 0]
Filtered: [1, 2, 1, 0, 2, 1, 1, 2]
```

## FizzBuzz Example

This is the classic FizzBuzz problem implemented using the flip-flop operator. [Source](https://juliansimioni.com/blog/deconstructing-fizz-buzz-with-flip-flops-in-ruby/).

```rust
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
```

Output (after removing text hidden by `\r`):

```text
1
2
Fizz
4
Buzz
Fizz
7
8
Fizz
Buzz
11
Fizz
13
14
FizzBuzz
16
…
```

## Bugs

The behavior of this crate should match that of Perl and Ruby, but there may be edge cases that are not handled correctly. If you find any, please open an issue (or a pull request)!

## License

This project is licensed under either of

 * Apache License, Version 2.0, (https://www.apache.org/licenses/LICENSE-2.0)
 * MIT license (https://opensource.org/licenses/MIT)

at your option.
