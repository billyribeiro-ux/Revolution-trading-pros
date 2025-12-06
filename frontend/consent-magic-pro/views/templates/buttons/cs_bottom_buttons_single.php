<?php

namespace ConsentMagicPro;

// If this file is called directly, abort.
if ( !defined( 'WPINC' ) ) {
	die;
}
?>
<div>
    <div class="cs-tab-footer btn-row">
		<?php if ( $deny_all_button ) : ?>
            <a role="button" href="#" tabindex="0" class="btn disable_all_btn cs_action_btn"
               data-cs_action="disable_all"
               style="background-color: <?php echo esc_attr( $style_array[ 'cs_deny_all_buttons_bg' ][ 0 ] ); ?>;
                       color: <?php echo esc_attr( $style_array[ 'cs_deny_all_buttons_text_color' ][ 0 ] ); ?>;
                       padding: <?php echo esc_attr( $op_btn_pd_t ); ?>px <?php echo esc_attr( $op_btn_pd_r ); ?>px <?php echo esc_attr( $op_btn_pd_b ); ?>px <?php echo esc_attr( $op_btn_pd_l ); ?>px;
                       margin: <?php echo esc_attr( $op_btn_mg_t ); ?>px <?php echo esc_attr( $op_btn_mg_r ); ?>px <?php echo esc_attr( $op_btn_mg_b ); ?>px <?php echo esc_attr( $op_btn_mg_l ); ?>px;
                       font-size: <?php echo esc_attr( $op_btn_font_s ); ?>px;
                       font-weight: <?php echo esc_attr( $op_btn_font_w ); ?>;
                       "><?php echo esc_html( $front_options[ '_cs_btn_text_disable_all_in_single_design' ] ); ?></a>
		<?php endif ?>

		<?php if ( isset( $single_design_bottom_buttons ) && $minimum_recommended_button ) : ?>
            <a role="button" href="#" tabindex="0"
               class="btn minimum_recommended_btn cs_action_btn"
               data-cs_action="minimum_recommended"
               style="background-color: <?php echo esc_attr( $style_array[ 'cs_custom_button_buttons_bg' ][ 0 ] ); ?>;
			   <?php echo ( !isset( $single_design_bottom_buttons ) ) ? esc_attr( ' display: none;' ) : ''; ?>
                       color: <?php echo esc_attr( $style_array[ 'cs_custom_button_buttons_text_color' ][ 0 ] ); ?>;
                       padding: <?php echo esc_attr( $op_btn_pd_t ); ?>px <?php echo esc_attr( $op_btn_pd_r ); ?>px <?php echo esc_attr( $op_btn_pd_b ); ?>px <?php echo esc_attr( $op_btn_pd_l ); ?>px;
                       margin: <?php echo esc_attr( $op_btn_mg_t ); ?>px <?php echo esc_attr( $op_btn_mg_r ); ?>px <?php echo esc_attr( $op_btn_mg_b ); ?>px <?php echo esc_attr( $op_btn_mg_l ); ?>px;
                       font-size: <?php echo esc_attr( $op_btn_font_s ); ?>px;
                       font-weight: <?php echo esc_attr( $op_btn_font_w ); ?>;
                       "><?php echo esc_html( $front_options[ '_cs_btn_text_custom_button_in_single_design' ] ); ?></a>
		<?php endif ?>

		<?php if ( $cs_type == 'iab' ) : ?>
            <a role="button" href="#" tabindex="0" class="btn cs_more_options_btn"
               data-cs_action="more_options"
               style="background-color: <?php echo esc_attr( $style_array[ 'cs_confirm_buttons_bg' ][ 0 ] ); ?>;
                       color: <?php echo esc_attr( $style_array[ 'cs_confirm_buttons_text_color' ][ 0 ] ); ?>;
                       padding: <?php echo esc_attr( $op_btn_pd_t ); ?>px <?php echo esc_attr( $op_btn_pd_r ); ?>px <?php echo esc_attr( $op_btn_pd_b ); ?>px <?php echo esc_attr( $op_btn_pd_l ); ?>px;
                       margin: <?php echo esc_attr( $op_btn_mg_t ); ?>px <?php echo esc_attr( $op_btn_mg_r ); ?>px <?php echo esc_attr( $op_btn_mg_b ); ?>px <?php echo esc_attr( $op_btn_mg_l ); ?>px;
                       font-size: <?php echo esc_attr( $op_btn_font_s ); ?>px;
                       font-weight: <?php echo esc_attr( $op_btn_font_w ); ?>;
                       "><?php echo esc_html( $front_options[ '_cs_btn_text_customize' ] ); ?></a>
		<?php endif ?>

        <a role="button" href="#" tabindex="0"
           class="cs_setting_save_button btn btn-grey cs_action_btn"
           data-cs_action="cs_confirm"
           style="background-color: <?php echo esc_attr( $style_array[ 'cs_confirm_buttons_bg' ][ 0 ] ); ?>;
		   <?php echo $cs_type == 'iab' ? 'display: none;' : ''; ?>
                   color: <?php echo esc_attr( $style_array[ 'cs_confirm_buttons_text_color' ][ 0 ] ); ?>;
                   padding: <?php echo esc_attr( $op_btn_pd_t ); ?>px <?php echo esc_attr( $op_btn_pd_r ); ?>px <?php echo esc_attr( $op_btn_pd_b ); ?>px <?php echo esc_attr( $op_btn_pd_l ); ?>px;
                   margin: <?php echo esc_attr( $op_btn_mg_t ); ?>px <?php echo esc_attr( $op_btn_mg_r ); ?>px <?php echo esc_attr( $op_btn_mg_b ); ?>px <?php echo esc_attr( $op_btn_mg_l ); ?>px;
                   font-size: <?php echo esc_attr( $op_btn_font_s ); ?>px;
                   font-weight: <?php echo esc_attr( $op_btn_font_w ); ?>;"><?php echo esc_html( $front_options[ '_cs_btn_text_confirm_in_single_design' ] ); ?></a>
        <a role="button" href="#" tabindex="0" class="btn btn-grey allow_all_btn cs_action_btn"
           data-cs_action="allow_all"
           style="background-color: <?php echo esc_attr( $style_array[ 'cs_accept_all_buttons_bg' ][ 0 ] ); ?>;
                   color: <?php echo esc_attr( $style_array[ 'cs_accept_all_buttons_text_color' ][ 0 ] ); ?>;
                   padding: <?php echo esc_attr( $op_btn_pd_t ); ?>px <?php echo esc_attr( $op_btn_pd_r ); ?>px <?php echo esc_attr( $op_btn_pd_b ); ?>px <?php echo esc_attr( $op_btn_pd_l ); ?>px;
                   margin: <?php echo esc_attr( $op_btn_mg_t ); ?>px <?php echo esc_attr( $op_btn_mg_r ); ?>px <?php echo esc_attr( $op_btn_mg_b ); ?>px <?php echo esc_attr( $op_btn_mg_l ); ?>px;
                   font-size: <?php echo esc_attr( $op_btn_font_s ); ?>px;
                   font-weight: <?php echo esc_attr( $op_btn_font_w ); ?>;"><?php echo esc_html( $front_options[ '_cs_btn_text_allow_all_in_single_design' ] ); ?></a>
		<?php if ( !isset( $single_design_bottom_buttons ) && $minimum_recommended_button ) : ?>
            <a role="button" href="#" tabindex="0"
               class="btn minimum_recommended_btn cs_action_btn"
               data-cs_action="minimum_recommended"
               style="background-color: <?php echo esc_attr( $style_array[ 'cs_custom_button_buttons_bg' ][ 0 ] ); ?>;
                       display: none;
                       color: <?php echo esc_attr( $style_array[ 'cs_custom_button_buttons_text_color' ][ 0 ] ); ?>;
                       padding: <?php echo esc_attr( $op_btn_pd_t ); ?>px <?php echo esc_attr( $op_btn_pd_r ); ?>px <?php echo esc_attr( $op_btn_pd_b ); ?>px <?php echo esc_attr( $op_btn_pd_l ); ?>px;
                       margin: <?php echo esc_attr( $op_btn_mg_t ); ?>px <?php echo esc_attr( $op_btn_mg_r ); ?>px <?php echo esc_attr( $op_btn_mg_b ); ?>px <?php echo esc_attr( $op_btn_mg_l ); ?>px;
                       font-size: <?php echo esc_attr( $op_btn_font_s ); ?>px;
                       font-weight: <?php echo esc_attr( $op_btn_font_w ); ?>;
                       "><?php echo esc_html( $front_options[ '_cs_btn_text_custom_button_in_single_design' ] ); ?></a>
		<?php endif ?>
    </div>
	<?php if ( $options_single_verified_link ) { ?>
        <div class="cs_copyright_link_wrap">
            <a href="<?php echo esc_url( 'https://www.pixelyoursite.com/plugins/consentmagic' ); ?>"
               rel="nofollow"
               target="_blank"
               class="cs_copyright_link"
               style="color: <?php echo esc_attr( $style_array[ 'cs_subtitles_text_color' ][ 0 ] ); ?>"><?php esc_html_e( 'Verified by ConsentMagic', 'consent-magic' ); ?></a>
        </div>
	<?php } ?>

</div>