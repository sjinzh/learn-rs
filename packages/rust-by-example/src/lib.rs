mod ch01_hello;
mod ch02_primitives; // 必须导入, 否则内部的 #[test] 标记, 无法识别

/*

参考:
    https://rustwiki.org/zh-CN/rust-by-example/index.html

说明:
    1. 根据例子学习Rust语言
    2. 借助 IDE 可以快速执行 test 方法的技巧, 快速学习.

 */

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }
}
