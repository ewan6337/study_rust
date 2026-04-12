use std::str::FromStr;
use std::io::Write;

struct Item {
    name: &'static str,
    purchase_price: u32,
    sell_price:u32,
}
enum Classification { Purchase, Sell }

const ITEMS: [Item; 6] = [
    Item { name: "캔커피", purchase_price: 500, sell_price: 1800 },
    Item { name: "삼각김밥", purchase_price: 900, sell_price: 1400 },
    Item { name: "바나나우유", purchase_price: 800, sell_price: 1800},
    Item { name: "도시락", purchase_price: 3500, sell_price: 4000 },
    Item { name: "콜라", purchase_price: 700, sell_price: 1500 },
    Item { name: "새우깡", purchase_price: 1000, sell_price: 2000 },
];

fn main_screen(total_price: &mut i32) {
    loop {
        println!("1.구매 2.판매 3.현 매출 4.나가기");
        match input::<u32>(None) {
            1u32 => *total_price -= trade(Classification::Purchase),
            2u32 => *total_price += trade(Classification::Sell),
            3u32 => println!("{}", *total_price),
            4u32 => return,
            _ => println!("올바른 값을 입력해주세요."),
        }
    }
}

fn input<T: FromStr>(prompt: Option<&str>) -> T {
    let mut input_value: String = String::new();
    loop {
        input_value.clear();

        if let Some(p) = prompt {
            print!("{}", p);
            std::io::stdout().flush().unwrap();
        }

        match std::io::stdin().read_line(&mut input_value) {
            Ok(_) => match input_value.trim().parse::<T>() {
                    Ok(value) => return value,
                    Err(_) => println!("올바른 값을 입력해주세요."),
            }
            Err(_) => println!("입력이 실패하였습니다. 다시 입력해주세요."),
        }
    }
}

fn trade (classification_p_s: Classification) -> i32 {
    
}

fn print_item_list(list: &[Item] , classification_p_s: Classification) -> &Item {
    loop {
        for i in &list { println!("{}: {}원", i.name, return_price(i, classification_p_s)) }
        let action: String = match classification_p_s {
            Classification::Purchase => "구매".to_string(),
            Classification::Sell => "판매".to_string(),
        };
        print!("어떤 것을 {}하시겠습니까?", action);
        std::io::stdout().flush().unwrap();
    }
}

fn select_item(list: &[Item], classification_p_s: Classification) -> &Item {
    
}

fn return_price(list: &Item, classification_p_s: Classification) -> u32 {
    match classification_p_s {
        Classification::Purchase => return list.purchase_price,
        Classification::Sell => return list.sell_price,
    }
}

fn main() {
    let mut total_price: i32 = 0;
    main_screen(&mut total_price);
}