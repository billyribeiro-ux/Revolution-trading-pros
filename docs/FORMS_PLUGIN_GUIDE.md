# Forms Plugin - Complete Analysis & Development Guide

## Table of Contents

1. [Architecture Overview](#architecture-overview)
2. [Frontend Components](#frontend-components)
3. [Backend System](#backend-system)
4. [API Reference](#api-reference)
5. [Data Flow](#data-flow)
6. [Step-by-Step Guide: Creating & Adding Forms](#step-by-step-guide)

---

## Architecture Overview

The Revolution Trading Pros forms plugin is an enterprise-grade form management system with full-stack implementation:

```
┌─────────────────────────────────────────────────────────────────┐
│                         FRONTEND (SvelteKit)                     │
├─────────────────────────────────────────────────────────────────┤
│  Routes:                                                         │
│  ├── /admin/forms              → FormList (management dashboard) │
│  ├── /admin/forms/create       → FormBuilder (create new)        │
│  ├── /admin/forms/[id]/edit    → FormBuilder (edit existing)     │
│  ├── /admin/forms/[id]/submissions → SubmissionsList             │
│  ├── /admin/forms/[id]/analytics   → FormAnalytics               │
│  └── /embed/form/[slug]        → Public form display             │
├─────────────────────────────────────────────────────────────────┤
│  Components:                                                     │
│  ├── FormBuilder.svelte        → Visual form builder             │
│  ├── FormRenderer.svelte       → Renders form for submission     │
│  ├── FormFieldRenderer.svelte  → Individual field rendering      │
│  ├── FormList.svelte           → Lists all forms                 │
│  ├── FieldEditor.svelte        → Edit individual fields          │
│  └── + 8 more specialized components                             │
├─────────────────────────────────────────────────────────────────┤
│  API Service: $lib/api/forms.ts                                  │
│  ├── FormsService class (singleton)                              │
│  ├── Svelte stores (forms, currentForm, submissions, etc.)       │
│  ├── Caching, offline support, WebSocket real-time               │
│  └── Exported functions: createForm, submitForm, etc.            │
└─────────────────────────────────────────────────────────────────┘
                              │
                              ▼ HTTP/REST API
┌─────────────────────────────────────────────────────────────────┐
│                         BACKEND (Laravel)                        │
├─────────────────────────────────────────────────────────────────┤
│  Controllers:                                                    │
│  ├── FormController.php        → CRUD, publish, duplicate        │
│  └── FormSubmissionController.php → Submissions management       │
├─────────────────────────────────────────────────────────────────┤
│  Models:                                                         │
│  ├── Form.php                  → Main form entity                │
│  ├── FormField.php             → Field definitions               │
│  ├── FormSubmission.php        → Submission records              │
│  └── FormSubmissionData.php    → Individual field values         │
├─────────────────────────────────────────────────────────────────┤
│  Database Tables:                                                │
│  ├── forms                     → Form metadata                   │
│  ├── form_fields               → Field configurations            │
│  ├── form_submissions          → Submission headers              │
│  └── form_submission_data      → Submission field values         │
└─────────────────────────────────────────────────────────────────┘
```

---

## Frontend Components

### 1. Core Components Location

All form components are in: `frontend/src/lib/components/forms/`

| Component | File | Purpose |
|-----------|------|---------|
| FormBuilder | `FormBuilder.svelte` | Visual form builder UI |
| FormRenderer | `FormRenderer.svelte` | Displays form for end-user submission |
| FormFieldRenderer | `FormFieldRenderer.svelte` | Renders individual form fields |
| FormList | `FormList.svelte` | Admin dashboard listing all forms |
| FieldEditor | `FieldEditor.svelte` | Edit field properties |
| FormAnalytics | `FormAnalytics.svelte` | Analytics dashboard |
| SubmissionsList | `SubmissionsList.svelte` | View form submissions |
| MultiStepFormRenderer | `MultiStepFormRenderer.svelte` | Multi-step wizard forms |
| ThemeCustomizer | `ThemeCustomizer.svelte` | Form styling options |
| EmbedCodeGenerator | `EmbedCodeGenerator.svelte` | Generate embed codes |
| FormTemplateSelector | `FormTemplateSelector.svelte` | Select from templates |
| QuizField | `QuizField.svelte` | Quiz/scoring field type |
| RepeaterField | `RepeaterField.svelte` | Repeatable field groups |

### 2. API Service (`$lib/api/forms.ts`)

The forms API service is a **1500+ line** enterprise-grade TypeScript module providing:

#### Key Exports

```typescript
// Stores (Svelte reactive stores)
export const forms: Writable<Form[]>;
export const currentForm: Writable<Form | null>;
export const submissions: Writable<FormSubmission[]>;
export const isLoading: Writable<boolean>;
export const error: Writable<string | null>;
export const offlineMode: Writable<boolean>;

// Form CRUD
export const getForms: (page?, perPage?, filters?) => Promise<{forms, total, perPage}>;
export const getForm: (id: number) => Promise<Form>;
export const createForm: (formData: Partial<Form>) => Promise<Form>;
export const updateForm: (id: number, formData: Partial<Form>) => Promise<Form>;
export const deleteForm: (id: number) => Promise<void>;
export const duplicateForm: (id: number) => Promise<Form>;

// Form Actions
export const publishForm: (formId: number) => Promise<Form>;
export const unpublishForm: (formId: number) => Promise<Form>;
export const archiveForm: (formId: number) => Promise<Form>;

// Submissions
export const submitForm: (slug: string, data: Record<string, any>) => Promise<SubmitResult>;
export const getSubmissions: (formId, page?, perPage?, filters?) => Promise<SubmissionsResult>;
export const exportSubmissions: (formId, format: 'csv'|'excel'|'pdf') => Promise<Blob>;

// Public form preview
export const previewForm: (slug: string) => Promise<Form>;

// Field types
export const getFieldTypes: () => FieldTypeInfo[];

// Validation
export const validateField: (field: FormField, value: any) => Promise<ValidationResult>;
```

#### TypeScript Interfaces

```typescript
// Core Form interface
interface Form {
  id?: number;
  title: string;
  slug: string;
  description?: string;
  type?: 'standard' | 'survey' | 'quiz' | 'payment' | 'registration';
  settings?: FormSettings;
  styles?: FormStyles;
  status: 'draft' | 'published' | 'scheduled' | 'archived';
  fields?: FormField[];
  submission_count?: number;
  created_at?: string;
  updated_at?: string;
}

// Form Field interface
interface FormField {
  id?: number;
  form_id?: number;
  field_type: FieldType;
  label: string;
  name: string;
  placeholder?: string;
  help_text?: string;
  default_value?: any;
  options?: FieldOption[] | null;
  validation?: FieldValidation | null;
  conditional_logic?: ConditionalLogic | null;
  attributes?: FieldAttributes | null;
  required: boolean;
  order: number;
  width: 1 | 2 | 3 | 4 | 6 | 12;  // Grid system
}

// Supported Field Types
type FieldType =
  | 'text' | 'email' | 'tel' | 'url' | 'number' | 'password'
  | 'textarea' | 'select' | 'multiselect' | 'checkbox' | 'radio'
  | 'date' | 'time' | 'datetime' | 'file' | 'image' | 'signature'
  | 'rating' | 'slider' | 'range' | 'toggle' | 'color' | 'location'
  | 'payment' | 'captcha' | 'hidden' | 'section' | 'html' | 'heading'
  | 'divider' | 'step' | 'page_break' | 'quiz' | 'repeater' | 'consent';

// Field Validation
interface FieldValidation {
  required?: boolean;
  min_length?: number;
  max_length?: number;
  pattern?: string;
  pattern_message?: string;
  min?: number;
  max?: number;
  step?: number;
  min_date?: string;
  max_date?: string;
  file_types?: string[];
  max_file_size?: number;
}

// Conditional Logic
interface ConditionalLogic {
  enabled: boolean;
  action: 'show' | 'hide' | 'enable' | 'disable' | 'require';
  logic: 'all' | 'any';
  rules: ConditionalRule[];
}

interface ConditionalRule {
  field: string;       // Field name to check
  operator: ConditionalOperator;
  value?: any;
}

type ConditionalOperator =
  | 'equals' | 'not_equals' | 'contains' | 'not_contains'
  | 'starts_with' | 'ends_with' | 'greater_than' | 'less_than'
  | 'is_empty' | 'is_not_empty' | 'is_checked' | 'is_not_checked';

// Form Settings
interface FormSettings {
  send_email?: boolean;
  save_submissions?: boolean;
  allow_drafts?: boolean;
  require_login?: boolean;
  limit_submissions?: number;
  close_after_date?: string;
  submit_text?: string;
  success_message?: string;
  error_message?: string;
  redirect_url?: string;
  notification_emails?: string[];
  webhook_url?: string;
  spam_protection?: SpamProtection;
}
```

### 3. Form Templates (`$lib/data/formTemplates.ts`)

Pre-built form templates with themes:

**Themes:**
- Modern Blue
- Elegant Purple
- Minimal Gray
- Vibrant Green
- Dark Mode

**Templates:**
1. Basic Contact Form
2. Customer Satisfaction Survey
3. Event Registration
4. Product Feedback
5. Custom Order Form
6. Job Application

---

## Backend System

### 1. Models

#### Form Model (`app/Models/Form.php`)

**Key Attributes:**
- `name`, `slug`, `description`
- `settings` (JSON)
- `active`, `accept_submissions` (boolean)
- `submit_button_text`, `success_message`, `error_message`
- `redirect_url`
- `notification_settings` (JSON)
- `anti_spam_settings` (JSON)
- `submission_limit`
- `expires_at`

**Relationships:**
```php
public function fields(): HasMany    // Form has many FormFields
public function submissions(): HasMany  // Form has many FormSubmissions
```

**Key Methods:**
```php
// Status checks
$form->isExpired(): bool
$form->canAcceptSubmissions(): bool
$form->hasReachedSubmissionLimit(): bool

// Field management
$form->getActiveFields(): Collection
$form->getInputFields(): Collection
$form->getRequiredFields(): Collection
$form->addField(array $fieldData): FormField

// Validation
$form->getValidationRules(): array
$form->validateSubmissionData(array $data): bool

// Statistics
$form->getSubmissionStats(): array
$form->getFieldResponseRates(): array

// Duplication
$form->duplicate(string $newName): Form
```

#### FormField Model (`app/Models/FormField.php`)

**Field Types Constant:**
```php
const FIELD_TYPES = [
    'text', 'email', 'tel', 'number', 'textarea', 'password', 'url',
    'select', 'radio', 'checkbox', 'multiselect',
    'date', 'time', 'datetime',
    'file', 'image', 'signature', 'rating', 'slider', 'color', 'range',
    'location', 'payment', 'captcha', 'hidden',
    'section', 'html', 'heading', 'divider', 'step', 'page_break',
    'quiz', 'repeater', 'consent'
];
```

### 2. Controllers

#### FormController (`app/Http/Controllers/Api/FormController.php`)

| Method | Route | Description |
|--------|-------|-------------|
| `index()` | GET `/api/forms` | List forms with pagination, filters, sorting |
| `store()` | POST `/api/forms` | Create form with fields |
| `show($id)` | GET `/api/forms/{id}` | Get single form with fields |
| `update($id)` | PUT `/api/forms/{id}` | Update form and fields |
| `destroy($id)` | DELETE `/api/forms/{id}` | Delete form |
| `publish($id)` | POST `/api/forms/{id}/publish` | Publish form |
| `unpublish($id)` | POST `/api/forms/{id}/unpublish` | Unpublish form |
| `duplicate($id)` | POST `/api/forms/{id}/duplicate` | Duplicate form |
| `fieldTypes()` | GET `/api/forms/field-types` | Get available field types |
| `stats()` | GET `/api/forms/stats` | Get form statistics |
| `preview($slug)` | GET `/api/forms/preview/{slug}` | **Public** - Get form by slug |
| `submit($slug)` | POST `/api/forms/{slug}/submit` | **Public** - Submit form |

---

## API Reference

### Public Endpoints (No Auth Required)

```
GET  /api/forms/preview/{slug}    → Get published form for display
POST /api/forms/{slug}/submit     → Submit form data
```

### Protected Endpoints (Auth Required)

```
# Forms CRUD
GET    /api/forms                           → List forms
POST   /api/forms                           → Create form
GET    /api/forms/{id}                      → Get form
PUT    /api/forms/{id}                      → Update form
DELETE /api/forms/{id}                      → Delete form

# Form Actions
POST   /api/forms/{id}/publish              → Publish form
POST   /api/forms/{id}/unpublish            → Unpublish form
POST   /api/forms/{id}/duplicate            → Duplicate form

# Utilities
GET    /api/forms/field-types               → Get field types
GET    /api/forms/stats                     → Get statistics

# Submissions
GET    /api/forms/{formId}/submissions              → List submissions
GET    /api/forms/{formId}/submissions/stats        → Submission stats
GET    /api/forms/{formId}/submissions/{id}         → Get submission
PUT    /api/forms/{formId}/submissions/{id}/status  → Update status
DELETE /api/forms/{formId}/submissions/{id}         → Delete submission
GET    /api/forms/{formId}/submissions/export       → Export submissions
POST   /api/forms/{formId}/submissions/bulk-update-status
POST   /api/forms/{formId}/submissions/bulk-delete
```

---

## Data Flow

### Form Submission Flow

```
┌─────────────┐   1. User fills form    ┌─────────────────┐
│   Browser   │ ─────────────────────▶  │  FormRenderer   │
│   (User)    │                         │    Component    │
└─────────────┘                         └────────┬────────┘
                                                 │
                   2. Client-side validation     │
                   3. Call submitForm()          ▼
                                        ┌─────────────────┐
                                        │  forms.ts API   │
                                        │    Service      │
                                        └────────┬────────┘
                                                 │
                   4. POST /forms/{slug}/submit  │
                                                 ▼
                                        ┌─────────────────┐
                                        │ FormController  │
                                        │   submit()      │
                                        └────────┬────────┘
                                                 │
                   5. Server validation          │
                   6. Anti-spam checks           ▼
                                        ┌─────────────────┐
                                        │ FormSubmission  │
                                        │   ::create()    │
                                        └────────┬────────┘
                                                 │
                   7. Store submission data      │
                   8. Trigger notifications      ▼
                                        ┌─────────────────┐
                                        │   Database      │
                                        │   (MySQL)       │
                                        └────────┬────────┘
                                                 │
                   9. Return success + message   │
                                                 ▼
                                        ┌─────────────────┐
                                        │  Success/Error  │
                                        │    Response     │
                                        └─────────────────┘
```

---

## Step-by-Step Guide

# Creating Forms & Adding to Svelte Pages

## Method 1: Using the Admin UI (No-Code)

### Step 1: Navigate to Forms Admin

Go to `/admin/forms` in your application.

### Step 2: Click "Create New Form"

This opens the FormBuilder interface.

### Step 3: Fill Form Details

- **Title**: Your form name (e.g., "Contact Us")
- **Description**: Optional description
- **Success Message**: Message shown after submission
- **Submit Button Text**: Custom button label

### Step 4: Add Fields

Click field type buttons to add fields:
- Text Input, Email, Number, Textarea
- Select, Radio, Checkbox
- Date, Time, File upload
- Rating, etc.

### Step 5: Configure Each Field

For each field, set:
- **Label**: Display label
- **Name**: Form field name (used in submission data)
- **Placeholder**: Hint text
- **Required**: Toggle required validation
- **Help Text**: Additional instructions

### Step 6: Save & Publish

1. Click "Create Form" to save as draft
2. Go to form settings and publish when ready

---

## Method 2: Programmatic Form Creation (Code)

### Step 1: Import the Forms API

```svelte
<script lang="ts">
  import { createForm, type Form, type FormField } from '$lib/api/forms';
</script>
```

### Step 2: Define Form Data

```typescript
const newForm: Partial<Form> = {
  title: 'Contact Form',
  description: 'Get in touch with us',
  settings: {
    success_message: 'Thank you! We will respond soon.',
    submit_text: 'Send Message',
    send_email: true,
    email_to: 'admin@example.com'
  },
  fields: [
    {
      field_type: 'text',
      label: 'Full Name',
      name: 'full_name',
      placeholder: 'John Doe',
      required: true,
      order: 0,
      width: 12,
      validation: {
        min_length: 2,
        max_length: 100
      }
    },
    {
      field_type: 'email',
      label: 'Email Address',
      name: 'email',
      placeholder: 'john@example.com',
      required: true,
      order: 1,
      width: 12
    },
    {
      field_type: 'tel',
      label: 'Phone Number',
      name: 'phone',
      placeholder: '(555) 123-4567',
      required: false,
      order: 2,
      width: 12
    },
    {
      field_type: 'textarea',
      label: 'Message',
      name: 'message',
      placeholder: 'How can we help you?',
      required: true,
      order: 3,
      width: 12,
      validation: {
        min_length: 10,
        max_length: 1000
      }
    }
  ]
};
```

### Step 3: Create the Form

```typescript
async function handleCreateForm() {
  try {
    const form = await createForm(newForm);
    console.log('Form created:', form);
    // form.id and form.slug are now available
  } catch (error) {
    console.error('Failed to create form:', error);
  }
}
```

---

## Method 3: Adding a Form to Any Svelte Page

### Option A: Using FormRenderer Component (Recommended)

This is the easiest way to display a form on any page.

#### Step 1: Create Your Page File

Create a new file, e.g., `frontend/src/routes/contact/+page.svelte`:

```svelte
<script lang="ts">
  import { onMount } from 'svelte';
  import FormRenderer from '$lib/components/forms/FormRenderer.svelte';
  import { previewForm, type Form } from '$lib/api/forms';

  let form: Form | null = $state(null);
  let loading = $state(true);
  let error = $state('');

  // Replace 'contact-form' with your form's slug
  const FORM_SLUG = 'contact-form';

  onMount(async () => {
    try {
      form = await previewForm(FORM_SLUG);
    } catch (err) {
      error = err instanceof Error ? err.message : 'Form not found';
    } finally {
      loading = false;
    }
  });

  function handleSuccess(submissionId: string) {
    console.log('Form submitted successfully:', submissionId);
    // Optional: Show success toast, redirect, etc.
  }

  function handleError(errorMessage: string) {
    console.error('Form submission error:', errorMessage);
    // Optional: Show error toast
  }
</script>

<div class="page-container">
  <h1>Contact Us</h1>

  {#if loading}
    <div class="loading">Loading form...</div>
  {:else if error}
    <div class="error">{error}</div>
  {:else if form}
    <FormRenderer
      {form}
      onSuccess={handleSuccess}
      onError={handleError}
    />
  {/if}
</div>

<style>
  .page-container {
    max-width: 800px;
    margin: 0 auto;
    padding: 2rem;
  }

  h1 {
    margin-bottom: 2rem;
  }

  .loading, .error {
    padding: 2rem;
    text-align: center;
  }

  .error {
    color: #dc2626;
  }
</style>
```

### Option B: Building a Custom Form Component

For full control, create your own form using the API:

```svelte
<script lang="ts">
  import { submitForm } from '$lib/api/forms';

  let formData = $state({
    name: '',
    email: '',
    message: ''
  });
  let isSubmitting = $state(false);
  let submitSuccess = $state(false);
  let submitError = $state('');

  async function handleSubmit(event: Event) {
    event.preventDefault();
    isSubmitting = true;
    submitError = '';

    try {
      // Replace 'contact-form' with your form's slug
      const result = await submitForm('contact-form', formData);

      if (result.success) {
        submitSuccess = true;
        // Reset form
        formData = { name: '', email: '', message: '' };
      } else if (result.errors) {
        submitError = 'Please correct the errors above.';
      }
    } catch (error) {
      submitError = error instanceof Error ? error.message : 'Submission failed';
    } finally {
      isSubmitting = false;
    }
  }
</script>

<form onsubmit={handleSubmit}>
  {#if submitSuccess}
    <div class="success-message">
      Thank you! Your message has been sent.
    </div>
  {/if}

  {#if submitError}
    <div class="error-message">{submitError}</div>
  {/if}

  <div class="form-group">
    <label for="name">Name *</label>
    <input
      id="name"
      type="text"
      bind:value={formData.name}
      required
    />
  </div>

  <div class="form-group">
    <label for="email">Email *</label>
    <input
      id="email"
      type="email"
      bind:value={formData.email}
      required
    />
  </div>

  <div class="form-group">
    <label for="message">Message *</label>
    <textarea
      id="message"
      bind:value={formData.message}
      rows="5"
      required
    ></textarea>
  </div>

  <button type="submit" disabled={isSubmitting}>
    {isSubmitting ? 'Sending...' : 'Send Message'}
  </button>
</form>

<style>
  .form-group {
    margin-bottom: 1rem;
  }

  label {
    display: block;
    margin-bottom: 0.5rem;
    font-weight: 500;
  }

  input, textarea {
    width: 100%;
    padding: 0.75rem;
    border: 1px solid #d1d5db;
    border-radius: 0.375rem;
  }

  button {
    padding: 0.75rem 1.5rem;
    background: #2563eb;
    color: white;
    border: none;
    border-radius: 0.375rem;
    cursor: pointer;
  }

  button:disabled {
    opacity: 0.5;
  }

  .success-message {
    padding: 1rem;
    background: #d1fae5;
    color: #065f46;
    border-radius: 0.375rem;
    margin-bottom: 1rem;
  }

  .error-message {
    padding: 1rem;
    background: #fee2e2;
    color: #991b1b;
    border-radius: 0.375rem;
    margin-bottom: 1rem;
  }
</style>
```

### Option C: Embedding via iframe

If you have a form with slug `my-form`, you can embed it anywhere:

```html
<iframe
  src="/embed/form/my-form"
  width="100%"
  height="600"
  frameborder="0"
></iframe>
```

The embed page automatically communicates back via `postMessage` when the form is submitted.

---

## Method 4: Using Form Templates

### Step 1: Import Templates

```typescript
import { templates, themes, getTemplateById } from '$lib/data/formTemplates';
```

### Step 2: Use a Pre-built Template

```typescript
import { createForm } from '$lib/api/forms';
import { getTemplateById } from '$lib/data/formTemplates';

async function createFromTemplate() {
  // Get the contact form template
  const template = getTemplateById('contact-basic');

  if (!template) return;

  const form = await createForm({
    title: template.name,
    description: template.description,
    settings: template.settings,
    fields: template.fields.map((field, index) => ({
      ...field,
      order: index,
      width: field.width || 12
    }))
  });

  console.log('Created form from template:', form);
}
```

### Available Templates

| Template ID | Name | Fields |
|-------------|------|--------|
| `contact-basic` | Basic Contact Form | Name, Email, Phone, Message |
| `survey-satisfaction` | Customer Satisfaction Survey | Rating, Recommendation, Aspects, Improvements |
| `registration-event` | Event Registration | Name, Email, Phone, Company, Ticket Type, Dietary |
| `feedback-product` | Product Feedback | Product, Usage, Rating, Features, Pros, Cons |
| `order-custom` | Custom Order Form | Customer info, Product type, Quantity, Specs, Budget |
| `application-job` | Job Application | Name, Contact, Position, Experience, Resume, Cover Letter |

---

## Advanced Features

### 1. Conditional Logic

Show/hide fields based on other field values:

```typescript
const field: FormField = {
  field_type: 'text',
  label: 'Company Name',
  name: 'company_name',
  required: false,
  order: 5,
  width: 12,
  conditional_logic: {
    enabled: true,
    action: 'show',      // 'show', 'hide', 'enable', 'disable', 'require'
    logic: 'any',        // 'all' or 'any'
    rules: [
      {
        field: 'employment_type',
        operator: 'equals',
        value: 'employed'
      }
    ]
  }
};
```

### 2. Field Validation

```typescript
const emailField: FormField = {
  field_type: 'email',
  label: 'Email',
  name: 'email',
  required: true,
  order: 1,
  width: 12,
  validation: {
    required: true,
    pattern: '^[^\\s@]+@[^\\s@]+\\.[^\\s@]+$',
    pattern_message: 'Please enter a valid email address'
  }
};

const passwordField: FormField = {
  field_type: 'password',
  label: 'Password',
  name: 'password',
  required: true,
  order: 2,
  width: 12,
  validation: {
    min_length: 8,
    max_length: 128,
    pattern: '^(?=.*[A-Za-z])(?=.*\\d)[A-Za-z\\d@$!%*#?&]{8,}$',
    pattern_message: 'Password must contain at least one letter and one number'
  }
};
```

### 3. Multi-Step Forms

Use `MultiStepFormRenderer` for wizard-style forms:

```svelte
<script lang="ts">
  import MultiStepFormRenderer from '$lib/components/forms/MultiStepFormRenderer.svelte';
</script>

<MultiStepFormRenderer
  {form}
  onSuccess={handleSuccess}
  onError={handleError}
/>
```

Add `page_break` or `step` fields to define step boundaries:

```typescript
const fields: FormField[] = [
  // Step 1: Personal Info
  { field_type: 'heading', label: 'Personal Information', name: 'step1_heading', order: 0, width: 12, required: false },
  { field_type: 'text', label: 'First Name', name: 'first_name', order: 1, width: 6, required: true },
  { field_type: 'text', label: 'Last Name', name: 'last_name', order: 2, width: 6, required: true },
  { field_type: 'page_break', label: '', name: 'step1_break', order: 3, width: 12, required: false },

  // Step 2: Contact Info
  { field_type: 'heading', label: 'Contact Information', name: 'step2_heading', order: 4, width: 12, required: false },
  { field_type: 'email', label: 'Email', name: 'email', order: 5, width: 12, required: true },
  { field_type: 'tel', label: 'Phone', name: 'phone', order: 6, width: 12, required: false },
  { field_type: 'page_break', label: '', name: 'step2_break', order: 7, width: 12, required: false },

  // Step 3: Additional Info
  { field_type: 'heading', label: 'Additional Information', name: 'step3_heading', order: 8, width: 12, required: false },
  { field_type: 'textarea', label: 'Comments', name: 'comments', order: 9, width: 12, required: false }
];
```

### 4. Handling File Uploads

```svelte
<script lang="ts">
  async function handleSubmit(event: Event) {
    event.preventDefault();

    const formElement = event.target as HTMLFormElement;
    const data = new FormData(formElement);

    // For file uploads, use FormData
    const response = await fetch(`/api/forms/${formSlug}/submit`, {
      method: 'POST',
      body: data  // Don't set Content-Type, browser will set it with boundary
    });

    const result = await response.json();
    // Handle result
  }
</script>
```

### 5. Real-time Form Updates (WebSocket)

```typescript
import { joinCollaboration, leaveCollaboration, sendCollaborationUpdate } from '$lib/api/forms';

// Join collaboration session
joinCollaboration(formId);

// Send updates to collaborators
sendCollaborationUpdate(formId, {
  type: 'field_edited',
  fieldId: 123,
  changes: { label: 'New Label' }
});

// Leave when done
leaveCollaboration(formId);
```

---

## Complete Example: Landing Page with Contact Form

Here's a complete example of a landing page with an embedded contact form:

```svelte
<!-- frontend/src/routes/contact/+page.svelte -->
<script lang="ts">
  import { onMount } from 'svelte';
  import FormRenderer from '$lib/components/forms/FormRenderer.svelte';
  import { previewForm, type Form } from '$lib/api/forms';

  let form: Form | null = $state(null);
  let loading = $state(true);
  let error = $state('');
  let submitted = $state(false);

  onMount(async () => {
    try {
      // Load the form by its slug
      form = await previewForm('contact-us');
    } catch (err) {
      error = 'Unable to load the contact form. Please try again later.';
      console.error(err);
    } finally {
      loading = false;
    }
  });

  function handleSuccess(submissionId: string) {
    submitted = true;
    console.log('Submission ID:', submissionId);
  }

  function handleError(errorMessage: string) {
    console.error('Form error:', errorMessage);
  }
</script>

<svelte:head>
  <title>Contact Us - Revolution Trading Pros</title>
  <meta name="description" content="Get in touch with our team" />
</svelte:head>

<main class="contact-page">
  <section class="hero">
    <h1>Contact Us</h1>
    <p>Have questions? We'd love to hear from you.</p>
  </section>

  <section class="form-section">
    <div class="form-container">
      {#if loading}
        <div class="loading-state">
          <div class="spinner"></div>
          <p>Loading form...</p>
        </div>
      {:else if error}
        <div class="error-state">
          <p>{error}</p>
          <button onclick={() => location.reload()}>Try Again</button>
        </div>
      {:else if submitted}
        <div class="success-state">
          <h2>Thank You!</h2>
          <p>We've received your message and will get back to you soon.</p>
          <button onclick={() => submitted = false}>Send Another Message</button>
        </div>
      {:else if form}
        <FormRenderer
          {form}
          onSuccess={handleSuccess}
          onError={handleError}
        />
      {/if}
    </div>
  </section>

  <section class="contact-info">
    <div class="info-card">
      <h3>Email</h3>
      <p>support@revolutiontradingpros.com</p>
    </div>
    <div class="info-card">
      <h3>Phone</h3>
      <p>+1 (555) 123-4567</p>
    </div>
    <div class="info-card">
      <h3>Address</h3>
      <p>123 Trading Street<br>Financial District<br>New York, NY 10004</p>
    </div>
  </section>
</main>

<style>
  .contact-page {
    min-height: 100vh;
  }

  .hero {
    text-align: center;
    padding: 4rem 2rem;
    background: linear-gradient(135deg, #1e293b, #0f172a);
    color: white;
  }

  .hero h1 {
    font-size: 3rem;
    margin-bottom: 1rem;
  }

  .hero p {
    font-size: 1.25rem;
    color: #94a3b8;
  }

  .form-section {
    padding: 4rem 2rem;
    background: #f8fafc;
  }

  .form-container {
    max-width: 600px;
    margin: 0 auto;
    background: white;
    padding: 2rem;
    border-radius: 1rem;
    box-shadow: 0 4px 6px -1px rgba(0, 0, 0, 0.1);
  }

  .loading-state, .error-state, .success-state {
    text-align: center;
    padding: 3rem 1rem;
  }

  .spinner {
    width: 40px;
    height: 40px;
    border: 3px solid #e5e7eb;
    border-top-color: #6366f1;
    border-radius: 50%;
    animation: spin 1s linear infinite;
    margin: 0 auto 1rem;
  }

  @keyframes spin {
    to { transform: rotate(360deg); }
  }

  .success-state {
    color: #065f46;
  }

  .success-state h2 {
    color: #059669;
    margin-bottom: 1rem;
  }

  .error-state {
    color: #991b1b;
  }

  button {
    margin-top: 1rem;
    padding: 0.75rem 1.5rem;
    background: #6366f1;
    color: white;
    border: none;
    border-radius: 0.5rem;
    cursor: pointer;
  }

  button:hover {
    background: #4f46e5;
  }

  .contact-info {
    display: grid;
    grid-template-columns: repeat(auto-fit, minmax(250px, 1fr));
    gap: 2rem;
    padding: 4rem 2rem;
    max-width: 1200px;
    margin: 0 auto;
  }

  .info-card {
    text-align: center;
    padding: 2rem;
  }

  .info-card h3 {
    color: #6366f1;
    margin-bottom: 0.5rem;
  }

  .info-card p {
    color: #64748b;
  }
</style>
```

---

## Summary

The Revolution Trading Pros forms plugin provides:

1. **Full CRUD Operations**: Create, read, update, delete forms
2. **20+ Field Types**: Text, email, select, date, file upload, rating, etc.
3. **Validation**: Required, min/max length, patterns, custom rules
4. **Conditional Logic**: Show/hide fields based on other field values
5. **Templates**: Pre-built form templates for common use cases
6. **Themes**: Customizable form styling
7. **Submissions Management**: View, filter, export submissions
8. **Analytics**: Track form performance and conversions
9. **Anti-Spam**: Honeypot, rate limiting, reCAPTCHA
10. **Real-time**: WebSocket support for live collaboration
11. **Offline Support**: Queue submissions when offline

### Quick Reference

| Task | Method |
|------|--------|
| Display a form | Use `FormRenderer` with `previewForm(slug)` |
| Create a form | Use `createForm(formData)` or admin UI |
| Submit form data | Use `submitForm(slug, data)` |
| List forms | Use `getForms()` |
| Get form analytics | Use `getFormAnalytics(formId)` |
| Export submissions | Use `exportSubmissions(formId, format)` |

### Key Files

| File | Purpose |
|------|---------|
| `$lib/api/forms.ts` | All API functions and types |
| `$lib/components/forms/FormRenderer.svelte` | Display forms |
| `$lib/components/forms/FormBuilder.svelte` | Build forms |
| `$lib/data/formTemplates.ts` | Pre-built templates |
