#!/bin/bash
#
# Release Preparation Script
# Automates version updates and pre-release checks
#

set -euo pipefail

# Colors
readonly RED='\033[0;31m'
readonly GREEN='\033[0;32m'
readonly YELLOW='\033[1;33m'
readonly BLUE='\033[1;34m'
readonly NC='\033[0m'

# Logging functions
info() {
    echo -e "${BLUE}ℹ️  $1${NC}"
}

success() {
    echo -e "${GREEN}✅ $1${NC}"
}

warning() {
    echo -e "${YELLOW}⚠️  $1${NC}"
}

error() {
    echo -e "${RED}❌ $1${NC}"
    exit 1
}

# Validate version format
validate_version() {
    local version=$1
    if ! [[ "$version" =~ ^[0-9]+\.[0-9]+\.[0-9]+$ ]]; then
        error "Invalid version format: $version (expected: X.Y.Z)"
    fi
}

# Get current version from Cargo.toml
get_current_version() {
    grep '^version = ' Cargo.toml | head -1 | sed 's/version = "\(.*\)"/\1/'
}

# Update version in Cargo.toml
update_cargo_version() {
    local new_version=$1
    info "Updating Cargo.toml to version $new_version..."

    if [[ "$OSTYPE" == "darwin"* ]]; then
        # macOS
        sed -i '' "s/^version = \".*\"/version = \"$new_version\"/" Cargo.toml
    else
        # Linux
        sed -i "s/^version = \".*\"/version = \"$new_version\"/" Cargo.toml
    fi

    success "Updated Cargo.toml"
}

# Update version in Homebrew Formula
update_formula_version() {
    local new_version=$1

    if [[ ! -f "Formula/cmdrun.rb" ]]; then
        warning "Formula/cmdrun.rb not found, skipping"
        return
    fi

    info "Updating Formula/cmdrun.rb to version $new_version..."

    if [[ "$OSTYPE" == "darwin"* ]]; then
        sed -i '' "s/version \".*\"/version \"$new_version\"/" Formula/cmdrun.rb
        sed -i '' "s/download\/v[0-9.]*\//download\/v$new_version\//" Formula/cmdrun.rb
        sed -i '' "s/cmdrun-[0-9.]*-/cmdrun-$new_version-/g" Formula/cmdrun.rb
    else
        sed -i "s/version \".*\"/version \"$new_version\"/" Formula/cmdrun.rb
        sed -i "s/download\/v[0-9.]*\//download\/v$new_version\//" Formula/cmdrun.rb
        sed -i "s/cmdrun-[0-9.]*-/cmdrun-$new_version-/g" Formula/cmdrun.rb
    fi

    success "Updated Formula/cmdrun.rb"
}

# Run pre-release checks
run_checks() {
    info "Running pre-release checks..."

    # Format check
    info "Checking code formatting..."
    if ! cargo fmt -- --check; then
        warning "Code formatting issues found. Running cargo fmt..."
        cargo fmt
    fi
    success "Code formatting OK"

    # Clippy
    info "Running clippy..."
    cargo clippy --all-targets --all-features -- -D warnings || error "Clippy checks failed"
    success "Clippy checks passed"

    # Tests
    info "Running tests..."
    cargo test --all-features || error "Tests failed"
    success "Tests passed"

    # Security audit
    info "Running security audit..."
    if ! command -v cargo-audit &> /dev/null; then
        warning "cargo-audit not installed, skipping security check"
    else
        cargo audit || warning "Security audit found issues"
    fi

    # Build
    info "Building release binary..."
    cargo build --release || error "Release build failed"
    success "Release build successful"
}

# Update Cargo.lock
update_cargo_lock() {
    info "Updating Cargo.lock..."
    cargo build --quiet
    success "Cargo.lock updated"
}

# Create changelog entry template
suggest_changelog_update() {
    local version=$1
    local date=$(date +%Y-%m-%d)

    echo ""
    info "Don't forget to update CHANGELOG.md!"
    echo ""
    echo "Suggested changelog entry:"
    echo ""
    echo "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"
    echo "## [$version] - $date"
    echo ""
    echo "### Added"
    echo "- New feature 1"
    echo "- New feature 2"
    echo ""
    echo "### Changed"
    echo "- Change 1"
    echo "- Change 2"
    echo ""
    echo "### Fixed"
    echo "- Bug fix 1"
    echo "- Bug fix 2"
    echo ""
    echo "### Security"
    echo "- Security improvement 1"
    echo "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"
    echo ""
}

# Main preparation flow
main() {
    echo "🚀 cmdrun Release Preparation Script"
    echo ""

    # Check if in git repository
    if ! git rev-parse --git-dir > /dev/null 2>&1; then
        error "Not in a git repository"
    fi

    # Get current version
    local current_version=$(get_current_version)
    info "Current version: $current_version"

    # Get new version
    if [[ $# -eq 0 ]]; then
        echo ""
        read -p "Enter new version (X.Y.Z): " new_version
    else
        new_version=$1
    fi

    # Validate version
    validate_version "$new_version"

    # Confirm
    echo ""
    warning "Preparing release: $current_version → $new_version"
    read -p "Continue? [y/N]: " -n 1 -r
    echo

    if [[ ! $REPLY =~ ^[Yy]$ ]]; then
        info "Release preparation cancelled"
        exit 0
    fi

    echo ""
    info "Starting release preparation..."
    echo ""

    # Update versions
    update_cargo_version "$new_version"
    update_formula_version "$new_version"
    update_cargo_lock

    echo ""
    info "Running pre-release checks..."
    echo ""

    # Run checks
    run_checks

    echo ""
    success "All checks passed!"
    echo ""

    # Suggest changelog update
    suggest_changelog_update "$new_version"

    # Next steps
    echo ""
    info "Next steps:"
    echo "  1. Review changes: git diff"
    echo "  2. Update CHANGELOG.md with release notes"
    echo "  3. Commit changes: git add -A && git commit -m 'chore: prepare release v$new_version'"
    echo "  4. Create tag: git tag -a v$new_version -m 'Release v$new_version'"
    echo "  5. Push: git push origin main && git push origin v$new_version"
    echo ""

    # Ask if user wants to see diff
    read -p "Show git diff? [y/N]: " -n 1 -r
    echo

    if [[ $REPLY =~ ^[Yy]$ ]]; then
        git diff
    fi

    echo ""
    success "Release preparation complete!"
}

# Run script
main "$@"
