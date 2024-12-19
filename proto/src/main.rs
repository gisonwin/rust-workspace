use crate::build::build;
use pb::hello;

mod build;
mod pb;

 fn test(){
     let request = hello::HelloRequest {
         name: "world".to_string(),
     };
     println!("request {:?}", request);

     let reply = hello::HelloReply { message: "reply".to_string() };
     println!("reply {:?}", reply);
 }

fn main() {
    // println!("Hello, world!");
    build();

}
