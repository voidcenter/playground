// primitive_types4.rs
// Get a slice out of Array a where the ??? is so that the test passes.
// Execute `rustlings hint primitive_types4` or use the `hint` watch subcommand for a hint.


fn t(x: String) {

}

#[test]
fn slice_out_of_array() {
    let a = [1, 2, 3, 4, 5];

    let nice_slice = &a[1 .. 4];

    let mut x = String::from('a');
    x.push_str("a");
    // let y = x;
    // t(x);
    println!("{}", x);


    assert_eq!([2, 3, 4], nice_slice)
}
