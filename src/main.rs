use chrono::Local;

// https://www.w3.org/TR/xml-entity-names/025.html
const DIGITS : [[&str; 11]; 7] = [
    ["┏━┓ ","  ╻  "," ┏━┓ ", " ┏━┓ "," ╻ ╻ "," ┏━┓ "," ┏   "," ┏━┓ "," ┏━┓ "," ┏━┓ ","   "],
    ["┃ ┃ ","  ┃  ","   ┃ ", "   ┃ "," ┃ ┃ "," ┃   "," ┃   ","   ┃ "," ┃ ┃ "," ┃ ┃ "," ╻ "],
    ["┃ ┃ ","  ┃  ","   ┃ ", "   ┃ "," ┃ ┃ "," ┃   "," ┃   ","   ┃ "," ┃ ┃ "," ┃ ┃ ","   "],
    ["┃ ┃ ","  ┃  "," ┏━┛ ", " ┣━┫ "," ┗━┫ "," ┗━┓ "," ┣━┓ ","   ┃ "," ┣━┫ "," ┗━┫ ","   "],
    ["┃ ┃ ","  ┃  "," ┃   ", "   ┃ ","   ┃ ","   ┃ "," ┃ ┃ ","   ┃ "," ┃ ┃ ","   ┃ ","   "],
    ["┃ ┃ ","  ┃  "," ┃   ", "   ┃ ","   ┃ ","   ┃ "," ┃ ┃ ","   ┃ "," ┃ ┃ ","   ┃ "," ╹ "],
    ["┗━┛ ","  ╹  "," ┗━━ ", " ┗━┛ ","   ╹ "," ┗━┛ "," ┗━┛ ","   ╹ "," ┗━┛ "," ┗━┛ ","   "],
];

fn main() {
    loop {
        // time object with date and time and all that
        let t = Local::now();

        // format this time into a string with only hours mins seconds
        let time = t.format("%H:%M:%S").to_string();

        for row in &DIGITS {
            for c in time.chars() {
                let col: usize = match c {
                    '0'..='9' => c.to_digit(10).unwrap(),
                    ':' => 10,
                    _ => panic!("got some other char I was not expecting")
                }.try_into().unwrap(); // these two convert u32 from match into usize for indexing

                print!("{} ", row[col]);
            }
            println!();
        }

        // sleep the thread for some number or miliseconds
        let sleep = 999;
        std::thread::sleep(std::time::Duration::from_millis(sleep));
    }
}

// fn main(){
//     let s = String::from("123456789");

//     for c in s.chars() {
//         println!("normal char:{} | char as usize:{} | char as usize minus zero:{}",
//             c,
//             c as usize,
//             c as usize - '0' as usize
//         );

//         println!("{}", '0' as usize);
//     }
// }