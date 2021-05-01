use crate::HexIterator;

#[test]
fn hexed_iterates_over_an_array() {
    let data = [0xbe, 0xef];
    let it = data.iter().copied().hexed();

    let expected = "beef".chars();
    itertools::assert_equal(expected, it);
}

#[test]
fn hexed_reverse_iterates_over_an_array() {
    let data = [0xbe, 0xef];
    let it = data.iter().copied().hexed().rev();

    let expected = "feeb".chars();
    itertools::assert_equal(expected, it);
}

#[test]
fn hexed_transforms_ranges() {
    let range = 10..16;
    let it = range.hexed();

    let expected = "0a0b0c0d0e0f".chars();
    itertools::assert_equal(expected, it);
}

#[test]
fn hexed_works_for_converted_strings() {
    let data = b"beef";
    let it = data.iter().copied().hexed();

    let expected = "62656566".chars();
    itertools::assert_equal(expected, it);
}

#[test]
fn hexed_can_be_consumed_from_both_ends() {
    let data = [0xde, 0xad, 0xbe, 0xef];
    let mut it = data.iter().copied().hexed();

    let front = it.by_ref().take(3);
    let expected = "dea".chars();
    itertools::assert_equal(expected, front);

    let back = it.rev();
    let expected_back = "feebd".chars();
    itertools::assert_equal(expected_back, back);
}
