// 12편 예제 3: Option<T> — null 안전 타입
//
// 표준 라이브러리의 Option 은 사실 enum 입니다:
//   pub enum Option<T> { None, Some(T) }
// Rust 에 null 이 없는 비결입니다.

fn find_even(nums: &[i32]) -> Option<i32> {
    for &n in nums {
        if n % 2 == 0 {
            return Some(n);
        }
    }
    None
}

fn main() {
    let result1 = find_even(&[1, 3, 5, 8, 9]);
    let result2 = find_even(&[1, 3, 5]);

    // 가장 기본적인 처리: match
    match result1 {
        Some(n) => println!("처음 짝수: {n}"),
        None => println!("짝수 없음"),
    }
    // unwrap_or: None 일 때 기본값
    let v = result2.unwrap_or(-1);
    println!("두 번째 결과(기본 -1) = {v}");

    // ? 연산자는 16편에서 자세히 다룹니다.
}
