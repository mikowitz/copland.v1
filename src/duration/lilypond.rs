use super::Duration;
use crate::error::Error;
use crate::to_lilypond::ToLilypond;

fn dots_count(duration: Duration) -> u32 {
    format!("{:b}", duration.numerator)
        .chars()
        .filter(|&c| c == '1')
        .count() as u32
        - 1
}

fn dots(duration: Duration) -> String {
    ".".repeat(dots_count(duration) as usize)
}

fn base_duration_string(duration: Duration) -> String {
    match duration.as_float() {
        f if f >= 8. => "\\maxima".to_string(),
        f if f >= 4. => "\\longa".to_string(),
        f if f >= 2. => "\\breve".to_string(),
        _ => {
            format!(
                "{}",
                2_i32.pow((duration.denominator as f64).log2() as u32 - dots_count(duration))
            )
        }
    }
}

impl ToLilypond for Duration {
    fn to_lilypond(&self) -> Result<String, Error> {
        if self.is_printable() {
            Ok(format!("{}{}", base_duration_string(*self), dots(*self)))
        } else {
            Err(Error::UnprintableDuration(self.numerator, self.denominator))
        }
    }
}
