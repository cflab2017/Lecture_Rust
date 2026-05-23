// 17편 예제 2: 인덱스 범위 초과 패닉
//
// 배열·벡터의 범위 밖 인덱스 접근은 컴파일러가 잡지 못하고 런타임에 패닉합니다.
// 안전한 대안은 `.get(i)` — 결과를 Option 으로 돌려줍니다.

fn main() {
    std::panic::set_hook(Box::new(|_| {}));

    let v = vec![10, 20, 30];
    println!("v[0] = {}, v[2] = {}", v[0], v[2]);

    // v[10] 은 인덱스 OOB → 패닉
    let r = std::panic::catch_unwind(|| v.clone()[10]);
    if r.is_err() {
        println!("v[10] 접근은 패닉 — 범위를 벗어났기 때문입니다.");
    }

    // 안전한 대안: Option 으로 받기
    println!("v.get(2)  = {:?}", v.get(2));
    println!("v.get(10) = {:?}", v.get(10));
}
