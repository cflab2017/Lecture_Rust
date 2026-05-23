// 3편 예제 1: 정수·실수·bool

fn main() {
    // 정수 타입은 i / u + 비트 수: i8, i16, i32(기본), i64, i128, isize
    // 부호 없는 정수는 u8 ~ u128, usize.
    let signed: i32 = -42;
    let unsigned: u64 = 100_000_000;
    let pointer_size: usize = 1024; // 배열/벡터 길이 표현에 사용

    println!("i32  = {signed}");
    println!("u64  = {unsigned}");
    println!("usize= {pointer_size}");

    // 실수 타입은 f32, f64(기본)
    let pi: f64 = 3.141_592_653_589_793;
    let half = 0.5_f32;
    println!("pi   = {pi}");
    println!("half = {half}");

    // 산술 연산자: + - * / %
    // 정수 나눗셈은 버림 (소수점 버려짐)
    let div = 7 / 2;       // 3
    let rem = 7 % 2;       // 1
    let fdiv = 7.0 / 2.0;  // 3.5
    println!("7/2 = {div}, 7%2 = {rem}, 7.0/2.0 = {fdiv}");

    // bool — true / false
    let on: bool = true;
    let off = !on;
    println!("on = {on}, off = {off}");
}
