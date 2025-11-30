<?php

namespace ConsentMagicPro;

// If this file is called directly, abort.
if ( !defined( 'WPINC' ) ) {
	die;
}

$wp_current_lang          = ( isset( $this->current_lang ) && $this->current_lang ) ? $this->current_lang : get_locale();
$cs_user_default_language = ConsentMagic()->getOption( 'cs_user_default_language' );
$cs_language_availability = ConsentMagic()->getOption( 'cs_language_availability' );
if ( isset( $cs_language_availability[ $wp_current_lang ] ) && $cs_language_availability[ $wp_current_lang ] == 0 ) {
	$wp_current_lang = $cs_user_default_language;
}

if ( !isset( $style_array ) ) {
	$style_array = get_post_meta( $id );
}

if ( !isset( $cs_preview_size_show ) ) {
	$prefix = 'cs_d_single_op_';

	$op_pd_t            = ConsentMagic()->getOption( $prefix . 'padding_top' ) ?: 0;
	$op_pd_r            = ConsentMagic()->getOption( $prefix . 'padding_right' ) ?: 0;
	$op_pd_b            = ConsentMagic()->getOption( $prefix . 'padding_bottom' ) ?: 0;
	$op_pd_l            = ConsentMagic()->getOption( $prefix . 'padding_left' ) ?: 0;
	$op_font_s          = ConsentMagic()->getOption( $prefix . 'font_size' );
	$op_font_w          = ConsentMagic()->getOption( $prefix . 'font_weight' );
	$op_second_font_s   = ConsentMagic()->getOption( $prefix . 'second_font_size' );
	$op_second_font_w   = ConsentMagic()->getOption( $prefix . 'second_font_weight' );
	$op_title_f_s       = ConsentMagic()->getOption( $prefix . 'titles_font_size' );
	$op_title_f_w       = ConsentMagic()->getOption( $prefix . 'titles_font_weight' );
	$op_subtitle_f_s    = ConsentMagic()->getOption( $prefix . 'subtitles_font_size' );
	$op_subtitle_f_w    = ConsentMagic()->getOption( $prefix . 'subtitles_font_weight' );
	$op_subsubtitle_f_s = ConsentMagic()->getOption( $prefix . 'subsubtitles_font_size' );
	$op_subsubtitle_f_w = ConsentMagic()->getOption( $prefix . 'subsubtitles_font_weight' );
	$op_btn_pd_t        = ConsentMagic()->getOption( $prefix . 'btn_pd_t' ) ?: 0;
	$op_btn_pd_r        = ConsentMagic()->getOption( $prefix . 'btn_pd_r' ) ?: 0;
	$op_btn_pd_b        = ConsentMagic()->getOption( $prefix . 'btn_pd_b' ) ?: 0;
	$op_btn_pd_l        = ConsentMagic()->getOption( $prefix . 'btn_pd_l' ) ?: 0;
	$op_btn_mg_t        = ConsentMagic()->getOption( $prefix . 'btn_mg_t' ) ?: 0;
	$op_btn_mg_b        = ConsentMagic()->getOption( $prefix . 'btn_mg_b' ) ?: 0;
	$op_btn_mg_l        = ConsentMagic()->getOption( $prefix . 'btn_mg_l' ) ?: 0;
	$op_btn_mg_r        = ConsentMagic()->getOption( $prefix . 'btn_mg_r' ) ?: 0;
	$op_btn_font_s      = ConsentMagic()->getOption( $prefix . 'btn_font_size' );
	$op_btn_font_w      = ConsentMagic()->getOption( $prefix . 'btn_font_weight' );
}

$op_block_border_color = ConsentMagic()->getOption( 'cs_text_block_bg' );

$custom_text                  = get_post_meta( $active_rule_id, '_cs_custom_text', true );
$cs_type                      = get_post_meta( $active_rule_id, '_cs_type', true );
$cs_active_toggle_color_label = $style_array[ 'cs_active_toggle_color' ][ 0 ] . '40';
$cs_text_underline_color      = $style_array[ 'cs_subtitles_text_color' ][ 0 ] . '40';
$cs_border_color              = $style_array[ 'cs_tab_buttons_bg' ][ 0 ];

if ( $id ) {
	$cs_theme_slug = get_post( $id )->post_name;
} else {
	$cs_theme_slug = '';
}

$disable_all_btn_bg = $style_array[ 'cs_deny_all_buttons_bg' ][ 0 ];
$privacy_link       = get_post_meta( $active_rule_id, '_cs_privacy_link', true );

list( $r, $g, $b ) = sscanf( $style_array[ 'cs_text_block_bg' ][ 0 ], "#%02x%02x%02x" );
$cs_text_block_bg_75 = "rgba( $r, $g, $b, 0.75)";

if ( !isset( $style_array ) ) {
	$style_array = get_post_meta( $id );
}

$button_width_calc = ( (int) $op_btn_mg_r + (int) $op_btn_mg_l + 8 ) . 'px';

$additional_styles = '--cs-color-active-toggle: ' . esc_attr( $cs_active_toggle_color_label );
$additional_styles .= '; --cs-active-toggle-background-color: ' . $style_array[ 'cs_active_toggle_color' ][ 0 ];
$additional_styles .= '; --cs-active-toggle-text-color: ' . $style_array[ 'cs_active_toggle_text_color' ][ 0 ];
$additional_styles .= '; --cs-text-underline-color: ' . $cs_text_underline_color;
$additional_styles .= '; --cs-text-color: ' . $style_array[ 'cs_text_color' ][ 0 ];
$additional_styles .= '; --cs-text-font-size: ' . $op_font_s . 'px';
$additional_styles .= '; --cs-text-font-weight: ' . $op_font_w;
$additional_styles .= '; --cs-color-underline: ' . $style_array[ 'cs_links_color' ][ 0 ];
$additional_styles .= '; --cs_subtitles_text_color: ' . $style_array[ 'cs_subtitles_text_color' ][ 0 ];
$additional_styles .= '; --cs_op_subtitle_f_w: ' . ConsentMagic()->getOption( 'cs_d_single_op_subtitles_font_weight' );
$additional_styles .= '; --cs_d_single_op_second_f_s: ' . ConsentMagic()->getOption( 'cs_d_single_op_second_font_size' ) . 'px';
$additional_styles .= '; --cs_d_single_op_second_f_w: ' . ConsentMagic()->getOption( 'cs_d_single_op_second_font_weight' );
$additional_styles .= '; --cs_d_single_op_subsubtitles_f_s: ' . ConsentMagic()->getOption( 'cs_d_single_op_subsubtitles_font_size' ) . 'px';
$additional_styles .= '; --cs_d_single_op_subsubtitles_f_w: ' . ConsentMagic()->getOption( 'cs_d_single_op_subsubtitles_font_weight' );
$additional_styles .= '; --cs_text_block_bg: ' . $style_array[ 'cs_text_block_bg' ][ 0 ];
$additional_styles .= '; --cs_text_block_bg_75: ' . $cs_text_block_bg_75;
$additional_styles .= '; --cs_d_single_op_f_s: ' . ConsentMagic()->getOption( 'cs_d_single_op_font_size' ) . 'px';
$additional_styles .= '; --cs_d_single_op_f_w: ' . ConsentMagic()->getOption( 'cs_d_single_op_font_weight' );
$additional_styles .= '; --cs_backend_color: ' . esc_attr( $style_array[ 'cs_backend_color' ][ 0 ] );
$additional_styles .= '; --cs_border_color: ' . esc_attr( $cs_border_color );
$additional_styles .= '; --cs_button_width_calc: ' . $button_width_calc;
$additional_styles .= '; --cs_tab_buttons_bg: ' . $style_array[ 'cs_tab_buttons_bg' ][ 0 ];
$additional_styles .= '; --cs_titles_text_color: ' . $style_array[ 'cs_titles_text_color' ][ 0 ];

$minimum_recommended_button   = !empty( $cs_custom_button_btn ) || !empty( $style_array[ '_cs_custom_button' ][ 0 ] );
$deny_all_button              = !empty( $cs_deny_all_btn ) || !empty( $style_array[ '_cs_deny_all_btn' ][ 0 ] );
$single_design_bottom_buttons = true;
$options_single_verified_link = ConsentMagic()->getOption( 'cs_options_single_verified_link' );

$logo = wp_get_attachment_image( $style_array[ 'cs_logo' ][ 0 ], $style_array[ 'cs_logo_size' ][ 0 ], false, array(
	'style' => 'display:inline-block;'
) );

// Disable indexing of ConsentMagic Cookie data
echo "<!--googleoff: all-->";

?>
    <div class="cs_popup_overlay"></div>
    <div class="<?php echo esc_attr( ConsentMagic()->get_containers_names()[ 'cs_cookie_bar_container' ] ); ?>"
         style="<?php echo esc_attr( $additional_styles ); ?>"
    >
        <div class="cs-modal cs_settings_popup <?php echo esc_attr( $cs_theme_slug ) . ' cs_single_design ' . ( CS_IAB_Integration()->enabled() ? 'cs-iab-enabled' : '' ) . ' ' . ( $minimum_recommended_button ? esc_attr( 'cs-minimum-recommended' ) : '' ) . ' ' . ( $deny_all_button ? esc_attr( 'cs-deny-all' ) : '' ) ?>"
             tabindex="-1" role="dialog"
             aria-labelledby="csSettingsPopup" aria-hidden="true">
            <div class="cs-modal-dialog-single" role="document">
				<?php if ( CS_IAB_Integration()->enabled() ) : ?>
					<?php include CMPRO_PLUGIN_VIEWS_PATH . 'templates/single/cs_popup_content_iab.php'; ?>
				<?php else : ?>
					<?php include CMPRO_PLUGIN_VIEWS_PATH . 'templates/single/cs_popup_content.php'; ?>
				<?php endif; ?>
            </div>
        </div>

        <div class="cs-modal-backdrop cs-fade cs-settings-overlay"></div>
        <div class="cs-modal-backdrop cs-fade cs-popupbar-overlay cs-popupbar-overlay-bar"></div>
    </div>
	<?php
// Re-enable indexing
echo "<!--googleon: all-->";