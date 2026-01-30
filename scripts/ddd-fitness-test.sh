#!/bin/bash
# DDD Fitness Test: Enforce Bounded Context Isolation
# Rules: 
# 1. src/{context}/* cannot import from crate::{other_context} directly.
# 2. allowed: crate::{context}::*, crate::shared::*

set -e

BCS=$(ls src | grep -vE "^(shared|main.rs|lib.rs)$")
EXIT_CODE=0

echo "🔍 Running DDD Fitness Test..."

# Rule 1: Bounded Context Isolation
for BC in $BCS; do
    if [ ! -d "src/$BC" ]; then continue; fi
    
    OTHER_BCS=$(ls src | grep -vE "^($BC|shared|main.rs|lib.rs)$")
    
    for OTHER in $OTHER_BCS; do
        VIOLATIONS=$(grep -rn "use crate::$OTHER" "src/$BC" || true)
        
        if [ ! -z "$VIOLATIONS" ]; then
            echo "❌ DDD VIOLATION: Bounded Context '$BC' imports directly from '$OTHER'"
            echo "   Files found:"
            echo "$VIOLATIONS" | sed 's/^/   /'
            EXIT_CODE=1
        fi
    done

    # Rule 2: Domain Layer is the Core (cannot import from outer layers)
    if [ -d "src/$BC/domain" ]; then
        # Within src/BC/domain, we check for imports from application, infrastructure, or presentation
        LAYER_VIOLATIONS=$(grep -rnE "use crate::$BC::(application|infrastructure|presentation)" "src/$BC/domain" || true)
        
        if [ ! -z "$LAYER_VIOLATIONS" ]; then
            echo "❌ DDD VIOLATION: Domain layer in '$BC' depends on outer layers!"
            echo "   Files found:"
            echo "$LAYER_VIOLATIONS" | sed 's/^/   /'
            EXIT_CODE=1
        fi
    fi
done

if [ $EXIT_CODE -eq 0 ]; then
    echo "✅ DDD Fitness Test passed! Bounded contexts and domain layers are isolated."
else
    echo "❌ DDD Fitness Test failed! Please check architectural boundaries."
fi

exit $EXIT_CODE
