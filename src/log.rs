use std::borrow::{Borrow, BorrowMut};
use std::fs;
use std::fs::{File, OpenOptions};
use std::intrinsics::write_bytes;
use std::io::Write;
use std::path::Path;
use std::sync::{Mutex, MutexGuard};
use std::sync::mpsc::RecvError;
use std::thread::sleep;
use std::time::{Duration, SystemTime};

use log::{Level, Metadata, Record};
use log::{LevelFilter, SetLoggerError};
use log::{error, info, warn};

use crate::utils::time_util;

pub struct SimpleLogger {
    pub sender: std::sync::mpsc::Sender<String>,
    pub recv: std::sync::mpsc::Receiver<String>,
}

unsafe impl Send for SimpleLogger {}
unsafe impl Sync for SimpleLogger {}

impl SimpleLogger {
    pub fn new() -> Self {
        let (s, r) = std::sync::mpsc::channel();
        return Self {
            sender: s,
            recv: r,
        };
    }
    pub fn send(&self, arg: String) {
        self.sender.send(arg);
    }

    pub fn recv(&self) -> Result<String, RecvError> {
        self.recv.recv()
    }
}

lazy_static! {
  static ref LOG:SimpleLogger=SimpleLogger::new();
}



pub struct Logger {}

impl log::Log for Logger {
    fn enabled(&self, metadata: &Metadata) -> bool {
        metadata.level() <= Level::Info
    }
    fn log(&self, record: &Record) {
        if self.enabled(record.metadata()) {
            let data = format!("{} - {}", record.level(), record.args());
            println!("{}", data.as_str());
            LOG.send(data);
        }
    }
    fn flush(&self) {}
}

static LOGGER: Logger = Logger {};

/// 初始化日志文件路径
pub fn init_log(log_file_path: &str) -> Result<(), SetLoggerError> {
    let log_path = log_file_path.to_owned();
    std::thread::spawn(move || {
        let mut file = OpenOptions::new().create(true).append(true).open(log_path.as_str());
        if file.is_err() {
            file = File::create(Path::new(log_path.as_str()));
        }
        if file.is_err() {
            println!("[log] the log path:{} is not true!", log_path.as_str());
            return;
        }
        let mut file = file.unwrap();
        loop {
            let data = LOG.recv();
            if data.is_ok() {
                let s: String = data.unwrap()+"\n";
                file.write(s.as_bytes());
                file.flush();
            }
        }
    });
    log::set_logger(&LOGGER)
        .map(|()| log::set_max_level(LevelFilter::Info))
}


#[test]
pub fn test_log() {
    init_log("rbatis.log");
    info!("Commencing yak shaving");
    std::thread::sleep(Duration::from_secs(5));
}

#[test]
pub fn bench_log() {
    init_log("rbatis.log");
    let total = 1000;
    let now = SystemTime::now();
    for i in 0..total {
        //sleep(Duration::from_secs(1));
        info!("Commencing yak shaving");
    }
    time_util::count_time_tps(total, now);
}