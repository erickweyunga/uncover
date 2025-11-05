#!/bin/bash

# Script to run Uncover framework benchmarks
# Usage: ./scripts/run_benchmarks.sh [benchmark_names...]

set -e

# Colors for output
GREEN='\033[0;32m'
BLUE='\033[0;34m'
YELLOW='\033[1;33m'
NC='\033[0m' # No Color

echo -e "${BLUE}=== Uncover Framework Benchmarks ===${NC}\n"

# Check if rewrk is installed
if ! command -v rewrk &> /dev/null; then
    echo -e "${YELLOW}rewrk is not installed.${NC}"
    echo -e "Installing rewrk from GitHub...\n"
    cargo install rewrk --git https://github.com/ChillFish8/rewrk.git
    echo -e "\n${GREEN}rewrk installed successfully!${NC}\n"
fi

# Run benchmarks
echo -e "${GREEN}Running benchmarks...${NC}\n"

if [ $# -eq 0 ]; then
    # No arguments - run all benchmarks
    echo -e "Running ${BLUE}all benchmarks${NC}"
    cargo bench --bench framework
else
    # Run specific benchmarks
    echo -e "Running benchmarks: ${BLUE}$*${NC}"
    cargo bench --bench framework -- "$@"
fi

echo -e "\n${GREEN}Benchmarks complete!${NC}"
