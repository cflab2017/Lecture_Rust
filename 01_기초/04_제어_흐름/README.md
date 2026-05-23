# 04. 제어 흐름 (if·loop·while·for·break value)

Rust 의 제어 흐름은 친숙해 보이지만 **표현식 지향** 이라는 점에서 다른 언어와 미묘하게 다릅니다. `if` 가 값을 가지고, `loop` 가 값을 반환할 수 있으며, `for` 는 이터레이터를 소비합니다. 이번 편에서 그 차이를 익혀 두면 이후 모든 코드가 한결 자연스러워집니다.

## 학습 목표

- `if`/`else`/`else if` 를 문장과 표현식 두 가지 형태로 모두 사용한다.
- `loop` + `break value` 패턴으로 값을 반환받는다.
- `while` 과 `for in` 을 상황에 맞게 골라 쓴다.
- 라벨 루프로 중첩 루프를 한 번에 빠져나간다.

## 핵심 개념

### 1) `if` 는 표현식이다

조건의 양 분기 타입이 같다면 `if` 전체를 값으로 사용할 수 있습니다.

```rust
let label = if score >= 60 { "통과" } else { "재시험" };
```

조건 자리에는 반드시 `bool` 만 옵니다(`if 0` 같은 정수 truthy 없음).

### 2) `loop` 와 `break <값>`

```rust
let answer = loop {
    if 조건 { break 42; }
};
```

`break` 에 값을 실어 보내면 `loop` 표현식의 평가값이 됩니다. 무한 루프 + 조건부 종료 패턴에 매우 유용합니다.

### 3) `while` vs `for in`

- `while`: 조건이 참인 동안 반복. 인덱스가 변하는 경우.
- `for in`: 이터레이터 / 범위(`0..n`, `0..=n`) / 배열·벡터 등을 순회. 인덱스가 없어도 되는 경우 우선.

### 4) 라벨 루프

`'label:` 로 루프에 이름을 붙이면 안쪽에서 `break 'label;` 으로 바깥을 한 번에 끝낼 수 있습니다.

## 예제로 보기

### 예제 1 — `ex01_if.rs` : 조건 분기

```rust
// 4편 예제 1: if·else·else if 와 if 를 표현식으로 사용하기

fn main() {
    let score = 78;

    // 전통적인 if/else if/else 사슬
    if score >= 90 {
        println!("A");
    } else if score >= 80 {
        println!("B");
    } else if score >= 70 {
        println!("C");
    } else {
        println!("F");
    }

    // Rust 의 if 는 표현식이라 값으로 사용할 수 있다.
    let label = if score >= 60 { "통과" } else { "재시험" };
    println!("결과: {label}");

    // 단, 모든 분기의 타입이 같아야 한다.
    // let bad = if true { 1 } else { "오류" }; // ❌ 타입 불일치
}
```

### 예제 2 — `ex02_loop_while.rs` : loop 의 break 값과 while

```rust
// 4편 예제 2: loop 의 break 값과 while

fn main() {
    // loop 는 무한 루프지만 break 에 값을 실어 보내면 표현식 값이 된다.
    let mut counter = 0;
    let result = loop {
        counter += 1;
        if counter == 10 {
            break counter * 2; // 이 값이 loop 표현식의 결과
        }
    };
    println!("loop 결과: {result}");

    // while 은 조건 기반 반복
    let mut n = 3;
    while n > 0 {
        println!("카운트다운: {n}");
        n -= 1;
    }
    println!("발사!");
}
```

### 예제 3 — `ex03_for.rs` : for in 범위·배열, 라벨 루프

```rust
// 4편 예제 3: for in 범위·배열, 그리고 라벨 루프

fn main() {
    // 0..5 는 0,1,2,3,4 (끝 제외). 0..=5 는 0~5 (끝 포함).
    for i in 0..5 {
        println!("i = {i}");
    }

    let arr = ["사과", "바나나", "체리"];
    for fruit in arr.iter() {
        println!("과일: {fruit}");
    }

    // 인덱스가 같이 필요하면 enumerate()
    for (idx, fruit) in arr.iter().enumerate() {
        println!("{idx}: {fruit}");
    }

    // 라벨 루프 — 바깥 루프 break 에 쓸 수 있다.
    'outer: for x in 0..3 {
        for y in 0..3 {
            if x + y == 3 {
                println!("breaking at ({x},{y})");
                break 'outer;
            }
        }
    }
}
```

## 자주 하는 실수

### Q. `if 1 == 1` 처럼 적었는데 컴파일이 막힙니다.

A. `==` 가 정확한 비교 연산자입니다. `=` 는 대입이므로 조건식에 쓸 수 없습니다. 또한 `if` 조건 자리에는 `bool` 만 올 수 있어 `if 1 { ... }` 같은 코드는 허용되지 않습니다.

### Q. `for i in 0..n` 에서 `n` 을 포함하려면?

A. 끝을 포함하려면 `0..=n` 처럼 `=` 를 붙이세요. `0..n` 은 끝을 제외합니다.

### Q. `loop` 와 `while true` 중 어느 쪽이 좋나요?

A. 무한 루프 의도가 분명하면 `loop` 가 표준입니다. `loop` 는 컴파일러가 "끝나지 않는 루프" 라는 정보를 활용해 더 똑똑한 검사를 해 줍니다. `break` 에 값을 실어 보낼 수 있는 것도 `loop` 만의 특권입니다.

### Q. 중첩 루프 한 번에 빠져나갈 때 플래그 변수가 필요한가요?

A. 라벨을 사용하세요(`'outer: for ... { ... break 'outer; }`). 불리언 플래그를 검사하는 패턴보다 의도가 명확합니다.

## 정리

- `if` 는 표현식 — 양 분기 타입이 같으면 값으로 쓸 수 있다.
- `loop` 는 무한 루프, `break value` 로 값을 반환 가능.
- `while` 은 조건 기반, `for in` 은 이터레이터·범위를 순회.
- 중첩 루프 탈출은 라벨 `'name:` + `break 'name;`.

## 직접 해 보기

`homework/` 폴더의 과제를 풀어 보세요. 정답은 `homework/answer/` 에 있습니다.

## 다음 단원

[05. 함수·표현식·반환값·문서화 주석](../05_함수와_표현식/README.md) — Rust 함수의 핵심 규약과 표현식 vs 문(statement) 의 차이를 정리합니다.
