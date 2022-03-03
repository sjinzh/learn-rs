#[test]
fn ex04_lifetime() {
    /*
    TODO X:
        生命周期（lifetime）是这样一种概念，编译器（中的借用检查器）用它来保证所有的 借用都是有效的。
        确切地说，一个变量的生命周期在它创建的时候开始，在它销毁的时候 结束。
        虽然生命周期和作用域经常被一起提到，但它们并不相同。
    */

    let i = 3;
    {
        let borrow1 = &i;
        println!("borrow1: {}", borrow1);
    }

    {
        let borrow2 = &i;
        println!("borrow2: {}", borrow2);
    }

    println!("i: {}", i);
}

#[test]
fn ex04_01_lifetime_explicit() {
    // TODO X: 显式标注生命周期

    fn print_refs<'a, 'b>(x: &'a i32, y: &'b i32) {
        println!("x is {}, y is {}", x, y);
    }

    fn failed_borrow<'a>() {
        let _x = 12;

        // error[E0597]:
        // let y: &'a i32 = &_x;
    }

    ////////////////////////////////////////////////////////////////////////////////

    let (four, nine) = (4, 9);

    // 引用传参
    print_refs(&four, &nine);

    failed_borrow();
}

#[test]
fn ex04_02_lifetime_fn() {
    ///
    /// todo x:
    ///     - 传参: 只读引用
    fn print_one<'a>(x: &'a i32) {
        println!("print_one: x = {}", x);
    }

    ///
    /// TODO X:
    ///     - 传数: 可变引用
    fn add_one<'a>(x: &'a mut i32) {
        // 类似 C 指针, 取值
        *x += 1;
    }

    fn print_multi<'a, 'b>(x: &'a i32, y: &'b i32) {
        println!("print_multi: x = {}, y = {}", x, y);
    }

    fn pass_x<'a, 'b>(x: &'a i32, _: &'b i32) -> &'a i32 {
        x
    }

    ////////////////////////////////////////////////////////////////////////////////

    let x = 7;
    let y = 9;

    print_one(&x);
    print_multi(&x, &y);

    let z = pass_x(&x, &y);
    print_one(z);

    // 值
    let mut t = 3;

    // todo x: 改变值: 传入的是可变引用
    add_one(&mut t);
    print_one(&t);
}
