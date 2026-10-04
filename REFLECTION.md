# Reflections

## CA1 Reflection Questions

### Question 1: Fractional Part Handling

In `src/scanner.rs:136`, I only take the decimal point as part of a number if the character after it is a digit. That means `5.` gets scanned as the number `5`, and then the `.` is left for the next scan. Since dot is not a token, the scanner reports `Character is not part of any token.` This matches section 1.4. For `12.75`, the lookahead succeeds and the whole decimal becomes one number token. The `trailing_dot.kobo` case and valid decimals in `numbers.kobo` both pass. Checking before consuming the dot is what lets the scanner distinguish those cases.

---

### Question 2: Line Counter and EOF

I increment the line counter in two places: `src/scanner.rs:96` for a normal newline and `src/scanner.rs:114` for a newline inside a string. EOF works differently: `src/scanner.rs:34-37` takes its line from the last real token, not from the counter. So if the last token was on line 1 and the file has two blank lines after it, EOF still says line 1, even though those newlines were counted. This matches section 6.1: blank lines at the end do not change the token stream. The `eof_line.kobo` test checks this. If the source is empty or only comments, there is no real token, so EOF uses line 1.

---

### Question 3: Debugging and Learning (Failed Test Reflection)

`tests/phase-1/valid/numbers.kobo` failed when `number()` was unfinished. In commit `50fb018732d1059263d143da5d863bdccb6338c7`, `src/scanner.rs:118` was `todo!("number")`, so scanning stopped with a panic. I had not implemented the rule that the dot belongs to a number only when followed by a digit. In commit `d136fa5ed272fa1849e269961bd06d0e554fc615`, the check was `src/scanner.rs:123`: `if self.peek() == '.' && self.peek_next().is_ascii_digit() {`, and the token was emitted at `src/scanner.rs:134`: `self.add(TokenType::Number);`. Those are the lines as they appeared in that commit. They now appear at `src/scanner.rs:136` and `src/scanner.rs:147`. Both commits are before the deadline. I understand why lookahead must happen before consuming the dot.

## Change & Development Reflection Log

I completed `string()` at `src/scanner.rs:109-126`: it counts newlines inside a string, keeps the opening line for an unterminated-string error, and includes the closing quote in a successful token. `cargo build` succeeds and `tool/run-golden phase-1` passes 14/14. Extra checks also passed for all keywords, operators, empty/comment-only input, CRLF, multiline strings, and multiple bad characters. The expected failures printed `[line 1] Error: Character is not part of any token.` for both `.5` and `5.`, and `[line 2] Error: String is never closed.` for a string opened on line 2; each exited 65. Three invalid characters produced three diagnostics and exit 65. A separate stress run passed all 257 cases, including very long tokens and strings, repeated operators, Unicode, and seeded random input, with no crashes. These checks passed; the hidden marking tests have not been run.
