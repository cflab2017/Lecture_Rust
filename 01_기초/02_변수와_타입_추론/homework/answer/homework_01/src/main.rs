// 핵심 포인트:
// - 같은 변수를 갱신해야 하므로 `mut` 가 필요하다.
// - `+=`, `*=`, `-=` 같은 복합 대입 연산자는 가독성을 높여 준다.

fn main() {
    let mut counter = 0;
    println!("시작: {counter}");

    counter += 5;
    println!("+5 후: {counter}");

    counter *= 3;
    println!("*3 후: {counter}");

    counter -= 2;
    println!("-2 후: {counter}");

    println!("최종: {counter}");
}
