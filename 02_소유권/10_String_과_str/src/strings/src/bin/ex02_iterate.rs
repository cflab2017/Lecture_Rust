// 10편 예제 2: 문자열 순회와 인덱싱
//
// 문자열은 UTF-8 가변 길이 인코딩이라 직접 인덱싱이 막혀 있습니다.
// 대신 `chars()` (유니코드 스칼라) 와 `bytes()` (원시 바이트) 를 사용합니다.

fn main() {
    let s = String::from("Rust 한");

    // s[0] 같은 인덱싱은 컴파일 에러
    // let c = s[0]; // ❌

    // 문자(char) 단위 순회
    print!("chars: ");
    for c in s.chars() {
        print!("{c} ");
    }
    println!();

    // 바이트 단위 순회 (UTF-8 raw bytes)
    print!("bytes: ");
    for b in s.bytes() {
        print!("{b} ");
    }
    println!();

    println!("byte len  = {}", s.len());
    println!("char count = {}", s.chars().count());

    // 안전한 슬라이싱: 문자 경계를 알 때만 사용
    // "Rust" 의 4 바이트만 가져오기
    let head = &s[..4];
    println!("head = {head}");
}
