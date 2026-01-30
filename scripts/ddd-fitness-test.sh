#!/bin/bash
# DDD Fitness Test: Enforce Bounded Context Isolation
# Rules: 
# 1. src/{context}/* cannot import from crate::{other_context} directly.
# 2. allowed: crate::{context}::*, crate::shared::*

set -e

BCS=$(ls src | grep -vE "^(shared|main.rs|lib.rs)$")
EXIT_CODE=0

echo "🔍 Running DDD Fitness Test..."

for BC in $BCS; do
    if [ ! -d "src/$BC" ]; then continue; fi
    
    # Define other BCs that should NOT be imported
    OTHER_BCS=$(ls src | grep -vE "^($BC|shared|main.rs|lib.rs)$")
    
    for OTHER in $OTHER_BCS; do
        # Search for 'use crate::{OTHER}' in 'src/{BC}'
        # We use -r (recursive), -n (line number), -I (ignore binary)
        VIOLATIONS=$(grep -rn "use crate::$OTHER" "src/$BC" || true)
        
        if [ ! -z "$VIOLATIONS" ]; then
            echo "❌ DDD VIOLATION: Bounded Context '$BC' imports directly from '$OTHER'"
            echo "   Files found:"
            echo "$VIOLATIONS" | sed 's/^/   /'
            EXIT_CODE=1
        fi
    done
done

if [ $EXIT_CODE -eq 0 ]; then
    echo "✅ DDD Fitness Test passed! Bounded contexts are isolated."
else
    echo "❌ DDD Fitness Test failed! Please use interfaces or shared services for cross-BC communication."
fi

exit $EXIT_CODE
