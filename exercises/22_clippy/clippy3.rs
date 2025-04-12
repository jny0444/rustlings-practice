fn main() {
    let my_option: Option<()> = None;
    if let Some(val) = my_option {
        println!("{val:?}");
    }

    let my_arr = &[
        -1, -2, -3,
        -4, -5, -6,
    ];
    println!("My array! Here it is: {:?}", my_arr);

    let mut my_empty_vec = vec![1, 2, 3, 4, 5];
    my_empty_vec.clear(); // ✅ Use clear() instead of resize(0, _)
    println!("This Vec is empty, see? {:?}", my_empty_vec);

    let mut value_a = 45;
    let mut value_b = 66;
    std::mem::swap(&mut value_a, &mut value_b); // ✅ Correct swap
    println!("value a: {}; value b: {}", value_a, value_b);
}
