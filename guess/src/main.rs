// 宣告使用外部的函式庫 rand
// Rust 若要引用外部的函式庫必須要這樣宣告
extern crate rand;

// 引入需要的函式
// 很多套件都會提供個叫 prelude 的模組，讓你可以快速的引入必要的函式
use rand::prelude::*;

// 這是標準輸入
use std::io::stdin;

// 程式進入點
fn main() {
    // Rust 中宣告變數的關鍵字為 let 只要使用 let 加上變數名稱就可以了
    // Rust 會根據初始值自動推導型態，所以大部份的情況下並不需要寫型態
    // 這邊用從 rand 引入的函式產生了一個範圍在 1~100 的亂數
    let ans = thread_rng().gen_range(1, 100);

    // Rust 中要求如果你之後要修改一個變數就必須像這樣，加上 mut 關鍵字
    // 來標記這個變數是可以修改的，否則預設會是唯讀的
    let mut upper_bound = 100;
    let mut lower_bound = 1;

    // Rust 的無窮迴圈， Rust 中共有三種迴圈，下面會介紹另外兩種 while 與 for
    loop {
        // 建立一個字串來存放使用者輸入， String 是 Rust 中的字串型態，其大小會自動成長
        // 就像 C++ 的 string 一樣
        let mut input = String::new();

        // Rust 的格式化字串使用的是 {}
        println!(
            "Answer is between {}~{}, please input a number",
            lower_bound, upper_bound
        );

        // 如果要讓一個函式能夠修改變數，就必須把變數以可變的參考 (&mut) 傳進去
        // read_line 傳回值為 Result ，這是 Rust 中代表正確與否的型態， Rust 中並沒有例外
        // 呼叫 Result 上的 expect 會讓在結果為錯誤 (Err) 時印出訊息，並結束程式
        stdin().read_line(&mut input).expect("Fail to read input");

        // 這邊把輸入結果去除掉結尾的換行後 parse 成數字格式
        // 同時這也是 Rust 一個有趣的地方， Rust 允許你重覆使用同一個變數名稱
        // 只是當你重新用 let 宣告變數時，前一個變數也就沒辦法再使用了
        // parse 的回傳值也是 Result 只是這次我們不再使用 expect 在錯誤時結束
        // 而是使用 match 檢查回傳值
        let input = match input.trim().parse::<i32>() {
            // Ok 代表的是正確，同時它會包含我們需要的結果
            // 因此這邊把值拿出來後回傳
            Ok(val) => val,
            // Err 則是錯誤，這邊我們提示使用者要輸入數字並結束這次迴圈的執行
            Err(_) => {
                println!("Please input a number!!!");
                continue;
            }
        };

        // Rust 的條件式都不需要括號，這跟 C 不一樣
        if input == ans {
            println!("You got it!");
            break;
        } else if input > ans {
            upper_bound = input;
        } else {
            lower_bound = input;
        }
    }
}
