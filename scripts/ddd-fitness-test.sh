#!/bin/bash
# DDD Fitness Test: Enforced Architectural Boundaries
# This script ensures that the codebase adheres to Bounded Context isolation 
# and Clean Architecture layer dependencies.

set -e

BCS=$(ls src | grep -vE "^(shared|main.rs|lib.rs)$")
EXIT_CODE=0

# Colors for output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
NC='\033[0m' # No Color

echo -e "${YELLOW}🔍 Running DDD & Clean Architecture Fitness Test...${NC}\n"

function check_violation() {
    local rule_name=$1
    local search_path=$2
    local pattern=$3
    local message=$4
    local exclude_pattern=$5
    
    if [ ! -z "$exclude_pattern" ]; then
        VIOLATIONS=$(grep -rnE "$pattern" "$search_path" | grep -vE "$exclude_pattern" || true)
    else
        VIOLATIONS=$(grep -rnE "$pattern" "$search_path" || true)
    fi
    
    if [ ! -z "$VIOLATIONS" ]; then
        echo -e "${RED}❌ VIOLATION: $rule_name${NC}"
        echo -e "   ${YELLOW}Reason:${NC} $message"
        echo "$VIOLATIONS" | sed 's/^/   /'
        EXIT_CODE=1
    fi
}

# 1. Bounded Context Isolation
echo "Checking Bounded Context Isolation..."
for BC in $BCS; do
    if [ ! -d "src/$BC" ]; then continue; fi
    OTHER_BCS=$(echo "$BCS" | grep -vE "^$BC$")
    
    for OTHER in $OTHER_BCS; do
        check_violation \
            "Cross-Context Leakage" \
            "src/$BC" \
            "(\b$OTHER::|crate::$OTHER\b)" \
            "Context '$BC' cannot reference or import directly from '$OTHER'. Use 'shared' or events."
    done
done

# 2. Layered Architecture: Domain Purity
echo "Checking Domain Layer Purity..."
ALL_DOMAINS=$(find src -type d -name "domain")
for DOMAIN_PATH in $ALL_DOMAINS; do
    # Extract BC name if it's not shared
    BC_NAME=$(echo "$DOMAIN_PATH" | cut -d'/' -f2)

    # Rule: Domain cannot depend on Application, Infra, or Presentation (only for Bounded Contexts)
    if [[ "$BC_NAME" != "shared" ]]; then
        check_violation \
            "Domain Layer Leak" \
            "$DOMAIN_PATH" \
            "use crate::$BC_NAME::(application|infrastructure|presentation)" \
            "Domain layer in '$BC_NAME' depends on outer layers!"
    fi

    # Rule: No Infrastructure libraries in Domain
    # We allow chrono for date types, but not for parsing errors or specific infra types
    check_violation \
        "Infrastructure Leak in Domain" \
        "$DOMAIN_PATH" \
        "(use (sqlx|reqwest|postgres|diesel|warp|axum|hyper|redis|serde)|sqlx::|chrono::(ParseError|format))" \
        "Domain layer contains infrastructure-specific dependencies or types (sqlx, axum, chrono::ParseError, etc.)."

    # Rule: No panics or unwraps in Domain (use Result)
    check_violation \
        "Unsafe operations in Domain" \
        "$DOMAIN_PATH" \
        "\.(unwrap|expect)\(" \
        "Domain layer should use Result handling instead of panics (unwrap/expect)."
done

# 3. Layered Architecture: Application Layer Isolation
echo "Checking Application Layer Isolation..."
for BC in $BCS; do
    if [ -d "src/$BC/application" ]; then
        # Rule: Application cannot depend on Infrastructure or Presentation
        check_violation \
            "Application Layer Leak" \
            "src/$BC/application" \
            "use crate::$BC::(infrastructure|presentation)" \
            "Application layer in '$BC' depends on outer layers!"
    fi
done

# 4. Layered Architecture: Presentation Isolation
echo "Checking Presentation Layer Isolation..."
for BC in $BCS; do
    if [ -d "src/$BC/presentation" ]; then
        # Rule: Presentation should not call Infrastructure directly
        check_violation \
            "Presentation calls Infrastructure" \
            "src/$BC/presentation" \
            "use crate::$BC::infrastructure" \
            "Presentation layer should go through Application layer, not direct to Infrastructure."
    fi
done

# 5. Shared Layer Purity
echo "Checking Shared Layer Purity..."
if [ -d "src/shared" ]; then
    # Rule: Shared/Domain should not depend on any Bounded Context
    if [ -d "src/shared/domain" ]; then
        for BC in $BCS; do
            check_violation \
                "Shared Domain Leak" \
                "src/shared/domain" \
                "use crate::$BC" \
                "Shared Domain cannot depend on specific Bounded Context '$BC'."
        done
    fi

    # Rule: Shared should not import from Infrastructure of any BC (even in factory)
    # This is negotiable, but usually Shared Infra should be generic.
    for BC in $BCS; do
        check_violation \
            "Shared calling BC Infrastructure" \
            "src/shared" \
            "use crate::$BC::infrastructure" \
            "Shared layer should not depend on specific BC infrastructure. Use dependency injection." \
            "test_factory.rs"
    done
fi

echo -e "\n--------------------------------------------------"
if [ $EXIT_CODE -eq 0 ]; then
    echo -e "${GREEN}✅ DDD Fitness Test passed! Architectural boundaries are solid.${NC}"
else
    echo -e "${RED}❌ DDD Fitness Test failed! Please fix the violations listed above.${NC}"
fi
echo "--------------------------------------------------"

exit $EXIT_CODE
