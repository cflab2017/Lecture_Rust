// 핵심 포인트:
// - `&mut Vec<i32>` 로 받으면 컬렉션 자체를 수정할 수 있다.
// - 내부에서 `iter_mut()` 로 각 원소의 가변 참조를 얻은 뒤 `*n *= 2;` 로 수정한다.

fn double_all(v: &mut Vec<i32>) {
    for n in v.iter_mut() {
        *n *= 2;
    }
}

fn main() {
    let mut nums = vec![1, 2, 3, 4];
    println!("호출 전: {:?}", nums);
    double_all(&mut nums);
    println!("호출 후: {:?}", nums);
}
