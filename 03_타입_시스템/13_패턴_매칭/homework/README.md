# 과제 - 13. 패턴 매칭

## 문제 1 — 요일과 영업일 판정
- 파일명: `homework_01/src/main.rs`
- 핵심 개념: `match`, `|` 패턴 결합, 가드

### 요구사항
- `Day` enum 을 `Mon, Tue, Wed, Thu, Fri, Sat, Sun` 로 정의한다.
- 함수 `kind(d: &Day) -> &'static str` 가
  - 평일(월~금)이면 `"평일"`
  - 주말(토,일)이면 `"주말"`
  를 돌려준다.
- main 에서 7요일 모두를 한 줄씩 출력한다.

### 예상 출력
```
Mon → 평일
Tue → 평일
Wed → 평일
Thu → 평일
Fri → 평일
Sat → 주말
Sun → 주말
```

### 힌트
- `Day::Mon | Day::Tue | Day::Wed | Day::Thu | Day::Fri => "평일"`.
- 7 개 모두 다루므로 `_` 와일드카드는 필요 없다(권장).
- `#[derive(Debug)]` 를 붙이고 `{:?}` 로 출력하면 변수명이 그대로 나옵니다.

## 문제 2 — Option Vec 합산
- 파일명: `homework_02/src/main.rs`
- 핵심 개념: `if let`, 반복

### 요구사항
- 입력: `let values: Vec<Option<i32>> = vec![Some(1), None, Some(3), Some(5), None, Some(7)];`
- `Some` 인 값만 합산해 출력한다.

### 예상 출력
```
Some 값들의 합 = 16
```

### 힌트
- `for v in &values { if let Some(n) = v { total += n; } }`.
- `values.iter().flatten().sum::<i32>()` 같이 한 줄 풀이도 가능 — 도전!

## 정답 확인

직접 풀어 본 후 [`answer/`](./answer/) 폴더의 정답과 비교해 보세요.
