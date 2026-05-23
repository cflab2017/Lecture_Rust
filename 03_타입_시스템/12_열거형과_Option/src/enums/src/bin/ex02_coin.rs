// 12편 예제 2: 데이터를 가지지 않는 단순 enum + 메서드
//
// match 의 전형적인 사용 예. 다음 편(13)에서 매칭을 자세히 다룹니다.

#[derive(Debug)]
enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter,
}

impl Coin {
    fn value_in_cents(&self) -> u32 {
        match self {
            Coin::Penny => 1,
            Coin::Nickel => 5,
            Coin::Dime => 10,
            Coin::Quarter => 25,
        }
    }
}

fn main() {
    let purse = [Coin::Penny, Coin::Nickel, Coin::Dime, Coin::Quarter, Coin::Dime];

    let total: u32 = purse.iter().map(|c| c.value_in_cents()).sum();
    println!("지갑 = {:?}", purse);
    println!("총합 = {total} 센트");
}
