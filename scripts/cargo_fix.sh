# Get the root directory of the repository
REPO_ROOT=$(git rev-parse --show-toplevel)

# Find all directories containing Cargo.toml within the repository
find "$REPO_ROOT" -name Cargo.toml -exec dirname {} \; | while read -r dir; do
    # Change to the directory
    cd "$dir"
    # Run cargo check to show only errors
    cargo check --message-format=short 2>&1 | grep -E 'error:'
    # Run cargo fix
    cargo fix --allow-dirty --allow-staged
    # Change back to the root directory
    cd "$REPO_ROOT"
done