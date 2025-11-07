# cmdrun Recipes & Best Practices

A collection of practical examples and use cases for cmdrun.

## Table of Contents

- [Web Development](#web-development)
- [Backend Development](#backend-development)
- [DevOps & Infrastructure](#devops--infrastructure)
- [Data Science](#data-science)
- [System Administration](#system-administration)
- [Personal Productivity](#personal-productivity)
- [Best Practices](#best-practices)

---

## Web Development

### Frontend Development Workflow

```toml
# Frontend project commands
[config]
language = "english"

[config.env]
NODE_ENV = "development"

# Development
[commands.dev]
description = "Start development server with HMR"
cmd = "npm run dev"
env = { PORT = "3000", BROWSER = "none" }

[commands.dev:watch]
description = "Auto-reload on file changes"
cmd = "npm run dev"
# Then use: cmdrun watch dev:watch --pattern "src/**/*.{ts,tsx}"

# Building
[commands.build]
description = "Production build"
cmd = "npm run build"
deps = ["lint", "test"]

[commands.build:analyze]
description = "Build with bundle analyzer"
cmd = "npm run build -- --analyze"

# Testing
[commands.test]
description = "Run tests"
cmd = "npm test"

[commands.test:watch]
description = "Run tests in watch mode"
cmd = "npm test -- --watch"

[commands.test:coverage]
description = "Generate coverage report"
cmd = "npm test -- --coverage"

# Quality Checks
[commands.lint]
description = "Run ESLint"
cmd = "npm run lint"

[commands.lint:fix]
description = "Auto-fix ESLint issues"
cmd = "npm run lint -- --fix"

[commands.type-check]
description = "TypeScript type checking"
cmd = "tsc --noEmit"

[commands.format]
description = "Format code with Prettier"
cmd = "npx prettier --write ."

# Complete quality check
[commands.check-all]
description = "Run all quality checks in parallel"
parallel = true
cmd = [
    "npm run lint",
    "tsc --noEmit",
    "npm test -- --passWithNoTests"
]

# Deployment
[commands.deploy:staging]
description = "Deploy to staging"
cmd = [
    "npm run build",
    "rsync -avz dist/ ${STAGING_USER}@${STAGING_HOST}:/var/www/staging"
]
deps = ["check-all"]
confirm = true

[commands.deploy:production]
description = "Deploy to production"
cmd = [
    "npm run build",
    "rsync -avz dist/ ${PROD_USER}@${PROD_HOST}:/var/www/production"
]
deps = ["check-all"]
confirm = true

# Hooks
[hooks.commands.deploy:production]
pre_run = "git diff --exit-code && git diff --cached --exit-code"
post_run = "echo $(date) - Deployed to production >> deploy.log"
```

**Usage:**
```bash
# Development
cmdrun run dev

# Watch and rebuild
cmdrun watch dev:watch --pattern "src/**/*.{ts,tsx}"

# Pre-commit checks
cmdrun run check-all

# Deploy
export PROD_USER="deploy"
export PROD_HOST="production.example.com"
cmdrun run deploy:production
```

---

### Full-Stack Development

```toml
# Frontend
[commands.frontend:dev]
description = "Start frontend dev server"
cmd = "npm run dev"
working_dir = "./frontend"
env = { PORT = "3000" }

[commands.frontend:build]
description = "Build frontend"
cmd = "npm run build"
working_dir = "./frontend"

# Backend
[commands.backend:dev]
description = "Start backend API server"
cmd = "cargo run"
working_dir = "./backend"
env = { RUST_LOG = "debug", PORT = "8080" }

[commands.backend:build]
description = "Build backend"
cmd = "cargo build --release"
working_dir = "./backend"

# Database
[commands.db:start]
description = "Start PostgreSQL database"
cmd = "docker-compose up -d postgres"

[commands.db:migrate]
description = "Run database migrations"
cmd = "diesel migration run"
working_dir = "./backend"
deps = ["db:start"]

[commands.db:seed]
description = "Seed database with test data"
cmd = "cargo run --bin seed"
working_dir = "./backend"
deps = ["db:migrate"]

# Development environment
[commands.dev:all]
description = "Start all services"
parallel = true
cmd = [
    "cmdrun run db:start",
    "cmdrun run backend:dev",
    "cmdrun run frontend:dev"
]

[commands.dev:stop]
description = "Stop all services"
cmd = "docker-compose down"

# Testing
[commands.test:frontend]
description = "Test frontend"
cmd = "npm test"
working_dir = "./frontend"

[commands.test:backend]
description = "Test backend"
cmd = "cargo test"
working_dir = "./backend"

[commands.test:e2e]
description = "Run E2E tests"
cmd = "npm run test:e2e"
deps = ["dev:all"]

[commands.test:all]
description = "Run all tests"
parallel = true
cmd = [
    "cmdrun run test:frontend",
    "cmdrun run test:backend"
]
```

**Usage:**
```bash
# Start full development environment
cmdrun run dev:all

# Run all tests
cmdrun run test:all

# Clean shutdown
cmdrun run dev:stop
```

---

## Backend Development

### Rust Project

```toml
# Development
[commands.dev]
description = "Run in development mode with auto-reload"
cmd = "cargo watch -x run"

[commands.check]
description = "Quick compile check"
cmd = "cargo check"

# Building
[commands.build]
description = "Build release binary"
cmd = "cargo build --release"

[commands.build:optimized]
description = "Build with maximum optimization"
cmd = "cargo build --release --target x86_64-unknown-linux-musl"

# Testing
[commands.test]
description = "Run all tests"
cmd = "cargo test"

[commands.test:unit]
description = "Run unit tests only"
cmd = "cargo test --lib"

[commands.test:integration]
description = "Run integration tests"
cmd = "cargo test --test '*'"

[commands.test:watch]
description = "Run tests on file change"
cmd = "cargo watch -x test"

# Code Quality
[commands.fmt]
description = "Format code"
cmd = "cargo fmt"

[commands.fmt:check]
description = "Check formatting"
cmd = "cargo fmt -- --check"

[commands.clippy]
description = "Run Clippy linter"
cmd = "cargo clippy -- -D warnings"

[commands.clippy:fix]
description = "Auto-fix Clippy warnings"
cmd = "cargo clippy --fix --allow-dirty --allow-staged"

[commands.audit]
description = "Security audit"
cmd = "cargo audit"

[commands.outdated]
description = "Check outdated dependencies"
cmd = "cargo outdated"

# Complete CI checks
[commands.ci]
description = "Run all CI checks"
cmd = [
    "cargo fmt -- --check",
    "cargo clippy -- -D warnings",
    "cargo test",
    "cargo audit"
]

# Documentation
[commands.doc]
description = "Generate and open documentation"
cmd = "cargo doc --no-deps --open"

[commands.doc:all]
description = "Generate docs for all dependencies"
cmd = "cargo doc --open"

# Benchmarking
[commands.bench]
description = "Run benchmarks"
cmd = "cargo bench"

[commands.bench:compare]
description = "Compare benchmark results"
cmd = "cargo bench -- --save-baseline main && cargo bench -- --baseline main"
```

**Usage:**
```bash
# Development cycle
cmdrun watch check --pattern "src/**/*.rs"

# Pre-commit
cmdrun run fmt && cmdrun run clippy

# CI pipeline (run locally)
cmdrun run ci

# Performance testing
cmdrun run bench
```

---

### Python/Django Project

```toml
# Development
[commands.dev]
description = "Run Django development server"
cmd = "python manage.py runserver"
env = { DJANGO_SETTINGS_MODULE = "myproject.settings.dev" }

[commands.shell]
description = "Django shell"
cmd = "python manage.py shell_plus"

# Database
[commands.db:migrate]
description = "Run database migrations"
cmd = "python manage.py migrate"

[commands.db:makemigrations]
description = "Create new migrations"
cmd = "python manage.py makemigrations"

[commands.db:reset]
description = "Reset database"
cmd = "python manage.py flush --no-input"
confirm = true

# Testing
[commands.test]
description = "Run tests with coverage"
cmd = "pytest --cov=myapp --cov-report=html"

[commands.test:watch]
description = "Run tests on file change"
cmd = "ptw"  # pytest-watch

# Code Quality
[commands.lint]
description = "Run linters"
parallel = true
cmd = [
    "flake8 .",
    "pylint myapp",
    "mypy myapp"
]

[commands.format]
description = "Format code with black"
cmd = "black ."

[commands.format:check]
description = "Check formatting"
cmd = "black --check ."

# Dependencies
[commands.deps:install]
description = "Install dependencies"
cmd = "pip install -r requirements.txt"

[commands.deps:update]
description = "Update dependencies"
cmd = "pip-compile requirements.in && pip install -r requirements.txt"

[commands.deps:check]
description = "Check for security issues"
cmd = "safety check"

# Deployment
[commands.collect-static]
description = "Collect static files"
cmd = "python manage.py collectstatic --no-input"

[commands.deploy]
description = "Deploy to production"
cmd = [
    "python manage.py collectstatic --no-input",
    "python manage.py migrate",
    "sudo systemctl restart gunicorn"
]
confirm = true
```

---

## DevOps & Infrastructure

### Docker Workflow

```toml
# Docker basics
[commands.docker:build]
description = "Build Docker image"
cmd = "docker build -t ${IMAGE_NAME:-myapp}:${TAG:-latest} ."

[commands.docker:run]
description = "Run Docker container"
cmd = "docker run -p 8080:8080 ${IMAGE_NAME:-myapp}:${TAG:-latest}"

[commands.docker:push]
description = "Push to registry"
cmd = "docker push ${REGISTRY:-docker.io}/${IMAGE_NAME}:${TAG:-latest}"
confirm = true

# Docker Compose
[commands.compose:up]
description = "Start all services"
cmd = "docker-compose up -d"

[commands.compose:down]
description = "Stop all services"
cmd = "docker-compose down"

[commands.compose:logs]
description = "View logs"
cmd = "docker-compose logs -f ${1:-}"

[commands.compose:restart]
description = "Restart service"
cmd = "docker-compose restart ${1:?Service name required}"

# Cleanup
[commands.docker:clean]
description = "Remove unused Docker resources"
cmd = "docker system prune -af --volumes"
confirm = true

[commands.docker:clean:images]
description = "Remove dangling images"
cmd = "docker rmi $(docker images -f 'dangling=true' -q)"

# Multi-stage build
[commands.build:dev]
description = "Build development image"
cmd = "docker build --target development -t myapp:dev ."

[commands.build:prod]
description = "Build production image"
cmd = "docker build --target production -t myapp:prod ."
deps = ["test"]
```

**Usage:**
```bash
# Development
cmdrun run compose:up
cmdrun run compose:logs api

# Production deployment
export IMAGE_NAME="mycompany/myapp"
export TAG="v1.0.0"
export REGISTRY="registry.example.com"

cmdrun run docker:build
cmdrun run docker:push

# Cleanup
cmdrun run docker:clean
```

---

### Kubernetes Deployment

```toml
# Cluster management
[commands.k8s:context]
description = "Show current context"
cmd = "kubectl config current-context"

[commands.k8s:switch:staging]
description = "Switch to staging cluster"
cmd = "kubectl config use-context staging"

[commands.k8s:switch:production]
description = "Switch to production cluster"
cmd = "kubectl config use-context production"

# Deployment
[commands.k8s:deploy]
description = "Deploy to Kubernetes"
cmd = "kubectl apply -f k8s/"

[commands.k8s:rollout:status]
description = "Check rollout status"
cmd = "kubectl rollout status deployment/${1:?Deployment name required}"

[commands.k8s:rollback]
description = "Rollback deployment"
cmd = "kubectl rollout undo deployment/${1:?Deployment name required}"
confirm = true

# Monitoring
[commands.k8s:pods]
description = "List pods"
cmd = "kubectl get pods"

[commands.k8s:logs]
description = "View pod logs"
cmd = "kubectl logs -f ${1:?Pod name required}"

[commands.k8s:describe]
description = "Describe resource"
cmd = "kubectl describe ${1:?Resource type required} ${2:?Resource name required}"

# Debugging
[commands.k8s:exec]
description = "Execute command in pod"
cmd = "kubectl exec -it ${1:?Pod name required} -- /bin/sh"

[commands.k8s:port-forward]
description = "Port forward to pod"
cmd = "kubectl port-forward ${1:?Pod name required} ${2:-8080}:8080"

# Complete deployment workflow
[commands.deploy:staging]
description = "Deploy to staging"
cmd = [
    "cmdrun run k8s:switch:staging",
    "cmdrun run k8s:deploy",
    "cmdrun run k8s:rollout:status myapp"
]

[commands.deploy:production]
description = "Deploy to production"
cmd = [
    "cmdrun run k8s:switch:production",
    "cmdrun run k8s:deploy",
    "cmdrun run k8s:rollout:status myapp"
]
confirm = true
```

---

### Terraform Infrastructure

```toml
# Initialization
[commands.tf:init]
description = "Initialize Terraform"
cmd = "terraform init"

# Planning
[commands.tf:plan]
description = "Show Terraform plan"
cmd = "terraform plan -out=tfplan"

[commands.tf:plan:destroy]
description = "Plan destroy"
cmd = "terraform plan -destroy"

# Apply
[commands.tf:apply]
description = "Apply Terraform changes"
cmd = "terraform apply tfplan"
deps = ["tf:plan"]
confirm = true

[commands.tf:apply:auto]
description = "Apply without confirmation (CI)"
cmd = "terraform apply -auto-approve"
confirm = true

# Destroy
[commands.tf:destroy]
description = "Destroy infrastructure"
cmd = "terraform destroy"
confirm = true

# Validation
[commands.tf:validate]
description = "Validate Terraform files"
cmd = "terraform validate"

[commands.tf:fmt]
description = "Format Terraform files"
cmd = "terraform fmt -recursive"

[commands.tf:fmt:check]
description = "Check Terraform formatting"
cmd = "terraform fmt -check -recursive"

# State management
[commands.tf:state:list]
description = "List resources in state"
cmd = "terraform state list"

[commands.tf:state:show]
description = "Show resource in state"
cmd = "terraform state show ${1:?Resource address required}"

# Environment-specific
[commands.tf:workspace:list]
description = "List workspaces"
cmd = "terraform workspace list"

[commands.tf:workspace:select]
description = "Select workspace"
cmd = "terraform workspace select ${1:?Workspace name required}"

# Complete workflow
[commands.tf:deploy:staging]
description = "Deploy to staging environment"
cmd = [
    "terraform workspace select staging",
    "terraform plan -out=tfplan",
    "terraform apply tfplan"
]
deps = ["tf:validate", "tf:fmt:check"]

[commands.tf:deploy:production]
description = "Deploy to production environment"
cmd = [
    "terraform workspace select production",
    "terraform plan -out=tfplan",
    "terraform apply tfplan"
]
deps = ["tf:validate", "tf:fmt:check"]
confirm = true
```

---

## Data Science

### Machine Learning Workflow

```toml
# Environment setup
[commands.env:create]
description = "Create conda environment"
cmd = "conda env create -f environment.yml"

[commands.env:update]
description = "Update conda environment"
cmd = "conda env update -f environment.yml --prune"

# Jupyter
[commands.jupyter]
description = "Start Jupyter Lab"
cmd = "jupyter lab --no-browser"
env = { JUPYTER_PORT = "8888" }

[commands.jupyter:convert]
description = "Convert notebook to script"
cmd = "jupyter nbconvert --to script ${1:?Notebook path required}"

# Data processing
[commands.data:download]
description = "Download datasets"
cmd = "python scripts/download_data.py"

[commands.data:preprocess]
description = "Preprocess data"
cmd = "python scripts/preprocess.py"
deps = ["data:download"]

[commands.data:validate]
description = "Validate data quality"
cmd = "python scripts/validate_data.py"

# Training
[commands.train]
description = "Train model"
cmd = "python train.py --config configs/${1:-default.yaml}"
deps = ["data:preprocess"]

[commands.train:gpu]
description = "Train on GPU"
cmd = "CUDA_VISIBLE_DEVICES=0 python train.py --config configs/${1:-default.yaml}"
deps = ["data:preprocess"]

[commands.train:distributed]
description = "Distributed training"
cmd = "torchrun --nproc_per_node=4 train.py --config configs/${1:-default.yaml}"

# Evaluation
[commands.eval]
description = "Evaluate model"
cmd = "python evaluate.py --checkpoint ${1:?Checkpoint path required}"

[commands.eval:test]
description = "Evaluate on test set"
cmd = "python evaluate.py --checkpoint ${1} --split test"

# Experiment tracking
[commands.mlflow:ui]
description = "Start MLflow UI"
cmd = "mlflow ui --host 0.0.0.0 --port 5000"

[commands.tensorboard]
description = "Start TensorBoard"
cmd = "tensorboard --logdir runs/"

# Model serving
[commands.serve]
description = "Serve model with FastAPI"
cmd = "uvicorn api:app --reload --host 0.0.0.0 --port 8000"

[commands.serve:prod]
description = "Serve model in production"
cmd = "gunicorn -w 4 -k uvicorn.workers.UvicornWorker api:app"

# Complete pipeline
[commands.pipeline:full]
description = "Run complete ML pipeline"
cmd = [
    "cmdrun run data:download",
    "cmdrun run data:preprocess",
    "cmdrun run data:validate",
    "cmdrun run train default",
    "cmdrun run eval checkpoints/best.pth"
]
```

**Usage:**
```bash
# Setup
cmdrun run env:create
conda activate myproject

# Development
cmdrun run jupyter

# Training
cmdrun run train experiment1
cmdrun watch train:gpu --pattern "src/**/*.py" --pattern "configs/*.yaml"

# Monitoring
cmdrun run mlflow:ui

# Deployment
cmdrun run serve:prod
```

---

## System Administration

### Server Management

```toml
# SSH connections
[commands.ssh:web1]
description = "Connect to web server 1"
cmd = "ssh ${WEB1_USER:-admin}@${WEB1_HOST:?WEB1_HOST not set}"

[commands.ssh:web2]
description = "Connect to web server 2"
cmd = "ssh ${WEB2_USER:-admin}@${WEB2_HOST:?WEB2_HOST not set}"

[commands.ssh:db]
description = "Connect to database server"
cmd = "ssh ${DB_USER:-admin}@${DB_HOST:?DB_HOST not set}"

# System monitoring
[commands.status:disk]
description = "Check disk usage on all servers"
parallel = true
cmd = [
    "ssh $WEB1_HOST df -h",
    "ssh $WEB2_HOST df -h",
    "ssh $DB_HOST df -h"
]

[commands.status:memory]
description = "Check memory usage"
parallel = true
cmd = [
    "ssh $WEB1_HOST free -h",
    "ssh $WEB2_HOST free -h",
    "ssh $DB_HOST free -h"
]

[commands.status:services]
description = "Check service status"
cmd = "ssh ${1:?Host required} systemctl status ${2:?Service required}"

# Backups
[commands.backup:db]
description = "Backup database"
cmd = "ssh $DB_HOST 'pg_dump mydb | gzip' > backup_$(date +%Y%m%d_%H%M%S).sql.gz"

[commands.backup:files]
description = "Backup application files"
cmd = "rsync -avz --delete ${1:?Source required} ${BACKUP_DIR:?BACKUP_DIR not set}/$(date +%Y%m%d)/"

# Log management
[commands.logs:web]
description = "View web server logs"
cmd = "ssh $WEB1_HOST tail -f /var/log/nginx/access.log"

[commands.logs:app]
description = "View application logs"
cmd = "ssh ${1:?Host required} journalctl -u myapp -f"

[commands.logs:analyze]
description = "Analyze logs for errors"
cmd = "ssh ${1} 'grep -i error /var/log/myapp/app.log | tail -50'"

# Deployment
[commands.deploy:app]
description = "Deploy application to servers"
cmd = [
    "rsync -avz --delete dist/ $WEB1_HOST:/var/www/app/",
    "rsync -avz --delete dist/ $WEB2_HOST:/var/www/app/",
    "ssh $WEB1_HOST systemctl restart myapp",
    "ssh $WEB2_HOST systemctl restart myapp"
]
confirm = true

# Maintenance
[commands.maint:update]
description = "Update system packages"
cmd = "ssh ${1:?Host required} 'sudo apt update && sudo apt upgrade -y'"
confirm = true

[commands.maint:restart]
description = "Restart service on all servers"
parallel = true
cmd = [
    "ssh $WEB1_HOST sudo systemctl restart ${1:?Service required}",
    "ssh $WEB2_HOST sudo systemctl restart ${1:?Service required}"
]
confirm = true
```

**Usage:**
```bash
# Environment setup
export WEB1_HOST="web1.example.com"
export WEB2_HOST="web2.example.com"
export DB_HOST="db.example.com"
export BACKUP_DIR="/backups"

# Daily tasks
cmdrun run status:disk
cmdrun run backup:db
cmdrun run logs:app web1.example.com

# Deployment
cmdrun run deploy:app

# Maintenance
cmdrun run maint:restart nginx
```

---

## Personal Productivity

### Daily Commands

```toml
# Git shortcuts
[commands.git:sync]
description = "Pull latest changes"
cmd = "git pull --rebase"

[commands.git:push]
description = "Commit and push all changes"
cmd = "git add . && git commit && git push"

[commands.git:amend]
description = "Amend last commit"
cmd = "git add . && git commit --amend --no-edit && git push --force-with-lease"

[commands.git:cleanup]
description = "Cleanup merged branches"
cmd = "git branch --merged | grep -v '\\*\\|main\\|master' | xargs -r git branch -d"

# Quick notes
[commands.note]
description = "Create quick note"
cmd = "echo '# ${1:?Note title required}' > notes/$(date +%Y%m%d)_${1// /_}.md && code notes/"

# Utilities
[commands.weather]
description = "Check weather"
cmd = "curl wttr.in/${1:-Tokyo}?lang=${2:-en}"

[commands.ip]
description = "Get external IP"
cmd = "curl -s https://ipinfo.io/ip"

[commands.speed-test]
description = "Test internet speed"
cmd = "speedtest-cli"

# Work session
[commands.work:start]
description = "Start work session"
cmd = [
    "echo 'Work started at $(date)' >> work.log",
    "cmdrun run git:sync"
]

[commands.work:end]
description = "End work session"
cmd = [
    "cmdrun run git:push",
    "echo 'Work ended at $(date)' >> work.log"
]

# Pomodoro timer
[commands.pomodoro]
description = "25-minute work timer"
cmd = "sleep 1500 && osascript -e 'display notification \"Break time!\" with title \"Pomodoro\"'"

# Cleanup
[commands.clean:downloads]
description = "Clean old downloads"
cmd = "find ~/Downloads -type f -mtime +30 -delete"
confirm = true

[commands.clean:cache]
description = "Clean various caches"
parallel = true
cmd = [
    "cargo clean",
    "npm cache clean --force",
    "pip cache purge"
]
```

---

## Best Practices

### Organizing Commands

**1. Use Namespaces (Prefixes):**
```toml
# Good
[commands.docker:build]
[commands.docker:run]
[commands.k8s:deploy]
[commands.k8s:rollback]

# Avoid flat structure for large configs
[commands.build]
[commands.run]
[commands.deploy]
[commands.rollback]
```

**2. Group Related Commands:**
```toml
# Development commands
[commands.dev]
[commands.dev:watch]
[commands.dev:debug]

# Testing commands
[commands.test]
[commands.test:unit]
[commands.test:integration]
[commands.test:e2e]

# Deployment commands
[commands.deploy:staging]
[commands.deploy:production]
```

**3. Use Dependencies Wisely:**
```toml
[commands.deploy]
deps = ["test", "build"]  # Always test before deploy

[commands.test]
deps = ["lint"]  # Lint before testing

# Avoid circular dependencies!
```

---

### Security Best Practices

**1. Use Environment Variables for Secrets:**
```toml
# ✅ Good
[commands.deploy]
cmd = "scp dist/ ${DEPLOY_USER}@${DEPLOY_HOST}:/var/www"

# ❌ Bad (secrets in config)
cmd = "scp dist/ admin@server.com:/var/www"
```

**2. Use Confirmation for Destructive Operations:**
```toml
[commands.delete-prod-db]
cmd = "dropdb production"
confirm = true  # Always prompt
```

**3. Use Required Variables:**
```toml
[commands.deploy]
cmd = "deploy to ${ENV:?Environment not set}"
# Error if ENV is not set
```

---

### Performance Tips

**1. Use Parallel Execution:**
```toml
[commands.check-all]
parallel = true  # Run simultaneously
cmd = [
    "cargo fmt -- --check",
    "cargo clippy",
    "cargo test"
]
```

**2. Split Large Configs:**
```bash
# Instead of one huge file:
# ~/.config/cmdrun/commands.toml (5000 lines)

# Use multiple configs:
# ~/.config/cmdrun/work.toml
# ~/.config/cmdrun/personal.toml
# ~/projects/myapp/commands.toml
```

**3. Use Watch Mode:**
```bash
# Instead of manual re-run
# cmdrun run build
# cmdrun run build
# cmdrun run build

# Use watch mode
cmdrun watch build --pattern "src/**/*.rs"
```

---

### Documentation

**Always add descriptions:**
```toml
[commands.deploy]
description = "Deploy to production (requires confirmation)"  # ✅ Clear
cmd = "..."

[commands.d]  # ❌ Unclear name
cmd = "..."  # ❌ No description
```

---

## Related Documentation

- [Configuration Reference](CONFIGURATION.md)
- [CLI Reference](CLI.md)
- [Watch Mode Guide](WATCH_MODE.md)
- [FAQ](FAQ.md)

---

**Last Updated:** 2025-11-07
**Version:** 1.0.0
