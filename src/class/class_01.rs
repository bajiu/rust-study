// 计算两个整数的最大公约数
// mut 表示在函数体内可以为 m, n 赋值
fn gcd(mut n: u64, mut m: u64) -> u64 {
    // assert! 宏 验证参数不为 0 , ! 表示为一个宏调用而不是函数调用
    assert!(n != 0 && m != 0);
    while m != 0 {
        if m < n {
            let t: u64 = m;
            m = n;
            n = t;
        }
        m = m % n;
    }
    n
}


// Rust 语言本身内置了简单的测试机制
// #[test] 表示 test_gcd 是一个测试函数, 常规编译中会跳过, 但在 cargo test 时会被包含并自动调用
#[test]
fn test_gcd() {
    assert_eq!(gcd(14, 15), 1);
    assert_eq!(gcd(2 * 3 * 5 * 11 * 17,
                   3 * 7 * 11 * 13 * 19), 3 * 11
    )
}