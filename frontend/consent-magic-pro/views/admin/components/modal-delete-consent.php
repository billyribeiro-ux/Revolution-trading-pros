<?php

namespace ConsentMagicPro;

// If this file is called directly, abort.
if ( !defined( 'WPINC' ) ) {
	die;
}

?>

<div class="cm-modal cs_delete_proof_consent_wrap">
    <div class="cm-modal-content-wrap">
        <div class="cm-modal-content">
            <p class="mb-24 fw-500"><?php esc_html_e( 'You are about to delete all stored consent!', 'consent-magic' ); ?></p>
            <div class="modal-buttons">
                <button type="button" class="btn btn-red btn-primary-type2 delete_proof_consent_close">
					<?php esc_html_e( 'Close', 'consent-magic' ); ?>
                </button>
                <button type="button"
                        class="btn btn-primary btn-primary-type2 export_proof_consent">
					<?php esc_html_e( 'Download consent CSV and Delete', 'consent-magic' ); ?>
                </button>

                <button type="button"
                        class="btn btn-primary btn-primary-type2 delete_proof_consent">
					<?php esc_html_e( 'Just delete', 'consent-magic' ); ?>
                </button>
            </div>

            <div class="popup-message">
				<?php render_info_message( '' ); ?>
            </div>
        </div>
    </div>
</div>