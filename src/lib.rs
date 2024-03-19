//! Mapping [E.164](https://en.wikipedia.org/wiki/E.164) international phone
//! numbers to
//! [ISO 3166-1 alpha-2](https://en.wikipedia.org/wiki/ISO_3166-1_alpha-2)
//! country codes as well as the ISO 3166-1 alpha-2 country codes to E.164
//! country codes (one to three-digit phone number prefixes).
//!
//! # Examples
//!
//! ```
//! use e164_phones_countries::*;
//!
//! assert_eq!(
//!     e164_number_to_iso3166("12069359290"),
//!     Some("US"),
//! );
//!
//! assert_eq!(
//!     e164_number_to_iso3166("12229359290"),
//!     None,
//! );
//!
//! assert_eq!(
//!     iso3166_to_e164_country_code("US"),
//!     Some(1),
//! );
//!
//! assert_eq!(
//!     iso3166_to_e164_country_code("ZZ"),
//!     None,
//! );
//! ```

mod data;
use data::*;

/// Converts a [E.164](https://en.wikipedia.org/wiki/E.164) phone number into
/// an [ISO 3166-1 alpha-2](https://en.wikipedia.org/wiki/ISO_3166-1_alpha-2)
/// two-letter country code.
///
/// The phone number must have at least 10 digits.
///
/// # Examples
///
/// ```
/// # use e164_phones_countries::*;
/// assert_eq!(e164_number_to_iso3166("12069359290"), Some("US"));
/// assert_eq!(e164_number_to_iso3166("12229359290"), None);
/// # assert_eq!(e164_number_to_iso3166("99893592902"), Some("UZ"));
/// # assert_eq!(e164_number_to_iso3166("26226992902"), Some("YT"));
/// ```
pub fn e164_number_to_iso3166(phone: &str) -> Option<&'static str> {
    if phone.len() < 10 {
        return None;
    }

    let mut prefix = 0;

    if &phone[0..1] == "1" {
        prefix = phone[0..4].parse::<u32>().unwrap();
    } else {
        let s = &phone[0..7];

        //let mut ns = 0;
        let mut done = false;
        let mut x = s.len();
        while !done {
            x -= 1;

            let ns = s[..x].to_owned().parse::<u32>().unwrap();

            if PHONE_PREFIX.contains_key(&ns) {
                prefix = ns;
                done = true;
            }

            if x == 1 {
                done = true;
            }
        }
    }

    PHONE_PREFIX.get(&prefix).copied()
}

/// Converts an
/// [ISO 3166-1 alpha-2](https://en.wikipedia.org/wiki/ISO_3166-1_alpha-2)
/// two-letter country code to a [E.164](https://en.wikipedia.org/wiki/E.164)
/// country calling code.
///
/// Returns `None` if the code is not found
///
/// # Examples
///
/// ```
/// # use e164_phones_countries::*;
/// assert_eq!(iso3166_to_e164_country_code("US"), Some(1));
/// assert_eq!(iso3166_to_e164_country_code("ZZ"), None);
/// ```
pub fn iso3166_to_e164_country_code(code: &str) -> Option<u32> {
    ISO_3166.get(code).copied()
}
