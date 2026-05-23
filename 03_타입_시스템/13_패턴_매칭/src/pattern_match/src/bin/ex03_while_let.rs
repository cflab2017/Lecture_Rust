// 13편 예제 3: while let — 패턴이 매칭되는 동안 반복
//
// Vec 의 pop() 처럼 Option 을 돌려주는 메서드와 자연스럽게 결합됩니다.

fn main() {
    let mut stack = vec![1, 2, 3, 4];

    // 스택이 빌 때까지 마지막 원소를 꺼낸다.
    while let Some(top) = stack.pop() {
        println!("꺼냄: {top}");
    }
    println!("최종 = {:?}", stack);
}
