#!/bin/bash

# Simple Jekyll server script for Vector.dev theme
# This script serves the documentation without requiring bundle

echo "🚀 Starting Shortas Documentation Server with Vector.dev Theme"
echo "📍 Server will be available at: http://localhost:4000"
echo "🎨 Vector.dev inspired theme is now the default"
echo ""

# Check if Jekyll is installed
if ! command -v jekyll &> /dev/null; then
    echo "❌ Jekyll not found. Installing Jekyll..."
    sudo gem install jekyll
fi

# Start Jekyll server
echo "🔄 Starting Jekyll server..."
jekyll serve --host 0.0.0.0 --port 4000 --drafts --incremental

echo ""
echo "✅ Documentation server started successfully!"
echo "🌐 Open http://localhost:4000 in your browser"
echo "🎨 Vector.dev theme is applied by default"
