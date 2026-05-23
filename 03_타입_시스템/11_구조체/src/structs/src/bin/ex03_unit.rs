// 11편 예제 3: 유닛 구조체 — 필드가 없는 타입
//
// 트레잇 구현용 마커, 상태 머신 표식 등에 자주 쓰입니다.

struct AlwaysReady; // 필드 없음

impl AlwaysReady {
    fn ping(&self) -> &'static str {
        "pong"
    }
}

// "상태" 자체를 타입으로 표현하는 패턴
struct Connected;
struct Disconnected;

fn describe<T>(_state: &T) -> &'static str
where
    T: HasName,
{
    T::name()
}

trait HasName {
    fn name() -> &'static str;
}

impl HasName for Connected {
    fn name() -> &'static str { "Connected" }
}

impl HasName for Disconnected {
    fn name() -> &'static str { "Disconnected" }
}

fn main() {
    let svc = AlwaysReady;
    println!("svc.ping() = {}", svc.ping());

    let on = Connected;
    let off = Disconnected;
    println!("on  → {}", describe(&on));
    println!("off → {}", describe(&off));
}
