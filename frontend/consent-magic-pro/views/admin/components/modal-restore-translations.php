<?php

namespace ConsentMagicPro;

// If this file is called directly, abort.
if ( !defined( 'WPINC' ) ) {
	die;
}

?>

<div class="cm-modal cs_restore_translations_wrap">
    <div class="cm-modal-content-wrap">
        <div class="cm-modal-content">
            <p class="mb-24 fw-500">
				<?php esc_html_e( 'This will restore the default translations for all front-end text, and default categories. Custom text translations and extra categories translations will not be affected.', 'consent-magic' ); ?>
            </p>
            <div class="modal-buttons">
                <button type="button" class="btn btn-red btn-primary-type2 restore_translations_cancel"><i
                            class="icon-restore"></i> <?php esc_html_e( 'Cancel', 'consent-magic' ); ?>
                </button>
                <button type="button"
                        class="btn btn-primary btn-primary-type2 restore_translations_ok"><i
                            class="icon-renew"></i> <?php esc_html_e( 'Delete and Restore', 'consent-magic' ); ?>
                </button>
            </div>
        </div>
    </div>
</div>