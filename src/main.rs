use std::str::FromStr;
use std::io::Write;

struct Item {
    name: &'static str,
    purchase_price: u32,
    sell_price:u32,
}

const ITEMS: [Item; 6] = [
    Item { name: "캔커피", purchase_price: 500, sell_price: 1800 },
    Item { name: "삼각김밥", purchase_price: 900, sell_price: 1400 },
    Item { name: "바나나우유", purchase_price: 800, sell_price: 1800},
    Item { name: "도시락", purchase_price: 3500, sell_price: 4000 },
    Item { name: "콜라", purchase_price: 700, sell_price: 1500 },
    Item { name: "새우깡", purchase_price: 1000, sell_price: 2000 },
];

fn main_screen(total_price: &i32) {
    println!("1.구매 2.판매 3.현 매출 4.나가기");
}

fn input<T: FromStr>(prompt: &str) -> T {
    let mut input_value: String = String::new();
    loop {
        print!("{}", prompt);
        std::io::stdout().flush().unwrap();

        match std::io::stdin().read_line(&mut input_value) {
            Ok(_) => match input_value.trim().parse::<T>() {
                    Ok(value) => return value,
                    Err(_) => {
                        println!("올바른 값을 입력해주세요.");
                        input_value.clear();
                },
            },
            Err(_) => {
                println!("입력이 실패하였습니다. 다시 입력해주세요.");
                input_value.clear();
            },
        }
    }
}

fn main() {
    let mut total_price: i32 = 0;
    main_screen(&total_price);
}