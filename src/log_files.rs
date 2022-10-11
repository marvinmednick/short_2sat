use std::path::{Path,PathBuf};
use std::fs::{self,File};
use log::{ info, error };
use std::io::{Write}; 
use lazy_static::lazy_static;
use std::sync::Mutex;


#[derive(Debug)]
pub struct LogFile {
    log_file_path: String,
    log_file:  File,
}

lazy_static! {
          static ref MY_GLOBAL: Mutex<String> = Mutex::new(".".to_string());
}

pub fn set_log_dir(log_dir_name: &str) {
    *MY_GLOBAL.lock().unwrap() = log_dir_name.to_string();
}

pub fn get_log_dir() -> String {
    MY_GLOBAL.lock().unwrap().to_string()
}

impl LogFile {

    pub fn new(log_file_name: &str) -> Result<LogFile, String> {

        let mut log_dir_name = get_log_dir();

        let mut log_file = Path::new(&log_file_name);
        let mut full_path = PathBuf::from(&log_dir_name).join(log_file_name);

        if log_file.is_absolute() || log_file.starts_with("./") {
            full_path = PathBuf::from(&log_file_name);
        }
        let log_dir = full_path.parent().unwrap();
        log_dir_name = log_dir.to_str().unwrap().to_string();

        if !log_dir.exists() {
            info!("Creating log dir {}",log_dir_name);
            fs::create_dir_all(&log_dir_name);
        }
        else if !log_dir.is_dir() {
            error!("Log directory {} exists, but is not a directory",log_dir_name);

        }

//        let log_file_path = log_dir_name.to_owned() + "/" + log_file_name;
//        let path = Path::new(&log_file_path);
        let display_name = full_path.display().to_string();

        
        match File::create(&full_path) {
            Err(why) => Err("Couldn't open file: ".to_owned() + &display_name + " - " + &why.to_string()),
            Ok(file) => Ok(LogFile {log_file_path: display_name,log_file: file})
        }
        
    }

    pub fn write_line(&mut self, line: String) -> std::io::Result<()> {
        writeln!(self.log_file,"{}",line)
    }

    pub fn file(&self) -> &File {
        &self.log_file
    }
}

#[macro_export]
macro_rules! log_writeln {
    ($logfile:expr, $fmt:expr $(, $($arg:tt)*)?) => {
        writeln!($logfile.file(), $fmt, $($($arg)*)?);
    };
}
