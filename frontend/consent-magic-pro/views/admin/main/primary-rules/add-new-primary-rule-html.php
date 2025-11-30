<?php

namespace ConsentMagicPro;

// If this file is called directly, abort.
if ( !defined( 'WPINC' ) ) {
	die;
}

$unassigned    = get_term_by( 'slug', 'unassigned', 'cs-cookies-category' );
$unassigned_id = $unassigned->term_id;
$options       = get_cookies_terms_objects( null, false, $unassigned_id );


?>
<div class="container">
    <form method="post"
          action="<?php echo esc_url( get_admin_url() . 'admin.php?page=consent-magic' ); ?>"
          id="cs_new_rule_form"
          class="cm-settings-form">

        <input type="hidden" name="cs_update_action" value="add_rule_form" id="cs_update_action"/>
		<?php
		// Set nonce:
		if ( function_exists( 'wp_create_nonce' ) ) {
			$nonce_value = wp_create_nonce( 'cs-update-' . CMPRO_SETTINGS_FIELD );
			echo '<input type="hidden" id="cm_nonce" name="_wpnonce" value="' . esc_attr( $nonce_value ) . '">';
		}
        ?>
        <div class="card card-static card-style3">
            <div class="card-body">
                <div class="gap-24">
                    <div class="list-item">
                        <h4 class="font-semibold-type2 mb-4">
							<?php esc_html_e( 'Name your Rule', 'consent-magic' ); ?>:</h4>
						<?php renderTextInput( 'cs_default_rule_name', null, null, false, true, '', 'js-control-element', 'required' ); ?>
                    </div>

                    <div class="list-item">
                        <div class="d-flex align-items-center">
							<?php render_switcher_input( '_cs_enable_rule', false, true ); ?>
                            <h4 class="secondary-heading switcher-label"><?php esc_html_e( 'Enable Rule', 'consent-magic' ); ?></h4>
                        </div>
                    </div>

                    <div class="list-item">
                        <h4 class="font-semibold-type2 mb-4"><?php esc_html_e( 'Locations', 'consent-magic' ); ?>:</h4>
                        <div class="cs_location">
							<?php renderSelectLocation( '_cs_general_target', null, true ); ?>
                        </div>
                    </div>

                    <div class="list-item">
                        <div class="d-flex align-items-center">
							<?php render_switcher_input( '_cs_no_ip_rule', false, true ); ?>
                            <h4 class="secondary-heading switcher-label"><?php esc_html_e( "Show this rule when we can't retrieve the IP", 'consent-magic' ); ?></h4>
                        </div>
                    </div>

                    <div class="list-item">
                        <h4 class="font-semibold-type2 mb-4"><?php esc_html_e( 'Consent type:', 'consent-magic' ); ?></h4>

                        <div>
							<?php renderSelectInputMeta( '_cs_type', null, renderCSTypeOptions(), true ); ?>
                        </div>
                    </div>

                    <div class="rule-options-list gap-24">
                        <div class="list-item" style="display: none;">
                            <div class="line mb-24"></div>

                            <div class="d-flex align-items-center mb-4 m-mb-8">
								<?php if ( !CS_Meta_LDU()->check_pys_version() ) : ?>
                                    <div class="dimmy_checkbox d-flex align-items-center">
										<?php render_switcher_input( '_cs_use_meta_ldu', false, true, null, false, true ); ?>
                                        <h4 class="secondary-heading switcher-label"><?php esc_html_e( 'For Meta, use the Limited Data Use (LDU) flag instead of blocking the browser and API events', 'consent-magic' ); ?></h4>
                                    </div>
								<?php else : ?>
                                    <div class="d-flex align-items-center">
										<?php render_switcher_input( '_cs_use_meta_ldu', false, true ); ?>
                                        <h4 class="secondary-heading switcher-label"><?php esc_html_e( 'For Meta, use the Limited Data Use (LDU) flag instead of blocking the browser and API events', 'consent-magic' ); ?></h4>
                                    </div>
								<?php endif; ?>
                            </div>

                            <p class="text-gray"><?php esc_html_e( 'It works ONLY when the pixel and API events are configured with the PixelYourSite plugin.', 'consent-magic' ); ?>
                                &nbsp;<?php echo wp_kses_post( sprintf( __( 'Details %shere%s.', 'consent-magic' ), '<a href="https://www.facebook.com/business/help/1151133471911882/" target="_blank" class="link">', '</a>' ) ); ?></p>

							<?php if ( !CS_Meta_LDU()->check_pys_version() ) : ?>
                                <div class="mt-4 m-mt-8">
									<?php render_warning_message( __( 'This option needs PixelYourSite Professional version 11.2.1 or more, or PixelYourSite free version 10.1.0 or more.', 'consent-magic' ) ); ?>
                                </div>
							<?php endif; ?>
                        </div>

                        <div class="list-item" style="display: none;">
                            <div class="line cs_showing_line mb-24" style="display: none;"></div>
							<?php render_switcher_input( '_cs_google_consent_mode', false, true, null, false, false, __( 'For Google Tags, use Google Consent Mode instead of blocking the scripts', 'consent-magic' ) ); ?>
                            <p class="text-gray mt-4 m-mt-8"><?php echo wp_kses_post( sprintf( __( 'Details %shere%s.', 'consent-magic' ), '<a href="https://support.google.com/google-ads/answer/10000067?hl=en" target="_blank" class="link">', '</a>' ) ); ?></p>
                        </div>

                        <div class="list-item" style="display: none;">
                            <div class="mb-4 m-mb-8">
								<?php if ( !CS_Bing_Consent_Mode()->check_pys_version() ) : ?>
                                    <div class="dimmy_checkbox d-flex align-items-center">
										<?php render_switcher_input( '_cs_bing_consent_mode', false, true, null, false, false, __( 'For Microsoft UET Tags, use Consent Mode instead of blocking the scripts', 'consent-magic' ) ); ?>
                                    </div>
								<?php else : ?>
									<?php render_switcher_input( '_cs_bing_consent_mode', false, true, null, false, false, __( 'For Microsoft UET Tags, use Consent Mode instead of blocking the scripts', 'consent-magic' ) ); ?>
								<?php endif; ?>
                            </div>

                            <p class="text-gray"><?php esc_html_e( 'It works ONLY when the Microsoft UET tag is configured with the PixelYourSite Bing add-on.', 'consent-magic' ); ?>
                                &nbsp;<?php echo wp_kses_post( sprintf( __( 'Details %shere%s', 'consent-magic' ), '<a href="https://www.pixelyoursite.com/bing-consent-mode" target="_blank" class="link">', '</a>' ) ); ?></p>

							<?php if ( !CS_Bing_Consent_Mode()->check_pys_version() ) : ?>
                                <div class="mt-4 m-mt-8">
									<?php render_warning_message( __( 'This option needs PixelYourSite Professional version 11.3.0.3 or more, or PixelYourSite free version 10.2.0.3 or more and Bing add-on version 3.5.3.1 or more.', 'consent-magic' ) ); ?>
                                </div>
							<?php endif; ?>
                        </div>

                        <div class="list-item" style="display: none;">
                            <div class="mb-4 m-mb-8">
			                    <?php if ( !CS_Reddit_LDU()->check_pys_version() ) : ?>
                                    <div class="dimmy_checkbox d-flex align-items-center">
					                    <?php render_switcher_input( '_cs_reddit_ldu', false, true, null, false, false, __( 'For Reddit pixels, use the Limited Data Use (LDU) flag instead of blocking events.', 'consent-magic' ) ); ?>
                                    </div>
			                    <?php else : ?>
				                    <?php render_switcher_input( '_cs_reddit_ldu', false, true, null, false, false, __( 'For Reddit pixels, use the Limited Data Use (LDU) flag instead of blocking events.', 'consent-magic' ) ); ?>
			                    <?php endif; ?>
                            </div>

                            <p class="text-gray"><?php esc_html_e( 'It works ONLY when the Reddit tag is configured with the PixelYourSite Reddit add-on.', 'consent-magic' ); ?></p>

		                    <?php if ( !CS_Reddit_LDU()->check_pys_version() ) : ?>
                                <div class="mt-4 m-mt-8">
				                    <?php render_warning_message( __( 'This option needs PixelYourSite Professional version 12.2.4 or more, or PixelYourSite free version 11.2.4 or more and Reddit add-on', 'consent-magic' ) ); ?>
                                </div>
		                    <?php endif; ?>
                        </div>

                        <div class="list-item" style="display: none;">
                            <div class="line mb-24"></div>
							<?php render_switcher_input( '_cs_native_scripts', false, true, null, false, false, __( 'Enable ConsentMagic native script controls', 'consent-magic' ) ); ?>
                        </div>

                        <div class="list-item" style="display: none;">
                            <div class="gap-24">
                                <div class="line"></div>

                                <div>
									<?php render_switcher_input( '_cs_sticky', false, true, null, false, false, __( 'Sticky', 'consent-magic' ) ); ?>
                                    <p class="text-gray mt-4 m-mt-8"><?php esc_html_e( 'Show a small consent bar after consent is express.', 'consent-magic' ); ?></p>
                                </div>

                                <div class="sticky-list-container gap-24 disabled">
                                    <div class="sticky-list sub-items">
                                        <div class="mb-4 m-mb-8">
											<?php render_checkbox_input( '_cs_smart_sticky', false, true, null, false, false, __( 'Smart Sticky', 'consent-magic' ), false, 'switcher-sub-label' ); ?>
                                        </div>

                                        <p class="text-gray mb-16"><?php esc_html_e( 'Show the Sticky message only for negative consent for any of the following categories:', 'consent-magic' ); ?></p>

                                        <div class="sticky_categories categories_list">
											<?php
											if ( !empty( $options ) ) {
												foreach ( $options as $option ) {
													$ignore_category = (int) get_term_meta( $option->term_id, 'cs_ignore_this_category', true );
													?>
                                                    <div class="<?php echo !$ignore_category ? 'checkbox-item' : '' ?> dimmy_checkbox">
														<?php render_checkbox_input( '_cs_smart_sticky_' . $option->term_id, false, true, null, false, $ignore_category, $option->name_l, $ignore_category ? 0 : false, 'small-switch' ); ?>
                                                    </div>
												<?php }
											}
											?>
                                        </div>
                                    </div>

                                    <div class="sub-items">
                                        <div class="line"></div>
                                    </div>

                                    <div class="sticky-list sub-items">
                                        <div class="mb-4 m-mb-8">
											<?php render_checkbox_input( '_cs_smart_sticky_mobile', false, true, null, false, false, __( 'Smart Mobile Sticky', 'consent-magic' ), false, 'switcher-sub-label' ); ?>
                                        </div>

                                        <p class="text-gray mb-16"><?php esc_html_e( 'Show the Sticky button only for negative consent for any of the following categories:', 'consent-magic' ); ?></p>

                                        <div class="sticky_categories categories_list">
											<?php
											if ( !empty( $options ) ) {
												foreach ( $options as $option ) {
													$ignore_category = (int) get_term_meta( $option->term_id, 'cs_ignore_this_category', true );
													?>
                                                    <div class="<?php echo !$ignore_category ? 'checkbox-item' : '' ?> dimmy_checkbox">
														<?php render_checkbox_input( '_cs_smart_sticky_mobile_' . $option->term_id, false, true, null, false, $ignore_category, $option->name_l, $ignore_category ? 0 : false, 'small-switch' ); ?>
                                                    </div>
												<?php }
											}
											?>
                                        </div>
                                    </div>
                                </div>

                                <div class="line"></div>
                            </div>
                        </div>

                        <div class="list-item" style="display: none;">
                            <div class="d-flex align-items-center mb-4 m-mb-8">
								<?php render_switcher_input( '_cs_mobile_side_sticky', false, true ); ?>
                                <h4 class="secondary-heading switcher-label"><?php esc_html_e( 'Mobile side sticky button', 'consent-magic' ); ?></h4>
                            </div>

                            <p class="text-gray mb-24"><?php esc_html_e( 'Place sticky button on the right side for mobile devices not to overlap the content.', 'consent-magic' ); ?></p>
                            <div class="line"></div>
                        </div>

                        <div class="list-item" style="display: none;">
                            <div class="d-flex align-items-center">
								<?php render_switcher_input( '_cs_deny_all_btn', false, true ); ?>
                                <h4 class="secondary-heading switcher-label"><?php esc_html_e( 'Deny all button', 'consent-magic' ); ?></h4>
                            </div>
                        </div>

                        <div class="list-item" style="display: none;">
                            <div class="d-flex align-items-center">
								<?php render_switcher_input( '_cs_hide_close_btn', false, true ); ?>
                                <h4 class="secondary-heading switcher-label"><?php esc_html_e( 'Hide close button', 'consent-magic' ); ?></h4>
                            </div>

                            <p class="text-gray mt-4 m-mt-8"><?php esc_html_e( 'It will hide the close button, forcing users to express positive or negative consent.', 'consent-magic' ); ?></p>
                        </div>

                        <div class="list-item" style="display: none;">
                            <div class="d-flex align-items-center">
								<?php render_switcher_input( '_cs_deny_consent_for_close', false, true ); ?>
                                <h4 class="secondary-heading switcher-label"><?php esc_html_e( 'Don\'t show the consent prompt after click on the close button', 'consent-magic' ); ?></h4>
                            </div>

                            <p class="text-gray mt-4 m-mt-8"><?php esc_html_e( 'This action will equivalent with negative consent for all categories.', 'consent-magic' ); ?></p>
                        </div>

                        <div class="list-item cs_custom_button_block" style="display: none;">
                            <div class="d-flex align-items-center">
								<?php render_switcher_input( '_cs_custom_button', false, true ); ?>
                                <h4 class="secondary-heading switcher-label"><?php esc_html_e( 'Add custom button', 'consent-magic' ); ?></h4>
                            </div>
                            <p class="text-gray mt-4 m-mt-8"><?php echo sprintf( esc_html__( 'The default text for the custom button is "Minimum recommended". You can edit it from the %s global text settings%s, or add a custom text for this rule.', 'consent-magic' ), '<a href="' . esc_url( get_admin_url( null, 'admin.php?page=consent-magic&tab=cs-text' ) ) . '" target="_blank" class="link">', '</a>' ); ?></p>

                            <div class="cs_custom_button_block_categories sub-items mt-24 mb-24 disabled">
                                <p class="secondary-heading mb-16"><?php esc_html_e( 'The custom button will turn ON the following categories:', 'consent-magic' ); ?></p>

                                <div class="categories_list">
									<?php
									if ( !empty( $options ) ) {
										foreach ( $options as $option ) {
											$ignore_category = (int) get_term_meta( $option->term_id, 'cs_ignore_this_category', true );
											?>
                                            <div class="<?php echo $ignore_category ? 'dimmy_checkbox' : ''; ?>">
												<?php render_checkbox_input( '_cs_custom_button_' . $option->term_id, false, true, null, false, $ignore_category, $option->name_l, $ignore_category ? 0 : false, 'small-switch' ); ?>
                                            </div>
										<?php }
									}
									?>
                                </div>
                            </div>

                            <div class="line"></div>
                        </div>

                        <div>
                            <div class="list-item">
                                <div class="d-flex align-items-center mb-24">
									<?php render_switcher_input( '_cs_privacy_link', false, true, null, false, false, '', true ); ?>
                                    <h4 class="secondary-heading switcher-label"><?php esc_html_e( 'Privacy link', 'consent-magic' ); ?></h4>
                                </div>
                            </div>

                            <div class="line"></div>
                        </div>


                        <div class="list-item" style="display: none;">
							<?php render_switcher_input( '_cs_block_content', false, true, null, false, false, __( 'Block scroll', 'consent-magic' ) ); ?>
                            <p class="text-gray mt-4 m-mt-8"><?php esc_html_e( 'Block scroll until the visitor interacts with the consent message.', 'consent-magic' ); ?></p>
                        </div>

                        <div class="list-item">
                            <div class="d-flex align-items-center">
								<?php render_switcher_input( '_cs_close_on_scroll', false, true, null, false, false, '', true ); ?>
                                <h4 class="secondary-heading switcher-label"><?php esc_html_e( 'Close on scroll', 'consent-magic' ); ?></h4>
                            </div>

                            <div class="line cs_showing_line mt-24"></div>
                        </div>

                        <div class="list-item" style="display: none;">
							<?php
							if ( isFbWooActivated() && !isPYSActivated() && (int) ConsentMagic()->getOption( 'cs_fb_woo_capi_enabled' ) === 1 && (int) ConsentMagic()->getOption( 'cs_enable_site_cache' ) === 0 ): ?>
                                <span class="dimmy_checkbox">
                                    <?php render_switcher_input( '_cs_refresh_after_consent', false, true, null, false, true, __( 'Refresh page after consent', 'consent-magic' ), true ); ?>
                                </span>
                                <p class="text-gray mt-4 m-mt-8"><?php esc_html_e( 'Facebook for WooCommerce CAPI events control is ON, we need to refresh the page after the user interacts with the consent message.', 'consent-magic' ); ?></p>
							<?php else: ?>
								<?php render_switcher_input( '_cs_refresh_after_consent', false, true, null, false, false, __( 'Refresh page after consent', 'consent-magic' ) ); ?>
                                <p class="text-gray mt-4 m-mt-8"><?php esc_html_e( 'If enabled, the plugin will refresh the page when the user interacts with the consent message (accept, reject).', 'consent-magic' ); ?></p>
							<?php endif; ?>

                            <div class="line cs_showing_line mt-24" style="display:none;"></div>
                        </div>

                        <div class="list-item" style="display:none;">
							<?php render_switcher_input( '_cs_showing_rule_until_express_consent', false, true, null, false, false, __( 'Show the consent message until the visitor interacts with it', 'consent-magic' ) ); ?>

                            <div class="line mt-24"></div>
                        </div>

                        <div class="list-item" style="display: none;">
                            <div class="mb-4 m-mb-8">
								<?php render_switcher_input( '_cs_track_analytics', false, true, null, false, false, __( 'Track Analytics cookies and scripts before consent is expressed', 'consent-magic' ) ); ?>
                            </div>

                            <p class="text-gray mb-24"><?php esc_html_e( '"Necessary" cookies and scripts will work every time.', 'consent-magic' ); ?></p>
                            <p class="fw-500 mb-24"><?php esc_html_e( '"Marketing" cookies and scripts will work only after consent is given.', 'consent-magic' ); ?></p>

                            <div class="block-with-button">
								<?php $message = __( 'You have more options under', 'consent-magic' );

								render_notice_message( $message, 500 );
								?>

                                <a href="<?php echo esc_url( get_admin_url() . 'admin.php?page=consent-magic&tab=cs-script-blocking' ); ?>"
                                   class="btn btn-primary btn-primary-type2"><?php esc_html_e( 'Script Blocking', 'consent-magic' ); ?></a>
                            </div>

                            <div class="line mt-24"></div>
                        </div>

                        <div class="list-item">
							<?php render_switcher_input( '_excluded_from_consent_storing', false, true, null, false, false, __( 'Excluded from consent storing', 'consent-magic' ) ); ?>
                        </div>

                        <div class="line"></div>

                        <h4 class="secondary-heading"><?php esc_html_e( 'Content:', 'consent-magic' ); ?></h4>

                        <div>
                            <div class="rule-content-wrap">
                                <div class="list-item cs_design_type_wrap" style="display: none;">
									<?php renderSelectInputMeta( '_cs_design_type', null, renderCSDesignTypeOptions(), true, false, true ); ?>
                                </div>

                                <div class="list-item">
									<?php renderSelectInputMeta( '_cs_theme', null, get_custom_theme_list(), true ); ?>
                                </div>

                                <div class="list-item">
									<?php renderSelectInputMeta( '_cs_bars_type', null, renderCSBarTypeOptions(), true ); ?>
                                </div>

                                <div class="list-item cs_bars_position_wrap">
									<?php renderSelectInputMeta( '_cs_bars_position', null, renderCSBarPositionOptions(), true ); ?>
                                </div>
                            </div>

                            <div class="list-item cs_top_push_wrap mt-24" style="display: none;">
                                <div class="d-flex align-items-center">
									<?php render_switcher_input( '_cs_top_push', false, true ); ?>
                                    <h4 class="secondary-heading switcher-label"><?php esc_html_e( 'Top bar: push content down', 'consent-magic' ); ?></h4>
                                </div>
                            </div>
                        </div>

                        <div class="cs_custom_text">
                            <div class="list-item mb-24">
								<?php render_switcher_input( '_cs_custom_text', false, true, null, false, false, __( 'Custom text', 'consent-magic' ) ); ?>
                            </div>

                            <div class="custom_text_wrap mb-24" style="display:none;">
								<?php
								$inputs   = array(
									array(
										'type'        => 'textarea',
										'meta'        => true,
										'key'         => '_cs_text_in_small_bar_popup',
										'input_title' => '',
									)
								);
								$title    = __( 'Small bar or popup', 'consent-magic' );
								$subtitle = __( 'This text is used when "small bar" or "small popup" are selected', 'consent-magic' );
								render_language_block_new_template( $inputs, $title, $subtitle );

								$inputs   = array(
									array(
										'type'        => 'textarea',
										'meta'        => true,
										'key'         => '_cs_text_in_large_bar_popup',
										'input_title' => '',
									)
								);
								$title    = __( 'Large bar or popup', 'consent-magic' );
								$subtitle = __( 'This text is used when "large bar" or "large popup" are selected', 'consent-magic' );
								render_language_block_new_template( $inputs, $title, $subtitle );

								$inputs   = array(
									array(
										'type'        => 'input',
										'meta'        => true,
										'key'         => '_cs_btn_text_allow_all',
										'input_title' => __( 'Allow all', 'consent-magic' ),
									),
									array(
										'type'        => 'input',
										'meta'        => true,
										'key'         => '_cs_btn_text_custom_button',
										'input_title' => __( 'Custom button', 'consent-magic' ),
									),
									array(
										'type'        => 'input',
										'meta'        => true,
										'key'         => '_cs_btn_text_disable_all',
										'input_title' => __( 'Disable all', 'consent-magic' ),
									),
									array(
										'type'        => 'input',
										'meta'        => true,
										'key'         => '_cs_btn_text_customize',
										'input_title' => __( 'Customize', 'consent-magic' ),
									),
									array(
										'type'        => 'input',
										'meta'        => true,
										'key'         => '_cs_btn_text_privacy_cookie',
										'input_title' => __( 'Privacy & Cookies', 'consent-magic' ),
									),
									array(
										'type'        => 'input',
										'meta'        => true,
										'key'         => '_cs_btn_text_sticky_cookie',
										'input_title' => __( 'Sticky', 'consent-magic' ),
									),
								);
								$title    = __( 'Button\'s text', 'consent-magic' );
								$subtitle = __( 'This text is used on buttons on all types of bars and popups', 'consent-magic' );
								render_language_block_new_template( $inputs, $title, $subtitle );

								$inputs   = array(
									array(
										'type'        => 'textarea',
										'meta'        => true,
										'key'         => '_cs_text_in_options_popup',
										'input_title' => '',
									),
									array(
										'type'        => 'input',
										'meta'        => true,
										'key'         => '_cs_title_in_options_popup',
										'input_title' => __( 'Title', 'consent-magic' ),
									),
									array(
										'type'        => 'input',
										'meta'        => true,
										'key'         => '_cs_subtitle_in_options_popup',
										'input_title' => __( 'Subtitle', 'consent-magic' ),
									),
									array(
										'type'        => 'input',
										'meta'        => true,
										'key'         => '_cs_btn_text_allow_all_in_options_popup',
										'input_title' => __( 'Allow all', 'consent-magic' ),
									),
									array(
										'type'        => 'input',
										'meta'        => true,
										'key'         => '_cs_btn_text_disable_all_in_options_popup',
										'input_title' => __( 'Disable all', 'consent-magic' ),
									),
									array(
										'type'        => 'input',
										'meta'        => true,
										'key'         => '_cs_btn_text_confirm_in_options_popup',
										'input_title' => __( 'Confirm my choices', 'consent-magic' ),
									),
									array(
										'type'        => 'input',
										'meta'        => true,
										'key'         => '_cs_always_on_in_options_popup',
										'input_title' => __( 'Always ON', 'consent-magic' ),
									),
								);
								$title    = __( 'Options popups', 'consent-magic' );
								$subtitle = __( 'Customize the content of the options popup', 'consent-magic' );
								render_language_block_new_template( $inputs, $title, $subtitle );


								$inputs   = array(
									array(
										'type'        => 'textarea',
										'meta'        => true,
										'key'         => '_cs_text_in_single_design',
										'input_title' => '',
									),
									array(
										'type'        => 'input',
										'meta'        => true,
										'key'         => '_cs_title_in_single_design',
										'input_title' => __( 'Title', 'consent-magic' ),
									),
									array(
										'type'        => 'input',
										'meta'        => true,
										'key'         => '_cs_subtitle_in_single_design',
										'input_title' => __( 'Subtitle', 'consent-magic' ),
									),
									array(
										'type'        => 'input',
										'meta'        => true,
										'key'         => '_cs_btn_text_allow_all_in_single_design',
										'input_title' => __( 'Allow all', 'consent-magic' ),
									),
									array(
										'type'        => 'input',
										'meta'        => true,
										'key'         => '_cs_btn_text_disable_all_in_single_design',
										'input_title' => __( 'Disable all', 'consent-magic' ),
									),
									array(
										'type'        => 'input',
										'meta'        => true,
										'key'         => '_cs_btn_text_confirm_in_single_design',
										'input_title' => __( 'Confirm my choices', 'consent-magic' ),
									),
									array(
										'type'        => 'input',
										'meta'        => true,
										'key'         => '_cs_btn_text_custom_button_in_single_design',
										'input_title' => __( 'Custom button', 'consent-magic' ),
									),
									array(
										'type'        => 'input',
										'meta'        => true,
										'key'         => '_cs_always_on_in_single_design',
										'input_title' => __( 'Always ON', 'consent-magic' ),
									),
								);
								$title    = __( 'Single-step', 'consent-magic' );
								$subtitle = __( 'Customize the content of the single-step', 'consent-magic' );
								render_language_block_new_template( $inputs, $title, $subtitle );
								?>
                            </div>

                            <div class="list-item">
                                <div class="block-with-button">
									<?php $message = __( 'You can change the default text from the', 'consent-magic' );

									render_notice_message( $message, 500 );
									?>

                                    <a href="<?php echo esc_url( get_admin_url() . 'admin.php?page=consent-magic&tab=cs-text' ); ?>"
                                       class="btn btn-primary btn-primary-type2"><?php esc_html_e( 'Text Section', 'consent-magic' ); ?></a>
                                </div>
                            </div>
                        </div>
                    </div>
                </div>
            </div>
        </div>
    </form>
</div>