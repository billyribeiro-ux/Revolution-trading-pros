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
	'_cs_title_in_options_popup'                => '',
	'_cs_text_in_options_popup'                 => '',
	'_cs_btn_text_disable_all_in_options_popup' => '',
	'_cs_btn_text_custom_button'                => '',
	'_cs_btn_text_allow_all_in_options_popup'   => '',
	'_cs_subtitle_in_options_popup'             => '',
	'_cs_btn_text_confirm_in_options_popup'     => '',
	'_cs_always_on_in_options_popup'            => '',
	'cs_advanced_matching_description'          => 'option',
	'cs_server_side_consent_description'        => 'option'
);

if ( !isset( $cs_preview_size_show ) ) {
	$op_btn_pd_t   = ConsentMagic()->getOption( 'cs_d_op_btn_pd_t' );
	$op_btn_pd_r   = ConsentMagic()->getOption( 'cs_d_op_btn_pd_r' );
	$op_btn_pd_b   = ConsentMagic()->getOption( 'cs_d_op_btn_pd_bt' );
	$op_btn_pd_l   = ConsentMagic()->getOption( 'cs_d_op_btn_pd_l' );
	$op_btn_mg_t   = ConsentMagic()->getOption( 'cs_d_op_btn_mg_t' );
	$op_btn_mg_r   = ConsentMagic()->getOption( 'cs_d_op_btn_mg_r' );
	$op_btn_mg_b   = ConsentMagic()->getOption( 'cs_d_op_btn_mg_bt' );
	$op_btn_mg_l   = ConsentMagic()->getOption( 'cs_d_op_btn_mg_l' );
	$op_btn_font_s = ConsentMagic()->getOption( 'cs_d_op_btn_font_size' );
	$op_btn_font_w = ConsentMagic()->getOption( 'cs_d_op_btn_font_weight' );

	$op_title_f_s    = ConsentMagic()->getOption( 'cs_d_op_titles_font_size' );
	$op_title_f_w    = ConsentMagic()->getOption( 'cs_d_op_titles_font_weight' );
	$op_subtitle_f_s = ConsentMagic()->getOption( 'cs_d_op_subtitles_font_size' );
	$op_subtitle_f_w = ConsentMagic()->getOption( 'cs_d_op_subtitles_font_weight' );
}

$cs_user_default_language = ConsentMagic()->getOption( 'cs_user_default_language' );
$cs_language_availability = ConsentMagic()->getOption( 'cs_language_availability' );
if ( isset( $cs_language_availability[ $wp_current_lang ] ) && $cs_language_availability[ $wp_current_lang ] == 0 ) {
	$wp_current_lang = $cs_user_default_language;
}

$custom_text = get_post_meta( $active_rule_id, '_cs_custom_text', true );

$front_options = generate_front_text( $front_options, $custom_text, $active_rule_id, $wp_current_lang, $cs_user_default_language );

?>
<div class="cs-container-fluid cs-tab-container <?php echo esc_attr( $cs_type );
echo ( ( ( isset( $cs_deny_all_btn ) && !empty( $cs_deny_all_btn ) ) || ( isset( $style_array[ '_cs_deny_all_btn' ][ 0 ] ) ) && !empty( $style_array[ '_cs_deny_all_btn' ][ 0 ] ) ) ) ? ' cs_deny_all_btn' : ''; ?> <?php
echo ( ( ( isset( $cs_custom_button_btn ) && !empty( $cs_custom_button_btn ) ) || ( isset( $style_array[ '_cs_custom_button' ][ 0 ] ) ) && !empty( $style_array[ '_cs_custom_button' ][ 0 ] ) ) ) ? ' cs_custom_button' : ''; ?>
">
    <div>
        <div>
            <div class="cs-privacy-overview"
                 style="font-size: <?php echo esc_attr( $op_font_s ); ?>px; font-weight: <?php echo esc_attr( $op_font_w ); ?>;">
                <h4 style="color:<?php echo esc_attr( $style_array[ 'cs_titles_text_color' ][ 0 ] ); ?>;
                        font-size: <?php echo esc_attr( $op_title_f_s ); ?>px;
                        font-weight: <?php echo esc_attr( $op_title_f_w ); ?>;"><?php echo esc_html( $front_options[ '_cs_title_in_options_popup' ] ); ?></h4>
                <div class="cs-privacy-content">
                    <div class="cs-privacy-content-text"
                         style="color:<?php echo esc_attr( $style_array[ 'cs_text_color' ][ 0 ] ); ?>;">
						<?php echo wp_kses_post( $front_options[ '_cs_text_in_options_popup' ] ); ?>
						<?php if ( $privacy_link ): ?>
                            <div class="privacy-link">
								<?php
                                $privacy_policy_page = renderPrivacyPolicyPage( $wp_current_lang );
								if ( $privacy_policy_page ) {
									echo '<div class="policy_wrap"><a href="' . esc_url( get_permalink( $privacy_policy_page ) ) . '" target="_blank" style="color:' . esc_attr( $style_array[ 'cs_links_color' ][ 0 ] ) . '; --cs-color-underline: ' . esc_attr( $style_array[ 'cs_links_color' ][ 0 ] ) . ';">' . esc_html__( get_the_title( $privacy_policy_page ), 'consent-magic' ) . '</a></div>';
								}
								?>
                            </div>
						<?php endif; ?>
                    </div>

                    <div class="cs_policy_existing_page">
                        <div class="btn-row">
							<?php if ( ( ( isset( $cs_deny_all_btn ) && !empty( $cs_deny_all_btn ) ) || ( isset( $style_array[ '_cs_deny_all_btn' ][ 0 ] ) ) && !empty( $style_array[ '_cs_deny_all_btn' ][ 0 ] ) ) ) { ?>
                                <a role="button" href="#" tabindex="0" class="btn disable_all_btn cs_action_btn"
                                   data-cs_action="disable_all"
                                   style="background-color: <?php echo esc_attr( $style_array[ 'cs_deny_all_buttons_bg' ][ 0 ] ); ?>;
                                           color: <?php echo esc_attr( $style_array[ 'cs_deny_all_buttons_text_color' ][ 0 ] ); ?>;
                                           padding: <?php echo esc_attr( $op_btn_pd_t ); ?>px <?php echo esc_attr( $op_btn_pd_r ); ?>px <?php echo esc_attr( $op_btn_pd_b ); ?>px <?php echo esc_attr( $op_btn_pd_l ); ?>px;
                                           margin: <?php echo esc_attr( $op_btn_mg_t ); ?>px <?php echo esc_attr( $op_btn_mg_r ); ?>px <?php echo esc_attr( $op_btn_mg_b ); ?>px <?php echo esc_attr( $op_btn_mg_l ); ?>px;
                                           font-size: <?php echo esc_attr( $op_btn_font_s ); ?>px;
                                           font-weight: <?php echo esc_attr( $op_btn_font_w ); ?>;"><?php echo esc_html( $front_options[ '_cs_btn_text_disable_all_in_options_popup' ] ); ?></a>
							<?php } ?>
							<?php if ( ( ( isset( $cs_custom_button_btn ) && !empty( $cs_custom_button_btn ) ) || ( isset( $style_array[ '_cs_custom_button' ][ 0 ] ) ) && !empty( $style_array[ '_cs_custom_button' ][ 0 ] ) ) ) { ?>
                                <a role="button" href="#" tabindex="0" class="btn minimum_recommended_btn cs_action_btn"
                                   data-cs_action="minimum_recommended"
                                   style="background-color: <?php echo esc_attr( $style_array[ 'cs_custom_button_buttons_bg' ][ 0 ] ); ?>;
                                           color: <?php echo esc_attr( $style_array[ 'cs_custom_button_buttons_text_color' ][ 0 ] ); ?>;
                                           padding: <?php echo esc_attr( $op_btn_pd_t ); ?>px <?php echo esc_attr( $op_btn_pd_r ); ?>px <?php echo esc_attr( $op_btn_pd_b ); ?>px <?php echo esc_attr( $op_btn_pd_l ); ?>px;
                                           margin: <?php echo esc_attr( $op_btn_mg_t ); ?>px <?php echo esc_attr( $op_btn_mg_r ); ?>px <?php echo esc_attr( $op_btn_mg_b ); ?>px <?php echo esc_attr( $op_btn_mg_l ); ?>px;
                                           font-size: <?php echo esc_attr( $op_btn_font_s ); ?>px;
                                           font-weight: <?php echo esc_attr( $op_btn_font_w ); ?>;"><?php echo esc_html( $front_options[ '_cs_btn_text_custom_button' ] ); ?></a>
							<?php } ?>
                            <a role="button" href="#" tabindex="0" class="btn btn-grey allow_all_btn cs_action_btn"
                               data-cs_action="allow_all"
                               style="background-color: <?php echo esc_attr( $style_array[ 'cs_accept_all_buttons_bg' ][ 0 ] ); ?>;
                                       color: <?php echo esc_attr( $style_array[ 'cs_accept_all_buttons_text_color' ][ 0 ] ); ?>;
                                       padding: <?php echo esc_attr( $op_btn_pd_t ); ?>px <?php echo esc_attr( $op_btn_pd_r ); ?>px <?php echo esc_attr( $op_btn_pd_b ); ?>px <?php echo esc_attr( $op_btn_pd_l ); ?>px;
                                       margin: <?php echo esc_attr( $op_btn_mg_t ); ?>px <?php echo esc_attr( $op_btn_mg_r ); ?>px <?php echo esc_attr( $op_btn_mg_b ); ?>px <?php echo esc_attr( $op_btn_mg_l ); ?>px;
                                       font-size: <?php echo esc_attr( $op_btn_font_s ); ?>px;
                                       font-weight: <?php echo esc_attr( $op_btn_font_w ); ?>;"><?php echo esc_html( $front_options[ '_cs_btn_text_allow_all_in_options_popup' ] ); ?></a>
                        </div>
                    </div>
                </div>
            </div>
        </div>

        <div class="line"></div>

        <div class="cs_popup_content">
            <h4 style="color:<?php echo esc_attr( $style_array[ 'cs_titles_text_color' ][ 0 ] ); ?>;
                    font-size: <?php echo esc_attr( $op_title_f_s ); ?>px;
                    font-weight: <?php echo esc_attr( $op_title_f_w ); ?>;
                    margin-bottom: 16px;"><?php echo esc_html( $front_options[ '_cs_subtitle_in_options_popup' ] ); ?></h4>
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
                                        <?php echo esc_html( $front_options[ '_cs_always_on_in_options_popup' ] ); ?>
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
												<?php echo wp_kses_post( $front_options[ 'cs_advanced_matching_description' ] ); ?>
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
												<?php echo wp_kses_post( $front_options[ 'cs_server_side_consent_description' ] ); ?>
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
												<?php echo wp_kses_post( $front_options[ 'cs_advanced_matching_description' ] ); ?>
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
												<?php echo wp_kses_post( $front_options[ 'cs_server_side_consent_description' ] ); ?>
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
        <div>
            <div class="cs-tab-footer btn-row right-aligned" style="margin-top: 32px;">
                <a role="button" href="#" tabindex="0" class="cs_setting_save_button btn btn-grey cs_action_btn"
                   data-cs_action="cs_confirm"
                   style="background-color: <?php echo esc_attr( $style_array[ 'cs_confirm_buttons_bg' ][ 0 ] ); ?>;
                           color: <?php echo esc_attr( $style_array[ 'cs_confirm_buttons_text_color' ][ 0 ] ); ?>;
                           padding: <?php echo esc_attr( $op_btn_pd_t ); ?>px <?php echo esc_attr( $op_btn_pd_r ); ?>px <?php echo esc_attr( $op_btn_pd_b ); ?>px <?php echo esc_attr( $op_btn_pd_l ); ?>px;
                           margin: <?php echo esc_attr( $op_btn_mg_t ); ?>px <?php echo esc_attr( $op_btn_mg_r ); ?>px <?php echo esc_attr( $op_btn_mg_b ); ?>px <?php echo esc_attr( $op_btn_mg_l ); ?>px;
                           font-size: <?php echo esc_attr( $op_btn_font_s ); ?>px;
                           font-weight: <?php echo esc_attr( $op_btn_font_w ); ?>;"><?php echo esc_html( $front_options[ '_cs_btn_text_confirm_in_options_popup' ] ); ?></a>
                <a role="button" href="#" tabindex="0" class="btn btn-grey allow_all_btn cs_action_btn"
                   data-cs_action="allow_all"
                   style="background-color: <?php echo esc_attr( $style_array[ 'cs_accept_all_buttons_bg' ][ 0 ] ); ?>;
                           color: <?php echo esc_attr( $style_array[ 'cs_accept_all_buttons_text_color' ][ 0 ] ); ?>;
                           padding: <?php echo esc_attr( $op_btn_pd_t ); ?>px <?php echo esc_attr( $op_btn_pd_r ); ?>px <?php echo esc_attr( $op_btn_pd_b ); ?>px <?php echo esc_attr( $op_btn_pd_l ); ?>px;
                           margin: <?php echo esc_attr( $op_btn_mg_t ); ?>px <?php echo esc_attr( $op_btn_mg_r ); ?>px <?php echo esc_attr( $op_btn_mg_b ); ?>px <?php echo esc_attr( $op_btn_mg_l ); ?>px;
                           font-size: <?php echo esc_attr( $op_btn_font_s ); ?>px;
                           font-weight: <?php echo esc_attr( $op_btn_font_w ); ?>;"><?php echo esc_html( $front_options[ '_cs_btn_text_allow_all_in_options_popup' ] ); ?></a>
				<?php if ( ( ( isset( $cs_custom_button_btn ) && !empty( $cs_custom_button_btn ) ) || ( isset( $style_array[ '_cs_custom_button' ][ 0 ] ) ) && !empty( $style_array[ '_cs_custom_button' ][ 0 ] ) ) ) { ?>
                    <a role="button" href="#" tabindex="0" class="btn minimum_recommended_btn cs_action_btn"
                       data-cs_action="minimum_recommended"
                       style="background-color: <?php echo esc_attr( $style_array[ 'cs_custom_button_buttons_bg' ][ 0 ] ); ?>;
                               display: none;
                               color: <?php echo esc_attr( $style_array[ 'cs_custom_button_buttons_text_color' ][ 0 ] ); ?>;
                               padding: <?php echo esc_attr( $op_btn_pd_t ); ?>px <?php echo esc_attr( $op_btn_pd_r ); ?>px <?php echo esc_attr( $op_btn_pd_b ); ?>px <?php echo esc_attr( $op_btn_pd_l ); ?>px;
                               margin: <?php echo esc_attr( $op_btn_mg_t ); ?>px <?php echo esc_attr( $op_btn_mg_r ); ?>px <?php echo esc_attr( $op_btn_mg_b ); ?>px <?php echo esc_attr( $op_btn_mg_l ); ?>px;
                               font-size: <?php echo esc_attr( $op_btn_font_s ); ?>px;
                               font-weight: <?php echo esc_attr( $op_btn_font_w ); ?>;
                               "><?php echo esc_html( $front_options[ '_cs_btn_text_custom_button' ] ); ?></a>
				<?php } ?>
				<?php if ( ( ( isset( $cs_deny_all_btn ) && !empty( $cs_deny_all_btn ) ) || ( isset( $style_array[ '_cs_deny_all_btn' ][ 0 ] ) ) && !empty( $style_array[ '_cs_deny_all_btn' ][ 0 ] ) ) ) { ?>
                    <a role="button" href="#" tabindex="0" class="btn disable_all_btn cs_action_btn"
                       data-cs_action="disable_all"
                       style="background-color: <?php echo esc_attr( $style_array[ 'cs_deny_all_buttons_bg' ][ 0 ] ); ?>;
                               display:none;
                               color: <?php echo esc_attr( $style_array[ 'cs_deny_all_buttons_text_color' ][ 0 ] ); ?>;
                               padding: <?php echo esc_attr( $op_btn_pd_t ); ?>px <?php echo esc_attr( $op_btn_pd_r ); ?>px <?php echo esc_attr( $op_btn_pd_b ); ?>px <?php echo esc_attr( $op_btn_pd_l ); ?>px;
                               margin: <?php echo esc_attr( $op_btn_mg_t ); ?>px <?php echo esc_attr( $op_btn_mg_r ); ?>px <?php echo esc_attr( $op_btn_mg_b ); ?>px <?php echo esc_attr( $op_btn_mg_l ); ?>px;
                               font-size: <?php echo esc_attr( $op_btn_font_s ); ?>px;
                               font-weight: <?php echo esc_attr( $op_btn_font_w ); ?>;
                               "><?php echo esc_html( $front_options[ '_cs_btn_text_disable_all_in_options_popup' ] ); ?></a>
				<?php } ?>
            </div>
        </div>

		<?php if ( ConsentMagic()->getOption( 'cs_options_verified_link' ) ) { ?>
            <div class="cs_copyright_link_wrap">
                <a href="<?php echo esc_url( 'https://www.pixelyoursite.com/plugins/consentmagic' ); ?>" rel="nofollow"
                   target="_blank"
                   class="cs_copyright_link"
                   style="color: <?php echo esc_attr( $style_array[ 'cs_subtitles_text_color' ][ 0 ] ); ?>"><?php esc_html_e( 'Verified by ConsentMagic', 'consent-magic' ); ?></a>
            </div>
		<?php } ?>
    </div>
</div>