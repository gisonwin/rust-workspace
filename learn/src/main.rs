extern crate core;

use std::{fs, thread};
use std::fs::{File, OpenOptions};
use std::io::{Read, Write};
use std::ops::Deref;
use std::time::Duration;
use mylib::add_salary::study;

use std::convert::{From,Into};

fn data_type() {
    let food = "rice";
    let price = 4.0;
    let checked = true;
    println!("food is {},price is {},checked is {}", food, price, checked);
    let p1: f64 = 3.1415926_5;
    let p2 = 168;
    let p3: f32 = 1.7312;
    let p4 = 1_000_000;
    let p5 = 1_000_000.666_890;
    println!("p2 {},p3 {},p4 {},p5 {}", p2, p3, p4, p5);
    println!("p1 {}", p1);
}

fn io_exercise() {
    let mut in_word = String::new();
    let result = std::io::stdin().read_line(&mut in_word).unwrap();
    println!("You Input is :{},Read bytes is {}", in_word, result);
    let write_in = std::io::stdout().write("Gison Win is learning rust".as_bytes()).unwrap();
    println!("\n bytes =={}", write_in);
}

fn file_exercise() {
    //打开文件
    // let file = std::fs::File::open("data.txt");
    // println!("open file {:?}", file);
    //创建新文件
    // let file = std::fs::File::create("data2.txt").expect("create failure");
    // println!("file create {:?}", file);
    //删除文件
    // fs::remove_file("data.txt").expect("do not remove file data.txt");
    // println!("data.txt had removed");
    //打开文件并向文件写入内容
    // let mut file = OpenOptions::new().append(true).open("data2.txt").expect("open data2.txt failure");
    // file.write("www.google.com/ncr".as_bytes()).expect("write data to data2.txt failure");
    // file.write_all("\n\rRust".as_bytes()).expect("write_all data failure");
    // file.write_all("write_all并不会在在写入数据后自动换行".as_bytes()).expect("write_all data failure");
    // println!("data append success");
    let mut file = std::fs::File::open("data2.txt").unwrap();
    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap();
    println!("{}", contents);
}

fn iter_exercise() {
    /***
    iter()只返回一个只读可冲入迭代器,迭代器元素的类型为&T,
    into_iter()返回一个只读不可重入迭代器,迭代器元素的类型为T,
    iter_mut()返回一个可修改重入迭代器,迭代器元素类型为&mut T.
     */
    let v = vec!["Go lang", "Go microsoft", "21days learn Go Lang"];
    let mut iter = v.iter();
    println!("{:?}", iter.next());
    println!("{:?}", iter.next());
    println!("{:?}", iter.next());
    println!("{:?}", iter.next());
    println!("{:?}", iter.next());

    let it = v.iter();
    for item in it {
        println!("{}", item);
    }
}

fn closure_exercise() {
    /***
    调用一个闭包和调用一个函数完全相同,不同的是调用闭包时输入和返回类型两者都可以自动推导,而输入变量名必须指明.
    声明是使用||代替()将输入参数括起来
    函数体界定符{}对于单个表达式是可选的,其他情况必须加上,
    有能力捕获外部环境的变量.
    |参数列表|{
    业务逻辑
    }
    ||{
    业务逻辑
    }
    闭包可以赋值一个变量
     */
    let dual = |y: i64| { y * 2 };
    let add = |a, b| { a + b };
    let x = add(2, 4);
    println!("{:?}", x);
    let dual_val = dual(5);
    println!("{}", dual_val);
    let v = 3;
    let add2 = add(v, 5);
    println!("{:?}", add2);
    /*****
    闭包是在一个函数内创建立即调用的另外一个函数,
    闭包是一个匿名函数
    闭包虽然没有函数名,但可以把整个闭包赋值给一个变量,通过该变量来完成闭包的调用.
    闭包不用声明返回值,但可以有返回值,并且使用最后一条语句的执行结果作为返回值,闭包返回值也可以给变量
    闭包也被称为内联函数,可以让闭包访问外层函数里的变量.
     */
}

fn thread_exercise() {
    /***
    创建线程 std::thread::spawn();
    */
    // thread::spawn(|| {
    //     for i in 1..10 {
    //         println!("sub thread {}", i);
    //         thread::sleep(Duration::from_millis(1));
    //     }
    // });
    // for i in 1..5 {
    //     println!("main thread {}", i);
    //     thread::sleep(Duration::from_millis(1));
    // }

    let handler = thread::spawn(|| {
        for i in 1..10 {
            println!("sub thread {}", i);
            thread::sleep(Duration::from_millis(1));
        }
    });
    for i in 1..5 {
        println!("main thread {}", i);
        thread::sleep(Duration::from_millis(1));
    }
    handler.join().unwrap();
}

fn error_exercise() {
    // panic!("出错了");
    // println!("hello world");
    //panic! 程序立即退出,退出时调用者 抛出退出原因
    // let v = vec!["a","b","c"];
    // v[5];
    // let file
    //     = File::open("abc.jpeg");
    // println!("{:?}", file);
    let is_even = |x| {
        if x % 2 == 0 {
            Ok(true)
        } else {
            Err("不是偶数".to_string())
        }
    };
    println!("6 result== {}", is_even(6).unwrap());
    println!("11 result == {}", is_even(11).unwrap());
    //unwrap是Result<T,E>方法,调用此方法时,如果是Ok,直接返回Ok中,如果是Err枚举,在运行时会panic.
    //expect方法接收Msg:&str作为参数,可自定义报错信息.
}

fn smart_pointer() {
    //智能指针
    /***
    Box可以把数据存储到堆上,而不是栈上,box装箱,栈还是包含指向堆上数据的指针.
    如果一个结构体,实现了deref和drop的trait,那他们就不是普通结构体了.Rust提供了堆上存储数据的能力并把这个能力封装到了Box中.把
    栈上数据扔到堆上的能力就称为装箱.
     */
    let a = 6;
    let b = Box::new(a);
    println!("b={}", b);
    let price1 = 158;
    let price2 = Box::new(price1);
    //基本类型的比较 ,值是否相等
    println!("{}", 158 == price1);
    // *号被称为解引用 符.
    println!("{:?}", 158 == *price2);

    let x = 666;
    let y = CustomBox::new(x);

    println!("666==x {}", 666 == x);
    println!("666==*y {}", 666 == *y);
    println!("*y==x {}", *y == x);
}

struct CustomBox<T> {
    value: T,
}

impl<T> CustomBox<T> {
    fn new(v: T) -> CustomBox<T> {
        CustomBox { value: v }
    }
}

impl<T> Deref for CustomBox<T> {
    type Target = T;

    fn deref(&self) -> &T {
        &self.value
    }
}

impl<T> Drop for CustomBox<T> {
    fn drop(&mut self) {
        println!("drop CustomBox object!")
    }
}


fn main() {
    // println!("Hello, world!");
    // data_type();
    // io_exercise();
    // file_exercise();
    // iter_exercise();
    // closure_exercise();
    // thread_exercise();
    // error_exercise();
    // smart_pointer();
    study("gison win".to_string());
}

