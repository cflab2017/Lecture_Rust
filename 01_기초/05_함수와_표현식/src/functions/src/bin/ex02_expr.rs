// 5편 예제 2: 표현식(expression) vs 문장(statement)
// 핵심: 세미콜론이 있으면 "문장", 없으면 "표현식".

fn double(x: i32) -> i32 {
    let y = x + x; // `let ...;` 는 문장 (값이 없음)
    y              // 마지막 줄에 세미콜론이 없으니 표현식 = 반환값
}

// 블록 `{ ... }` 도 표현식이다. 마지막 식의 값이 블록의 값이 된다.
fn classify(score: i32) -> &'static str {
    let result = {
        if score >= 60 {
            "통과"
        } else {
            "재시험"
        }
    };
    result
}

// 명시적인 early return 도 가능
fn abs(x: i32) -> i32 {
    if x < 0 {
        return -x;
    }
    x
}

fn main() {
    println!("double(7) = {}", double(7));
    println!("classify(55) = {}", classify(55));
    println!("classify(75) = {}", classify(75));
    println!("abs(-9) = {}", abs(-9));
    println!("abs(9)  = {}", abs(9));
}
