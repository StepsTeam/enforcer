### Enforcer Guidelines ###

WHAT:
Enforcer will become the "longest CI pipeline" by implementing hundreds of AppSec and DevSecOps tools
Many new tools will be added into the pipeline, so the code must be decoupled to avoid technical debt
All of the tools in the pipeline will be rewritten in TrainTrack Rust and compiled in one binary file

HOW:
. Hundreds of open source SAST and DAST tools are downloaded
. Each tool is put in a separate folder named after the tool
. All tools MUST enforce the same codebase style conventions
. Enforcer MUST use Podman securely when developing software 
. The tools are implemented using boilerplate Bash functions
. Dockerfiles MUST call TrainTrack Bash functions for builds

. The tools MUST be configured to use the strictest settings
. Each tool MUST be implemented simply and avoid common code
. Linters MUST be run before the SAST tools in all languages
. SAST tools MUST be run before DAST tools in every language
. SAST and DAST tools SHOULD be run concurrently saving time 
. It MUST adhere to Federal Information Processing Standards
. Each tool MUST be able to work starting from the file_path
. Each tool will be emulated in Rust in the TrainTrack style

. Enforcer will be air-gapped after the initial installation
. Enforcer MUST return SARIF prompt messages to fix warnings 
. Enforcer offers clear reinforcement learning (RL) for LLMs
. Enforcer provides similar training to all human developers

WHY:
. Enforcer will manage LLMs using Reinforcement Learning
. Enforcer will be run each time LLMs generate some code
. Enforcer will automate FIPS-compliant coding standards
. Developers save time they may spend implementing tools

WHERE:
. The Enforcer git repository is also the knowledge base

WHEN:
. Enforcer ALWAYS forces the strict rules to be followed

WHO:
. MicroManager runs a Enforcer uPlan to develop all code


