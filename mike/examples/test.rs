fn foo() -> u32 {
    10u32
}

#[cfg(test)] // 这里配置测试模块
mod tests {
    use crate::foo;

    #[test] // 具体的单元测试用例
    fn it_works() {
        let result = foo(); // 调用被测试的函数或功能
        assert_eq!(result, 10u32); // 断言
    }
}

fn main() {
    println!("{}", foo());
}
