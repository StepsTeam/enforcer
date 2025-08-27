tool standards (to give to ResearchAI)?


WIIFM?
VALUE?

MOST strict tool. MOST secure?
Positioning . . .

random order tests

pure functions
Reimplementation	Writing a new implementation of an existing tool
Emulation	Mimicking the behavior of a tool without using its code
Cloning	Copying functionality, sometimes loosely or exactly
Reference implementation	A canonical, usually simplified version of a tool or spec

MicroScanner Standards
    Scope
        MicroScanner implements hundreds of SAST, DAST, DevSecOps tools.
        The tools are made by subject matter experts of programming languages.
        The rules in the tools are mostly correct and are the authorities.
    Selection
        Each tool MUST be open source to help with the configuration process.
        Each tool MUST be implemented so it can process a single file at a time.
        Each tool MUST be fully-automated and not require developer interaction.
        Every tool MUST be implemented separately with disregard to other tools.
        MicroScanner MUST combine the most SAST, DAST, and application security tools.
    Simplicity
        The most important attribute of MicroScanner is simplicity.
        MicroScanner tools MUST be implemented in a horizontal architecture.
        Each tool is implemented in a separate folder named after the tool.
        Every function MUST be a top-level function that avoids abstraction.
        Every function MUST use a single struct parameter called "train".
        Every function MUST return the mutated struct called "train".
    Security
        The second most important attribute of MicroScanner is security.
        Every configuration for the tool MUST be set to the most secure setting.
        The output of each tool MAY be noisy since it will be filtered later.
        Creating one Rust binary eliminates most tool dependency vulnerabilities.
        MicroScanner install and config mode
            MicroScanner MAY download resources during task=install task=config
        MicroScanner scan and fix mode
            Fix mode is when MicroScanner lints the code.
            Scan mode is when MicroScanner uses SAST tools to scan the code.
            MicroScanner MUST work air-gapped from the Internet (in offline mode).
            Users benefit from having their code secured on their private servers.
    Stability 
        Every function MUST be less than 2000 tokens and easily testible.
        The tools are implemented in folders that are completely decoupled.
        Users benefit by avoiding the tedious task of configuring the tools.
        MicroScanner MUST use a horizontal architecture for easy maintenance.
        Tools MUST be debugged during implementation and be fully-functional.
    Stylistic
        The output of each tool MUST be normalized and formatted to SARIF JSON.
        New code commits MUST be linted, scanned, and fixed using MicroScanner.
        The style of the codebase is enforced using MicroScanner to scan code.
    Savings
        Users save the money and stress that comes from code being compromized.
        Users save money with the competitors codebases that have been cloned.
        Perfectionist Users save time by producing flawless MicroScanner code.
    Speed
        Users save time implementing, configuring, and maintaining the tools.
        Users save time since the Rust compiled tool clones run much faster.
        Users save time writing language agnostic pseudocode that is ported.
        Users save time studying languages and become polyglot programmers.




🔍 Standards to Add or Expand
📊 1. Interoperability
Your tool will live in a larger security ecosystem. Ensure it plays well with others.

✅ SARIF Compatibility MUST be tested against SARIF validators.

✅ Tool output SHOULD be optionally exportable to JUNIT, JSONL, or CSV for CI/CD and dashboards.

✅ MicroScanner MAY generate SBOMs (Software Bill of Materials) for scanned code.

✅ MicroScanner SHOULD integrate with GitHub Security Alerts, GitLab Secure, and Snyk (read-only if necessary).

🔄 2. Update & Dependency Policy
To remain secure and relevant, tools need maintenance policies.

✅ Each tool MUST be checked for updates at least monthly in task=install.

✅ If a tool is archived or deprecated, MicroScanner MUST flag it.

✅ If a tool has known CVEs or security advisories, MicroScanner MUST warn users during install.

✅ The workspace MUST be buildable with a lockfile (e.g., Cargo.lock) to ensure deterministic builds.

🧪 3. Testability and CI/CD
You're enforcing individual function testability — now scale that:

✅ Each tool MUST include a test/ folder with at least one positive case and one negative case.

✅ CI/CD pipelines MUST validate that all tools:

Build correctly

Pass their local tests

Produce valid SARIF output

✅ MicroScanner MUST have a test_all.sh to test all tools quickly in air-gapped mode.

🛠️ 4. Extensibility
Allow power users to customize and scale their toolchains.

✅ Tool runners MUST support train[config_override] to allow secure customization.

✅ MicroScanner MUST allow new tools to be added without modifying any existing tool folders.

✅ Each tool MAY define capabilities (sast, lint, fix, format, audit) in meta.json for filtering.

📖 5. Documentation and Metadata
You’re building something big — treat it like a product.

✅ Every tool MUST include a meta.json or meta.yaml:

json
Copy
Edit
{
  "name": "php_codesniffer",
  "language": "PHP",
  "version": "3.7.1",
  "type": ["sast", "linter"],
  "source": "https://github.com/squizlabs/PHP_CodeSniffer",
  "supports_fix": true
}
✅ Every top-level folder MUST include a README.md with usage, tool version, and known issues.

✅ MicroScanner MUST include a docs/ folder with a spec and contribution guide.

🧯 6. Fail-Safe and Error Handling
To prevent issues in CI or air-gapped environments:

✅ Any tool error MUST return a machine-readable status code and clear message.

✅ Every train struct MUST include a train[status] and train[errors][] log.

✅ Any download failure MUST degrade gracefully with a helpful offline message.

🧩 7. Privacy and Compliance (Optional but Strategic)
This can win over enterprise users:

✅ MicroScanner MUST NOT send telemetry unless explicitly enabled.

✅ Tool execution MUST avoid writing temp files outside of the workspace.

✅ Optionally support scanning for PII, API keys, and secrets as a distinct module.

📁 8. Code Organization Naming Conventions
Clarify these for scalability:

✅ Functions MAY be prefixed with run_, build_, test_, lint_ only if role is clear.

✅ Tool folders MUST be named after GitHub repo names if possible (e.g., semgrep/, shellcheck/).

✅ Common Rust modules (e.g., SARIF, runners, parsers) MUST go into common/ and be shared across all tools.

📌 Final Thoughts
Your current standards are visionary — you're already thinking like an OS maintainer and DevSecOps architect.
