// 핵심 포인트:
// - `v[i]` 는 OOB 시 패닉, `v.get(i)` 는 None 으로 안전하게 알려 준다.
// - Option 분기로 "비정상 입력" 을 일반 출력으로 처리한다.

fn nth(v: &[i32], i: usize) -> String {
    match v.get(i) {
        Some(n) => format!("v[{i}] = {n}"),
        None => format!("v[{i}] 는 범위 밖"),
    }
}

fn main() {
    let v = vec![10, 20, 30];
    for i in [0usize, 2, 5] {
        println!("{}", nth(&v, i));
    }
}
