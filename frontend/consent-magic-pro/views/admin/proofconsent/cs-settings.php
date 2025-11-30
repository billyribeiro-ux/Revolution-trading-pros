<?php

namespace ConsentMagicPro;

// If this file is called directly, abort.
if ( !defined( 'WPINC' ) ) {
	die;
}
?>
<div class="cs-tab-content cm-proofconsent" data-id="<?php echo esc_attr( $target_id ); ?>">
    <div class="gap-24">
        <div class="d-flex align-items-center">
			<?php render_switcher_input( 'cs_proof_consent_enable' ); ?>
            <h4 class="secondary-heading switcher-label"><?php esc_html_e( 'Enable Consent Proof Storing', 'consent-magic' ); ?></h4>
        </div>

        <div class="line"></div>

        <h3 class="primary-heading-type2"><?php esc_html_e( 'Store consent for these types of consent', 'consent-magic' ); ?></h3>

        <div class="d-flex align-items-center">
			<?php render_switcher_input( 'cs_proof_just_inform' ); ?>
            <h4 class="secondary-heading switcher-label"><?php esc_html_e( 'Just inform', 'consent-magic' ); ?></h4>
        </div>

        <div class="d-flex align-items-center">
			<?php render_switcher_input( 'cs_proof_inform_and_optout' ); ?>
            <h4 class="secondary-heading switcher-label"><?php esc_html_e( 'Inform and opt-out', 'consent-magic' ); ?></h4>
        </div>

        <div class="d-flex align-items-center">
			<?php render_switcher_input( 'cs_proof_ask_before_tracking' ); ?>
            <h4 class="secondary-heading switcher-label"><?php esc_html_e( 'Ask before tracking', 'consent-magic' ); ?></h4>
        </div>

        <div class="d-flex align-items-center">
			<?php render_switcher_input( 'cs_proof_iab' ); ?>
            <h4 class="secondary-heading switcher-label"><?php esc_html_e( 'IAB', 'consent-magic' ); ?></h4>
        </div>

		<?php render_info_message( __( 'Each rule has the option to exclude it from consent storage.', 'consent-magic' ) ); ?>

        <div class="line"></div>

        <div class="store-consent">
            <p class="font-semibold mr-8"><?php esc_html_e( 'Store consent for ', 'consent-magic' ); ?></p>
            <div class="mr-8">
				<?php render_number_input( 'cs_stored_consent_for', false ); ?>
            </div>
            <p class="font-semibold"><?php esc_html_e( 'days.', 'consent-magic' ); ?></p>
			<?php esc_html_e( 'Leave blank for unlimited.', 'consent-magic' ); ?>
        </div>

        <div class="line"></div>

        <div class="d-flex align-items-center">
			<?php render_switcher_input( 'cs_proof_auto_delete' ); ?>
            <h4 class="secondary-heading switcher-label"><?php esc_html_e( 'Automatically delete consent', 'consent-magic' ); ?></h4>
        </div>

        <div>
            <div class="mb-4">
                <h4 class="font-semibold-type2 mb-4">
					<?php esc_html_e( 'If there are more than', 'consent-magic' ); ?>
                </h4>
				<?php render_number_text_input( 'cs_proof_entries_count', null, false, false, false, null, 'full' ); ?>
            </div>
            <p class="text-gray text-small"><?php esc_html_e( 'entries.', 'consent-magic' ); ?><?php esc_html_e( 'Maximum', 'consent-magic' ); ?><?php echo esc_html( ConsentMagic()->getOption( 'cs_proof_entries_count' ) ); ?><?php esc_html_e( 'entries, to keep the database size under control.', 'consent-magic' ); ?></p>
        </div>

        <div class="d-flex align-items-center">
			<?php render_switcher_input( 'cs_email_before_delete_consent' ); ?>
            <h4 class="secondary-heading switcher-label"><?php esc_html_e( 'Send consent to email before deleting', 'consent-magic' ); ?></h4>
        </div>

        <div>
            <h4 class="font-semibold-type2 mb-4">
				<?php esc_html_e( 'Send consent to this email before deleting', 'consent-magic' ); ?>:
            </h4>
			<?php renderProofDeletingEmail( 'cs_send_proof_deleting_email', false, 'full' ); ?>
        </div>
    </div>
</div>
