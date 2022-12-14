use std::fs;
use std::cmp::Ordering;

const DIV_1: &str = "[[2]]";
const DIV_2: &str = "[[6]]";

enum Item {
    List(Vec<Item>),
    Number(u8),
}

impl Item {
    fn to_string(&self) -> String {
        match self {
            Item::Number(n) => format!("{n}"),
            Item::List(l) => {
                let items = l.iter().map(|i| i.to_string()).collect::<Vec<String>>().join(",");
                format!("[{}]", items)
            }
        }
    }

    fn cmp(&self, other: &Item) -> Ordering {
        match (self, other) {
            (Item::Number(a), Item::Number(b)) => a.cmp(b),
            (Item::Number(a), Item::List(_)) => Item::List(vec![Item::Number(a.clone())]).cmp(other),
            (Item::List(_), Item::Number(b)) => self.cmp(&Item::List(vec![Item::Number(b.clone())])),
            (Item::List(a), Item::List(b)) => {
                let (mut a, mut b) = (a.iter(), b.iter());
                loop {
                    match (a.next(), b.next()) {
                        (None, None) => return Ordering::Equal, // two lists of equal length and values
                        (None, _) => return Ordering::Less,
                        (_, None) => return Ordering::Greater,
                        (Some(a), Some(b)) => match a.cmp(b) {
                            Ordering::Equal => {}
                            ord => return ord,
                        },
                    }
                }
            }
        }
    }
}

fn main() {
    let input = fs::read_to_string("input.txt").expect("Should have been able to read the input");

    println!("{}", part_one(&input));
    println!("{}", part_two(&input));
}

fn part_one(input: &str) -> usize {
    let pairs: Vec<&str> = input.split("\n\n").collect();
    let mut correct_order = 0;
    for (i, pair) in pairs.iter().enumerate() {
        let (left, right) = pair.split_once("\n").unwrap();
        let left = parse(left);
        let right = parse(right);

        correct_order = if left.cmp(&right) != Ordering::Greater { correct_order + i + 1 } else { correct_order }
    }

    correct_order
}

fn part_two(input: &str) -> usize {
    let mut input: String = input.replace("\n\n", "\n");
    input.push_str("\n");
    input.push_str(DIV_1);
    input.push_str("\n");
    input.push_str(DIV_2);

    let pairs = input.split("\n");
    let mut pairs : Vec<Item> = pairs.map(|p| parse(p)).collect();
    pairs.sort_by(|a, b| a.cmp(b));

    let mut distress = 1;
    for (i, p) in pairs.iter().enumerate() {
        let pair = p.to_string();
        if pair == DIV_1 || pair == DIV_2 {
            distress *= i+1
        }
    }

    distress
}


fn parse(list: &str) -> Item {
    let list = &list[1..list.len()-1];
    if list == "" {
        return Item::List(Vec::new());
    }
    let items: Vec<&str> = list.split(",").collect();

    let mut list = Vec::new();
    let mut i = 0;
    let mut bracket = 0;
    let mut sublist_start = 0;
    while i < items.len() {
        let (first, last) = {
            let chars: Vec<char> = items[i].chars().collect();
            (chars[0], chars[chars.len()-1])
        };
        if first == '[' {
            if bracket == 0 {
                sublist_start = i;
            }

            for item in items[i].chars() {
                if item == '[' {
                    bracket += 1
                }
            }
        }
        if last == ']' {
            for item in items[i].chars() {
                if item == ']' {
                    bracket -= 1
                }
            }

            if bracket == 0 {
                let sublist = items[sublist_start..i+1].join(",");
                list.push(parse(sublist.as_str()));
            }
        }

        if first != '[' && last != ']' {
            if bracket == 0 {
                let n = items[i].parse().unwrap();
                list.push(Item::Number(n));
            }
        }

        i+=1;
    }


    Item::List(list)
}
