<?php

namespace App\Models;

use Illuminate\Database\Eloquent\Model;

class Tag extends Model
{
    protected $fillable = [
        'name',
        'slug',
        'color',
        'order',
        'is_visible',
        'post_count',
    ];

    protected $casts = [
        'is_visible' => 'boolean',
        'order' => 'integer',
        'post_count' => 'integer',
    ];

    protected $appends = ['post_count'];

    public function getPostCountAttribute()
    {
        // Count posts that have this tag in their tags JSON array
        return Post::whereJsonContains('tags', $this->slug)->count();
    }

    protected static function boot()
    {
        parent::boot();

        static::creating(function ($tag) {
            if (is_null($tag->order)) {
                $tag->order = static::max('order') + 1 ?? 0;
            }
        });
    }
}
