<?php

namespace ConsentMagicPro;

// If this file is called directly, abort.
if ( !defined( 'WPINC' ) ) {
	die;
}

?>
<input type="hidden" name="cm-restore-defaults-design" id="cm-restore-defaults-design" value="">
<div class="cs_preview_design_block mb-24">
	<?php include CMPRO_PLUGIN_VIEWS_PATH . "admin/components/spinner.php"; ?>
    <div class="loading_body cs_preview_design_wrap">
        <div class="preview-title"><?php esc_html_e( 'Preview', 'consent-magic' ); ?>:</div>

        <div class="preview-links">
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
							<?php esc_html_e( 'Padding top', 'consent-magic' ); ?>:
                        </h4>
						<?php render_number_text_input( 'cs_d_single_op_padding_top', '', '', true ); ?>
                    </div>

                    <div>
                        <h4 class="font-semibold-type2 limit-width mb-4">
							<?php esc_html_e( 'Padding bottom', 'consent-magic' ); ?>:
                        </h4>
						<?php render_number_text_input( 'cs_d_single_op_padding_bottom', '', '', true ); ?>
                    </div>

                    <div>
                        <h4 class="font-semibold-type2 limit-width mb-4">
							<?php esc_html_e( 'Padding left', 'consent-magic' ); ?>:
                        </h4>
						<?php render_number_text_input( 'cs_d_single_op_padding_left', '', '', true ); ?>
                    </div>

                    <div>
                        <h4 class="font-semibold-type2 limit-width mb-4">
							<?php esc_html_e( 'Padding right', 'consent-magic' ); ?>:
                        </h4>
						<?php render_number_text_input( 'cs_d_single_op_padding_right', '', '', true ); ?>
                    </div>
                </div>

                <div class="line"></div>
                <h3 class="primary-heading-type2"><?php esc_html_e( 'Font', 'consent-magic' ); ?></h3>

                <div class="list-item row-design-settings">
                    <div>
                        <h4 class="font-semibold-type2 limit-width mb-4">
							<?php esc_html_e( 'Font size', 'consent-magic' ); ?>:
                        </h4>
						<?php render_number_text_input( 'cs_d_single_op_font_size', '', '', true ); ?>
                    </div>

                    <div>
                        <h4 class="font-semibold-type2 limit-width mb-4">
							<?php esc_html_e( 'Font weight', 'consent-magic' ); ?>:
                        </h4>
						<?php renderSelectInput( 'cs_d_single_op_font_weight', ConsentMagic()->getOption( 'cs_font_weight_list' ), true, true, 'short' ); ?>
                    </div>
                </div>

                <div class="list-item row-design-settings">
                    <div>
                        <h4 class="font-semibold-type2 limit-width mb-4">
							<?php esc_html_e( 'Second font size', 'consent-magic' ); ?>:
                        </h4>
						<?php render_number_text_input( 'cs_d_single_op_second_font_size', '', '', true ); ?>
                    </div>

                    <div>
                        <h4 class="font-semibold-type2 limit-width mb-4">
							<?php esc_html_e( 'Second font weight', 'consent-magic' ); ?>:
                        </h4>
						<?php renderSelectInput( 'cs_d_single_op_second_font_weight', ConsentMagic()->getOption( 'cs_font_weight_list' ), true, true, 'short' ); ?>
                    </div>
                </div>

                <div class="list-item row-design-settings">
                    <div>
                        <h4 class="font-semibold-type2 limit-width mb-4">
							<?php esc_html_e( 'Title\'s Font size', 'consent-magic' ); ?>:
                        </h4>
						<?php render_number_text_input( 'cs_d_single_op_titles_font_size', '', '', true ); ?>
                    </div>

                    <div>
                        <h4 class="font-semibold-type2 limit-width mb-4">
							<?php esc_html_e( 'Title\'s Font weight', 'consent-magic' ); ?>:
                        </h4>
						<?php renderSelectInput( 'cs_d_single_op_titles_font_weight', ConsentMagic()->getOption( 'cs_font_weight_list' ), true, true, 'short' ); ?>
                    </div>
                </div>

                <div class="list-item row-design-settings">
                    <div>
                        <h4 class="font-semibold-type2 limit-width mb-4">
							<?php esc_html_e( 'Subtitle\'s Font size', 'consent-magic' ); ?>:
                        </h4>
						<?php render_number_text_input( 'cs_d_single_op_subtitles_font_size', '', '', true ); ?>
                    </div>

                    <div>
                        <h4 class="font-semibold-type2 limit-width mb-4">
							<?php esc_html_e( 'Subtitle\'s Font weight', 'consent-magic' ); ?>:
                        </h4>
						<?php renderSelectInput( 'cs_d_single_op_subtitles_font_weight', ConsentMagic()->getOption( 'cs_font_weight_list' ), true, true, 'short' ); ?>
                    </div>
                </div>

                <div class="list-item row-design-settings">
                    <div>
                        <h4 class="font-semibold-type2 limit-width mb-4">
							<?php esc_html_e( 'Sub subtitle\'s Font size', 'consent-magic' ); ?>:
                        </h4>
						<?php render_number_text_input( 'cs_d_single_op_subsubtitles_font_size', '', '', true ); ?>
                    </div>

                    <div>
                        <h4 class="font-semibold-type2 limit-width mb-4">
							<?php esc_html_e( 'Sub subtitle\'s Font weight', 'consent-magic' ); ?>:
                        </h4>
						<?php renderSelectInput( 'cs_d_single_op_subsubtitles_font_weight', ConsentMagic()->getOption( 'cs_font_weight_list' ), true, true, 'short' ); ?>
                    </div>
                </div>

                <div class="line"></div>
                <h3 class="primary-heading-type2"><?php esc_html_e( 'Button\'s', 'consent-magic' ); ?></h3>

                <div class="list-item row-design-settings">
                    <div>
                        <h4 class="font-semibold-type2 limit-width mb-4">
							<?php esc_html_e( 'Button\'s padding top', 'consent-magic' ); ?>:
                        </h4>
						<?php render_number_text_input( 'cs_d_single_op_btn_pd_t', '', '', true ); ?>
                    </div>

                    <div>
                        <h4 class="font-semibold-type2 limit-width mb-4">
							<?php esc_html_e( 'Button\'s padding bottom', 'consent-magic' ); ?>:
                        </h4>
						<?php render_number_text_input( 'cs_d_single_op_btn_pd_b', '', '', true ); ?>
                    </div>

                    <div>
                        <h4 class="font-semibold-type2 limit-width mb-4">
							<?php esc_html_e( 'Button\'s padding right', 'consent-magic' ); ?>:
                        </h4>
						<?php render_number_text_input( 'cs_d_single_op_btn_pd_r', '', '', true ); ?>
                    </div>

                    <div>
                        <h4 class="font-semibold-type2 limit-width mb-4">
							<?php esc_html_e( 'Button\'s padding left', 'consent-magic' ); ?>:
                        </h4>
						<?php render_number_text_input( 'cs_d_single_op_btn_pd_l', '', '', true ); ?>
                    </div>

                    <div>
                        <h4 class="font-semibold-type2 limit-width mb-4">
							<?php esc_html_e( 'Button\'s margin top', 'consent-magic' ); ?>:
                        </h4>
						<?php render_number_text_input( 'cs_d_single_op_btn_mg_t', '', '', true ); ?>
                    </div>

                    <div>
                        <h4 class="font-semibold-type2 limit-width mb-4">
							<?php esc_html_e( 'Button\'s margin bottom', 'consent-magic' ); ?>:
                        </h4>
						<?php render_number_text_input( 'cs_d_single_op_btn_mg_b', '', '', true ); ?>
                    </div>

                    <div>
                        <h4 class="font-semibold-type2 limit-width mb-4">
							<?php esc_html_e( 'Button\'s margin right', 'consent-magic' ); ?>:
                        </h4>
						<?php render_number_text_input( 'cs_d_single_op_btn_mg_r', '', '', true ); ?>
                    </div>

                    <div>
                        <h4 class="font-semibold-type2 limit-width mb-4">
							<?php esc_html_e( 'Button\'s margin left', 'consent-magic' ); ?>:
                        </h4>
						<?php render_number_text_input( 'cs_d_single_op_btn_mg_l', '', '', true ); ?>
                    </div>

                    <div>
                        <h4 class="font-semibold-type2 limit-width mb-4">
							<?php esc_html_e( 'Button\'s font size', 'consent-magic' ); ?>:
                        </h4>
						<?php render_number_text_input( 'cs_d_single_op_btn_font_size', '', '', true ); ?>
                    </div>

                    <div>
                        <h4 class="font-semibold-type2 limit-width mb-4">
							<?php esc_html_e( 'Button\'s font weight', 'consent-magic' ); ?>:
                        </h4>
						<?php renderSelectInput( 'cs_d_single_op_btn_font_weight', ConsentMagic()->getOption( 'cs_font_weight_list' ), true, true, 'short' ); ?>
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
						<?php render_number_text_input( 'cs_d_single_stp_min_height', '', '', true ); ?>
                    </div>

                    <div>
                        <h4 class="font-semibold-type2 limit-width mb-4">
							<?php esc_html_e( 'Padding top', 'consent-magic' ); ?>:
                        </h4>
						<?php render_number_text_input( 'cs_d_single_stp_padding_top', '', '', true ); ?>
                    </div>

                    <div>
                        <h4 class="font-semibold-type2 limit-width mb-4">
							<?php esc_html_e( 'Padding bottom', 'consent-magic' ); ?>:
                        </h4>
						<?php render_number_text_input( 'cs_d_single_stp_padding_bottom', '', '', true ); ?>
                    </div>

                    <div>
                        <h4 class="font-semibold-type2 limit-width limit-width mb-4">
							<?php esc_html_e( 'Padding left', 'consent-magic' ); ?>:
                        </h4>
						<?php render_number_text_input( 'cs_d_single_stp_padding_left', '', '', true ); ?>
                    </div>

                    <div>
                        <h4 class="font-semibold-type2 limit-width mb-4">
							<?php esc_html_e( 'Padding right', 'consent-magic' ); ?>:
                        </h4>
						<?php render_number_text_input( 'cs_d_single_stp_padding_right', '', '', true ); ?>
                    </div>
                </div>

                <div class="line"></div>
                <h3 class="primary-heading-type2"><?php esc_html_e( 'Font', 'consent-magic' ); ?></h3>

                <div class="list-item row-design-settings">
                    <div>
                        <h4 class="font-semibold-type2 limit-width mb-4">
							<?php esc_html_e( 'Font size', 'consent-magic' ); ?>:
                        </h4>
						<?php render_number_text_input( 'cs_d_single_stp_font_size', '', '', true ); ?>
                    </div>

                    <div>
                        <h4 class="font-semibold-type2 limit-width mb-4">
							<?php esc_html_e( 'Font weight', 'consent-magic' ); ?>:
                        </h4>
						<?php renderSelectInput( 'cs_d_single_stp_font_weight', ConsentMagic()->getOption( 'cs_font_weight_list' ), true, true, 'short' ); ?>
                    </div>
                </div>

                <div class="line"></div>
                <h3 class="primary-heading-type2"><?php esc_html_e( 'Position', 'consent-magic' ); ?></h3>

                <div class="list-item row-design-settings">
                    <div>
                        <h4 class="font-semibold-type2 mb-4">
							<?php esc_html_e( 'Sticky popup desktop position', 'consent-magic' ); ?>:
                        </h4>
						<?php renderSelectInput( 'cs_d_single_stp_desktop_position', ConsentMagic()->getOption( 'cs_sticky_position_list' ), true, true, 'middle' ); ?>
                    </div>

                    <div>
                        <h4 class="font-semibold-type2 limit-width mb-4">
							<?php esc_html_e( 'Sticky vertical position', 'consent-magic' ); ?>:
                        </h4>
						<?php renderSelectInput( 'cs_d_single_stp_sticky_position_vertical', ConsentMagic()->getOption( 'cs_sticky_position_vertical_list' ), true, true, 'short' ); ?>
                    </div>
                </div>
            </div>
        </div>
    </div>
</div>
