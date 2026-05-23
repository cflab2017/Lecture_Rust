// 핵심 포인트:
// - 튜플 구조체의 필드 접근은 `.0`, `.1`, `.2` 처럼 인덱스 기반.
// - `#[derive(Debug)]` 를 붙이면 `{:?}` 로 보기 좋게 출력된다.

#[derive(Debug)]
struct Rgb(u8, u8, u8);

impl Rgb {
    fn invert(&self) -> Rgb {
        Rgb(255 - self.0, 255 - self.1, 255 - self.2)
    }
}

fn main() {
    let red = Rgb(255, 0, 0);
    let inv = red.invert();
    println!("원본: {:?}", red);
    println!("반전: {:?}", inv);
}
