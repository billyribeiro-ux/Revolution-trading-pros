# How to Create Forms, Popups, and SEO Pages

## 📝 Creating a New Page with Backend Integration

### Step 1: Create Your Page File

Create a new file in `frontend/src/routes/your-page-name/+page.svelte`

```svelte
<script lang="ts">
  import { onMount } from 'svelte';
  import SEOHead from '$lib/components/SEOHead.svelte';
  
  // Your page logic here
</script>

<SEOHead
  title="Your Page Title"
  description="Your page description"
  canonical="/your-page-name"
/>

<div class="page-container">
  <!-- Your content here -->
</div>

<style>
  /* Your styles here */
</style>
```

---

## 📋 Creating a Form

### Option 1: Simple Newsletter Form (No Backend Form Builder)

```svelte
<script lang="ts">
  let email = '';
  let status: 'idle' | 'submitting' | 'success' | 'error' = 'idle';
  let message = '';

  async function handleSubmit(e: Event) {
    e.preventDefault();
    status = 'submitting';
    
    try {
      const response = await fetch('http://localhost:8000/api/newsletter/subscribe', {
        method: 'POST',
        headers: {
          'Content-Type': 'application/json',
          'Accept': 'application/json'
        },
        body: JSON.stringify({ email })
      });

      const data = await response.json();

      if (response.ok) {
        status = 'success';
        message = data.message || 'Success!';
        email = '';
      } else {
        status = 'error';
        message = data.message || 'Error occurred';
      }
    } catch (error) {
      status = 'error';
      message = 'Network error. Please try again.';
    }
  }
</script>

<form on:submit={handleSubmit}>
  <input
    type="email"
    bind:value={email}
    placeholder="Enter your email"
    required
    disabled={status === 'submitting'}
  />
  
  <button type="submit" disabled={status === 'submitting'}>
    {#if status === 'submitting'}
      Submitting...
    {:else}
      Submit
    {/if}
  </button>

  {#if message}
    <div class="message" class:success={status === 'success'} class:error={status === 'error'}>
      {message}
    </div>
  {/if}
</form>
```

### Option 2: Using Form Builder API

```svelte
<script lang="ts">
  import { submitForm } from '$lib/api/forms';

  let formData = {
    name: '',
    email: '',
    message: ''
  };
  let status: 'idle' | 'submitting' | 'success' | 'error' = 'idle';
  let message = '';
  let errors: Record<string, string[]> = {};

  async function handleSubmit(e: Event) {
    e.preventDefault();
    status = 'submitting';
    errors = {};
    
    try {
      // 'contact' is the form slug you created in backend
      const result = await submitForm('contact', formData);

      if (result.success) {
        status = 'success';
        message = result.message || 'Form submitted successfully!';
        formData = { name: '', email: '', message: '' };
      } else {
        status = 'error';
        errors = result.errors || {};
        message = 'Please fix the errors below.';
      }
    } catch (error) {
      status = 'error';
      message = error instanceof Error ? error.message : 'Failed to submit';
    }
  }
</script>

<form on:submit={handleSubmit}>
  <div>
    <label for="name">Name</label>
    <input
      id="name"
      type="text"
      bind:value={formData.name}
      required
      disabled={status === 'submitting'}
    />
    {#if errors.name}
      <span class="error">{errors.name[0]}</span>
    {/if}
  </div>

  <div>
    <label for="email">Email</label>
    <input
      id="email"
      type="email"
      bind:value={formData.email}
      required
      disabled={status === 'submitting'}
    />
    {#if errors.email}
      <span class="error">{errors.email[0]}</span>
    {/if}
  </div>

  <div>
    <label for="message">Message</label>
    <textarea
      id="message"
      bind:value={formData.message}
      required
      disabled={status === 'submitting'}
    ></textarea>
    {#if errors.message}
      <span class="error">{errors.message[0]}</span>
    {/if}
  </div>

  <button type="submit" disabled={status === 'submitting'}>
    {status === 'submitting' ? 'Sending...' : 'Send Message'}
  </button>

  {#if message}
    <div class="message" class:success={status === 'success'} class:error={status === 'error'}>
      {message}
    </div>
  {/if}
</form>
```

---

## 🎯 Creating a Popup

### Basic Popup Component

```svelte
<script lang="ts">
  import { onMount } from 'svelte';
  import { getActivePopups, recordPopupImpression, recordPopupConversion } from '$lib/api/popups';

  let showPopup = false;
  let email = '';
  let status: 'idle' | 'submitting' | 'success' | 'error' = 'idle';
  let message = '';

  onMount(async () => {
    // Load popups for this page
    try {
      const popups = await getActivePopups('/your-page-name');
      if (popups.length > 0) {
        // Show popup after 3 seconds
        setTimeout(() => {
          showPopup = true;
          recordPopupImpression(popups[0].id);
        }, 3000);
      }
    } catch (error) {
      console.error('Error loading popups:', error);
    }
  });

  async function handleSubmit(e: Event) {
    e.preventDefault();
    status = 'submitting';

    try {
      const response = await fetch('http://localhost:8000/api/newsletter/subscribe', {
        method: 'POST',
        headers: {
          'Content-Type': 'application/json',
          'Accept': 'application/json'
        },
        body: JSON.stringify({ email })
      });

      const data = await response.json();

      if (response.ok) {
        status = 'success';
        message = data.message || 'Successfully subscribed!';
        email = '';
        
        // Record conversion
        // await recordPopupConversion('popup-id', { action: 'newsletter_signup' });
        
        // Close popup after 2 seconds
        setTimeout(() => {
          showPopup = false;
        }, 2000);
      } else {
        status = 'error';
        message = data.message || 'Failed to subscribe.';
      }
    } catch (error) {
      status = 'error';
      message = 'Network error. Please try again.';
    }
  }

  function closePopup() {
    showPopup = false;
  }
</script>

<!-- Your page content -->

<!-- Popup -->
{#if showPopup}
  <div 
    class="popup-overlay" 
    on:click={closePopup}
    on:keydown={(e) => e.key === 'Escape' && closePopup()}
    role="button"
    tabindex="0"
  >
    <div 
      class="popup-content" 
      on:click|stopPropagation
      on:keydown|stopPropagation
      role="dialog"
      tabindex="-1"
    >
      <button class="popup-close" on:click={closePopup} aria-label="Close">
        ×
      </button>
      
      <div class="popup-header">
        <h3>🎉 Special Offer!</h3>
        <p>Subscribe to our newsletter</p>
      </div>

      <form on:submit={handleSubmit}>
        <input
          type="email"
          bind:value={email}
          placeholder="Enter your email"
          required
          disabled={status === 'submitting'}
        />
        
        {#if message}
          <div class="message" class:success={status === 'success'} class:error={status === 'error'}>
            {message}
          </div>
        {/if}

        <button type="submit" disabled={status === 'submitting'}>
          {status === 'submitting' ? 'Subscribing...' : 'Subscribe Now'}
        </button>
      </form>
    </div>
  </div>
{/if}

<style>
  .popup-overlay {
    position: fixed;
    inset: 0;
    background: rgba(0, 0, 0, 0.7);
    backdrop-filter: blur(4px);
    display: flex;
    align-items: center;
    justify-content: center;
    z-index: 1000;
    animation: fadeIn 0.3s ease;
  }

  .popup-content {
    background: linear-gradient(135deg, #1e293b, #0f172a);
    border: 1px solid rgba(96, 165, 250, 0.3);
    border-radius: 20px;
    padding: 3rem;
    max-width: 500px;
    width: 90%;
    position: relative;
    animation: slideUp 0.3s ease;
  }

  .popup-close {
    position: absolute;
    top: 1rem;
    right: 1rem;
    background: none;
    border: none;
    color: #94a3b8;
    font-size: 2rem;
    cursor: pointer;
    line-height: 1;
  }

  .popup-close:hover {
    color: #f1f5f9;
  }

  @keyframes fadeIn {
    from { opacity: 0; }
    to { opacity: 1; }
  }

  @keyframes slideUp {
    from {
      transform: translateY(30px);
      opacity: 0;
    }
    to {
      transform: translateY(0);
      opacity: 1;
    }
  }
</style>
```

---

## 🔍 Adding SEO

### Basic SEO Setup

```svelte
<script lang="ts">
  import SEOHead from '$lib/components/SEOHead.svelte';

  // Define structured data
  const pageSchema = {
    '@context': 'https://schema.org',
    '@type': 'WebPage',
    name: 'Your Page Name',
    description: 'Your page description',
    url: 'https://revolutiontradingpros.com/your-page'
  };
</script>

<SEOHead
  title="Your Page Title - Revolution Trading Pros"
  description="A compelling description of your page that will appear in search results"
  canonical="/your-page"
  ogType="website"
  schema={pageSchema}
/>
```

### Advanced SEO with Article Schema

```svelte
<script lang="ts">
  import SEOHead from '$lib/components/SEOHead.svelte';

  const articleSchema = {
    '@context': 'https://schema.org',
    '@type': 'Article',
    headline: 'Your Article Title',
    description: 'Article description',
    image: 'https://revolutiontradingpros.com/images/article.jpg',
    datePublished: '2025-11-21',
    dateModified: '2025-11-21',
    author: {
      '@type': 'Person',
      name: 'Author Name'
    },
    publisher: {
      '@type': 'Organization',
      name: 'Revolution Trading Pros',
      logo: {
        '@type': 'ImageObject',
        url: 'https://revolutiontradingpros.com/logo.png'
      }
    }
  };
</script>

<SEOHead
  title="Article Title - Revolution Trading Pros"
  description="Article description for SEO"
  canonical="/articles/your-article"
  ogType="article"
  ogImage="https://revolutiontradingpros.com/images/article.jpg"
  schema={articleSchema}
/>
```

---

## 🎨 Complete Page Template

Here's a complete template combining all features:

```svelte
<script lang="ts">
  import { onMount } from 'svelte';
  import SEOHead from '$lib/components/SEOHead.svelte';
  import { IconMail, IconUser } from '@tabler/icons-svelte';

  // Form state
  let formData = {
    name: '',
    email: '',
    message: ''
  };
  let formStatus: 'idle' | 'submitting' | 'success' | 'error' = 'idle';
  let formMessage = '';

  // Popup state
  let showPopup = false;
  let popupEmail = '';
  let popupStatus: 'idle' | 'submitting' | 'success' | 'error' = 'idle';
  let popupMessage = '';

  // Show popup after 5 seconds
  onMount(() => {
    setTimeout(() => {
      showPopup = true;
    }, 5000);
  });

  async function handleFormSubmit(e: Event) {
    e.preventDefault();
    formStatus = 'submitting';

    try {
      // Your form submission logic here
      await new Promise(resolve => setTimeout(resolve, 1000)); // Simulate API call
      
      formStatus = 'success';
      formMessage = 'Thank you! We\'ll be in touch soon.';
      formData = { name: '', email: '', message: '' };
    } catch (error) {
      formStatus = 'error';
      formMessage = 'Something went wrong. Please try again.';
    }
  }

  async function handlePopupSubmit(e: Event) {
    e.preventDefault();
    popupStatus = 'submitting';

    try {
      const response = await fetch('http://localhost:8000/api/newsletter/subscribe', {
        method: 'POST',
        headers: {
          'Content-Type': 'application/json',
          'Accept': 'application/json'
        },
        body: JSON.stringify({ email: popupEmail })
      });

      const data = await response.json();

      if (response.ok) {
        popupStatus = 'success';
        popupMessage = data.message || 'Successfully subscribed!';
        setTimeout(() => showPopup = false, 2000);
      } else {
        popupStatus = 'error';
        popupMessage = data.message || 'Failed to subscribe.';
      }
    } catch (error) {
      popupStatus = 'error';
      popupMessage = 'Network error. Please try again.';
    }
  }

  function closePopup() {
    showPopup = false;
  }

  // SEO Schema
  const pageSchema = {
    '@context': 'https://schema.org',
    '@type': 'WebPage',
    name: 'My Custom Page',
    description: 'A custom page with forms and popups',
    url: 'https://revolutiontradingpros.com/my-page'
  };
</script>

<SEOHead
  title="My Custom Page - Revolution Trading Pros"
  description="Create your own custom page with forms, popups, and SEO"
  canonical="/my-page"
  ogType="website"
  schema={pageSchema}
/>

<div class="page">
  <div class="container">
    <header class="header">
      <h1>My Custom Page</h1>
      <p>With forms, popups, and SEO</p>
    </header>

    <section class="section">
      <h2>Contact Us</h2>
      
      <form class="form" on:submit={handleFormSubmit}>
        <div class="form-group">
          <label for="name">
            <IconUser size={18} />
            Name
          </label>
          <input
            id="name"
            type="text"
            bind:value={formData.name}
            placeholder="Your name"
            required
            disabled={formStatus === 'submitting'}
          />
        </div>

        <div class="form-group">
          <label for="email">
            <IconMail size={18} />
            Email
          </label>
          <input
            id="email"
            type="email"
            bind:value={formData.email}
            placeholder="your@email.com"
            required
            disabled={formStatus === 'submitting'}
          />
        </div>

        <div class="form-group">
          <label for="message">Message</label>
          <textarea
            id="message"
            bind:value={formData.message}
            placeholder="Your message..."
            required
            disabled={formStatus === 'submitting'}
            rows="5"
          ></textarea>
        </div>

        {#if formMessage}
          <div class="message" class:success={formStatus === 'success'} class:error={formStatus === 'error'}>
            {formMessage}
          </div>
        {/if}

        <button type="submit" disabled={formStatus === 'submitting'} class="btn-submit">
          {formStatus === 'submitting' ? 'Sending...' : 'Send Message'}
        </button>
      </form>
    </section>
  </div>
</div>

<!-- Popup -->
{#if showPopup}
  <div 
    class="popup-overlay" 
    on:click={closePopup}
    on:keydown={(e) => e.key === 'Escape' && closePopup()}
    role="button"
    tabindex="0"
  >
    <div 
      class="popup-content" 
      on:click|stopPropagation
      on:keydown|stopPropagation
      role="dialog"
      tabindex="-1"
    >
      <button class="popup-close" on:click={closePopup} aria-label="Close">×</button>
      
      <div class="popup-header">
        <h3>🎉 Join Our Newsletter!</h3>
        <p>Get exclusive trading insights delivered to your inbox</p>
      </div>

      <form class="popup-form" on:submit={handlePopupSubmit}>
        <input
          type="email"
          bind:value={popupEmail}
          placeholder="Enter your email"
          required
          disabled={popupStatus === 'submitting'}
        />
        
        {#if popupMessage}
          <div class="popup-message" class:success={popupStatus === 'success'} class:error={popupStatus === 'error'}>
            {popupMessage}
          </div>
        {/if}

        <button type="submit" disabled={popupStatus === 'submitting'}>
          {popupStatus === 'submitting' ? 'Subscribing...' : 'Subscribe Now'}
        </button>
      </form>
    </div>
  </div>
{/if}

<style>
  .page {
    min-height: 100vh;
    background: linear-gradient(135deg, #0f172a 0%, #1e293b 100%);
    padding: 4rem 2rem;
  }

  .container {
    max-width: 800px;
    margin: 0 auto;
  }

  .header {
    text-align: center;
    margin-bottom: 4rem;
  }

  .header h1 {
    font-size: 3rem;
    font-weight: 800;
    background: linear-gradient(135deg, #60a5fa, #a78bfa);
    -webkit-background-clip: text;
    -webkit-text-fill-color: transparent;
    background-clip: text;
    margin-bottom: 1rem;
  }

  .header p {
    font-size: 1.25rem;
    color: #94a3b8;
  }

  .section {
    background: rgba(30, 41, 59, 0.6);
    border: 1px solid rgba(148, 163, 184, 0.2);
    border-radius: 16px;
    padding: 3rem;
    backdrop-filter: blur(10px);
  }

  .section h2 {
    font-size: 2rem;
    font-weight: 700;
    color: #f1f5f9;
    margin-bottom: 2rem;
  }

  .form-group {
    margin-bottom: 1.5rem;
  }

  .form-group label {
    display: flex;
    align-items: center;
    gap: 0.5rem;
    font-weight: 600;
    color: #cbd5e1;
    margin-bottom: 0.5rem;
  }

  input,
  textarea {
    width: 100%;
    padding: 0.875rem 1rem;
    background: rgba(15, 23, 42, 0.8);
    border: 1px solid rgba(148, 163, 184, 0.3);
    border-radius: 8px;
    color: #f1f5f9;
    font-size: 1rem;
    transition: all 0.3s;
  }

  input:focus,
  textarea:focus {
    outline: none;
    border-color: #60a5fa;
    box-shadow: 0 0 0 3px rgba(96, 165, 250, 0.1);
  }

  textarea {
    resize: vertical;
  }

  .message {
    padding: 1rem;
    border-radius: 8px;
    margin-bottom: 1rem;
    font-weight: 500;
  }

  .message.success {
    background: rgba(16, 185, 129, 0.1);
    border: 1px solid rgba(16, 185, 129, 0.3);
    color: #34d399;
  }

  .message.error {
    background: rgba(239, 68, 68, 0.1);
    border: 1px solid rgba(239, 68, 68, 0.3);
    color: #f87171;
  }

  .btn-submit {
    width: 100%;
    padding: 1rem 2rem;
    background: linear-gradient(135deg, #3b82f6, #8b5cf6);
    color: white;
    border: none;
    border-radius: 8px;
    font-weight: 600;
    font-size: 1rem;
    cursor: pointer;
    transition: all 0.3s;
  }

  .btn-submit:hover:not(:disabled) {
    transform: translateY(-2px);
    box-shadow: 0 10px 30px rgba(59, 130, 246, 0.3);
  }

  .btn-submit:disabled {
    opacity: 0.6;
    cursor: not-allowed;
  }

  /* Popup Styles */
  .popup-overlay {
    position: fixed;
    inset: 0;
    background: rgba(0, 0, 0, 0.7);
    backdrop-filter: blur(4px);
    display: flex;
    align-items: center;
    justify-content: center;
    z-index: 1000;
    animation: fadeIn 0.3s ease;
  }

  .popup-content {
    background: linear-gradient(135deg, #1e293b, #0f172a);
    border: 1px solid rgba(96, 165, 250, 0.3);
    border-radius: 20px;
    padding: 3rem;
    max-width: 500px;
    width: 90%;
    position: relative;
    animation: slideUp 0.3s ease;
  }

  .popup-close {
    position: absolute;
    top: 1rem;
    right: 1rem;
    background: none;
    border: none;
    color: #94a3b8;
    font-size: 2rem;
    cursor: pointer;
    line-height: 1;
  }

  .popup-close:hover {
    color: #f1f5f9;
  }

  .popup-header {
    text-align: center;
    margin-bottom: 2rem;
  }

  .popup-header h3 {
    font-size: 2rem;
    font-weight: 800;
    color: #f1f5f9;
    margin-bottom: 0.75rem;
  }

  .popup-header p {
    color: #94a3b8;
  }

  .popup-form {
    display: flex;
    flex-direction: column;
    gap: 1rem;
  }

  .popup-message {
    padding: 0.75rem;
    border-radius: 8px;
    font-size: 0.875rem;
    text-align: center;
  }

  .popup-message.success {
    background: rgba(16, 185, 129, 0.1);
    border: 1px solid rgba(16, 185, 129, 0.3);
    color: #34d399;
  }

  .popup-message.error {
    background: rgba(239, 68, 68, 0.1);
    border: 1px solid rgba(239, 68, 68, 0.3);
    color: #f87171;
  }

  @keyframes fadeIn {
    from { opacity: 0; }
    to { opacity: 1; }
  }

  @keyframes slideUp {
    from {
      transform: translateY(30px);
      opacity: 0;
    }
    to {
      transform: translateY(0);
      opacity: 1;
    }
  }

  @media (max-width: 768px) {
    .page {
      padding: 2rem 1rem;
    }

    .section {
      padding: 2rem 1.5rem;
    }

    .popup-content {
      padding: 2rem 1.5rem;
    }
  }
</style>
```

---

## 🚀 Quick Start Steps

1. **Create your page file**: `frontend/src/routes/my-page/+page.svelte`
2. **Copy the complete template** above into your file
3. **Customize**:
   - Change page title and description
   - Modify form fields
   - Adjust popup timing and content
   - Update SEO metadata
4. **Visit**: `http://localhost:5174/my-page`

---

## 📚 Available API Functions

### Forms
```typescript
import { submitForm } from '$lib/api/forms';
await submitForm('form-slug', { field: 'value' });
```

### Popups
```typescript
import { getActivePopups, recordPopupImpression } from '$lib/api/popups';
const popups = await getActivePopups('/page-path');
await recordPopupImpression(popupId);
```

### Newsletter
```typescript
const response = await fetch('http://localhost:8000/api/newsletter/subscribe', {
  method: 'POST',
  headers: { 'Content-Type': 'application/json' },
  body: JSON.stringify({ email: 'user@example.com' })
});
```

---

## 💡 Tips

1. **Always test locally first** before deploying
2. **Use proper error handling** for all API calls
3. **Add loading states** for better UX
4. **Make forms accessible** with proper labels and ARIA attributes
5. **Test on mobile** - use responsive design
6. **Validate on both client and server** for security

---

Happy coding! 🎉
