# 📋 Form Builder Guide - Fluent Forms Pro Style

## ✨ Your Form Builder System

You already have a complete **Fluent Forms Pro-like** form builder system! Here's everything you need to know.

---

## 🎯 Available Pages

### 1. **Forms Dashboard**
**URL**: `http://localhost:5174/admin/forms`

Features:
- View all your forms
- See submission counts
- Quick actions (Edit, View Submissions, Analytics)
- Create new forms button

### 2. **Create New Form**
**URL**: `http://localhost:5174/admin/forms/create`

Features:
- Drag-and-drop form builder
- Field library with multiple field types
- Visual form editor
- Form settings configuration
- Live preview
- Save as draft or publish

### 3. **Edit Form**
**URL**: `http://localhost:5174/admin/forms/[id]/edit`

Features:
- Edit existing forms
- Modify fields
- Update settings
- Change form status

### 4. **View Submissions**
**URL**: `http://localhost:5174/admin/forms/[id]/submissions`

Features:
- View all form submissions
- Filter and search submissions
- Export to CSV
- Mark as read/unread/starred
- Delete submissions

### 5. **Form Analytics**
**URL**: `http://localhost:5174/admin/forms/[id]/analytics`

Features:
- Submission statistics
- Conversion rates
- Timeline charts
- Performance metrics

---

## 🛠️ Available Form Field Types

Your form builder supports these field types:

### Basic Fields
- **Text Input** - Single line text
- **Email** - Email validation
- **Number** - Numeric input
- **Phone** - Phone number
- **URL** - Website URL
- **Password** - Secure password input

### Text Fields
- **Textarea** - Multi-line text
- **Rich Text Editor** - WYSIWYG editor

### Choice Fields
- **Select Dropdown** - Single choice dropdown
- **Radio Buttons** - Single choice buttons
- **Checkboxes** - Multiple choice
- **Multi-Select** - Multiple choice dropdown

### Date & Time
- **Date Picker** - Select date
- **Time Picker** - Select time
- **DateTime** - Date and time combined

### File Upload
- **File Upload** - Single file
- **Multiple Files** - Multiple file upload
- **Image Upload** - Image-specific upload

### Advanced Fields
- **Rating** - Star rating
- **Slider** - Range slider
- **Hidden Field** - Hidden data
- **HTML Block** - Custom HTML content
- **Section Break** - Visual separator

---

## 🚀 How to Create a Form

### Step 1: Navigate to Form Builder
```
http://localhost:5174/admin/forms/create
```

### Step 2: Set Form Details
1. Enter **Form Title** (e.g., "Contact Us")
2. Add **Description** (optional)
3. Set **Form Slug** (auto-generated from title)

### Step 3: Add Fields
1. **Drag fields** from the field library on the left
2. **Drop them** into the form canvas
3. **Click a field** to edit its properties:
   - Label
   - Placeholder
   - Help text
   - Required/Optional
   - Validation rules
   - Default value
   - Field width (25%, 50%, 75%, 100%)

### Step 4: Configure Field Properties

Each field can have:
- **Label**: Display name
- **Name**: Unique field identifier
- **Placeholder**: Hint text
- **Help Text**: Additional instructions
- **Required**: Make field mandatory
- **Validation**: Custom validation rules
- **Conditional Logic**: Show/hide based on other fields
- **Default Value**: Pre-filled value
- **Options**: For select/radio/checkbox fields

### Step 5: Configure Form Settings

**General Settings:**
- Success message
- Redirect URL after submission
- Submit button text

**Email Notifications:**
- Enable/disable email notifications
- Notification email addresses
- Email template

**Styling:**
- Primary color
- Button style
- Field spacing
- Border radius
- Custom CSS

### Step 6: Save & Publish
1. Click **Save as Draft** to save without publishing
2. Click **Publish** to make form live
3. Get the form embed code or URL

---

## 📝 Using Forms on Your Site

### Option 1: Embed Form Component

```svelte
<script lang="ts">
  import { submitForm } from '$lib/api/forms';

  let formData = {};
  let status = 'idle';
  let message = '';

  async function handleSubmit(e: Event) {
    e.preventDefault();
    status = 'submitting';

    try {
      const result = await submitForm('your-form-slug', formData);
      
      if (result.success) {
        status = 'success';
        message = result.message;
        // Optionally redirect
        if (result.redirect_url) {
          window.location.href = result.redirect_url;
        }
      } else {
        status = 'error';
        message = 'Please check your inputs';
      }
    } catch (error) {
      status = 'error';
      message = 'Failed to submit form';
    }
  }
</script>

<form on:submit={handleSubmit}>
  <!-- Your form fields here -->
</form>
```

### Option 2: Use Form Preview API

```svelte
<script lang="ts">
  import { onMount } from 'svelte';
  import { previewForm, submitForm } from '$lib/api/forms';
  import type { Form } from '$lib/api/forms';

  let form: Form | null = null;
  let loading = true;

  onMount(async () => {
    try {
      form = await previewForm('contact-form');
      loading = false;
    } catch (error) {
      console.error('Failed to load form:', error);
      loading = false;
    }
  });
</script>

{#if loading}
  <p>Loading form...</p>
{:else if form}
  <!-- Render form fields dynamically -->
  <form>
    {#each form.fields as field}
      <!-- Render field based on field.field_type -->
    {/each}
  </form>
{/if}
```

---

## 🎨 Form Builder Components

Your system includes these components:

### 1. **FormBuilder.svelte**
Location: `frontend/src/lib/components/forms/FormBuilder.svelte`

Main form builder component with:
- Field management
- Drag-and-drop interface
- Field editor
- Form settings

### 2. **FormList.svelte**
Location: `frontend/src/lib/components/forms/FormList.svelte`

Lists all forms with:
- Pagination
- Search/filter
- Quick actions
- Status indicators

### 3. **FieldEditor.svelte**
Location: `frontend/src/lib/components/forms/FieldEditor.svelte`

Field property editor with:
- Field type selection
- Validation rules
- Conditional logic
- Custom attributes

### 4. **FormPreview.svelte**
Live preview of the form as users will see it

### 5. **SubmissionsList.svelte**
View and manage form submissions

---

## 🔧 API Endpoints

Your backend provides these form endpoints:

### Forms Management
```bash
GET    /api/forms                    # List all forms
POST   /api/forms                    # Create new form
GET    /api/forms/{id}               # Get form details
PUT    /api/forms/{id}               # Update form
DELETE /api/forms/{id}               # Delete form
POST   /api/forms/{id}/publish       # Publish form
POST   /api/forms/{id}/unpublish     # Unpublish form
POST   /api/forms/{id}/duplicate     # Duplicate form
```

### Form Submissions
```bash
POST   /api/forms/{slug}/submit                    # Submit form (public)
GET    /api/forms/{id}/submissions                 # List submissions
GET    /api/forms/{id}/submissions/{submissionId}  # Get submission
PUT    /api/forms/{id}/submissions/{submissionId}  # Update submission
DELETE /api/forms/{id}/submissions/{submissionId}  # Delete submission
GET    /api/forms/{id}/submissions/export          # Export to CSV
```

### Form Preview
```bash
GET    /api/forms/preview/{slug}     # Preview published form (public)
```

---

## 💡 Example: Creating a Contact Form

### 1. Create the Form
1. Go to `http://localhost:5174/admin/forms/create`
2. Title: "Contact Us"
3. Slug: "contact-us" (auto-generated)

### 2. Add Fields
Drag these fields into the form:
- **Name** (Text Input, Required, Width: 50%)
- **Email** (Email, Required, Width: 50%)
- **Phone** (Phone, Optional, Width: 50%)
- **Subject** (Select Dropdown, Required, Width: 50%)
  - Options: "General Inquiry", "Support", "Sales"
- **Message** (Textarea, Required, Width: 100%)

### 3. Configure Settings
- Success Message: "Thank you! We'll get back to you soon."
- Send Email: Yes
- Notification Email: "admin@revolutiontradingpros.com"
- Submit Button Text: "Send Message"

### 4. Style the Form
- Primary Color: #3b82f6
- Border Radius: 8px
- Field Spacing: Medium

### 5. Publish
Click "Publish" button

### 6. Use on Your Site
```svelte
<script lang="ts">
  import { submitForm } from '$lib/api/forms';

  let formData = {
    name: '',
    email: '',
    phone: '',
    subject: '',
    message: ''
  };

  async function handleSubmit(e: Event) {
    e.preventDefault();
    const result = await submitForm('contact-us', formData);
    
    if (result.success) {
      alert(result.message);
      formData = { name: '', email: '', phone: '', subject: '', message: '' };
    }
  }
</script>

<form on:submit={handleSubmit}>
  <input bind:value={formData.name} placeholder="Name" required />
  <input bind:value={formData.email} type="email" placeholder="Email" required />
  <input bind:value={formData.phone} type="tel" placeholder="Phone" />
  <select bind:value={formData.subject} required>
    <option value="">Select Subject</option>
    <option value="General Inquiry">General Inquiry</option>
    <option value="Support">Support</option>
    <option value="Sales">Sales</option>
  </select>
  <textarea bind:value={formData.message} placeholder="Message" required></textarea>
  <button type="submit">Send Message</button>
</form>
```

---

## 📊 Viewing Submissions

### Access Submissions
1. Go to `http://localhost:5174/admin/forms`
2. Find your form
3. Click "View Submissions"

### Submission Features
- **Filter by status**: Unread, Read, Starred, Archived, Spam
- **Search**: Search through submission data
- **Export**: Download as CSV
- **Bulk actions**: Mark multiple as read, delete, etc.
- **Individual actions**: View details, star, archive, delete

---

## 🎯 Advanced Features

### Conditional Logic
Show/hide fields based on other field values:

```typescript
{
  field_type: 'text',
  label: 'Company Name',
  name: 'company',
  conditional_logic: {
    action: 'show',
    logic: 'all',
    rules: [
      {
        field: 'customer_type',
        operator: 'equals',
        value: 'business'
      }
    ]
  }
}
```

### Custom Validation
Add custom validation rules:

```typescript
{
  field_type: 'text',
  label: 'Username',
  name: 'username',
  validation: {
    min_length: 3,
    max_length: 20,
    pattern: '^[a-zA-Z0-9_]+$'
  }
}
```

### Field Attributes
Add custom HTML attributes:

```typescript
{
  field_type: 'text',
  label: 'Website',
  name: 'website',
  attributes: {
    autocomplete: 'url',
    spellcheck: false,
    'data-custom': 'value'
  }
}
```

---

## 🔐 Security Features

Your form builder includes:
- ✅ CSRF protection
- ✅ Input sanitization
- ✅ SQL injection prevention
- ✅ XSS protection
- ✅ Rate limiting
- ✅ Spam protection
- ✅ File upload validation

---

## 🚀 Quick Start Checklist

- [ ] Navigate to `/admin/forms`
- [ ] Click "Create New Form"
- [ ] Add form title and description
- [ ] Drag fields from the library
- [ ] Configure field properties
- [ ] Set up form settings
- [ ] Style your form
- [ ] Save and publish
- [ ] Embed on your site
- [ ] Test submissions
- [ ] View analytics

---

## 📚 Additional Resources

- **API Documentation**: `/BACKEND_TEST_GUIDE.md`
- **Form API**: `frontend/src/lib/api/forms.ts`
- **Components**: `frontend/src/lib/components/forms/`
- **Backend Routes**: `backend/routes/api.php`

---

## 🎉 You're Ready!

Your Fluent Forms Pro-like form builder is fully functional and ready to use!

**Start building forms**: http://localhost:5174/admin/forms/create

Need help? Check the components in `frontend/src/lib/components/forms/` for examples and implementation details.
