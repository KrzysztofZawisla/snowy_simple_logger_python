use chrono;
use colorful::Color;
use colorful::Colorful;
use pyo3::prelude::*;
use std::fs;
use std::path;

#[pyclass]
struct Console {}

#[pymethods]
impl Console {
    #[staticmethod]
    fn success(message: String) -> PyResult<()> {
        let _ = console_output(message, "success");
        Ok(())
    }
    #[staticmethod]
    fn error(message: String) -> PyResult<()> {
        let _ = console_output(message, "error");
        Ok(())
    }
    #[staticmethod]
    fn warn(message: String) -> PyResult<()> {
        let _ = console_output(message, "warn");
        Ok(())
    }
    #[staticmethod]
    fn info(message: String) -> PyResult<()> {
        let _ = console_output(message, "info");
        Ok(())
    }
}

#[pyclass]
struct File {
    path: String,
}

#[pymethods]
impl File {
    #[new]
    fn __new__(obj: &PyRawObject, file_path: Option<String>) {
        let true_path = match file_path {
            None => String::from("log.log"),
            Some(x) => x,
        };
        obj.init({ File { path: true_path } });
    }
    #[getter]
    fn get_path(&self) -> PyResult<&str> {
        Ok(&self.path)
    }
    #[setter]
    fn set_path(&mut self, file_path: Option<String>) -> PyResult<()> {
        let true_path = match file_path {
            None => String::from("log.log"),
            Some(x) => x,
        };
        self.path = true_path;
        Ok(())
    }
    fn success(&self, message: String) -> PyResult<()> {
        let _ = file_output(message, "success", &self.path);
        Ok(())
    }
    fn error(&self, message: String) -> PyResult<()> {
        let _ = file_output(message, "error", &self.path);
        Ok(())
    }
    fn warn(&self, message: String) -> PyResult<()> {
        let _ = file_output(message, "warn", &self.path);
        Ok(())
    }
    fn info(&self, message: String) -> PyResult<()> {
        let _ = file_output(message, "info", &self.path);
        Ok(())
    }
}

fn console_output(message: String, log_type: &str) -> PyResult<()> {
    let now = chrono::Local::now();
    let now = now.format("[%Y-%m-%d %H:%M:%S%.3f] ").to_string();
    let output = now + "[" + log_type + "] " + &message;
    if log_type == "info" {
        println!("{}", output.color(Color::Cyan));
    } else if log_type == "error" {
        println!("{}", output.color(Color::Red));
    } else if log_type == "success" {
        println!("{}", output.color(Color::Green));
    } else if log_type == "warn" {
        println!("{}", output.color(Color::Orange1));
    }
    Ok(())
}

fn file_output(message: String, log_type: &str, file_path: &str) -> () {
    let now = chrono::Local::now();
    let now = now.format("[%Y-%m-%d %H:%M:%S%.3f] ").to_string();
    let output = now + "[" + log_type + "] " + &message;
    if !path::Path::new(&file_path).exists() {
        fs::File::create(&file_path).expect("Cannot create log file");
    }
    let log_file_content = fs::read_to_string(&file_path).expect("Cannot read log file");
    let mut new_line = "";
    if log_file_content != "" {
        new_line = "\n";
    }
    let log_file_content = log_file_content + new_line + &output;
    fs::write(file_path, log_file_content).expect("Cannot write to log file");
}

#[pymodule]
fn snowy_simple_logger(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_class::<Console>()?;
    m.add_class::<File>()?;
    Ok(())
}
