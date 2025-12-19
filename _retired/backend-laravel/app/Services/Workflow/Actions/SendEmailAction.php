<?php

namespace App\Services\Workflow\Actions;

use Illuminate\Support\Facades\Mail;

class SendEmailAction implements ActionInterface
{
    public function execute(array $config, array $context): array
    {
        $to = $config['to'] ?? $context['trigger']['email'] ?? null;
        $subject = $config['subject'] ?? 'Notification';
        $template = $config['template'] ?? null;
        $data = $config['data'] ?? [];

        if (!$to) {
            throw new \Exception('Recipient email is required');
        }

        // Send email
        Mail::send($template, $data, function ($message) use ($to, $subject) {
            $message->to($to)->subject($subject);
        });

        return array_merge($context, [
            '_action_result' => [
                'success' => true,
                'email_sent_to' => $to,
                'subject' => $subject,
            ],
        ]);
    }

    public function getMetadata(): array
    {
        return [
            'name' => 'Send Email',
            'description' => 'Send an email to a contact',
            'required_fields' => ['to', 'subject', 'template'],
            'optional_fields' => ['data'],
        ];
    }
}
