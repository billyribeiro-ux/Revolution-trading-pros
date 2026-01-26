/**
 * Form Components - Revolution Trading Pros
 *
 * FluentForm Pro-style form embedding for SvelteKit.
 *
 * ═══════════════════════════════════════════════════════════════════════════
 * QUICKSTART - Adding a form to any page (FluentForm style):
 * ═══════════════════════════════════════════════════════════════════════════
 *
 * ```svelte
 * <script>
 *   import { Form } from '$lib/components/forms';
 * </script>
 *
 * <!-- By ID (recommended - like FluentForm's [fluentform id="5"]) -->
 * <Form id={5} />
 *
 * <!-- By slug (URL-friendly alternative) -->
 * <Form slug="contact-us" />
 *
 * <!-- With options -->
 * <Form id={5} theme="card" hideTitle cssClasses="my-custom-form" />
 * ```
 *
 * ═══════════════════════════════════════════════════════════════════════════
 * AVAILABLE PROPS (matching FluentForm shortcode attributes):
 * ═══════════════════════════════════════════════════════════════════════════
 *
 * | Prop              | Type                                    | FluentForm Equiv    |
 * |-------------------|-----------------------------------------|---------------------|
 * | id                | number                                  | id="5"              |
 * | slug              | string                                  | (n/a)               |
 * | theme             | 'default'|'minimal'|'bordered'|'card'   | theme="..."         |
 * | cssClasses        | string                                  | css_classes="..."   |
 * | type              | 'classic'|'conversational'              | type="..."          |
 * | hideTitle         | boolean                                 | (custom)            |
 * | hideDescription   | boolean                                 | (custom)            |
 * | submitText        | string                                  | (custom)            |
 * | successMessage    | string                                  | (custom)            |
 * | permissionMessage | string                                  | permission_message  |
 * | onSuccess         | (id: string) => void                    | (JS callback)       |
 * | onError           | (error: string) => void                 | (JS callback)       |
 * | redirectUrl       | string                                  | (custom)            |
 *
 * @see https://fluentforms.com/wordpress-form-shortcode/
 */

export { default as FormBuilder } from './FormBuilder.svelte';
export { default as FormRenderer } from './FormRenderer.svelte';
// FormEmbed, MultiStepFormRenderer retired 2026-01-26 - zero imports found
export { default as FormFieldRenderer } from './FormFieldRenderer.svelte';
export { default as FormAnalytics } from './FormAnalytics.svelte';
export { default as FormList } from './FormList.svelte';
export { default as SubmissionsList } from './SubmissionsList.svelte';
export { default as ThemeCustomizer } from './ThemeCustomizer.svelte';
export { default as EmbedCodeGenerator } from './EmbedCodeGenerator.svelte';
export { default as FormTemplateSelector } from './FormTemplateSelector.svelte';
export { default as FieldEditor } from './FieldEditor.svelte';
// QuizField, RepeaterField retired 2026-01-26 - zero imports found
