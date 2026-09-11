# Security Policy

## Scope

**lumio-contracts** is a pre-audit scaffold (0.1.x).  
The contracts compile and tests pass, but they are **not complete, not audited, and must not custody real funds.**  
See the README for the full caveat.

Any finding against the scaffold code is noted but will be evaluated in the context of what is already known: the contracts are intentionally incomplete.

## Supported Versions

| Version | Supported |
| ------- | --------- |
| 0.1.x   | ✅ (scaffold — limited scope) |

## Reporting a Vulnerability

Please **do not** open a public GitHub issue for security vulnerabilities.

Use one of the following private channels:

- **GitHub private vulnerability reporting** — click *"Report a vulnerability"* on the [Security tab](../../security/advisories/new) of this repository. This is the preferred path.
- If you are unable to use GitHub's reporting flow, open a [blank security advisory draft](../../security/advisories/new) and leave a note; a maintainer will respond within **5 business days**.

### What to include

- A clear description of the issue and its potential impact.
- Contract(s) and function(s) affected (e.g. `voting::cast_vote`).
- Minimal reproduction steps or a proof-of-concept.
- Any suggested fix, if you have one.

## Response expectations

| Step | Target |
| ---- | ------ |
| Initial acknowledgement | 5 business days |
| Triage / severity assessment | 10 business days |
| Fix or mitigation plan communicated | 20 business days |

We will credit reporters in the release notes unless anonymity is requested.
