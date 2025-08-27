

    Parameter	Description
    --file-path	Path to the source code or directory to analyze
    --config	Path to configuration file
    --rules, --rule-set	Specify which rules to apply
    --severity	Minimum severity to report (e.g., low, medium, high)
    --format, --output	Output format (e.g., json, sarif, text)
    --output-file	Write output to a specific file
    --exclude, --ignore	Files or directories to exclude
    --language	Specify language (e.g., php, python, js) if not auto-detected
    --threads, --jobs	Number of parallel threads to use
    --fail-on-warning	Fail if a warning is detected
    --timeout	Max time for analysis
    --stdin	Accept input via stdin (e.g., from piped editor)
    --quiet, --verbose	Adjust verbosity of output
    --debug	Enable debug output for tool execution
    --version	Print tool version
    --help	Show usage

    Execution & Environment
    --env, --env-file	Load environment variables from a file (e.g., .env)
    --ci	Enable CI mode (less interactive, more structured output)
    --dry-run	Simulate execution without actually running the scan
    --interactive	Run tool in interactive mode (e.g., guided auth)
    --no-color	Disable colored output for CI logs
    --continue-on-error	Don’t exit on first error (useful in batch or chained scans)
    --cache, --no-cache	Enable/disable caching of scan results
    --progress-bar Display percentage of scans complete
        How do we know the total scans that will be used? 

    Output Control
    --output-dir	Directory to store all outputs (logs, reports, metadata)
    --sarif, --json, --csv	Directly specify desired output format
    --log-level	Set logging level (debug, info, warn, error)
    --summary, --stats	Include or export summary/stats of findings
    --report-name	Custom name for report file

    --original-output or microscanner prompts

    Security & Access
    --scope	Limit scan scope (e.g., only this domain or repo)
    --upload, --push	Automatically upload results to dashboard or external service

    Network / Remote Control
    --remote-config	Load configuration from a remote URL or API
    --notify-webhook	Send webhook when scan completes
    --heartbeat	Emit periodic status signals (for orchestration or CI)

    Token Counting 
        token counting can be disabled?
            No. It is part of --refactor --strict mode
    rs-bpe 
        https://pypi.org/project/rs-bpe
    --token-limit   Limit the output based on the number of tokens
    --token-encoding
        model-specific encodings (cl100k_base, p50k_base)

    --flaw-limit  Limit the number of flaw messages returned