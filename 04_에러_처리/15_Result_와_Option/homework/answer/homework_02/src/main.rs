// 핵심 포인트:
// - `and_then` 은 "Some 이면 다음 함수 호출, None 이면 그대로 None" 인 체이닝.
// - 한 단계라도 None 이 되면 그 뒤 단계는 자동으로 None 전파.

fn half(n: i32) -> Option<i32> {
    if n % 2 == 0 { Some(n / 2) } else { None }
}

fn triple(n: i32) -> Option<i32> {
    Some(n * 3)
}

fn pipeline(n: i32) -> Option<i32> {
    Some(n).and_then(half).and_then(triple).and_then(half)
}

fn main() {
    for n in [16, 9] {
        println!("{n} → {:?}", pipeline(n));
    }
}
