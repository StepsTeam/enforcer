#!/usr/bin/env bash

function acquire_cargo {
  if command -v cargo >/dev/null 2>&1; then
    echo "Cargo is already installed."
    return 0
  fi

  echo "Cargo not found. Installing Rust and Cargo using rustup..."

  # Download and run rustup installer silently
  curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y

  if [ $? -ne 0 ]; then
    echo "Failed to install Rust and Cargo."
    return 1
  fi

  # Add cargo bin to PATH for current shell session
  export PATH="$HOME/.cargo/bin:$PATH"

  # Source cargo environment script (if exists)
  if [ -f "$HOME/.cargo/env" ]; then
    source "$HOME/.cargo/env"
  fi

  # Persist PATH update in ~/.bashrc if not already added
  if ! grep -q 'export PATH="$HOME/.cargo/bin:$PATH"' ~/.bashrc 2>/dev/null; then
    echo 'export PATH="$HOME/.cargo/bin:$PATH"' >> ~/.bashrc
    echo "Added cargo bin path to ~/.bashrc"
  fi

  echo "Cargo installed successfully."
}

acquire_cargo
