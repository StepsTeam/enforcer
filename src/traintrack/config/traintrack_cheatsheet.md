# TrainTrack Architecture Cheat Sheet

## Function Structure Standards
- **MUST**: Each function has exactly one input parameter: `train` or `subway`  
- **MUST**: Each function returns the modified `train` or `subway` object  
- **MUST**: Each function exists in its own file with filename = function name  
- **MUST**: Functions follow procedural programming style  
- **MUST**: Function names use `verb`, `verb_noun`, or `verb_adjective_noun`  
- **MUST**: Names are self-documenting and clear to non-programmers  
- **MUST**: Functions < 3,000 tokens to reduce cognitive load  
- **WHY**: Keeps code readable, focused, testable, comprehensible  

---

## Code Structure Requirements
- **MUST**: Use early exit `if` statements  
- **MUST NOT**: Use `elif`, `elseif`, `else`, `case`, `switch`  
- **MUST**: Code must be language-agnostic  
- **SHOULD**: Avoid language-specific idioms/functions  
- **MUST NOT**: Rely on language-unique features  
- **MUST NOT**: Use OOP patterns, classes, arrow/lambda/anonymous functions  
- **MUST NOT**: Use ternary operators (`a ? b : c`)  
- **MUST NOT**: Use special/cryptic variables (`$`, `_`, `tmp`, `i`)  

---

## Error and Logging Standards
- **watch[]**: Assigns debug levels/messages for train logs  
- **warn[]**: Aggregates SARIF warnings from tools  
- **wreck[]**: Critical rule violations, Enforcer-only  
- **get_tool_sarif_rules[]**: Resolves SARIF rule names or defaults to debug placeholders  
- **MUST**: All error messages follow SARIF format  
- **MUST**: Errors include actionable guidance  
- **MUST**: Errors written as prompts for LLMs/junior developers  

---

## Architecture Principles
- **SHOULD**: Folders named after matching OSS tools (e.g., `shellcheck`)  
- **MUST**: Treat folders as namespaces  
- **MUST**: Only share across `debug/`, `sarif/`, `train/`  
- **MUST**: Functions decoupled and independent  
- **MUST**: Functions replaceable without side effects  
- **SHOULD**: Prefer duplicate code instead of dependencies  
- **WHY**: Reduces coupling, increases maintainability  
- **MUST**: Use horizontal file/folder structure  
- **MUST**: One function per file  
- **MUST**: Organize by feature/domain, not layer  
- **SHOULD**: Each folder has `track_` orchestration function  

---

## Testing Requirements
- **MUST**: Create `test_` function for each TrainTrack function  
- **MUST**: Include normal unit tests  
- **MUST**: Include mutation (edge-case) tests  
- **MUST**: Include fuzzy/randomized input tests  

---

## Code Comments
- **No Redundant Comments**: Do not explain obvious code  
- **No Verbose Comments**: Keep comments concise  
- **Only "Why" Comments**: Use comments only for non-obvious decisions or workarounds  
- **No Changelog Comments**: Do not describe incremental edits  
- **Default to No Comments**: Assume no comments unless necessary  
- **Rule**: Strive for self-documenting code  

---

## Implementation Checklist
- [ ] Function name uses valid `verb_noun` or `verb_adjective_noun` pattern  
- [ ] One `train` input / output  
- [ ] One function per file  
- [ ] Function < 3,000 tokens  
- [ ] Early exit `if` only, no `else/elif/case`  
- [ ] Language-agnostic  
- [ ] No OOP, arrow/lambda, or ternaries  
- [ ] Clear variable names (non-cryptic)  
- [ ] SARIF-compliant errors where applicable  
- [ ] Unit + Mutation + Fuzzy tests exist  
- [ ] Function completely decoupled from others  

---

## Benefits of TrainTrack
1. **LLM-Friendly**: Easy for AI to parse/modify  
2. **Non-Programmer Accessible**: Business-readable code  
3. **Maintainable**: Replace/update without side effects  
4. **Testable**: Small focused functions easy to test  
5. **Debuggable**: Straightforward linear flow  
6. **Scalable**: Naturally grows with horizontal structure  
```