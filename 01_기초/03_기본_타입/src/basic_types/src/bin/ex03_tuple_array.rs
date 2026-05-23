// 3편 예제 3: 튜플과 배열

fn main() {
    // 튜플: 서로 다른 타입을 묶을 수 있는 고정 길이 컬렉션
    let point: (i32, i32, &str) = (3, 5, "origin");

    // 인덱스 접근은 `.0`, `.1` ...
    println!("x = {}, y = {}, name = {}", point.0, point.1, point.2);

    // 구조 분해 (destructuring)
    let (x, y, name) = point;
    println!("분해: {x}, {y}, {name}");

    // 배열: 같은 타입의 고정 길이 컬렉션 [T; N]
    let scores: [u32; 5] = [80, 90, 75, 100, 60];
    println!("첫 점수: {}", scores[0]);
    println!("점수 개수: {}", scores.len());

    // 동일 값으로 채우기: [값; 개수]
    let zeros = [0u8; 4];
    println!("zeros = {:?}", zeros);
}
