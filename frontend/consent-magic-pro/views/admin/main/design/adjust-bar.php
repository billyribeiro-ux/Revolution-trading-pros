<?php

namespace ConsentMagicPro;

// If this file is called directly, abort.
if ( !defined( 'WPINC' ) ) {
	die;
}

?>
<input type="hidden" name="cm-restore-defaults-design" id="cm-restore-defaults-design" value="">
<div class="cs_preview_design_block adjust-bar-preview mb-24">
	<?php include CMPRO_PLUGIN_VIEWS_PATH . "admin/components/spinner.php"; ?>
    <div class="loading_body cs_preview_design_wrap">
        <div class="preview-title"><?php esc_html_e( 'Preview', 'consent-magic' ); ?>:</div>

        <div class="preview-links">
			<?php if ( $type === 'multi' ) : ?>
                <div>
                    <a type="button" class="link link-underline lh-162 preview_size_btn"
                       data-template="bar_small"><?php esc_html_e( 'Small bar', 'consent-magic' ); ?></a>
                </div>

                <div>
                    <a type="button" class="link link-underline lh-162 preview_size_btn"
                       data-template="bar_large"><?php esc_html_e( 'Large bar', 'consent-magic' ); ?></a>
                </div>

                <div>
                    <a type="button" class="link link-underline lh-162 preview_size_btn"
                       data-template="popup_small"><?php esc_html_e( 'Small popup', 'consent-magic' ); ?></a>
                </div>
			<?php endif; ?>

            <div>
                <a type="button"
                   class="link link-underline lh-162 <?php echo $type === 'multi' ? 'preview_size_btn' : 'preview_size_single_btn'; ?>"
                   data-template="popup_large"><?php esc_html_e( 'Large popup', 'consent-magic' ); ?></a>
            </div>
        </div>
    </div>
</div>

<div class="adjust-bar-wrap">
    <div class="card card-style8">
        <div class="card-header card-header-style3">
            <h3 class="primary-heading-type2">
				<?php esc_html_e( 'Small bar', 'consent-magic' ); ?>
            </h3>

			<?php cardCollapseSettingsWithText(); ?>
        </div>

        <div class="card-body">
            <div class="gap-24 design-wrap">
                <h3 class="primary-heading-type2"><?php esc_html_e( 'Content Block', 'consent-magic' ); ?></h3>

                <div class="list-item row-design-settings">
                    <div>
                        <h4 class="font-semibold-type2 limit-width mb-4">
							<?php esc_html_e( 'Minimum height', 'consent-magic' ); ?>:
                        </h4>
						<?php render_number_text_input( 'cs_d_sb_min_height', '', '', true ); ?>
                    </div>

                    <div>
                        <h4 class="font-semibold-type2 limit-width mb-4">
							<?php esc_html_e( 'Padding top', 'consent-magic' ); ?>:
                        </h4>
						<?php render_number_text_input( 'cs_d_sb_padding_top', '', '', true ); ?>
                    </div>

                    <div>
                        <h4 class="font-semibold-type2 limit-width mb-4">
							<?php esc_html_e( 'Padding bottom', 'consent-magic' ); ?>:
                        </h4>
						<?php render_number_text_input( 'cs_d_sb_padding_bottom', '', '', true ); ?>
                    </div>

                    <div>
                        <h4 class="font-semibold-type2 limit-width mb-4">
							<?php esc_html_e( 'Padding left', 'consent-magic' ); ?>:
                        </h4>
						<?php render_number_text_input( 'cs_d_sb_padding_left', '', '', true ); ?>
                    </div>

                    <div>
                        <h4 class="font-semibold-type2 limit-width mb-4">
							<?php esc_html_e( 'Padding right', 'consent-magic' ); ?>:
                        </h4>
						<?php render_number_text_input( 'cs_d_sb_padding_right', '', '', true ); ?>
                    </div>
                </div>

                <div class="line"></div>
                <h3 class="primary-heading-type2"><?php esc_html_e( 'Font', 'consent-magic' ); ?></h3>

                <div class="list-item row-design-settings">
                    <div>
                        <h4 class="font-semibold-type2 limit-width mb-4">
							<?php esc_html_e( 'Font size', 'consent-magic' ); ?>:
                        </h4>
						<?php render_number_text_input( 'cs_d_sb_font_size', '', '', true ); ?>
                    </div>

                    <div>
                        <h4 class="font-semibold-type2 limit-width mb-4">
							<?php esc_html_e( 'Font weight', 'consent-magic' ); ?>:
                        </h4>
						<?php renderSelectInput( 'cs_d_sb_font_weight', ConsentMagic()->getOption( 'cs_font_weight_list' ), true, true, 'short' ); ?>
                    </div>
                </div>

                <div class="line"></div>
                <h3 class="primary-heading-type2"><?php esc_html_e( 'Button\'s', 'consent-magic' ); ?></h3>

                <div class="list-item row-design-settings">
                    <div>
                        <h4 class="font-semibold-type2 limit-width mb-4">
							<?php esc_html_e( 'Button\'s padding top', 'consent-magic' ); ?>:
                        </h4>
						<?php render_number_text_input( 'cs_d_sb_btn_pd_t', '', '', true ); ?>
                    </div>

                    <div>
                        <h4 class="font-semibold-type2 limit-width mb-4">
							<?php esc_html_e( 'Button\'s padding bottom', 'consent-magic' ); ?>:
                        </h4>
						<?php render_number_text_input( 'cs_d_sb_btn_pd_bt', '', '', true ); ?>
                    </div>

                    <div>
                        <h4 class="font-semibold-type2 limit-width mb-4">
							<?php esc_html_e( 'Button\'s padding right', 'consent-magic' ); ?>:
                        </h4>
						<?php render_number_text_input( 'cs_d_sb_btn_pd_r', '', '', true ); ?>
                    </div>

                    <div>
                        <h4 class="font-semibold-type2 limit-width mb-4">
							<?php esc_html_e( 'Button\'s padding left', 'consent-magic' ); ?>:
                        </h4>
						<?php render_number_text_input( 'cs_d_sb_btn_pd_l', '', '', true ); ?>
                    </div>

                    <div>
                        <h4 class="font-semibold-type2 limit-width mb-4">
							<?php esc_html_e( 'Button\'s margin top', 'consent-magic' ); ?>:
                        </h4>
						<?php render_number_text_input( 'cs_d_sb_btn_mg_t', '', '', true ); ?>
                    </div>

                    <div>
                        <h4 class="font-semibold-type2 limit-width mb-4">
							<?php esc_html_e( 'Button\'s margin bottom', 'consent-magic' ); ?>:
                        </h4>
						<?php render_number_text_input( 'cs_d_sb_btn_mg_bt', '', '', true ); ?>
                    </div>

                    <div>
                        <h4 class="font-semibold-type2 limit-width mb-4">
							<?php esc_html_e( 'Button\'s margin right', 'consent-magic' ); ?>:
                        </h4>
						<?php render_number_text_input( 'cs_d_sb_btn_mg_r', '', '', true ); ?>
                    </div>

                    <div>
                        <h4 class="font-semibold-type2 limit-width mb-4">
							<?php esc_html_e( 'Button\'s margin left', 'consent-magic' ); ?>:
                        </h4>
						<?php render_number_text_input( 'cs_d_sb_btn_mg_l', '', '', true ); ?>
                    </div>

                    <div>
                        <h4 class="font-semibold-type2 limit-width mb-4">
							<?php esc_html_e( 'Button\'s font size', 'consent-magic' ); ?>:
                        </h4>
						<?php render_number_text_input( 'cs_d_sb_btn_font_size', '', '', true ); ?>
                    </div>

                    <div>
                        <h4 class="font-semibold-type2 limit-width mb-4">
							<?php esc_html_e( 'Button\'s font weight', 'consent-magic' ); ?>:
                        </h4>
						<?php renderSelectInput( 'cs_d_sb_btn_font_weight', ConsentMagic()->getOption( 'cs_font_weight_list' ), true, true, 'short' ); ?>
                    </div>
                </div>
            </div>
        </div>
    </div>

    <div class="card card-style8">
        <div class="card-header card-header-style3">
            <h3 class="primary-heading-type2">
				<?php esc_html_e( 'Large bar', 'consent-magic' ); ?>
            </h3>

			<?php cardCollapseSettingsWithText(); ?>
        </div>

        <div class="card-body">
            <div class="gap-24 design-wrap">
                <h3 class="primary-heading-type2"><?php esc_html_e( 'Content Block', 'consent-magic' ); ?></h3>

                <div class="list-item row-design-settings">
                    <div>
                        <h4 class="font-semibold-type2 limit-width mb-4">
							<?php esc_html_e( 'Minimum height', 'consent-magic' ); ?>:
                        </h4>
						<?php render_number_text_input( 'cs_d_lb_min_height', '', '', true ); ?>
                    </div>

                    <div>
                        <h4 class="font-semibold-type2 limit-width mb-4">
							<?php esc_html_e( 'Padding top', 'consent-magic' ); ?>:
                        </h4>
						<?php render_number_text_input( 'cs_d_lb_padding_top', '', '', true ); ?>
                    </div>

                    <div>
                        <h4 class="font-semibold-type2 limit-width mb-4">
							<?php esc_html_e( 'Padding bottom', 'consent-magic' ); ?>:
                        </h4>
						<?php render_number_text_input( 'cs_d_lb_padding_bottom', '', '', true ); ?>
                    </div>

                    <div>
                        <h4 class="font-semibold-type2 limit-width mb-4">
							<?php esc_html_e( 'Padding left', 'consent-magic' ); ?>:
                        </h4>
						<?php render_number_text_input( 'cs_d_lb_padding_left', '', '', true ); ?>
                    </div>

                    <div>
                        <h4 class="font-semibold-type2 limit-width mb-4">
							<?php esc_html_e( 'Padding right', 'consent-magic' ); ?>:
                        </h4>
						<?php render_number_text_input( 'cs_d_lb_padding_right', '', '', true ); ?>
                    </div>
                </div>

                <div class="line"></div>
                <h3 class="primary-heading-type2"><?php esc_html_e( 'Font', 'consent-magic' ); ?></h3>

                <div class="list-item row-design-settings">
                    <div>
                        <h4 class="font-semibold-type2 limit-width mb-4">
							<?php esc_html_e( 'Font size', 'consent-magic' ); ?>:
                        </h4>
						<?php render_number_text_input( 'cs_d_lb_font_size', '', '', true ); ?>
                    </div>

                    <div>
                        <h4 class="font-semibold-type2 limit-width mb-4">
							<?php esc_html_e( 'Font weight', 'consent-magic' ); ?>:
                        </h4>
						<?php renderSelectInput( 'cs_d_lb_font_weight', ConsentMagic()->getOption( 'cs_font_weight_list' ), true, true, 'short' ); ?>
                    </div>
                </div>

                <div class="line"></div>
                <h3 class="primary-heading-type2"><?php esc_html_e( 'Button\'s', 'consent-magic' ); ?></h3>

                <div class="list-item row-design-settings">
                    <div>
                        <h4 class="font-semibold-type2 limit-width mb-4">
							<?php esc_html_e( 'Button\'s padding top', 'consent-magic' ); ?>:
                        </h4>
						<?php render_number_text_input( 'cs_d_lb_btn_pd_t', '', '', true ); ?>
                    </div>

                    <div>
                        <h4 class="font-semibold-type2 limit-width mb-4">
							<?php esc_html_e( 'Button\'s padding bottom', 'consent-magic' ); ?>:
                        </h4>
						<?php render_number_text_input( 'cs_d_lb_btn_pd_bt', '', '', true ); ?>
                    </div>

                    <div>
                        <h4 class="font-semibold-type2 limit-width mb-4">
							<?php esc_html_e( 'Button\'s padding right', 'consent-magic' ); ?>:
                        </h4>
						<?php render_number_text_input( 'cs_d_lb_btn_pd_r', '', '', true ); ?>
                    </div>

                    <div>
                        <h4 class="font-semibold-type2 limit-width mb-4">
							<?php esc_html_e( 'Button\'s padding left', 'consent-magic' ); ?>:
                        </h4>
						<?php render_number_text_input( 'cs_d_lb_btn_pd_l', '', '', true ); ?>
                    </div>

                    <div>
                        <h4 class="font-semibold-type2 limit-width mb-4">
							<?php esc_html_e( 'Button\'s margin top', 'consent-magic' ); ?>:
                        </h4>
						<?php render_number_text_input( 'cs_d_lb_btn_mg_t', '', '', true ); ?>
                    </div>

                    <div>
                        <h4 class="font-semibold-type2 limit-width mb-4">
							<?php esc_html_e( 'Button\'s margin bottom', 'consent-magic' ); ?>:
                        </h4>
						<?php render_number_text_input( 'cs_d_lb_btn_mg_bt', '', '', true ); ?>
                    </div>

                    <div>
                        <h4 class="font-semibold-type2 limit-width mb-4">
							<?php esc_html_e( 'Button\'s margin right', 'consent-magic' ); ?>:
                        </h4>
						<?php render_number_text_input( 'cs_d_lb_btn_mg_r', '', '', true ); ?>
                    </div>

                    <div>
                        <h4 class="font-semibold-type2 limit-width mb-4">
							<?php esc_html_e( 'Button\'s margin left', 'consent-magic' ); ?>:
                        </h4>
						<?php render_number_text_input( 'cs_d_lb_btn_mg_l', '', '', true ); ?>
                    </div>

                    <div>
                        <h4 class="font-semibold-type2 limit-width mb-4">
							<?php esc_html_e( 'Button\'s font size', 'consent-magic' ); ?>:
                        </h4>
						<?php render_number_text_input( 'cs_d_lb_btn_font_size', '', '', true ); ?>
                    </div>

                    <div>
                        <h4 class="font-semibold-type2 limit-width mb-4">
							<?php esc_html_e( 'Button\'s font weight', 'consent-magic' ); ?>:
                        </h4>
						<?php renderSelectInput( 'cs_d_lb_btn_font_weight', ConsentMagic()->getOption( 'cs_font_weight_list' ), true, true, 'short' ); ?>
                    </div>
                </div>
            </div>
        </div>
    </div>

    <div class="card card-style8">
        <div class="card-header card-header-style3">
            <h3 class="primary-heading-type2">
				<?php esc_html_e( 'Small popup', 'consent-magic' ); ?>
            </h3>

			<?php cardCollapseSettingsWithText(); ?>
        </div>

        <div class="card-body">
            <div class="gap-24 design-wrap">
                <h3 class="primary-heading-type2"><?php esc_html_e( 'Content Block', 'consent-magic' ); ?></h3>

                <div class="list-item row-design-settings">
                    <div>
                        <h4 class="font-semibold-type2 limit-width mb-4">
							<?php esc_html_e( 'Minimum height', 'consent-magic' ); ?>:
                        </h4>
						<?php render_number_text_input( 'cs_d_sp_min_height', '', '', true ); ?>
                    </div>

                    <div>
                        <h4 class="font-semibold-type2 limit-width mb-4">
							<?php esc_html_e( 'Padding top', 'consent-magic' ); ?>:
                        </h4>
						<?php render_number_text_input( 'cs_d_sp_padding_top', '', '', true ); ?>
                    </div>

                    <div>
                        <h4 class="font-semibold-type2 limit-width mb-4">
							<?php esc_html_e( 'Padding bottom', 'consent-magic' ); ?>:
                        </h4>
						<?php render_number_text_input( 'cs_d_sp_padding_bottom', '', '', true ); ?>
                    </div>

                    <div>
                        <h4 class="font-semibold-type2 limit-width mb-4">
							<?php esc_html_e( 'Padding left', 'consent-magic' ); ?>:
                        </h4>
						<?php render_number_text_input( 'cs_d_sp_padding_left', '', '', true ); ?>
                    </div>

                    <div>
                        <h4 class="font-semibold-type2 limit-width mb-4">
							<?php esc_html_e( 'Padding right', 'consent-magic' ); ?>:
                        </h4>
						<?php render_number_text_input( 'cs_d_sp_padding_right', '', '', true ); ?>
                    </div>
                </div>

                <div class="line"></div>
                <h3 class="primary-heading-type2"><?php esc_html_e( 'Font', 'consent-magic' ); ?></h3>

                <div class="list-item row-design-settings">
                    <div>
                        <h4 class="font-semibold-type2 limit-width mb-4">
							<?php esc_html_e( 'Font size', 'consent-magic' ); ?>:
                        </h4>
						<?php render_number_text_input( 'cs_d_sp_font_size', '', '', true ); ?>
                    </div>

                    <div>
                        <h4 class="font-semibold-type2 limit-width mb-4">
							<?php esc_html_e( 'Font weight', 'consent-magic' ); ?>:
                        </h4>
						<?php renderSelectInput( 'cs_d_sp_font_weight', ConsentMagic()->getOption( 'cs_font_weight_list' ), true, true, 'short' ); ?>
                    </div>
                </div>

                <div class="line"></div>
                <h3 class="primary-heading-type2"><?php esc_html_e( 'Button\'s', 'consent-magic' ); ?></h3>

                <div class="list-item row-design-settings">
                    <div>
                        <h4 class="font-semibold-type2 limit-width mb-4">
							<?php esc_html_e( 'Button\'s padding top', 'consent-magic' ); ?>:
                        </h4>
						<?php render_number_text_input( 'cs_d_sp_btn_pd_t', '', '', true ); ?>
                    </div>

                    <div>
                        <h4 class="font-semibold-type2 limit-width mb-4">
							<?php esc_html_e( 'Button\'s padding bottom', 'consent-magic' ); ?>:
                        </h4>
						<?php render_number_text_input( 'cs_d_sp_btn_pd_bt', '', '', true ); ?>
                    </div>

                    <div>
                        <h4 class="font-semibold-type2 limit-width mb-4">
							<?php esc_html_e( 'Button\'s padding right', 'consent-magic' ); ?>:
                        </h4>
						<?php render_number_text_input( 'cs_d_sp_btn_pd_r', '', '', true ); ?>
                    </div>

                    <div>
                        <h4 class="font-semibold-type2 limit-width mb-4">
							<?php esc_html_e( 'Button\'s padding left', 'consent-magic' ); ?>:
                        </h4>
						<?php render_number_text_input( 'cs_d_sp_btn_pd_l', '', '', true ); ?>
                    </div>

                    <div>
                        <h4 class="font-semibold-type2 limit-width mb-4">
							<?php esc_html_e( 'Button\'s margin top', 'consent-magic' ); ?>:
                        </h4>
						<?php render_number_text_input( 'cs_d_sp_btn_mg_t', '', '', true ); ?>
                    </div>

                    <div>
                        <h4 class="font-semibold-type2 limit-width mb-4">
							<?php esc_html_e( 'Button\'s margin bottom', 'consent-magic' ); ?>:
                        </h4>
						<?php render_number_text_input( 'cs_d_sp_btn_mg_bt', '', '', true ); ?>
                    </div>

                    <div>
                        <h4 class="font-semibold-type2 limit-width mb-4">
							<?php esc_html_e( 'Button\'s margin right', 'consent-magic' ); ?>:
                        </h4>
						<?php render_number_text_input( 'cs_d_sp_btn_mg_r', '', '', true ); ?>
                    </div>

                    <div>
                        <h4 class="font-semibold-type2 limit-width mb-4">
							<?php esc_html_e( 'Button\'s margin left', 'consent-magic' ); ?>:
                        </h4>
						<?php render_number_text_input( 'cs_d_sp_btn_mg_l', '', '', true ); ?>
                    </div>

                    <div>
                        <h4 class="font-semibold-type2 limit-width mb-4">
							<?php esc_html_e( 'Button\'s font size', 'consent-magic' ); ?>:
                        </h4>
						<?php render_number_text_input( 'cs_d_sp_btn_font_size', '', '', true ); ?>
                    </div>

                    <div>
                        <h4 class="font-semibold-type2 limit-width mb-4">
							<?php esc_html_e( 'Button\'s font weight', 'consent-magic' ); ?>:
                        </h4>
						<?php renderSelectInput( 'cs_d_sp_btn_font_weight', ConsentMagic()->getOption( 'cs_font_weight_list' ), true, true, 'short' ); ?>
                    </div>
                </div>
            </div>
        </div>
    </div>

    <div class="card card-style8">
        <div class="card-header card-header-style3">
            <h3 class="primary-heading-type2">
				<?php esc_html_e( 'Large popup', 'consent-magic' ); ?>
            </h3>

			<?php cardCollapseSettingsWithText(); ?>
        </div>

        <div class="card-body">
            <div class="gap-24 design-wrap">
                <h3 class="primary-heading-type2"><?php esc_html_e( 'Content Block', 'consent-magic' ); ?></h3>

                <div class="list-item row-design-settings">
                    <div>
                        <h4 class="font-semibold-type2 limit-width mb-4">
							<?php esc_html_e( 'Minimum height', 'consent-magic' ); ?>:
                        </h4>
						<?php render_number_text_input( 'cs_d_lp_min_height', '', '', true ); ?>
                    </div>

                    <div>
                        <h4 class="font-semibold-type2 limit-width mb-4">
							<?php esc_html_e( 'Padding top', 'consent-magic' ); ?>:
                        </h4>
						<?php render_number_text_input( 'cs_d_lp_padding_top', '', '', true ); ?>
                    </div>

                    <div>
                        <h4 class="font-semibold-type2 limit-width mb-4">
							<?php esc_html_e( 'Padding bottom', 'consent-magic' ); ?>:
                        </h4>
						<?php render_number_text_input( 'cs_d_lp_padding_bottom', '', '', true ); ?>
                    </div>

                    <div>
                        <h4 class="font-semibold-type2 limit-width mb-4">
							<?php esc_html_e( 'Padding left', 'consent-magic' ); ?>:
                        </h4>
						<?php render_number_text_input( 'cs_d_lp_padding_left', '', '', true ); ?>
                    </div>

                    <div>
                        <h4 class="font-semibold-type2 limit-width mb-4">
							<?php esc_html_e( 'Padding right', 'consent-magic' ); ?>:
                        </h4>
						<?php render_number_text_input( 'cs_d_lp_padding_right', '', '', true ); ?>
                    </div>
                </div>

                <div class="line"></div>
                <h3 class="primary-heading-type2"><?php esc_html_e( 'Font', 'consent-magic' ); ?></h3>

                <div class="list-item row-design-settings">
                    <div>
                        <h4 class="font-semibold-type2 limit-width mb-4">
							<?php esc_html_e( 'Font size', 'consent-magic' ); ?>:
                        </h4>
						<?php render_number_text_input( 'cs_d_lp_font_size', '', '', true ); ?>
                    </div>

                    <div>
                        <h4 class="font-semibold-type2 limit-width mb-4">
							<?php esc_html_e( 'Font weight', 'consent-magic' ); ?>:
                        </h4>
						<?php renderSelectInput( 'cs_d_lp_font_weight', ConsentMagic()->getOption( 'cs_font_weight_list' ), true, true, 'short' ); ?>
                    </div>
                </div>

                <div class="line"></div>
                <h3 class="primary-heading-type2"><?php esc_html_e( 'Button\'s', 'consent-magic' ); ?></h3>

                <div class="list-item row-design-settings">
                    <div>
                        <h4 class="font-semibold-type2 limit-width mb-4">
							<?php esc_html_e( 'Button\'s padding top', 'consent-magic' ); ?>:
                        </h4>
						<?php render_number_text_input( 'cs_d_lp_btn_pd_t', '', '', true ); ?>
                    </div>

                    <div>
                        <h4 class="font-semibold-type2 limit-width mb-4">
							<?php esc_html_e( 'Button\'s padding bottom', 'consent-magic' ); ?>:
                        </h4>
						<?php render_number_text_input( 'cs_d_lp_btn_pd_bt', '', '', true ); ?>
                    </div>

                    <div>
                        <h4 class="font-semibold-type2 limit-width mb-4">
							<?php esc_html_e( 'Button\'s padding right', 'consent-magic' ); ?>:
                        </h4>
						<?php render_number_text_input( 'cs_d_lp_btn_pd_r', '', '', true ); ?>
                    </div>

                    <div>
                        <h4 class="font-semibold-type2 limit-width mb-4">
							<?php esc_html_e( 'Button\'s padding left', 'consent-magic' ); ?>:
                        </h4>
						<?php render_number_text_input( 'cs_d_lp_btn_pd_l', '', '', true ); ?>
                    </div>

                    <div>
                        <h4 class="font-semibold-type2 limit-width mb-4">
							<?php esc_html_e( 'Button\'s margin top', 'consent-magic' ); ?>:
                        </h4>
						<?php render_number_text_input( 'cs_d_lp_btn_mg_t', '', '', true ); ?>
                    </div>

                    <div>
                        <h4 class="font-semibold-type2 limit-width mb-4">
							<?php esc_html_e( 'Button\'s margin bottom', 'consent-magic' ); ?>:
                        </h4>
						<?php render_number_text_input( 'cs_d_lp_btn_mg_bt', '', '', true ); ?>
                    </div>

                    <div>
                        <h4 class="font-semibold-type2 limit-width mb-4">
							<?php esc_html_e( 'Button\'s margin right', 'consent-magic' ); ?>:
                        </h4>
						<?php render_number_text_input( 'cs_d_lp_btn_mg_r', '', '', true ); ?>
                    </div>

                    <div>
                        <h4 class="font-semibold-type2 limit-width mb-4">
							<?php esc_html_e( 'Button\'s margin left', 'consent-magic' ); ?>:
                        </h4>
						<?php render_number_text_input( 'cs_d_lp_btn_mg_l', '', '', true ); ?>
                    </div>

                    <div>
                        <h4 class="font-semibold-type2 limit-width mb-4">
							<?php esc_html_e( 'Button\'s font size', 'consent-magic' ); ?>:
                        </h4>
						<?php render_number_text_input( 'cs_d_lp_btn_font_size', '', '', true ); ?>
                    </div>

                    <div>
                        <h4 class="font-semibold-type2 limit-width mb-4">
							<?php esc_html_e( 'Button\'s font weight', 'consent-magic' ); ?>:
                        </h4>
						<?php renderSelectInput( 'cs_d_lp_btn_font_weight', ConsentMagic()->getOption( 'cs_font_weight_list' ), true, true, 'short' ); ?>
                    </div>
                </div>
            </div>
        </div>
    </div>

    <div class="card card-style8">
        <div class="card-header card-header-style3">
            <h3 class="primary-heading-type2">
				<?php esc_html_e( 'Options popup', 'consent-magic' ); ?>
            </h3>

			<?php cardCollapseSettingsWithText(); ?>
        </div>

        <div class="card-body">
            <div class="gap-24 design-wrap">
                <h3 class="primary-heading-type2"><?php esc_html_e( 'Content Block', 'consent-magic' ); ?></h3>

                <div class="list-item row-design-settings">
                    <div>
                        <h4 class="font-semibold-type2 limit-width mb-4">
							<?php esc_html_e( 'Minimum height', 'consent-magic' ); ?>:
                        </h4>
						<?php render_number_text_input( 'cs_d_op_min_height', '', '', true ); ?>
                    </div>

                    <div>
                        <h4 class="font-semibold-type2 limit-width mb-4">
							<?php esc_html_e( 'Padding top', 'consent-magic' ); ?>:
                        </h4>
						<?php render_number_text_input( 'cs_d_op_padding_top', '', '', true ); ?>
                    </div>

                    <div>
                        <h4 class="font-semibold-type2 limit-width mb-4">
							<?php esc_html_e( 'Padding bottom', 'consent-magic' ); ?>:
                        </h4>
						<?php render_number_text_input( 'cs_d_op_padding_bottom', '', '', true ); ?>
                    </div>

                    <div>
                        <h4 class="font-semibold-type2 limit-width mb-4">
							<?php esc_html_e( 'Padding left', 'consent-magic' ); ?>:
                        </h4>
						<?php render_number_text_input( 'cs_d_op_padding_left', '', '', true ); ?>
                    </div>

                    <div>
                        <h4 class="font-semibold-type2 limit-width mb-4">
							<?php esc_html_e( 'Padding right', 'consent-magic' ); ?>:
                        </h4>
						<?php render_number_text_input( 'cs_d_op_padding_right', '', '', true ); ?>
                    </div>
                </div>

                <div class="line"></div>
                <h3 class="primary-heading-type2"><?php esc_html_e( 'Font', 'consent-magic' ); ?></h3>

                <div class="list-item row-design-settings">
                    <div>
                        <h4 class="font-semibold-type2 limit-width mb-4">
							<?php esc_html_e( 'Font size', 'consent-magic' ); ?>:
                        </h4>
						<?php render_number_text_input( 'cs_d_op_font_size', '', '', true ); ?>
                    </div>

                    <div>
                        <h4 class="font-semibold-type2 limit-width mb-4">
							<?php esc_html_e( 'Font weight', 'consent-magic' ); ?>:
                        </h4>
						<?php renderSelectInput( 'cs_d_op_font_weight', ConsentMagic()->getOption( 'cs_font_weight_list' ), true, true, 'short' ); ?>
                    </div>
                </div>

                <div class="list-item row-design-settings">
                    <div>
                        <h4 class="font-semibold-type2 limit-width mb-4">
							<?php esc_html_e( 'Title\'s Font size', 'consent-magic' ); ?>:
                        </h4>
						<?php render_number_text_input( 'cs_d_op_titles_font_size', '', '', true ); ?>
                    </div>

                    <div>
                        <h4 class="font-semibold-type2 limit-width mb-4">
							<?php esc_html_e( 'Title\'s Font weight', 'consent-magic' ); ?>:
                        </h4>
						<?php renderSelectInput( 'cs_d_op_titles_font_weight', ConsentMagic()->getOption( 'cs_font_weight_list' ), true, true, 'short' ); ?>
                    </div>
                </div>

                <div class="list-item row-design-settings">
                    <div>
                        <h4 class="font-semibold-type2 limit-width mb-4">
							<?php esc_html_e( 'Subtitle\'s Font size', 'consent-magic' ); ?>:
                        </h4>
						<?php render_number_text_input( 'cs_d_op_subtitles_font_size', '', '', true ); ?>
                    </div>

                    <div>
                        <h4 class="font-semibold-type2 limit-width mb-4">
							<?php esc_html_e( 'Subtitle\'s Font weight', 'consent-magic' ); ?>:
                        </h4>
						<?php renderSelectInput( 'cs_d_op_subtitles_font_weight', ConsentMagic()->getOption( 'cs_font_weight_list' ), true, true, 'short' ); ?>
                    </div>
                </div>


                <div class="line"></div>
                <h3 class="primary-heading-type2"><?php esc_html_e( 'Button\'s', 'consent-magic' ); ?></h3>

                <div class="list-item row-design-settings">
                    <div>
                        <h4 class="font-semibold-type2 limit-width mb-4">
							<?php esc_html_e( 'Button\'s padding top', 'consent-magic' ); ?>:
                        </h4>
						<?php render_number_text_input( 'cs_d_op_btn_pd_t', '', '', true ); ?>
                    </div>

                    <div>
                        <h4 class="font-semibold-type2 limit-width mb-4">
							<?php esc_html_e( 'Button\'s padding bottom', 'consent-magic' ); ?>:
                        </h4>
						<?php render_number_text_input( 'cs_d_op_btn_pd_bt', '', '', true ); ?>
                    </div>

                    <div>
                        <h4 class="font-semibold-type2 limit-width mb-4">
							<?php esc_html_e( 'Button\'s padding right', 'consent-magic' ); ?>:
                        </h4>
						<?php render_number_text_input( 'cs_d_op_btn_pd_r', '', '', true ); ?>
                    </div>

                    <div>
                        <h4 class="font-semibold-type2 limit-width mb-4">
							<?php esc_html_e( 'Button\'s padding left', 'consent-magic' ); ?>:
                        </h4>
						<?php render_number_text_input( 'cs_d_op_btn_pd_l', '', '', true ); ?>
                    </div>

                    <div>
                        <h4 class="font-semibold-type2 limit-width mb-4">
							<?php esc_html_e( 'Button\'s margin top', 'consent-magic' ); ?>:
                        </h4>
						<?php render_number_text_input( 'cs_d_op_btn_mg_t', '', '', true ); ?>
                    </div>

                    <div>
                        <h4 class="font-semibold-type2 limit-width mb-4">
							<?php esc_html_e( 'Button\'s margin bottom', 'consent-magic' ); ?>:
                        </h4>
						<?php render_number_text_input( 'cs_d_op_btn_mg_bt', '', '', true ); ?>
                    </div>

                    <div>
                        <h4 class="font-semibold-type2 limit-width mb-4">
							<?php esc_html_e( 'Button\'s margin right', 'consent-magic' ); ?>:
                        </h4>
						<?php render_number_text_input( 'cs_d_op_btn_mg_r', '', '', true ); ?>
                    </div>

                    <div>
                        <h4 class="font-semibold-type2 limit-width mb-4">
							<?php esc_html_e( 'Button\'s margin left', 'consent-magic' ); ?>:
                        </h4>
						<?php render_number_text_input( 'cs_d_op_btn_mg_l', '', '', true ); ?>
                    </div>

                    <div>
                        <h4 class="font-semibold-type2 limit-width mb-4">
							<?php esc_html_e( 'Button\'s font size', 'consent-magic' ); ?>:
                        </h4>
						<?php render_number_text_input( 'cs_d_op_btn_font_size', '', '', true ); ?>
                    </div>

                    <div>
                        <h4 class="font-semibold-type2 limit-width mb-4">
							<?php esc_html_e( 'Button\'s font weight', 'consent-magic' ); ?>:
                        </h4>
						<?php renderSelectInput( 'cs_d_op_btn_font_weight', ConsentMagic()->getOption( 'cs_font_weight_list' ), true, true, 'short' ); ?>
                    </div>
                </div>
            </div>
        </div>
    </div>

    <div class="card card-style8">
        <div class="card-header card-header-style3">
            <h3 class="primary-heading-type2">
				<?php esc_html_e( 'Sticky popup', 'consent-magic' ); ?>
            </h3>

			<?php cardCollapseSettingsWithText(); ?>
        </div>

        <div class="card-body">
            <div class="gap-24 design-wrap">
                <h3 class="primary-heading-type2"><?php esc_html_e( 'Content Block', 'consent-magic' ); ?></h3>

                <div class="list-item row-design-settings">
                    <div>
                        <h4 class="font-semibold-type2 limit-width mb-4">
							<?php esc_html_e( 'Minimum height', 'consent-magic' ); ?>:
                        </h4>
						<?php render_number_text_input( 'cs_d_stp_min_height', '', '', true ); ?>
                    </div>

                    <div>
                        <h4 class="font-semibold-type2 limit-width mb-4">
							<?php esc_html_e( 'Padding top', 'consent-magic' ); ?>:
                        </h4>
						<?php render_number_text_input( 'cs_d_stp_padding_top', '', '', true ); ?>
                    </div>

                    <div>
                        <h4 class="font-semibold-type2 limit-width mb-4">
							<?php esc_html_e( 'Padding bottom', 'consent-magic' ); ?>:
                        </h4>
						<?php render_number_text_input( 'cs_d_stp_padding_bottom', '', '', true ); ?>
                    </div>

                    <div>
                        <h4 class="font-semibold-type2 limit-width mb-4">
							<?php esc_html_e( 'Padding left', 'consent-magic' ); ?>:
                        </h4>
						<?php render_number_text_input( 'cs_d_stp_padding_left', '', '', true ); ?>
                    </div>

                    <div>
                        <h4 class="font-semibold-type2 limit-width mb-4">
							<?php esc_html_e( 'Padding right', 'consent-magic' ); ?>:
                        </h4>
						<?php render_number_text_input( 'cs_d_stp_padding_right', '', '', true ); ?>
                    </div>
                </div>

                <div class="line"></div>
                <h3 class="primary-heading-type2"><?php esc_html_e( 'Font', 'consent-magic' ); ?></h3>

                <div class="list-item row-design-settings">
                    <div>
                        <h4 class="font-semibold-type2 limit-width mb-4">
							<?php esc_html_e( 'Font size', 'consent-magic' ); ?>:
                        </h4>
						<?php render_number_text_input( 'cs_d_stp_font_size', '', '', true ); ?>
                    </div>

                    <div>
                        <h4 class="font-semibold-type2 limit-width mb-4">
							<?php esc_html_e( 'Font weight', 'consent-magic' ); ?>:
                        </h4>
						<?php renderSelectInput( 'cs_d_stp_font_weight', ConsentMagic()->getOption( 'cs_font_weight_list' ), true, true, 'short' ); ?>
                    </div>
                </div>

                <div class="line"></div>
                <h3 class="primary-heading-type2"><?php esc_html_e( 'Position', 'consent-magic' ); ?></h3>

                <div class="list-item row-design-settings">
                    <div>
                        <h4 class="font-semibold-type2 mb-4">
							<?php esc_html_e( 'Sticky popup desktop position', 'consent-magic' ); ?>:
                        </h4>
						<?php renderSelectInput( 'cs_d_stp_desktop_position', ConsentMagic()->getOption( 'cs_sticky_position_list' ), true, true, 'middle' ); ?>
                    </div>

                    <div>
                        <h4 class="font-semibold-type2 limit-width mb-4">
							<?php esc_html_e( 'Sticky vertical position', 'consent-magic' ); ?>:
                        </h4>
						<?php renderSelectInput( 'cs_d_stp_sticky_position_vertical', ConsentMagic()->getOption( 'cs_sticky_position_vertical_list' ), true, true, 'short' ); ?>
                    </div>
                </div>
            </div>
        </div>
    </div>
</div>