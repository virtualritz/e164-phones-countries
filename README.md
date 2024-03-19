# `e164-phones-countries`

<!-- cargo-rdme start -->

Mapping [E.164](https://en.wikipedia.org/wiki/E.164) international phone
numbers to
[ISO 3166-1 alpha-2](https://en.wikipedia.org/wiki/ISO_3166-1_alpha-2)
country codes as well as the ISO 3166-1 alpha-2 country codes to E.164
country codes (one to three-digit phone number prefixes).

## Examples

```rust
use e164_phones_countries::*;

assert_eq!(
    e164_number_to_iso3166("12069359290"),
    Some("US"),
);

assert_eq!(
    e164_number_to_iso3166("12229359290"),
    None,
);

assert_eq!(
    iso3166_to_e164_country_code("US"),
    Some(1),
);

assert_eq!(
    iso3166_to_e164_country_code("ZZ"),
    None,
);
```

<!-- cargo-rdme end -->
