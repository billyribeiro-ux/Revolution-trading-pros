<?php

namespace ConsentMagicPro;

// If this file is called directly, abort.
if ( !defined( 'WPINC' ) ) {
	die;
}

?>
<div class="cs-tab-content cm-proofconsent" data-id="<?php echo esc_attr( $target_id ); ?>">
    <input type="hidden" name="cm-delete-proof-consent" id="cm-delete-proof-consent" value="">
    <div class="cm-table-wrap">
        <table id="proof_consent_data" class="display cm-table">
            <thead>
            <tr>
                <th><?php esc_html_e( 'IP', "consent-magic" ); ?></th>
                <th><?php esc_html_e( 'Time', "consent-magic" ); ?></th>
                <th><?php esc_html_e( 'UUID', "consent-magic" ); ?></th>
                <th><?php esc_html_e( 'Consent', "consent-magic" ); ?></th>
                <th></th>
            </tr>
            </thead>
            <tfoot>
            <tr>
                <th><?php esc_html_e( 'IP', "consent-magic" ); ?></th>
                <th><?php esc_html_e( 'Time', "consent-magic" ); ?></th>
                <th><?php esc_html_e( 'UUID', "consent-magic" ); ?></th>
                <th><?php esc_html_e( 'Consent', "consent-magic" ); ?></th>
                <th></th>
            </tr>
            </tfoot>
        </table>
    </div>

    <div class="export-info">
        <div class="popup-message mt-24">
			<?php render_info_message( '' ); ?>
        </div>
    </div>

	<?php include CMPRO_PLUGIN_VIEWS_PATH . "admin/components/modal-delete-consent.php"; ?>
</div>