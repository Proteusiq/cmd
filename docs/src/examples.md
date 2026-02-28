# Examples

## File Operations

```bash
# Find large files
cmd "find files larger than 100MB"

# Find recently modified files
cmd "find files modified in the last hour"

# Count files by extension
cmd "count files by extension in current directory"

# Delete old files (use --dry first!)
cmd --dry "delete all .log files older than 30 days"
```

## Git Operations

```bash
# Show recent commits
cmd "show commits from last week with author names"

# Find commits by message
cmd "find commits mentioning 'bug fix'"

# Show changed files
cmd "list files changed in the last commit"

# Undo last commit
cmd --dry "undo the last commit but keep changes"
```

## Docker Operations

```bash
# Container management
cmd "list all running containers"
cmd "stop all running containers"
cmd "remove all stopped containers"

# Image management
cmd "list all docker images sorted by size"
cmd "remove dangling docker images"

# Logs
cmd "show logs from container named 'web' last 100 lines"
```

## System Monitoring

```bash
# Process information
cmd "show top 10 processes using most CPU"
cmd "show top 10 processes using most memory"

# Disk usage
cmd "show disk usage sorted by size"
cmd "find largest directories in home folder"

# Network
cmd "show listening ports"
cmd "show active network connections"
```

## Text Processing

```bash
# Search in files
cmd "find all TODO comments in rust files"
cmd "count lines of code in python files"

# File manipulation
cmd "replace tabs with spaces in all .py files"
cmd "convert all filenames to lowercase"
```

## Compression

```bash
# Create archives
cmd "compress this folder into a tar.gz"
cmd "create a zip file of all .txt files"

# Extract
cmd "extract this tar.gz file"
```

## Safety Tips

1. **Always use `--dry` first** for destructive operations
2. **Review the command** before executing
3. **Start specific** - the more details you provide, the better the command
