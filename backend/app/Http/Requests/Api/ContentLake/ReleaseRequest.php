<?php

declare(strict_types=1);

namespace App\Http\Requests\Api\ContentLake;

use Illuminate\Foundation\Http\FormRequest;
use Illuminate\Contracts\Validation\Validator;
use Illuminate\Http\Exceptions\HttpResponseException;

/**
 * Release Request Validation
 *
 * Validates release bundle creation and scheduling requests.
 *
 * @level ICT11 Principal Engineer
 */
class ReleaseRequest extends FormRequest
{
    public function authorize(): bool
    {
        return auth()->check();
    }

    public function rules(): array
    {
        return [
            'name' => ['required', 'string', 'max:255', 'min:3'],
            'description' => ['nullable', 'string', 'max:2000'],
            'scheduled_at' => [
                'nullable',
                'date',
                'after:now',
                function ($attribute, $value, $fail) {
                    if ($value) {
                        $this->validateScheduleTime($value, $fail);
                    }
                },
            ],
            'metadata' => ['nullable', 'array'],
            'metadata.priority' => ['nullable', 'string', 'in:low,medium,high,critical'],
            'metadata.tags' => ['nullable', 'array', 'max:10'],
            'metadata.tags.*' => ['string', 'max:50'],
            'metadata.notification_channels' => ['nullable', 'array'],
            'metadata.notification_channels.*' => ['string', 'in:email,slack,webhook'],
            'auto_publish' => ['nullable', 'boolean'],
        ];
    }

    public function messages(): array
    {
        return [
            'name.required' => 'Release name is required.',
            'name.min' => 'Release name must be at least 3 characters.',
            'scheduled_at.after' => 'Scheduled time must be in the future.',
            'metadata.priority.in' => 'Priority must be one of: low, medium, high, critical.',
            'metadata.tags.max' => 'Maximum of 10 tags allowed.',
        ];
    }

    /**
     * Validate schedule time constraints
     */
    private function validateScheduleTime(string $scheduledAt, callable $fail): void
    {
        $scheduled = new \DateTime($scheduledAt);
        $now = new \DateTime();

        // Must be at least 5 minutes in the future
        $minTime = (clone $now)->modify('+5 minutes');
        if ($scheduled < $minTime) {
            $fail('Scheduled time must be at least 5 minutes in the future.');
            return;
        }

        // Cannot be more than 1 year in the future
        $maxTime = (clone $now)->modify('+1 year');
        if ($scheduled > $maxTime) {
            $fail('Scheduled time cannot be more than 1 year in the future.');
        }
    }

    protected function failedValidation(Validator $validator): void
    {
        throw new HttpResponseException(
            response()->json([
                'success' => false,
                'message' => 'Invalid release configuration',
                'error_code' => 'VALIDATION_ERROR',
                'errors' => $validator->errors(),
            ], 422)
        );
    }
}
