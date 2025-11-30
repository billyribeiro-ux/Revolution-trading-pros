<?php

namespace ConsentMagicPro;

// If this file is called directly, abort.
if ( !defined( 'WPINC' ) ) {
	die;
}

$wp_current_lang          = ( isset( $this->current_lang ) && $this->current_lang ) ? $this->current_lang : get_locale();
CS_Translator()->setCurrentLanguage( $wp_current_lang );

$terms_primary = get_cookies_terms_objects( 'cs_necessary_term', true );
$terms         = get_cookies_terms_objects( 'cs_necessary_term' );

$fb_cat = ConsentMagic()->getOption( ConsentMagic()->getOption( 'cs_block_fb_pixel_scripts_cat' ) . '_cat_id' );

$front_options = array(
	'_cs_text_consent'                            => '',
	'_cs_title_in_single_design'                  => '',
	'_cs_text_in_single_design'                   => '',
	'_cs_btn_text_disable_all_in_single_design'   => '',
	'_cs_btn_text_custom_button_in_single_design' => '',
	'_cs_btn_text_allow_all_in_single_design'     => '',
	'_cs_subtitle_in_single_design'               => '',
	'_cs_btn_text_confirm_in_single_design'       => '',
	'_cs_always_on_in_single_design'              => '',
	'_cs_btn_text_customize'                      => '',
	'_cs_consent'                                 => '',
	'cs_advanced_matching_description'            => 'option',
	'cs_server_side_consent_description'          => 'option',
);

$cs_user_default_language = ConsentMagic()->getOption( 'cs_user_default_language' );
$cs_language_availability = ConsentMagic()->getOption( 'cs_language_availability' );
if ( isset( $cs_language_availability[ $wp_current_lang ] ) && $cs_language_availability[ $wp_current_lang ] == 0 ) {
	$wp_current_lang = $cs_user_default_language;
}


$custom_text = get_post_meta( $active_rule_id, '_cs_custom_text', true );
$test_prefix = ConsentMagic()->getOption( 'cs_test_mode' ) ? 'test_' : '';

$logo = wp_get_attachment_image( $style_array[ 'cs_logo' ][ 0 ], $style_array[ 'cs_logo_size' ][ 0 ], false, array(
	'style' => 'display:inline-block;'
) );

$front_options = generate_front_text( $front_options, $custom_text, $active_rule_id, $wp_current_lang, $cs_user_default_language );

?>

<div class="cs-tab-content">
    <div class="cs-modal-content"
         style="background-color: <?php echo esc_attr( $style_array[ 'cs_backend_color' ][ 0 ] ); ?>;
                 border: <?php echo esc_attr( $style_array[ 'cs_border_weight' ][ 0 ] ) . 'px ' . esc_attr( $style_array[ 'cs_border_style' ][ 0 ] ) . ' ' . esc_attr( $style_array[ 'cs_border_color' ][ 0 ] ); ?>;
                 color: <?php echo esc_attr( $style_array[ 'cs_text_color' ][ 0 ] ); ?>;
                 fill: <?php echo esc_attr( $style_array[ 'cs_text_color' ][ 0 ] ); ?>;
                 padding: <?php echo esc_attr( $op_pd_t ) . 'px ' . esc_attr( $op_pd_r ) . 'px ' . esc_attr( $op_pd_b ) . 'px ' . esc_attr( $op_pd_l ) . 'px'; ?>;
                 font-size: <?php echo esc_attr( $op_font_s ); ?>px;
                 font-weight: <?php echo esc_attr( $op_font_w ); ?>;
                 ">

		<?php if ( ( isset( $cs_hide_close_btn ) && $cs_hide_close_btn == '0' ) || $cs_type == 'just_inform' ) { ?>
            <button type="button" class="cs-modal-close" data-action="cs_close_opt_popup">
                <svg viewBox="0 0 24 24">
                    <path d="M19 6.41l-1.41-1.41-5.59 5.59-5.59-5.59-1.41 1.41 5.59 5.59-5.59 5.59 1.41 1.41 5.59-5.59 5.59 5.59 1.41-1.41-5.59-5.59z"></path>
                    <path d="M0 0h24v24h-24z" fill="none"></path>
                </svg>
                <span class="cs-sr-only"><?php esc_html_e( 'Close', 'consent-magic' ); ?></span>
            </button>
		<?php } ?>

        <div id="cs-modal-body"
             class="cs-modal-body <?php echo ( $minimum_recommended_button ? esc_attr( 'cs-minimum-recommended' ) : '' ) . ' ' . ( $deny_all_button ? esc_attr( 'cs-deny-all' ) : '' ) . ' ' . ( $options_single_verified_link ? esc_attr( 'cs-options-single-verified-link' ) : '' ); ?>">

            <div class="cs-container-fluid cs-tab-container <?php echo esc_attr( $cs_type ) . ' ';
			echo $deny_all_button ? 'cs_deny_all_btn' : ''; ?> <?php
			echo $minimum_recommended_button ? 'cs_custom_button' : ''; ?>">
                <div class="cs_sub_tab_container" style="border: none;">
                    <div>
                        <div style="margin-bottom: 16px;">
                            <div class="cs-privacy-overview"
                                 style="font-size: <?php echo esc_attr( $op_font_s ); ?>px; font-weight: <?php echo esc_attr( $op_font_w ); ?>;">
								<?php if ( $style_array[ 'cs_position_vertical_list' ][ 0 ] == 'top' && $logo ) :
									echo '<div style="margin: 0 auto 20px;text-align: ' . esc_attr( $style_array[ 'cs_position_horizontal_list' ][ 0 ] ) . ';">' . wp_kses_post( $logo ) . '</div>';
								endif;
								?>
                                <h4 style="color:<?php echo esc_attr( $style_array[ 'cs_titles_text_color' ][ 0 ] ); ?>;
                                        font-size: <?php echo esc_attr( $op_title_f_s ); ?>px;
                                        font-weight: <?php echo esc_attr( $op_title_f_w ); ?>;"><?php echo esc_html( $front_options[ '_cs_title_in_single_design' ] ); ?></h4>
                                <div class="cs-privacy-content">
                                    <div class="cs-privacy-content-text"
                                         style="color:<?php echo esc_attr( $style_array[ 'cs_text_color' ][ 0 ] ); ?>;">
										<?php echo wp_kses_post( $front_options[ '_cs_text_in_single_design' ] ); ?>
										<?php if ( $privacy_link ): ?>
                                            <div class="privacy-link">
												<?php
                                                $privacy_policy_page = renderPrivacyPolicyPage( $wp_current_lang );
												if ( $privacy_policy_page ) {
													echo '<div class="policy_wrap"><a href="' . esc_url( get_permalink( $privacy_policy_page ) ) . '" target="_blank" style="color:' . esc_attr( $style_array[ 'cs_links_color' ][ 0 ] ) . '; ">' . esc_html__( get_the_title( $privacy_policy_page ), 'consent-magic' ) . '</a></div>';
												}
												?>
                                            </div>
										<?php endif; ?>
                                    </div>
                                </div>
								<?php
								if ( $style_array[ 'cs_position_vertical_list' ][ 0 ] == 'bottom' && $logo ) :
									echo '<div style="margin: 20px auto 0;text-align: ' . esc_attr( $style_array[ 'cs_position_horizontal_list' ][ 0 ] ) . ';">' . wp_kses_post( $logo ) . '</div>';
								endif;
								?>
                            </div>
                        </div>

                        <div class="line"></div>

                        <div class="cs_manage_consent_properties">
                            <h4 style="color:<?php echo esc_attr( $style_array[ 'cs_titles_text_color' ][ 0 ] ); ?>;
                                    font-size: <?php echo esc_attr( $op_title_f_s ); ?>px;
                                    font-weight: <?php echo esc_attr( $op_title_f_w ); ?>;">
								<?php echo esc_html( $front_options[ '_cs_subtitle_in_single_design' ] ); ?>
                            </h4>

							<?php
							if ( $terms_primary ) {
								foreach ( $terms_primary as $term ) { ?>
									<?php if ( (int) get_term_meta( $term->term_id, 'cs_ignore_this_category', true ) == 0 ) { ?>
                                        <div>
                                            <div class="cm-script-title-block">
                                                <h4
                                                        style="color:<?php echo esc_attr( $style_array[ 'cs_subtitles_text_color' ][ 0 ] ); ?>;
                                                                font-size: <?php echo esc_attr( $op_subtitle_f_s ); ?>px;
                                                                font-weight: <?php echo esc_attr( $op_subtitle_f_w ); ?>;"><?php echo esc_html( $term->name_l ); ?></h4>

                                                <div class="green-text">
                                                <span style="color:<?php echo esc_attr( $style_array[ 'cs_cat_color' ][ 0 ] ); ?>;
                                                        font-size: <?php echo esc_attr( $op_subtitle_f_s ); ?>px;
                                                        font-weight: <?php echo esc_attr( $op_subtitle_f_w ); ?>;">
                                                    <?php echo esc_html( $front_options[ '_cs_always_on_in_single_design' ] ); ?>
                                                </span>
                                                </div>
                                            </div>

                                            <div class="cm-script-description"
                                                 style="color:<?php echo esc_attr( $style_array[ 'cs_text_color' ][ 0 ] ); ?>;">
												<?php echo wp_kses_post( $term->description_l ); ?>
                                            </div>
                                        </div>

										<?php
										if ( isPYSActivated() ) {
											if ( $fb_cat == $term->term_id ) {
												$disabled = ( ( CS_Cookies()->getCookie( 'cs_enabled_cookie_term_' . $term->term_id ) && CS_Cookies()->getCookie( 'cs_enabled_cookie_term_' . $term->term_id ) == 'no' ) || !CS_Cookies()->getCookie( 'cs_viewed_cookie_policy' ) && $cs_type == 'inform_and_opiout' ) ? true : false;
												if ( ConsentMagic()->getOption( 'cs_test_mode' ) ) {
													$disabled = ( ( CS_Cookies()->getCookie( 'cs_enabled_cookie_term_test_' . $term->term_id ) && CS_Cookies()->getCookie( 'cs_enabled_cookie_term_test_' . $term->term_id ) == 'no' ) || !CS_Cookies()->getCookie( 'CS-Magic_test' ) && $cs_type == 'inform_and_opiout' ) ? true : false;
												}
												?>
                                                <div style="padding-left: 30px;" class="facebook_term_group_switchers">
													<?php if ( ConsentMagic()->getOption( 'cs_advanced_matching_consent_enabled' ) ) { ?>
                                                        <div>
                                                            <div class="cm-script-title-block">
                                                                <h4
                                                                        style="color:<?php echo esc_attr( $style_array[ 'cs_subtitles_text_color' ][ 0 ] ); ?>;
                                                                                font-size: <?php echo esc_attr( $op_subtitle_f_s ); ?>px;
                                                                                font-weight: <?php echo esc_attr( $op_subtitle_f_w ); ?>;">
																	<?php esc_html_e( 'Advanced Matching', 'consent-magic' ); ?>
                                                                </h4>

                                                                <div>
																	<?php render_switcher_input_front( ConsentMagic()->getOption( 'cs_test_mode' ) ? 'cs_enabled_advanced_matching_test' : 'cs_enabled_advanced_matching', false, false, null, false, $disabled ); ?>
                                                                </div>
                                                            </div>

                                                            <div class="cm-script-description"
                                                                 style="color:<?php echo esc_attr( $style_array[ 'cs_text_color' ][ 0 ] ); ?>;">
																<?php echo esc_html( $front_options[ 'cs_advanced_matching_description' ] ); ?>
                                                            </div>
                                                        </div>
													<?php } ?>

													<?php if ( ConsentMagic()->getOption( 'cs_server_side_consent_enabled' ) ) { ?>
                                                        <div>
                                                            <div class="cm-script-title-block">
                                                                <h4
                                                                        style="color:<?php echo esc_attr( $style_array[ 'cs_subtitles_text_color' ][ 0 ] ); ?>;
                                                                                font-size: <?php echo esc_attr( $op_subtitle_f_s ); ?>px;
                                                                                font-weight: <?php echo esc_attr( $op_subtitle_f_w ); ?>;"><?php esc_attr_e( 'Server-side events', 'consent-magic' ); ?></h4>

                                                                <div>
																	<?php render_switcher_input_front( ConsentMagic()->getOption( 'cs_test_mode' ) ? 'cs_enabled_server_side_test' : 'cs_enabled_server_side', false, false, null, false, $disabled ); ?>
                                                                </div>
                                                            </div>

                                                            <div class="cm-script-description"
                                                                 style="color:<?php echo esc_attr( $style_array[ 'cs_text_color' ][ 0 ] ); ?>;">
																<?php echo esc_html( $front_options[ 'cs_server_side_consent_description' ] ); ?>
                                                            </div>
                                                        </div>
													<?php } ?>
                                                </div>
												<?php
											}
										}
									}
								}
							} ?>

							<?php
							if ( $terms ) {
								foreach ( $terms as $term ) { ?>
									<?php if ( (int) get_term_meta( $term->term_id, 'cs_ignore_this_category', true ) == 0 ) { ?>
                                        <div>
                                            <div class="cm-script-title-block">
                                                <h4
                                                        style="color:<?php echo esc_attr( $style_array[ 'cs_subtitles_text_color' ][ 0 ] ); ?>;
                                                                font-size: <?php echo esc_attr( $op_subtitle_f_s ); ?>px;
                                                                font-weight: <?php echo esc_attr( $op_subtitle_f_w ); ?>;"><?php echo esc_html( $term->name_l ); ?></h4>

                                                <div class="<?php echo ( $fb_cat == $term->term_id ) ? 'facebook_term_group' : ''; ?>">
													<?php render_switcher_input_front( ConsentMagic()->getOption( 'cs_test_mode' ) ? 'cs_enabled_cookie_term_test_' . $term->term_id : 'cs_enabled_cookie_term_' . $term->term_id, false ); ?>
                                                </div>
                                            </div>

                                            <div class="cm-script-description"
                                                 style="color:<?php echo esc_attr( $style_array[ 'cs_text_color' ][ 0 ] ); ?>;">
												<?php echo wp_kses_post( $term->description_l ); ?>
                                            </div>
                                        </div>

										<?php
										if ( isPYSActivated() ) {
											if ( $fb_cat == $term->term_id ) {
												$disabled = ( ( CS_Cookies()->getCookie( 'cs_enabled_cookie_term_' . $term->term_id ) && CS_Cookies()->getCookie( 'cs_enabled_cookie_term_' . $term->term_id ) == 'no' ) || !CS_Cookies()->getCookie( 'cs_viewed_cookie_policy' ) && $cs_type == 'inform_and_opiout' ) ? true : false;
												if ( ConsentMagic()->getOption( 'cs_test_mode' ) ) {
													$disabled = ( ( CS_Cookies()->getCookie( 'cs_enabled_cookie_term_test_' . $term->term_id ) && CS_Cookies()->getCookie( 'cs_enabled_cookie_term_test_' . $term->term_id ) == 'no' ) || !CS_Cookies()->getCookie( 'CS-Magic_test' ) && $cs_type == 'inform_and_opiout' ) ? true : false;
												}
												?>
                                                <div style="padding-left: 30px;" class="facebook_term_group_switchers">
													<?php if ( ConsentMagic()->getOption( 'cs_advanced_matching_consent_enabled' ) ) { ?>
                                                        <div>
                                                            <div class="cm-script-title-block">
                                                                <h4
                                                                        style="color:<?php echo esc_attr( $style_array[ 'cs_subtitles_text_color' ][ 0 ] ); ?>;
                                                                                font-size: <?php echo esc_attr( $op_subtitle_f_s ); ?>px;
                                                                                font-weight: <?php echo esc_attr( $op_subtitle_f_w ); ?>;"><?php esc_html_e( 'Facebook Advanced Matching', 'consent-magic' ); ?></h4>

                                                                <div>
																	<?php render_switcher_input_front( ConsentMagic()->getOption( 'cs_test_mode' ) ? 'cs_enabled_advanced_matching_test' : 'cs_enabled_advanced_matching', false, false, null, false, $disabled ); ?>
                                                                </div>
                                                            </div>

                                                            <div class="cm-script-description"
                                                                 style="color:<?php echo esc_attr( $style_array[ 'cs_text_color' ][ 0 ] ); ?>;">
																<?php echo esc_html( $front_options[ 'cs_advanced_matching_description' ] ); ?>
                                                            </div>
                                                        </div>
													<?php } ?>

													<?php if ( ConsentMagic()->getOption( 'cs_server_side_consent_enabled' ) ) { ?>
                                                        <div>
                                                            <div class="cm-script-title-block">
                                                                <h4
                                                                        style="color:<?php echo esc_attr( $style_array[ 'cs_subtitles_text_color' ][ 0 ] ); ?>;
                                                                                font-size: <?php echo esc_attr( $op_subtitle_f_s ); ?>px;
                                                                                font-weight: <?php echo esc_attr( $op_subtitle_f_w ); ?>;"><?php esc_html_e( 'Facebook CAPI', 'consent-magic' ); ?></h4>

                                                                <div>
																	<?php render_switcher_input_front( ConsentMagic()->getOption( 'cs_test_mode' ) ? 'cs_enabled_server_side_test' : 'cs_enabled_server_side', false, false, null, false, $disabled ); ?>
                                                                </div>
                                                            </div>

                                                            <div class="cm-script-description"
                                                                 style="color:<?php echo esc_attr( $style_array[ 'cs_text_color' ][ 0 ] ); ?>;">
																<?php echo esc_html( $front_options[ 'cs_server_side_consent_description' ] ); ?>
                                                            </div>
                                                        </div>
													<?php } ?>
                                                </div>
												<?php
											}
										}
									}
								}
							} ?>
                        </div>
                    </div>
                </div>
            </div>
        </div>

        <div class="cs-row cs-bottom-buttons-row">
			<?php
			include CMPRO_PLUGIN_VIEWS_PATH . 'templates/buttons/cs_bottom_buttons_single.php'; ?>
        </div>
    </div>
</div>
