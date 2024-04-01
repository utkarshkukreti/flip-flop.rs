use flip_flop::flip_flop;

#[test]
fn test_flip_flop() {
    // See ./reference.rb.

    let (mut count_1, mut left_1, mut right_1) = (0, 0, 0);
    let (mut count_2, mut left_2, mut right_2) = (0, 0, 0);
    let (mut count_3, mut left_3, mut right_3) = (0, 0, 0);
    let (mut count_4, mut left_4, mut right_4) = (0, 0, 0);

    #[rustfmt::skip]
    for i in 0..=20 {
        if flip_flop!(({ left_1 += 1; i == 5 })..({ right_1 += 1; i == 10 })) {
            count_1 += 1;
        }

        if flip_flop!(({ left_2 += 1; i == 5 })...({ right_2 += 1; i == 10 })) {
            count_2 += 1;
        }

        if flip_flop!(({ left_3 += 1; i % 2 == 0 })..({ right_3 += 1; i % 4 == 0 })) {
            count_3 += 1
        }

        if flip_flop!(({ left_4 += 1; i % 2 == 0 })...({ right_4 += 1; i % 4 == 0 })) {
            count_4 += 1
        }
    };

    assert_eq!((count_1, left_1, right_1), (6, 16, 6));
    assert_eq!((count_2, left_2, right_2), (6, 16, 5));
    assert_eq!((count_3, left_3, right_3), (16, 11, 16));
    assert_eq!((count_4, left_4, right_4), (17, 9, 12));
}
