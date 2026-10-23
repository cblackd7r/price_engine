//! Basic price engine.
//!
//! Takes three price inputs, one metric and a 4CP (4 Coincident Peak) flag,
//! validates them, and decides whether the controlled load should be ON or OFF.
//!
//! Usage:
//!     price_engine <price_zero> <price_one> <price_two> <metric_one> <is_4cp>
//!
//! Example:
//!     price_engine 5 3 2 4 false

use std::env;
use std::fmt;
use std::process::ExitCode;

// ---------------------------------------------------------------------------
// Limits
// ---------------------------------------------------------------------------

const PRICE_ZERO_RANGE: (u32, u32) = (1, 200);
const PRICE_ONE_RANGE: (u32, u32) = (1, 200);
const PRICE_TWO_RANGE: (u32, u32) = (1, 10);
const METRIC_ONE_RANGE: (u32, u32) = (1, 10);

/// Blended price above this turns the load OFF.
const BLENDED_PRICE_CEILING: u32 = 10;

// ---------------------------------------------------------------------------
// Inputs
// ---------------------------------------------------------------------------

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Inputs {
    pub price_zero: u32,
    pub price_one: u32,
    pub price_two: u32,
    pub metric_one: u32,
    /// True when the current interval is a likely 4CP peak.
    pub is_4cp: bool,
}

#[derive(Debug, PartialEq, Eq)]
pub enum InputError {
    Usage,
    NotANumber { name: &'static str, value: String },
    NotABool { name: &'static str, value: String },
    OutOfRange { name: &'static str, value: u32, min: u32, max: u32 },
}

impl fmt::Display for InputError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            InputError::Usage => write!(
                f,
                "usage: price_engine <price_zero> <price_one> <price_two> <metric_one> <is_4cp>"
            ),
            InputError::NotANumber { name, value } => {
                write!(f, "{name}: '{value}' is not a whole number")
            }
            InputError::NotABool { name, value } => {
                write!(f, "{name}: '{value}' must be 'true' or 'false'")
            }
            InputError::OutOfRange { name, value, min, max } => {
                write!(f, "{name}: {value} is out of range (expected {min} to {max})")
            }
        }
    }
}

fn parse_in_range(name: &'static str, raw: &str, (min, max): (u32, u32)) -> Result<u32, InputError> {
    let value = raw.trim().parse::<u32>().map_err(|_| InputError::NotANumber {
        name,
        value: raw.to_string(),
    })?;
    if !(min..=max).contains(&value) {
        return Err(InputError::OutOfRange { name, value, min, max });
    }
    Ok(value)
}

fn parse_bool(name: &'static str, raw: &str) -> Result<bool, InputError> {
    raw.trim().to_ascii_lowercase().parse::<bool>().map_err(|_| InputError::NotABool {
        name,
        value: raw.to_string(),
    })
}

impl Inputs {
    /// Parses and validates the five positional arguments (program name excluded).
    pub fn from_args(args: &[String]) -> Result<Self, InputError> {
        let [p0, p1, p2, m1, cp] = args else {
            return Err(InputError::Usage);
        };
        Ok(Inputs {
            price_zero: parse_in_range("price_zero", p0, PRICE_ZERO_RANGE)?,
            price_one: parse_in_range("price_one", p1, PRICE_ONE_RANGE)?,
            price_two: parse_in_range("price_two", p2, PRICE_TWO_RANGE)?,
            metric_one: parse_in_range("metric_one", m1, METRIC_ONE_RANGE)?,
            is_4cp: parse_bool("is_4cp", cp)?,
        })
    }
}

// ---------------------------------------------------------------------------
// Decision
// ---------------------------------------------------------------------------

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum State {
    On,
    Off,
}

impl fmt::Display for State {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            State::On => write!(f, "ON \u{1F7E2}"),
            State::Off => write!(f, "OFF \u{1F534}"),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Reason {
    FourCpPeak,
    PriceOneAtFloor,
    BlendedAboveCeiling(u32),
    BlendedWithinCeiling(u32),
}

impl fmt::Display for Reason {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Reason::FourCpPeak => write!(f, "4CP peak interval"),
            Reason::PriceOneAtFloor => write!(f, "price_one is at its floor (1)"),
            Reason::BlendedAboveCeiling(b) => {
                write!(f, "blended price {b} > {BLENDED_PRICE_CEILING}")
            }
            Reason::BlendedWithinCeiling(b) => {
                write!(f, "blended price {b} <= {BLENDED_PRICE_CEILING}")
            }
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Decision {
    pub state: State,
    pub reason: Reason,
}

/// Same formula the original used: `price_zero + (price_one * price_two) / 10`,
/// with integer division.
pub fn blended_price(i: &Inputs) -> u32 {
    i.price_zero + (i.price_one * i.price_two) / 10
}

/// The control rules. This reproduces every outcome the original program could
/// actually reach, without the unreachable branches.
pub fn decide(i: &Inputs) -> Decision {
    if i.is_4cp {
        return Decision { state: State::Off, reason: Reason::FourCpPeak };
    }
    // Kept from the original. Note: this turns the load OFF at the *lowest*
    // price_one, which looks backwards for a cost-driven controller.
    if i.price_one <= 1 {
        return Decision { state: State::Off, reason: Reason::PriceOneAtFloor };
    }
    let blended = blended_price(i);
    if blended > BLENDED_PRICE_CEILING {
        Decision { state: State::Off, reason: Reason::BlendedAboveCeiling(blended) }
    } else {
        Decision { state: State::On, reason: Reason::BlendedWithinCeiling(blended) }
    }
}

// ---------------------------------------------------------------------------
// Entry point
// ---------------------------------------------------------------------------

fn main() -> ExitCode {
    let args: Vec<String> = env::args().skip(1).collect();

    let inputs = match Inputs::from_args(&args) {
        Ok(i) => i,
        Err(e) => {
            eprintln!("error: {e}");
            return ExitCode::from(2);
        }
    };

    println!(
        "Inputs: price_zero={} price_one={} price_two={} metric_one={} is_4cp={}",
        inputs.price_zero, inputs.price_one, inputs.price_two, inputs.metric_one, inputs.is_4cp
    );

    let decision = decide(&inputs);
    println!("Decision: {} ({})", decision.state, decision.reason);

    ExitCode::SUCCESS
}

// ---------------------------------------------------------------------------
// Tests
// ---------------------------------------------------------------------------

#[cfg(test)]
mod tests {
    use super::*;

    fn args(v: &[&str]) -> Vec<String> {
        v.iter().map(|s| s.to_string()).collect()
    }

    fn inputs(p0: u32, p1: u32, p2: u32, m1: u32, cp: bool) -> Inputs {
        Inputs { price_zero: p0, price_one: p1, price_two: p2, metric_one: m1, is_4cp: cp }
    }

    // --- parsing / validation ---

    #[test]
    fn parses_valid_args() {
        let i = Inputs::from_args(&args(&["5", "3", "2", "4", "false"])).unwrap();
        assert_eq!(i, inputs(5, 3, 2, 4, false));
    }

    #[test]
    fn accepts_upper_limits() {
        assert!(Inputs::from_args(&args(&["200", "200", "10", "10", "TRUE"])).is_ok());
    }

    #[test]
    fn metric_one_validated_independently_of_price_zero() {
        // The original rejected this because it checked price_zero instead of metric_one.
        assert!(Inputs::from_args(&args(&["50", "3", "2", "4", "false"])).is_ok());
    }

    #[test]
    fn rejects_out_of_range() {
        let err = Inputs::from_args(&args(&["600", "600", "600", "600", "false"])).unwrap_err();
        assert_eq!(
            err,
            InputError::OutOfRange { name: "price_zero", value: 600, min: 1, max: 200 }
        );
        assert!(Inputs::from_args(&args(&["0", "3", "2", "4", "false"])).is_err());
        assert!(Inputs::from_args(&args(&["5", "3", "11", "4", "false"])).is_err());
        assert!(Inputs::from_args(&args(&["5", "3", "2", "200", "false"])).is_err());
    }

    #[test]
    fn rejects_bad_types_and_counts() {
        assert_eq!(Inputs::from_args(&args(&["5", "3"])), Err(InputError::Usage));
        assert!(matches!(
            Inputs::from_args(&args(&["x", "3", "2", "4", "false"])),
            Err(InputError::NotANumber { .. })
        ));
        assert!(matches!(
            Inputs::from_args(&args(&["5", "3", "2", "4", "maybe"])),
            Err(InputError::NotABool { .. })
        ));
    }

    // --- decision rules ---

    #[test]
    fn four_cp_always_off() {
        assert_eq!(decide(&inputs(1, 5, 1, 1, true)).state, State::Off);
    }

    #[test]
    fn price_one_floor_is_off() {
        assert_eq!(decide(&inputs(1, 1, 1, 1, false)).reason, Reason::PriceOneAtFloor);
    }

    #[test]
    fn low_blended_is_on() {
        // 5 + (3*2)/10 = 5
        let d = decide(&inputs(5, 3, 2, 4, false));
        assert_eq!(d, Decision { state: State::On, reason: Reason::BlendedWithinCeiling(5) });
    }

    #[test]
    fn ceiling_is_inclusive() {
        // 10 + (2*1)/10 = 10 -> ON
        assert_eq!(decide(&inputs(10, 2, 1, 1, false)).state, State::On);
        // 11 + 0 = 11 -> OFF
        assert_eq!(decide(&inputs(11, 2, 1, 1, false)).state, State::Off);
    }

    #[test]
    fn high_blended_is_off() {
        // 1 + (200*10)/10 = 201
        let d = decide(&inputs(1, 200, 10, 1, false));
        assert_eq!(d, Decision { state: State::Off, reason: Reason::BlendedAboveCeiling(201) });
    }
}
