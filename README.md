# rust-tiny-project

A deliberately small Rust project for learning the language by reading and running
short, commented lessons, plus a playable terminal football game. The lessons use
only the standard library; the game adds `crossterm` for raw keyboard input.

```
rust-tiny-project/
├── Cargo.toml
└── src/
    ├── main.rs            runner: picks which lesson to print
    ├── ownership.rs       1. ownership, borrowing, slices, lifetimes
    ├── types.rs           2. structs, methods, enums, pattern matching
    ├── errors.rs          3. Option, Result, `?`, custom error types
    ├── traits_iter.rs     4. traits, generics, iterators, closures, HashMap
    └── bin/football/
        ├── main.rs        terminal setup, key handling, game loop
        ├── game.rs        pitch physics, AI, clock, scoring (+ tests)
        └── render.rs      frame building with ANSI colours (+ tests)
```

## Run it

```bash
cd rust-tiny-project
cargo run             # all four lessons
cargo run -- 3        # only lesson 3
cargo run -- list     # table of contents
cargo test            # 30 unit tests
```

## Play football

```bash
cargo run --bin football             # play (needs a terminal of 76x25 or larger)
cargo run --bin football -- --demo   # simulate a full match headless, print frames
```

A two-minute match on a 74x20 pitch. You are `@`, your team-mate is `t`, the two
rivals are `x`, the keepers are `K`, the ball is `o`.

| key | action |
| --- | --- |
| `W A S D` / arrows | run |
| `SPACE` | shoot — aim is your running direction, bent towards the goal |
| `R` | restart the match |
| `Q` / `ESC` | quit |

How it is built, file by file:

* `game.rs` is pure simulation: a fixed 60 Hz tick, exponential ball drag, bounces off
the touchlines, a dribble that pushes the ball from your feet, one chasing rival plus
one covering rival, keepers that shuffle along the goal line, and a tiny xorshift PRNG
so no random crate is needed. Because it has no terminal code, every rule is testable:
`a_shot_can_find_the_net`, `the_ball_bounces_off_the_goal_line_outside_the_posts`,
`a_goal_freezes_play_before_the_next_kickoff`, and so on.
* `render.rs` draws a `Canvas` of `(char, Ink)` cells — pitch lines, penalty areas,
goal mouths, players — then serialises it once into a string of ANSI escapes.
`frame_plain` is the same picture without colour, which is what `--demo` prints and
what the renderer tests assert on.
* `main.rs` owns the terminal: raw mode, alternate screen, a non-blocking key loop and
a frame-accumulator that keeps the simulation at a fixed step even if drawing stutters.

Tuning knobs live at the top of `game.rs`: `PLAYER_SPEED`, `RIVAL_KICK_SPEED`,
`SHOOT_SPEED`, `BALL_DRAG`, `MATCH_SECONDS`.

## What each lesson shows

**1. Ownership and borrowing.** Moving a `String` invalidates the original binding,
while `i32` is `Copy` and survives. `&T` lets many readers share a value; `&mut T` is
exclusive. Slices borrow a range without copying. `longest<'a>` shows how a lifetime
ties the returned reference to its inputs.

**2. Structs and enums.** `Creator::new` is an associated function, `tier(&self)`
borrows, `into_audience(self)` consumes. `Niche` is a plain enum; `Application` shows
enums that carry data. `match` is exhaustive, `if let` handles a single variant.

**3. Errors without exceptions.** `Option<T>` for "maybe", `Result<T, E>` for "may
fail". `?` returns early and converts errors through `From`, which is why `top_up`
can combine a `ParseIntError` and a `WalletError` into one `TopUpError` while keeping
the original error as `source()`. `first_two_evens` shows `?` on `Option`.

**4. Traits and iterators.** `Summary` has a required method and a default
`headline()` that `Campaign` overrides. `largest<T: PartialOrd + Copy>` is generic.
`filter`/`map`/`fold` chain lazily, and `HashMap::entry` inserts a counter on first use.

## Try these next (small edits, big lessons)

1. Delete one `Niche` arm in `Niche::label` and run `cargo build` — read the
   non-exhaustive `match` error.
2. In `ownership::demo`, uncomment `println!("{s1}");` and watch the borrow checker
   explain the move.
3. Add a `wallet.withdraw(&mut wallet.balance)` style helper that returns `&mut u64`
   and try to also read `wallet.balance` while it is alive — the compile error is the
   whole lesson.
4. Add a `Variant` to `Application` (e.g. `Withdrawn`) and let the compiler list every
   `match` that must change.
5. Change `tag_cloud` to take `&[String]` instead of `&[&str]` — feel how `&str` vs
   `String` and `Deref` coercion behave.
