// 7편 예제 2: 가변 참조(&mut T) 로 값을 수정하기
//
// 가변 참조를 만들려면 원본도 `mut` 이어야 하고,
// 같은 시점에 가변 참조는 **단 하나** 만 존재할 수 있다.

fn append_excl(s: &mut String) {
    s.push('!');
}

fn main() {
    let mut greeting = String::from("안녕");
    append_excl(&mut greeting);
    append_excl(&mut greeting);
    println!("{greeting}");

    // 직접 가변 참조를 만들어 사용하기
    let r = &mut greeting;
    r.push_str(" Rust");
    println!("{r}");
    // 이 시점부터는 r 을 더 쓰지 않으니 다른 참조를 만들어도 됩니다.
    println!("(원본) {greeting}");
}
