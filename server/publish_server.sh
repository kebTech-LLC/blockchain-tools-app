#!/bin/bash

# Retry logic for Docker builds
RETRIES=5
i=1

# Default tag is "latest"
TAG="latest"

# Check if -t argument is provided
while getopts ":t:" opt; do
  case $opt in
    t)
      TAG=$OPTARG
      ;;
    \?)
      echo "Invalid option: -$OPTARG" >&2
      exit 1
      ;;
    :)
      echo "Option -$OPTARG requires an argument." >&2
      exit 1
      ;;
  esac
done

#!/bin/bash

# Variables
LOCAL_DEPENDENCIES="./local_dependencies"
MAIN_CARGO_TOML="./Cargo.toml"
MODIFIED_CARGO_TOML="$LOCAL_DEPENDENCIES/Cargo.toml.modified"

# Clean up and recreate the local dependencies directory
rm -rf "$LOCAL_DEPENDENCIES"
mkdir -p "$LOCAL_DEPENDENCIES"

# Function to resolve dependencies from a Cargo.toml
resolve_dependencies() {
  local cargo_toml=$1
  local current_dir=$(dirname "$cargo_toml")

  echo "Resolving dependencies for Cargo.toml: $cargo_toml"
  echo "Current directory: $current_dir"

  # Parse path dependencies in the Cargo.toml
  grep -E 'path *= *".*"' "$cargo_toml" | while read -r line; do
    echo "Processing line: $line"

    # Extract the raw path
    local raw_path=$(echo "$line" | sed -E 's/.*path *= *"(.*)".*/\1/' | xargs)
    echo "Raw path: $raw_path"

    # Resolve absolute path
    local abs_path=$(realpath "$current_dir/$raw_path" 2>/dev/null)
    echo "Absolute path: $abs_path"

    # Skip if the path doesn't exist
    if [ ! -d "$abs_path" ]; then
      echo "Error: Path $raw_path does not exist. Skipping."
      continue
    fi

    # Get folder name
    local folder_name=$(basename "$abs_path")
    echo "Folder name: $folder_name"

    # Copy the crate to `local_dependencies`
    if [ ! -d "$LOCAL_DEPENDENCIES/$folder_name" ]; then
      echo "Copying $folder_name from $abs_path to $LOCAL_DEPENDENCIES/$folder_name"
      cp -r "$abs_path" "$LOCAL_DEPENDENCIES/$folder_name"

      # Recursively resolve dependencies for the copied crate
      if [ -f "$abs_path/Cargo.toml" ]; then
        resolve_dependencies "$abs_path/Cargo.toml"
      fi
    else
      echo "Folder $folder_name already exists in $LOCAL_DEPENDENCIES. Skipping copy."
    fi
  done
}

# Step 1: Resolve dependencies for the main Cargo.toml
resolve_dependencies "$MAIN_CARGO_TOML"

# Step 2: Rewrite Cargo.toml files in local_dependencies
echo "Rewriting Cargo.toml files in local_dependencies..."
find "$LOCAL_DEPENDENCIES" -name "Cargo.toml" | while read -r dep_toml; do
  echo "Rewriting $dep_toml"
  sed -i -E "s|path *= *\".*\"|path = \"../$(basename $(dirname $dep_toml))\"|g" "$dep_toml"
done

# Step 3: Create a modified Cargo.toml for the main project
echo "Creating modified Cargo.toml at $MODIFIED_CARGO_TOML"
cp "$MAIN_CARGO_TOML" "$MODIFIED_CARGO_TOML"
grep -E 'path *= *".*"' "$MAIN_CARGO_TOML" | while read -r line; do
  # Extract raw path and folder name
  raw_path=$(echo "$line" | sed -E 's/.*path *= *"(.*)".*/\1/' | xargs)
  folder_name=$(basename "$raw_path")

  # Replace the path with `local_dependencies`
  sed -i -E "s|path *= *\"$raw_path\"|path = \"./local_dependencies/$folder_name\"|g" "$MODIFIED_CARGO_TOML"
done

echo "Dependency resolution and rewriting complete."

# Step 4: Build and push the Docker image
until [ $i -gt $RETRIES ]; do
  if docker build --build-arg LOCAL_DEPENDENCIES="$LOCAL_DEPENDENCIES" --platform=linux/amd64 --tag kebtech/blockchain-tools-server:$TAG --file Dockerfile .; then
    if docker push kebtech/blockchain-tools-server:$TAG; then
      # Remove only the image with the specific tag
      docker rmi kebtech/blockchain-tools-server:$TAG
      docker image prune -f --filter "dangling=true"
      docker system prune -f --filter "until=30m" --filter "label=maintainer=kebtech/blockchain-tools-server"
      break
    else
      echo "Push failed"
    fi
  else
    echo "Build failed, retrying..."
    i=$((i+1))
    sleep 15
  fi
done

# Clean up local dependencies
rm -rf "$LOCAL_DEPENDENCIES"
