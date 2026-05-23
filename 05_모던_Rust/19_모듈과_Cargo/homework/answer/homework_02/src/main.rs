// 핵심 포인트:
// - 자식 모듈도 `pub mod` 로 외부에 노출해야 부모 밖에서 접근 가능.
// - `shape::circle::area` 처럼 트리 경로로 호출한다.

mod shape {
    pub mod circle {
        use std::f64::consts::PI;

        pub fn area(r: f64) -> f64 {
            PI * r * r
        }
    }
}

fn main() {
    let r = 4.0;
    println!("circle area (r={}) = {:.2}", r as i32, shape::circle::area(r));
}
