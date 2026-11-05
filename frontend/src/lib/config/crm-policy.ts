import source from '../../../../api/assets/policies/crm-permissions.json?raw';
import { CrmPolicyError, hashCrmPolicy, parseCrmPolicy, type CrmPolicy } from '../crm/policy.js';

let compiled:
	| { readonly policy: CrmPolicy; readonly hash: Promise<string> }
	| CrmPolicyError
	| undefined;
/** Cache only immutable public policy, never identity/grants/authorization results. */
export function currentCrmPolicy(): {
	readonly policy: CrmPolicy;
	readonly hash: Promise<string>;
} {
	if (!compiled) {
		try {
			const policy = parseCrmPolicy(source);
			let digest: Promise<string> | undefined;
			compiled = Object.freeze({
				policy,
				get hash() {
					return (digest ??= hashCrmPolicy(source));
				}
			});
		} catch {
			compiled = new CrmPolicyError('Current CRM policy is invalid');
		}
	}
	if (compiled instanceof CrmPolicyError) throw compiled;
	return compiled;
}
