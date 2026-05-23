// 17편 예제 3: expect 와 RUST_BACKTRACE
//
// `unwrap` 은 "값이 없으면 패닉" 의 일반 메시지만 띄웁니다.
// `expect("이유")` 는 패닉 시 메시지를 함께 출력해 디버깅에 큰 도움이 됩니다.
// 또한 환경변수 RUST_BACKTRACE=1 로 실행하면 호출 스택을 볼 수 있습니다.

fn main() {
    std::panic::set_hook(Box::new(|_| {}));

    // 안전한 자리에서의 expect — 메시지가 "왜 안전한지" 의 문서가 된다.
    let s = "42";
    let n: i32 = s.parse().expect("이 자리는 항상 정수 — 위에서 검증함");
    println!("n = {n}");

    // 실패 시 expect 메시지 확인
    let r = std::panic::catch_unwind(|| {
        let _: i32 = "abc".parse().expect("입력은 항상 정수여야 함");
    });
    if r.is_err() {
        println!("expect 패닉 발생 — 메시지가 디버그에 큰 도움.");
    }

    println!("환경변수 RUST_BACKTRACE=1 로 실행하면 스택 트레이스를 확인할 수 있습니다.");
}
