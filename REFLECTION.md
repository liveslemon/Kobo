# Reflections

## CA2 Reflection Questions

### Question 1: The Dangling Else

The lines that settle this are `src/parser.rs:161`, where `then_branch` gets parsed with `self.statement()`, and `src/parser.rs:162-166` right after it, the `else` check. It's recursion deciding this and not the grammar because when the `then` branch happens to be another `if`, that call on line 161 recurses straight into a second, inner `if_statement()`. That inner call runs its own lines 162-166 and grabs the `else` for itself before it ever returns to the outer call. So by the time the outer call checks for its own `else`, there's nothing left — it's already been eaten. For `if (false) if (true) print "a"; else print "b";` my parser prints `(if false (if true (print "a") (print "b")))`, so the else goes to the inner if. If it bound to the outer one it'd print `(if false (if true (print "a")) (print "b"))` instead.

History: `src/parser.rs:104` in commit `d8edcc2` was just `todo!("if_statement")` — no answer at all, it would've panicked. It became the real version in `1691d0f`, where line 161 above first shows up.

---

### Question 2: Assignment and the LL(1) Problem

The actual check is `src/parser.rs:221`, `return match expr {`, with the identifier case right under it on line 222. The reason it can't happen before the left side is parsed is that "is this a valid assignment target" isn't really a question you can ask a token — it's a question about the shape of a tree. Before `self.or()` (line 215) even runs there's no tree yet, just tokens that could turn into basically anything. The only way to know is to let it finish parsing as a normal expression and then look at what came out. A bare `Expr::Variable` becomes an `Expr::Assign`; anything else gets rejected.

I ran `1 = 2;` through it to check, and it gives `[line 1] Error at '=': Cannot assign to this expression.`, the message from section 5.1. It reports at the `=` token (that's `equals` on line 217), not the `1`, since that's where the parser actually notices something's wrong.

History: `src/parser.rs:142` in `d8edcc2` was just `self.or()` — no check at all, it skipped straight past assignment. It became the real check in `2d2bcf0`, line 146.

---

### Question 3: synchronise and Error Recovery

`synchronise` is `src/parser.rs:420-445`. It stops on two things — either the token it just consumed was a `;` (line 431), or the upcoming token is one of `fun var if while print return` (lines 434-440). Those six keywords are the only things that can legally start a new declaration or statement per section 3.1, so one of them, or just past a semicolon, is basically the only time it's safe to try parsing again.

Didn't want to guess on the test question so I tried it — commented out the `synchronise()` call in `parse()` and ran every file in `tests/phase-2/invalid/`. Most were unaffected, weirdly, since the next token after the error already happened to be a valid statement start. But `expect_expression.kobo` (`print 1 + * 2;`) hangs forever: the position never moves past the `*`, so `declaration()` keeps hitting the same `Expect expression.` error in a loop. The normal output (one error, exit 65) never happens, it just never ends.

Good history here too, I actually broke this for real while testing. `src/parser.rs:425` in commit `7b093f1` was `self.current += 1;` with no guard, which panics if an error hits while sitting on EOF, like `print 1 +`. Found it by throwing unterminated input at it that wasn't in the given tests. Fixed in `4a94b9b`, lines 427-429: `if !self.at_end() { self.current += 1; }`.
