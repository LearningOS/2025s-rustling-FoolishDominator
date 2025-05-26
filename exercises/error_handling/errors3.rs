/*
 * @Author: FoolishDominator 1340995873@qq.com
 * @Date: 2025-05-16 16:17:02
 * @LastEditors: FoolishDominator 1340995873@qq.com
 * @LastEditTime: 2025-05-25 16:05:20
 * @FilePath: /2025s-rustling-FoolishDominator/exercises/error_handling/errors3.rs
 * @Description: 这是默认设置,请设置`customMade`, 打开koroFileHeader查看配置 进行设置: https://github.com/OBKoro1/koro1FileHeader/wiki/%E9%85%8D%E7%BD%AE
 */
// errors3.rs
//
// This is a program that is trying to use a completed version of the
// `total_cost` function from the previous exercise. It's not working though!
// Why not? What should we do to fix it?
//
// Execute `rustlings hint errors3` or use the `hint` watch subcommand for a
// hint.

use std::num::ParseIntError;

fn main() {
    let mut tokens = 100;
    let pretend_user_input = "8";

    let cost = total_cost(pretend_user_input);
    match cost {
        Ok(cost) => {
            if cost > tokens {
                println!("You can't afford that many!");
            } else {
                tokens -= cost;
                println!("You now have {} tokens.", tokens);
            }
        }
        Err(e) => {
            println!("Error: {}", e);
        }
    }
}

pub fn total_cost(item_quantity: &str) -> Result<i32, ParseIntError> {
    let processing_fee = 1;
    let cost_per_item = 5;
    let qty = item_quantity.parse::<i32>()?;

    Ok(qty * cost_per_item + processing_fee)
}
