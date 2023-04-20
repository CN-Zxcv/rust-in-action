//! 准备在没有操作系统的情况下运行
// 禁用标准库，不能依赖操作系统提供的内存分配，标准库用不了
#![no_std]
// main 函数非常特殊
// 通常它的参数有编译器提供 (_start())，并且返回值在程序退出前被解释
#![no_main]
// 引入 LLVM 内部函数
// 这些实现不受 Rust 稳定性保证，所以必须使用 nightly
#![feature(core_intrinsics)]
// 标记为语言项目，当我们禁用了 std 后，需要自己实现语言的一些功能
#![feature(lang_items)]

use core::intrinsics;
use core::panic::PanicInfo;
use x86_64::instructions::hlt;
use core::fmt;
use core::fmt::Write;

// VGA 颜色定义
#[allow(unused)]
#[derive(Clone, Copy)]
// 指示编译器使用单字节表示
// 编译器对枚举具体类型有自由裁量权(i32,u8,i16,u16..)，编译器会根据情况选择合适的类型
// 如果通过标记严格指定类型 #[repr()]
#[repr(u8)]
enum Color {
    Black = 0x0,
    Blue = 0x1,
    Green = 0x2,
    Cyan = 0x3,
    Red = 0x4,
    Magenta = 0x5,
    Brown = 0x6,
    Gray = 0x7,
    DarkGray = 0x8,
    BrightBlue = 0x9,
    BrightGreen = 0xA,
    BrightCyan = 0xB,
    BrightRed = 0xC,
    BrightMagenta = 0xD,
    Yellow = 0xE,
    White = 0xF,
}

// no_mangle 禁用 Rust 符号命名约定
// 符号名称时已编译的二进制文件中的字符串
// Rust 支持多个库共存，编译时防止名称冲突，会通过名称重整创建符号来避免这个问题
// 这个过程被称为 mangle

// 异常捕获
#[panic_handler]
#[no_mangle]
pub fn panic(info: &PanicInfo) -> ! {
    // intrinsics::abort();
    
    let mut cursor = Cursor {
        position: 0,
        foreground: Color::White,
        background: Color::Red,
    };

    // 清屏，设置成红色背景
    for _ in 0 ..(80 * 25) {
        cursor.print(b" ");
    }
    cursor.position = 0;
    write!(cursor, "{}", info).unwrap();

    loop { 
        hlt() 
    }
}

// 异常处理？
// 这里没看懂，和堆栈展开是什么关系？
#[lang = "eh_personality"]
#[no_mangle]
pub extern "C" fn eh_personality() {}

// _start 是链接器约定的程序入口函数
// 在正常的环境中，_start() 有三个工作
// _start() {
//  // 重置系统，例如在嵌入式中，会讲内存清0
//  _reset_env()
//  // 调用 main() 函数
//  main()
//  // 调用 exit
//  _exit()
// }
// 无系统启动时，这些都要自己处理，这些工作暂时都不重要，所以直接在 _start() 中写逻辑
//
// 返回值为 !，表示永不返回
#[no_mangle]
pub extern "C" fn _start() -> ! {
    // 引导模式下使用原始汇编设置一些魔法字节，这些字节由硬件解析
    // 硬件显示是 80 * 25 的网格
    // 对应了一个固定的内存缓冲区，由硬件解释输出到屏幕
    // 每个点都映射到内存中的位置，这个缓存区称为帧缓冲区
    //
    // 每个网格在内存中由两个字节表示，数据结构大致如下
    // 遵循 VGA 标准
    // struct VGACell {
    //      is_blinking: u1,
    //      background_color: u3,
    //      is_bright: u1,
    //      character_color: u3,
    //      character: u8, // 可用字符取自 code page 437，大约是 ASCII 的扩展
    // }
    //
    // VGA 文本有一个 16 色的调色板
    // 0xb8000 是硬件定义的 缓冲区开始地址

    //// 写缓存区测试
    // let framebuffer = 0xb8000 as *mut u8;

    // unsafe {
    //     // write_volatile
    //     // 强制写内存
    //     // volatile 禁止编译器进行优化
    //     framebuffer.offset(1).write_volatile(0x30);
    // }

    //// 输出文本
    // let text = b"Hello World!!!";
    // let mut cursor = Cursor {
    //     position: 0,
    //     foreground: Color::Red,
    //     background: Color::Black,
    // };

    // cursor.print(text);

    //// 异常捕获测试
    panic!("help!");

    loop {
        // hlt 简单避免 cpu 100%
        hlt();
    }
}

// 封装写屏幕
struct Cursor {
    position: isize,
    foreground: Color,
    background: Color,
}

impl Cursor {
    fn color(&self) -> u8 {
        let fg = self.foreground as u8;
        let bg = (self.background as u8) << 4;
        fg | bg
    }

    fn print(&mut self, text: &[u8]) {
        let color = self.color();
        let framebuffer = 0xb8000 as *mut u8;

        for &character in text {
            unsafe {
                framebuffer.offset(self.position).write_volatile(character);
                framebuffer.offset(self.position + 1).write_volatile(color);
            }
            self.position += 2;
        }
    }
}

// write!() 需要的特征实现
impl fmt::Write for Cursor {
    fn write_str(&mut self, s: &str) -> fmt::Result {
        self.print(s.as_bytes());
        Ok(())
    }
}
