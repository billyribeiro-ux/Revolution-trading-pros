<?php

namespace ConsentMagicPro;

// If this file is called directly, abort.
if ( !defined( 'WPINC' ) ) {
	die;
}

if ( $id ) {
	$cs_theme_slug = get_post( $id )->post_name;
} else {
	$cs_theme_slug = '';
}

if ( !isset( $style_array ) ) {
	$style_array = get_post_meta( $id );
}

$privacy_link = get_post_meta( $active_rule_id, '_cs_privacy_link', true );

if ( !isset( $cs_preview_size_show ) ) {
	$prefix = 'cs_d_sp_';

	if ( $cs_bars_type == 'popup_large' ) {
		$prefix = 'cs_d_lp_';
	}
	$min_height = ConsentMagic()->getOption( $prefix . 'min_height' );
	$pd_top     = ConsentMagic()->getOption( $prefix . 'padding_top' );
	$pd_bottom  = ConsentMagic()->getOption( $prefix . 'padding_bottom' );
	$pd_left    = ConsentMagic()->getOption( $prefix . 'padding_left' );
	$pd_right   = ConsentMagic()->getOption( $prefix . 'padding_right' );

	$font_s = ConsentMagic()->getOption( $prefix . 'font_size' );
	$font_w = ConsentMagic()->getOption( $prefix . 'font_weight' );

	$btn_pd_t   = ConsentMagic()->getOption( $prefix . 'btn_pd_t' );
	$btn_pd_b   = ConsentMagic()->getOption( $prefix . 'btn_pd_bt' );
	$btn_pd_l   = ConsentMagic()->getOption( $prefix . 'btn_pd_l' );
	$btn_pd_r   = ConsentMagic()->getOption( $prefix . 'btn_pd_r' );
	$btn_mg_t   = ConsentMagic()->getOption( $prefix . 'btn_mg_t' );
	$btn_mg_b   = ConsentMagic()->getOption( $prefix . 'btn_mg_bt' );
	$btn_mg_l   = ConsentMagic()->getOption( $prefix . 'btn_mg_l' );
	$btn_mg_r   = ConsentMagic()->getOption( $prefix . 'btn_mg_r' );
	$btn_font_s = ConsentMagic()->getOption( $prefix . 'btn_font_size' );
	$btn_font_w = ConsentMagic()->getOption( $prefix . 'btn_font_weight' );

	$op_min_height = ConsentMagic()->getOption( 'cs_d_op_min_height' );
	$op_pd_top     = ConsentMagic()->getOption( 'cs_d_op_padding_top' );
	$op_pd_bottom  = ConsentMagic()->getOption( 'cs_d_op_padding_bottom' );
	$op_pd_left    = ConsentMagic()->getOption( 'cs_d_op_padding_left' );
	$op_pd_right   = ConsentMagic()->getOption( 'cs_d_op_padding_right' );

	$op_font_s = ConsentMagic()->getOption( 'cs_d_op_font_size' );
	$op_font_w = ConsentMagic()->getOption( 'cs_d_op_font_weight' );

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

$logo = wp_get_attachment_image( $style_array[ 'cs_logo' ][ 0 ], $style_array[ 'cs_logo_size' ][ 0 ], false, array(
	'style' => 'display:inline-block;'
) );

$front_options = array(
	'_cs_text_in_small_bar_popup' => '',
	'_cs_text_in_large_bar_popup' => '',
	'_cs_btn_text_customize'      => '',
	'_cs_btn_text_disable_all'    => '',
	'_cs_btn_text_allow_all'      => '',
	'_cs_btn_text_custom_button'  => '',
);

$wp_current_lang          = ( isset( $this->current_lang ) && $this->current_lang ) ? $this->current_lang : get_locale();
$cs_user_default_language = ConsentMagic()->getOption( 'cs_user_default_language' );
$cs_language_availability = ConsentMagic()->getOption( 'cs_language_availability' );
if ( isset( $cs_language_availability[ $wp_current_lang ] ) && $cs_language_availability[ $wp_current_lang ] == 0 ) {
	$wp_current_lang = $cs_user_default_language;
}

$custom_text = get_post_meta( $active_rule_id, '_cs_custom_text', true );
$cs_type     = get_post_meta( $active_rule_id, '_cs_type', true );

$cs_active_toggle_color_label = $style_array[ 'cs_active_toggle_color' ][ 0 ] . '40';
$cs_text_underline_color      = $style_array[ 'cs_subtitles_text_color' ][ 0 ] . '80';

$additional_styles = '--cs-color-active-toggle: ' . esc_attr( $cs_active_toggle_color_label );
$additional_styles .= '; --cs-color-active-toggle-button: ' . esc_attr( $style_array[ 'cs_active_toggle_color' ][ 0 ] );
$additional_styles .= '; --cs-active-toggle-background-color: ' . $style_array[ 'cs_active_toggle_color' ][ 0 ];
$additional_styles .= '; --cs-active-toggle-text-color: ' . $style_array[ 'cs_active_toggle_text_color' ][ 0 ];
$additional_styles .= '; --cs-text-underline-color: ' . $cs_text_underline_color;
$additional_styles .= '; --cs-text-color: ' . $style_array[ 'cs_text_color' ][ 0 ];
$additional_styles .= '; --cs-color-underline: ' . $style_array[ 'cs_links_color' ][ 0 ];
$additional_styles .= '; --cs_subtitles_text_color: ' . $style_array[ 'cs_subtitles_text_color' ][ 0 ];
$additional_styles .= '; --cs_op_subtitle_f_w: ' . $op_subtitle_f_w;
$additional_styles .= '; --cs_tab_buttons_bg: ' . $style_array[ 'cs_tab_buttons_bg' ][ 0 ];

$minimum_recommended_button = !empty( $cs_custom_button_btn ) || !empty( $style_array[ '_cs_custom_button' ][ 0 ] );
$deny_all_button            = !empty( $cs_deny_all_btn ) || !empty( $style_array[ '_cs_deny_all_btn' ][ 0 ] );

$front_options = generate_front_text( $front_options, $custom_text, $active_rule_id, $wp_current_lang, $cs_user_default_language );

// Disable indexing of ConsentMagic Cookie data
echo "<!--googleoff: all-->";

?>
    <div class="cs_popup_overlay"></div>
    <div class="<?php echo esc_attr( ConsentMagic()->get_containers_names()[ 'cs_cookie_bar_container' ] ); ?>"
         style="<?php echo esc_attr( $additional_styles ); ?>"
    >
        <div data-cli-geo-loc=""
             class="cs-info-bar cs-public-cookie-popup <?php echo esc_attr( $cs_bars_type ) . ' cs_popup ' . esc_attr( $cs_bars_position ) . ' ' . esc_attr( $cs_theme_slug ); ?>"
             style="background-color: <?php echo esc_attr( $style_array[ 'cs_backend_color' ][ 0 ] ); ?>;
                     border: <?php echo esc_attr( $style_array[ 'cs_border_weight' ][ 0 ] ) . 'px ' . esc_attr( $style_array[ 'cs_border_style' ][ 0 ] ) . ' ' . esc_attr( $style_array[ 'cs_border_color' ][ 0 ] ); ?>;
                     color: <?php echo esc_attr( $style_array[ 'cs_text_color' ][ 0 ] ); ?>;
                     fill: <?php echo esc_attr( $style_array[ 'cs_text_color' ][ 0 ] ); ?>;
                     padding: <?php echo esc_attr( $pd_top ) . 'px ' . esc_attr( $pd_right ) . 'px ' . esc_attr( $pd_bottom ) . 'px ' . esc_attr( $pd_left ) . 'px'; ?>;
                     font-size: <?php echo esc_attr( $font_s ); ?>px;
                     font-weight: <?php echo esc_attr( $font_w ); ?>;
                     min-height: <?php echo esc_attr( $min_height ); ?>px;">

			<?php if ( ( isset( $cs_hide_close_btn ) && $cs_hide_close_btn == '0' ) || $cs_type == 'just_inform' ) { ?>
                <button type="button"
                        class="cs-modal-close <?php echo isset( $cs_hide_close_btn_admin ) ? 'cs-modal-close-admin' : ''; ?>"
                        data-action="cs_close_consent"
                        style="--cs-text-color: <?php esc_html_e( $style_array[ 'cs_text_color' ][ 0 ] ); ?>">
                    <svg viewBox="0 0 24 24">
                        <path d="M19 6.41l-1.41-1.41-5.59 5.59-5.59-5.59-1.41 1.41 5.59 5.59-5.59 5.59 1.41 1.41 5.59-5.59 5.59 5.59 1.41-1.41-5.59-5.59z"></path>
                        <path d="M0 0h24v24h-24z" fill="none"></path>
                    </svg>
                    <span class="cs-sr-only"><?php esc_html_e( 'Close', 'consent-magic' ); ?></span>
                </button>
			<?php } ?>

            <div class="cs-wrapper">
				<?php
				if ( $style_array[ 'cs_position_vertical_list' ][ 0 ] == 'top' && $logo ) {
					echo '<div style="margin: 0 auto 20px; text-align: ' . esc_attr( $style_array[ 'cs_position_horizontal_list' ][ 0 ] ) . ';">' . wp_kses_post( $logo ) . '</div>';
				}
				?>

                <div>
                    <div>
						<?php
						if ( $cs_bars_type == 'popup_small' ) : ?>
                            <div class="bar_description_text"
                                 style="color:<?php echo esc_attr( $style_array[ 'cs_text_color' ][ 0 ] ); ?>;
                                         font-size: <?php echo esc_attr( $font_s ); ?>px;
                                         font-weight: <?php echo esc_attr( $font_w ); ?>;">
								<?php echo wp_kses_post( $front_options[ '_cs_text_in_small_bar_popup' ] ); ?>
                            </div>
						<?php elseif ( $cs_bars_type == 'popup_large' ) : ?>
                            <div class="bar_description_text"
                                 style="color:<?php echo esc_attr( $style_array[ 'cs_text_color' ][ 0 ] ); ?>;
                                         font-size: <?php echo esc_attr( $font_s ); ?>px;
                                         font-weight: <?php echo esc_attr( $font_w ); ?>;">
								<?php echo wp_kses_post( $front_options[ '_cs_text_in_large_bar_popup' ] ); ?>
                            </div>
						<?php endif; ?>

						<?php
                        $privacy_policy_page = renderPrivacyPolicyPage( $wp_current_lang );
                        if ( $privacy_policy_page ) {
							echo '<div class="policy_wrap"><a href="' . esc_url( get_permalink( $privacy_policy_page ) ) . '" target="_blank" style="color:' . esc_attr( $style_array[ 'cs_links_color' ][ 0 ] ) . ';">' . esc_html__( get_the_title( $privacy_policy_page ), 'consent-magic' ) . '</a></div>';
						}
						?>
                    </div>

                    <div class="btns_column <?php echo $deny_all_button ? 'cs_deny_all_btn' : ''; ?> <?php
					echo $minimum_recommended_button ? 'cs_custom_button' : ''; ?>">
                        <div class="btn-row">
							<?php if ( $cs_type !== 'just_inform' ) { ?>
                                <button type="button" class="btn options_btn"
                                        style="background-color: <?php echo esc_attr( $style_array[ 'cs_options_buttons_bg' ][ 0 ] ); ?>;
                                                color: <?php echo esc_attr( $style_array[ 'cs_options_buttons_text_color' ][ 0 ] ); ?>;
                                                padding: <?php echo esc_attr( $btn_pd_t ); ?>px <?php echo esc_attr( $btn_pd_r ); ?>px <?php echo esc_attr( $btn_pd_b ); ?>px <?php echo esc_attr( $btn_pd_l ); ?>px;
								        <?php if ( $cs_bars_type == 'popup_large' ) : ?>
                                                margin: <?php echo esc_attr( $btn_mg_t ); ?>px <?php echo esc_attr( $btn_mg_r ); ?>px <?php echo esc_attr( $btn_mg_b ); ?>px <?php echo esc_attr( $btn_mg_l ); ?>px;
								        <?php else: ?>
                                                margin: <?php echo esc_attr( $btn_mg_t ); ?>px 0 <?php echo esc_attr( $btn_mg_b ); ?>px 0;
								        <?php endif; ?>
                                                font-size: <?php echo esc_attr( $btn_font_s ); ?>px;
                                                font-weight: <?php echo esc_attr( $btn_font_w ); ?>;"><?php echo esc_html( $front_options[ '_cs_btn_text_customize' ] ); ?></button>
								<?php if ( $deny_all_button ) { ?>
                                    <button type="button" class="btn disable_all_btn cs_action_btn"
                                            data-cs_action="disable_all"
                                            style="background-color: <?php echo esc_attr( $style_array[ 'cs_deny_all_buttons_bg' ][ 0 ] ); ?>;
                                                    color: <?php echo esc_attr( $style_array[ 'cs_deny_all_buttons_text_color' ][ 0 ] ); ?>;
                                                    padding: <?php echo esc_attr( $btn_pd_t ); ?>px <?php echo esc_attr( $btn_pd_r ); ?>px <?php echo esc_attr( $btn_pd_b ); ?>px <?php echo esc_attr( $btn_pd_l ); ?>px;
									        <?php if ( $cs_bars_type == 'popup_large' ) : ?>
                                                    margin: <?php echo esc_attr( $btn_mg_t ); ?>px <?php echo esc_attr( $btn_mg_r ); ?>px <?php echo esc_attr( $btn_mg_b ); ?>px <?php echo esc_attr( $btn_mg_l ); ?>px;
									        <?php else: ?>
                                                    margin: <?php echo esc_attr( $btn_mg_t ); ?>px 0 <?php echo esc_attr( $btn_mg_b ); ?>px 0;
									        <?php endif; ?>
                                                    font-size: <?php echo esc_attr( $btn_font_s ); ?>px;
                                                    font-weight: <?php echo esc_attr( $btn_font_w ); ?>;"><?php echo esc_html( $front_options[ '_cs_btn_text_disable_all' ] ); ?></button>
								<?php } ?>
								<?php if ( $minimum_recommended_button ) { ?>
                                    <button type="button"
                                            class="btn minimum_recommended_btn cs_action_btn"
                                            data-cs_action="minimum_recommended"
                                            style="background-color: <?php echo esc_attr( $style_array[ 'cs_custom_button_buttons_bg' ][ 0 ] ); ?>;
                                                    color: <?php echo esc_attr( $style_array[ 'cs_custom_button_buttons_text_color' ][ 0 ] ); ?>;
                                                    padding: <?php echo esc_attr( $btn_pd_t ); ?>px <?php echo esc_attr( $btn_pd_r ); ?>px <?php echo esc_attr( $btn_pd_b ); ?>px <?php echo esc_attr( $btn_pd_l ); ?>px;
									        <?php if ( $cs_bars_type == 'popup_large' ) : ?>
                                                    margin: <?php echo esc_attr( $btn_mg_t ); ?>px <?php echo esc_attr( $btn_mg_r ); ?>px <?php echo esc_attr( $btn_mg_b ); ?>px <?php echo esc_attr( $btn_mg_l ); ?>px;
									        <?php else: ?>
                                                    margin: <?php echo esc_attr( $btn_mg_t ); ?>px 0 <?php echo esc_attr( $btn_mg_b ); ?>px 0;
									        <?php endif; ?>
                                                    font-size: <?php echo esc_attr( $btn_font_s ); ?>px;
                                                    font-weight: <?php echo esc_attr( $btn_font_w ); ?>;"><?php echo esc_html( $front_options[ '_cs_btn_text_custom_button' ] ); ?></button>
								<?php } ?>
							<?php } ?>

                            <button type="button" class="btn btn-grey allow_all_btn cs_action_btn"
                                    data-cs_action="allow_all"
                                    style="background-color: <?php echo esc_attr( $style_array[ 'cs_accept_all_buttons_bg' ][ 0 ] ); ?>;
                                            color: <?php echo esc_attr( $style_array[ 'cs_accept_all_buttons_text_color' ][ 0 ] ); ?>;
                                            padding: <?php echo esc_attr( $btn_pd_t ); ?>px <?php echo esc_attr( $btn_pd_r ); ?>px <?php echo esc_attr( $btn_pd_b ); ?>px <?php echo esc_attr( $btn_pd_l ); ?>px;
							        <?php if ( $cs_bars_type == 'popup_large' ) : ?>
                                            margin: <?php echo esc_attr( $btn_mg_t ); ?>px <?php echo esc_attr( $btn_mg_r ); ?>px <?php echo esc_attr( $btn_mg_b ); ?>px <?php echo esc_attr( $btn_mg_l ); ?>px;
							        <?php else: ?>
                                            margin: <?php echo esc_attr( $btn_mg_t ); ?>px 0 <?php echo esc_attr( $btn_mg_b ); ?>px 0;
							        <?php endif; ?>
                                            font-size: <?php echo esc_attr( $btn_font_s ); ?>px;
                                            font-weight: <?php echo esc_attr( $btn_font_w ); ?>;"><?php echo esc_html( $front_options[ '_cs_btn_text_allow_all' ] ); ?></button>
                        </div>
                    </div>

					<?php if ( ConsentMagic()->getOption( 'cs_bar_verified_link' ) ) { ?>
                        <div class="cm-verified-link" style="order: 2;">
                            <a href="<?php echo esc_url( 'https://www.pixelyoursite.com/plugins/consentmagic' ); ?>"
                               rel="nofollow"
                               target="_blank" class="cs_copyright_link"
                               style="color: <?php echo esc_attr( $style_array[ 'cs_links_color' ][ 0 ] ); ?>"><?php esc_html_e( 'Verified by ConsentMagic', 'consent-magic' ); ?></a>
                        </div>
					<?php } ?>
                </div>

				<?php
				if ( $style_array[ 'cs_position_vertical_list' ][ 0 ] == 'bottom' && $logo ) {
					echo '<div style="margin: 20px auto 0;text-align: ' . esc_attr( $style_array[ 'cs_position_horizontal_list' ][ 0 ] ) . ';">' . wp_kses_post( $logo ) . '</div>';
				}
				?>
            </div>
        </div>

        <div class="cs-modal cs_settings_popup <?php echo esc_attr( $cs_theme_slug ) . ' cs_multi_design ' . ( $minimum_recommended_button ? esc_attr( 'cs-minimum-recommended' ) : '' ) . ' ' . ( $deny_all_button ? esc_attr( 'cs-deny-all' ) : '' ) ?>"
             tabindex="-1" role="dialog"
             aria-labelledby="csSettingsPopup" aria-hidden="true">
            <div class="cs-modal-dialog" role="document">
                <div class="cs-modal-content cs-bar-popup"
                     style="background-color: <?php echo esc_attr( $style_array[ 'cs_backend_color' ][ 0 ] ); ?>;
                             border: <?php echo esc_attr( $style_array[ 'cs_border_weight' ][ 0 ] ) . 'px ' . esc_attr( $style_array[ 'cs_border_style' ][ 0 ] ) . ' ' . esc_attr( $style_array[ 'cs_border_color' ][ 0 ] ); ?>;
                             color: <?php echo esc_attr( $style_array[ 'cs_text_color' ][ 0 ] ); ?>;
                             fill: <?php echo esc_attr( $style_array[ 'cs_text_color' ][ 0 ] ); ?>;
                             padding: <?php echo esc_attr( $op_pd_top ) . 'px ' . esc_attr( $op_pd_right ) . 'px ' . esc_attr( $op_pd_bottom ) . 'px ' . esc_attr( $op_pd_left ) . 'px'; ?>;
                             font-size: <?php echo esc_attr( $op_font_s ); ?>px;
                             font-weight: <?php echo esc_attr( $op_font_w ); ?>;
                             min-height: <?php echo esc_attr( $op_min_height ); ?>px;">
					<?php if ( ( isset( $cs_hide_close_btn ) && $cs_hide_close_btn == '0' ) || $cs_type == 'just_inform' ) { ?>
                        <button type="button"
                                class="cs-modal-close <?php echo isset( $cs_hide_close_btn_admin ) ? 'cs-modal-close-admin' : ''; ?>"
                                data-action="cs_close_opt_popup"
                                style="--cs-text-color: <?php esc_html_e( $style_array[ 'cs_text_color' ][ 0 ] ); ?>">
                            <svg viewBox="0 0 24 24">
                                <path d="M19 6.41l-1.41-1.41-5.59 5.59-5.59-5.59-1.41 1.41 5.59 5.59-5.59 5.59 1.41 1.41 5.59-5.59 5.59 5.59 1.41-1.41-5.59-5.59z"></path>
                                <path d="M0 0h24v24h-24z" fill="none"></path>
                            </svg>
                            <span class="cs-sr-only"><?php esc_html_e( 'Close', 'consent-magic' ); ?></span>
                        </button>
					<?php } ?>

                    <div class="cs-modal-body">
						<?php include CMPRO_PLUGIN_VIEWS_PATH . 'templates/multi/cs_options_popup_content.php'; ?>
                    </div>
                </div>

            </div>
        </div>
        <div class="cs-modal-backdrop cs-fade cs-settings-overlay"></div>
        <div class="cs-modal-backdrop cs-fade cs-popupbar-overlay"></div>
    </div>

	<?php
// Re-enable indexing
echo "<!--googleon: all-->";