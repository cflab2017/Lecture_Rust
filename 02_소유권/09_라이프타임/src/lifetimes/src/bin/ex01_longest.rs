// 9편 예제 1: 명시적 라이프타임 'a 를 가진 함수
//
// 두 슬라이스 중 하나를 돌려주는 함수의 반환 라이프타임은
// "둘 다 살아 있는 동안" 이어야 합니다.

fn longest<'a>(a: &'a str, b: &'a str) -> &'a str {
    if a.len() >= b.len() { a } else { b }
}

fn main() {
    let s1 = String::from("길어요");
    let s2 = String::from("짧음");
    let result = longest(&s1, &s2);
    println!("긴 것: {result}");

    // 두 입력의 스코프가 다를 때
    let s_outer = String::from("outer scope");
    {
        let s_inner = String::from("inner scope-long");
        let r = longest(&s_outer, &s_inner);
        println!("긴 것(스코프 안): {r}");
        // s_inner 가 살아 있는 범위 내에서 r 도 유효
    }
}
