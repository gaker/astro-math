#!/bin/bash
# Script to run Python tests for astro-math bindings

set -e

echo "Building astro-math Python package..."
maturin develop

echo "Installing test dependencies..."
pip install pytest pytest-cov numpy

echo "Running tests..."
python -m pytest tests/ -v --cov=astro_math --cov-report=term-missing --cov-report=html:coverage_html

echo "Coverage report saved to coverage_html/index.html"