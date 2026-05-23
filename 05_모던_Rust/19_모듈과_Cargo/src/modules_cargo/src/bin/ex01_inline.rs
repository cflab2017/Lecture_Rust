// 19편 예제 1: 인라인 모듈과 pub 가시성
//
// `mod` 키워드로 같은 파일 안에서 모듈을 정의할 수 있습니다.
// 모듈 안의 항목은 기본적으로 비공개(private) — `pub` 을 붙여야 외부에서 보입니다.

mod math {
    // pub 가 없으면 모듈 밖에서 호출 불가
    fn _internal() -> i32 { 42 }

    pub fn add(a: i32, b: i32) -> i32 {
        a + b
    }

    pub fn double(x: i32) -> i32 {
        x * 2
    }
}

fn main() {
    // 외부에서는 `모듈명::함수명` 으로 호출
    let sum = math::add(3, 4);
    let twice = math::double(sum);
    println!("(3+4)*2 = {twice}");

    // math::_internal();  // ❌ 비공개라 호출 불가
}
