/*
 * @Author: FoolishDominator 1340995873@qq.com
 * @Date: 2025-05-16 16:17:02
 * @LastEditors: FoolishDominator 1340995873@qq.com
 * @LastEditTime: 2025-06-06 00:34:25
 * @FilePath: /2025s-rustling-FoolishDominator/exercises/clippy/clippy2.rs
 * @Description: 这是默认设置,请设置`customMade`, 打开koroFileHeader查看配置 进行设置: https://github.com/OBKoro1/koro1FileHeader/wiki/%E9%85%8D%E7%BD%AE
 */
// clippy2.rs
//
// Execute `rustlings hint clippy2` or use the `hint` watch subcommand for a
// hint.

fn main() {
    let mut res = 42;
    let option = Some(12);
    // for x in option {
    //     res += x;
    // }
    if let Some(x) = option {
        res += x;
    }
    println!("{}", res);
}
