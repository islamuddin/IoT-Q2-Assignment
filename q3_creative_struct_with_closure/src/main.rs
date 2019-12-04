#[derive(Debug)]
struct Number {
    pub plus_one: fn(usize) -> usize,
}
impl Number {
    fn new(plus_one: fn(usize) -> usize) -> Self {
        Self { plus_one }
    }
}

fn main() {
    let mut number =42;
    println!("Number {:?}",number);
    // way 1
    let plus_one = Number { plus_one: |a| a + 1 };
    number= (plus_one.plus_one)(number);
    println!("Plus one {:?}",number);

    // way 2
    number =(Number::new(|a| a + 1).plus_one)(number);
    println!("Plus one {:?}",number);


}