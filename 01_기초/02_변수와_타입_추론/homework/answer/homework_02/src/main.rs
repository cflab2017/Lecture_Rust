// 핵심 포인트:
// - 섀도잉은 같은 이름을 `let` 으로 다시 묶어 새 변수를 만든다. 타입을 바꿀 수 있다.
// - parse 는 결과가 일반화되어 있으므로, 좌변에 `: i32` 를 명시해 타입을 알려 줘야 한다.

fn main() {
    let input = "12345";
    println!("원본 문자열: {input}");

    // 섀도잉: &str → i32
    let input: i32 = input.parse().expect("숫자가 아님");
    println!("정수 변환: {input}");

    let plus = input + 100;
    println!("+100: {plus}");
}
