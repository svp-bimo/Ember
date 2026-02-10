# ADR 0003: Config Precedence

## Status
Proposed

## Context
Services need a predictable configuration resolution strategy that works locally and in production. Ember currently supports YAML files and environment variables.

## Decision
- Configuration is loaded in this precedence order:
  1. **Profile-specific YAML** (e.g., `application-dev.yaml`) if `EMBER_PROFILE` is set.
  2. **Default YAML** (`application.yaml`) if present.
  3. **Environment JSON** via `EMBER_CONFIG_JSON` as a fallback.
- Missing YAML files fall back to environment JSON without failing.
- Configuration types are strongly typed via `serde` and fail fast on parse errors.

## Consequences
- Developers can use profiles locally while still supporting runtime env configs.
- CI/CD pipelines can inject config via environment variables without needing YAML files.
- Misconfigured JSON/YAML will fail at startup with clear errors.
