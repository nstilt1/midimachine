#!/bin/bash

# Path to your settings.json file
settings_file="../.vscode/settings.json"

# Read current target
current_target=$(grep '"target":' $settings_file | awk -F'"' '{print $4}')

# Set new target
if [ "$current_target" == "wasm32-unknown-unknown" ]; then
  new_target="x86_64-unknown-linux-gnu"
else
  new_target="wasm32-unknown-unknown"
fi

# Update settings.json
sed -i "s/\"target\": \"$current_target\"/\"target\": \"$new_target\"/" $settings_file

echo "Switched target to $new_target"
