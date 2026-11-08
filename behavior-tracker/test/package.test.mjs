import assert from 'node:assert/strict';
import test from 'node:test';
import { BehaviorTracker, generateSessionId } from '../dist/index.js';

test('packed entry point loads without browser globals', () => {
	assert.equal(typeof BehaviorTracker, 'function');
	assert.notEqual(generateSessionId(), generateSessionId());
});
