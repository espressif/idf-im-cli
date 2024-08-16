# Contributing to `idf-im-cli`

Thank you for considering contributing to the `idf-im-cli` project! Whether you're reporting bugs, suggesting new features, or submitting patches, your contributions are highly appreciated. This guide outlines the process to ensure your contribution can be easily integrated into the project.

## Table of Contents
- [Getting Started](#getting-started)
- [Reporting Issues](#reporting-issues)
- [Feature Requests](#feature-requests)
- [Setting Up the Development Environment](#setting-up-the-development-environment)
- [Submitting Contributions](#submitting-contributions)
  - [Pull Requests](#pull-requests)
  - [Code Style](#code-style)
  - [Testing](#testing)
- [Code of Conduct](#code-of-conduct)
- [License](#license)

## Getting Started

1. **Fork the Repository**: Start by forking the repository to your GitHub account.

2. **Clone the Forked Repository**:
    ```sh
    git clone https://github.com/your-username/idf-im-cli.git
    cd idf-im-cli
    ```

3. **Create a New Branch**:
    ```sh
    git checkout -b feature/your-feature-name
    ```

4. **Make Your Changes**: Implement your feature, bug fix, or improvement.

5. **Test Your Changes**: Ensure your changes work as expected and do not break existing functionality.

6. **Commit and Push**:
    ```sh
    git add .
    git commit -m "Description of your changes"
    git push origin feature/your-feature-name
    ```

7. **Open a Pull Request**: Navigate to the original repository and open a pull request from your forked branch.

## Reporting Issues

If you encounter a bug or have a question, please report it by creating an issue on GitHub. To help us understand and resolve the issue, please include the following details:

- A descriptive title.
- Steps to reproduce the issue.
- Expected behavior.
- Actual behavior.
- Any relevant logs, screenshots, or terminal output.

## Feature Requests

Feature requests are welcome! If you have an idea for a new feature, please open an issue to discuss it before starting any implementation. This allows us to evaluate the proposal and provide feedback.

## Setting Up the Development Environment

To contribute code, you need to set up a local development environment. Follow these steps:

1. **Install Rust**: Ensure you have Rust installed. You can install Rust using [rustup](https://rustup.rs/).

2. **Install Dependencies**:
    ```sh
    cargo build
    ```

3. **Run the Application**: You can run the application to verify everything is set up correctly:
    ```sh
    cargo run -- <args>
    ```

4. **Run Tests**:
    ```sh
    cargo test
    ```

## Submitting Contributions

### Pull Requests

- **One Change per PR**: Keep your pull requests focused on a single change. This makes it easier to review and merge.

- **Descriptive Titles**: Use clear and descriptive titles for your pull requests.

- **Link to Issues**: If your pull request addresses an existing issue, link to it in the PR description.

- **Review Process**: Your pull request will be reviewed by maintainers. Please respond to any feedback or questions.

### Code Style

- Follow Rust's [standard coding style](https://doc.rust-lang.org/1.0.0/style/style/).
- Use `rustfmt` to format your code:
    ```sh
    cargo fmt
    ```
- Ensure your code is free of warnings and errors:
    ```sh
    cargo clippy
    ```

### Testing

- **Unit Tests**: Ensure that your code is covered by unit tests. Add new tests for any new functionality.
  
- **Run All Tests**: Before submitting your PR, run all tests to ensure nothing is broken:
    ```sh
    cargo test
    ```

## Code of Conduct

This project adheres to the [Contributor Covenant Code of Conduct](https://www.contributor-covenant.org/version/2/0/code_of_conduct/). By participating, you are expected to uphold this code.

## License

By contributing to `idf-im-cli`, you agree that your contributions will be licensed under the [Apache License 2.0](LICENSE).
