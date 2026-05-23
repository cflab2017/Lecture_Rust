// 2편 예제 1: let 불변 변수와 let mut 가변 변수
// `cargo run --bin ex01_basics`

fn main() {
    // 기본 변수는 "불변" 입니다. 한 번 묶은 값은 다시 바꿀 수 없습니다.
    let x = 10;
    println!("x = {x}");

    // 같은 변수에 다시 대입하려면 `mut` 키워드가 필요합니다.
    let mut y = 5;
    println!("처음 y = {y}");
    y = y + 1;
    println!("증가 후 y = {y}");

    // 타입을 명시할 수도 있습니다. (생략하면 Rust 가 추론)
    let pi: f64 = 3.14;
    let flag: bool = true;
    println!("pi = {pi}, flag = {flag}");
}
