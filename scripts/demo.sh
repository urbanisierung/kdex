#!/usr/bin/env bash
# Demo script for kdex - record with asciinema
#
# Usage:
#   1. Install asciinema: pip install asciinema
#   2. Install kdex: cargo install --path .
#   3. Record: asciinema rec --command="./scripts/demo.sh" demo.cast
#   4. Upload: asciinema upload demo.cast
#
# Or use asciinema-automation for fully scripted recording.

set -e

# Colors
CYAN='\033[0;36m'
GREEN='\033[0;32m'
YELLOW='\033[0;33m'
NC='\033[0m' # No Color

# Typing simulation delay (seconds between chars)
DELAY=0.03

# Simulate typing
type_cmd() {
    echo -en "${GREEN}❯${NC} "
    for (( i=0; i<${#1}; i++ )); do
        echo -n "${1:$i:1}"
        sleep $DELAY
    done
    echo
    sleep 0.3
}

# Run command with simulated typing
run() {
    type_cmd "$1"
    eval "$1"
    sleep 1
}

# Section header
section() {
    echo
    echo -e "${CYAN}━━━ $1 ━━━${NC}"
    echo
    sleep 0.5
}

# Clear and start
clear
echo -e "${YELLOW}kdex${NC} - Knowledge DEX: Fast CLI for indexing code and knowledge bases"
echo
sleep 2

# Show help
section "Getting Started"
run "kdex --help"
sleep 2

# Index a project
section "Indexing a Repository"
echo -e "${CYAN}Let's index a sample project...${NC}"
sleep 1

# Create a sample project for demo
DEMO_DIR=$(mktemp -d)
mkdir -p "$DEMO_DIR/src"
cat > "$DEMO_DIR/README.md" << 'EOF'
# Sample Project

This is a demo project for kdex.

## Features
- Fast indexing
- Semantic search
- AI-ready output
EOF

cat > "$DEMO_DIR/src/main.rs" << 'EOF'
fn main() {
    println!("Hello, kdex!");
}

fn greet(name: &str) -> String {
    format!("Hello, {}!", name)
}
EOF

cat > "$DEMO_DIR/notes.md" << 'EOF'
# Developer Notes

## How to kill a process on a port

```bash
# Find process using port 3000
lsof -i :3000

# Kill the process
kill -9 <PID>

# Or in one command
kill -9 $(lsof -t -i:3000)
```

## Debugging tips
- Use RUST_BACKTRACE=1 for stack traces
- Check logs in ~/.config/kdex/
EOF

run "kdex index $DEMO_DIR --name demo-project"
sleep 2

# List indexed repos
section "Listing Repositories"
run "kdex list"
sleep 2

# Search
section "Searching Content"
run "kdex search 'greet'"
sleep 2

run "kdex search 'kill port'"
sleep 2

# JSON output
section "JSON Output for Scripting"
run "kdex search 'greet' --json | head -20"
sleep 2

# Clean up
section "Cleanup"
run "kdex remove $DEMO_DIR --force"
sleep 1

# MCP integration
section "AI Integration (MCP)"
echo -e "${CYAN}Start the MCP server for AI assistants:${NC}"
sleep 1
type_cmd "kdex mcp"
echo
echo -e "${YELLOW}The MCP server runs on stdio for tools like GitHub Copilot CLI and Claude Desktop.${NC}"
sleep 2

# Done
echo
echo -e "${GREEN}✓${NC} Demo complete! Install with: ${CYAN}cargo install --path .${NC}"
echo
sleep 2

# Cleanup temp dir
rm -rf "$DEMO_DIR"
