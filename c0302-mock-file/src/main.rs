#![allow(unused_variables)]

//! 模拟文件

use core::fmt;
use rand::prelude::*;
use std::fmt::Display;
use std::vec;

#[derive(Debug, PartialEq)]
pub enum FileState {
    Open,
    Closed,
}

impl Display for FileState {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            FileState::Open => write!(f, "OPEN"),
            FileState::Closed => write!(f, "CLOSED"),
        }
    }
}

/// 文件
#[derive(Debug)]
pub struct File {
    name: String,
    data: Vec<u8>,
    state: FileState,
}

impl File {
    /// 创建文件
    pub fn new(name: &str) -> File {
        File {
            name: String::from(name),
            data: Vec::new(),
            state: FileState::Closed,
        }
    }

    /// 文件长度
    pub fn len(&self) -> usize {
        self.data.len()
    }

    /// 文件名
    pub fn name(&self) -> String {
        self.name.clone()
    }

    fn new_with_data(name: &str, data: &Vec<u8>) -> File {
        let mut f = File::new(name);
        f.data = data.clone();
        f
    }

    fn read(self: &File, save_to: &mut Vec<u8>) -> Result<usize, String> {
        if self.state != FileState::Open {
            return Err(String::from("File must be open for reading"));
        }

        let mut tmp = self.data.clone();
        let read_length = tmp.len();
        save_to.reserve(read_length);
        save_to.append(&mut tmp);
        Ok(read_length)
    }
}

impl Display for File {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "<{} {}>", self.name, self.state)
    }
}

fn one_in(d: u32) -> bool {
    thread_rng().gen_ratio(1, d)
}

fn open(mut f: File) -> Result<File, String> {
    f.state = FileState::Open;
    if one_in(10_000) {
        let err_msg = String::from("permission denied");
        return Err(err_msg);
    }
    return Ok(f);
}

fn close(mut f: File) -> Result<File, String> {
    f.state = FileState::Closed;
    if one_in(100_000) {
        let err_msg = String::from("Interrput by signal");
        return Err(err_msg);
    }
    Ok(f)
}

fn main() {
    let f_data: Vec<u8> = vec![114, 117, 115, 116, 33];
    let mut f = File::new_with_data("2.txt", &f_data);

    let mut buffer: Vec<u8> = vec![];

    if f.read(&mut buffer).is_err() {
        println!("error checking is working");
    }

    f = open(f).unwrap();
    let f_length = f.read(&mut buffer).unwrap();
    f = close(f).unwrap();

    let text = String::from_utf8_lossy(&buffer);

    println!("{:?}", f);
    println!("{}", f);
    println!("{} is {} bytes long", &f.name, f_length);
    println!("{}", text);
}
