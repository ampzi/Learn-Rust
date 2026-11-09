//! Lesson 2 — structs, methods, enums and pattern matching.
//!
//! * a `struct` groups named fields,
//! * `impl` blocks attach methods: `&self` borrows, `self` consumes,
//! * an `enum` is a type with several shapes — `match` forces you to handle
//!   every one of them, which is how Rust turns "forgot a case" into a compile error.

use std::fmt;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Niche {
    Gaming,
    Food,
    Lifestyle,
}

impl Niche {
    pub fn label(self) -> &'static str {
        match self {
            Niche::Gaming => "gaming",
            Niche::Food => "food",
            Niche::Lifestyle => "lifestyle",
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct Creator {
    pub name: String,
    pub followers: u32,
    pub niche: Niche,
}

impl Creator {
    /// Associated function (no `self`) — the idiomatic constructor.
    pub fn new(name: impl Into<String>, followers: u32, niche: Niche) -> Self {
        Self { name: name.into(), followers, niche }
    }

    /// `&self`: borrows, so the caller keeps the value afterwards.
    pub fn tier(&self) -> &'static str {
        match self.followers {
            0..=10_000 => "micro",
            10_001..=100_000 => "mid",
            _ => "macro",
        }
    }

    /// `self`: consumes the value, useful for the final conversion.
    pub fn into_audience(self) -> String {
        format!("{} ({}, {})", self.name, self.niche.label(), self.tier())
    }
}

impl fmt::Display for Creator {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{} — {} followers", self.name, self.followers)
    }
}

#[derive(Debug, PartialEq)]
pub enum Application {
    Pending,
    Accepted { budget: u32 },
    Rejected(String),
}

impl Application {
    pub fn status(&self) -> String {
        match self {
            Application::Pending => "waiting".to_string(),
            Application::Accepted { budget } => format!("accepted, budget {budget}"),
            Application::Rejected(reason) => format!("rejected: {reason}"),
        }
    }
}

pub fn demo() {
    let mut creators = vec![
        Creator::new("Sobhan", 8_400, Niche::Gaming),
        Creator::new("Sara", 54_000, Niche::Lifestyle),
        Creator::new("Farnaz", 320_000, Niche::Food),
    ];
    creators.sort_by_key(|c| c.followers);

    for creator in &creators {
        println!("2a. {creator:<28} tier={}", creator.tier());
    }

    let applications = [
        Application::Accepted { budget: 2_500_000 },
        Application::Pending,
        Application::Rejected("budget cut".to_string()),
    ];

    for (creator, application) in creators.iter().zip(&applications) {
        println!("2b. {} => {}", creator.name, application.status());
    }

    // `if let` when a single variant matters.
    for application in &applications {
        if let Application::Accepted { budget } = application {
            println!("2c. won a campaign worth {budget}");
        }
    }

    // Consuming method: the Creator is moved out of the Vec here.
    let audience = creators.remove(0).into_audience();
    println!("2d. audience: {audience} (creators left: {})", creators.len());
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn tier_boundaries_are_inclusive() {
        assert_eq!(Creator::new("a", 10_000, Niche::Food).tier(), "micro");
        assert_eq!(Creator::new("b", 10_001, Niche::Food).tier(), "mid");
        assert_eq!(Creator::new("c", 100_000, Niche::Food).tier(), "mid");
        assert_eq!(Creator::new("d", 100_001, Niche::Food).tier(), "macro");
    }

    #[test]
    fn display_and_display_impl() {
        let creator = Creator::new("Sara", 54_000, Niche::Lifestyle);
        assert_eq!(creator.to_string(), "Sara — 54000 followers");
    }

    #[test]
    fn enums_carry_data() {
        let accepted = Application::Accepted { budget: 900 };
        assert_eq!(accepted.status(), "accepted, budget 900");
        assert_ne!(accepted, Application::Pending);
    }

    #[test]
    fn consuming_method_builds_a_label() {
        let creator = Creator::new("Sobhan", 8_400, Niche::Gaming);
        assert_eq!(creator.into_audience(), "Sobhan (gaming, micro)");
    }
}
