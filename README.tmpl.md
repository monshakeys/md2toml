<!-- 
This is a README.md template for Jackal's Rust projects.
AI Agent Instructions:
1. Replace all bracketed placeholders (e.g., `[PROJECT_NAME]`) with the specific details of the project.
2. Fill in the content for each section based on the project's functionality.
3. Remove or keep optional sections as needed.
4. Remove all `<!-- ... -->` comments in the final output.
-->
# [PROJECT_NAME] - [CATCHY_TAGLINE]

<!-- Brief Description: A one-paragraph summary of what the project does. -->
[BRIEF_DESCRIPTION]

## Features

<!-- List the key features and selling points of the project. Use emojis for visual appeal. -->
- 🌟 [FEATURE_1]: [Brief explanation of the feature].
- 🚀 [FEATURE_2]: [Brief explanation of the feature].
- ⚙️ [FEATURE_3]: [Brief explanation of the feature].
- 🎨 [FEATURE_4]: [Brief explanation of the feature].

## Installation

### Homebrew (Recommended)

```bash
brew install monshakeys/tap/[PROJECT_NAME]
```

### Alternative Methods

- Download the latest binary from the [releases page](https://github.com/monshakeys/[PROJECT_NAME]/releases) and place it in your `$PATH`.
- Or these methods:

```bash
# Clone the repository
git clone git@github.com:monshakeys/[PROJECT_NAME].git
cd [PROJECT_NAME]

# Build for development
cargo build
./target/debug/[PROJECT_NAME]

# Build for production
cargo build --locked --release
./target/release/[PROJECT_NAME]

# Run without building an executable
cargo run              # Debug mode
cargo run --release    # Release mode

# Install to local cargo path
cargo install --path .
```

## Usage

<!-- Provide clear examples of how to use the command-line tool. Include common flags and arguments. -->
```bash
# Show help message
[PROJECT_NAME] --help
```

## Output Example

<!-- Show a screenshot or a code block demonstrating the tool's output. This helps users quickly understand what to expect. -->
```
[PASTE_OUTPUT_EXAMPLE_HERE]
```

## Project Structure

<!-- Display the project's directory layout to give users an overview of the codebase. -->
```
[PROJECT_NAME]/
├── Cargo.toml
├── crates/
│   ├── core/
│   │   └── src/
│   └── cli/
│       └── src/
└── README.md
```
