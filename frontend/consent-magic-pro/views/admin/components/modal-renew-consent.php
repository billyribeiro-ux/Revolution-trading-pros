<?php

namespace ConsentMagicPro;

// If this file is called directly, abort.
if ( !defined( 'WPINC' ) ) {
	die;
}

?>

<div class="cm-modal cs_renew_consent_wrap">
    <div class="cm-modal-content-wrap">
        <div class="cm-modal-content">
            <p class="mb-24 fw-500">
				<?php esc_html_e( 'Are you sure you want to renew consent?', 'consent-magic' ); ?>
            </p>
            <div class="modal-buttons">
                <button type="button" class="btn btn-red btn-primary-type2 cs_renew_consent_cancel"><i
                            class="icon-restore"></i> <?php esc_html_e( 'Cancel', 'consent-magic' ); ?>
                </button>
                <button type="button"
                        class="btn btn-primary btn-primary-type2 cs_renew_consent"><i
                            class="icon-renew"></i> <?php esc_html_e( 'Yes Renew Consent', 'consent-magic' ); ?>
                </button>
            </div>
        </div>
    </div>
</div>