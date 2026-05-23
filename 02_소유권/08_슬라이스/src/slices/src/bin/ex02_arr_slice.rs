// 8편 예제 2: 배열/벡터 슬라이스 &[T]
//
// 배열·벡터의 연속된 일부분을 빌리는 타입이 &[T] 입니다.

fn sum(nums: &[i32]) -> i32 {
    let mut total = 0;
    for n in nums {
        total += n;
    }
    total
}

fn main() {
    let arr = [10, 20, 30, 40, 50];
    let vec = vec![1, 2, 3, 4, 5];

    // 배열·벡터의 전체를 슬라이스로 빌리기
    println!("배열 전체 합 = {}", sum(&arr));
    println!("벡터 전체 합 = {}", sum(&vec));

    // 일부 구간 빌리기
    let mid: &[i32] = &arr[1..4]; // 20, 30, 40
    println!("arr[1..4] = {:?}, 합 = {}", mid, sum(mid));
}
