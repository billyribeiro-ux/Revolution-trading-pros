<?php

namespace ConsentMagicPro;

// If this file is called directly, abort.
if ( !defined( 'WPINC' ) ) {
	die;
}
$tab_head_arr = array(
	'cs-settings' => __( 'Settings', 'consent-magic' ),
	'cs-records'  => __( 'Records', 'consent-magic' )
);

?>

<div class="container">
    <div class="cards-wrapper cards-wrapper-style2">
        <div class="card card-static card-style3">
            <div class="card-header card-header-with-tabs cs_tab_header">
                <ul class="cs_tab">
					<?php foreach ( $tab_head_arr as $key => $item ) :
						if ( $key == getCurrentAdminTab( 'cs-settings' ) ) {
							$class = 'active';
						} else {
							$class = '';
						}
						?>
                        <li class="<?php echo esc_attr( $class ); ?>"><a
                                    href="<?php echo esc_url( get_admin_url( null, 'admin.php?page=cs-proof-consent&tab=' . $key ) ); ?>"><?php echo esc_html( $item ); ?></a>
                        </li>
					<?php endforeach; ?>
                </ul>
            </div>

            <div class="card-body cs_tab_container">
				<?php
				//inside the settings form
				$setting_views_a = array(
					'cs-settings' => 'cs-settings.php',
					'cs-records'  => 'cs-records.php',
				);
				?>
                <form method="post"
                      action="<?php echo esc_url(get_admin_url( null, 'admin.php?page=cs-proof-consent&tab=' . getCurrentAdminTab( 'cs-settings' ) ) ); ?>"
                      id="cs_settings_form" class="cm-settings-form">
                    <input type="hidden" name="cs_update_action" value="update_admin_settings_form"
                           id="cs_update_action"/>

					<?php
					// Set nonce:
					if ( function_exists( 'wp_create_nonce' ) ) {
						$nonce_value = wp_create_nonce( 'cs-update-' . CMPRO_SETTINGS_FIELD );
						echo '<input type="hidden" id="cm_nonce" name="_wpnonce" value="' . esc_attr( $nonce_value ) . '">';
					}

					foreach ( $setting_views_a as $target_id => $value ) {
						$settings_view = CMPRO_PLUGIN_VIEWS_PATH . '/admin/proofconsent/' . $value;

						if ( file_exists( $settings_view ) && getCurrentAdminTab( 'cs-settings' ) == $target_id ) {
							include $settings_view;
						}
					}
					?>
                </form>
            </div>
        </div>
    </div>
</div>
