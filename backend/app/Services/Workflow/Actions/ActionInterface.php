<?php

namespace App\Services\Workflow\Actions;

interface ActionInterface
{
    /**
     * Execute the action
     *
     * @param array $config Action configuration
     * @param array $context Workflow execution context
     * @return array Updated context
     */
    public function execute(array $config, array $context): array;

    /**
     * Get action metadata
     *
     * @return array Action name, description, required fields
     */
    public function getMetadata(): array;
}
