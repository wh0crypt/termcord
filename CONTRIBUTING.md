# Contributing to TermCord

Thank you for your interest in contributing to **TermCord**! This document outlines the guidelines for reporting issues, proposing features, and submitting code changes.

## Reporting Issues

- Before opening a new issue, please check if a similar issue already exists.  
- Provide a **clear title** and **detailed description**.  
- Include steps to reproduce bugs whenever possible.  

## Feature Requests

- Feature requests are welcome!  
- Please describe the problem and your proposed solution.  
- Keep requests focused and clear.  

## Code Contribution

1. **Fork the repository** and create a branch:

```bash
git checkout -b feature/<your-feature-name>
# or
git checkout -b bugfix/<your-bug-name>
```

2. Make your changes on your branch.

3. Ensure code follows the **Rust style guidelines** (`cargo fmt`).  

4. Run tests to ensure nothing is broken:

```bash
cargo test
cargo clippy --all-targets --all-features -- -D warnings
```

5. Commit your changes with a clear message:

```bash
git commit -m "feat: add new command to TermCord"
```

6. Push your branch to your fork and open a **pull request**.

## Code of Conduct

By participating, you agree to abide by the [Contributor Covenant Code of Conduct](https://www.contributor-covenant.org/version/2/1/code_of_conduct/).  

## Additional Notes

- Keep commits **atomic and descriptive**.  
- Follow the **Semantic Versioning** guidelines for any changes affecting the API.  
- Large changes or breaking changes should be discussed in an issue before implementation.
