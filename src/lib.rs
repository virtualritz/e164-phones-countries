mod data;
use data::*;

/// Converts a phone number into an
/// [ISO 3166-1 alpha-2](https://en.wikipedia.org/wiki/ISO_3166-1_alpha-2)
/// two-letter country code.
///
/// The phone number must have at least 10 digits.
///
/// # Examples
///
/// ```
/// # use e164_phones_countries::phone_number_to_country_code;
/// assert_eq!(phone_number_to_country_code("12069359290"), Some("US"));
/// assert_eq!(phone_number_to_country_code("12229359290"), None);
/// # assert_eq!(phone_number_to_country_code("99893592902"), Some("UZ"));
/// # assert_eq!(phone_number_to_country_code("26226992902"), Some("YT"));
/// ```
pub fn phone_number_to_country_code(phone: &str) -> Option<&'static str> {
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
/// two-letter country code to a country calling code.
///
/// Returns `None` if the code is not found
///
/// # Examples
///
/// ```
/// # use e164_phones_countries::country_code_to_calling_code;
/// assert_eq!(country_code_to_calling_code("US"), Some(1));
/// assert_eq!(country_code_to_calling_code("ZZ"), None);
/// ```
pub fn country_code_to_calling_code(code: &str) -> Option<u32> {
    ISO_3166.get(code).copied()
}
