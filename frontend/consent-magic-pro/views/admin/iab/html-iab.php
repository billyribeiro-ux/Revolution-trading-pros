<?php

namespace ConsentMagicPro;

// If this file is called directly, abort.
if ( !defined( 'WPINC' ) ) {
	die;
}

?>
<div class="container">
    <form method="post" action="<?php echo esc_url( get_admin_url( null, 'admin.php?page=cs-iab' ) ); ?>"
          id="cs_settings_form" class="cm-settings-form">
        <input type="hidden" name="cs_update_action" value="update_admin_settings_form" id="cs_update_action"/>
		<?php
		// Set nonce:
		if ( function_exists( 'wp_create_nonce' ) ) {
			$nonce_value = wp_create_nonce( 'cs-update-' . CMPRO_SETTINGS_FIELD );
			echo '<input type="hidden" id="cm_nonce" name="_wpnonce" value="' . esc_attr( $nonce_value ) . '">';
		}
        ?>

        <div class="cs-tab-content">
			<?php
			//save button
			include_once CMPRO_PLUGIN_VIEWS_PATH . "admin/iab/general-iab-html.php";
			?>
        </div>
    </form>
</div>