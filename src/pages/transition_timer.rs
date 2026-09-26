use chrono::DateTime;
use chrono_humanize::{Accuracy, HumanTime, Tense};
use leptos::prelude::*;
use window_lib::prelude::*;

#[component]
pub fn transition_timer(is_open: bool) -> impl IntoView {
    let data = WindowData::new(icons::TRANS, "Transition Timer").open(is_open);
    view! {
        <Window data id="transition-timer" disable_maximize=true>
            <div id="flag">
                <div class="flag-blue"></div>
                <div class="flag-pink"></div>
                <div class="flag-white"></div>
                <div class="flag-pink"></div>
                <div class="flag-blue"></div>
            </div>
            <div>
                <h1>"It has been"</h1>
                <h1>{format_timer()}</h1>
                <h1>"since starting HRT"</h1>
            </div>
        </Window>
    }
}

fn format_timer() -> String {
    let date = DateTime::from_timestamp(1786734000, 0).unwrap();
    let ago = HumanTime::from(date);
    let ago = ago.to_text_en(Accuracy::Precise, Tense::Present);
    let ago = if let Some(trimmed) = trim_to_word(&ago, "day") {
        trimmed
    } else if let Some(trimmed) = trim_to_word(&ago, "week") {
        trimmed
    } else if let Some(trimmed) = trim_to_word(&ago, "month") {
        trimmed
    } else {
        match trim_to_word(&ago, "year") {
            Some(trimmed) => trimmed,
            None => ago.as_str(),
        }
    };
    insert_and_into_string(&ago)
}

/// Checks input string for the pattern and trims everything after it. \
/// Assumes ASCII and checks for plurality. Returns an Option with the trimmed string.
fn trim_to_word<'a>(input: &'a str, pattern: &str) -> Option<&'a str> {
    match input.find(pattern) {
        Some(index) => {
            let pattern_len = pattern.len();
            Some(if input.as_bytes()[index + pattern_len] == 's' as u8 {
                &input[..index + pattern_len + 1]
            } else {
                &input[..index + pattern_len]
            })
        }
        None => None,
    }
}

/// Takes a string and inserts " and" into it after the last comma.\
/// If there is no comma simply returns the input string.
fn insert_and_into_string(input: &str) -> String {
    match input.rfind(",") {
        Some(index) => {
            let mut result = String::with_capacity(input.len() + 5);
            result.push_str(&input[..index + 1]);
            result.push_str(" and");
            result.push_str(&input[index + 1..]);
            result
        }
        None => input.to_string(),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const SINGULAR: &str = "1 year, 1 month, 1 week, 1 day, 1 hour, 1 minute, 1 sec, and 1ms";
    const PLURAL: &str = "2 years, 2 months, 2 weeks, 2 days, 2 hours, 2 minutes, 2 secs, and 2ms";

    #[test]
    fn test_day() {
        assert_eq!(
            Some("1 year, 1 month, 1 week, 1 day"),
            trim_to_word(&SINGULAR, "day")
        )
    }

    #[test]
    fn test_days() {
        assert_eq!(
            Some("2 years, 2 months, 2 weeks, 2 days"),
            trim_to_word(&PLURAL, "day")
        )
    }

    #[test]
    fn test_week() {
        assert_eq!(
            Some("1 year, 1 month, 1 week"),
            trim_to_word(&SINGULAR, "week")
        )
    }

    #[test]
    fn test_weeks() {
        assert_eq!(
            Some("2 years, 2 months, 2 weeks"),
            trim_to_word(&PLURAL, "week")
        )
    }

    #[test]
    fn test_insert_and() {
        let trimmed = trim_to_word(&SINGULAR, "day").unwrap();
        assert_eq!(
            "1 year, 1 month, 1 week, and 1 day",
            insert_and_into_string(trimmed).as_str()
        )
    }

    #[test]
    fn test_no_insert_and() {
        let trimmed = trim_to_word(&SINGULAR, "year").unwrap();
        assert_eq!("1 year", insert_and_into_string(trimmed).as_str())
    }
}
