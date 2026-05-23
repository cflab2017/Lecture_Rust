// 19편 예제 3: 중첩 모듈과 super
//
// 모듈은 트리 구조라 자식 → 부모를 `super::` 로 접근할 수 있습니다.
// 또한 같은 깊이의 형제 모듈끼리도 부모를 거쳐 접근합니다.

mod geometry {
    pub mod circle {
        use std::f64::consts::PI;

        pub fn area(r: f64) -> f64 {
            // 부모의 형제(square)에 접근하려면 super::square::...
            PI * r * r
        }
    }

    pub mod square {
        pub fn area(side: f64) -> f64 {
            side * side
        }
    }
}

// 깊은 경로는 use 로 줄이기
use geometry::{circle, square};

fn main() {
    println!("circle area (r=2)  = {:.2}", circle::area(2.0));
    println!("square area (s=3)  = {:.2}", square::area(3.0));
}
