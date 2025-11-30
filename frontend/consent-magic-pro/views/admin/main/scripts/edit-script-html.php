<?php

namespace ConsentMagicPro;

// If this file is called directly, abort.
if ( !defined( 'WPINC' ) ) {
	die;
}

$options = get_cookies_terms_list();

?>
<div class="container">
    <form method="post"
          action="<?php echo esc_url( get_admin_url() . 'admin.php?page=consent-magic&tab=cs-script-blocking' ); ?>"
          id="cs_script_form"
          class="cm-settings-form">
		<?php
		// Set nonce:
		if ( function_exists( 'wp_create_nonce' ) ) {
			$nonce_value = wp_create_nonce( 'cs-update-' . CMPRO_SETTINGS_FIELD );
			echo '<input type="hidden" id="cm_nonce" name="_wpnonce" value="' . esc_attr( $nonce_value ) . '">';
		}
        ?>
        <input type="hidden" name="cs_update_action" value="update_script_form" id="cs_update_action"/>
        <input type="hidden" name="cs[consent-magic][cs_script_id]" value="<?php echo esc_attr( $id ); ?>"
               id="cs_script_id"/>
        <div class="card card-static card-style3">
            <div class="card-body">
                <div class="gap-24">
                    <div class="list-item">
                        <h4 class="font-semibold mb-4">
							<?php esc_html_e( 'Name your script', 'consent-magic' ); ?>:</h4>

                        <input type="text" name="cs[consent-magic][cs_post_name]" id="cs_post_name"
                               class="input-full"
                               value="<?php echo esc_attr( get_the_title( $id ) ); ?>" disabled="disabled"/>
                    </div>

                    <div class="list-item">
                        <h4 class="font-semibold mb-4"><?php esc_html_e( 'Script category', 'consent-magic' ); ?></h4>
						<?php renderSelectInput( 'cs_' . $id . '_script_cat', $options, true ); ?>
                    </div>

                    <div class="line"></div>

                    <div class="list-item gap-24">
                        <h4 class="primary-heading-type2 mb-4"><?php esc_html_e( 'Js Needle', 'consent-magic' ); ?></h4>
                        <p class="text-gray"><?php esc_html_e( "Into this field you can add urls which are placed in Script Body or in Attribute scr. You also can add JS parameters, which are placed in Script Body.", 'consent-magic' ); ?></p>

                        <div>
							<?php render_multi_select_input( 'cs_default_script_js_heedle', explode( ",", get_post_meta( $id, 'cs_default_script_js_heedle', true ) ), false, 'cs_multi_select_tags', $id ); ?>
                            <p class="text-description text-small mt-4"><?php esc_html_e( 'For example: GoogleAnalyticsObject, __gaTracker, window.ga=window.ga, www.google-analytics.com/analytics.js', 'consent-magic' ); ?></p>
                        </div>
                    </div>

                    <div class="line"></div>

                    <div class="list-item script-description-inner gap-24">
                        <h4 class="primary-heading-type2 mb-4">
							<?php esc_html_e( 'Your script description', 'consent-magic' ); ?>:
                        </h4>

						<?php renderSimpleWpEditor( 'cs_default_script_desc', 'js-control-element', true, $id ); ?>
                    </div>
                </div>
            </div>
        </div>
    </form>
</div>