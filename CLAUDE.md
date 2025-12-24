# RuSheet Project

## Multi-Agent Workflow

This project uses a multi-agent architecture for efficient development.

### Main Thread (Orchestrator/Planner)
The main Claude thread acts as an orchestrator ONLY:
- **DO**: Understand requests, create plans, delegate tasks, review results
- **DO NOT**: Read code directly, write code directly
- Delegate all code reading to Explorer agent
- Delegate all code writing to Implementer agent

### Explorer Agent
- **Model**: haiku (fast, cost-effective)
- **Purpose**: Read and summarize code
- **Usage**:
  ```
  Task tool with:
  - subagent_type: "Explore"
  - model: "haiku"
  - prompt: What to find/analyze
  ```

### Implementer Agent
- **Model**: sonnet (balanced for code generation)
- **Purpose**: Write and modify code
- **Usage**:
  ```
  Task tool with:
  - subagent_type: "general-purpose"
  - model: "sonnet"
  - prompt: Detailed implementation instructions
  ```

## Workflow Example

1. User requests a feature
2. Orchestrator plans the approach
3. Orchestrator delegates to Explorer to understand existing code
4. Orchestrator creates detailed implementation plan
5. Orchestrator delegates to Implementer to write code
6. Orchestrator reviews and validates results

## Project Structure

- Rust WASM spreadsheet application
- Crates: rusheet-core, rusheet-formula, rusheet-wasm, rusheet-history
- Frontend: TypeScript + Vite + Canvas rendering
