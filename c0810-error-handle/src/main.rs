use std::fs::File;
use std::net::Ipv6Addr;
use std::net::AddrParseError;
use std::fmt;
use std::error;
use std::io;
use std::net;


// 使用 ? 关键字，本质上是通过宏展开的方式封装提前返回的处理 Result，代码类似下面
// marcro try {
//     match expression {
//         Result::Ok(val) = > val,
//         Result::Err(err) => {
//             let converted = convert::From::from(err);
//             return Result::Err(converted);
//         }
//     }
// }
// 当我们的子错误有多个类型时，直接返回是不被允许的
// 想返回可以有多种类型，一种方式是使用特征对象
// Box<dyn Error>
// 特征对象的缺点
// 特征对象也称为类型擦除，下游无法获得上游的原始类型
// 另外一种方式是使用enum，enum 是 rust 中在返回多种类型并保留原始信息最推荐的方式
// fn main() -> Result<(), Box<dyn Error>> {
//     let _f = File::open("no-file.txt")?;

//     let _localhost = "::1".parse::<Ipv6Addr>()?;

//     Ok(())
// }


#[derive(Debug)]
enum UpstreamError {
    IO(std::io::Error),
    Parsing(AddrParseError),
}

impl fmt::Display for UpstreamError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

impl error::Error for UpstreamError {}

// 没有实现 impl from 需要显式调用 map_err 来进行转换
// let _f = File::open("no-file.txt").map_err(UpstreamError::IO)?;
// let _localhost = "::1".parse::<Ipv6Addr>().map_err(UpstreamError::Parsing)?;
// 实现后
// let _f = File::open("no-file.txt")?;
// let _localhost = "::1".parse::<Ipv6Addr>()?;
impl From<io::Error> for UpstreamError {
    fn from(error: io::Error) -> Self {
        UpstreamError::IO(error)
    }
}

impl From<net::AddrParseError> for UpstreamError {
    fn from(error: net::AddrParseError) -> Self {
        UpstreamError::Parsing(error)
    }
}

fn main() -> Result<(), UpstreamError> {
    let _f = File::open("no-file.txt")?;
    let _localhost = "::1".parse::<Ipv6Addr>()?;
    Ok(())
}