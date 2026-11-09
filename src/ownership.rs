//! Lesson 1 — ownership, borrowing and lifetimes.
//!
//! The three rules of ownership:
//!   1. every value has exactly one owner,
//!   2. there can only be one owner at a time,
//!   3. when the owner goes out of scope, the value is dropped.
//!
//! Borrowing lets you *use* a value without taking ownership:
//!   * `&T`     shared borrow   — as many at a time as you like,
//!   * `&mut T` exclusive borrow — one at a time, and never together with `&T`.

pub fn demo() {
    // --- move vs copy ----------------------------------------------------
    let s1 = String::from("Simorgh");
    let s2 = s1; // MOVED: s1 is dead now, only s2 owns the heap buffer.
    // println!("{s1}"); // <- error[E0382]: borrow of moved value: `s1`
    println!("1a. moved value: s2 = {s2}");

    let n1 = 42;
    let n2 = n1; // i32 is `Copy`, so this copies instead of moving.
    println!("1b. copied value: n1 = {n1}, n2 = {n2} (both usable)");

    // --- borrowing -------------------------------------------------------
    let mut name = String::from("Simorgh");
    let bytes = len(&name); // shared borrow, released at the end of the call
    println!("2a. {name} is {bytes} bytes long");
    shout(&mut name); // exclusive borrow, released at the end of the call
    println!("2b. after shout(): {name}");

    // --- slices: borrow part of a collection, no allocation ---------------
    let handle = "with.simorgh";
    println!("3. head = {:?}, tail = {:?}", &handle[..4], &handle[4..]);

    // --- lifetimes: the result borrows from its inputs ---------------------
    println!("4. longest = {:?}", longest("creator", "creator campaign"));
}

fn len(s: &str) -> usize {
    s.len()
}

fn shout(s: &mut String) {
    // `s` is a mutable reference, so the caller keeps ownership.
    *s = format!("{}!", s.to_uppercase());
}

/// The `'a` says: the returned slice lives no longer than the shorter input,
/// because it may point into either one.
pub fn longest<'a>(a: &'a str, b: &'a str) -> &'a str {
    if a.len() >= b.len() { a } else { b }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn borrowing_does_not_move() {
        let name = String::from("simorgh");
        assert_eq!(len(&name), 7);
        assert_eq!(name, "simorgh", "the caller still owns the String");
    }

    #[test]
    fn mutation_through_a_mutable_reference() {
        let mut name = String::from("simorgh");
        shout(&mut name);
        assert_eq!(name, "SIMORGH!");
    }

    #[test]
    fn longest_picks_the_longer_slice() {
        assert_eq!(longest("crew", "creator"), "creator");
        assert_eq!(longest("same", "size"), "same", "ties keep the first");
    }
}
