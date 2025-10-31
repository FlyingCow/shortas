#!/bin/bash
# Benchmark runner script for click-tracker optimizations
# Usage: ./run_benchmarks.sh [options]

set -e

RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
NC='\033[0m' # No Color

echo -e "${BLUE}================================================${NC}"
echo -e "${BLUE}  Click Tracker Performance Benchmarks${NC}"
echo -e "${BLUE}================================================${NC}"
echo ""

# Check if cargo is available
if ! command -v cargo &> /dev/null; then
    echo -e "${RED}Error: cargo not found. Please install Rust.${NC}"
    exit 1
fi

# Function to run benchmarks
run_bench() {
    local bench_name=$1
    local description=$2

    echo -e "${YELLOW}Running: $description${NC}"
    echo "Benchmark: $bench_name"
    echo ""

    if cargo bench --bench "$bench_name" 2>&1 | tee "benchmark_${bench_name}.log"; then
        echo -e "${GREEN}✓ Completed: $description${NC}"
        echo ""
    else
        echo -e "${RED}✗ Failed: $description${NC}"
        echo ""
        return 1
    fi
}

# Parse command line arguments
BENCH_SUITE=""
BASELINE=""
SAVE_BASELINE=""

while [[ $# -gt 0 ]]; do
    case $1 in
        --suite)
            BENCH_SUITE="$2"
            shift 2
            ;;
        --baseline)
            BASELINE="$2"
            shift 2
            ;;
        --save-baseline)
            SAVE_BASELINE="$2"
            shift 2
            ;;
        --help)
            echo "Usage: $0 [options]"
            echo ""
            echo "Options:"
            echo "  --suite <name>           Run specific benchmark suite (user_agent_parsing, aggregate_module, context_creation)"
            echo "  --baseline <name>        Compare against saved baseline"
            echo "  --save-baseline <name>   Save results as baseline"
            echo "  --help                   Show this help message"
            echo ""
            echo "Examples:"
            echo "  $0                                  # Run all benchmarks"
            echo "  $0 --suite user_agent_parsing       # Run specific suite"
            echo "  $0 --save-baseline main             # Save baseline for comparison"
            echo "  $0 --baseline main                  # Compare against baseline"
            exit 0
            ;;
        *)
            echo -e "${RED}Unknown option: $1${NC}"
            echo "Use --help for usage information"
            exit 1
            ;;
    esac
done

# Check if project compiles
echo -e "${YELLOW}Checking if project compiles...${NC}"
if ! cargo check --benches 2>&1 | grep -q "Finished"; then
    echo -e "${RED}Error: Project does not compile. Please fix compilation errors first.${NC}"
    echo ""
    echo "Common issues:"
    echo "  - Check src/core/conversion.rs for missing module declarations"
    echo "  - Run 'cargo check' for detailed error messages"
    exit 1
fi
echo -e "${GREEN}✓ Project compiles successfully${NC}"
echo ""

# Build benchmark arguments
BENCH_ARGS=""
if [ -n "$BASELINE" ]; then
    BENCH_ARGS="$BENCH_ARGS -- --baseline $BASELINE"
fi
if [ -n "$SAVE_BASELINE" ]; then
    BENCH_ARGS="$BENCH_ARGS -- --save-baseline $SAVE_BASELINE"
fi

# Run benchmarks
if [ -z "$BENCH_SUITE" ]; then
    # Run all benchmarks
    echo -e "${BLUE}Running all benchmark suites...${NC}"
    echo ""

    run_bench "user_agent_parsing" "User Agent Parsing Optimization"
    run_bench "aggregate_module" "Aggregate Module String Cloning"
    run_bench "context_creation" "Context Creation HashMap Allocation"
    run_bench "full_pipeline" "Full Pipeline End-to-End (No External Deps)"

else
    # Run specific suite
    case $BENCH_SUITE in
        user_agent_parsing|aggregate_module|context_creation|full_pipeline)
            run_bench "$BENCH_SUITE" "Benchmark: $BENCH_SUITE"
            ;;
        *)
            echo -e "${RED}Error: Unknown benchmark suite: $BENCH_SUITE${NC}"
            echo "Available suites: user_agent_parsing, aggregate_module, context_creation, full_pipeline"
            exit 1
            ;;
    esac
fi

echo -e "${BLUE}================================================${NC}"
echo -e "${GREEN}✓ All benchmarks completed!${NC}"
echo -e "${BLUE}================================================${NC}"
echo ""
echo "Results saved to:"
echo "  - target/criterion/ (detailed HTML reports)"
echo "  - benchmark_*.log (console output)"
echo ""
echo "View HTML reports:"
echo "  firefox target/criterion/report/index.html"
echo ""
echo "For detailed documentation, see:"
echo "  - BENCHMARKS.md"
echo "  - PERFORMANCE_OPTIMIZATIONS.md"
echo "  - OPTIMIZATION_SUMMARY.md"
