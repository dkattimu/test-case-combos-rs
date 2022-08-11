fn main() {
    println!("Hello, world!");
    let mut v = vec![1, 2, 3, 5];
    for i in &mut v{
        *i *= 5;
        println!("{}", i)
    }
}
