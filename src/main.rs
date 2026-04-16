use std::str::FromStr;

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

enum Classification { Purchase, Sell }

fn main_screen(sales: &mut u32, cost: &mut u32) {
    loop {
        match input::<u32>("1.구매 2.판매 3.현매출 4.나가기\n") {
            1u32 => *cost += trade(Classification::Purchase),
            2u32 => *sales += trade(Classification::Sell),
            3u32 => println!("현  매출은 {}원, 원가는 {}원, 순이익은 {}원입니다.", *sales, *cost, *sales as i32 - *cost as i32),
            4u32 => return,
            _ => println!("올바른 값을 입력해주세요."),
        }
    }
}

fn input<T: FromStr>(prompt: &str) -> T {
    loop {
        let mut input_value: String = String::new();
        print!("{}", prompt);
        match std::io::stdin().read_line(&mut input_value) {
            Ok(_) => match input_value.trim().parse::<T>() {
                    Ok(return_value) => return return_value,
                    Err(_) => println!("올바른 값을 입력해주세요."),
            }
            Err(_) => println!("입력이 실패하였습니다. 이 문제가 지속된다면 관리자에게 문의하세요."),
        }
    }
}

fn trade(classification: Classification) -> u32 {
    let (action, price_fn): (&str, fn(&Item) -> u32) = match classification {
        Classification::Purchase => ("구매", |i: &Item| i.purchase_price),
        Classification::Sell => ("판매", |i: &Item| i.sell_price),
    };
    for i in &ITEMS {
        println!("{} : {}원", i.name, price_fn(i));
    }
    let item_name = input::<String>(&format!("어떤 것을 {}하시겠습니까?", action));
    match ITEMS.iter().find(|i| i.name == item_name) {
        Some(item) => price_fn(item) * input::<u32>(&format!("얼마나 {}하시겠습니까?", action)),
        None => { eprintln!("{}는 존재하지 않는 상품입니다.", item_name); 0 },
    }
}

fn main() {
    let mut sales: u32 = 0;
    let mut cost: u32 = 0;
    main_screen(&mut sales, &mut cost);
    println!("영업을 종료합니다. 매출은 {}원, 원가는 {}원, 순이익은 {}원입니다.", sales, cost, sales as i32 - cost as i32);
}