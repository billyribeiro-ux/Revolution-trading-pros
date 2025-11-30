<?php

namespace ConsentMagicPro;

// If this file is called directly, abort.
if ( !defined( 'WPINC' ) ) {
	die;
}
?>

<div class="cards-wrapper cards-wrapper-style2">
    <div class="card card-static card-style9">
        <div class="card-body">
            <div class="gap-24">
                <h3 class="primary-heading-type2"><?php esc_html_e( 'Shortcodes', 'consent-magic' ); ?></h3>

                <div class="shortcodes-wrap">
                    <div class="shortcode-wrap">
						<?php $code = '[cm_options]' . __( 'some text here', 'consent-magic' ) . '[/cm_options]'; ?>
                        <div class="shortcode">
                            <div class="shortcode-inner">
                                <span class="shortcode-title">
                                    <?php echo esc_html( '[cm_options]' ); ?>
                                </span>
                                <span class="fw-500">
                                    <?php echo esc_html__( 'some text here', 'consent-magic' ); ?>
                                </span>
                                <span class="shortcode-title">
                                    <?php echo esc_html( '[/cm_options]' ); ?>
                                </span>
                            </div>

                            <div class="shortcode-copy"
                                 onclick="copyShortcode( '<?php echo esc_html( $code ); ?>', this)">
								<?php renderTooltip( 'icon-content-copy' ); ?>
                            </div>
                        </div>

                        <div class="line"></div>
                        <div class="shortcode-content mb-24">
                            <p>
								<?php echo esc_html__( 'It will add a link that opens the options popup.', 'consent-magic' ); ?>
                            </p>
                        </div>

                        <div class="gap-16">
							<?php render_radio_input( 'cs_hide_shortcode_cm_options', 'show', esc_html__( 'For "Just inform", open the main consent message (no options popup is available for this consent type)', 'consent-magic' ) ); ?>
							<?php render_radio_input( 'cs_hide_shortcode_cm_options', 'hide', esc_html__( 'Hide it for "Just inform"', 'consent-magic' ) ); ?>
                        </div>
                    </div>
					<?php $shortcodes = array(
						array(
							'code'        => array(
								array(
									'text' => '[cm_privacy_link]',
									'type' => 'code',
								),
								array(
									'text' => __( 'some text here', 'consent-magic' ),
									'type' => 'text',
								),
								array(
									'text' => '[/cm_privacy_link]',
									'type' => 'code',
								),
							),
							'description' => __( 'It will add a link that opens the privacy page configured inside the plugin.', 'consent-magic' ),
						),
						array(
							'code'        => '[cm_categories]',
							'description' => __( 'It will display a list with cookies/scripts categories.', 'consent-magic' ),
						),
						array(
							'code'        => '[disable_cm]',
							'description' => __( 'Use this shortcode if you want to disable ConsentMagic on a particular page.', 'consent-magic' ),
						),
						array(
							'code'        => '[cm_remove_sticky]',
							'description' => __( 'Use this shortcode if you want to remove the sticky button on a particular page. It doesn\'t work with the IAB rule, the sticky button is required there.', 'consent-magic' ),
						),
						array(
							'code'        => '[cm_scan]',
							'description' => __( 'It will show all cookies and scripts detected by the last scan.', 'consent-magic' ),
						),
						array(
							'code'        => '[cm_scan_cookies]',
							'description' => __( 'It will show all cookies detected by the last scan.', 'consent-magic' ),
						),
						array(
							'code'        => '[cm_scan_scripts]',
							'description' => __( 'It will show all scripts detected by the last scan.', 'consent-magic' ),
						),
						array(
							'code'        => '[cm_scan_description]',
							'description' => __( 'It will show all cookies and scripts detected by the last scan and their descriptions.', 'consent-magic' ),
						),
						array(
							'code'        => '[cm_scan_cookies_description]',
							'description' => __( 'It will show all cookies detected by the last scan and their descriptions.', 'consent-magic' ),
						),
						array(
							'code'        => '[cm_scan_scrips_description]',
							'description' => __( 'It will show all scripts detected by the last scan and their descriptions.', 'consent-magic' ),
						),
					);

					renderShortcodeBlock( $shortcodes );
					?>
                </div>
            </div>
        </div>
    </div>
</div>
