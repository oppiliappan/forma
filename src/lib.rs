#[cfg(test)]
mod tests {
    #[test]
    fn prints_stuff() {
        use super::*;
        radix_fmt("asdf", 4);
    }
}

use std::fmt::{Display, Formatter, Result};

static TABLE: [char; 36] = ['0', '1', '2', '3', '4', '5', '6', '7', '8', '9', 'A', 'B', 'C', 'D', 'E', 'F', 'G', 'H', 'I', 'J', 'K', 'L', 'M', 'N', 'O', 'P', 'Q', 'R', 'S', 'T', 'U', 'V', 'W', 'X', 'Y', 'Z'];

pub fn radix_fmt(number: &str, obase: usize) {
    if obase > 36 {
        panic!("LOOL");
    }
    let parts: Vec<&str> = number.split('.').collect();
    println!("{}.{}", format_integral("25", 5), format_fract(".225", 5));
}

fn format_integral(n: &str, obase: usize) -> String {
    let mut number = n.parse::<usize>().unwrap();
    let mut obase_int = String::new();
    while number >= obase {
        let letter = (number % obase ) as usize;
        obase_int.push(TABLE[letter]);
        number /= obase;
    }
    obase_int.push(TABLE[number as usize]);
    let obase_int = obase_int.chars().rev().collect::<String>();
    return obase_int
}

fn format_fract(n: &str, obase: usize) -> String {
    let mut fract = n.parse::<f64>().unwrap();
    let mut obase_fract = String::new();
    let mut i = 0;
    loop {
        fract *= obase as f64;
        obase_fract.push(TABLE[fract.trunc() as usize]);
        i += 1;
        if fract.fract() == 0. || i >= 15 {
            break;
        }
        fract = fract.fract();
    }
    return obase_fract
}
