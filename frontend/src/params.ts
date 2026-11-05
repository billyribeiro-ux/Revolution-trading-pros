import { defineParams } from '@sveltejs/kit/params';

/** Numeric profile endpoints must not consume named member APIs such as churned/templates. */
export const params = defineParams({
	member_id: (value) => {
		if (!/^[1-9]\d{0,18}$/.test(value) || BigInt(value) > 9223372036854775807n) return;
		return value;
	}
});
