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

$vendor_list         = CS_IAB_Integration()->get_vendor_list();
$legitimate_purposes = CS_IAB_Integration()->get_legitimate_purposes();

$unassigned             = get_term_by( 'slug', 'unassigned', 'cs-cookies-category' );
$unassigned_id          = $unassigned->term_id;
$scripts                = get_cookies_terms_objects( null, true, $unassigned_id );
$scripts_cat_by_purpose = new \stdClass;
foreach ( $vendor_list->purposes as $purpose ) {
	$scripts_cat_by_purpose->{$purpose->id} = array();
}

if ( !empty( $scripts ) ) {
	foreach ( $scripts as $script ) {
		$iab_cat = get_term_meta( $script->term_id, '_cs_iab_cat', true );

		if ( $iab_cat === '' ) {
			$iab_default_cats = CS_IAB_Integration()->get_default_script_categories();
			if ( isset( $iab_default_cats[ $script->slug ] ) ) {
				$scripts_cat_by_purpose->{$iab_default_cats[ $script->slug ]}[] = $script->term_id;
			} else {
				$scripts_cat_by_purpose->{$iab_default_cats[ 'default' ]}[] = $script->term_id;
			}

		} elseif ( $iab_cat != 0 && isset( $scripts_cat_by_purpose->{$iab_cat} ) ) {
			$scripts_cat_by_purpose->{$iab_cat}[] = $script->term_id;
		}
	}
}

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
	'_cs_consent'                                 => '',
	'_cs_btn_text_customize'                      => '',
	'_cs_iab_btn_text_allow'                      => '',
	'_cs_btn_text_sticky_cookie'                  => '',
	'_cs_text_in_consent_tab'                     => '',
	'cs_advanced_matching_description'            => 'option',
	'cs_server_side_consent_description'          => 'option',
	'cs_iab_purposes'                             => 'option',
	'cs_iab_reject_all'                           => 'option',
	'cs_iab_accept_all'                           => 'option',
	'cs_iab_legitimate_interest'                  => 'option',
	'cs_iab_examples'                             => 'option',
	'cs_iab_vendor_quantity'                      => 'option',
	'cs_iab_special_purposes'                     => 'option',
	'cs_iab_features'                             => 'option',
	'cs_iab_special_features'                     => 'option',
	'cs_iab_vendors'                              => 'option',
	'cs_iab_additional_vendors'                   => 'option',
);

$cs_user_default_language = ConsentMagic()->getOption( 'cs_user_default_language' );
$cs_language_availability = ConsentMagic()->getOption( 'cs_language_availability' );
if ( isset( $cs_language_availability[ $wp_current_lang ] ) && $cs_language_availability[ $wp_current_lang ] == 0 ) {
	$wp_current_lang = $cs_user_default_language;
}

$custom_text   = get_post_meta( $active_rule_id, '_cs_custom_text', true );
$iab_settings  = CS_IAB_Integration()->get_settings();
$vendors_count = $iab_settings->vendors_count + $iab_settings->additional_vendors_count;
$test_prefix   = ConsentMagic()->getOption( 'cs_test_mode' ) ? 'test_' : '';

$logo = wp_get_attachment_image( $style_array[ 'cs_logo' ][ 0 ], $style_array[ 'cs_logo_size' ][ 0 ], false, array(
	'style' => 'display:inline-block;'
) );

$front_options = generate_front_text( $front_options, $custom_text, $active_rule_id, $wp_current_lang, $cs_user_default_language );

$front_options[ '_cs_text_in_consent_tab' ] = str_replace( '%link_start%', '<a class="cs_open_vendors" style="color:' . esc_attr( $style_array[ 'cs_links_color' ][ 0 ] ) . '">', $front_options[ '_cs_text_in_consent_tab' ] );
$front_options[ '_cs_text_in_consent_tab' ] = str_replace( '%link_end%', '</a>', $front_options[ '_cs_text_in_consent_tab' ] );
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

        <ul class="cs_sub_tab cs_sub_tab__popup_content"
            style="border-bottom: 2px solid <?php echo esc_attr( $style_array[ 'cs_tab_buttons_bg' ][ 0 ] ); ?>">
            <li class="active" data-target="consent"
                style="
                        padding: <?php echo esc_attr( $op_btn_pd_t ); ?>px <?php echo esc_attr( $op_btn_pd_r ) * 1.33; ?>px <?php echo esc_attr( $op_btn_pd_b ); ?>px <?php echo esc_attr( $op_btn_pd_l ) * 1.33; ?>px;"
            >
                <a
                        style="color:<?php echo esc_attr( $style_array[ 'cs_tab_buttons_text_color' ][ 0 ] ); ?>;
                                font-size: <?php echo esc_attr( $op_font_s ); ?>px;
                                font-weight: <?php echo esc_attr( $op_title_f_w ); ?>;
                                ">
					<?php echo esc_html( $front_options[ '_cs_consent' ] ); ?></a>
            </li>
            <li data-target="options"
                style="
                        padding: <?php echo esc_attr( $op_btn_pd_t ); ?>px <?php echo esc_attr( $op_btn_pd_r ) * 1.33; ?>px <?php echo esc_attr( $op_btn_pd_b ); ?>px <?php echo esc_attr( $op_btn_pd_l ) * 1.33; ?>px;"
            >
                <a
                        style="color:<?php echo esc_attr( $style_array[ 'cs_tab_buttons_text_color' ][ 0 ] ); ?>;
                                font-size: <?php echo esc_attr( $op_font_s ); ?>px;
                                font-weight: <?php echo esc_attr( $op_title_f_w ); ?>;
                                ">
					<?php echo esc_html( $front_options[ '_cs_btn_text_customize' ] ); ?></a>
            </li>
        </ul>

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
             class="cs-modal-body cs-iab-enabled <?php echo ( $minimum_recommended_button ? esc_attr( 'cs-minimum-recommended' ) : '' ) . ' ' . ( $deny_all_button ? esc_attr( 'cs-deny-all' ) : '' ) . ' ' . ( $options_single_verified_link ? esc_attr( 'cs-options-single-verified-link' ) : '' ); ?>">

            <div class="cs-container-fluid cs-tab-container <?php echo esc_attr( $cs_type ) . ' ';
			echo $deny_all_button ? 'cs_deny_all_btn' : ''; ?> <?php
			echo $minimum_recommended_button ? 'cs_custom_button' : ''; ?>
                        ">
                <div class="cs_sub_tab_container" style="border: none;">
                    <div class="cs_sub_tab_content_iab consent" data-id="consent" style="display:block;">
                        <div class="cs-privacy-overview"
                             style="font-size: <?php echo esc_attr( $op_font_s ); ?>px; font-weight: <?php echo esc_attr( $op_font_w ); ?>;">
							<?php if ( $style_array[ 'cs_position_vertical_list' ][ 0 ] == 'top' && $logo ) :
								echo '<div style="margin: 0 auto 20px;text-align: ' . esc_attr( $style_array[ 'cs_position_horizontal_list' ][ 0 ] ) . ';">' . wp_kses_post($logo ) . '</div>';
							endif;
							?>
                            <h4 style="color:<?php echo esc_attr( $style_array[ 'cs_titles_text_color' ][ 0 ] ); ?>;
                                    font-size: <?php echo esc_attr( $op_title_f_s ); ?>px;
                                    font-weight: <?php echo esc_attr( $op_title_f_w ); ?>;"><?php echo esc_html( $front_options[ '_cs_text_consent' ] ); ?></h4>
                            <div class="cs-privacy-content">
                                <div class="cs-privacy-content-text"
                                     style="color:<?php echo esc_attr( $style_array[ 'cs_subtitles_text_color' ][ 0 ] ); ?>;">
									<?php
									$privacy_policy_page = renderPrivacyPolicyPage( $wp_current_lang );
                                    echo wp_kses_post( sprintf( $front_options[ '_cs_text_in_consent_tab' ], $vendors_count, $front_options[ '_cs_btn_text_customize' ], $front_options[ '_cs_btn_text_sticky_cookie' ], $front_options[ '_cs_btn_text_customize' ], $vendors_count, '<a href="' . esc_url( get_permalink( $privacy_policy_page ) ) . '" target="_blank" style="color:' . esc_attr( $style_array[ 'cs_links_color' ][ 0 ] ) . '; ">' . esc_html__( get_the_title( $privacy_policy_page ), 'consent-magic' ) . '</a>' ) ); ?>
                                </div>
                            </div>
							<?php
							if ( $style_array[ 'cs_position_vertical_list' ][ 0 ] == 'bottom' && $logo ) :
								echo '<div style="margin: 20px auto 0;text-align: ' . esc_attr( $style_array[ 'cs_position_horizontal_list' ][ 0 ] ) . ';">' . wp_kses_post( $logo ) . '</div>';
							endif;
							?>
                        </div>

                        <div class="cs_sub_tab_content_iab_wrap cs_sub_tab_content_iab_consent_tab">
							<?php if ( !empty( $vendor_list ) ) : ?>
								<?php
								if ( !empty( $vendor_list->purposes ) && $iab_settings->purposes_count > 0 ) : ?>
									<?php foreach ( $vendor_list->purposes as $purpose ):
										if ( !isset( $iab_settings->purposes->{$purpose->id} ) ) {
											$iab_settings = CS_IAB_Integration()->set_default_option( 'purposes', $purpose->id );
										}
										if ( $iab_settings->purposes->{$purpose->id} ) : ?>
                                            <div class="cs_card_single cs_card_single_first_layer">
                                                <div class="cs_card-header">
                                                    <div class="cs_card_arrow">
                                                        <i class="icon-chevron-right"></i>
                                                    </div>
                                                    <div class="cm-script-title-block-single">
                                                        <h5 style="
                                                                color:<?php echo esc_attr( $style_array[ 'cs_subtitles_text_color' ][ 0 ] ); ?>;
                                                                font-size: <?php echo esc_attr( $op_font_s ); ?>px;
                                                                font-weight: <?php echo esc_attr( $op_font_w ); ?>;
                                                                "><?php echo esc_html( $purpose->name ); ?>
                                                        </h5>
                                                    </div>
                                                </div>

                                                <div class="cs_card-body cs_card_items" style="display: none;
                                                        color:<?php echo esc_attr( $style_array[ 'cs_text_color' ][ 0 ] ); ?>;
                                                        background: <?php echo esc_attr( $style_array[ 'cs_text_block_bg' ][ 0 ] ); ?>;
                                                        ">
                                                    <div class="cs_sub_card">
                                                        <div class="cs_sub_card_body">
                                                            <div class="cs_sub_card_item_description">
																<?php echo esc_html( $purpose->description ); ?>
                                                            </div>
                                                        </div>
                                                    </div>
                                                </div>
                                            </div>
										<?php endif; ?>
									<?php endforeach; ?>
								<?php endif; ?>
							<?php else : ?>
                                <div class="cs_no_data"><?php esc_html_e( 'No Data', 'consent-magic' ) ?></div>
							<?php endif ?>
                        </div>
                    </div>

                    <div class="cs_sub_tab_content_iab options" data-id="options" style="display:none;">
                        <div class="cs_sub_tab_content_iab_wrap">
							<?php if ( !empty( $vendor_list ) ) :
								if ( !empty( $vendor_list->purposes ) && $iab_settings->purposes_count > 0 ) : ?>
                                    <div class="cs_card_single cs_card_single_second_layer" style="
                                            border: 1px solid <?php echo esc_attr( $cs_border_color ); ?>;
                                            ">
                                        <div class="cs_card-header">
                                            <div class="cs_card_arrow">
                                                <i class="icon-chevron-right"></i>
                                            </div>
                                            <div class="cm-script-title-block-single">
                                                <h5 style="
                                                        color:<?php echo esc_attr( $style_array[ 'cs_titles_text_color' ][ 0 ] ); ?>;
                                                        font-size: <?php echo esc_attr( $op_font_s ); ?>px;
                                                        font-weight: <?php echo esc_attr( $op_font_w ); ?>;
                                                        "><?php echo esc_html( $front_options[ 'cs_iab_purposes' ] ) ?>

                                                </h5>
                                                <span class="cs-card-vendor-count"
                                                      style="background: <?php echo esc_attr( $style_array[ 'cs_tab_buttons_bg' ][ 0 ] ); ?>;
                                                              font-size: <?php echo esc_attr( $op_subsubtitle_f_s ); ?>px;
                                                              font-weight: <?php echo esc_attr( $op_subtitle_f_w ); ?>;
                                                              color: <?php echo esc_attr( $style_array[ 'cs_active_toggle_color' ][ 0 ] ); ?>;
                                                              "><?php echo esc_html( $iab_settings->purposes_count ); ?></span>
                                            </div>
                                        </div>

                                        <div class="cs_card-body cs_card_items" style="display: none;
                                                color:<?php echo esc_attr( $style_array[ 'cs_text_color' ][ 0 ] ); ?>;
                                                background: <?php echo esc_attr( $style_array[ 'cs_text_block_bg' ][ 0 ] ); ?>;
                                                ">
                                            <div class="cs_sub_card">
                                                <div class="cs_sub_card_header"
                                                     style="border-bottom: 1px solid <?php echo esc_attr( $cs_text_underline_color ); ?>
                                                             ">
                                                    <div class="cs_sub_card_buttons">
                                                        <button role="button" href="#" tabindex="0"
                                                                class="btn cs_action_purposes_btn disable_all_iab"
                                                                data-cs_action="disable_all_purposes"
                                                                style="background-color: <?php echo esc_attr( $disable_all_btn_bg ); ?>;
                                                                        color: <?php echo esc_attr( $style_array[ 'cs_deny_all_buttons_text_color' ][ 0 ] ); ?>;
                                                                        padding: <?php echo esc_attr( $op_btn_pd_t ); ?>px <?php echo esc_attr( $op_btn_pd_r ); ?>px <?php echo esc_attr( $op_btn_pd_b ); ?>px <?php echo esc_attr( $op_btn_pd_l ); ?>px;
                                                                        margin: <?php echo esc_attr( $op_btn_mg_t ); ?>px <?php echo esc_attr( $op_btn_mg_r ); ?>px <?php echo esc_attr( $op_btn_mg_b ); ?>px <?php echo esc_attr( $op_btn_mg_l ); ?>px;
                                                                        font-size: <?php echo esc_attr( $op_btn_font_s ); ?>px;
                                                                        font-weight: <?php echo esc_attr( $op_btn_font_w ); ?>;
                                                                        ">
															<?php echo esc_html( $front_options[ 'cs_iab_reject_all' ] ); ?>
                                                        </button>

                                                        <button role="button" href="#" tabindex="0"
                                                                class="btn btn-grey cs_action_purposes_btn"
                                                                data-cs_action="allow_all_purposes"
                                                                style="background-color: <?php echo esc_attr( $style_array[ 'cs_accept_all_buttons_bg' ][ 0 ] ); ?>;
                                                                        color: <?php echo esc_attr( $style_array[ 'cs_accept_all_buttons_text_color' ][ 0 ] ); ?>;
                                                                        padding: <?php echo esc_attr( $op_btn_pd_t ); ?>px <?php echo esc_attr( $op_btn_pd_r ); ?>px <?php echo esc_attr( $op_btn_pd_b ); ?>px <?php echo esc_attr( $op_btn_pd_l ); ?>px;
                                                                        margin: <?php echo esc_attr( $op_btn_mg_t ); ?>px <?php echo esc_attr( $op_btn_mg_r ); ?>px <?php echo esc_attr( $op_btn_mg_b ); ?>px <?php echo esc_attr( $op_btn_mg_l ); ?>px;
                                                                        font-size: <?php echo esc_attr( $op_btn_font_s ); ?>px;
                                                                        font-weight: <?php echo esc_attr( $op_btn_font_w ); ?>;">
															<?php echo esc_html( $front_options[ 'cs_iab_accept_all' ] ); ?>
                                                        </button>
                                                    </div>
                                                </div>
                                                <div class="cs_sub_card_body">
													<?php
													foreach ( $vendor_list->purposes as $purpose ):
														if ( $iab_settings->purposes->{$purpose->id} ) :
															$dataset = array(
																'script_cat' => json_encode( $scripts_cat_by_purpose->{$purpose->id} ),
															);
															?>
                                                            <div class="cs_sub_card_item">
                                                                <div class="cs_sub_card_item_header">
                                                                    <div class="cs_sub_card-header-wrap">
                                                                        <h5 style="
                                                                                color:<?php echo esc_attr( $style_array[ 'cs_subtitles_text_color' ][ 0 ] ); ?>;
                                                                                font-size: <?php echo esc_attr( $op_subsubtitle_f_s ); ?>px;
                                                                                font-weight: <?php echo esc_attr( $op_subtitle_f_w ); ?>;
                                                                                ">
																			<?php echo esc_html( $purpose->name ); ?></h5>
                                                                    </div>

                                                                    <div class="cs_sub_card_switcher cs_sub_card-header-wrap">
																		<?php if ( in_array( $purpose->id, $legitimate_purposes ) ): ?>
                                                                            <div class="cs_iab_legitimate_switcher">
                                                                                <div class="cs_sub_card_item_sub_switcher">
                                                                                    <div class="cs_sub_card_switcher">
																						<?php render_switcher_input_front( 'cs_enable_legitimate_purpose_' . $test_prefix . esc_attr( $purpose->id ), false, false, null, false, false, false, true, '', 'cs-purposes-legitimate-input cs-iab-input', false, $dataset ); ?>
                                                                                    </div>

                                                                                    <div class="cs_switcher_label"
                                                                                         style="
                                                                                                 color:<?php echo esc_attr( $style_array[ 'cs_subtitles_text_color' ][ 0 ] ); ?>;
                                                                                                 font-size: <?php echo esc_attr( $op_subsubtitle_f_s ); ?>px;
                                                                                                 font-weight: <?php echo esc_attr( $op_subtitle_f_w ); ?>
                                                                                                 ">

																						<?php echo esc_html( $front_options[ 'cs_iab_legitimate_interest' ] ); ?>

                                                                                    </div>
                                                                                </div>
                                                                            </div>
																		<?php endif; ?>

                                                                        <div class="cs_sub_card_item_sub_switcher">
                                                                            <div class="cs_sub_card_switcher">
																				<?php render_switcher_input_front( 'cs_enable_purpose_' . $test_prefix . esc_attr( $purpose->id ), false, false, null, false, false, false, false, '', 'cs-purposes-input cs-iab-input', false, $dataset ); ?>
                                                                            </div>

                                                                            <div class="cs_switcher_label" style="
                                                                                    color:<?php echo esc_attr( $style_array[ 'cs_subtitles_text_color' ][ 0 ] ); ?>;
                                                                                    font-size: <?php echo esc_attr( $op_subsubtitle_f_s ); ?>px;
                                                                                    font-weight: <?php echo esc_attr( $op_subtitle_f_w ); ?>
                                                                                    ">
																				<?php echo esc_html( $front_options[ '_cs_iab_btn_text_allow' ] ); ?>
                                                                            </div>
                                                                        </div>
                                                                    </div>
                                                                </div>

                                                                <div class="cs_sub_card_item_description">
																	<?php echo esc_html( $purpose->description ); ?>
                                                                </div>

																<?php if ( !empty( $purpose->illustrations ) ) : ?>
                                                                    <div class="cs_sub_card_item_examples">
                                                                        <div style="
                                                                                font-size: <?php echo esc_attr( $op_subsubtitle_f_s ); ?>px;
                                                                                font-weight: <?php echo esc_attr( $op_subtitle_f_w ); ?>;
                                                                                color: <?php echo esc_attr( $style_array[ 'cs_subtitles_text_color' ][ 0 ] ); ?>;
                                                                                ">
																			<?php echo esc_html( $front_options[ 'cs_iab_examples' ] ); ?>
                                                                        </div>

                                                                        <ul>
																			<?php foreach ( $purpose->illustrations as $illustration ) : ?>
                                                                                <li style="
                                                                                        color: <?php echo esc_attr( $style_array[ 'cs_subtitles_text_color' ][ 0 ] ); ?>;
                                                                                        "><?php echo esc_html( $illustration ); ?></li>
																			<?php endforeach;
																			?>
                                                                        </ul>
                                                                    </div>
																<?php endif; ?>

                                                                <div class="cs_sub_card_item_vendor_count">
                                                                    <a class='cs_open_vendors current'
                                                                       style="
                                                                               font-size: <?php echo esc_attr( $op_second_font_s ); ?>px;
                                                                               font-weight: <?php echo esc_attr( $op_second_font_w ); ?>;
                                                                               ">
																		<?php echo esc_html( str_replace( '%', $iab_settings->quantity->purposes->{$purpose->id}, $front_options[ 'cs_iab_vendor_quantity' ] ) ); ?>
                                                                    </a>
                                                                </div>
                                                            </div>
														<?php endif; ?>
													<?php endforeach; ?>
                                                </div>
                                            </div>
                                        </div>
                                    </div>
								<?php endif; ?>

								<?php
								if ( !empty( $vendor_list->specialPurposes ) && $iab_settings->special_purposes_count > 0 ) : ?>
                                    <div class="cs_card_single cs_card_single_second_layer" style="
                                            border: 1px solid <?php echo esc_attr( $cs_border_color ); ?>;
                                            ">
                                        <div class="cs_card-header">
                                            <div class="cs_card_arrow">
                                                <i class="icon-chevron-right"></i>
                                            </div>
                                            <div class="cm-script-title-block-single">
                                                <h5 style="
                                                        color:<?php echo esc_attr( $style_array[ 'cs_titles_text_color' ][ 0 ] ); ?>;
                                                        font-size: <?php echo esc_attr( $op_font_s ); ?>px;
                                                        font-weight: <?php echo esc_attr( $op_font_w ); ?>;
                                                        "><?php echo esc_html( $front_options[ 'cs_iab_special_purposes' ] ) ?>

                                                </h5>
                                                <span class="cs-card-vendor-count"
                                                      style="background: <?php echo esc_attr( $style_array[ 'cs_tab_buttons_bg' ][ 0 ] ); ?>;
                                                              font-size: <?php echo esc_attr( $op_subsubtitle_f_s ); ?>px;
                                                              font-weight: <?php echo esc_attr( $op_subtitle_f_w ); ?>;
                                                              color: <?php echo esc_attr( $style_array[ 'cs_active_toggle_color' ][ 0 ] ); ?>;
                                                              "><?php echo esc_html( $iab_settings->special_purposes_count ); ?></span>
                                            </div>
                                        </div>

                                        <div class="cs_card-body cs_card_items" style="display: none;
                                                color:<?php echo esc_attr( $style_array[ 'cs_text_color' ][ 0 ] ); ?>;
                                                background: <?php echo esc_attr( $style_array[ 'cs_text_block_bg' ][ 0 ] ); ?>;
                                                ">
                                            <div class="cs_sub_card">
                                                <div class="cs_sub_card_body">
													<?php
													foreach ( $vendor_list->specialPurposes as $purpose ):
														if ( !isset( $iab_settings->specialPurposes->{$purpose->id} ) ) {
															$iab_settings = CS_IAB_Integration()->set_default_option( 'specialPurposes', $purpose->id );
														}
														if ( $iab_settings->specialPurposes->{$purpose->id} ) :
															?>
                                                            <div class="cs_sub_card_item">
                                                                <div class="cs_sub_card_item_header">
                                                                    <div>
                                                                        <h5 style="
                                                                                color:<?php echo esc_attr( $style_array[ 'cs_subtitles_text_color' ][ 0 ] ); ?>;
                                                                                font-size: <?php echo esc_attr( $op_subsubtitle_f_s ); ?>px;
                                                                                font-weight: <?php echo esc_attr( $op_subtitle_f_w ); ?>;
                                                                                ">
																			<?php echo esc_html( $purpose->name ); ?></h5>
                                                                    </div>
                                                                </div>

                                                                <div class="cs_sub_card_item_description">
																	<?php echo esc_html( $purpose->description ); ?>
                                                                </div>

																<?php if ( !empty( $purpose->illustrations ) ) : ?>
                                                                    <div class="cs_sub_card_item_examples">
                                                                        <div style="
                                                                                font-size: <?php echo esc_attr( $op_subsubtitle_f_s ); ?>px;
                                                                                font-weight: <?php echo esc_attr( $op_subtitle_f_w ); ?>;
                                                                                color: <?php echo esc_attr( $style_array[ 'cs_subtitles_text_color' ][ 0 ] ); ?>;
                                                                                ">
																			<?php echo esc_html( $front_options[ 'cs_iab_examples' ] ); ?>
                                                                        </div>

                                                                        <ul>
																			<?php foreach ( $purpose->illustrations as $illustration ) : ?>
                                                                                <li style="
                                                                                        color: <?php echo esc_attr( $style_array[ 'cs_subtitles_text_color' ][ 0 ] ); ?>;
                                                                                        "><?php echo esc_html( $illustration ); ?></li>
																			<?php endforeach;
																			?>
                                                                        </ul>
                                                                    </div>
																<?php endif; ?>
                                                            </div>
														<?php endif; ?>
													<?php endforeach; ?>
                                                </div>
                                            </div>
                                        </div>
                                    </div>
								<?php endif; ?>

								<?php
								if ( !empty( $vendor_list->features ) && $iab_settings->features_count > 0 ) : ?>
                                    <div class="cs_card_single cs_card_single_second_layer" style="
                                            border: 1px solid <?php echo esc_attr( $cs_border_color ); ?>;
                                            ">
                                        <div class="cs_card-header">
                                            <div class="cs_card_arrow">
                                                <i class="icon-chevron-right"></i>
                                            </div>
                                            <div class="cm-script-title-block-single">
                                                <h5 style="
                                                        color:<?php echo esc_attr( $style_array[ 'cs_titles_text_color' ][ 0 ] ); ?>;
                                                        font-size: <?php echo esc_attr( $op_font_s ); ?>px;
                                                        font-weight: <?php echo esc_attr( $op_font_w ); ?>;
                                                        "><?php echo esc_html( $front_options[ 'cs_iab_features' ] ) ?>

                                                </h5>
                                                <span class="cs-card-vendor-count"
                                                      style="background: <?php echo esc_attr( $style_array[ 'cs_tab_buttons_bg' ][ 0 ] ); ?>;
                                                              font-size: <?php echo esc_attr( $op_subsubtitle_f_s ); ?>px;
                                                              font-weight: <?php echo esc_attr( $op_subtitle_f_w ); ?>;
                                                              color: <?php echo esc_attr( $style_array[ 'cs_active_toggle_color' ][ 0 ] ); ?>;
                                                              "><?php echo esc_html( $iab_settings->features_count ); ?></span>
                                            </div>
                                        </div>

                                        <div class="cs_card-body cs_card_items" style="display: none;
                                                color:<?php echo esc_attr( $style_array[ 'cs_text_color' ][ 0 ] ); ?>;
                                                background: <?php echo esc_attr( $style_array[ 'cs_text_block_bg' ][ 0 ] ); ?>;
                                                ">
                                            <div class="cs_sub_card">
                                                <div class="cs_sub_card_body">
													<?php
													foreach ( $vendor_list->features as $feature ):
														if ( !isset( $iab_settings->features->{$feature->id} ) ) {
															$iab_settings = CS_IAB_Integration()->set_default_option( 'features', $feature->id );
														}
														if ( $iab_settings->features->{$feature->id} ) :
															?>
                                                            <div class="cs_sub_card_item">
                                                                <div class="cs_sub_card_item_header">
                                                                    <h5 style="
                                                                            color:<?php echo esc_attr( $style_array[ 'cs_subtitles_text_color' ][ 0 ] ); ?>;
                                                                            font-size: <?php echo esc_attr( $op_subsubtitle_f_s ); ?>px;
                                                                            font-weight: <?php echo esc_attr( $op_subtitle_f_w ); ?>;
                                                                            ">
																		<?php echo esc_html( $feature->name ); ?></h5>
                                                                </div>

                                                                <div class="cs_sub_card_item_description">
																	<?php echo esc_html( $feature->description ); ?>
                                                                </div>

																<?php if ( !empty( $feature->illustrations ) ) : ?>
                                                                    <div class="cs_sub_card_item_examples">
                                                                        <div style="
                                                                                font-size: <?php echo esc_attr( $op_subsubtitle_f_s ); ?>px;
                                                                                font-weight: <?php echo esc_attr( $op_subtitle_f_w ); ?>;
                                                                                color: <?php echo esc_attr( $style_array[ 'cs_subtitles_text_color' ][ 0 ] ); ?>;
                                                                                ">
																			<?php echo esc_html( $front_options[ 'cs_iab_examples' ] ); ?>
                                                                        </div>

                                                                        <ul>
																			<?php foreach ( $feature->illustrations as $illustration ) : ?>
                                                                                <li style="
                                                                                        color: <?php echo esc_attr( $style_array[ 'cs_subtitles_text_color' ][ 0 ] ); ?>;
                                                                                        "><?php echo esc_html( $illustration ); ?></li>
																			<?php endforeach;
																			?>
                                                                        </ul>
                                                                    </div>
																<?php endif; ?>
                                                            </div>
														<?php endif; ?>
													<?php endforeach; ?>
                                                </div>
                                            </div>
                                        </div>
                                    </div>
								<?php endif; ?>

								<?php
								if ( !empty( $vendor_list->specialFeatures ) && $iab_settings->special_features_count > 0 ) : ?>
                                    <div class="cs_card_single cs_card_single_second_layer" style="
                                            border: 1px solid <?php echo esc_attr( $cs_border_color ); ?>;
                                            ">
                                        <div class="cs_card-header">
                                            <div class="cs_card_arrow">
                                                <i class="icon-chevron-right"></i>
                                            </div>
                                            <div class="cm-script-title-block-single">
                                                <h5 style="
                                                        color:<?php echo esc_attr( $style_array[ 'cs_titles_text_color' ][ 0 ] ); ?>;
                                                        font-size: <?php echo esc_attr( $op_font_s ); ?>px;
                                                        font-weight: <?php echo esc_attr( $op_font_w ); ?>;
                                                        "><?php echo esc_html( $front_options[ 'cs_iab_special_features' ] ) ?>

                                                </h5>
                                                <span class="cs-card-vendor-count"
                                                      style="background: <?php echo esc_attr( $style_array[ 'cs_tab_buttons_bg' ][ 0 ] ); ?>;
                                                              font-size: <?php echo esc_attr( $op_subsubtitle_f_s ); ?>px;
                                                              font-weight: <?php echo esc_attr( $op_subtitle_f_w ); ?>;
                                                              color: <?php echo esc_attr( $style_array[ 'cs_active_toggle_color' ][ 0 ] ); ?>;
                                                              "><?php echo esc_html( $iab_settings->special_features_count ); ?></span>
                                            </div>
                                        </div>

                                        <div class="cs_card-body cs_card_items" style="display: none;
                                                color:<?php echo esc_attr( $style_array[ 'cs_text_color' ][ 0 ] ); ?>;
                                                background: <?php echo esc_attr( $style_array[ 'cs_text_block_bg' ][ 0 ] ); ?>;
                                                ">
                                            <div class="cs_sub_card">
                                                <div class="cs_sub_card_header"
                                                     style="border-bottom: 1px solid <?php echo esc_attr( $cs_text_underline_color ); ?>
                                                             ">
                                                    <div class="cs_sub_card_buttons">
                                                        <button role="button" href="#" tabindex="0"
                                                                class="btn cs_action_purposes_btn disable_all_iab"
                                                                data-cs_action="disable_all_special_features"
                                                                style="background-color: <?php echo esc_attr( $style_array[ 'cs_deny_all_buttons_bg' ][ 0 ] ); ?>;
                                                                        color: <?php echo esc_attr( $style_array[ 'cs_deny_all_buttons_text_color' ][ 0 ] ); ?>;
                                                                        padding: <?php echo esc_attr( $op_btn_pd_t ); ?>px <?php echo esc_attr( $op_btn_pd_r ); ?>px <?php echo esc_attr( $op_btn_pd_b ); ?>px <?php echo esc_attr( $op_btn_pd_l ); ?>px;
                                                                        margin: <?php echo esc_attr( $op_btn_mg_t ); ?>px <?php echo esc_attr( $op_btn_mg_r ); ?>px <?php echo esc_attr( $op_btn_mg_b ); ?>px <?php echo esc_attr( $op_btn_mg_l ); ?>px;
                                                                        font-size: <?php echo esc_attr( $op_btn_font_s ); ?>px;
                                                                        font-weight: <?php echo esc_attr( $op_btn_font_w ); ?>;
                                                                        ">
															<?php echo esc_html( $front_options[ 'cs_iab_reject_all' ] ); ?>
                                                        </button>

                                                        <button role="button" href="#" tabindex="0"
                                                                class="btn btn-grey cs_action_purposes_btn"
                                                                data-cs_action="allow_all_special_features"
                                                                style="background-color: <?php echo esc_attr( $style_array[ 'cs_accept_all_buttons_bg' ][ 0 ] ); ?>;
                                                                        color: <?php echo esc_attr( $style_array[ 'cs_accept_all_buttons_text_color' ][ 0 ] ); ?>;
                                                                        padding: <?php echo esc_attr( $op_btn_pd_t ); ?>px <?php echo esc_attr( $op_btn_pd_r ); ?>px <?php echo esc_attr( $op_btn_pd_b ); ?>px <?php echo esc_attr( $op_btn_pd_l ); ?>px;
                                                                        margin: <?php echo esc_attr( $op_btn_mg_t ); ?>px <?php echo esc_attr( $op_btn_mg_r ); ?>px <?php echo esc_attr( $op_btn_mg_b ); ?>px <?php echo esc_attr( $op_btn_mg_l ); ?>px;
                                                                        font-size: <?php echo esc_attr( $op_btn_font_s ); ?>px;
                                                                        font-weight: <?php echo esc_attr( $op_btn_font_w ); ?>;">
															<?php echo esc_html( $front_options[ 'cs_iab_accept_all' ] ); ?>
                                                        </button>

                                                    </div>
                                                </div>
                                                <div class="cs_sub_card_body">
													<?php
													foreach ( $vendor_list->specialFeatures as $feature ):
														if ( !isset( $iab_settings->specialFeatures->{$feature->id} ) ) {
															$iab_settings = CS_IAB_Integration()->set_default_option( 'specialFeatures', $feature->id );
														}
														if ( $iab_settings->specialFeatures->{$feature->id} ) :
															?>
                                                            <div class="cs_sub_card_item">
                                                                <div class="cs_sub_card_item_header">
                                                                    <div class="cs_sub_card_item_header_inner">
                                                                        <h5 style="
                                                                                color:<?php echo esc_attr( $style_array[ 'cs_subtitles_text_color' ][ 0 ] ); ?>;
                                                                                font-size: <?php echo esc_attr( $op_subsubtitle_f_s ); ?>px;
                                                                                font-weight: <?php echo esc_attr( $op_subtitle_f_w ); ?>;
                                                                                ">
																			<?php echo esc_html( $feature->name ); ?>
                                                                        </h5>

                                                                        <div class="cs_sub_card_switcher">
		                                                                    <?php render_switcher_input_front( 'cs_enable_special_feature_' . $test_prefix . esc_attr( $feature->id ), false, false, null, false, false, false, false, '', 'cs-special-feature-input cs-iab-input', false ); ?>
                                                                        </div>
                                                                    </div>
                                                                </div>

                                                                <div class="cs_sub_card_item_description">
																	<?php echo esc_html( $feature->description ); ?>
                                                                </div>

																<?php if ( !empty( $feature->illustrations ) ) : ?>
                                                                    <div class="cs_sub_card_item_examples">
                                                                        <div style="
                                                                                font-size: <?php echo esc_attr( $op_subsubtitle_f_s ); ?>px;
                                                                                font-weight: <?php echo esc_attr( $op_subtitle_f_w ); ?>;
                                                                                color: <?php echo esc_attr( $style_array[ 'cs_subtitles_text_color' ][ 0 ] ); ?>;
                                                                                ">
																			<?php echo esc_html( $front_options[ 'cs_iab_examples' ] ); ?>
                                                                        </div>

                                                                        <ul>
																			<?php foreach ( $feature->illustrations as $illustration ) : ?>
                                                                                <li style="
                                                                                        color: <?php echo esc_attr( $style_array[ 'cs_subtitles_text_color' ][ 0 ] ); ?>;
                                                                                        "><?php echo esc_html( $illustration ); ?></li>
																			<?php endforeach;
																			?>
                                                                        </ul>
                                                                    </div>
																<?php endif; ?>

                                                                <div class="cs_sub_card_item_vendor_count">
                                                                    <a class='cs_open_vendors current'
                                                                       style="
                                                                               font-size: <?php echo esc_attr( $op_second_font_s ); ?>px;
                                                                               font-weight: <?php echo esc_attr( $op_second_font_w ); ?>;
                                                                               ">
																		<?php echo esc_html( str_replace( '%', $iab_settings->quantity->specialFeatures->{$feature->id}, $front_options[ 'cs_iab_vendor_quantity' ] ) ); ?>
                                                                    </a>
                                                                </div>
                                                            </div>
														<?php endif; ?>
													<?php endforeach; ?>
                                                </div>
                                            </div>
                                        </div>
                                    </div>
								<?php endif; ?>

								<?php
								if ( ( !empty( $vendor_list->vendors ) && $iab_settings->vendors_count > 0 ) || ( !empty( $vendor_list->additional_vendors ) && $iab_settings->additional_vendors_count > 0 ) ) : ?>
                                    <div class="cs_card_single cs_card_single_second_layer" style="
                                            border: 1px solid <?php echo esc_attr( $cs_border_color ); ?>;
                                            ">
                                        <div class="cs_card-header" id="cs_card-header-vendors">
                                            <div class="cs_card_arrow">
                                                <i class="icon-chevron-right"></i>
                                            </div>
                                            <div class="cm-script-title-block-single">
                                                <h5 style="
                                                        color:<?php echo esc_attr( $style_array[ 'cs_titles_text_color' ][ 0 ] ); ?>;
                                                        font-size: <?php echo esc_attr( $op_font_s ); ?>px;
                                                        font-weight: <?php echo esc_attr( $op_font_w ); ?>;
                                                        "><?php echo esc_html( $front_options[ 'cs_iab_vendors' ] ) ?>

                                                </h5>
                                                <span class="cs-card-vendor-count"
                                                      style="background: <?php echo esc_attr( $style_array[ 'cs_tab_buttons_bg' ][ 0 ] ); ?>;
                                                              font-size: <?php echo esc_attr( $op_subsubtitle_f_s ); ?>px;
                                                              font-weight: <?php echo esc_attr( $op_subtitle_f_w ); ?>;
                                                              color: <?php echo esc_attr( $style_array[ 'cs_active_toggle_color' ][ 0 ] ); ?>;
                                                              "><?php echo esc_html( $vendors_count ); ?></span>
                                            </div>
                                        </div>

                                        <div class="cs_card-body cs_card_items cs_card_items_vendors"
                                             style="display:none">
                                            <ul class="cs_sub_sub_tab cs_sub_sub_tab__popup_content"
                                                style="border-bottom: 2px solid <?php echo esc_attr( $style_array[ 'cs_tab_buttons_bg' ][ 0 ] ); ?>">
												<?php if ( ( !empty( $vendor_list->vendors ) && $iab_settings->vendors_count > 0 ) ) : ?>
                                                    <li class="active" data-target="vendors"
                                                        style="padding: <?php echo esc_attr( $op_btn_pd_t ); ?>px <?php echo esc_attr( $op_btn_pd_r ) * 1.33; ?>px <?php echo esc_attr( $op_btn_pd_b ); ?>px <?php echo esc_attr( $op_btn_pd_l ) * 1.33; ?>px;">
                                                        <a style="color:<?php echo esc_attr( $style_array[ 'cs_tab_buttons_text_color' ][ 0 ] ); ?>;
                                                                font-size: <?php echo esc_attr( $op_font_s ); ?>px;
                                                                font-weight: <?php echo esc_attr( $op_title_f_w ); ?>;
                                                                ">
															<?php echo esc_html( sprintf( '%s (%s)', $front_options[ 'cs_iab_vendors' ], $iab_settings->vendors_count ) ); ?></h5></a>
                                                    </li>
												<?php endif; ?>

												<?php if ( ( !empty( $vendor_list->additional_vendors ) && $iab_settings->additional_vendors_count > 0 ) ) : ?>
                                                    <li data-target="additional_vendors"
                                                        style="padding: <?php echo esc_attr( $op_btn_pd_t ); ?>px <?php echo esc_attr( $op_btn_pd_r ) * 1.33; ?>px <?php echo esc_attr( $op_btn_pd_b ); ?>px <?php echo esc_attr( $op_btn_pd_l ) * 1.33; ?>px;">
                                                        <a style="color:<?php echo esc_attr( $style_array[ 'cs_tab_buttons_text_color' ][ 0 ] ); ?>;
                                                                font-size: <?php echo esc_attr( $op_font_s ); ?>px;
                                                                font-weight: <?php echo esc_attr( $op_title_f_w ); ?>;
                                                                ">
															<?php echo esc_html( sprintf( '%s (%s)', $front_options[ 'cs_iab_additional_vendors' ], $iab_settings->additional_vendors_count ) ); ?></h5></a>
                                                    </li>
												<?php endif; ?>
                                            </ul>

                                            <div class="cs_sub_tab_container">
												<?php if ( ( !empty( $vendor_list->vendors ) && $iab_settings->vendors_count > 0 ) ) : ?>
                                                    <div class="cs_sub_sub_tab_content_iab"
                                                         data-id="vendors"
                                                         style="display:block;">
                                                        <div class="cs_sub-card-body cs_card_items cs_vendor_list"
                                                             style="
                                                                     color:<?php echo esc_attr( $style_array[ 'cs_text_color' ][ 0 ] ); ?>;
                                                                     ">
                                                            <div class="cs_sub_card">
																<?php if ( !is_admin() || ( wp_doing_ajax() && !isset( $cs_preview_popup_admin ) ) ) : ?>

                                                                    <div class="cs_sub_card_header">
                                                                        <div class="cs_sub_card_buttons">
                                                                            <button role="button" href="#"
                                                                                    tabindex="0"
                                                                                    class="btn cs_action_purposes_btn disable_all_iab disable_all_border"
                                                                                    data-cs_action="disable_all_vendors"
                                                                                    style="background-color: <?php echo esc_attr( $disable_all_btn_bg ); ?>;
                                                                                            color: <?php echo esc_attr( $style_array[ 'cs_deny_all_buttons_text_color' ][ 0 ] ); ?>;
                                                                                            padding: <?php echo esc_attr( $op_btn_pd_t ); ?>px <?php echo esc_attr( $op_btn_pd_r ); ?>px <?php echo esc_attr( $op_btn_pd_b ); ?>px <?php echo esc_attr( $op_btn_pd_l ); ?>px;
                                                                                            margin: <?php echo esc_attr( $op_btn_mg_t ); ?>px <?php echo esc_attr( $op_btn_mg_r ); ?>px <?php echo esc_attr( $op_btn_mg_b ); ?>px <?php echo esc_attr( $op_btn_mg_l ); ?>px;
                                                                                            font-size: <?php echo esc_attr( $op_btn_font_s ); ?>px;
                                                                                            font-weight: <?php echo esc_attr( $op_btn_font_w ); ?>;
                                                                                            ">
																				<?php echo esc_html( $front_options[ 'cs_iab_reject_all' ] ); ?>
                                                                            </button>
                                                                            <button role="button" href="#"
                                                                                    tabindex="0"
                                                                                    class="btn btn-grey cs_action_purposes_btn"
                                                                                    data-cs_action="allow_all_vendors"
                                                                                    style="background-color: <?php echo esc_attr( $style_array[ 'cs_accept_all_buttons_bg' ][ 0 ] ); ?>;
                                                                                            color: <?php echo esc_attr( $style_array[ 'cs_accept_all_buttons_text_color' ][ 0 ] ); ?>;
                                                                                            padding: <?php echo esc_attr( $op_btn_pd_t ); ?>px <?php echo esc_attr( $op_btn_pd_r ); ?>px <?php echo esc_attr( $op_btn_pd_b ); ?>px <?php echo esc_attr( $op_btn_pd_l ); ?>px;
                                                                                            margin: <?php echo esc_attr( $op_btn_mg_t ); ?>px <?php echo esc_attr( $op_btn_mg_r ); ?>px <?php echo esc_attr( $op_btn_mg_b ); ?>px <?php echo esc_attr( $op_btn_mg_l ); ?>px;
                                                                                            font-size: <?php echo esc_attr( $op_btn_font_s ); ?>px;
                                                                                            font-weight: <?php echo esc_attr( $op_btn_font_w ); ?>;">
																				<?php echo esc_html( $front_options[ 'cs_iab_accept_all' ] ); ?>
                                                                            </button>
                                                                        </div>
                                                                    </div>

                                                                    <div class="line"></div>

                                                                    <div class="cs_sub_card_body cs_iab_container">
                                                                    </div>

																<?php else : ?>
                                                                    <div class="cs_vendor_notice"><?php esc_html_e( 'Vendor list (not available in the admin panel)', 'consent-magic' ); ?></div>
																<?php endif; ?>
                                                            </div>
                                                        </div>

                                                    </div>
												<?php endif; ?>

												<?php if ( ( !empty( $vendor_list->additional_vendors ) && $iab_settings->additional_vendors_count > 0 ) ) : ?>
                                                    <div class="cs_sub_sub_tab_content_iab"
                                                         data-id="additional_vendors">
                                                        <div class="cs_sub-card-body cs_card_items cs_additional_vendor_list"
                                                             style="color:<?php echo esc_attr( $style_array[ 'cs_text_color' ][ 0 ] ); ?>;
                                                                     ">
                                                            <div class="cs_sub_card">
																<?php if ( !is_admin() || ( wp_doing_ajax() && !isset( $cs_preview_popup_admin ) ) ) : ?>

																	<?php if ( !empty( $vendor_list->additional_vendors ) ) : ?>
                                                                        <div class="cs_sub_card_header">
                                                                            <div class="cs_sub_card_buttons">
                                                                                <button role="button"
                                                                                        href="#"
                                                                                        tabindex="0"
                                                                                        class="btn cs_action_purposes_btn disable_all_iab disable_all_border"
                                                                                        data-cs_action="disable_all_additional_vendors"
                                                                                        style="background-color: <?php echo esc_attr( $disable_all_btn_bg ); ?>;
                                                                                                color: <?php echo esc_attr( $style_array[ 'cs_deny_all_buttons_text_color' ][ 0 ] ); ?>;
                                                                                                padding: <?php echo esc_attr( $op_btn_pd_t ); ?>px <?php echo esc_attr( $op_btn_pd_r ); ?>px <?php echo esc_attr( $op_btn_pd_b ); ?>px <?php echo esc_attr( $op_btn_pd_l ); ?>px;
                                                                                                margin: <?php echo esc_attr( $op_btn_mg_t ); ?>px <?php echo esc_attr( $op_btn_mg_r ); ?>px <?php echo esc_attr( $op_btn_mg_b ); ?>px <?php echo esc_attr( $op_btn_mg_l ); ?>px;
                                                                                                font-size: <?php echo esc_attr( $op_btn_font_s ); ?>px;
                                                                                                font-weight: <?php echo esc_attr( $op_btn_font_w ); ?>;
                                                                                                ">
																					<?php echo esc_html( $front_options[ 'cs_iab_reject_all' ] ); ?>
                                                                                </button>

                                                                                <button role="button"
                                                                                        href="#"
                                                                                        tabindex="0"
                                                                                        class="btn btn-grey cs_action_purposes_btn"
                                                                                        data-cs_action="allow_all_additional_vendors"
                                                                                        style="background-color: <?php echo esc_attr( $style_array[ 'cs_accept_all_buttons_bg' ][ 0 ] ); ?>;
                                                                                                color: <?php echo esc_attr( $style_array[ 'cs_accept_all_buttons_text_color' ][ 0 ] ); ?>;
                                                                                                padding: <?php echo esc_attr( $op_btn_pd_t ); ?>px <?php echo esc_attr( $op_btn_pd_r ); ?>px <?php echo esc_attr( $op_btn_pd_b ); ?>px <?php echo esc_attr( $op_btn_pd_l ); ?>px;
                                                                                                margin: <?php echo esc_attr( $op_btn_mg_t ); ?>px <?php echo esc_attr( $op_btn_mg_r ); ?>px <?php echo esc_attr( $op_btn_mg_b ); ?>px <?php echo esc_attr( $op_btn_mg_l ); ?>px;
                                                                                                font-size: <?php echo esc_attr( $op_btn_font_s ); ?>px;
                                                                                                font-weight: <?php echo esc_attr( $op_btn_font_w ); ?>;">
																					<?php echo esc_html( $front_options[ 'cs_iab_accept_all' ] ); ?>
                                                                                </button>
                                                                            </div>
                                                                        </div>
																	<?php endif; ?>

                                                                    <div class="cs_sub_card_body cs_iab_container_additional">
                                                                    </div>

                                                                    <div class="line"></div>

																<?php else : ?>
                                                                    <div class="cs_vendor_notice"><?php esc_html_e( 'Additional Vendor list (not available in the admin panel)', 'consent-magic' ); ?></div>
																<?php endif; ?>
                                                            </div>
                                                        </div>
                                                    </div>
												<?php endif; ?>
                                            </div>
                                        </div>
                                    </div>
								<?php endif; ?>
							<?php else : ?>
                                <div class="cs_no_data"><?php esc_html_e( 'No Data', 'consent-magic' ) ?></div>
							<?php endif ?>

                            <div style="display: none;">
								<?php
								if ( $terms_primary ) {
									foreach ( $terms_primary as $term ) { ?>
										<?php if ( (int) get_term_meta( $term->term_id, 'cs_ignore_this_category', true ) == 0 ) {
											if ( isPYSActivated() ) {
												if ( $fb_cat == $term->term_id ) {
													$disabled = ( ( CS_Cookies()->getCookie( 'cs_enabled_cookie_term_' . $term->term_id ) && CS_Cookies()->getCookie( 'cs_enabled_cookie_term_' . $term->term_id ) == 'no' ) || !CS_Cookies()->getCookie( 'cs_viewed_cookie_policy' ) && $cs_type == 'inform_and_opiout' ) ? true : false;
													if ( ConsentMagic()->getOption( 'cs_test_mode' ) ) {
														$disabled = ( ( CS_Cookies()->getCookie( 'cs_enabled_cookie_term_test_' . $term->term_id ) && CS_Cookies()->getCookie( 'cs_enabled_cookie_term_test_' . $term->term_id ) == 'no' ) || !CS_Cookies()->getCookie( 'cs_enabled_cookie_term_test_' . $term->term_id ) && $cs_type == 'inform_and_opiout' ) ? true : false;
													}

													if ( ConsentMagic()->getOption( 'cs_advanced_matching_consent_enabled' ) ) { ?>
														<?php render_switcher_input_front( ConsentMagic()->getOption( 'cs_test_mode' ) ? 'cs_enabled_advanced_matching_test' : 'cs_enabled_advanced_matching', false, false, null, false, $disabled ); ?>
													<?php } ?>

													<?php if ( ConsentMagic()->getOption( 'cs_server_side_consent_enabled' ) ) { ?>
														<?php render_switcher_input_front( ConsentMagic()->getOption( 'cs_test_mode' ) ? 'cs_enabled_server_side_test' : 'cs_enabled_server_side', false, false, null, false, $disabled ); ?>
														<?php
													}
												}
											}
										}
									}
								} ?>

								<?php
								if ( $terms ) {
									foreach ( $terms as $term ) { ?>
										<?php if ( (int) get_term_meta( $term->term_id, 'cs_ignore_this_category', true ) == 0 ) { ?>
                                            <div class="<?php echo ( $fb_cat == $term->term_id ) ? 'facebook_term_group' : ''; ?>">
												<?php render_switcher_input_front( ConsentMagic()->getOption( 'cs_test_mode' ) ? 'cs_enabled_cookie_term_test_' . $term->term_id : 'cs_enabled_cookie_term_' . $term->term_id, false ); ?>
                                            </div>
											<?php
											if ( isPYSActivated() ) {
												if ( $fb_cat == $term->term_id ) {
													$disabled = ( ( CS_Cookies()->getCookie( 'cs_enabled_cookie_term_' . $term->term_id ) && CS_Cookies()->getCookie( 'cs_enabled_cookie_term_' . $term->term_id ) == 'no' ) || !CS_Cookies()->getCookie( 'cs_viewed_cookie_policy' ) && $cs_type == 'inform_and_opiout' ) ? true : false;
													if ( ConsentMagic()->getOption( 'cs_test_mode' ) ) {
														$disabled = ( ( CS_Cookies()->getCookie( 'cs_enabled_cookie_term_test_' . $term->term_id ) && CS_Cookies()->getCookie( 'cs_enabled_cookie_term_test_' . $term->term_id ) == 'no' ) || !CS_Cookies()->getCookie( 'CS-Magic_test' ) && $cs_type == 'inform_and_opiout' ) ? true : false;
													}
													?>

													<?php if ( ConsentMagic()->getOption( 'cs_advanced_matching_consent_enabled' ) ) { ?>
														<?php render_switcher_input_front( ConsentMagic()->getOption( 'cs_test_mode' ) ? 'cs_enabled_advanced_matching_test' : 'cs_enabled_advanced_matching', false, false, null, false, $disabled ); ?>
													<?php } ?>

													<?php if ( ConsentMagic()->getOption( 'cs_server_side_consent_enabled' ) ) { ?>
														<?php render_switcher_input_front( ConsentMagic()->getOption( 'cs_test_mode' ) ? 'cs_enabled_server_side_test' : 'cs_enabled_server_side', false, false, null, false, $disabled ); ?>
														<?php
													}
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

            <!-- Container for vendors -->
            <div class="cs_sub_sub_card cs_iab_vendor_container" style="display: none">
                <div class="cs_sub_card-header">
                    <div class="cs_sub_card-header-item">
                        <div class="cs_sub_card-header-wrap">
                            <div class="cs_sub_card-header-name">
                                <div class="cs_card_arrow">
                                    <i class="icon-chevron-right"></i>
                                </div>

                                <h5 class="cs_iab_vendor_name"></h5>
                            </div>
                        </div>

                        <div class="cs_sub_card_switcher cs_sub_card-header-wrap">
                            <div class="cs_iab_legitimate_switcher"></div>
                            <div class="cs_sub_card_switcher cs_iab_vendor_switcher"></div>
                        </div>

                        <div class="cs_sub_sub_card-body cs_card_items cs_vendor_properties"
                             style="display: none"></div>
                    </div>
                </div>
            </div>
        </div>

        <div class="cs-bottom-buttons-row">
			<?php
			include CMPRO_PLUGIN_VIEWS_PATH . 'templates/buttons/cs_bottom_buttons_single.php'; ?>
        </div>
    </div>
</div>

