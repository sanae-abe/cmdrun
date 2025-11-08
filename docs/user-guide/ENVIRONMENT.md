# Environment Management

> **Manage multiple environments (dev, staging, prod) with different command configurations**

## Overview

cmdrun's environment management allows you to maintain different command configurations for different deployment targets or development stages. Each environment can have:

- **Different command definitions**: Override commands for specific environments
- **Environment-specific variables**: Set different API URLs, database connections, etc.
- **Isolated configurations**: Keep development and production settings separate

## Quick Start

### 1. Create Environments

```bash
# Create development environment
cmdrun env create dev --description "Development environment"

# Create production environment
cmdrun env create prod --description "Production environment"
```

### 2. Set Environment Variables

```bash
# Set variables for dev environment
cmdrun env set API_URL https://api.dev.example.com --env dev
cmdrun env set DEBUG true --env dev

# Set variables for prod environment
cmdrun env set API_URL https://api.prod.example.com --env prod
cmdrun env set DEBUG false --env prod
```

### 3. Create Environment-Specific Configurations

Create `commands.dev.toml`:

```toml
# Development environment configuration
[commands.build]
description = "Build in development mode"
cmd = "npm run build:dev"

[commands.dev-server]
description = "Start development server"
cmd = "npm run dev"
```

Create `commands.prod.toml`:

```toml
# Production environment configuration
[commands.build]
description = "Build for production"
cmd = "npm run build:prod -- --minify"

[commands.deploy]
description = "Deploy to production"
cmd = "npm run deploy"
```

### 4. Switch Between Environments

```bash
# Switch to dev environment
cmdrun env use dev

# Run commands (uses dev configuration)
cmdrun run build  # Executes "npm run build:dev"

# Switch to prod environment
cmdrun env use prod

# Run commands (uses prod configuration)
cmdrun run build  # Executes "npm run build:prod -- --minify"
```

## Configuration File Structure

### Base Configuration (`commands.toml`)

The base configuration is always loaded first:

```toml
[config]
shell = "bash"

[commands.hello]
description = "Hello command (shared across all environments)"
cmd = "echo 'Hello from cmdrun'"

[commands.build]
description = "Build command (can be overridden)"
cmd = "echo 'Default build'"
```

### Environment-Specific Configuration

Environment-specific configurations can be created in two ways:

1. **Root directory**: `commands.{env}.toml` (e.g., `commands.dev.toml`)
2. **`.cmdrun` directory**: `.cmdrun/config.{env}.toml`

Priority order (highest to lowest):
1. Environment-specific configuration
2. Base configuration (`commands.toml`)
3. Global configuration (`~/.config/cmdrun/commands.toml`)

### Environment Configuration Storage

Environment metadata is stored in `.cmdrun/config.toml`:

```toml
[environment]
current = "dev"

[environment.environments.dev]
description = "Development environment"

[environment.environments.dev.variables]
API_URL = "https://api.dev.example.com"
DEBUG = "true"

[environment.environments.prod]
description = "Production environment"

[environment.environments.prod.variables]
API_URL = "https://api.prod.example.com"
DEBUG = "false"
```

## Common Use Cases

### 1. Different Build Configurations

**Base**: `commands.toml`
```toml
[commands.build]
description = "Build project"
cmd = "npm run build"
```

**Development**: `commands.dev.toml`
```toml
[commands.build]
description = "Build with source maps"
cmd = "npm run build -- --sourcemap"
```

**Production**: `commands.prod.toml`
```toml
[commands.build]
description = "Build optimized production bundle"
cmd = "npm run build -- --minify --no-sourcemap"
```

### 2. Environment-Specific Database Connections

```bash
# Development environment
cmdrun env set DB_HOST localhost --env dev
cmdrun env set DB_PORT 5432 --env dev
cmdrun env set DB_NAME myapp_dev --env dev

# Production environment
cmdrun env set DB_HOST prod-db.example.com --env prod
cmdrun env set DB_PORT 5432 --env prod
cmdrun env set DB_NAME myapp_prod --env prod
```

**Shared command** (`commands.toml`):
```toml
[commands.db-migrate]
description = "Run database migrations"
cmd = "psql -h ${DB_HOST} -p ${DB_PORT} -d ${DB_NAME} -f migrations/latest.sql"
```

### 3. Environment-Only Commands

**Development**: `commands.dev.toml`
```toml
[commands.seed-db]
description = "Seed database with test data"
cmd = "node scripts/seed-database.js"

[commands.test-watch]
description = "Run tests in watch mode"
cmd = "npm run test:watch"
```

**Production**: `commands.prod.toml`
```toml
[commands.deploy]
description = "Deploy to production server"
cmd = "npm run deploy"

[commands.rollback]
description = "Rollback to previous deployment"
cmd = "npm run rollback"
```

### 4. Multi-Stage Deployment Pipeline

```bash
# Create staging environment
cmdrun env create staging --description "Staging environment"
cmdrun env set DEPLOY_TARGET staging.example.com --env staging

# Create production environment
cmdrun env create prod --description "Production environment"
cmdrun env set DEPLOY_TARGET prod.example.com --env prod
```

**Shared deployment** (`commands.toml`):
```toml
[commands.deploy]
description = "Deploy to target server"
cmd = "scp -r dist/ user@${DEPLOY_TARGET}:/var/www/app"
```

## Environment Management Commands

### Create Environment

```bash
cmdrun env create <name> [--description <desc>]
```

**Example**:
```bash
cmdrun env create staging --description "Staging environment for QA"
```

### List Environments

```bash
cmdrun env list
```

**Output**:
```
Available environments:

  → default - Default environment
    dev - Development environment
    staging - Staging environment for QA
    prod - Production environment
```

### Switch Environment

```bash
cmdrun env use <name>
```

**Example**:
```bash
cmdrun env use dev
```

### Show Current Environment

```bash
cmdrun env current
```

**Output**:
```
Current environment:
  dev
```

### Set Environment Variable

```bash
cmdrun env set <KEY> <VALUE> [--env <name>]
```

**Examples**:
```bash
# Set for current environment
cmdrun env set NODE_ENV development

# Set for specific environment
cmdrun env set NODE_ENV production --env prod
```

### View Environment Information

```bash
cmdrun env info [name]
```

**Output**:
```
Environment: dev

  Description: Development environment

  Environment variables:
    API_URL = https://api.dev.example.com
    DEBUG = true

  Configuration files:
    Base config: commands.toml
    Environment config: /path/to/commands.dev.toml (found)
```

## Best Practices

### 1. Use Meaningful Environment Names

```bash
# Good
cmdrun env create dev
cmdrun env create staging
cmdrun env create prod

# Avoid
cmdrun env create env1
cmdrun env create test123
```

### 2. Keep Sensitive Data Out of Configuration Files

```bash
# Store API keys in environment variables, not config files
cmdrun env set API_KEY ${your_api_key} --env prod

# Reference them in commands
```

**commands.toml**:
```toml
[commands.deploy]
cmd = "curl -H 'Authorization: Bearer ${API_KEY}' ..."
```

### 3. Use Base Configuration for Shared Commands

Keep commonly-used commands in `commands.toml` and only override when necessary.

**Base** (`commands.toml`):
```toml
[commands.test]
description = "Run tests"
cmd = "npm test"

[commands.lint]
description = "Run linter"
cmd = "npm run lint"
```

**Development** (`commands.dev.toml`):
```toml
# Only override what's different
[commands.test]
cmd = "npm test -- --watch"
```

### 4. Document Environment-Specific Behavior

Add comments in environment-specific configuration files:

```toml
# Production configuration - optimized builds, no debug output

[commands.build]
description = "Production build with optimizations"
cmd = "npm run build -- --minify --tree-shaking"
```

### 5. Use Version Control

```bash
# Track environment configurations
git add commands.toml commands.dev.toml commands.prod.toml

# Don't track environment state and variables
echo ".cmdrun/config.toml" >> .gitignore
```

## Advanced Usage

### Combining Environment Variables and Command Variables

```toml
[commands.deploy]
description = "Deploy to environment"
cmd = "deploy.sh --env ${ENV_NAME} --api-url ${API_URL} --version ${1}"
```

Usage:
```bash
cmdrun env set ENV_NAME staging --env staging
cmdrun run deploy v1.2.3  # ${1} = v1.2.3
```

### Environment-Specific Working Directories

**Development**:
```toml
[commands.start]
working_dir = "./dev-server"
cmd = "npm start"
```

**Production**:
```toml
[commands.start]
working_dir = "./dist"
cmd = "node server.js"
```

### Conditional Environment Detection

You can check the current environment within scripts:

```bash
# In a shell script referenced by cmdrun
if [ "$ENV_NAME" = "prod" ]; then
    echo "Running production build..."
else
    echo "Running development build..."
fi
```

## Troubleshooting

### Environment Not Switching

**Problem**: Commands don't change when switching environments

**Solution**:
1. Verify environment config file exists:
   ```bash
   cmdrun env info dev
   ```
2. Check file naming: `commands.{env}.toml` (e.g., `commands.dev.toml`)
3. Verify current environment:
   ```bash
   cmdrun env current
   ```

### Variables Not Expanding

**Problem**: `${VAR}` not replaced in commands

**Solution**:
1. Check variable is set:
   ```bash
   cmdrun env info
   ```
2. Set variable if missing:
   ```bash
   cmdrun env set VAR value
   ```
3. Use correct syntax: `${VAR}` not `$VAR`

### Configuration File Not Found

**Problem**: `Environment config: /path/to/commands.dev.toml (not found)`

**Solution**:
1. Create the configuration file:
   ```bash
   touch commands.dev.toml
   ```
2. Add environment-specific commands
3. Verify with:
   ```bash
   cmdrun env info dev
   ```

## Examples

### Complete Example: Web Application

**Setup**:
```bash
# Create environments
cmdrun env create dev --description "Local development"
cmdrun env create staging --description "Staging server"
cmdrun env create prod --description "Production server"

# Set environment variables
cmdrun env set API_URL http://localhost:3000 --env dev
cmdrun env set API_URL https://staging-api.example.com --env staging
cmdrun env set API_URL https://api.example.com --env prod
```

**Base Configuration** (`commands.toml`):
```toml
[commands.install]
description = "Install dependencies"
cmd = "npm install"

[commands.test]
description = "Run tests"
cmd = "npm test"
```

**Development** (`commands.dev.toml`):
```toml
[commands.start]
description = "Start dev server"
cmd = "npm run dev"

[commands.build]
description = "Development build"
cmd = "npm run build:dev"
```

**Production** (`commands.prod.toml`):
```toml
[commands.build]
description = "Production build"
cmd = "npm run build:prod"

[commands.deploy]
description = "Deploy to production"
cmd = "npm run deploy"
```

**Usage**:
```bash
# Development workflow
cmdrun env use dev
cmdrun run install
cmdrun run start

# Production deployment
cmdrun env use prod
cmdrun run build
cmdrun run deploy
```

## See Also

- [Configuration Guide](CONFIGURATION.md)
- [CLI Reference](CLI.md)
- [Recipes](RECIPES.md)
