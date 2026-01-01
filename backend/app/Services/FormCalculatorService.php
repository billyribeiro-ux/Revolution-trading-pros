<?php

declare(strict_types=1);

namespace App\Services;

use App\Models\Form;
use App\Models\FormField;
use Illuminate\Support\Str;

/**
 * Form Calculator Service - Enterprise-grade calculated fields engine
 *
 * Supports:
 * - Mathematical operations (+, -, *, /, %, ^)
 * - Functions (SUM, AVG, MIN, MAX, COUNT, ROUND, ABS, CEIL, FLOOR)
 * - Conditional calculations (IF, AND, OR, NOT)
 * - Field references ({field_name})
 * - Smart tags ({today}, {user.name}, etc.)
 * - Nested expressions
 * - Currency formatting
 * - Date calculations
 *
 * @version 2.0.0
 * @security Sanitizes all expressions to prevent code injection
 */
class FormCalculatorService
{
    /**
     * Available functions and their implementations
     */
    private const FUNCTIONS = [
        'SUM' => 'calculateSum',
        'AVG' => 'calculateAvg',
        'MIN' => 'calculateMin',
        'MAX' => 'calculateMax',
        'COUNT' => 'calculateCount',
        'ROUND' => 'calculateRound',
        'ABS' => 'calculateAbs',
        'CEIL' => 'calculateCeil',
        'FLOOR' => 'calculateFloor',
        'IF' => 'calculateIf',
        'AND' => 'calculateAnd',
        'OR' => 'calculateOr',
        'NOT' => 'calculateNot',
        'CONCAT' => 'calculateConcat',
        'DATEDIFF' => 'calculateDateDiff',
        'DATEADD' => 'calculateDateAdd',
        'NOW' => 'calculateNow',
        'TODAY' => 'calculateToday',
        'FORMAT' => 'calculateFormat',
        'CURRENCY' => 'calculateCurrency',
        'PERCENTAGE' => 'calculatePercentage',
        'LOOKUP' => 'calculateLookup',
    ];

    /**
     * Allowed operators for security
     */
    private const ALLOWED_OPERATORS = ['+', '-', '*', '/', '%', '^', '(', ')', ',', '.', ' '];

    /**
     * Comparison operators
     */
    private const COMPARISON_OPERATORS = ['==', '!=', '>', '<', '>=', '<=', '===', '!=='];

    /**
     * Current form data context
     */
    private array $formData = [];

    /**
     * Current user context
     */
    private ?object $user = null;

    /**
     * Form fields indexed by name
     */
    private array $fieldsMap = [];

    /**
     * Calculate a field value based on its formula
     *
     * @param FormField $field The calculated field
     * @param array $formData Current form data
     * @param object|null $user Current user (optional)
     * @return mixed Calculated result
     */
    public function calculate(FormField $field, array $formData, ?object $user = null): mixed
    {
        $this->formData = $formData;
        $this->user = $user;

        // Get the formula from field settings
        $formula = $field->attributes['formula'] ?? '';

        if (empty($formula)) {
            return null;
        }

        // Build fields map for the form
        $this->buildFieldsMap($field->form);

        try {
            // Parse and evaluate the formula
            $result = $this->evaluateFormula($formula);

            // Apply formatting if specified
            return $this->formatResult($result, $field->attributes);
        } catch (\Throwable $e) {
            \Log::warning('FormCalculator error', [
                'field' => $field->name,
                'formula' => $formula,
                'error' => $e->getMessage(),
            ]);

            return $field->attributes['default_value'] ?? null;
        }
    }

    /**
     * Validate a formula without executing it
     *
     * @param string $formula The formula to validate
     * @param Form $form The form context
     * @return array Validation result ['valid' => bool, 'errors' => array]
     */
    public function validateFormula(string $formula, Form $form): array
    {
        $errors = [];

        // Check for balanced parentheses
        if (substr_count($formula, '(') !== substr_count($formula, ')')) {
            $errors[] = 'Unbalanced parentheses';
        }

        // Check for invalid characters (security)
        $sanitized = $this->sanitizeFormula($formula);
        if ($sanitized !== $formula) {
            $errors[] = 'Formula contains invalid characters';
        }

        // Check field references exist
        preg_match_all('/\{([a-zA-Z_][a-zA-Z0-9_]*)\}/', $formula, $matches);
        $this->buildFieldsMap($form);

        foreach ($matches[1] as $fieldRef) {
            if (!$this->isValidReference($fieldRef)) {
                $errors[] = "Invalid field reference: {$fieldRef}";
            }
        }

        // Check function names
        preg_match_all('/([A-Z]+)\s*\(/', $formula, $funcMatches);
        foreach ($funcMatches[1] as $funcName) {
            if (!isset(self::FUNCTIONS[$funcName])) {
                $errors[] = "Unknown function: {$funcName}";
            }
        }

        return [
            'valid' => empty($errors),
            'errors' => $errors,
        ];
    }

    /**
     * Get all available functions with descriptions
     */
    public function getAvailableFunctions(): array
    {
        return [
            'Mathematical' => [
                'SUM(values...)' => 'Sum of all values',
                'AVG(values...)' => 'Average of all values',
                'MIN(values...)' => 'Minimum value',
                'MAX(values...)' => 'Maximum value',
                'COUNT(values...)' => 'Count of values',
                'ROUND(value, decimals)' => 'Round to decimals',
                'ABS(value)' => 'Absolute value',
                'CEIL(value)' => 'Round up',
                'FLOOR(value)' => 'Round down',
            ],
            'Logical' => [
                'IF(condition, true_value, false_value)' => 'Conditional result',
                'AND(conditions...)' => 'All conditions true',
                'OR(conditions...)' => 'Any condition true',
                'NOT(condition)' => 'Negate condition',
            ],
            'Text' => [
                'CONCAT(values...)' => 'Concatenate text',
                'FORMAT(value, format)' => 'Format value',
            ],
            'Date' => [
                'NOW()' => 'Current date and time',
                'TODAY()' => 'Current date',
                'DATEDIFF(date1, date2, unit)' => 'Difference between dates',
                'DATEADD(date, amount, unit)' => 'Add to date',
            ],
            'Financial' => [
                'CURRENCY(value, currency_code)' => 'Format as currency',
                'PERCENTAGE(value, total)' => 'Calculate percentage',
            ],
            'Advanced' => [
                'LOOKUP(value, table, column)' => 'Lookup value in table',
            ],
        ];
    }

    /**
     * Build a map of field names to field objects
     */
    private function buildFieldsMap(Form $form): void
    {
        $this->fieldsMap = [];

        foreach ($form->fields as $field) {
            $this->fieldsMap[$field->name] = $field;
        }
    }

    /**
     * Check if a reference is valid
     */
    private function isValidReference(string $ref): bool
    {
        // Check form fields
        if (isset($this->fieldsMap[$ref])) {
            return true;
        }

        // Check smart tags
        $smartTags = ['today', 'now', 'user.name', 'user.email', 'user.id', 'form.id', 'form.title'];
        if (in_array($ref, $smartTags)) {
            return true;
        }

        return false;
    }

    /**
     * Sanitize formula to prevent code injection
     */
    private function sanitizeFormula(string $formula): string
    {
        // Remove any PHP/JS code constructs
        $dangerous = ['<?', '?>', '<script', '</script', 'eval', 'exec', 'system', 'shell_exec'];
        foreach ($dangerous as $pattern) {
            if (stripos($formula, $pattern) !== false) {
                return '';
            }
        }

        return $formula;
    }

    /**
     * Evaluate a formula expression
     */
    private function evaluateFormula(string $formula): mixed
    {
        // Sanitize first
        $formula = $this->sanitizeFormula($formula);

        if (empty($formula)) {
            return null;
        }

        // Replace field references with values
        $formula = $this->replaceFieldReferences($formula);

        // Replace smart tags
        $formula = $this->replaceSmartTags($formula);

        // Process functions
        $formula = $this->processFunctions($formula);

        // Evaluate mathematical expression
        return $this->evaluateMath($formula);
    }

    /**
     * Replace field references {field_name} with actual values
     */
    private function replaceFieldReferences(string $formula): string
    {
        return preg_replace_callback('/\{([a-zA-Z_][a-zA-Z0-9_]*)\}/', function ($matches) {
            $fieldName = $matches[1];

            if (isset($this->formData[$fieldName])) {
                $value = $this->formData[$fieldName];

                // Handle arrays (multi-select, checkboxes)
                if (is_array($value)) {
                    return json_encode($value);
                }

                // Handle numeric values
                if (is_numeric($value)) {
                    return $value;
                }

                // Handle strings - quote them
                return '"' . addslashes($value) . '"';
            }

            return '0'; // Default for missing fields
        }, $formula);
    }

    /**
     * Replace smart tags with actual values
     */
    private function replaceSmartTags(string $formula): string
    {
        $tags = [
            '{today}' => '"' . date('Y-m-d') . '"',
            '{now}' => '"' . date('Y-m-d H:i:s') . '"',
            '{user.name}' => '"' . ($this->user?->name ?? 'Guest') . '"',
            '{user.email}' => '"' . ($this->user?->email ?? '') . '"',
            '{user.id}' => $this->user?->id ?? '0',
            '{timestamp}' => time(),
        ];

        return str_replace(array_keys($tags), array_values($tags), $formula);
    }

    /**
     * Process function calls in formula
     */
    private function processFunctions(string $formula): string
    {
        // Process functions from innermost to outermost
        $maxIterations = 100; // Prevent infinite loops
        $iteration = 0;

        while (preg_match('/([A-Z]+)\s*\(([^()]*)\)/', $formula, $matches) && $iteration < $maxIterations) {
            $fullMatch = $matches[0];
            $funcName = $matches[1];
            $args = $matches[2];

            if (isset(self::FUNCTIONS[$funcName])) {
                $method = self::FUNCTIONS[$funcName];
                $result = $this->$method($args);
                $formula = str_replace($fullMatch, (string) $result, $formula);
            }

            $iteration++;
        }

        return $formula;
    }

    /**
     * Evaluate mathematical expression safely
     */
    private function evaluateMath(string $expression): mixed
    {
        // Clean the expression
        $expression = trim($expression);

        // If it's a simple number, return it
        if (is_numeric($expression)) {
            return floatval($expression);
        }

        // If it's a quoted string, return it
        if (preg_match('/^"([^"]*)"$/', $expression, $matches)) {
            return $matches[1];
        }

        // Use a safe math evaluator
        try {
            // Only allow safe characters
            $safeExpression = preg_replace('/[^0-9+\-*\/().%^ ]/', '', $expression);

            if (empty($safeExpression)) {
                return $expression; // Return original if not math
            }

            // Use bcmath for precision
            return $this->safeMathEval($safeExpression);
        } catch (\Throwable $e) {
            return $expression;
        }
    }

    /**
     * Safe math evaluation without eval()
     */
    private function safeMathEval(string $expression): float
    {
        // Tokenize the expression
        $tokens = $this->tokenize($expression);

        // Convert to postfix notation (Shunting Yard algorithm)
        $postfix = $this->toPostfix($tokens);

        // Evaluate postfix expression
        return $this->evaluatePostfix($postfix);
    }

    /**
     * Tokenize mathematical expression
     */
    private function tokenize(string $expression): array
    {
        $tokens = [];
        $number = '';

        for ($i = 0; $i < strlen($expression); $i++) {
            $char = $expression[$i];

            if (is_numeric($char) || $char === '.') {
                $number .= $char;
            } else {
                if ($number !== '') {
                    $tokens[] = floatval($number);
                    $number = '';
                }

                if (in_array($char, ['+', '-', '*', '/', '%', '^', '(', ')'])) {
                    $tokens[] = $char;
                }
            }
        }

        if ($number !== '') {
            $tokens[] = floatval($number);
        }

        return $tokens;
    }

    /**
     * Convert infix to postfix notation
     */
    private function toPostfix(array $tokens): array
    {
        $output = [];
        $operators = [];

        $precedence = ['+' => 1, '-' => 1, '*' => 2, '/' => 2, '%' => 2, '^' => 3];

        foreach ($tokens as $token) {
            if (is_numeric($token)) {
                $output[] = $token;
            } elseif ($token === '(') {
                $operators[] = $token;
            } elseif ($token === ')') {
                while (!empty($operators) && end($operators) !== '(') {
                    $output[] = array_pop($operators);
                }
                array_pop($operators); // Remove '('
            } elseif (isset($precedence[$token])) {
                while (
                    !empty($operators) &&
                    end($operators) !== '(' &&
                    isset($precedence[end($operators)]) &&
                    $precedence[end($operators)] >= $precedence[$token]
                ) {
                    $output[] = array_pop($operators);
                }
                $operators[] = $token;
            }
        }

        while (!empty($operators)) {
            $output[] = array_pop($operators);
        }

        return $output;
    }

    /**
     * Evaluate postfix expression
     */
    private function evaluatePostfix(array $postfix): float
    {
        $stack = [];

        foreach ($postfix as $token) {
            if (is_numeric($token)) {
                $stack[] = $token;
            } else {
                $b = array_pop($stack) ?? 0;
                $a = array_pop($stack) ?? 0;

                $result = match ($token) {
                    '+' => $a + $b,
                    '-' => $a - $b,
                    '*' => $a * $b,
                    '/' => $b != 0 ? $a / $b : 0,
                    '%' => $b != 0 ? fmod($a, $b) : 0,
                    '^' => pow($a, $b),
                    default => 0,
                };

                $stack[] = $result;
            }
        }

        return array_pop($stack) ?? 0;
    }

    /**
     * Format the result based on field settings
     */
    private function formatResult(mixed $result, array $attributes): mixed
    {
        $format = $attributes['format'] ?? null;
        $decimals = $attributes['decimals'] ?? 2;
        $prefix = $attributes['prefix'] ?? '';
        $suffix = $attributes['suffix'] ?? '';

        if (!is_numeric($result)) {
            return $result;
        }

        switch ($format) {
            case 'currency':
                $currency = $attributes['currency'] ?? 'USD';
                $formatted = number_format((float) $result, $decimals);
                $symbols = ['USD' => '$', 'EUR' => '€', 'GBP' => '£', 'JPY' => '¥', 'BRL' => 'R$'];
                return ($symbols[$currency] ?? $currency . ' ') . $formatted;

            case 'percentage':
                return number_format((float) $result, $decimals) . '%';

            case 'number':
                return number_format((float) $result, $decimals);

            default:
                return $prefix . $result . $suffix;
        }
    }

    // =========================================================================
    // FUNCTION IMPLEMENTATIONS
    // =========================================================================

    private function calculateSum(string $args): float
    {
        $values = $this->parseArgs($args);
        return array_sum(array_map('floatval', $values));
    }

    private function calculateAvg(string $args): float
    {
        $values = $this->parseArgs($args);
        $count = count($values);
        return $count > 0 ? array_sum(array_map('floatval', $values)) / $count : 0;
    }

    private function calculateMin(string $args): float
    {
        $values = array_map('floatval', $this->parseArgs($args));
        return !empty($values) ? min($values) : 0;
    }

    private function calculateMax(string $args): float
    {
        $values = array_map('floatval', $this->parseArgs($args));
        return !empty($values) ? max($values) : 0;
    }

    private function calculateCount(string $args): int
    {
        return count($this->parseArgs($args));
    }

    private function calculateRound(string $args): float
    {
        $parts = $this->parseArgs($args);
        $value = floatval($parts[0] ?? 0);
        $decimals = intval($parts[1] ?? 0);
        return round($value, $decimals);
    }

    private function calculateAbs(string $args): float
    {
        return abs(floatval(trim($args)));
    }

    private function calculateCeil(string $args): float
    {
        return ceil(floatval(trim($args)));
    }

    private function calculateFloor(string $args): float
    {
        return floor(floatval(trim($args)));
    }

    private function calculateIf(string $args): mixed
    {
        $parts = $this->parseArgs($args);

        if (count($parts) < 3) {
            return $parts[1] ?? '';
        }

        $condition = $this->evaluateCondition(trim($parts[0]));
        return $condition ? trim($parts[1]) : trim($parts[2]);
    }

    private function calculateAnd(string $args): bool
    {
        $conditions = $this->parseArgs($args);
        foreach ($conditions as $condition) {
            if (!$this->evaluateCondition(trim($condition))) {
                return false;
            }
        }
        return true;
    }

    private function calculateOr(string $args): bool
    {
        $conditions = $this->parseArgs($args);
        foreach ($conditions as $condition) {
            if ($this->evaluateCondition(trim($condition))) {
                return true;
            }
        }
        return false;
    }

    private function calculateNot(string $args): bool
    {
        return !$this->evaluateCondition(trim($args));
    }

    private function calculateConcat(string $args): string
    {
        $parts = $this->parseArgs($args);
        return implode('', array_map(function ($part) {
            return trim($part, '"\'');
        }, $parts));
    }

    private function calculateDateDiff(string $args): int
    {
        $parts = $this->parseArgs($args);
        $date1 = strtotime(trim($parts[0] ?? '', '"'));
        $date2 = strtotime(trim($parts[1] ?? '', '"'));
        $unit = strtolower(trim($parts[2] ?? 'days', '"'));

        $diff = abs($date2 - $date1);

        return match ($unit) {
            'seconds' => $diff,
            'minutes' => intval($diff / 60),
            'hours' => intval($diff / 3600),
            'days' => intval($diff / 86400),
            'weeks' => intval($diff / 604800),
            'months' => intval($diff / 2592000),
            'years' => intval($diff / 31536000),
            default => intval($diff / 86400),
        };
    }

    private function calculateDateAdd(string $args): string
    {
        $parts = $this->parseArgs($args);
        $date = trim($parts[0] ?? '', '"');
        $amount = intval($parts[1] ?? 0);
        $unit = strtolower(trim($parts[2] ?? 'days', '"'));

        $timestamp = strtotime($date);
        $modified = strtotime("+{$amount} {$unit}", $timestamp);

        return date('Y-m-d', $modified);
    }

    private function calculateNow(): string
    {
        return '"' . date('Y-m-d H:i:s') . '"';
    }

    private function calculateToday(): string
    {
        return '"' . date('Y-m-d') . '"';
    }

    private function calculateFormat(string $args): string
    {
        $parts = $this->parseArgs($args);
        $value = trim($parts[0] ?? '');
        $format = trim($parts[1] ?? '', '"');

        if (is_numeric($value)) {
            return sprintf($format, floatval($value));
        }

        return $value;
    }

    private function calculateCurrency(string $args): string
    {
        $parts = $this->parseArgs($args);
        $value = floatval($parts[0] ?? 0);
        $currency = strtoupper(trim($parts[1] ?? 'USD', '"'));

        $symbols = ['USD' => '$', 'EUR' => '€', 'GBP' => '£', 'JPY' => '¥', 'BRL' => 'R$', 'CAD' => 'C$', 'AUD' => 'A$'];
        $symbol = $symbols[$currency] ?? $currency . ' ';

        return $symbol . number_format($value, 2);
    }

    private function calculatePercentage(string $args): string
    {
        $parts = $this->parseArgs($args);
        $value = floatval($parts[0] ?? 0);
        $total = floatval($parts[1] ?? 100);

        if ($total == 0) {
            return '0%';
        }

        return number_format(($value / $total) * 100, 2) . '%';
    }

    private function calculateLookup(string $args): mixed
    {
        $parts = $this->parseArgs($args);
        $value = trim($parts[0] ?? '');
        $table = json_decode(trim($parts[1] ?? '[]'), true);
        $column = trim($parts[2] ?? 'value', '"');

        if (!is_array($table)) {
            return '';
        }

        foreach ($table as $row) {
            if (isset($row['key']) && $row['key'] == $value) {
                return $row[$column] ?? '';
            }
        }

        return '';
    }

    /**
     * Parse function arguments
     */
    private function parseArgs(string $args): array
    {
        $result = [];
        $current = '';
        $depth = 0;
        $inQuote = false;
        $quoteChar = '';

        for ($i = 0; $i < strlen($args); $i++) {
            $char = $args[$i];

            if (($char === '"' || $char === "'") && !$inQuote) {
                $inQuote = true;
                $quoteChar = $char;
                $current .= $char;
            } elseif ($char === $quoteChar && $inQuote) {
                $inQuote = false;
                $current .= $char;
            } elseif ($char === '(' && !$inQuote) {
                $depth++;
                $current .= $char;
            } elseif ($char === ')' && !$inQuote) {
                $depth--;
                $current .= $char;
            } elseif ($char === ',' && $depth === 0 && !$inQuote) {
                $result[] = trim($current);
                $current = '';
            } else {
                $current .= $char;
            }
        }

        if (trim($current) !== '') {
            $result[] = trim($current);
        }

        return $result;
    }

    /**
     * Evaluate a condition expression
     */
    private function evaluateCondition(string $condition): bool
    {
        // Check for comparison operators
        foreach (self::COMPARISON_OPERATORS as $op) {
            if (strpos($condition, $op) !== false) {
                $parts = explode($op, $condition, 2);
                $left = $this->evaluateMath(trim($parts[0]));
                $right = $this->evaluateMath(trim($parts[1]));

                return match ($op) {
                    '==' => $left == $right,
                    '!=' => $left != $right,
                    '>' => $left > $right,
                    '<' => $left < $right,
                    '>=' => $left >= $right,
                    '<=' => $left <= $right,
                    '===' => $left === $right,
                    '!==' => $left !== $right,
                    default => false,
                };
            }
        }

        // Truthy check
        $value = $this->evaluateMath($condition);
        return !empty($value) && $value !== '0' && $value !== 'false';
    }
}
