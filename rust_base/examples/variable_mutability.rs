fn main() {
    // let x = 5;  // 不可变
    let mut x = 5; // 可变
    println!("The value of x is: {x}");
    x = 6;
    println!("The value of x is: {x}");

    // const THREE_HOURS_IN_SECONDS: u32 = 60 * 60 * 3;

    // shadowing
    let y = 5;
    let y = y+1;
    {
        let y= x*2;
        println!("The value of y is: {y}");
    }
    println!("The value of y is: {y}");

    // difference between shadowing and mut
    // shadowing: &str to usize
    let _spaces = "   ";
    let _spaces = _spaces.len();

    // cannot shadow [mut] variable
    // let mut spaces = "   ";
    // spaces = spaces.len();
}