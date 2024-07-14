use random::Source;
fn main() {
    let mut source: Xorshift128Plus = random::default(seed:42);
    let num: i32 = source.read::<i32>();
    println!("Random number: {}", num);
}