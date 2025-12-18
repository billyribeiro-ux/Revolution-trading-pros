<?php

declare(strict_types=1);

namespace App\Services\Post;

use App\Models\Post;
use Illuminate\Contracts\Cache\Repository as CacheRepository;
use Illuminate\Support\Collection;
use Illuminate\Support\Facades\Cache;
use Illuminate\Support\Str;

class FeedService
{
    private CacheRepository $cache;
    private bool $cacheEnabled;

    public function __construct(?CacheRepository $cache = null)
    {
        $this->cache = $cache ?? Cache::store();
        $this->cacheEnabled = config('posts.feeds.cache.enabled', true);
    }

    public function generateRss(?Collection $posts = null): string
    {
        $feed = $this->buildFeedData($posts, 'rss');
        
        return $this->renderRssFeed($feed);
    }

    public function generateAtom(?Collection $posts = null): string
    {
        $feed = $this->buildFeedData($posts, 'atom');
        
        return $this->renderAtomFeed($feed);
    }

    public function buildFeedData(?Collection $posts = null, string $format = 'rss'): array
    {
        if ($this->cacheEnabled) {
            $cacheKey = $this->getFeedCacheKey($format);
            $cached = $this->cache->get($cacheKey);
            
            if ($cached !== null) {
                return $cached;
            }
        }

        if ($posts === null) {
            $posts = $this->getDefaultPosts();
        }

        $config = config("posts.feeds.{$format}", config('posts.feeds.rss'));
        
        $feed = [
            'format' => $format,
            'title' => $config['title'] ?? config('app.name') . ' Blog',
            'link' => url('/blog'),
            'description' => $config['description'] ?? $config['subtitle'] ?? 'Latest blog posts',
            'language' => $config['language'] ?? 'en-us',
            'copyright' => $config['copyright'] ?? $config['rights'] ?? null,
            'lastBuildDate' => now()->toRfc2822String(),
            'generator' => sprintf('%s Feed Generator', config('app.name')),
            'ttl' => $config['ttl'] ?? 60,
            'items' => $this->transformPosts($posts),
        ];

        if (isset($config['image']) && !empty($config['image']['url'])) {
            $feed['image'] = $config['image'];
        }

        if ($this->cacheEnabled) {
            $ttl = config('posts.feeds.cache.ttl', 900);
            $this->cache->put($this->getFeedCacheKey($format), $feed, $ttl);
        }

        return $feed;
    }

    public function invalidateCache(): void
    {
        $this->cache->forget($this->getFeedCacheKey('rss'));
        $this->cache->forget($this->getFeedCacheKey('atom'));
    }

    private function transformPosts(Collection $posts): array
    {
        $includeFullContent = config('posts.feeds.include_full_content', false);
        $excerptLength = config('posts.feeds.excerpt_length', 300);

        return $posts->map(function (Post $post) use ($includeFullContent, $excerptLength) {
            $description = $this->getPostDescription($post, $includeFullContent, $excerptLength);

            return [
                'title' => $post->title,
                'link' => $this->getPostUrl($post),
                'guid' => $this->getPostGuid($post),
                'description' => $description,
                'content' => $includeFullContent ? $this->getPostContent($post) : null,
                'pubDate' => $post->published_at?->toRfc2822String(),
                'updated' => $post->updated_at?->toRfc2822String(),
                'author' => $this->getPostAuthor($post),
                'categories' => $post->categories ?? [],
                'tags' => $post->tags ?? [],
                'enclosure' => $this->getPostEnclosure($post),
            ];
        })->all();
    }

    private function getPostDescription(Post $post, bool $includeFullContent, int $excerptLength): string
    {
        if (!empty($post->excerpt)) {
            return $post->excerpt;
        }

        $content = $this->extractTextFromContentBlocks($post->content_blocks ?? []);

        if (empty($content)) {
            return '';
        }

        if (!$includeFullContent && Str::length($content) > $excerptLength) {
            return Str::limit($content, $excerptLength);
        }

        return $content;
    }

    private function getPostContent(Post $post): string
    {
        if (!empty($post->content_html)) {
            return $post->content_html;
        }

        return $this->extractTextFromContentBlocks($post->content_blocks ?? []);
    }

    private function extractTextFromContentBlocks(array $blocks): string
    {
        $text = collect($blocks)
            ->filter(fn($block) => isset($block['type'], $block['content']))
            ->filter(fn($block) => in_array($block['type'], ['text', 'paragraph', 'heading']))
            ->pluck('content')
            ->map(fn($content) => strip_tags($content))
            ->filter()
            ->implode("\n\n");

        return trim($text);
    }

    private function getPostUrl(Post $post): string
    {
        if (!empty($post->canonical_url)) {
            return $post->canonical_url;
        }

        return url("/blog/{$post->slug}");
    }

    private function getPostGuid(Post $post): string
    {
        return $this->getPostUrl($post);
    }

    private function getPostAuthor(Post $post): array
    {
        if (!$post->author) {
            return [
                'name' => 'Unknown',
                'email' => null,
                'uri' => null,
            ];
        }

        return [
            'name' => $post->author->name,
            'email' => $post->author->email ?? null,
            'uri' => $post->author->website ?? null,
        ];
    }

    private function getPostEnclosure(Post $post): ?array
    {
        if (!$post->featured_image_url) {
            return null;
        }

        return [
            'url' => $post->featured_image_url,
            'type' => 'image/jpeg',
            'length' => 0,
        ];
    }

    private function renderRssFeed(array $feed): string
    {
        $xml = new \SimpleXMLElement('<?xml version="1.0" encoding="UTF-8"?><rss version="2.0" xmlns:content="http://purl.org/rss/1.0/modules/content/" xmlns:atom="http://www.w3.org/2005/Atom"></rss>');
        
        $channel = $xml->addChild('channel');
        
        $atomLink = $channel->addChild('atom:link', null, 'http://www.w3.org/2005/Atom');
        $atomLink->addAttribute('href', url('/feed/rss'));
        $atomLink->addAttribute('rel', 'self');
        $atomLink->addAttribute('type', 'application/rss+xml');
        
        $channel->addChild('title', htmlspecialchars($feed['title'], ENT_XML1));
        $channel->addChild('link', htmlspecialchars($feed['link'], ENT_XML1));
        $channel->addChild('description', htmlspecialchars($feed['description'], ENT_XML1));
        $channel->addChild('language', $feed['language']);
        $channel->addChild('lastBuildDate', $feed['lastBuildDate']);
        $channel->addChild('generator', htmlspecialchars($feed['generator'], ENT_XML1));
        
        if (!empty($feed['copyright'])) {
            $channel->addChild('copyright', htmlspecialchars($feed['copyright'], ENT_XML1));
        }
        
        if (isset($feed['ttl'])) {
            $channel->addChild('ttl', (string) $feed['ttl']);
        }
        
        if (isset($feed['image'])) {
            $image = $channel->addChild('image');
            $image->addChild('url', htmlspecialchars($feed['image']['url'], ENT_XML1));
            $image->addChild('title', htmlspecialchars($feed['image']['title'] ?? $feed['title'], ENT_XML1));
            $image->addChild('link', htmlspecialchars($feed['image']['link'] ?? $feed['link'], ENT_XML1));
        }
        
        foreach ($feed['items'] as $itemData) {
            $item = $channel->addChild('item');
            $item->addChild('title', htmlspecialchars($itemData['title'], ENT_XML1));
            $item->addChild('link', htmlspecialchars($itemData['link'], ENT_XML1));
            $item->addChild('guid', htmlspecialchars($itemData['guid'], ENT_XML1));
            $item->addChild('description', htmlspecialchars($itemData['description'], ENT_XML1));
            
            if (!empty($itemData['content'])) {
                $item->addChild('content:encoded', htmlspecialchars($itemData['content'], ENT_XML1), 'http://purl.org/rss/1.0/modules/content/');
            }
            
            if (!empty($itemData['pubDate'])) {
                $item->addChild('pubDate', $itemData['pubDate']);
            }
            
            if (!empty($itemData['author']['name'])) {
                $item->addChild('author', htmlspecialchars($itemData['author']['name'], ENT_XML1));
            }
            
            foreach ($itemData['categories'] as $category) {
                $item->addChild('category', htmlspecialchars($category, ENT_XML1));
            }
            
            if (!empty($itemData['enclosure'])) {
                $enclosure = $item->addChild('enclosure');
                $enclosure->addAttribute('url', $itemData['enclosure']['url']);
                $enclosure->addAttribute('type', $itemData['enclosure']['type']);
                $enclosure->addAttribute('length', (string) $itemData['enclosure']['length']);
            }
        }
        
        return $xml->asXML();
    }

    private function renderAtomFeed(array $feed): string
    {
        $xml = new \SimpleXMLElement('<?xml version="1.0" encoding="UTF-8"?><feed xmlns="http://www.w3.org/2005/Atom"></feed>');
        
        $xml->addChild('title', htmlspecialchars($feed['title'], ENT_XML1));
        $xml->addChild('subtitle', htmlspecialchars($feed['description'], ENT_XML1));
        $xml->addChild('updated', now()->toAtomString());
        $xml->addChild('id', htmlspecialchars($feed['link'], ENT_XML1));
        
        $linkSelf = $xml->addChild('link');
        $linkSelf->addAttribute('href', url('/feed/atom'));
        $linkSelf->addAttribute('rel', 'self');
        
        $linkAlt = $xml->addChild('link');
        $linkAlt->addAttribute('href', $feed['link']);
        $linkAlt->addAttribute('rel', 'alternate');
        
        if (!empty($feed['copyright'])) {
            $xml->addChild('rights', htmlspecialchars($feed['copyright'], ENT_XML1));
        }
        
        $generator = $xml->addChild('generator', htmlspecialchars($feed['generator'], ENT_XML1));
        $generator->addAttribute('uri', url('/'));
        
        foreach ($feed['items'] as $itemData) {
            $entry = $xml->addChild('entry');
            $entry->addChild('title', htmlspecialchars($itemData['title'], ENT_XML1));
            $entry->addChild('id', htmlspecialchars($itemData['guid'], ENT_XML1));
            $entry->addChild('updated', $itemData['updated'] ?? $itemData['pubDate']);
            
            if (!empty($itemData['pubDate'])) {
                $entry->addChild('published', $itemData['pubDate']);
            }
            
            $link = $entry->addChild('link');
            $link->addAttribute('href', $itemData['link']);
            $link->addAttribute('rel', 'alternate');
            
            $summary = $entry->addChild('summary', htmlspecialchars($itemData['description'], ENT_XML1));
            $summary->addAttribute('type', 'text');
            
            if (!empty($itemData['content'])) {
                $content = $entry->addChild('content', htmlspecialchars($itemData['content'], ENT_XML1));
                $content->addAttribute('type', 'html');
            }
            
            if (!empty($itemData['author']['name'])) {
                $author = $entry->addChild('author');
                $author->addChild('name', htmlspecialchars($itemData['author']['name'], ENT_XML1));
                
                if (!empty($itemData['author']['email'])) {
                    $author->addChild('email', htmlspecialchars($itemData['author']['email'], ENT_XML1));
                }
                
                if (!empty($itemData['author']['uri'])) {
                    $author->addChild('uri', htmlspecialchars($itemData['author']['uri'], ENT_XML1));
                }
            }
            
            foreach ($itemData['categories'] as $category) {
                $cat = $entry->addChild('category');
                $cat->addAttribute('term', $category);
            }
        }
        
        return $xml->asXML();
    }

    private function getDefaultPosts(): Collection
    {
        $limit = config('posts.feeds.items_limit', 50);

        return Post::query()
            ->where('status', 'published')
            ->whereNotNull('published_at')
            ->with(['author', 'media', 'categories', 'tags'])
            ->orderByDesc('published_at')
            ->limit($limit)
            ->get();
    }

    private function getFeedCacheKey(string $format): string
    {
        return sprintf('feed:%s', $format);
    }
}
