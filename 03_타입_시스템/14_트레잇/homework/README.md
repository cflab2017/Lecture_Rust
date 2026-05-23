# 과제 - 14. 트레잇

## 문제 1 — `Greet` 트레잇 두 타입 구현
- 파일명: `homework_01/src/main.rs`
- 핵심 개념: trait 정의 + 다중 구현

### 요구사항
- 트레잇 `Greet` 를 정의한다.
  - `fn name(&self) -> &str;` (시그니처만)
  - `fn hello(&self) -> String { format!("안녕, {}!", self.name()) }` (기본 메서드)
- 구조체 `Person { name: String }` 과 `Dog { name: String }` 를 정의하고 둘 다 `Greet` 를 구현한다.
  - `Person` 은 기본 메서드 그대로 사용.
  - `Dog` 는 `hello` 를 오버라이드해 `"{name}: 멍멍!"` 로 돌려준다.
- main 에서 두 인스턴스의 `hello()` 를 출력한다.

### 예상 출력
```
안녕, 지수!
초코: 멍멍!
```

### 힌트
- `impl Greet for Person { fn name(&self) -> &str { &self.name } }`.
- Dog 는 `fn hello(&self) -> String { format!("{}: 멍멍!", self.name) }` 도 정의.

## 문제 2 — trait object 컬렉션
- 파일명: `homework_02/src/main.rs`
- 핵심 개념: `Vec<Box<dyn Trait>>`

### 요구사항
- 트레잇 `Speak { fn speak(&self) -> String; }`.
- 두 구조체 `Cat`, `Cow` 가 각각 `"야옹"`, `"음매"` 를 돌려주도록 구현.
- `Vec<Box<dyn Speak>>` 에 두 인스턴스를 넣고 for 루프로 출력한다.

### 예상 출력
```
야옹
음매
```

### 힌트
- `let animals: Vec<Box<dyn Speak>> = vec![Box::new(Cat), Box::new(Cow)];`
- `for a in &animals { println!("{}", a.speak()); }`.

## 정답 확인

직접 풀어 본 후 [`answer/`](./answer/) 폴더의 정답과 비교해 보세요.
