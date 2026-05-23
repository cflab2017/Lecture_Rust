# 과제 - 21. 동시성

## 문제 1 — 네 스레드로 1..=100 합산
- 파일명: `homework_01/src/main.rs`
- 핵심 개념: `thread::spawn`, `join`, 작업 분할

### 요구사항
- 1 부터 100 까지의 합을 네 스레드로 나누어 계산한다.
  - 스레드 1: 1..=25, 스레드 2: 26..=50, 스레드 3: 51..=75, 스레드 4: 76..=100
- 각 스레드는 자기 구간의 합을 반환한다(`thread::spawn` + 마지막 표현식).
- main 에서 네 결과를 모아 총합을 출력한다.

### 예상 출력
```
1..=100 의 총합 = 5050
```

### 힌트
- `let h = thread::spawn(move || (start..=end).sum::<i32>());`
- 핸들 4 개를 Vec 에 모아 두고 `for h in handles { total += h.join().unwrap(); }`.

## 문제 2 — 채널로 받은 단어들을 모아 출력
- 파일명: `homework_02/src/main.rs`
- 핵심 개념: `mpsc::channel`, `clone`

### 요구사항
- 두 송신 스레드가 각각 `["hello", "world"]`, `["from", "rust"]` 를 채널로 보낸다.
- 메인 스레드는 모든 메시지를 받아 `Vec<String>` 으로 모은 뒤 길이를 출력한다.

### 예상 출력
```
받은 단어 수: 4
```

### 힌트
- `let (tx, rx) = mpsc::channel::<String>();`
- 두 번째 송신자는 `tx.clone()`.
- `let words: Vec<String> = rx.iter().collect();`.

## 정답 확인

직접 풀어 본 후 [`answer/`](./answer/) 폴더의 정답과 비교해 보세요.
