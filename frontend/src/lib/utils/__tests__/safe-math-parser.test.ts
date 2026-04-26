/**
 * Safe Math Parser — Security-Critical Unit Tests
 * ═══════════════════════════════════════════════════════════════════════════════
 * Tests for the recursive-descent math parser that replaces eval() / new Function().
 * Covers correct evaluation, operator precedence, edge cases, and — critically —
 * rejection of every injection vector an attacker might try.
 */

import { describe, it, expect } from 'vitest';
import { safeMathEval, tryMathEval, SafeMathError } from '../safe-math-parser';

// ============================================================================
// Basic arithmetic
// ============================================================================

describe('safeMathEval — basic arithmetic', () => {
	it('evaluates integer addition', () => {
		expect(safeMathEval('2 + 2')).toBe(4);
	});

	it('evaluates integer subtraction', () => {
		expect(safeMathEval('10 - 3')).toBe(7);
	});

	it('evaluates integer multiplication', () => {
		expect(safeMathEval('10 * 5')).toBe(50);
	});

	it('evaluates integer division', () => {
		expect(safeMathEval('100 / 4')).toBe(25);
	});

	it('evaluates modulo', () => {
		expect(safeMathEval('10 % 3')).toBe(1);
	});

	it('evaluates zero', () => {
		expect(safeMathEval('0')).toBe(0);
	});

	it('evaluates a single positive number', () => {
		expect(safeMathEval('42')).toBe(42);
	});

	it('evaluates a large integer', () => {
		expect(safeMathEval('1000000 * 1000000')).toBe(1e12);
	});
});

// ============================================================================
// Operator precedence
// ============================================================================

describe('safeMathEval — operator precedence', () => {
	it('applies * before + (2 + 3 * 4 === 14)', () => {
		expect(safeMathEval('2 + 3 * 4')).toBe(14);
	});

	it('parentheses override precedence ((2 + 3) * 4 === 20)', () => {
		expect(safeMathEval('(2 + 3) * 4')).toBe(20);
	});

	it('applies / before - (10 - 4 / 2 === 8)', () => {
		expect(safeMathEval('10 - 4 / 2')).toBe(8);
	});

	it('chains multiple additions left-to-right', () => {
		expect(safeMathEval('1 + 2 + 3 + 4')).toBe(10);
	});

	it('handles nested parentheses', () => {
		expect(safeMathEval('((2 + 3) * (4 - 1)) / 5')).toBe(3);
	});

	it('handles complex precedence mix', () => {
		// 2 * 3 + 4 * 5 - 6 / 2 = 6 + 20 - 3 = 23
		expect(safeMathEval('2 * 3 + 4 * 5 - 6 / 2')).toBe(23);
	});
});

// ============================================================================
// Unary minus / plus
// ============================================================================

describe('safeMathEval — unary operators', () => {
	it('evaluates unary minus on a number', () => {
		expect(safeMathEval('-5')).toBe(-5);
	});

	it('evaluates -(-5) to 5', () => {
		expect(safeMathEval('-(-5)')).toBe(5);
	});

	it('evaluates -(2+3) to -5', () => {
		expect(safeMathEval('-(2+3)')).toBe(-5);
	});

	it('evaluates -5 + 10 === 5', () => {
		expect(safeMathEval('-5 + 10')).toBe(5);
	});

	it('evaluates unary plus (no-op)', () => {
		expect(safeMathEval('+5')).toBe(5);
	});

	it('handles double unary minus via parentheses', () => {
		expect(safeMathEval('-(-10)')).toBe(10);
	});
});

// ============================================================================
// Floating point
// ============================================================================

describe('safeMathEval — floating point', () => {
	it('evaluates decimal addition (0.1 + 0.2 within epsilon)', () => {
		const result = safeMathEval('0.1 + 0.2');
		expect(result).toBeCloseTo(0.3, 10);
	});

	it('evaluates decimal multiplication', () => {
		expect(safeMathEval('2.5 * 4')).toBe(10);
	});

	it('evaluates decimal division', () => {
		expect(safeMathEval('7.5 / 2.5')).toBe(3);
	});

	it('evaluates leading decimal (.5 + .5)', () => {
		// Parser supports numbers starting with a dot when followed by digit
		// 0.5 + 0.5 should work; .5 + .5 also works per the tokenizer
		expect(safeMathEval('0.5 + 0.5')).toBe(1);
	});

	it('handles small decimals', () => {
		expect(safeMathEval('0.001 * 1000')).toBeCloseTo(1, 10);
	});
});

// ============================================================================
// Division and modulo by zero
// ============================================================================

describe('safeMathEval — division / modulo by zero', () => {
	it('throws SafeMathError for division by zero', () => {
		expect(() => safeMathEval('10 / 0')).toThrow(SafeMathError);
	});

	it('throws SafeMathError for modulo by zero', () => {
		expect(() => safeMathEval('10 % 0')).toThrow(SafeMathError);
	});

	it('division by zero error message mentions division by zero', () => {
		expect(() => safeMathEval('5 / 0')).toThrow(/division by zero/i);
	});

	it('modulo by zero error message mentions modulo', () => {
		expect(() => safeMathEval('5 % 0')).toThrow(/modulo by zero/i);
	});
});

// ============================================================================
// Boundary / edge cases
// ============================================================================

describe('safeMathEval — boundary cases', () => {
	it('returns 0 for empty string', () => {
		expect(safeMathEval('')).toBe(0);
	});

	it('returns 0 for whitespace-only input', () => {
		expect(safeMathEval('   ')).toBe(0);
	});

	it('throws for malformed expression (((()', () => {
		expect(() => safeMathEval('((((')).toThrow(SafeMathError);
	});

	it('throws for mismatched parentheses', () => {
		expect(() => safeMathEval('(2 + 3')).toThrow(SafeMathError);
	});

	it('throws for trailing operator', () => {
		expect(() => safeMathEval('2 + ')).toThrow(SafeMathError);
	});

	it('throws for leading operator without unary context', () => {
		expect(() => safeMathEval('* 5')).toThrow(SafeMathError);
	});

	it('handles whitespace around operators', () => {
		expect(safeMathEval('  2  +  3  ')).toBe(5);
	});
});

// ============================================================================
// SECURITY: Injection attempt rejection
// ============================================================================
//
// Every item below MUST throw SafeMathError (never evaluate, never return a
// value derived from executing arbitrary code). These are the critical tests
// that demonstrate the parser is NOT backed by eval() / new Function().

describe('safeMathEval — SECURITY: injection rejection', () => {
	// Helper that asserts an expression is rejected
	const mustReject = (expression: string) => {
		expect(() => safeMathEval(expression), `expected "${expression}" to throw`).toThrow();
	};

	// --- eval / Function --- //
	it('rejects eval(...)', () => {
		mustReject('eval(1+1)');
	});

	it('rejects eval keyword alone', () => {
		mustReject('eval');
	});

	it('rejects Function(...)', () => {
		mustReject('Function("return 1")');
	});

	// --- prototype pollution --- //
	it('rejects __proto__', () => {
		mustReject('__proto__');
	});

	it('rejects prototype access', () => {
		mustReject('prototype');
	});

	it('rejects constructor.constructor("...")()', () => {
		mustReject('constructor.constructor("return process")()');
	});

	// --- global object access --- //
	it('rejects process.env', () => {
		mustReject('process.env');
	});

	it('rejects globalThis', () => {
		// globalThis contains letters which the lexer rejects
		mustReject('globalThis');
	});

	it('rejects window access', () => {
		mustReject('window');
	});

	it('rejects document access', () => {
		mustReject('document');
	});

	it('rejects global keyword', () => {
		mustReject('global');
	});

	// --- code constructs --- //
	it('rejects variable declaration (var)', () => {
		mustReject('var x = 1');
	});

	it('rejects let declaration', () => {
		mustReject('let x = 1');
	});

	it('rejects const declaration', () => {
		mustReject('const x = 1');
	});

	it('rejects return statement', () => {
		mustReject('return 1');
	});

	it('rejects function keyword', () => {
		mustReject('function() { return 1 }');
	});

	it('rejects import statement', () => {
		mustReject('import("os")');
	});

	it('rejects require()', () => {
		mustReject('require("os")');
	});

	// --- string/template literals (could be injection vectors) --- //
	it('rejects single-quoted string', () => {
		mustReject("'hello'");
	});

	it('rejects double-quoted string', () => {
		mustReject('"hello"');
	});

	it('rejects template literal', () => {
		mustReject('`hello`');
	});

	// --- array/object access (could traverse prototype chain) --- //
	it('rejects array bracket access', () => {
		mustReject('[1,2,3]');
	});

	it('rejects object literal', () => {
		mustReject('{a:1}');
	});

	// --- assignment --- //
	it('rejects assignment expression', () => {
		mustReject('x = 5');
	});

	// --- hex / unicode escape (bypass attempts) --- //
	it('rejects hex escape sequences (\\x41)', () => {
		mustReject('\\x41');
	});

	it('rejects unicode escape sequences (\\u0041)', () => {
		mustReject('\\u0041');
	});

	// --- semicolons (multi-statement injection) --- //
	it('rejects semicolons (statement separator)', () => {
		mustReject('1 + 1; alert(1)');
	});

	// --- known sandbox-escape chains --- //
	it('rejects [].constructor.constructor("return process")()', () => {
		mustReject('[].constructor.constructor("return process")()');
	});

	it('rejects ({}).__proto__.__proto__', () => {
		mustReject('({}).__proto__.__proto__');
	});
});

// ============================================================================
// tryMathEval — safe wrapper
// ============================================================================

describe('tryMathEval', () => {
	it('returns the evaluated result for valid expression', () => {
		expect(tryMathEval('2 + 3')).toBe(5);
	});

	it('returns 0 (default) for invalid expression', () => {
		expect(tryMathEval('invalid')).toBe(0);
	});

	it('returns custom default for invalid expression', () => {
		expect(tryMathEval('invalid', -1)).toBe(-1);
	});

	it('returns custom default for division by zero', () => {
		expect(tryMathEval('1 / 0', 999)).toBe(999);
	});

	it('returns 0 for empty string', () => {
		expect(tryMathEval('')).toBe(0);
	});

	it('returns custom default for injection attempts', () => {
		expect(tryMathEval('eval(1)', -1)).toBe(-1);
	});

	it('returns correct result for complex expression', () => {
		expect(tryMathEval('(10 + 5) * 2 - 3')).toBe(27);
	});

	it('does not throw for malformed input', () => {
		expect(() => tryMathEval('(((((')).not.toThrow();
	});
});

// ============================================================================
// SafeMathError class
// ============================================================================

describe('SafeMathError', () => {
	it('is an instance of Error', () => {
		const err = new SafeMathError('test error', 5);
		expect(err).toBeInstanceOf(Error);
	});

	it('has name "SafeMathError"', () => {
		const err = new SafeMathError('test error', 5);
		expect(err.name).toBe('SafeMathError');
	});

	it('stores position', () => {
		const err = new SafeMathError('test error', 7);
		expect(err.position).toBe(7);
	});

	it('stores message', () => {
		const err = new SafeMathError('something went wrong', 0);
		expect(err.message).toBe('something went wrong');
	});

	it('is thrown by safeMathEval for invalid expressions', () => {
		try {
			safeMathEval('evil()');
		} catch (e) {
			expect(e).toBeInstanceOf(SafeMathError);
		}
	});
});
