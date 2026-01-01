<?php

declare(strict_types=1);

namespace App\Contracts;

/**
 * Renderable Interface
 *
 * Contract for models that can be rendered as HTML/text.
 */
interface Renderable
{
    /**
     * Render to HTML.
     */
    public function renderHtml(array $data = []): string;

    /**
     * Render to plain text.
     */
    public function renderText(array $data = []): string;
}
