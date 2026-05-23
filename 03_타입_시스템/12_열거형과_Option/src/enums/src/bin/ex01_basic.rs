// 12편 예제 1: 열거형 — "이 중 하나" 를 표현하는 타입
//
// 각 variant 는 데이터를 가질 수 있고, 데이터 모양도 자유롭다.

#[derive(Debug)]
enum IpAddr {
    V4(u8, u8, u8, u8),    // 튜플형 데이터
    V6(String),            // 한 필드
    Loopback,              // 데이터 없는 variant
}

fn main() {
    let home = IpAddr::V4(127, 0, 0, 1);
    let server = IpAddr::V6(String::from("::1"));
    let lb = IpAddr::Loopback;

    println!("home   = {:?}", home);
    println!("server = {:?}", server);
    println!("lb     = {:?}", lb);
}
