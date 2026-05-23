// 15편 예제 3: Result 메서드 모음
//
// map / map_err / ok / err / expect

fn main() {
    let good: Result<i32, &str> = Ok(7);
    let bad: Result<i32, &str> = Err("입력 형식 오류");

    // map — Ok 안의 값 변환 (에러는 그대로)
    println!("good.map(|x| x*2) = {:?}", good.map(|x| x * 2));
    println!("bad.map (|x| x*2) = {:?}", bad.map(|x| x * 2));

    // map_err — Err 안의 값 변환
    let bad2: Result<i32, String> = bad.map_err(|e| format!("[입력에러] {e}"));
    println!("map_err: {:?}", bad2);

    // ok — Result → Option (에러 버리고 Some/None 으로)
    let opt = good.ok();
    println!("good.ok() = {:?}", opt);

    // expect — Err 일 때 패닉, 메시지를 같이 출력 (안전 명백할 때만)
    let safe = Ok::<i32, &str>(99).expect("이 자리는 항상 Ok");
    println!("expect 결과 = {safe}");
}
