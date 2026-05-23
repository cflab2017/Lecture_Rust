// 핵심 포인트:
// - 첫 원소로 max 를 초기화하고 나머지를 순회하며 갱신한다.
// - `for n in nums.iter()` 는 `n: &i32` 를 돌려주므로 비교·대입 시 `*n` 으로 값을 꺼낸다.

fn main() {
    let nums = [37, 12, 88, 45, 91, 23, 76];

    let mut max = nums[0];
    for n in nums.iter() {
        if *n > max {
            max = *n;
        }
    }

    println!("배열: {:?}", nums);
    println!("최댓값: {max}");
}
