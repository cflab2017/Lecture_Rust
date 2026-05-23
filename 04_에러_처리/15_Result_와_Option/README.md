# 15. Result<T,E> 와 Option<T> 다루기

Rust 의 에러 처리는 예외(exception) 가 아니라 **값** 으로 합니다. 실패할 수 있는 작업은 `Result<T, E>` 를, 값이 없을 수 있는 자리는 `Option<T>` 를 반환합니다. 이번 편에서는 두 타입의 핵심 메서드를 짚어 보고, 어느 자리에 어떤 처리를 쓰는지 정리합니다. (`?` 연산자는 16편에서 본격적으로 다룹니다.)

## 학습 목표

- `Result<T, E>` 와 `Option<T>` 의 의미·차이를 안다.
- `match`, `if let` 외에도 `map`, `and_then`, `unwrap_or` 등 체이닝 메서드를 사용한다.
- `parse::<i32>()` 같이 표준 라이브러리에서 자주 만나는 Result 를 처리한다.
- `expect` / `unwrap` 을 안전한 자리에서만 쓰는 감각을 익힌다.

## 핵심 개념

### 1) 두 타입의 정의

```rust
enum Option<T> { None, Some(T) }
enum Result<T, E> { Ok(T), Err(E) }
```

- `Option<T>` — 값이 있을 수도, 없을 수도 있는 자리.
- `Result<T, E>` — 성공·실패가 갈리는 자리. 실패의 이유까지 표현해야 할 때.

### 2) 가장 흔한 처리 — `match` / `if let`

```rust
match parsed {
    Ok(n) => println!("성공: {n}"),
    Err(e) => println!("실패: {e}"),
}
```

### 3) Option 메서드 표

| 메서드 | 동작 | 예 |
|--------|------|----|
| `map(\|x\| ...)` | Some 일 때 값 변환 | `Some(2).map(\|x\| x*10)` → `Some(20)` |
| `and_then(\|x\| ...)` | Some 일 때 또 다른 Option 반환 | 체이닝 |
| `unwrap_or(default)` | None 일 때 기본값 | `None.unwrap_or(-1)` → `-1` |
| `unwrap_or_else(\|\| ...)` | None 일 때 클로저로 계산 | |
| `ok_or(err)` | Option → Result | `Some(1).ok_or("없음")` → `Ok(1)` |
| `is_some()`, `is_none()` | 존재 여부 | |

### 4) Result 메서드 표

| 메서드 | 동작 |
|--------|------|
| `map(\|x\| ...)` | Ok 값 변환 |
| `map_err(\|e\| ...)` | Err 값 변환 — 에러 타입 변환에 유용 |
| `ok()` | Result → Option (Err → None) |
| `err()` | Err 값을 Option 으로 |
| `unwrap_or(default)` | Err 일 때 기본값 |
| `expect(msg)` | Err 일 때 메시지와 함께 패닉 |
| `?` | Err 를 호출자로 전파 (16편) |

### 5) `unwrap` 과 `expect` 는 언제?

> 명백히 실패할 수 없는 자리 — 또는 실패가 곧 프로그램 종료여도 무방한 자리 — 에서만 사용. 사용자 입력·파일 I/O·네트워크 결과에는 `?` 또는 `match` 가 정답.

`expect("이 자리는 항상 Ok")` 처럼 **왜 안전한지** 를 메시지에 남겨 두면 좋습니다.

## 예제로 보기

### 예제 1 — `ex01_parse.rs` : parse 결과 처리

```rust
// 15편 예제 1: str.parse 의 Result 처리
//
// `parse::<i32>()` 는 `Result<i32, ParseIntError>` 를 돌려준다.
// 성공·실패를 호출자가 명시적으로 처리해야 한다.

fn main() {
    let inputs = ["42", "오류", "  7  ", "-3"];

    for s in inputs {
        // 공백 제거 후 parse
        let parsed = s.trim().parse::<i32>();

        match parsed {
            Ok(n) => println!("'{s}' → {n}"),
            Err(e) => println!("'{s}' → 실패: {e}"),
        }
    }

    // unwrap_or: 실패 시 기본값
    let n = "abc".parse::<i32>().unwrap_or(0);
    println!("unwrap_or 결과 = {n}");
}
```

### 예제 2 — `ex02_option_methods.rs` : Option 메서드

```rust
// 15편 예제 2: Option 메서드 모음
//
// map / and_then / unwrap_or / unwrap_or_else / ok_or

fn half(n: i32) -> Option<i32> {
    if n % 2 == 0 { Some(n / 2) } else { None }
}

fn main() {
    let v = Some(10);

    // map — Some 안의 값을 변환
    let plus_one = v.map(|x| x + 1);
    println!("map: {:?}", plus_one);

    // and_then — Some 일 때 다른 Option 반환 (체이닝)
    let chained = v.and_then(half).and_then(half);
    println!("and_then 두 번: {:?}", chained);

    // unwrap_or — None 일 때 기본값
    let backup = None::<i32>.unwrap_or(-1);
    println!("unwrap_or(-1) = {backup}");

    // ok_or — Option → Result 로 변환 (에러 메시지 추가)
    let r: Result<i32, &str> = Some(3).ok_or("값 없음");
    println!("ok_or = {:?}", r);
}
```

### 예제 3 — `ex03_result_methods.rs` : Result 메서드

```rust
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
```

## 자주 하는 실수

### Q. `unwrap_or` 와 `unwrap_or_else` 의 차이는?

A. `unwrap_or(default)` 는 기본값을 **미리 계산** 합니다 — Some/Ok 여도 기본값 식이 평가됩니다. 기본값 계산이 비싸면 `unwrap_or_else(|| 비싼식)` 로 None/Err 일 때만 평가되게 하세요.

### Q. `match` 와 `map` 중 어느 쪽이 좋나요?

A. 단순히 "Ok/Some 안의 값을 변환" 만 한다면 `map` 한 줄이 깔끔합니다. 분기마다 사이드 이펙트(출력 등)가 다르면 `match` 가 더 명확합니다.

### Q. `?` 와 `unwrap` 의 차이?

A. `?` 는 Err/None 일 때 **호출자에게 전파**, `unwrap` 은 **즉시 패닉**. 라이브러리 코드는 거의 항상 `?`. 입문 단계의 작은 예제에서는 `unwrap` 으로 빨리 돌려 보고, 정식 코드에 옮길 때 `?` 로 바꾸는 흐름이 자연스럽습니다.

### Q. `expect` 메시지에는 뭐라 적나요?

A. "왜 이 자리에서 Err 가 나올 수 없는지" 를 적어 두면 다음 사람이 이해하기 좋습니다. 예: `.expect("입력은 비어 있을 수 없다 — 위에서 검증함")`.

## 정리

- 실패 가능성은 `Result<T, E>`, 부재 가능성은 `Option<T>` 로 타입에 명시.
- 변환·체이닝은 `map`, `and_then`, `map_err` 같은 메서드로.
- 기본값으로 회복은 `unwrap_or` / `unwrap_or_else`.
- `unwrap`/`expect` 는 안전 명백한 자리에서만, 그 외에는 `match`/`if let`/`?`.

## 직접 해 보기

`homework/` 폴더의 과제를 풀어 보세요. 정답은 `homework/answer/` 에 있습니다.

## 다음 단원

[16. `?` 연산자와 에러 전파](../16_물음표_연산자/README.md) — 호출자로 에러를 깔끔하게 전파하는 핵심 문법을 배웁니다.
