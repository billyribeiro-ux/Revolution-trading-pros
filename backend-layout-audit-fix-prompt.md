Act as a distinguished principal frontend engineer specializing in responsive application architecture, complex admin interfaces, CSS layout, accessibility, and browser debugging.

Audit the entire admin dashboard and every discoverable admin page. The reported problem is that many pages break at smaller viewport sizes.

Your assignment is to reproduce the failures, gather concrete evidence, identify their underlying causes, and produce a comprehensive, implementation-ready remediation plan.

This phase is investigation and proposal. Do not modify production application code yet. You may create isolated diagnostic scripts, screenshots, test artifacts, and proposed layout demonstrations.

Do not stop at a generic responsive-design checklist or recommend scattered CSS patches. Determine whether failures originate in the shared application shell, reusable components, page composition, content, or a combination of these.

OBJECTIVE

Define a coherent responsive architecture that:

- Supports the dashboard’s actual users and workflows.
- Keeps every page usable across the supported viewport range.
- Preserves access to important information and actions.
- Handles realistic content, zoom, and intermediate widths.
- Resolves shared causes consistently across affected pages.
- Fits the repository’s framework and existing conventions.
- Can be verified through repeatable regression checks.

Treat “fix it once and for all” as an engineering objective: eliminate identified causes and establish safeguards against recurrence. Do not promise that future changes can never introduce regressions.

1. DISCOVER THE DASHBOARD AND ITS LAYOUT SYSTEM

Read applicable repository instructions and identify:

- The actual framework, versions, styling system, and component libraries.
- Admin routes, nested layouts, navigation groups, and role-dependent pages.
- The shared shell: sidebar, header, breadcrumbs, main content, toolbars, and footer.
- Layout primitives, containers, grids, spacing tokens, and breakpoint definitions.
- Shared tables, forms, charts, cards, tabs, dialogs, drawers, and menus.
- Existing responsive rules, accessibility checks, visual tests, and browser tooling.
- Supported browsers, minimum viewport requirements, and documented device targets.

Inspect route definitions as well as visible navigation so pages omitted from menus are not overlooked.

Build a page inventory. Mark inaccessible or role-restricted pages explicitly. Do not claim to have audited pages you could not access.

If the minimum supported width is undocumented, use 320 CSS pixels as an initial reflow diagnostic and propose a supported baseline appropriate to the product. Distinguish diagnostic coverage from an agreed product requirement.

2. REPRODUCE THE FAILURES IN THE RUNNING APPLICATION

Launch the dashboard and interact with it using available browser tools.

Inspect every discoverable admin page at representative widths:

- 320 and 375 CSS pixels.
- 480 and 768 CSS pixels.
- 1024, 1280, and 1440 CSS pixels.
- Any existing breakpoint boundaries relevant to observed failures.

These are sampling points, not proposed breakpoint values. Resize continuously around failing layouts and record the approximate width at which each failure begins.

Include:

- Short laptop windows and mobile landscape heights.
- Sidebar expanded and collapsed states.
- Browser zoom at 200%.
- A reflow check equivalent to a 320 CSS-pixel viewport where applicable.
- Keyboard navigation and focused elements.
- Open dropdowns, dialogs, drawers, date pickers, and other overlays.
- Long titles, emails, identifiers, translated text where supported, and unbroken strings.
- Empty, loading, error, and realistically populated states.
- Dense tables, long forms, charts, pagination, filters, and bulk actions.

Do not multiply checks mechanically when pages share an identical layout. Inspect every page, then use deeper representative testing for shared patterns and targeted testing for unique behavior.

Use safe test data. Do not alter real records or perform destructive actions without authorization.

If runtime access is unavailable, continue source analysis and clearly separate suspected issues from reproduced failures.

3. CAPTURE AN EVIDENCE LEDGER

For every meaningful failure, record:

Issue ID | route | viewport and UI state | reproduction steps | expected behavior | observed behavior | screenshot/evidence | responsible component or rule | severity | affected pages

Capture evidence such as:

- Screenshots showing clipping, overlap, overflow, or inaccessible controls.
- Relevant DOM measurements and computed styles.
- The ancestor chain responsible for a width or height constraint.
- Whether overflow belongs to the viewport or an intentional local container.
- Verified source file and line references.
- The first failing width and behavior immediately above it.

Differentiate:

- Observed symptom.
- Confirmed root cause.
- Suspected cause requiring further investigation.

Do not infer causality from a screenshot alone. Correlate visual evidence with layout behavior and source code.

Group repeated instances under their shared cause while retaining the affected-page list.

4. TRACE ROOT CAUSES THROUGH THE LAYOUT HIERARCHY

Investigate the shell before treating every page as an independent defect.

Specifically inspect:

Application shell:

- Sidebar width and collapse behavior.
- Main-content available width.
- Header and navigation wrapping.
- Nested scroll containers.
- Fixed and sticky positioning.
- Viewport height assumptions.
- Content obscured by persistent controls.

Flex and grid:

- Intrinsic sizing and default minimum sizes.
- Missing shrink behavior.
- Tracks that cannot contract.
- Fixed columns, widths, and minimum widths.
- Unintended nowrap behavior.
- Content-driven overflow.
- Conflicting breakpoint rules.

Content and components:

- Long strings and text wrapping.
- Button groups, toolbars, filters, and tabs.
- Form columns and label/control alignment.
- Table width, column priorities, and action visibility.
- Chart sizing, legends, axes, and resize handling.
- Dialog dimensions and internal scrolling.
- Popover positioning and clipping ancestors.
- Images, embeds, and fixed aspect ratios.

CSS architecture:

- Global rules overriding component behavior.
- Excessive specificity and contradictory media queries.
- Inline dimensions and scattered breakpoint constants.
- Repeated page-level workarounds.
- Broad overflow suppression hiding defects.
- Inconsistent container and spacing rules.

Use temporary, non-persistent DevTools experiments where useful to test a hypothesis. Verify that a candidate correction addresses the cause rather than merely hiding the symptom.

Do not assume a particular CSS property is the answer before inspecting the constraint chain.

5. DEFINE THE RESPONSIVE BEHAVIOR THE PRODUCT NEEDS

Responsive admin design must preserve task completion, not merely fit content inside a screenshot.

For each major pattern, specify how it should behave as available space decreases:

Navigation:

- When the sidebar collapses or becomes a drawer.
- How navigation remains discoverable.
- Focus management, escape behavior, and focus restoration.

Page headers and actions:

- Heading and metadata wrapping.
- Primary action visibility.
- Secondary action grouping.
- Toolbar and filter reorganization.

Forms:

- Column collapse.
- Label and help-text behavior.
- Validation placement.
- Long forms and persistent submit actions.

Tables:

- When local horizontal scrolling is appropriate.
- Which columns are essential.
- Whether column selection, detail expansion, or an alternate presentation is justified.
- How row actions, selection, sorting, and pagination remain accessible.

Charts:

- Container-driven resizing.
- Axis and legend adaptation.
- Minimum useful dimensions.
- Accessible summaries or alternatives where needed.

Overlays:

- Dialog and drawer sizing.
- Internal scrolling.
- Reachable close and submit controls.
- Behavior with virtual keyboards where relevant.

Explain the tradeoffs. Do not automatically convert every table to cards, hide important columns, or shrink typography to make a layout fit.

Horizontal scrolling may be appropriate inside a genuinely two-dimensional component. Accidental page-level horizontal overflow is a separate problem.

6. PROPOSE A SYSTEMIC REMEDIATION ARCHITECTURE

Develop one recommended approach, with alternatives only where a meaningful tradeoff exists.

Specify:

- Shared-shell corrections.
- Reusable layout primitives and component contracts.
- Consistent container, spacing, and sizing rules.
- Content-driven breakpoints.
- Appropriate use of media queries versus container queries.
- Rules for intentional scrolling.
- Responsive component variants.
- Page-specific exceptions that genuinely require them.
- A migration strategy that preserves working layouts.

Prefer established project conventions and native CSS capabilities when they solve the problem clearly.

Do not introduce a new layout framework, global rewrite, or broad abstraction without demonstrating why the current architecture cannot support the correction.

Explicitly avoid:

- Global overflow-x: hidden as a blanket fix.
- Arbitrary fixed widths.
- Tiny text or controls.
- Hiding essential features at smaller widths.
- Device-name breakpoints with no relationship to content.
- Accumulating page-specific media queries for shared defects.
- !important rules used to avoid understanding the cascade.

For every proposed correction, identify:
Root cause → shared fix → affected pages → exceptions → regression checks.

7. MAKE THE PROPOSAL CONCRETE

Provide:

- Annotated current-state screenshots for representative failures.
- Proposed wide and narrow layouts for the most important shared patterns.
- A component-level explanation of the intended behavior.
- Small illustrative code examples or isolated demonstrations where they clarify the approach.

Clearly label demonstrations as proposals. Keep them separate from production code.

Show at least:

- The shared shell at a constrained width.
- A dense table or data-heavy page.
- A form or toolbar with competing controls.
- An overlay if overlays are affected.

Do not spend the entire audit creating polished mockups while leaving root causes unexplained.

8. DEFINE THE IMPLEMENTATION AND REGRESSION PLAN

Prioritize:

P0: A critical task cannot be completed, essential information is inaccessible, or an overlay traps the user.
P1: Major clipping, overlap, overflow, navigation failure, or inaccessible primary actions.
P2: Significant readability, density, spacing, or consistency problems.
P3: Minor visual refinements.

For each work item, include:

Priority | confirmed cause | proposed change | files/components | affected routes | dependencies | relative effort | acceptance criteria

Order the work by dependency:

1. Shared shell and width constraints.
2. Shared layout primitives and components.
3. Page composition and justified exceptions.
4. Visual refinement and regression safeguards.

Define repeatable checks using the repository’s actual tooling. Recommend browser automation and visual regression tests where useful.

Combine:

- Geometry assertions for unintended overflow and visibility.
- Interaction checks for navigation, actions, and overlays.
- Representative screenshots for visual regression.
- Manual review for readability, hierarchy, and task completion.

A screenshot alone does not prove usability. A passing overflow assertion does not prove that controls remain accessible.

9. ACCEPTANCE CRITERIA FOR THE FUTURE FIX

The proposed implementation must be verifiable against criteria such as:

- No unintended page-level horizontal overflow across the supported range.
- Important content and actions remain reachable.
- Navigation works at constrained widths.
- Text and controls do not overlap or become unintentionally clipped.
- Local scrolling is intentional and discoverable.
- Tables preserve access to necessary data and actions.
- Forms remain readable and operable.
- Dialogs and popovers fit available space and support keyboard use.
- Zoom and text enlargement preserve task completion.
- Existing desktop layouts remain functional.
- Shared fixes work across all affected routes.
- Relevant formatting, type checks, tests, and builds pass.

Adapt these to the product. Do not use “looks good on mobile” as an acceptance criterion.

10. REQUIRED HANDOFF

Lead with the central diagnosis: is this primarily a shell problem, shared-component problem, page-composition problem, or a combination?

Then deliver:

1. Audit scope and route/viewport coverage.
2. The most consequential failures with evidence.
3. Confirmed root causes and their affected pages.
4. The recommended responsive architecture.
5. Annotated layout proposals.
6. A prioritized implementation plan.
7. Verifiable acceptance criteria and regression strategy.
8. Unverified areas, blockers, and decisions requiring input.

Recommend a direction confidently. Resolve routine technical choices from evidence rather than handing every decision back to me.

Do not modify production code in this phase. Finish with a concrete proposal that can be approved and implemented—not a plan to begin investigating.

START NOW

Inspect the dashboard’s routes and shared layout, launch the running application, reproduce the smaller-viewport failures, and trace the first failures to their actual causes. Continue until the audit and remediation proposal are complete within the accessible scope.
