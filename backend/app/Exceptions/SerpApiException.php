<?php

namespace App\Exceptions;

use Exception;

/**
 * Exception thrown when SERP API operations fail.
 * 
 * @version 1.0.0
 * @author Revolution Trading Pros
 */
class SerpApiException extends Exception
{
    /**
     * The search engine that caused the error.
     */
    protected ?string $searchEngine = null;

    /**
     * The keyword being searched when error occurred.
     */
    protected ?string $keyword = null;

    /**
     * Create a new SERP API exception.
     */
    public function __construct(
        string $message = 'SERP API request failed',
        int $code = 0,
        ?\Throwable $previous = null
    ) {
        parent::__construct($message, $code, $previous);
    }

    /**
     * Set the search engine context.
     */
    public function setSearchEngine(string $engine): self
    {
        $this->searchEngine = $engine;
        return $this;
    }

    /**
     * Set the keyword context.
     */
    public function setKeyword(string $keyword): self
    {
        $this->keyword = $keyword;
        return $this;
    }

    /**
     * Get the search engine that caused the error.
     */
    public function getSearchEngine(): ?string
    {
        return $this->searchEngine;
    }

    /**
     * Get the keyword being searched.
     */
    public function getKeyword(): ?string
    {
        return $this->keyword;
    }

    /**
     * Get detailed context for logging.
     */
    public function getContext(): array
    {
        return [
            'search_engine' => $this->searchEngine,
            'keyword' => $this->keyword,
            'message' => $this->getMessage(),
            'code' => $this->getCode(),
        ];
    }
}
