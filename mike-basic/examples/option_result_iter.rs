fn unpack() {
    println!("***** unpack *****");
    // Option
    println!("expect()");
    let x: Option<&str> = Some("value");
    assert_eq!(x.expect("fruits are healthy"), "value");
    // Result
    // let path = std::env::var("IMPORTANT_PATH")
    //     .expect("env variable `IMPORTANT_PATH` should be set by `wrapper_script.sh`");
    // println!("path = {}", path);

    println!("unwrap()");
    // Option
    let x = Some("air");
    assert_eq!(x.unwrap(), "air");
    // Result
    let x: Result<u32, &str> = Ok(2);
    assert_eq!(x.unwrap(), 2);

    println!("unwarp_or()");
    // Option
    assert_eq!(Some("car").unwrap_or("bike"), "car");
    assert_eq!(None.unwrap_or("bike"), "bike");

    // Result
    let default = 2;
    let x: Result<u32, &str> = Ok(9);
    assert_eq!(x.unwrap_or(default), 9);

    let x: Result<u32, &str> = Err("error");
    assert_eq!(x.unwrap_or(default), default);

    println!("unwarp_or_default()");
    // Option
    let x: Option<u32> = None;
    let y: Option<u32> = Some(12);

    assert_eq!(x.unwrap_or_default(), 0);
    assert_eq!(y.unwrap_or_default(), 12);

    // Result
    let good_year_from_input = "1909";
    let bad_year_from_input = "190blarg";
    let good_year = good_year_from_input.parse().unwrap_or_default();
    let bad_year = bad_year_from_input.parse().unwrap_or_default();

    assert_eq!(1909, good_year);
    assert_eq!(0, bad_year);
}

// 不解包情况下的操作
fn option_no_unpack() {
    println!("\n***** no_unpack *****");
    println!("map()");
    let maybe_some_string = Some(String::from("Hello, World!"));
    let maybe_some_len = maybe_some_string.map(|s| s.len());
    assert_eq!(maybe_some_len, Some(13));
    let x: Option<&str> = None;
    assert_eq!(x.map(|s| s.len()), None);

    println!("cloned()");
    let x = 12;
    let opt_x = Some(&x);
    assert_eq!(opt_x, Some(&12));
    let cloned = opt_x.cloned();
    assert_eq!(cloned, Some(12));

    println!("is_some()");
    let x: Option<u32> = Some(2);
    assert_eq!(x.is_some(), true);
    let x: Option<u32> = None;
    assert_eq!(x.is_some(), false);

    println!("is_none()");
    let x: Option<u32> = Some(2);
    assert_eq!(x.is_some(), true);
    let x: Option<u32> = None;
    assert_eq!(x.is_some(), false);

    println!("as_ref()");
    let text: Option<String> = Some("Hello, world!".to_string());
    let text_length: Option<usize> = text.as_ref().map(|s| s.len());
    println!("still can print text: {text:?}");
    println!("text_length is {text_length:?}");

    println!("as_mut()");
    let mut x = Some(2);
    match x.as_mut() {
        Some(v) => *v = 42,
        None => {}
    }
    assert_eq!(x, Some(42));

    println!("take()");
    let mut x = Some(2);
    let y = x.take();
    assert_eq!(x, None);
    assert_eq!(y, Some(2));

    let mut x: Option<u32> = None;
    let y = x.take();
    assert_eq!(x, None);
    assert_eq!(y, None);

    println!("replace()");
    let mut x = Some(2);
    let old = x.replace(5);
    assert_eq!(x, Some(5));
    assert_eq!(old, Some(2));

    let mut x = None;
    let old = x.replace(3);
    assert_eq!(x, Some(3));
    assert_eq!(old, None);

    println!("and_then()");
    let mut x = Some(2);
    let old = x.replace(5);
    assert_eq!(x, Some(5));
    assert_eq!(old, Some(2));

    let mut x = None;
    let old = x.replace(3);
    assert_eq!(x, Some(3));
    assert_eq!(old, None);
}

fn result_no_unpack() {
    println!("\n***** no_unpack *****");
    println!("map()");
    let line = "1\n2\n3\n4\n";
    for num in line.lines() {
        match num.parse::<i32>().map(|i| i * 2) {
            Ok(n) => println!("{n}"),
            Err(..) => {}
        }
    }

    println!("is_ok()");
    let x: Result<i32, &str> = Ok(-3);
    assert_eq!(x.is_ok(), true);

    let x: Result<i32, &str> = Err("Some error message");
    assert_eq!(x.is_ok(), false);

    println!("is_err()");
    let x: Result<i32, &str> = Ok(-3);
    assert_eq!(x.is_err(), false);

    let x: Result<i32, &str> = Err("Some error message");
    assert_eq!(x.is_err(), true);

    println!("as_ref()");
    let x: Result<u32, &str> = Ok(2);
    assert_eq!(x.as_ref(), Ok(&2));

    let x: Result<u32, &str> = Err("Error");
    assert_eq!(x.as_ref(), Err(&"Error"));

    println!("as_mut()");
    fn mutate(r: &mut Result<i32, i32>) {
        match r.as_mut() {
            Ok(v) => *v = 42,
            Err(e) => *e = 0,
        }
    }
    let mut x: Result<i32, i32> = Ok(2);
    mutate(&mut x);
    assert_eq!(x.unwrap(), 42);
    let mut x: Result<i32, i32> = Err(13);
    mutate(&mut x);
    assert_eq!(x.unwrap_err(), 0);

    println!("and_then()");
    fn sq_then_to_string(x: u32) -> Result<String, &'static str> {
        x.checked_mul(x)
            .map(|sq| sq.to_string())
            .ok_or("overflowed")
    }

    assert_eq!(Ok(2).and_then(sq_then_to_string), Ok(4.to_string()));
    assert_eq!(Ok(1_000_000).and_then(sq_then_to_string), Err("overflowed"));
    assert_eq!(
        Err("not a number").and_then(sq_then_to_string),
        Err("not a number")
    );

    println!("map_err()");
    fn stringify(x: u32) -> String {
        format!("error code: {x}")
    }
    let x: Result<u32, u32> = Ok(2);
    assert_eq!(x.map_err(stringify), Ok(2));
    let x: Result<u32, u32> = Err(13);
    assert_eq!(x.map_err(stringify), Err("error code: 13".to_string()));
}

// Option<T> 与 Result<T,E> 相互转换
fn option_result() {
    println!("\n***** option_result 互相转换 *****");

    // ok_or(): 从 Option<T> 到 Result<T,E>
    println!("ok_or()");
    let x = Some("foo");
    assert_eq!(x.ok_or(0), Ok("foo"));

    let x: Option<&str> = None;
    assert_eq!(x.ok_or(0), Err(0));

    // ok(): 从 Result<T,E> 到 Option<T>
    println!("ok()");
    let x: Result<u32, &str> = Ok(2);
    assert_eq!(x.ok(), Some(2));

    let x: Result<u32, &str> = Err("Nothing here");
    assert_eq!(x.ok(), None);

    // err(): 从 Result<T,E> 到 Option<E>
    println!("err()");
    let x: Result<u32, &str> = Ok(2);
    assert_eq!(x.err(), None);

    let x: Result<u32, &str> = Err("Nothing here");
    assert_eq!(x.err(), Some("Nothing here"));
}

// 三种迭代器
fn iter3() {
    println!("\n***** iter3 *****");
    // next() 方法
    println!("next()");
    let a: Vec<u32> = vec![1, 2, 3, 4, 5];
    let mut an_iter = a.into_iter(); // 将Vec<u32>转换为迭代器

    while let Some(i) = an_iter.next() {
        // 调用 .next() 方法
        println!("{i}");
    }

    println!("三种迭代器: iter(), iter_mut(), into_iter()");
    let mut a = [1, 2, 3]; // 一个整数数组

    let mut an_iter = a.iter(); // 转换成第一种迭代器

    assert_eq!(Some(&1), an_iter.next());
    assert_eq!(Some(&2), an_iter.next());
    assert_eq!(Some(&3), an_iter.next());
    assert_eq!(None, an_iter.next());

    let mut an_iter = a.iter_mut(); // 转换成第二种迭代器

    assert_eq!(Some(&mut 1), an_iter.next());
    assert_eq!(Some(&mut 2), an_iter.next());
    assert_eq!(Some(&mut 3), an_iter.next());
    assert_eq!(None, an_iter.next());

    let mut an_iter = a.into_iter(); // 转换成第三种迭代器，并消耗掉a

    assert_eq!(Some(1), an_iter.next());
    assert_eq!(Some(2), an_iter.next());
    assert_eq!(Some(3), an_iter.next());
    assert_eq!(None, an_iter.next());

    println!("{:?}", a); // 由于是固定大小的数组，因此默认是 copy 一份，into_iter() 会消耗掉 an_iter，但是 a 本身不会变
}

// for 语句真面目
fn for_reveal() {
    println!("\n***** for_reveal *****");
    let c = vec![1, 2, 3];
    // for item in c {}
    // 相当于下面的语句，for 实际上是语法糖
    let mut tmp_iter = c.into_iter();
    while let Some(_item) = tmp_iter.next() {}

    // for 语句作为一种基础语法，它会消耗掉原集合。有时候希望不获取原集合元素所有权，
    // 比如只是打印一下，这时只需要获取集合元素的引用 ，应该怎么办呢？
    let mut a = ["1".to_string(), "2".to_string(), "3".to_string()];

    for item in &a {
        println!("{}", item);
    }

    for item in &mut a {
        println!("{}", item);
    }

    for item in a {
        // 请想一想为什么要把这一句放在后面
        println!("{}", item);
    }

    // println!("{:?}", a);  // String 类型是不可复制的，所以 a 会被移动
}

// 获取集合中元素的所有权
fn get_ownership_from_set() {
    let s1 = String::from("aaa");
    let s2 = String::from("bbb");
    let s3 = String::from("ccc");
    let s4 = String::from("ddd");

    let v = vec![s1, s2, s3, s4];
    for s in v {
        // 这里，s拿到了集合元素的所有权
        println!("{}", s);
    }
}

fn main() {
    // Denifition of Option<T>
    // pub enum Option {
    //     None,
    //     Some(T),
    // }

    // Denifition of Result<T, E>
    // pub enum Result {
    //     Ok(T),
    //     Err(E),
    // }
    unpack();
    option_no_unpack();
    result_no_unpack();
    option_result();
    iter3();
    for_reveal();
    get_ownership_from_set();
}
