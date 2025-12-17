<?php

namespace App\Rules;

use Closure;
use Illuminate\Contracts\Validation\ValidationRule;

/**
 * Strong Password Validation Rule
 * ICT11+ Principal Engineer: Matches frontend password requirements
 *
 * Requirements:
 * - Minimum 8 characters
 * - At least one uppercase letter
 * - At least one lowercase letter
 * - At least one number
 * - At least one special character (!@#$%^&*)
 *
 * @version 1.0.0
 * @author Revolution Trading Pros
 */
class StrongPassword implements ValidationRule
{
    /**
     * Run the validation rule.
     *
     * @param  \Closure(string, ?string=): \Illuminate\Translation\PotentiallyTranslatedString  $fail
     */
    public function validate(string $attribute, mixed $value, Closure $fail): void
    {
        $password = (string) $value;
        $errors = [];

        // Minimum length
        if (strlen($password) < 8) {
            $errors[] = 'at least 8 characters';
        }

        // Uppercase letter
        if (!preg_match('/[A-Z]/', $password)) {
            $errors[] = 'at least one uppercase letter';
        }

        // Lowercase letter
        if (!preg_match('/[a-z]/', $password)) {
            $errors[] = 'at least one lowercase letter';
        }

        // Number
        if (!preg_match('/[0-9]/', $password)) {
            $errors[] = 'at least one number';
        }

        // Special character
        if (!preg_match('/[!@#$%^&*]/', $password)) {
            $errors[] = 'at least one special character (!@#$%^&*)';
        }

        if (!empty($errors)) {
            $fail('The :attribute must contain ' . implode(', ', $errors) . '.');
        }
    }
}
