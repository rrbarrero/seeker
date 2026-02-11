#!/bin/bash
set -e

# Load environment variables from .env
if [ -f .env ]; then
    export $(grep -v '^#' .env | xargs)
fi

CONTAINER_NAME="best-seeker-garage"
BUCKET_NAME=${S3_SCRAPER_BUCKET:-scraper-data}
DEV_KEY="dev-key"

echo "Starting Garage provisioning..."

# 1. Wait for Garage to be ready
echo "Checking Garage status..."
timeout=30
while ! docker exec $CONTAINER_NAME /garage status > /dev/null 2>&1; do
  if [ $timeout -le 0 ]; then
    echo "Timed out waiting for Garage."
    exit 1
  fi
  echo "Waiting for Garage container..."
  sleep 2
  timeout=$((timeout - 2))
done

# 2. Check and Apply Layout if needed
STATUS=$(docker exec $CONTAINER_NAME /garage status 2>&1)
if echo "$STATUS" | grep -q "NO ROLE ASSIGNED"; then
    echo "Initializing Garage Layout..."
    NODE_ID=$(echo "$STATUS" | grep "NO ROLE ASSIGNED" | head -n 1 | awk '{print $1}')
    
    if [ -n "$NODE_ID" ]; then
        # Assign layout
        docker exec $CONTAINER_NAME /garage layout assign -z dc1 -c 100M "$NODE_ID"
        
        # Determine next version to apply
        CURRENT_VER=$(docker exec $CONTAINER_NAME /garage layout show | grep "Version:" | head -n 1 | awk '{print $2}')
        CURRENT_VER=${CURRENT_VER:-0}
        NEXT_VER=$((CURRENT_VER + 1))
        
        docker exec $CONTAINER_NAME /garage layout apply --version "$NEXT_VER"
        echo "Layout initialized (Version $NEXT_VER)."
    fi
else
    echo "Garage layout already active."
fi

# 3. Create Key and Update .env (CRITICAL: Sync credentials)
# We delete specific key to force regeneration so we have the secret
echo "Refreshing key '$DEV_KEY'..."
docker exec $CONTAINER_NAME /garage key delete $DEV_KEY > /dev/null 2>&1 || true
KEY_OUTPUT=$(docker exec $CONTAINER_NAME /garage key create $DEV_KEY)
echo "Key '$DEV_KEY' recreated."

# Extract credentials
NEW_ACCESS_KEY=$(echo "$KEY_OUTPUT" | grep "Key ID" | awk '{print $3}')
NEW_SECRET_KEY=$(echo "$KEY_OUTPUT" | grep "Secret key" | awk '{print $3}')

if [ -n "$NEW_ACCESS_KEY" ] && [ -n "$NEW_SECRET_KEY" ]; then
    echo "Updating .env with new credentials..."
    if [ -f .env ]; then
        sed -i "s/^AWS_ACCESS_KEY_ID=.*/AWS_ACCESS_KEY_ID=$NEW_ACCESS_KEY/" .env
        sed -i "s/^AWS_SECRET_ACCESS_KEY=.*/AWS_SECRET_ACCESS_KEY=$NEW_SECRET_KEY/" .env
    else
        echo ".env file not found!"
    fi
else
    echo "Failed to extract keys. Check Garage output: $KEY_OUTPUT"
fi

# 4. Ensure S3_SCRAPER_BUCKET exists
echo "Ensuring bucket '$BUCKET_NAME' exists..."
if ! docker exec $CONTAINER_NAME /garage bucket info $BUCKET_NAME > /dev/null 2>&1; then
    docker exec $CONTAINER_NAME /garage bucket create $BUCKET_NAME
    echo "Bucket '$BUCKET_NAME' created."
else
    echo "Bucket '$BUCKET_NAME' already exists."
fi

# 5. Grant permissions
echo "Granting permissions..."
docker exec $CONTAINER_NAME /garage bucket allow --read --write --owner $BUCKET_NAME --key $DEV_KEY
echo "Permissions granted."

echo "Garage provisioning completed successfully."
