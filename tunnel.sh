#!/bin/bash

if [ $# -eq 0 ]; then
    echo "Usage: $0 <message>"
    exit 1
fi

message="$*"

# Ensure cargo-lambda is installed
if ! command -v cargo-lambda &> /dev/null; then
    echo "cargo-lambda is not installed. Please install it first."
    echo "You can install it using: cargo install cargo-lambda"
    exit 1
fi

# Invoke the Lambda function
cargo lambda invoke \
    --data-ascii "{\"message\": \"$message\"}" \
    --output-format json

if [ $? -eq 0 ]; then
    echo "Message sent successfully"
else
    echo "Failed to send message"
fi