//! # Luhn Algorithm
//!
//! Luhn algorithm is known as modulus 10 algorithm, that is used to validate
//! different identification numbers.
//!
//! Example: the following algorithm describes validating credit card number.
//!
//! # Algorithm
//!
//! 1. remove spaces from string if it contains any.
//!
//!    `4539 3195 0343 6467` -> `4539319503436467`
//!
//! 2. for every second digit (example 1st, 3rd, ...,) double the number.
//!
//!    `(4*2=8) 5 (3*2=6) 9  (3*2=6) 1 (9*2=18) 5 (0*2=0) 3 (4*2=8) 3 (6*2=12) 4 (6*2=12) 7`
//!
//! 3. if the doubled number is greater than 9, subtract it by 9 to get single digit.
//!
//!    `(8) 5 (6) 9  (6) 1 (18-9=9) 5 (0) 3 (8) 3 (12-9=3) 4 (12-9=3) 7`
//!
//!    = `(8) 5 (6) 9  (6) 1 (9) 5 (0) 3 (8) 3 (3) 4 (3) 7`
//!
//! 4. add all digits
//!
//!    `8 + 5 + 6 + 9 + 6 + 1 + 9 + 5 + 0 + 3 + 8 + 3 + 3 + 4 + 3 + 7`
//!    = `80`
//!
//! 5. if it is divisible by 10, it is valid card number else not.
//!    Here 80 %10 = 0 so it is a valid card number.
//!
//! for more details, check <https://en.wikipedia.org/wiki/Luhn_algorithm>.

use common::input;

struct CreditCard {
    number: String,
}
impl CreditCard {
    fn new(number: String) -> Self {
        return Self { number };
    }
    fn is_valid(&self) -> bool {
        if self.number.len() == 0 {
            return false;
        }
        let number = self
            .number
            .replace(" ", "") // remove spaces if available
            .chars()
            .map(|c| c.to_digit(10).unwrap()) // parse as u32
            .enumerate()
            .map(|(idx, num)| {
                // check alternating digits for validation
                if idx % 2 == 0 {
                    // if the double of the digit is greater than 9, subtract it by 9
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
    // println!("valid: {}", card.is_valid());
    let card_number =
        input("Enter card number (with or without spaces)\n[example: 4539 3195 0343 6467]: ");
    let card = CreditCard::new(card_number);
    println!(
        "The card number is {}.",
        if card.is_valid() { "valid" } else { " invalid" }
    )
}

#[cfg(test)]
mod tests {
    use crate::CreditCard;

    #[test]
    fn test_valid_cards() {
        assert_eq!(
            CreditCard::new("4242424242424242".to_owned()).is_valid(),
            true
        );
        assert_eq!(
            CreditCard::new("5105105105105100".to_owned()).is_valid(),
            true
        );
        assert_eq!(
            CreditCard::new("5555552500001001".to_owned()).is_valid(),
            true
        );
    }

    #[test]
    fn test_valid_cards_with_spaces() {
        assert_eq!(
            CreditCard::new("4242 4242 4242 4242".to_owned()).is_valid(),
            true
        );
        assert_eq!(
            CreditCard::new("5105 1051 0510 5100".to_owned()).is_valid(),
            true
        );
        assert_eq!(
            CreditCard::new("5555 5525 0000 1001".to_owned()).is_valid(),
            true
        );
    }

    #[test]
    fn test_invalid_cards() {
        assert_eq!(
            CreditCard::new("4242 4242 4242 4243".to_owned()).is_valid(),
            false
        );
        assert_eq!(
            CreditCard::new("5105 1051 0510 5101".to_owned()).is_valid(),
            false
        );
    }

    #[test]
    fn test_empty() {
        assert_eq!(CreditCard::new("".to_owned()).is_valid(), false);
    }
}
