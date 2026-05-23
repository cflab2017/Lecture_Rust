// 17편 예제 1: panic! 직접 호출
//
// "회복 불가능한" 상황을 만나면 panic!("메시지") 로 프로그램을 즉시 중단합니다.
// 이 예제는 정상 흐름과 패닉 흐름을 모두 보여 주기 위해 catch_unwind 로 감쌉니다.
// (실제 코드에서 catch_unwind 는 거의 쓰지 않습니다 — 보통은 그냥 죽게 둡니다.)

fn divide(a: i32, b: i32) -> i32 {
    if b == 0 {
        panic!("0으로 나눌 수 없습니다 (a={a})");
    }
    a / b
}

fn main() {
    // 기본 패닉 훅을 끄면 스택 트레이스가 안 보이고 콘솔이 깨끗합니다.
    std::panic::set_hook(Box::new(|_| {}));

    println!("정상: 10/2 = {}", divide(10, 2));

    // 패닉을 잡아 메시지를 추출 (학습 용도)
    let result = std::panic::catch_unwind(|| divide(10, 0));
    if let Err(payload) = result {
        let msg = payload
            .downcast_ref::<String>()
            .cloned()
            .or_else(|| payload.downcast_ref::<&str>().map(|s| s.to_string()))
            .unwrap_or_else(|| String::from("(메시지 추출 실패)"));
        println!("패닉 발생! 메시지: {msg}");
    }
}
