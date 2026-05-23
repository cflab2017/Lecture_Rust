// 7편 예제 3: 빌림 규칙 — 같은 시점에
//   (a) 가변 참조 1개  또는
//   (b) 불변 참조 N개
// 둘 중 하나만 허용됨.

fn main() {
    let mut v = vec![1, 2, 3];

    // 불변 참조 여러 개는 OK
    let a = &v;
    let b = &v;
    println!("a = {a:?}, b = {b:?}");
    // 위 두 참조는 여기서 마지막 사용 → 이후로는 비활성화

    // 비활성화된 뒤에는 가변 참조를 새로 만들 수 있다.
    let m = &mut v;
    m.push(4);
    println!("m = {m:?}");

    // 댕글링 참조도 컴파일러가 막아 줍니다:
    // fn dangle() -> &String {
    //     let s = String::from("oops");
    //     &s   // ❌ s 는 함수가 끝나면 drop 됨
    // }
}
