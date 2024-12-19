use std::task::Context;
use actix::prelude::*;

///define message
#[derive(Message)]
#[rtype(result = "Result<bool,std::io::Error>")]
struct Ping;

///define actor
struct MyActor;

///provide actor implementation for our actor
impl Actor for MyActor {
    type Context = Context<Self>;

    fn started(&mut self, ctx: &mut Context<Self>) {
        println!("Actor is alive");
    }
    fn stopped(&mut self, ctx: &mut Context<Self>) {
        println!("Actor is stopped");
    }
}

/// define handler for Ping message

impl Handler<Ping> for MyActor {
    type Result = Result<bool, std::io::Error>;
    fn handler(&mut self, msg: Ping, ctx: &mut Context<Self>) -> Self::Result {
        println!("Ping received");
        Ok(true)
    }
}

#[actix_rt::main]
async fn main() {
    let addr = MyActor.start();

    let result
        = addr.send(Ping).await;

    match result {
        Ok(res) => println!("Got result:{}", res.unwrap()),
        Err(err) => println!("Got error:{}", err),
    }
}

// let addr = MyActor { count: 10 }.start();
// let res = addr.send(Ping(10)).await;
// println!("RESULT:{}", res.unwrap()==20);
// System::current().stop();