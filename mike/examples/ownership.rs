fn main() {
    // borrow
    println!("***** borrow *****");
    let s1 = String::from("I am a borrow.");
    let s2 = s1; // now s1 is invalid(无效状态)
                 // println!("{s1}"); // error: value borrowed here after move
    println!("{s2}");

    // clone
    println!("\n***** clone *****");
    let s1 = String::from("I am a clone.");
    let s2 = s1.clone(); // if the performance cost is accetable, use clone
    println!("{s1}");
    println!("{s2}");

    // ownership
    println!("\n***** ownership *****");

    fn foo1(s: String) {
        println!("{s}");
    }

    let s1 = String::from("I am foo1.");
    foo1(s1); // 此时 s1 无效
              // println!("{s1}"); // error: value borrowed here after move

    fn foo2(s: String) -> String {
        println!("{s}");
        s // return s
    }
    let s1 = String::from("I am foo2.");
    let s1 = foo2(s1); // 通过返回值重新获取所有权
    println!("{s1}");

    // 思考题1
    println!("\n***** 思考题1 *****");
    // let s = "I am a superman.".to_string();
    // for _ in 1..10 {
    //     let tmp_s = s; // 在上一次循环中，s 已经无效了
    //     println!("s is {}", tmp_s);
    // }

    // 思考题2
    println!("\n***** 思考题2 *****");
    # [derive(Debug)]
    struct Point {
        _x: i64,
        _y: i64,
        _z: i64,
    }
    let a = Point { _x: 1, _y: 2, _z: 3 };
    let b = a; // a is moved to b
    println!("b is {:?}", b); // error: value borrowed here after move
}
