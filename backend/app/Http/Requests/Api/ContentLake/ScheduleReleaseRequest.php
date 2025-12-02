<?php

declare(strict_types=1);

namespace App\Http\Requests\Api\ContentLake;

use Illuminate\Foundation\Http\FormRequest;
use Illuminate\Contracts\Validation\Validator;
use Illuminate\Http\Exceptions\HttpResponseException;

/**
 * Schedule Release Request Validation
 *
 * Validates release scheduling requests.
 *
 * @level ICT11 Principal Engineer
 */
class ScheduleReleaseRequest extends FormRequest
{
    public function authorize(): bool
    {
        return auth()->check();
    }

    public function rules(): array
    {
        return [
            'scheduled_at' => [
                'required',
                'date',
                'after:+5 minutes',
                'before:+1 year',
            ],
            'notify_on_publish' => ['nullable', 'boolean'],
            'notification_emails' => ['nullable', 'array', 'max:10'],
            'notification_emails.*' => ['email', 'max:255'],
        ];
    }

    public function messages(): array
    {
        return [
            'scheduled_at.required' => 'Scheduled time is required.',
            'scheduled_at.date' => 'Please provide a valid date and time.',
            'scheduled_at.after' => 'Scheduled time must be at least 5 minutes in the future.',
            'scheduled_at.before' => 'Scheduled time cannot be more than 1 year in the future.',
            'notification_emails.max' => 'Maximum of 10 notification emails allowed.',
            'notification_emails.*.email' => 'Please provide valid email addresses.',
        ];
    }

    protected function failedValidation(Validator $validator): void
    {
        throw new HttpResponseException(
            response()->json([
                'success' => false,
                'message' => 'Invalid schedule request',
                'error_code' => 'VALIDATION_ERROR',
                'errors' => $validator->errors(),
            ], 422)
        );
    }
}
