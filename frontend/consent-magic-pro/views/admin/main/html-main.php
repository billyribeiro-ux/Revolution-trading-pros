<?php

namespace ConsentMagicPro;

// If this file is called directly, abort.
if ( !defined( 'WPINC' ) ) {
	die;
}

if ( isset( $_GET[ 'primary_rule_id' ] ) ) {
	require_once CMPRO_PLUGIN_VIEWS_PATH . 'admin/main/primary-rules/edit-primary-rule-html.php';
} elseif ( isset( $_GET[ 'new_rule' ] ) ) {
	require_once CMPRO_PLUGIN_VIEWS_PATH . 'admin/main/primary-rules/add-new-primary-rule-html.php';
} elseif ( isset( $_GET[ 'new_script' ] ) || isset( $_GET[ 'script_id' ] ) ) {
	if ( isset( $_GET[ 'script_id' ] ) ) {
		$id = sanitize_text_field( $_GET[ 'script_id' ] );
		require_once CMPRO_PLUGIN_VIEWS_PATH . 'admin/main/scripts/edit-script-html.php';
	} else {
		require_once CMPRO_PLUGIN_VIEWS_PATH . 'admin/main/scripts/add-script-html.php';
	}
} elseif ( isset( $_GET[ 'design_template' ] ) ) {

	if ( $_GET[ 'design_template' ] == 'add' ) {
		require_once CMPRO_PLUGIN_VIEWS_PATH . 'admin/main/design/add-design-template-html.php';
	} elseif ( $_GET[ 'design_template' ] == 'edit' ) {
		$id = isset( $_GET[ 'template_id' ] ) ? sanitize_text_field( $_GET[ 'template_id' ] ) : '';
		require_once CMPRO_PLUGIN_VIEWS_PATH . 'admin/main/design/edit-design-template-html.php';
	}
} else {

	//inside the settings form
	$setting_views_a = array(
		'cs-general'            => 'admin-settings-general.php',
		'cs-script-blocking'    => 'admin-settings-script-blocking.php',
		'cs-policy-gen'         => 'admin-settings-policy-gen.php',
		'cs-text'               => 'admin-settings-text.php',
		'cs-multi-step-design'  => 'admin-settings-design.php',
		'cs-single-step-design' => 'admin-settings-design-single.php',
	);

	$admin_url = buildAdminUrl( 'consent-magic', getCurrentAdminTab() );

	?>
    <div class="container">
        <form method="post" action="<?php echo esc_url( $admin_url ); ?>"
              id="cs_settings_form" class="cm-settings-form">
            <input type="hidden" name="cs_update_action" value="update_admin_settings_form"/>

			<?php
			// Set nonce:
			if ( function_exists( 'wp_create_nonce' ) ) {
				$nonce_value = wp_create_nonce( 'cs-update-' . CMPRO_SETTINGS_FIELD );
				echo '<input type="hidden" id="cm_nonce" name="_wpnonce" value="' . esc_attr( $nonce_value ) . '">';
			}

			foreach ( $setting_views_a as $target_id => $value ) {
				$settings_view = CMPRO_PLUGIN_VIEWS_PATH . '/admin/main/' . $value;

				if ( file_exists( $settings_view ) && getCurrentAdminTab() == $target_id ) {
					require_once $settings_view;
				}
			}
			?>
        </form>
    </div>
	<?php
}
