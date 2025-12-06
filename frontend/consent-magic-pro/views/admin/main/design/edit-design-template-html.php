<?php

namespace ConsentMagicPro;

// If this file is called directly, abort.
if ( !defined( 'WPINC' ) ) {
	die;
}

if ( isset( $_GET[ 'design' ] ) && $_GET[ 'design' ] == 'cs-multi-step-design' ) {
	$design_type = 'multi';
} else {
	$design_type = 'single';
}

$border_style    = ConsentMagic()->getOption( 'cs_border_style_list' );
$sizes_array     = get_intermediate_image_sizes();
$img_sizes_array = array();

foreach ( $sizes_array as $key => $val ) {
	$img_sizes_array[ $val ] = $val;
}

?>
<div class="container design-template-wrap">
    <form method="post"
          action="<?php echo esc_url( get_admin_url() . "admin.php?page=consent-magic&tab=cs-$design_type-step-design" ); ?>"
          id="cs_design_form"
          class="cm-settings-form">
		<?php
		// Set nonce:
		if ( function_exists( 'wp_create_nonce' ) ) {
			$nonce_value = wp_create_nonce( 'cs-update-' . CMPRO_SETTINGS_FIELD );
			echo '<input type="hidden" id="cm_nonce" name="_wpnonce" value="' . esc_attr( $nonce_value ) . '">';
		}
        ?>
        <input type="hidden" name="cs_update_action" value="update_design_template_form" id="cs_update_action"/>
        <input type="hidden" name="cs[consent-magic][cs_template_id]" value="<?php echo esc_attr( $id ); ?>"
               id="cs_template_id"/>

        <div class="card card-static card-style3">
            <div class="card-body">
                <div class="gap-24 design-wrap">
                    <div class="list-item">
                        <h4 class="font-semibold-type2 mb-4">
							<?php esc_html_e( 'Name your template', 'consent-magic' ); ?>:</h4>

                        <input type="text" name="cs[consent-magic][cs_post_name]" id="cs_post_name"
                               class="form-control js-control-element input-full"
                               value="<?php echo esc_attr( get_the_title( $id ) ); ?>"/>
                    </div>

                    <div class="line"></div>
                    <h3 class="primary-heading"><?php esc_html_e( 'Background', 'consent-magic' ); ?></h3>

                    <div class="list-item">
                        <h4 class="font-semibold-type2 mb-4"><?php esc_html_e( 'Backend color', 'consent-magic' ); ?></h4>
						<?php renderColorPickerInput( 'cs_backend_color', $id ); ?>
                    </div>

                    <div class="line"></div>
                    <h3 class="primary-heading"><?php esc_html_e( 'Border', 'consent-magic' ); ?></h3>

                    <div class="list-item row-design-settings">
                        <div class="s-w-100">
                            <h4 class="font-semibold-type2 mb-4">
								<?php esc_html_e( 'Border style', 'consent-magic' ); ?>:
                            </h4>
							<?php renderSelectInputMeta( 'cs_border_style', $id, $border_style, true, true ); ?>
                        </div>

                        <div class="d-flex align-items-center">
                            <label class="font-semibold-type2 mr-16">
								<?php esc_html_e( 'Border weight', 'consent-magic' ); ?>:
                            </label>
							<?php render_number_input( 'cs_border_weight', '', false, false, true, $id ); ?>
                        </div>

                        <div>
                            <h4 class="font-semibold-type2 mb-4">
								<?php esc_html_e( 'Border color', 'consent-magic' ); ?>:
                            </h4>
							<?php renderColorPickerInput( 'cs_border_color', $id ); ?>
                        </div>
                    </div>

                    <div class="line"></div>
                    <h3 class="primary-heading"><?php esc_html_e( 'Text', 'consent-magic' ); ?></h3>

                    <div class="list-item row-design-settings">
                        <div>
                            <h4 class="font-semibold-type2 mb-4">
								<?php esc_html_e( 'Text color', 'consent-magic' ); ?>:
                            </h4>
							<?php renderColorPickerInput( 'cs_text_color', $id ); ?>
                        </div>

                        <div>
                            <h4 class="font-semibold-type2 mb-4">
								<?php esc_html_e( 'Link\'s color', 'consent-magic' ); ?>:
                            </h4>
							<?php renderColorPickerInput( 'cs_links_color', $id ); ?>
                        </div>

                        <div>
                            <h4 class="font-semibold-type2 mb-4">
								<?php esc_html_e( 'Title\'s text color', 'consent-magic' ); ?>:
                            </h4>
							<?php renderColorPickerInput( 'cs_titles_text_color', $id ); ?>
                        </div>

                        <div>
                            <h4 class="font-semibold-type2 mb-4">
								<?php esc_html_e( 'Subtitle\'s text color', 'consent-magic' ); ?>:
                            </h4>
							<?php renderColorPickerInput( 'cs_subtitles_text_color', $id ); ?>
                        </div>

                    </div>

                    <div class="line"></div>
                    <h3 class="primary-heading"><?php esc_html_e( 'Buttons', 'consent-magic' ); ?></h3>

                    <div class="list-item row-design-settings pb-24">
                        <div>
                            <h4 class="font-semibold-type2 mb-4">
								<?php esc_html_e( 'Accept all button\'s background', 'consent-magic' ); ?>:
                            </h4>
							<?php renderColorPickerInput( 'cs_accept_all_buttons_bg', $id ); ?>
                        </div>

                        <div>
                            <h4 class="font-semibold-type2 mb-4">
								<?php esc_html_e( 'Accept all button\'s text color', 'consent-magic' ); ?>:
                            </h4>
							<?php renderColorPickerInput( 'cs_accept_all_buttons_text_color', $id ); ?>
                        </div>
                    </div>

                    <div class="list-item row-design-settings pb-24">
                        <div>
                            <h4 class="font-semibold-type2 mb-4">
								<?php esc_html_e( 'Custom button button\'s background', 'consent-magic' ); ?>:
                            </h4>
							<?php renderColorPickerInput( 'cs_custom_button_buttons_bg', $id ); ?>
                        </div>

                        <div>
                            <h4 class="font-semibold-type2 mb-4">
								<?php esc_html_e( 'Custom button button\'s text color', 'consent-magic' ); ?>:
                            </h4>
							<?php renderColorPickerInput( 'cs_custom_button_buttons_text_color', $id ); ?>
                        </div>
                    </div>

                    <div class="list-item row-design-settings pb-24">
                        <div>
                            <h4 class="font-semibold-type2 mb-4">
								<?php esc_html_e( 'Deny all button\'s background', 'consent-magic' ); ?>:
                            </h4>
							<?php renderColorPickerInput( 'cs_deny_all_buttons_bg', $id ); ?>
                        </div>

                        <div>
                            <h4 class="font-semibold-type2 mb-4">
								<?php esc_html_e( 'Deny all button\'s text color', 'consent-magic' ); ?>:
                            </h4>
							<?php renderColorPickerInput( 'cs_deny_all_buttons_text_color', $id ); ?>
                        </div>
                    </div>

                    <div class="list-item row-design-settings pb-24">
                        <div>
                            <h4 class="font-semibold-type2 mb-4">
								<?php esc_html_e( 'Options button\'s background', 'consent-magic' ); ?>:
                            </h4>
							<?php renderColorPickerInput( 'cs_options_buttons_bg', $id ); ?>
                        </div>

                        <div>
                            <h4 class="font-semibold-type2 mb-4">
								<?php esc_html_e( 'Options button\'s text color', 'consent-magic' ); ?>:
                            </h4>
							<?php renderColorPickerInput( 'cs_options_buttons_text_color', $id ); ?>
                        </div>
                    </div>

                    <div class="list-item row-design-settings pb-24">
                        <div>
                            <h4 class="font-semibold-type2 mb-4">
								<?php esc_html_e( 'Confirm choices button\'s background', 'consent-magic' ); ?>:
                            </h4>
							<?php renderColorPickerInput( 'cs_confirm_buttons_bg', $id ); ?>
                        </div>

                        <div>
                            <h4 class="font-semibold-type2 mb-4">
								<?php esc_html_e( 'Confirm choices button\'s text color', 'consent-magic' ); ?>:
                            </h4>
							<?php renderColorPickerInput( 'cs_confirm_buttons_text_color', $id ); ?>
                        </div>
                    </div>

                    <div class="list-item row-design-settings pb-24">
                        <div>
                            <h4 class="font-semibold-type2 mb-4">
								<?php esc_html_e( 'Sticky background', 'consent-magic' ); ?>:
                            </h4>
							<?php renderColorPickerInput( 'cs_sticky_bg', $id ); ?>
                        </div>

                        <div>
                            <h4 class="font-semibold-type2 mb-4">
								<?php esc_html_e( 'Sticky link color', 'consent-magic' ); ?>:
                            </h4>
							<?php renderColorPickerInput( 'cs_sticky_link_color', $id ); ?>
                        </div>
                    </div>

                    <div class="list-item row-design-settings">
                        <div>
                            <h4 class="font-semibold-type2 mb-4">
								<?php esc_html_e( 'Tab button background', 'consent-magic' ); ?>:
                            </h4>
							<?php renderColorPickerInput( 'cs_tab_buttons_bg', $id ); ?>
                        </div>

                        <div>
                            <h4 class="font-semibold-type2 mb-4">
								<?php esc_html_e( 'Tab button text color', 'consent-magic' ); ?>:
                            </h4>
							<?php renderColorPickerInput( 'cs_tab_buttons_text_color', $id ); ?>
                        </div>
                    </div>

                    <div class="line"></div>
                    <h3 class="primary-heading"><?php esc_html_e( 'Logo', 'consent-magic' ); ?></h3>

                    <div class="list-item row-design-settings design-logo">
						<?php renderUploadBtn( $id, 'cs_logo' ); ?>
                    </div>

                    <div class="list-item row-design-settings">
                        <div class="s-w-100">
                            <h4 class="font-semibold-type2 mb-4">
								<?php esc_html_e( 'Logo position', 'consent-magic' ); ?>:
                            </h4>
							<?php renderSelectInputMeta( 'cs_position_vertical_list', $id, ConsentMagic()->getOption( 'cs_position_vertical_list' ), true, true ); ?>
                        </div>

						<?php renderSelectInputMeta( 'cs_position_horizontal_list', $id, ConsentMagic()->getOption( 'cs_position_horizontal_list' ), true, true ); ?>

                        <div class="s-w-100">
                            <h4 class="font-semibold-type2 mb-4">
								<?php esc_html_e( 'Logo size', 'consent-magic' ); ?>:
                            </h4>
							<?php renderSelectInputMeta( 'cs_logo_size', $id, $img_sizes_array, true, true ); ?>
                        </div>
                    </div>

                    <div class="line"></div>
                    <h3 class="primary-heading"><?php esc_html_e( 'Additional', 'consent-magic' ); ?></h3>

                    <div class="list-item row-design-settings">
                        <div>
                            <h4 class="font-semibold-type2 mb-4">
								<?php esc_html_e( 'Category color', 'consent-magic' ); ?>:
                            </h4>
							<?php renderColorPickerInput( 'cs_cat_color', $id ); ?>
                        </div>

                        <div>
                            <h4 class="font-semibold-type2 mb-4">
								<?php esc_html_e( 'Active toggle color', 'consent-magic' ); ?>:
                            </h4>
							<?php renderColorPickerInput( 'cs_active_toggle_color', $id ); ?>
                        </div>

                        <div>
                            <h4 class="font-semibold-type2 mb-4">
								<?php esc_html_e( 'Active toggle text color', 'consent-magic' ); ?>:
                            </h4>
							<?php renderColorPickerInput( 'cs_active_toggle_text_color', $id ); ?>
                        </div>

                        <div>
                            <h4 class="font-semibold-type2 mb-4">
								<?php esc_html_e( 'Background color for text', 'consent-magic' ); ?>:
                            </h4>
							<?php renderColorPickerInput( 'cs_text_block_bg', $id ); ?>
                        </div>

                        <div>
                            <h4 class="font-semibold-type2 mb-4">
								<?php esc_html_e( 'Shortcode text color', 'consent-magic' ); ?>:
                            </h4>
							<?php renderColorPickerInput( 'cs_shortcodes_text_color', $id ); ?>
                        </div>
                    </div>
                </div>
            </div>
        </div>
    </form>
</div>
