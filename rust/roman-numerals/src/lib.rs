use std::fmt::{Display, Formatter, Result};

const CUTOFFS: [u32; 13] = [1,4,5,9,10,40,50,90,100,400,500,900,1000];
const SYMBOLS: [&str; 13] = ["I","IV","V","IX","X","XL","L","XC","C","CD","D","CM","M"];

/// function printRoman(number)
/// {
///   let num = [1,4,5,9,10,40,50,90,100,400,500,900,1000];
///   let sym = ["I","IV","V","IX","X","XL","L","XC","C","CD","D","CM","M"];
///   let i=12;
///   while(number>0) {
///     let div = Math.floor(number/num[i]);
///     number = number%num[i];
///     while(div--) {
///       document.write(sym[i]);
///     }
///     i--;
///   }
/// }

pub struct Roman {
    num: u32,
}

impl Display for Roman {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        let mut output = String::new();
        let mut num = self.num;
        let mut i = 12;
        while num > 0 {
            let mut even_div = num / CUTOFFS[i];
            num %= CUTOFFS[i];
            while even_div > 0 {
                output.push_str(SYMBOLS[i]);
                even_div -= 1;
            }
            if i > 0 { i -= 1 };
        };
        f.write_str(output.as_str())
    }
}

impl From<u32> for Roman {
    fn from(num: u32) -> Self {
        Roman { num }
    }
}
