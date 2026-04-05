You are able to use the Svelte MCP server, where you have access to comprehensive Svelte 5 and SvelteKit documentation. Here's how to use the available tools effectively:

## Available MCP Tools:

### 1. list-sections

Use this FIRST to discover all available documentation sections. Returns a structured list with titles, use_cases, and paths.
When asked about Svelte or SvelteKit topics, ALWAYS use this tool at the start of the chat to find relevant sections.

### 2. get-documentation

Retrieves full documentation content for specific sections. Accepts single or multiple sections.
After calling the list-sections tool, you MUST analyze the returned documentation sections (especially the use_cases field) and then use the get-documentation tool to fetch ALL documentation sections that are relevant for the user's task.

### 3. svelte-autofixer

Analyzes Svelte code and returns issues and suggestions.
You MUST use this tool whenever writing Svelte code before sending it to the user. Keep calling it until no issues or suggestions are returned.

**Local CLI (same engine as MCP):** from `frontend/`, `pnpm run mcp:svelte-autofixer -- path/to/File.svelte` runs one file. `pnpm run mcp:svelte-autofixer:all` scans every `src/**/*.svelte` (slow); use `node scripts/run-svelte-autofixer.mjs --limit 20` for a quick sample.

### 4. playground-link

Generates a Svelte Playground link with the provided code.
After completing the code, ask the user if they want a playground link. Only call this tool after user confirmation and NEVER if code was written to files in their project.

## PE7 end-to-end verification (this repo)

When “fix everything E2E” is requested, run these in order (Svelte MCP cannot replace static checks):

1. `pnpm run check` — `svelte-check` + Svelte 5 / Kit types (same constraints Svelte MCP autofixer looks for).
2. `pnpm run test:unit` — Vitest for `src/**/*.test.ts` (pure logic, components).
3. `pnpm exec playwright test tests/e2e --project=chromium` — browser smoke + route matrix + flows.

Single command: `pnpm run verify` (from `frontend/`). Fix CSP/source issues first, then relax E2E filters only for true infrastructure noise.
