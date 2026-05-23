// 3편 예제 2: char 는 유니코드 스칼라 한 글자 (4 바이트)

fn main() {
    let a: char = 'A';
    let han: char = '한';
    let emoji: char = '🦀';
    let escape: char = '\n';

    println!("ASCII : {a}");
    println!("한글  : {han}");
    println!("이모지: {emoji}");
    println!("개행 이스케이프 문자도 char: {}", escape as u32);

    // char 는 작은따옴표, 문자열(`&str`) 은 큰따옴표
    let s: &str = "안녕";
    println!("문자열: {s}, 바이트 길이: {}", s.len());
    // ⚠️ s.len() 은 "바이트" 길이입니다. 한국어 한 글자는 보통 3 바이트입니다.
    println!("문자 개수: {}", s.chars().count());
}
