// `tests/` 폴더의 파일은 통합 테스트(integration test) 로 취급됩니다.
// 외부 사용자처럼 라이브러리의 공개 API 만 사용해 검증합니다.

use testing::{add, is_even};

#[test]
fn integration_add() {
    assert_eq!(add(10, 20), 30);
    assert_eq!(add(-5, 5), 0);
}

#[test]
fn integration_even() {
    for n in [0, 2, 4, 100] {
        assert!(is_even(n));
    }
    for n in [1, 3, 5] {
        assert!(!is_even(n));
    }
}
