<?php

namespace ConsentMagicPro;

// If this file is called directly, abort.
if ( !defined( 'WPINC' ) ) {
	die;
}

?>
<div data-id="<?php echo esc_attr( $target_id ); ?>">
    <div class="cards-wrapper cards-wrapper-style2 text-section-wrapper">

		<?php
		$title    = __( 'Small bar or popup', 'consent-magic' );
		$subtitle = __( 'This text is used when "small bar" or "small popup" are selected', 'consent-magic' );
		$inputs   = array(
			array(
				'type'           => 'textarea',
				'key'            => 'cs_text_in_small_bar_popup',
				'input_title'    => $title,
				'input_subtitle' => '',
			),
		);
		render_language_block_ajax( $inputs, $title, $subtitle, 8 );

		$title    = __( 'Large bar or popup', 'consent-magic' );
		$subtitle = __( 'This text is used when "large bar" or "large popup" are selected', 'consent-magic' );
		$inputs   = array(
			array(
				'type'           => 'textarea',
				'key'            => 'cs_text_in_large_bar_popup',
				'input_title'    => $title,
				'input_subtitle' => '',
			)
		);
		render_language_block_ajax( $inputs, $title, $subtitle, 8 );

		$title    = __( 'Button\'s text', 'consent-magic' );
		$subtitle = __( 'This text is used on buttons on all types of bars and popups', 'consent-magic' );
		$inputs   = array(
			array(
				'type'           => 'input',
				'key'            => 'cs_btn_text_allow_all',
				'input_title'    => __( 'Allow all', 'consent-magic' ),
				'input_subtitle' => __( 'Allow all', 'consent-magic' ),
			),
			array(
				'type'           => 'input',
				'key'            => 'cs_btn_text_custom_button',
				'input_title'    => __( 'Custom button', 'consent-magic' ),
				'input_subtitle' => __( 'Custom button', 'consent-magic' ),
			),
			array(
				'type'           => 'input',
				'key'            => 'cs_btn_text_disable_all',
				'input_title'    => __( 'Disable all', 'consent-magic' ),
				'input_subtitle' => __( 'Disable all', 'consent-magic' ),
			),
			array(
				'type'           => 'input',
				'key'            => 'cs_btn_text_customize',
				'input_title'    => __( 'Customize', 'consent-magic' ),
				'input_subtitle' => __( 'Customize', 'consent-magic' ),
			),
			array(
				'type'           => 'input',
				'key'            => 'cs_btn_text_privacy_cookie',
				'input_title'    => __( 'Privacy & Cookies', 'consent-magic' ),
				'input_subtitle' => __( 'Privacy & Cookies', 'consent-magic' ),
			),
			array(
				'type'           => 'input',
				'key'            => 'cs_btn_text_sticky_cookie',
				'input_title'    => __( 'Sticky', 'consent-magic' ),
				'input_subtitle' => __( 'Sticky', 'consent-magic' ),
			),
		);
		render_language_block_ajax( $inputs, $title, $subtitle, 8 );

		$title    = __( 'Options popups', 'consent-magic' );
		$subtitle = __( 'Customize the content of the options popup', 'consent-magic' );
		$inputs   = array(
			array(
				'type'           => 'textarea',
				'key'            => 'cs_text_in_options_popup',
				'input_title'    => $title,
				'input_subtitle' => '',
			),
			array(
				'type'           => 'input',
				'key'            => 'cs_title_in_options_popup',
				'input_title'    => __( 'Title', 'consent-magic' ),
				'input_subtitle' => __( 'Title', 'consent-magic' ),
			),
			array(
				'type'           => 'input',
				'key'            => 'cs_subtitle_in_options_popup',
				'input_title'    => __( 'Subtitle', 'consent-magic' ),
				'input_subtitle' => __( 'Subtitle', 'consent-magic' ),
			),
			array(
				'type'           => 'input',
				'key'            => 'cs_btn_text_allow_all_in_options_popup',
				'input_title'    => __( 'Allow all', 'consent-magic' ),
				'input_subtitle' => __( 'Allow all', 'consent-magic' ),
			),
			array(
				'type'           => 'input',
				'key'            => 'cs_btn_text_disable_all_in_options_popup',
				'input_title'    => __( 'Disable all', 'consent-magic' ),
				'input_subtitle' => __( 'Disable all', 'consent-magic' ),
			),
			array(
				'type'           => 'input',
				'key'            => 'cs_btn_text_confirm_in_options_popup',
				'input_title'    => __( 'Confirm my choices', 'consent-magic' ),
				'input_subtitle' => __( 'Confirm my choices', 'consent-magic' ),
			),
			array(
				'type'           => 'input',
				'key'            => 'cs_always_on_in_options_popup',
				'input_title'    => __( 'Always ON', 'consent-magic' ),
				'input_subtitle' => __( 'Always ON', 'consent-magic' ),
			),
		);
		render_language_block_ajax( $inputs, $title, $subtitle, 8 );

		$title    = __( 'Single-step', 'consent-magic' );
		$subtitle = __( 'Customize the content of the Single-step', 'consent-magic' );
		$inputs   = array(
			array(
				'type'           => 'textarea',
				'key'            => 'cs_text_in_single_design',
				'input_title'    => $title,
				'input_subtitle' => '',
			),
			array(
				'type'           => 'input',
				'key'            => 'cs_title_in_single_design',
				'input_title'    => __( 'Title', 'consent-magic' ),
				'input_subtitle' => __( 'Title', 'consent-magic' ),
			),
			array(
				'type'           => 'input',
				'key'            => 'cs_subtitle_in_single_design',
				'input_title'    => __( 'Subtitle', 'consent-magic' ),
				'input_subtitle' => __( 'Subtitle', 'consent-magic' ),
			),
			array(
				'type'           => 'input',
				'key'            => 'cs_btn_text_allow_all_in_single_design',
				'input_title'    => __( 'Allow all', 'consent-magic' ),
				'input_subtitle' => __( 'Allow all', 'consent-magic' ),
			),
			array(
				'type'           => 'input',
				'key'            => 'cs_btn_text_disable_all_in_single_design',
				'input_title'    => __( 'Disable all', 'consent-magic' ),
				'input_subtitle' => __( 'Disable all', 'consent-magic' ),
			),
			array(
				'type'           => 'input',
				'key'            => 'cs_btn_text_confirm_in_single_design',
				'input_title'    => __( 'Confirm my choices', 'consent-magic' ),
				'input_subtitle' => __( 'Confirm my choices', 'consent-magic' ),
			),
			array(
				'type'           => 'input',
				'key'            => 'cs_btn_text_custom_button_in_single_design',
				'input_title'    => __( 'Custom button', 'consent-magic' ),
				'input_subtitle' => __( 'Custom button', 'consent-magic' ),
			),
			array(
				'type'           => 'input',
				'key'            => 'cs_always_on_in_single_design',
				'input_title'    => __( 'Always ON', 'consent-magic' ),
				'input_subtitle' => __( 'Always ON', 'consent-magic' ),
			),
		);
		render_language_block_ajax( $inputs, $title, $subtitle, 8 );

		$title    = __( 'Video consent message', 'consent-magic' );
		$subtitle = __( 'This text is used in video instead of video content', 'consent-magic' );
		$inputs   = array(
			array(
				'type'           => 'textarea',
				'key'            => 'cs_video_consent_general_text',
				'input_title'    => __( 'General text', 'consent-magic' ),
				'input_subtitle' => __( 'General text', 'consent-magic' ),
			),
			array(
				'type'           => 'textarea',
				'key'            => 'cs_video_consent_rule_text',
				'input_title'    => __( 'Rule text', 'consent-magic' ),
				'input_subtitle' => __( 'Rule text', 'consent-magic' ),
			),
		);
		render_language_block_ajax( $inputs, $title, $subtitle, 8 );

		?>
        <div class="card card-style1">
            <div class="card-header card-header-style3">
                <div class="gap-8">
                    <h3 class="primary-heading"><?php esc_html_e( 'IAB', 'consent-magic' ); ?></h3>
                    <p class="text-gray"><?php esc_html_e( 'This text is used in IAB consent content', 'consent-magic' ); ?></p>
                </div>

				<?php cardCollapseSettingsWithText(); ?>
            </div>

            <div class="card-body">
                <p>
					<?php esc_html_e( 'Due to the strict requirements by IAB regarding the consent text show to users, is not possible to edit the text for the IAB rule.', 'consent-magic' ); ?>
                </p>
            </div>
        </div>
    </div>
</div>