# MCP Core

## Overview
The core protocol logic for Machine-Centric Protocol (MCP) on Solana. This repository contains the fundamental Solana programs (smart contracts) that power the MCP ecosystem.

## Components
- **Registry Layer**: Agent, context, and permission management
- **Execution Tracking**: Transaction monitoring and metering
- **IDL Files**: Anchor-compatible interface definition files
- **Client Bindings**: JavaScript/TypeScript SDK for Solana using Anchor

## Development Setup
```bash
# Install dependencies
npm install

# Build the Solana programs
anchor build

# Run tests
npm test
```

## Repository Structure
- `/programs` - Solana smart contracts written in Rust with Anchor
- `/clients` - JavaScript/TypeScript SDK for interacting with MCP
- `/tests` - Unit and integration tests
- `/.github` - CI/CD workflows for testing and deployment

## Links
- [Documentation](https://github.com/MyCryptoProtocol/mcp-docs)
- [Examples](https://github.com/MyCryptoProtocol/mcp-examples)
- [Agents](https://github.com/MyCryptoProtocol/mcp-agents)
- [Server](https://github.com/MyCryptoProtocol/mcp-server)
