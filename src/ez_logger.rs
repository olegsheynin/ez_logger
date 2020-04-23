extern crate strfmt;
use strfmt::strfmt;

use std::io::{Write};
use crate::timestamp::timestamp_fmt;

#[derive(Debug)]
#[allow(dead_code)]
#[derive(PartialOrd, PartialEq, Eq)] 
pub enum LogLevel {
    TRACE = 0
    , DEBUG = 1
    , INFO = 2
    , WARNING = 3
    , ERROR = 4
    , FATAL = 5
}

pub enum LogStream { 
    Stdout(std::io::Stdout),
    OutputFile(std::fs::File),
    AbstractWriter(Box<dyn std::io::Write + std::marker::Send>),    
}

impl LogStream {
    pub fn write(&mut self, rec: &str) ->std::io::Result<()> {
        match self {
            LogStream::Stdout(stdout) => {
                let mut handle = stdout.lock();
                handle.write_all(rec.as_bytes())?;
                Ok(())
            },
            LogStream::OutputFile(file) => {
                file.write_all(rec.as_bytes())?;
                file.sync_data()?;
                Ok(())
            },
            LogStream::AbstractWriter(wrtr) => {
                wrtr.write_all(rec.as_bytes())?;
                Ok(())
            }

        }
    }
}

pub struct Logger {
    min_level_ : LogLevel,
    out_stream_ : LogStream,
    ts_format_ : Option<String>,
    rec_format_ : Option<String>
}

impl Logger {
    pub fn new() -> Self {
        let res = Self {
            min_level_ : LogLevel::INFO,
            out_stream_ : LogStream::Stdout(std::io::stdout()) ,
            ts_format_ : None,
            rec_format_ : None,
        };
        return res;
    }

    pub fn set_min_level(&mut self, minlevel: LogLevel) {
        self.min_level_ = minlevel;
    }

    pub fn set_stream(&mut self, strm: LogStream) {
        self.out_stream_ = strm;
    }

    pub fn set_ts_format(&mut self, ts_fmt: String) {
        self.ts_format_ = if ts_fmt.len() == 0 {None} else {Some(ts_fmt)};
    }

    pub fn set_rec_format(&mut self, mut rec_fmt:  String) {
        self.rec_format_ = if rec_fmt.len() == 0 {None} else {
            if !rec_fmt.ends_with("\n") {
                rec_fmt += "\n";
            }
            Some(rec_fmt)
        };
    }

    pub fn log(&mut self, level: LogLevel, msg: &str) {
        if level >= self.min_level_ { 
            let rec = self.create_rec(level, msg);
            let res = self.out_stream_.write(&rec);
            if res.is_err() {
                std::io::stderr().write(b"Error writing to the log").unwrap();
                std::process::exit(1);
            }
        }
    }
    fn create_rec(&mut self, level: LogLevel, msg: &str) -> String {
        if self.rec_format_.is_none()  {
            return format!("{} {:?} {}\n"
            , timestamp_fmt(self.ts_format_.as_ref())
            , level, msg); 
        }
        use std::collections::HashMap;
        use std::thread;
        let mut vars = HashMap::new();
        vars.insert("timestamp".to_string(), format!("{}", timestamp_fmt(self.ts_format_.as_ref())));
        vars.insert("level".to_string(), format!("{:?}", level));
        vars.insert("message".to_string(), format!("{}", msg));
        vars.insert("thread".to_string(), format!("{:?}", thread::current().id()));
    
        let fmt = &self.rec_format_.as_ref().unwrap().to_string();
        return strfmt(fmt, &vars).unwrap();
    }
}

use std::sync::{Mutex, MutexGuard};
lazy_static! {
    pub static ref LOGGER_INSTANCE: Mutex<Logger> = Mutex::new(Logger::new());
}

pub fn log(level : LogLevel, msg: &str) {
    LOGGER_INSTANCE.lock().unwrap().log(level, msg);
}

pub fn get_logger() -> MutexGuard<'static, Logger> {
    return LOGGER_INSTANCE.lock().unwrap();
}

#[macro_export]
macro_rules! trace {
    ($($msg:expr),+) => {
        log(crate::ez_logger::LogLevel::TRACE, &format!($($msg),+));
    };
}

#[macro_export]
macro_rules! debug {
    ($($msg:expr),+) => {
        log(crate::ez_logger::LogLevel::DEBUG, &format!($($msg),+));
    };
}

#[macro_export]
macro_rules! info {
    ($($msg:expr),+) => {
        log(crate::ez_logger::LogLevel::INFO, &format!($($msg),+));
    };
}

#[macro_export]
macro_rules! warning {
    ($($msg:expr),+) => {
        log(crate::ez_logger::LogLevel::WARNING, &format!($($msg),+));
    };
}

#[macro_export]
macro_rules! error {
    ($($msg:expr),+) => {
        log(crate::ez_logger::LogLevel::ERROR, &format!($($msg),+));
    };
}

#[macro_export]
macro_rules! fatal {
    ($($msg:expr),+) => {
        log(crate::ez_logger::LogLevel::FATAL, &format!($($msg),+));
    };
}

#[cfg(test)]
mod tests {

    #[test]
    fn logger_test() {
        use crate::ez_logger::{log, get_logger};
        use crate::ez_logger::LogLevel::{
            TRACE,
        //     DEBUG, 
        //     INFO, 
            WARNING,
        //     ERROR, 
            // FATAL,
        };

        let val = 345;
        debug!("this is DEBUG message"); 
        info!("this is Info message {}", val * 3); 

        get_logger().set_min_level(WARNING);
        info!("this is INFO message: {}", val); 
        warning!("this is WARNING message"); 
        fatal!("this is FATAL message"); 
        error!("this is ERROR message {}", val); 

        get_logger().set_ts_format("hours={hours}".to_string());
        error!("different timestamp formatting {}", "Fatal"); 

        get_logger().set_ts_format(String::new());
        get_logger().set_rec_format("--- {timestamp} --- {thread} [[{level}]] {message}".to_string());
        fatal!("different rec formatting FATAL"); 

        get_logger().set_min_level(TRACE);
        trace!("trace message from macro");
        debug!("debug message from macro");
        info!("info message from macro");
        warning!("warning message from macro");
        error!("error message from macro");
        fatal!("fatal message from macro");
        assert!(true);
    }
}