#![no_std]

use bitset_const::bitset;

bitset!(struct Bits:256);

#[test]
fn starts_zeroed() {
    let bits = Bits::new();
    for byte in bits.0 {
        assert_eq!(byte, 0);
    }
}

#[test]
fn set_and_get() {
    let mut bits = Bits::new();
    bits.set(124, true);
    assert!(bits.get(124));
}

#[test]
fn bitwise_and() {
    let mut a = Bits::new();
    a.set(1, true);
    let mut b = Bits::new();
    b.set(102, true);
    let a = a & b;
    assert!(!a.get(1));
    assert!(!a.get(102));
}

#[test]
fn bitwise_or() {
    let mut a = Bits::new();
    a.set(1, true);
    let mut b = Bits::new();
    b.set(102, true);
    let a = a | b;
    assert!(a.get(1));
    assert!(a.get(102));
}

#[test]
fn count() {
    let mut a = Bits::new();
    a.set(10, true);
    a.set(20, true);
    a.set(30, true);
    assert_eq!(a.count(), 3)
}
