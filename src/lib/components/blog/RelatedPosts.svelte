<script lang="ts">
  import type { Post } from '$lib/api/posts';

  export let posts: Post[] = [];
  export let loading = false;
</script>

<section class="related-posts mt-12 border-t pt-8">
  <h2 class="text-2xl font-bold mb-6">Related Articles</h2>
  
  {#if loading}
    <div class="grid gap-6 md:grid-cols-2 lg:grid-cols-3">
      {#each Array(3) as _}
        <div class="animate-pulse">
          <div class="bg-gray-200 h-48 rounded-lg mb-4"></div>
          <div class="bg-gray-200 h-6 rounded mb-2"></div>
          <div class="bg-gray-200 h-4 rounded w-3/4"></div>
        </div>
      {/each}
    </div>
  {:else if posts.length > 0}
    <div class="grid gap-6 md:grid-cols-2 lg:grid-cols-3">
      {#each posts as post}
        <article class="group rounded-lg border p-4 transition hover:shadow-lg">
          <a href="/blog/{post.slug}" class="block">
            {#if post.featured_image_url}
              <img 
                src={post.featured_image_url} 
                alt={post.title}
                class="w-full h-48 object-cover rounded-lg mb-4 transition-transform group-hover:scale-105"
              />
            {/if}
            
            <h3 class="text-lg font-semibold mb-2 group-hover:text-blue-600 transition-colors">
              {post.title}
            </h3>
            
            {#if post.excerpt}
              <p class="text-gray-600 text-sm line-clamp-2">
                {post.excerpt}
              </p>
            {/if}
            
            {#if post.categories && post.categories.length > 0}
              <div class="flex gap-2 mt-3">
                {#each post.categories.slice(0, 2) as category}
                  <span class="text-xs bg-blue-100 text-blue-800 px-2 py-1 rounded">
                    {category}
                  </span>
                {/each}
              </div>
            {/if}
          </a>
        </article>
      {/each}
    </div>
  {:else}
    <p class="text-gray-500">No related posts found.</p>
  {/if}
</section>
