<?php

declare(strict_types=1);

namespace App\Traits;

use Illuminate\Support\Facades\Crypt;

/**
 * Encryptable Trait
 *
 * Automatically encrypt/decrypt sensitive attributes.
 */
trait Encryptable
{
    /**
     * Get encryptable attributes.
     */
    public function getEncryptableAttributes(): array
    {
        return $this->encryptable ?? [];
    }

    /**
     * Encrypt a value.
     */
    protected function encryptAttribute(string $value): string
    {
        return Crypt::encryptString($value);
    }

    /**
     * Decrypt a value.
     */
    protected function decryptAttribute(string $value): string
    {
        try {
            return Crypt::decryptString($value);
        } catch (\Exception $e) {
            return $value;
        }
    }

    /**
     * Override getAttribute to decrypt.
     */
    public function getAttributeValue($key)
    {
        $value = parent::getAttributeValue($key);

        if (in_array($key, $this->getEncryptableAttributes()) && !empty($value)) {
            return $this->decryptAttribute($value);
        }

        return $value;
    }

    /**
     * Override setAttribute to encrypt.
     */
    public function setAttribute($key, $value)
    {
        if (in_array($key, $this->getEncryptableAttributes()) && !empty($value)) {
            $value = $this->encryptAttribute($value);
        }

        return parent::setAttribute($key, $value);
    }
}
