<?php

namespace ConsentMagicPro;

// If this file is called directly, abort.
if ( !defined( 'WPINC' ) ) {
	die;
}

?>

<div class="cm-modal cs_restart_setup_wrap">
    <div class="cm-modal-content-wrap">
        <div class="cm-modal-content">
            <p class="mb-24 fw-500">
				<?php
				if ( ConsentMagic()->getOption( 'cs_deactivation_db_clear' ) == 1 ) {
					echo esc_html__( 'Warning!', 'consent-magic' ) . '<br>' . esc_html__( 'You enabled option "Remove all data when deactivating the plugin". With this option all settings and user data will reset to default. Are you sure you want to restart the setup flow?', 'consent-magic' );
				} else {
					esc_html_e( 'Are you sure to restart the setup flow, and reset all predefined settings to default? User settings and data will not be affected.', 'consent-magic' );
				}
				?>
            </p>
            <div class="modal-buttons">
                <button type="button" class="btn btn-red btn-primary-type2 cs_restart_setup_cancel"><i
                            class="icon-restore"></i> <?php esc_html_e( 'Cancel', 'consent-magic' ); ?>
                </button>
                <button type="button"
                        class="btn btn-primary btn-primary-type2 cs_restart_setup"><i
                            class="icon-renew"></i> <?php esc_html_e( 'Confirm', 'consent-magic' ); ?>
                </button>
            </div>
        </div>
    </div>
</div>