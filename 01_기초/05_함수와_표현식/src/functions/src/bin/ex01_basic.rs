// 5편 예제 1: 함수 정의·호출, 매개변수와 반환값

// 매개변수에는 타입을 반드시 명시한다.
// `-> i32` 가 반환 타입.
fn add(a: i32, b: i32) -> i32 {
    a + b // 세미콜론 없음 → 표현식 → 반환값
}

// 반환값이 없는 함수의 반환 타입은 유닛 `()`. 생략 가능.
fn greet(name: &str) {
    println!("안녕하세요, {name}!");
}

fn main() {
    let sum = add(3, 4);
    println!("3 + 4 = {sum}");

    greet("Rust");

    // 함수 안에서 다른 함수를 호출할 수 있다.
    let total = add(add(1, 2), add(3, 4));
    println!("total = {total}");
}
