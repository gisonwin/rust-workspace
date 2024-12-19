extern crate tokio;

use tokio::io;
use tokio::net::TcpStream;
// use tokio::prelude::*;
use tokio::time::{self, sleep, Duration, sleep_until, Instant};
// use log::{debug,error,log_enabled,info,trace,Level};
use tklog::{trace, debug, error, fatal, info, warn, sync::Logger, LEVEL, LOG, Format, MODE};

fn log_init() {
    LOG.set_console(true) //set to console
        .set_level(LEVEL::Trace) //logger level ,default Debug
        .set_format(Format::LevelFlag | Format::Date | Format::Microseconds | Format::ShortFileName)//结构化日志,定义输出日志信息
        // .set_cutmode_by_size("logs/param.log",1<<20,10,true)//每1M文件切分一次，保留10个备份日志文件，并压缩备份日志 1<<10=1024=1K 1024<<10=1048576 1KX1K=1M字节
        .set_cutmode_by_time("logs/param.log", MODE::DAY, 0, false) //按天生成日志,0为不限备份文件个数,false是不压缩备份日志文件 MODE::HOUR 小时 MODE::MONTH 按月
        .set_formatter("{level}{time} {file}:{message}\r\n"); //自定义日志输出格式,默认 level,time,file:message
}

#[tokio::main]
async fn main() {
    use std::future::Future;
    use async_std::future::Future;
    log_init();
    trace!("starting");
    // let addr = "127.0.0.1:6142".parse().unwrap();
    // let stream = TcpStream::connect(addr);
    //
    // let hello_world = TcpStream::connect(&addr).await;

    sleep(Duration::from_millis(1000)).await;
    debug!("debugging");
    sleep_until(Instant::now() + Duration::from_millis(1000)).await; //sleep_until 等待直到指定的时间点
    info!("end");
    let mut interval = time::interval(Duration::from_millis(10));
    interval.tick().await;
    warn!("warn log");
    interval.tick().await;
    error!("error log");
    interval.tick().await;
    fatal!("fatal log");
}
