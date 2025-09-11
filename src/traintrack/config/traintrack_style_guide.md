# TrainTrack Architecture Style Guide

## Core Philosophy

The **TrainTrack architecture** prioritizes **clear communication to non-programmers and LLMs** above all else. Every design decision serves this fundamental goal of making code readable, understandable, and maintainable by humans and LLM systems alike.

---

## Function Structure Standards

### Input/Output Pattern
- **MUST**: Every function has exactly ONE input parameter called `train` or `subway`  
- **MUST**: Every function returns the modified `train` or `subway` object  
- **MUST**: Functions follow procedural programming paradigms only  
- **MUST**: Each function is in its own file and the filename matches the function name  

```
station verb_adjective_noun[train]
    // Process train
    return train
```

### Function Naming Convention
- **MUST**: Use `verb_adjective_noun`, `verb_noun`, or `verb` names  
- **MUST**: Names must be self-documenting and immediately clear to non-programmers  
- **Examples**:  
  - `parse_json_data(train)`  
  - `validate_user_input(train)`  
  - `generate_error_report(train)`  

Naming the functions verb_adjective_noun[] make the names grammatically correct.  
The style allows humans to read the sequential functions to understand the code.

### Function Size Limits
- **MUST**: Keep functions under 3,000 tokens to lower cognitive load & context  
- **WHY**: Ensures functions remain focused, testable, and comprehensible  

---

## Code Structure Requirements

### Control Flow
- **MUST**: Use early exit `if` statements exclusively  
- **MUST NOT**: Use `elif`, `elseif`, `else`, or `case`/`switch` statements  
- **WHY**: Early exits create clearer, more linear logic flow  

```
// ✅ CORRECT - Early exit pattern
function validate_data[train]
    if [!train.data]
        train.error = No data found
        return train
    
    if [train.data.length == 0]
        train.error = Empty data set
        return train
    
    // Process valid data
    train.processed = true
    return train

// ❌ INCORRECT - Nested conditions
function validate_data[train]
    if [train.data]
        if [train.data.length > 0]
            train.processed = true
          else 
            train.error = Empty data set
      else 
        train.error = No data found
    
    return train
```

### Language Agnostic Code
- **MUST**: Write code that translates easily between programming languages  
- **SHOULD**: Avoid language-specific native functions and idioms  
- **MUST NOT**: Use language-unique features that don't exist elsewhere  

### Forbidden Constructs
- **MUST NOT**: Use classes or object-oriented patterns  
- **MUST NOT**: Use arrow functions, lambda expressions, or anonymous functions  
- **MUST NOT**: Use ternary operators (`condition ? true : false`)  
- **MUST NOT**: Use special variables or cryptic abbreviations (`$`, `_`, `tmp`, `i`)  

```
// ✅ CORRECT - Clear variable names
function process_user_data[train]
    user_count = train.users.length
    current_user = train.users[user_index]
    return train

// ❌ INCORRECT - Cryptic variables
function process_user_data[train]
    n = train.users.length
    u = train.users[i]
    return train
```

---

## Error and Logging Handling Standards

### How the Enforcer Debugging Works

**watch[]**  
```
train[watch][level] = 3
train[watch][message] = configure_cli:
train = watch[train]

level: integer; set so that the watch[] messages can be filtered
message: string; Dynamically set in functions for debugging logs
watch[]: array: updates the train[log] array
```

**warn[]**  
```
train[warn][rule_name] = TOOL_RULE_NAME
train[warn][message] = Dynamically set message

rule_name: string; Starts with the tool/folder name
                       a key set in config/<tool>_rules.sarif
message: string; Optionally set to overwrite the default prompt
warn[] is used to aggregate SARIF warning messages from SAST tools
```

**wreck[]**  
```
train[wreck][rule_name] = TOOL_RULE_NAME
train[wreck][message] = Dynamically set message

rule_name: string; Starts with the tool/folder name
                       a key set in config/<tool>_rules.sarif
                       sarif module uses it to look up details
message: string; Optionally set to overwrite the default prompt 
wreck[] is only called in the Enforcer project's business logic
```

**get_tool_sarif_rules[]**  
```
if train[tool][tool_name] is not set, rule_name = DEBUG_TOOL_NAME_NOT_SET
    Sets SARIF details based on debug/config/debug_rules.sarif for wreck[]
    return wreck[train]

Sets rule_name if train[warn][rule_name] is set
Resets rule_name if train[wreck][rule_name] is set
if rule_name is not set, rule_name = DEBUG_RULE_NAME_NOT_SET
    Sets SARIF details based on debug/config/debug_rules.sarif for wreck[]
    return wreck[train]

Reads from <TOOL>_SARIF_RULES that has been compiled into a costant
```

### SARIF Format Compliance
- **MUST**: Format all error messages in valid SARIF (Static Analysis Results Interchange Format)  
- **MUST**: Include actionable guidance for fixes  
- **MUST**: Write error messages as prompts for LLMs or junior developers  

```
{
    "ruleId": "train_validation_error",
    "message": {
        "text": "Train object missing required 'data' field. Add train.data = {} before calling this function."
    },
    "level": "error",
    "locations": [{
        "physicalLocation": {
            "artifactLocation": {
                "uri": "validate_train_data.js"
            },
            "region": {
                "startLine": 15
            }
        }
    }]
}
```

---

## Architecture Principles

### Pipeline Architecture
- **SHOULD**: Folders are named based on their open source names (ex shellcheck, tree)  
- **SHOULD**: Folder names should be treated as though they are namespaces or classes  
- **MUST**: Folders must be decoupled only allowing sharing of debug/ sarif/ and train  

### Decoupling Requirements
- **MUST**: Keep functions completely independent and decoupled  
- **MUST**: Enable easy replacement or modification of individual functions  
- **SHOULD**: Duplicate code rather than create dependencies between functions  
- **WHY**: Reduces coupling and increases maintainability  

### File Organization
- **MUST**: Use horizontal folder and file structure  
- **MUST**: One function per file  
- **MUST**: Organize by feature/domain, not by technical layer  

```
tool/
├── config/ 
├── acquire_tool.rs
├── configure_tool.rs
├── download_tool_artifacts.rs
└── execute_tool.rs
```

### track_ (Pipeline) Functions
- **SHOULD**: Folders should have a track file that calls the folder function in order  

```
function track_tool[train]
    train = acquire_tool[train]
    train = configure_tool[train]
    train = download_tool_artifacts[train]
    train = execute_tool[train]
    return train
```

---

## Testing Requirements

### Comprehensive Testing
- **MUST**: Create `test_` functions for every TrainTrack function  
- **MUST**: Include unit tests for normal operation  
- **MUST**: Include mutation tests for edge cases  
- **MUST**: Include fuzzy tests for unexpected inputs  

```
function test_validate_user_input[]
    // Unit test - normal case
    // Mutation test - invalid data
    // Fuzzy test - random inputs
```

---

## Code Comments

- **No Redundant Comments**: Do not explain what the code does if it's obvious from the code itself  
- **No Verbose Comments**: Keep comments extremely concise and to the point  
- **Only "Why" Comments**: Comments should only be used to explain why a particular piece of code exists, if it's a non-obvious design decision, a workaround, or addresses a specific complex scenario  
- **No Change Log Comments**: Do not add comments describing what you changed or added between iterations  
- **Default to No Comments**: Unless a line or block of code genuinely requires a "why" explanation as per point 3, assume no comments are needed  

Essentially, strive for self-documenting code, and use comments as a very last resort for critical, non-obvious context.  

---

## Implementation Checklist

For every TrainTrack function, verify:

- [ ] Function name follows `verb_noun` or `verb_adjective_noun` pattern  
- [ ] Single `train` parameter input and output  
- [ ] Function exists in its own file  
- [ ] Under 3,000 tokens in size  
- [ ] Uses only early exit `if` statements  
- [ ] No `elif`, `else`, or `case` statements  
- [ ] Language-agnostic code style  
- [ ] No classes, arrow functions, or ternary operators  
- [ ] Clear, non-programmer-friendly variable names  
- [ ] SARIF-compliant error messages  
- [ ] Comprehensive test coverage  
- [ ] Function is completely decoupled from others  

---

## Benefits of TrainTrack

1. **LLM-Friendly**: AI systems can easily understand and modify individual functions  
2. **Non-Programmer Accessible**: Business stakeholders can read and understand the code  
3. **Maintainable**: Functions can be replaced or updated without affecting others  
4. **Testable**: Small, focused functions are easy to test thoroughly  
5. **Debuggable**: Linear flow with early exits makes debugging straightforward  
6. **Scalable**: Horizontal structure grows naturally as features are added  

---

*Remember: Every decision in TrainTrack serves the goal of clear communication to humans and AI systems. When in doubt, choose the more explicit, readable option.*

1. Remember the rules in this TrainTrack Architecture Style Guide 
2. Return "I understand TrainTrack" and we will begin programming

```