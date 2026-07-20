## 1.0.0 (2026-07-20)

### ⚠ BREAKING CHANGES

* This is the initial release of CommitKit, and it requires Rust to be installed on the system.

### Features

* add advanced features to enhance commit functionality ([235cece](https://github.com/somethingwithproof/commitkit-rust/commit/235ceceb70938b109724f135954436053b9f6767))
* add Dependabot auto-merge workflow ([b8859e1](https://github.com/somethingwithproof/commitkit-rust/commit/b8859e117eefbc0d7aa2b1ba45b0640ab5753168))
* add semantic versioning and automated releases ([400699b](https://github.com/somethingwithproof/commitkit-rust/commit/400699b0f21a73540614245c8a2d36cab4d89c31))
* add SLSA Level 3 provenance workflow ([d347bb8](https://github.com/somethingwithproof/commitkit-rust/commit/d347bb88a56db131afa43e0fad26e1dfe261c117))
* enhance workflows with job summaries and optimized matrices ([b4ad1bf](https://github.com/somethingwithproof/commitkit-rust/commit/b4ad1bf608f00971ea6fe513417387c85b9ab68d))
* Implement CommitKit CLI app for conventional commits ([0225d8f](https://github.com/somethingwithproof/commitkit-rust/commit/0225d8fc724c7e0887527c9f09e14e22a9ec7f1b))
* implement Google Rust style guide ([e7253da](https://github.com/somethingwithproof/commitkit-rust/commit/e7253dac027acf715f9a7285342d687f233da8cf))
* restructure project with idiomatic Rust architecture ([3ee2b6e](https://github.com/somethingwithproof/commitkit-rust/commit/3ee2b6e1d68673a0ed93c1787a746519d878df13))
* upgrade Rust version from 1.70.0 to 1.85.0 ([5edffdd](https://github.com/somethingwithproof/commitkit-rust/commit/5edffddb054bdb061a49c51ce462be4e7fcae00c))

### Bug Fixes

* apply pre-commit formatting to CODEOWNERS ([92b82be](https://github.com/somethingwithproof/commitkit-rust/commit/92b82be952aabb53a67f3df009afe96dd3d1a934))
* **ci:** address lint and coverage failures ([#214](https://github.com/somethingwithproof/commitkit-rust/issues/214)) ([fa2873d](https://github.com/somethingwithproof/commitkit-rust/commit/fa2873d126eaf63db5fd186e081e13e84925f112))
* **ci:** align labeler action inputs ([2040e9b](https://github.com/somethingwithproof/commitkit-rust/commit/2040e9be7e6a1eb7fcaba5923ff526f77d88e76d))
* **ci:** align release build with crate MSRV ([802e83c](https://github.com/somethingwithproof/commitkit-rust/commit/802e83cf00c609c0e68f2874a1e272b16d9d8376))
* **ci:** call shared release with supported inputs ([e7c80a2](https://github.com/somethingwithproof/commitkit-rust/commit/e7c80a243fb52ceb3338e30c6273178263e1bb0b))
* **ci:** correct auto-merge workflow check name ([131fd32](https://github.com/somethingwithproof/commitkit-rust/commit/131fd326233ae5ddca88431056ff38530881344a))
* **ci:** make semantic release self-contained ([7c17858](https://github.com/somethingwithproof/commitkit-rust/commit/7c178583694d7ce40965b2471121ef97a5fa6818))
* **ci:** reference the current shared-workflow repository ([3c38326](https://github.com/somethingwithproof/commitkit-rust/commit/3c38326e6d76ee6b08a82e83dc0731d139e19e42))
* **ci:** repair labeler step indentation ([83417d4](https://github.com/somethingwithproof/commitkit-rust/commit/83417d4d8efbcb4dcf3b80dbbd4763a1a46c5473))
* **ci:** repair malformed workflow triggers ([4cfa2ca](https://github.com/somethingwithproof/commitkit-rust/commit/4cfa2caca360b2006b72e0b62783042707ad8f69))
* **ci:** repair malformed workflow triggers ([112e3b1](https://github.com/somethingwithproof/commitkit-rust/commit/112e3b16155c9233d23b72cd39b8382f5c20ff6e))
* **ci:** repair malformed workflow triggers ([db2430e](https://github.com/somethingwithproof/commitkit-rust/commit/db2430ee176847dd414854e44b66224815cca924))
* **ci:** repair malformed workflow triggers ([4d1474d](https://github.com/somethingwithproof/commitkit-rust/commit/4d1474d7ded5fc564c905b1e8c8ce4f55bbafa41))
* **ci:** use caller token for shared release ([75c27f9](https://github.com/somethingwithproof/commitkit-rust/commit/75c27f96bd7eaf69f10d91e403c1e5fd3d467a15))
* **ci:** use specific runner labels, fix codeql concurrency ([be25927](https://github.com/somethingwithproof/commitkit-rust/commit/be259274eacef9488d653389d76d2226c95458bf))
* **ci:** use valid CodeQL concurrency context ([bf14ce9](https://github.com/somethingwithproof/commitkit-rust/commit/bf14ce9b842650bf7eaa31ab03d4d2c63c4dce1d))
* disable auto-assign-to-project workflow ([4b12053](https://github.com/somethingwithproof/commitkit-rust/commit/4b12053cc6595def7064ab6bb8e9509a6656e56a))
* Remove duplicated Job Summary sections and update auto-assign action ([55a51bc](https://github.com/somethingwithproof/commitkit-rust/commit/55a51bcadab66aa401933aa8f897d489140753ef))
* resolve build issues and add pre-commit configuration ([0def93f](https://github.com/somethingwithproof/commitkit-rust/commit/0def93f214b0ad4700298c58f81db28868af3fa5))
* resolve emoji spacing issue and clippy warnings ([bc9266a](https://github.com/somethingwithproof/commitkit-rust/commit/bc9266ad5057d82e018d7d3dd8fbd15797a5f458))
* resolve formatting and build issues ([da7ae5d](https://github.com/somethingwithproof/commitkit-rust/commit/da7ae5db561e80462c5318f1dea2bb4ce23f139e))
* resolve workflow YAML syntax errors ([cc88a7b](https://github.com/somethingwithproof/commitkit-rust/commit/cc88a7b7d2e97a8b826dbc9cc5549f025166528b))
* update GitHub Actions versions and add workflow_dispatch ([6af1247](https://github.com/somethingwithproof/commitkit-rust/commit/6af1247ff0d5772dfe13f51f44382267079b9b4b))

# Changelog

All notable changes to CommitKit will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## Unreleased

## 0.1.0 (2024-05-05)

### Added
- **Added** (core): Initial version with basic conventional commit functionality
- **Added** (ui): Interactive prompts for commit components
- **Added**: Emoji support for commit messages
- **Added**: Template support for automated commits
- **Added**: Changelog generation and maintenance
- **Added**: Git hooks for commit validation and preparation
- **Added**: Commit statistics and analysis
- **Added**: Support for custom configurations via .commitkit.toml

### Changed
- **Changed**: Refactored to use proper Rust project structure
- **Changed**: Improved error handling with anyhow and thiserror

### Fixed
- **Fixed**: Proper handling of empty scopes
