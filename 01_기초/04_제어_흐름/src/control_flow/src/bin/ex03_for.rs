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
