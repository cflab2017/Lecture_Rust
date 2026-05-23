// 핵심 포인트:
// - 입력이 명백히 안전한 자리에서는 expect 의 메시지로 "왜 안전한가" 를 설명해 두는 게 좋다.
// - iter().map(...).sum() 패턴은 매우 자주 등장한다.

fn main() {
    let inputs = ["1", "2", "3"];
    let sum: i32 = inputs
        .iter()
        .map(|s| s.parse::<i32>().expect("위에서 정수임을 검증"))
        .sum();
    println!("합계 = {sum}");
}
