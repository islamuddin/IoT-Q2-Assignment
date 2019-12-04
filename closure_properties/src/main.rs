fn main (){
    solution1();
    solution2();
    solution3();
    solution4();
    solution5();
}
fn solution1(){
    println!("====-= [ solution1() ] =====");
    let x = || {"Hello World"}; //make a closure which takes no argument and prints hello world
    let y = x();
    println!("{}",y);
}
fn solution2(){
    println!("====-= [ solution2() ] =====");
    let num :u32 = 2;
    let x = |num :u32| { num+1 }; //Make a closure which takes one u32 data type as argument and returns with adding 1 to it.
    println!("The Function returns {}",x(num));
}
fn solution3() {
    println!("====-= [ solution3() ] =====");
    let c = 1;
    let x = |c|{ c+1 }; //Make a closure which captures value of variable "c" from environment and change the value of c with adding 1.
    let c = x(c);
    println!("The new value of c is: {}",c); // should print 2
}
//Write a function which accepts a closure, and in the funciton body, it calls the closure. The closure accepts no argument and returns nothing. What should the closure be about? Be creative!
fn solution4() {
    println!("====-= [ solution4() ] =====");
    let x = || {};
 let nothing = |x| {x};
 nothing(x);
}
//Write a function which expects a closure as an argument and in the funciton body, it calls the closure. The closure expects u32 argument and returns the u32 value. The closure adds 1 to the argument and returns it.
fn solution5() {
    println!("====-= [ solution5() ] =====");
let num = 2;
let x = |num :u32| -> u32 { num+1};
let y = x(num);
println!("{}",y);
}