use std::fs::File;
use std::io::Read;

fn main() {
    println!("> 원본 텍스트");

    let mut target_file = File::open("./IOTest.txt").expect("열기 오류");
    let mut data_buffer = String::new();

    target_file.read_to_string(&mut data_buffer).expect("읽기 오류");
    println!("{}", data_buffer);
}