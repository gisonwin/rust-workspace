fn main() {
    /***
    变量的作用域,只在一个代码块中生存.代码块{}也允许变量遮蔽.
     */
    let spend = 1;
    {
        let target = "aaaa";
        println!("code block inner target == {}", target);
        let spend = 2.0;
        println!("code block inner spend == {}", spend);
    }
    //here target is out of code block scope. and panic("not found in the scope");
    // println!("out target =={}", target);
    println!("out spend == {}", spend);
    let spend = String::from("spend string");
    println!("reset value spend == {}", spend);

    let spend2;
    {
        let x = 2;
        spend2 = x * x;
    }
    println!("spend2:{}", spend2);

    let spend3;
    // println!("spend3:{}", spend3);//here panic!: spend3 used here but it is possibly-uninitialized.
    spend3 = 1;
    println!("another binding spend3:{}", spend3);

    //冻结  资源存在使用的引用 时,在当前作用域中这一资源是不可修改的.
    let mut spend4 = Box::new(4);
    let spend5 = &spend4;//此处借用了spend4
    // spend4 = Box::new(100);//所以此处 不允许 赋值.
    println!("{}", spend4);
    println!("{}", spend5);
}

