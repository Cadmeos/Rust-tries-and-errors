pub fn is_empty(a: i32) -> bool {
    match a {
        0..=9 => false,
        _ => true
    }
}

fn main () {
    let mut vec = Vec::new();

    vec.push(1);
    vec.push(2);

    println!("{:?}, {}", vec, vec.is_empty());
}