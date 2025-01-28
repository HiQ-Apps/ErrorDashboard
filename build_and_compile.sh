#!/bin/bash

# Exit script on any error
set -e

echo "Navigating to the frontend directory..."
cd ./ErrorDashboardClient/client
echo "Running npm build..."
npm run build

echo "Navigating to the server directory..."
cd ../../ErrorDashboardServer
echo "Compiling the server..."

shuttle --debug run

echo "Build and compilation completed successfully."
