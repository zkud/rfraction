use rfraction::Fraction;

#[test]
fn with_usual_nums_add_works() {
    let first = Fraction::new(10, 140, false);
    let second = Fraction::new(15, 280, false);

    let result = first.add(&second);

    assert!(result.is_positive());
    assert_eq!(result.numerator(), 1);
    assert_eq!(result.denominator(), 8);
}
