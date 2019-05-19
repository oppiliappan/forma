#[cfg(test)]
mod tests {
    #[test]
    fn prints_stuff() {
        use super::*;
        println!("{}", radix_fmt("10", 2));
    }
}

use std::fmt::{Display, Formatter, Result};
use std::f64;

static TABLE: [char; 36] = ['0', '1', '2', '3', '4', '5', '6', '7', '8', '9',
                            'A', 'B', 'C', 'D', 'E', 'F', 'G', 'H', 'I', 'J', 'K', 'L', 'M',
                            'N', 'O', 'P', 'Q', 'R', 'S', 'T', 'U', 'V', 'W', 'X', 'Y', 'Z'];

// TODO: write this function, aka finish this library
pub fn radix_fmt(number: &str, obase: usize) -> String {
    if obase > 36 {
        panic!("LOOL");
    }
    match number {
        "inf"  => return "inf".into(),
        "-inf" => return "-inf".into(),
        "NaN"  => return "NaN".into(),
        _ => {},
    }

    return converted;
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
