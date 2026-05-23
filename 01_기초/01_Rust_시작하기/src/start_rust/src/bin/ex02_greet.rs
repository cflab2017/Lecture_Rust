// 1편 예제 2: println! 매크로로 변수 값 출력
// `{}` 자리에 인수 값이 순서대로 들어갑니다.

fn main() {
    let name = "지수";
    let year = 2026;

    println!("이름: {}", name);
    println!("연도: {}", year);

    // 위치 인수를 여러 개 넣을 수도 있습니다.
    println!("{} 님, {} 년에 Rust 를 시작했어요!", name, year);

    // 변수를 직접 보간하는 짧은 문법 (Rust 1.58+ 안정)
    println!("{name} 님 환영합니다.");
}
