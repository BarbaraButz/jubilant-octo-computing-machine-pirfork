use Vector2;

#[test]
fn vector2_additiontest() {
    let a = Vector2::new(5, 7);
    let b = Vector2::new(6, 8);
    let c = Vector2::new(11, 15);
    assert_eq!((a + b), c);

}

#[test]
fn vector2_multiplicationtest() {
    let e = Vector2::new(8, 6);
    let f = Vector2::new(4, 3);
    assert_eq!((f * 2), e);
}

#[test]
fn origin_test() {
    let o = Vector2::new(0, 0);
    assert_eq!(o, Vector2::origin());
}

#[test]
fn unitvec_test() {
    let x = Vector2::new(1, 0);
    let y = Vector2::new(0, 1);
    assert_eq!(x, Vector2::unit_x());
    assert_eq!(y, Vector2::unit_y());
}
