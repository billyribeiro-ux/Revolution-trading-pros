/**
 * Safe Math Expression Parser
 *
 * A secure recursive descent parser for evaluating mathematical expressions.
 * This replaces dangerous new Function() / eval() usage with a safe whitelist-based approach.
 *
 * Supported operations:
 * - Numbers (integers and decimals, including negative)
 * - Addition (+)
 * - Subtraction (-)
 * - Multiplication (*)
 * - Division (/)
 * - Modulo (%)
 * - Parentheses for grouping
 *
 * @version 1.0.0
 * @security This parser does NOT execute arbitrary code - it only evaluates safe math expressions
 */

// Token types for the lexer
type TokenType =
	| 'NUMBER'
	| 'PLUS'
	| 'MINUS'
	| 'MULTIPLY'
	| 'DIVIDE'
	| 'MODULO'
	| 'LPAREN'
	| 'RPAREN'
	| 'EOF';

interface Token {
	type: TokenType;
	value: string;
	position: number;
}

/**
 * Lexer: Converts expression string into tokens
 */
function tokenize(expression: string): Token[] {
	const tokens: Token[] = [];
	let position = 0;

	// Skip whitespace
	const skipWhitespace = () => {
		while (position < expression.length && /\s/.test(expression[position])) {
			position++;
		}
	};

	while (position < expression.length) {
		skipWhitespace();

		if (position >= expression.length) break;

		const char = expression[position];

		// Numbers (including decimals)
		if (/[0-9]/.test(char) || (char === '.' && /[0-9]/.test(expression[position + 1] || ''))) {
			let numStr = '';
			const startPos = position;
			let hasDecimal = false;

			while (position < expression.length) {
				const c = expression[position];
				if (/[0-9]/.test(c)) {
					numStr += c;
					position++;
				} else if (c === '.' && !hasDecimal) {
					hasDecimal = true;
					numStr += c;
					position++;
				} else {
					break;
				}
			}

			tokens.push({ type: 'NUMBER', value: numStr, position: startPos });
			continue;
		}

		// Operators and parentheses
		switch (char) {
			case '+':
				tokens.push({ type: 'PLUS', value: '+', position });
				position++;
				break;
			case '-':
				tokens.push({ type: 'MINUS', value: '-', position });
				position++;
				break;
			case '*':
				tokens.push({ type: 'MULTIPLY', value: '*', position });
				position++;
				break;
			case '/':
				tokens.push({ type: 'DIVIDE', value: '/', position });
				position++;
				break;
			case '%':
				tokens.push({ type: 'MODULO', value: '%', position });
				position++;
				break;
			case '(':
				tokens.push({ type: 'LPAREN', value: '(', position });
				position++;
				break;
			case ')':
				tokens.push({ type: 'RPAREN', value: ')', position });
				position++;
				break;
			default:
				// Invalid character - reject the expression for security
				throw new SafeMathError(`Invalid character '${char}' at position ${position}`, position);
		}
	}

	tokens.push({ type: 'EOF', value: '', position });
	return tokens;
}

/**
 * Custom error class for safe math parsing errors
 */
export class SafeMathError extends Error {
	position: number;

	constructor(message: string, position: number) {
		super(message);
		this.name = 'SafeMathError';
		this.position = position;
	}
}

/**
 * Parser: Recursive descent parser implementing proper operator precedence
 *
 * Grammar:
 *   expression := term (('+' | '-') term)*
 *   term       := factor (('*' | '/' | '%') factor)*
 *   factor     := unary | '(' expression ')' | NUMBER
 *   unary      := ('-' | '+') factor
 */
class SafeMathParser {
	private tokens: Token[];
	private current: number;

	constructor(tokens: Token[]) {
		this.tokens = tokens;
		this.current = 0;
	}

	private peek(): Token {
		return this.tokens[this.current];
	}

	private advance(): Token {
		const token = this.tokens[this.current];
		if (token.type !== 'EOF') {
			this.current++;
		}
		return token;
	}

	private expect(type: TokenType): Token {
		const token = this.peek();
		if (token.type !== type) {
			throw new SafeMathError(
				`Expected ${type} but found ${token.type} at position ${token.position}`,
				token.position
			);
		}
		return this.advance();
	}

	/**
	 * Parse and evaluate the expression
	 */
	parse(): number {
		const result = this.expression();

		// Ensure we consumed all tokens
		if (this.peek().type !== 'EOF') {
			const token = this.peek();
			throw new SafeMathError(
				`Unexpected token '${token.value}' at position ${token.position}`,
				token.position
			);
		}

		return result;
	}

	/**
	 * expression := term (('+' | '-') term)*
	 */
	private expression(): number {
		let result = this.term();

		while (this.peek().type === 'PLUS' || this.peek().type === 'MINUS') {
			const operator = this.advance();
			const right = this.term();

			if (operator.type === 'PLUS') {
				result = result + right;
			} else {
				result = result - right;
			}
		}

		return result;
	}

	/**
	 * term := factor (('*' | '/' | '%') factor)*
	 */
	private term(): number {
		let result = this.factor();

		while (
			this.peek().type === 'MULTIPLY' ||
			this.peek().type === 'DIVIDE' ||
			this.peek().type === 'MODULO'
		) {
			const operator = this.advance();
			const right = this.factor();

			if (operator.type === 'MULTIPLY') {
				result = result * right;
			} else if (operator.type === 'DIVIDE') {
				if (right === 0) {
					throw new SafeMathError('Division by zero', operator.position);
				}
				result = result / right;
			} else {
				if (right === 0) {
					throw new SafeMathError('Modulo by zero', operator.position);
				}
				result = result % right;
			}
		}

		return result;
	}

	/**
	 * factor := unary | '(' expression ')' | NUMBER
	 */
	private factor(): number {
		const token = this.peek();

		// Unary minus or plus
		if (token.type === 'MINUS' || token.type === 'PLUS') {
			this.advance();
			const value = this.factor();
			return token.type === 'MINUS' ? -value : value;
		}

		// Parenthesized expression
		if (token.type === 'LPAREN') {
			this.advance();
			const result = this.expression();
			this.expect('RPAREN');
			return result;
		}

		// Number literal
		if (token.type === 'NUMBER') {
			this.advance();
			const value = parseFloat(token.value);
			if (isNaN(value)) {
				throw new SafeMathError(`Invalid number '${token.value}'`, token.position);
			}
			return value;
		}

		throw new SafeMathError(
			`Unexpected token '${token.value}' at position ${token.position}`,
			token.position
		);
	}
}

/**
 * Safely evaluate a mathematical expression
 *
 * @param expression - The math expression to evaluate
 * @returns The calculated result
 * @throws SafeMathError if the expression is invalid or contains unsafe content
 *
 * @example
 * safeMathEval('2 + 3 * 4')      // Returns 14
 * safeMathEval('(2 + 3) * 4')    // Returns 20
 * safeMathEval('10 / 2 - 3')     // Returns 2
 * safeMathEval('-5 + 10')        // Returns 5
 * safeMathEval('100 % 30')       // Returns 10
 */
export function safeMathEval(expression: string): number {
	// Reject empty expressions
	if (!expression || !expression.trim()) {
		return 0;
	}

	// Additional security: Check for suspicious patterns before tokenizing
	// This catches attempts to use object property access, function calls, etc.
	const suspiciousPatterns = [
		/\[/, // Array access
		/\]/, // Array access
		/{/, // Object literal
		/}/, // Object literal
		/`/, // Template literal
		/"/, // String literal
		/'/, // String literal
		/;/, // Statement separator
		/=/, // Assignment
		/\bfunction\b/i, // Function keyword
		/\breturn\b/i, // Return keyword
		/\bvar\b/i, // var keyword
		/\blet\b/i, // let keyword
		/\bconst\b/i, // const keyword
		/\beval\b/i, // eval
		/\bimport\b/i, // import
		/\bexport\b/i, // export
		/\brequire\b/i, // require
		/\bwindow\b/i, // window
		/\bdocument\b/i, // document
		/\bglobal\b/i, // global
		/\bprocess\b/i, // process
		/\b__proto__\b/i, // prototype pollution
		/\bprototype\b/i, // prototype
		/\bconstructor\b/i, // constructor
		/\\x/, // Hex escape
		/\\u/ // Unicode escape
	];

	for (const pattern of suspiciousPatterns) {
		if (pattern.test(expression)) {
			throw new SafeMathError('Expression contains disallowed characters or keywords', 0);
		}
	}

	try {
		const tokens = tokenize(expression);
		const parser = new SafeMathParser(tokens);
		const result = parser.parse();

		// Validate the result
		if (!isFinite(result)) {
			throw new SafeMathError('Result is not a finite number', 0);
		}

		return result;
	} catch (error) {
		if (error instanceof SafeMathError) {
			throw error;
		}
		throw new SafeMathError('Failed to evaluate expression', 0);
	}
}

/**
 * Try to safely evaluate a math expression, returning a default value on error
 *
 * @param expression - The math expression to evaluate
 * @param defaultValue - Value to return if evaluation fails (default: 0)
 * @returns The calculated result or the default value
 *
 * @example
 * tryMathEval('2 + 3')           // Returns 5
 * tryMathEval('invalid', 0)      // Returns 0
 * tryMathEval('1/0', -1)         // Returns -1 (division by zero)
 */
export function tryMathEval(expression: string, defaultValue: number = 0): number {
	try {
		return safeMathEval(expression);
	} catch {
		return defaultValue;
	}
}
