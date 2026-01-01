//! # Luhn Algorithm
//!
//! Luhn algorithm is known as modulus 10 algorithm, that is used to validate
//! different identification numbers.
//!
//! Example: the following algorithm describes validating credit card number.
//!
//! ```text
//! function isValid(cardNumber[1..length])
//!     sum := 0
//!     parity := length mod 2
//!     for i from 1 to (length - 1) do
//!         if i mod 2 == parity then
//!             sum := sum + cardNumber[i]
//!         elseif cardNumber[i] > 4 then
//!             sum := sum + 2 * cardNumber[i] - 9
//!         else
//!             sum := sum + 2 * cardNumber[i]
//!         end if
//!     end for
//!     return cardNumber[length] == ((10 - (sum mod 10)) mod 10)
//! end function
//! ```
//!
//! for more details, check <https://en.wikipedia.org/wiki/Luhn_algorithm>.

use core::num;

struct CreditCard {
    number: String,
}
impl CreditCard {
    fn new(number: String) -> Self {
        return Self { number };
    }
    fn is_valid(&self) -> bool {
        println!("Input: {:?}", &self.number);
        let number = self
            .number
            .replace(" ", "")
            .chars()
            .map(|c| c.to_digit(10).unwrap())
            .enumerate()
            .map(|(idx, num)| {
                if idx % 2 == 0 {
                    if num * 2 < 9 { num * 2 } else { num * 2 - 9 }
                } else {
                    num
                }
            })
            .collect::<Vec<u32>>()
            .iter()
            .sum::<u32>()
            % 10;

        return number == 0;
    }
}

fn main() {
    let card = CreditCard::new("4539 3195 0343 6467".to_owned());
    println!("valid: {}", card.is_valid())
}
