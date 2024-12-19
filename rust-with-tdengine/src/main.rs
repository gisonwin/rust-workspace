// mod arrays;

use chrono::{DateTime, Local};
use taos::*;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let dsn = "taos://root:taosdata@124.70.89.186:6030";
    let builder = TaosBuilder::from_dsn(dsn)?;
    let taos = builder.build().await?;
    let db = "rustdb";

    //prepare database
    taos.exec_many([
        format!("DROP DATABASE IF EXISTS `{db}`"),
        format!("CREATE DATABASE `{db}`"),
        format!("USE `{db}`"),
    ])
    .await?;

    let inserted = taos.exec_many([
        //create super table
        "CREATE TABLE `meters` (`ts` TIMESTAMP,`current` FLOAT,`voltage` INT,`phase` FLOAT) TAGS (`groupid` INT,`location` BINARY(16))",
        //create child table
        "CREATE TABLE `d0` USING `meters` TAGS(0,'Los Angles')",
        //insert data into child table
        "INSERT INTO `d0` values (now-10s,10,116,0.32)",
        //insert null values
        "INSERT INTO `d0` values (now-8s,NULL,NULL,NULL)",
        //insert and automatically create table with tags if not exist
        "INSERT INTO `d1` USING `meters` TAGS(1,'San Francisco') values(now -9s,10.1,119,0.33)",
        //insert many records in a single sql
        "INSERT INTO `d1` values (now-8s,10,120,0.33) (now-6s,10,119,0.34) (now-4s,11.2,118,0.322)",
    ]).await?;

    assert_eq!(inserted, 6);
    let mut result = taos.query("select * from `meters`").await?;
    for field in result.fields() {
        println!("got field:{}", field.name());
    }
    // Query option 1, use rows stream.
    let mut rows = result.rows();
    while let Some(row) = rows.try_next().await? {
        for (name, value) in row {
            println!("got value of {}: {}", name, value);
        }
    }

    // Query options 2, use deserialization with serde.
    #[derive(Debug, serde::Deserialize)]
    #[allow(dead_code)]
    struct Record {
        // deserialize timestamp to chrono::DateTime<Local>
        ts: DateTime<Local>,
        // float to f32
        current: Option<f32>,
        // int to i32
        voltage: Option<i32>,
        phase: Option<f32>,
        groupid: i32,
        // binary/varchar to String
        location: String,
    }

    let records: Vec<Record> = taos
        .query("select * from `meters`")
        .await?
        .deserialize()
        .try_collect()
        .await?;

    dbg!(records);
    Ok(())
}

// fn main() {
//     println!("Hello, world!温庭辉");
//     let mut x:i8=127;
//     println!("{:?}", x);
//     x =-128;
//     println!("{:?}", x);
//     arras();
//     tuples();
//     destruct();
//     references();
// dangling_reference();

// }
// fn arras(){
//     let mut a:[i8;10] = [42;10];
//     a[5] = 0;
//     println!("a: {:?}", a);
// }
// fn tuples(){
//     let t:(i8,bool) = (7,true);
//     println!("1st index:{}", t.0);
//     println!("2dn index:{}", t.1);
// }
//
// fn destruct(){
//     let meal=("Falafel",4.5);
//     let (name,price) = meal;
//     println!("{name} costs {price} $");
// }
//
// fn references(){
//     let mut x:i32=10;
//     let ref_x:&mut i32 = &mut x;
//     *ref_x =20;
//     println!("x:{x}");
// }

// fn dangling_reference(){
//     let ref_x:&i32;
//     {
//         let x:i32=10;
//         ref_x = &x;
//     }
//     println!("ref_x:{ref_x}");
// }
