// 2편 예제 2: 섀도잉 — 같은 이름으로 새 변수를 덮어쓰기
// `cargo run --bin ex02_shadowing`

fn main() {
    // 섀도잉은 `let` 으로 같은 이름을 다시 묶는 것입니다.
    // `mut` 와 달리 타입도 바꿀 수 있고, 결과적으로 "새 변수" 가 됩니다.
    let value = 5;
    let value = value + 1;       // 6
    let value = value * 2;       // 12
    println!("value = {value}");

    // 타입을 바꾸는 섀도잉
    let spaces = "   ";          // &str
    let spaces = spaces.len();   // usize
    println!("공백 길이 = {spaces}");

    // mut 로는 불가능: 다른 타입을 대입하면 컴파일 에러.
    // let mut s = "   "; s = s.len(); // ❌
}
