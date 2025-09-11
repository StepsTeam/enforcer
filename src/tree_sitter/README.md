## Language Configurations

The file `language_configurations.json` defines how Enforcer detects and configures supported languages.
It replaces an older approach of using pre-compiled crates, since all grammars are now compiled directly
from their upstream repositories. 
This avoids version conflicts and ensures that the latest grammar definitions can be used consistently.

### Structure

Each language entry contains:

- **extensions**  
  A list of common file extensions associated with the language.  
  Example: `"rs"` for Rust, `"php"` for PHP files.

- **heuristics**  
  Snippets of code or tokens that are commonly found in files of this language.  
  These are used in cases where the file extension is ambiguous or missing.  
  Example: `"fn "` and `"pub "` for Rust.

- **repository_url**  
  The official upstream Tree-sitter grammar repository.  
  Grammars are cloned and compiled directly from this source.

- **traintrack**  
  Indicates whether the language can be written in TrainTrack style.  
  Values: `"yes"` or `"no"`.  
  - `"yes"` → Full AST enforcement and TrainTrack rules applied.  
  - `"no"` → Only basic parsing, no enforcement.

### Example

```json
"rust": {
  "extensions": ["rs"],
  "heuristics": ["fn ", "let mut ", "mod ", "pub "],
  "repository_url": "https://github.com/tree-sitter/tree-sitter-rust",
  "traintrack": "yes"
}
