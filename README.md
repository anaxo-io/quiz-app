# Quiz App

A quiz application built with Rust and Yew for WASM. The app presents 10 random consecutive questions from a pool of 100 general knowledge questions.

## Features

- Random selection of 10 consecutive questions
- Multiple choice answers
- Immediate feedback after submitting an answer
- Progress tracking
- Final score display
- Option to retry with a new set of questions

## Getting Started

### Prerequisites

You need to have the following installed:
- Rust and Cargo
- Trunk (WASM bundler)
- wasm32-unknown-unknown target

```bash
# Install Rust and Cargo if you don't have them
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

# Add wasm32 target
rustup target add wasm32-unknown-unknown

# Install Trunk
cargo install trunk
```

### Running the App Locally

```bash
# Start the development server
trunk serve
```

Open your browser and navigate to http://localhost:8080

### Building for Production

```bash
# Build the app
trunk build --release
```

The production files will be in the `dist` directory.

### Deploying to GitHub Pages

1. Build the app with the appropriate base path:
```bash
trunk build --release --public-url <your-repository-name>
```

2. Copy the contents of the `dist` directory to your GitHub Pages repository.

## Adding More Questions

To add more questions, edit the `get_all_questions()` function in `src/models.rs`.
