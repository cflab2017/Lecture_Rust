// 핵심 포인트:
// - 15 의 배수 검사를 먼저 두면 if/else if 사슬이 깔끔해진다.
// - 분기 결과 타입이 모두 같아야 표현식으로 묶을 수 있는데, 여기서는 출력만 하므로 신경 쓰지 않아도 된다.

fn main() {
    for i in 1..=15 {
        if i % 15 == 0 {
            println!("FizzBuzz");
        } else if i % 3 == 0 {
            println!("Fizz");
        } else if i % 5 == 0 {
            println!("Buzz");
        } else {
            println!("{i}");
        }
    }
}
