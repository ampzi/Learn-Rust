//! Lesson 4 — traits, generics, iterators, closures and `HashMap`.
//!
//! * a trait is a shared contract, with optional default methods,
//! * generics let one function work for many types without losing type safety,
//! * iterator adapters (`map`, `filter`, `fold`) are lazy: nothing runs until
//!   the iterator is consumed by `collect`, `sum`, a `for` loop, ...

use std::collections::HashMap;

pub trait Summary {
    fn summarize(&self) -> String;

    /// Default method: implementors get it for free and may override it.
    fn headline(&self) -> String {
        format!(">> {}", self.summarize())
    }
}

pub struct Post {
    pub title: String,
    pub tags: Vec<String>,
}

impl Summary for Post {
    fn summarize(&self) -> String {
        format!("{} [{} tags]", self.title, self.tags.len())
    }
}

pub struct Campaign {
    pub name: String,
    pub budget: u64,
}

impl Summary for Campaign {
    fn summarize(&self) -> String {
        format!("{} ({} million)", self.name, self.budget / 1_000_000)
    }

    /// Override the default implementation.
    fn headline(&self) -> String {
        format!("*** {} ***", self.name)
    }
}

/// Generic + trait bound: works for anything comparable and cheap to copy.
pub fn largest<T: PartialOrd + Copy>(items: &[T]) -> Option<T> {
    let mut biggest = *items.first()?;
    for &item in &items[1..] {
        if item > biggest {
            biggest = item;
        }
    }
    Some(biggest)
}

/// `impl Trait` in argument position is sugar for a generic parameter.
pub fn print_summary(item: &impl Summary) {
    println!("4a. {}", item.headline());
}

/// Count how often each tag appears; `entry` inserts on first use.
pub fn tag_cloud(tags: &[&str]) -> HashMap<String, usize> {
    let mut counts = HashMap::new();
    for tag in tags {
        *counts.entry(tag.to_string()).or_insert(0) += 1;
    }
    counts
}

pub fn demo() {
    let post = Post {
        title: "Why UGC works".to_string(),
        tags: vec!["ugc".to_string(), "creator".to_string()],
    };
    let campaign = Campaign { name: "Point of Contact".to_string(), budget: 25_000_000 };

    print_summary(&post);
    print_summary(&campaign);

    println!("4b. largest of [4, 9, 2]     = {:?}", largest(&[4, 9, 2]));
    println!("4c. largest of ['a', 'z', 'k'] = {:?}", largest(&['a', 'z', 'k']));

    let followers = [500, 12_000, 88_000, 4, 320_000];
    let thousands: Vec<u32> =
        followers.iter().filter(|&&f| f >= 1_000).map(|&f| f / 1_000).collect();
    println!("4d. thousands of followers (>= 1k): {thousands:?}");

    // `fold` threads state through the whole sequence.
    let click_counts = [3, 12, 0, 11, 1, 7];
    let total = click_counts.iter().fold(0, |acc, &clicks| acc + clicks);
    println!("4e. total clicks = {total}");

    // A closure capturing a local by reference; no `move` needed here.
    let threshold = 5;
    let above: Vec<&i32> = click_counts.iter().filter(|&&c| c > threshold).collect();
    println!("4f. counts above {threshold}: {above:?}");

    let cloud = tag_cloud(&["ugc", "creator", "ugc", "brand", "ugc", "creator"]);
    let mut ranked: Vec<_> = cloud.iter().collect();
    ranked.sort_by(|a, b| b.1.cmp(a.1).then(a.0.cmp(b.0)));
    for (tag, count) in ranked {
        println!("4g. {tag:<10} {count}");
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn traits_share_a_default_method() {
        let post = Post { title: "UGC".to_string(), tags: vec!["a".to_string()] };
        assert_eq!(post.headline(), ">> UGC [1 tags]");

        let campaign = Campaign { name: "Launch".to_string(), budget: 2_000_000 };
        assert_eq!(campaign.headline(), "*** Launch ***", "override wins");
        assert_eq!(campaign.summarize(), "Launch (2 million)");
    }

    #[test]
    fn generics_work_for_several_types() {
        assert_eq!(largest(&[4, 9, 2]), Some(9));
        assert_eq!(largest(&['a', 'z', 'k']), Some('z'));
        assert_eq!(largest::<i32>(&[]), None, "empty input is handled by Option");
    }

    #[test]
    fn iterators_are_chainable() {
        let followers = [500, 12_000, 88_000, 4];
        let thousands: Vec<u32> =
            followers.iter().filter(|&&f| f >= 1_000).map(|&f| f / 1_000).collect();
        assert_eq!(thousands, vec![12, 88]);
    }

    #[test]
    fn hashmap_counts_entries() {
        let cloud = tag_cloud(&["ugc", "creator", "ugc"]);
        assert_eq!(cloud.get("ugc"), Some(&2));
        assert_eq!(cloud.get("creator"), Some(&1));
        assert_eq!(cloud.get("missing"), None);
    }
}
