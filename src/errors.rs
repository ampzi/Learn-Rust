//! Lesson 3 — `Option`, `Result`, the `?` operator and a custom error type.
//!
//! * `Option<T>`  = "there may be no value"      -> `Some(v)` / `None`
//! * `Result<T,E>` = "this may fail"             -> `Ok(v)`  / `Err(e)`
//! * `?` propagates the unhappy path and converts the error with `From`,
//! * recovering: `unwrap_or`, `map`, `and_then`, `if let`.

use std::error::Error;
use std::fmt;
use std::num::ParseIntError;

#[derive(Debug, PartialEq)]
pub enum WalletError {
    Zero,
    Overflow,
    NotEnough { balance: u64, requested: u64 },
}

impl fmt::Display for WalletError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            WalletError::Zero => write!(f, "amount must be greater than zero"),
            WalletError::Overflow => write!(f, "balance would overflow"),
            WalletError::NotEnough { balance, requested } => {
                write!(f, "not enough money: balance {balance}, requested {requested}")
            }
        }
    }
}

impl Error for WalletError {}

/// One error type for the whole operation, with `From` so `?` can build it.
#[derive(Debug)]
pub enum TopUpError {
    Parse(ParseIntError),
    Wallet(WalletError),
}

impl fmt::Display for TopUpError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            TopUpError::Parse(e) => write!(f, "not a number: {e}"),
            TopUpError::Wallet(e) => write!(f, "{e}"),
        }
    }
}

impl Error for TopUpError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        match self {
            TopUpError::Parse(e) => Some(e),
            TopUpError::Wallet(e) => Some(e),
        }
    }
}

impl From<ParseIntError> for TopUpError {
    fn from(e: ParseIntError) -> Self {
        TopUpError::Parse(e)
    }
}

impl From<WalletError> for TopUpError {
    fn from(e: WalletError) -> Self {
        TopUpError::Wallet(e)
    }
}

#[derive(Debug, Default)]
pub struct Wallet {
    balance: u64,
}

impl Wallet {
    pub fn balance(&self) -> u64 {
        self.balance
    }

    pub fn deposit(&mut self, amount: u64) -> Result<u64, WalletError> {
        if amount == 0 {
            return Err(WalletError::Zero);
        }
        // `checked_add` returns Option, `ok_or` turns None into an error.
        self.balance = self.balance.checked_add(amount).ok_or(WalletError::Overflow)?;
        Ok(self.balance)
    }

    pub fn withdraw(&mut self, amount: u64) -> Result<u64, WalletError> {
        if amount == 0 {
            return Err(WalletError::Zero);
        }
        if amount > self.balance {
            return Err(WalletError::NotEnough { balance: self.balance, requested: amount });
        }
        self.balance -= amount;
        Ok(self.balance)
    }

    /// Both `?` calls convert their error into `TopUpError` via `From`.
    pub fn top_up(&mut self, input: &str) -> Result<u64, TopUpError> {
        let amount: u64 = input.trim().parse()?;
        self.deposit(amount)?;
        Ok(self.balance)
    }
}

/// `Option` in a return type: "not every slice has an even number".
pub fn first_even(numbers: &[i32]) -> Option<i32> {
    numbers.iter().copied().find(|n| n % 2 == 0)
}

/// `?` works on `Option` too: the first `None` short-circuits the whole chain.
pub fn first_two_evens(numbers: &[i32]) -> Option<(i32, i32)> {
    let first = first_even(numbers)?;
    let index = numbers.iter().position(|&n| n == first)?;
    let second = first_even(&numbers[index + 1..])?;
    Some((first, second))
}

pub fn demo() {
    let mut wallet = Wallet::default();
    println!("3a. deposit(10_000)  -> {:?}", wallet.deposit(10_000));
    println!("3b. withdraw(4_000)  -> {:?}", wallet.withdraw(4_000));
    println!("3c. withdraw(99_999) -> {:?}", wallet.withdraw(99_999).unwrap_err());
    println!("3d. top_up(\"250\")    -> {:?}", wallet.top_up("250"));

    if let Err(err) = wallet.top_up("nope") {
        println!("3e. error: {err}");
        if let Some(source) = err.source() {
            println!("    caused by: {source}");
        }
    }

    let numbers = [3, 7, 8, 9, 12];
    println!("3f. first even in {numbers:?} = {:?}", first_even(&numbers));
    println!("3g. first two evens        = {:?}", first_two_evens(&numbers));

    let no_evens = first_even(&[1, 3, 5]);
    println!(
        "3h. first even in [1, 3, 5] = {}",
        no_evens.map(|n| n.to_string()).unwrap_or_else(|| "none".to_string())
    );

    println!("3i. final balance = {}", wallet.balance());
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn deposit_and_withdraw_track_the_balance() {
        let mut wallet = Wallet::default();
        assert_eq!(wallet.deposit(10_000), Ok(10_000));
        assert_eq!(wallet.withdraw(4_000), Ok(6_000));
        assert_eq!(wallet.balance(), 6_000);
    }

    #[test]
    fn withdraw_more_than_balance_fails_without_mutating() {
        let mut wallet = Wallet::default();
        wallet.deposit(500).unwrap();
        let err = wallet.withdraw(900).unwrap_err();
        assert_eq!(err, WalletError::NotEnough { balance: 500, requested: 900 });
        assert_eq!(wallet.balance(), 500, "failed withdraw keeps the balance");
        assert_eq!(err.to_string(), "not enough money: balance 500, requested 900");
    }

    #[test]
    fn zero_and_overflow_are_rejected() {
        let mut wallet = Wallet::default();
        assert_eq!(wallet.deposit(0), Err(WalletError::Zero));
        wallet.deposit(u64::MAX).unwrap();
        assert_eq!(wallet.deposit(1), Err(WalletError::Overflow));
    }

    #[test]
    fn question_mark_converts_the_error_type() {
        let mut wallet = Wallet::default();
        let err = wallet.top_up("abc").unwrap_err();
        assert!(matches!(err, TopUpError::Parse(_)));
        assert!(err.source().is_some(), "the original error is kept as source");

        let err = wallet.top_up("0").unwrap_err();
        assert!(matches!(err, TopUpError::Wallet(WalletError::Zero)));
    }

    #[test]
    fn option_helpers() {
        assert_eq!(first_even(&[3, 7, 8, 9]), Some(8));
        assert_eq!(first_even(&[1, 3, 5]), None);
        assert_eq!(first_two_evens(&[3, 7, 8, 9, 12]), Some((8, 12)));
        assert_eq!(first_two_evens(&[3, 8]), None, "the second even is missing");
    }
}
