#[test]
fn vector2_additiontest() {
    use ::Vector2;
    let a = Vector2::new(5, 7);
    let b = Vector2::new(6, 8);
    let c = Vector2::new(11, 15);
    assert!((a + b) == c);

}

#[test]
fn vector2_multiplicationtest() {
    use ::Vector2;
    let d = 2;
    let e = Vector2::new(8, 6);
    let f = Vector2::new(4, 3);
    assert!((f * d) == e);
}

#[test]
fn origin_test() {
    use ::Vector2;
    let o = Vector2::new(0, 0);
    assert!(o == Vector2::origin());
}

#[test]
fn unitvec_test() {
    use ::Vector2;
    let x = Vector2::new(1, 0);
    let y = Vector2::new(0, 1);
    assert!(x == Vector2::unit_x());
    assert!(y == Vector2::unit_y());
}
