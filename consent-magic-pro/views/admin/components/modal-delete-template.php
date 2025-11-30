<?php

namespace ConsentMagicPro;

// If this file is called directly, abort.
if ( !defined( 'WPINC' ) ) {
	die;
}

?>

<div class="cm-modal cs_restore_defaults_design_wrap">
    <div class="cm-modal-content-wrap">
        <div class="cm-modal-content">
            <p class="mb-24 fw-500"><?php esc_html_e( 'Are you sure you want to restore design defaults?', 'consent-magic' ); ?></p>
            <div class="modal-buttons">
                <button type="button" class="btn btn-red btn-primary-type2 cs_restore_defaults_design_cancel"><i
                            class="icon-restore"></i> <?php esc_html_e( 'Cancel', 'consent-magic' ); ?>
                </button>
                <button type="button"
                        class="btn btn-primary btn-primary-type2 cs_restore_defaults_design"><i
                            class="icon-renew"></i> <?php esc_html_e( 'OK', 'consent-magic' ); ?>
                </button>
            </div>
        </div>
    </div>
</div>
