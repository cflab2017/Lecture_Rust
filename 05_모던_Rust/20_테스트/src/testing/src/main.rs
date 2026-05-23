// 라이브러리(`testing`) 의 API 를 사용하는 작은 실행 파일.
// 테스트는 `cargo test` 로 실행하세요.

use testing::{add, is_even};

fn main() {
    println!("add(2, 3) = {}", add(2, 3));
    println!("is_even(4) = {}", is_even(4));

    println!();
    println!("이 크레잇의 테스트는 다음 명령으로 실행하세요:");
    println!("  cargo test                          # 전체");
    println!("  cargo test --lib                    # 단위 테스트만");
    println!("  cargo test --test integration_test  # 통합 테스트만");
    println!("  cargo test --doc                    # doctest 만");
}
