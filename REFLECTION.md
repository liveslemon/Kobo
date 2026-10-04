# Reflections

## CA1 Reflection Questions

### Question 1: Fractional Part Handling
In `src/scanner.rs:123`, I only take the decimal point as part of a number if the character after it is a digit. That means `5.` gets scanned as the number `5`, and then the `.` is left for the next scan. The dot is not its own token, so it gets reported as `Character is not part of any token.` This matches section 1.4: a decimal point needs digits after it. The `trailing_dot.kobo` test checks this exact case, and it passes after rebuilding the current code.

---

### Question 2: Line Counter and EOF
The only place I change the line counter is `src/scanner.rs:96`, when the scanner reads a newline. EOF works differently: `src/scanner.rs:34-37` takes the line from the last real token, not from the counter. So if the last token was on line 1 and the file has two blank lines after it, EOF still says line 1. This matches section 6.1, which says blank lines at the end should not change the token stream. The `eof_line.kobo` test checks that trailing newlines do not move EOF, and the code uses line 1 when there were no tokens at all.

---

### Question 3: Debugging and Learning (Failed Test Reflection)
After rebuilding, `tests/phase-1/invalid/unterminated_string.kobo` failed because `string()` at `src/scanner.rs:112` is still a `todo!()`. The expected error is `String is never closed.`, but the scanner panics before it can report it. I don't have a fix commit for this failure yet, so I can't honestly give the two commit hashes or quote the wrong line and its fixed version. I still need to implement string scanning, get this test passing, and then record the actual before-and-after commits.

## Change & Development Reflection Log

I filled in the identifier scanner so it reads the whole word before checking whether it is a keyword (`src/scanner.rs:138-145`). That helped me understand why `while` should become a `WHILE` token, but `while2` should stay an identifier.

I also wrote the number scanner (`src/scanner.rs:115-135`). It reads the digits, checks one character ahead before including a decimal point, and then adds a number token. The number cases, including the leading- and trailing-dot errors, pass in the current golden run.

The scanner test run currently reports `12/14`. `string()` is still unfinished at `src/scanner.rs:109-112`, so string input is the part I still need to work on.
