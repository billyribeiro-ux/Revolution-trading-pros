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
        <input type="hidden" name="cs_update_action" value="add_script_form" id="cs_update_action"/>

        <div class="card card-static card-style3">
            <div class="card-body">
                <div class="gap-24">
                    <div class="list-item">
                        <h4 class="font-semibold mb-4">
							<?php esc_html_e( 'Name your script', 'consent-magic' ); ?>:</h4>

						<?php renderTextInput( 'cs_default_script_name', null, null, false, '', 'Name your script', 'required' ); ?>
                    </div>

                    <div class="list-item">
                        <h4 class="font-semibold mb-4"><?php esc_html_e( 'Script category', 'consent-magic' ); ?></h4>

                        <div class="select-wrap select-standard-wrap">
                            <select class="form-control-sm" id="add_new_script_select"
                                    name="cs[consent-magic][add_new_script_select]" autocomplete="off"
                                    style="width: 100%;">
								<?php foreach ( $options as $option_key => $option_value ) : ?>
                                    <option value="<?php echo esc_attr( $option_key ); ?>"><?php echo esc_attr( $option_value ); ?></option>
								<?php endforeach; ?>
                            </select>
                        </div>
                    </div>

                    <div class="line"></div>

                    <div class="list-item gap-24">
                        <h4 class="primary-heading-type2 mb-4"><?php esc_html_e( 'Js Needle', 'consent-magic' ); ?></h4>
                        <p class="text-gray"><?php esc_html_e( "Into this field you can add urls which are placed in Script Body or in Attribute scr. You also can add JS parameters, which are placed in Script Body.", 'consent-magic' ); ?></p>

                        <div>
							<?php render_multi_select_input( 'cs_default_script_js_heedle', array(), false, 'cs_multi_select_tags' ); ?>
                            <p class="text-description text-small mt-4"><?php esc_html_e( 'For example: GoogleAnalyticsObject, __gaTracker, window.ga=window.ga, www.google-analytics.com/analytics.js', 'consent-magic' ); ?></p>
                        </div>
                    </div>

                    <div class="line"></div>

                    <div class="list-item script-description-inner gap-24">
                        <h4 class="primary-heading-type2 mb-4">
							<?php esc_html_e( 'Your script description', 'consent-magic' ); ?>:
                        </h4>

						<?php renderSimpleWpEditor( 'cs_default_script_desc', 'js-control-element' ); ?>
                    </div>
                </div>
            </div>
        </div>
    </form>
</div>