# Keysmith local passphrase API (`passphrase/v1`)

Keysmith integrations generate passphrases on the user's device. Do not send
generation requests or generated secrets to a server.

The Rust API is `keysmith::generate_passphrase(&PassphraseOptions)`. The
versioned cross-platform contract is:

```json
{
  "api": "passphrase/v1",
  "words": 6,
  "separator": "-",
  "capitalize": false
}
```

`words` must be 3–12. `separator` is at most 8 printable UTF-8 bytes. Word
indices are selected uniformly from the complete EFF Large Wordlist embedded
as `src/wordlist.txt` (7,776 unique entries). Implementations must use the
platform's cryptographic random source and unbiased bounded-integer sampling;
do not use `% 7776` directly on a random integer unless values above the
largest divisible range are rejected first. Capitalization uppercases only
the first character of each selected word. The default separator is `-`.

The API returns a secret string, not a loggable result. Keep it in memory only
until the user chooses to copy or save it. Never put it in telemetry, URLs,
command arguments, crash reports, or application logs. The wordlist is public
data; generated passphrases are not.

The clients vendor the exact wordlist. CI should verify their asset hash
against `src/wordlist.txt` so a list update cannot silently make integrations
inconsistent.
