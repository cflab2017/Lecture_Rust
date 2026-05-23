// 핵심 포인트:
// - `let` 으로 변수에 값을 묶고, `println!` 의 `{}` 자리에 변수 이름을 직접 적어 보간할 수 있다.
// - 모든 문장은 세미콜론으로 끝나야 한다.

fn main() {
    let name = "김루스트";
    let hobby = "책 읽기";
    let language = "Rust";

    println!("이름: {name}");
    println!("취미: {hobby}");
    println!("좋아하는 언어: {language}");
}
