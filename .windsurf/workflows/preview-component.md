---
description: Preview current component in IDE workbench
---

# Component Preview Workflow

Opens the currently active component file in the Component Workbench with IDE browser preview.

## Usage

1. Open any `.svelte` component file in your editor
2. Run this workflow with `/preview-component` or trigger it from the command palette
3. The component will open in the workbench inside your IDE's browser preview panel

## Steps

### 1. Verify dev server is running
Check if the dev server is running on port 5173. If not, you'll need to start it manually.

### 2. Run the preview script with the current file
// turbo
Execute the preview script with the currently open file path. The script will:
- Validate the file is a Svelte component
- Extract the relative path from `src/lib/components/`
- Generate the workbench URL with the component pre-selected
- Output the URL for the IDE to open

```bash
cd frontend && node scripts/preview-component.js $CURRENT_FILE
```

### 3. Open in IDE browser preview
The script outputs a URL in the format:
```
WORKBENCH_URL=http://localhost:5173/workbench?component={relativePath}
```

The IDE will automatically open this URL in the embedded browser preview panel, showing:
- Component preview with live props editing
- Viewport controls for responsive testing
- Source code viewer
- Props editor panel

## Example

If you have `frontend/src/lib/components/dashboard/VideoCard.svelte` open:
- Relative path: `dashboard/VideoCard.svelte`
- Workbench URL: `http://localhost:5173/workbench?component=dashboard/VideoCard.svelte`

## Notes

- Only works with files inside `src/lib/components/`
- Requires dev server to be running
- Component must be a valid `.svelte` file
- Works in development mode only
