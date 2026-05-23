// 2편 예제 3: const 와 static
// `cargo run --bin ex03_consts`

// const 는 컴파일 타임에 결정되며, 타입을 반드시 명시합니다.
const MAX_USERS: u32 = 100_000;

// static 은 프로그램 수명 내내 같은 메모리에 존재합니다.
static APP_NAME: &str = "Lecture_Rust";

fn main() {
    println!("APP_NAME  = {APP_NAME}");
    println!("MAX_USERS = {MAX_USERS}");

    // 숫자 리터럴 안의 `_` 는 가독성 구분자 (값에는 영향 없음)
    let big = 1_000_000;
    println!("big = {big}");

    // 타입 접미사 — 23i64, 1.5f32 처럼 명시 가능
    let count = 23i64;
    let ratio = 1.5f32;
    println!("count = {count}, ratio = {ratio}");
}
