<script lang="ts">
	interface Props {
		password: string;
		memberName: string | null;
		onCopy: () => void;
		onDismiss: () => void;
	}

	let { password, memberName, onCopy, onDismiss }: Props = $props();
</script>

<!--
	FIX-2026-04-26 (audit 02 §P1-9): one-time temporary-password reveal modal.
	Replaces the old plaintext-password-in-toast pattern. The modal is the
	ONLY surface that ever renders the password (no logging, no toast).
-->
<div
	class="temp-password-overlay"
	role="dialog"
	aria-modal="true"
	aria-labelledby="temp-password-title"
>
	<div class="temp-password-modal">
		<h2 id="temp-password-title">Temporary password</h2>
		<p class="temp-password-subtitle">
			One-time view for <strong>{memberName}</strong>. Copy it now — it will not be shown again, and
			we never store it server-side after creation.
		</p>
		<div class="temp-password-value">
			<code>{password}</code>
		</div>
		<div class="temp-password-actions">
			<button type="button" class="btn-primary" onclick={onCopy}>Copy to clipboard</button>
			<button type="button" class="btn-secondary" onclick={onDismiss}>
				I have recorded this — close
			</button>
		</div>
	</div>
</div>
