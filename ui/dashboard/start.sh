#!/bin/bash

# Shortas Dashboard Start Script

echo "🚀 Starting Shortas Dashboard..."

# Check if node_modules exists
if [ ! -d "node_modules" ]; then
    echo "📦 Installing dependencies..."
    npm install
fi

# Check if .env.local exists
if [ ! -f ".env.local" ]; then
    echo "⚙️ Creating .env.local from template..."
    cp env.example .env.local
    echo "✏️ Please edit .env.local with your configuration"
fi

# Start the development server
echo "🌟 Starting development server..."
npm start


