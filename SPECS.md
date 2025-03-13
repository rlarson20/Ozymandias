# Ozymandias Specifications

This document provides an overview of all specifications for the Ozymandias knowledge management application.

## Specification Documents

| Topic | Description | Link |
|-------|-------------|------|
| Architecture | High-level system architecture and design decisions | [ARCHITECTURE.md](specs/ARCHITECTURE.md) |
| Logging | Logging, metrics, and telemetry specifications | [LOGGING.md](specs/LOGGING.md) |
| CLI | Command-line interface specifications | [CLI.md](specs/CLI.md) |
| Init Command | Specifications for the initialization command | [INIT_COMMAND.md](specs/INIT_COMMAND.md) |
| Cursor Rules | Guidelines for documenting learned techniques and best practices | [CURSOR_RULES.md](specs/CURSOR_RULES.md) |
| Knowledge Management Issues | Analysis of common issues and their solutions | [KNOWLEDGE_MANAGEMENT_ISSUES.md](specs/KNOWLEDGE_MANAGEMENT_ISSUES.md) |

## Overview

Ozymandias is a knowledge management application written in Rust. It provides tools for organizing, managing, and accessing knowledge in a structured way. The application uses modern Rust practices and includes comprehensive logging and telemetry capabilities.

## Core Features

- Command-line interface using clap
- Comprehensive logging and telemetry using the tracing crate
- Initialization command for setting up new knowledge bases
- Documentation of learned techniques and best practices through Cursor Rules
- Solutions for common knowledge management challenges 