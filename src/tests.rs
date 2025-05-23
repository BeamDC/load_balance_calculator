use crate::helpers::{gcd, gcd_vec};

#[test]
fn test_gcd() {
    let expected = 10;
    let gcd = gcd(270, 260);
    assert_eq!(expected, gcd);
}

#[test]
fn test_gcd_vec() {
    let expected = 10;
    let gcd = gcd_vec(vec![270, 270, 260]);
    assert_eq!(expected, gcd);
}