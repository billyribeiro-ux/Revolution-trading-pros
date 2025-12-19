<?php

declare(strict_types=1);

namespace App\DataTransferObjects\Email;

/**
 * DTO for email headers.
 *
 * @version 1.0.0
 */
readonly class EmailHeader
{
    public function __construct(
        public string $name,
        public string $value,
    ) {}

    /**
     * Create from array.
     *
     * @param array{Name?: string, name?: string, Value?: string, value?: string} $data
     */
    public static function fromArray(array $data): self
    {
        return new self(
            name: $data['Name'] ?? $data['name'] ?? '',
            value: $data['Value'] ?? $data['value'] ?? '',
        );
    }

    /**
     * Convert to array.
     *
     * @return array{name: string, value: string}
     */
    public function toArray(): array
    {
        return [
            'name' => $this->name,
            'value' => $this->value,
        ];
    }

    /**
     * Check if header name matches (case-insensitive).
     */
    public function nameIs(string $name): bool
    {
        return strtolower($this->name) === strtolower($name);
    }
}
