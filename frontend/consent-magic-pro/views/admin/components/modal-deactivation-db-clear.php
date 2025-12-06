<?php

namespace ConsentMagicPro;

// If this file is called directly, abort.
if ( !defined( 'WPINC' ) ) {
	die;
}

?>

<div class="cm-modal cs_deactivation_db_clear_wrap">
    <div class="cm-modal-content-wrap">
        <div class="cm-modal-content">
            <p class="mb-24 fw-500">
				<?php esc_html_e( 'Caution: if you enable this option, all the data will be deleted when the plugin is deactivated', 'consent-magic' ); ?>
            </p>
            <div class="modal-buttons">
                <button type="button" class="btn btn-red btn-primary-type2 cs_deactivation_db_clear_wrap_cancel"><i
                            class="icon-restore"></i> <?php esc_html_e( 'Cancel', 'consent-magic' ); ?>
                </button>
                <button type="button"
                        class="btn btn-primary btn-primary-type2 cs_deactivation_db_clear_ok"><i
                            class="icon-renew"></i> <?php esc_html_e( 'Confirm', 'consent-magic' ); ?>
                </button>
            </div>
        </div>
    </div>
</div>