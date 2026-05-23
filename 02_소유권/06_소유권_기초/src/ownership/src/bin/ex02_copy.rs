// 6편 예제 2: Copy 트레잇이 구현된 타입 — 복사로 동작
//
// 스칼라 타입(i32, f64, bool, char, ...) 과 그들로만 구성된 튜플은 `Copy` 입니다.
// 대입·전달 시 비트 복사가 일어나므로 "이동" 처럼 보이지 않습니다.

fn main() {
    let a = 10;
    let b = a;        // a 의 비트가 b 로 복사
    println!("a = {a}, b = {b}"); // a 도 여전히 사용 가능

    let p = (1, 2);   // (i32, i32) 도 Copy
    let q = p;
    println!("p = {:?}, q = {:?}", p, q);

    show(a);
    println!("show 호출 후에도 a = {a}");
}

fn show(n: i32) {
    println!("show: {n}");
}
