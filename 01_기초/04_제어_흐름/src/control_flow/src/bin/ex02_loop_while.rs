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
