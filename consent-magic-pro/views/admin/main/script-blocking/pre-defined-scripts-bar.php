<?php

namespace ConsentMagicPro;

// If this file is called directly, abort.
if ( !defined( 'WPINC' ) ) {
	die;
}
?>

<div class="cards-wrapper cards-wrapper-style2 gap-22">
    <div class="d-flex align-items-center">
		<?php render_switcher_input( 'cs_block_pre_defined_scripts' ); ?>
        <h4 class="secondary-heading switcher-label"><?php esc_html_e( 'Block pre-defined scripts', 'consent-magic' ); ?></h4>
    </div>

	<?php
	$predefined_scripts = array(
		array(
			'key'  => 'cs_block_ad_storage_scripts',
			'name' => __( 'Google ad_storage', 'consent-magic' ),
		),
		array(
			'key'  => 'cs_block_analytics_storage_scripts',
			'name' => __( 'Google analytics_storage', 'consent-magic' ),
		),
		array(
			'key'  => 'cs_block_ad_user_data_scripts',
			'name' => __( 'Google ad_user_data', 'consent-magic' ),
		),
		array(
			'key'  => 'cs_block_ad_personalization_scripts',
			'name' => __( 'Google ad_personalization', 'consent-magic' ),
		),
		array(
			'key'  => 'cs_block_url_passthrough_scripts',
			'name' => __( 'Google url_passthrough', 'consent-magic' ),
		),
		array(
			'key'  => 'cs_block_google_analytics_scripts',
			'name' => __( 'Google Analytics', 'consent-magic' ),
		),
		array(
			'key'  => 'cs_block_google_ads_tag_scripts',
			'name' => __( 'Google Ads Tag', 'consent-magic' ),
		),
		array(
			'key'  => 'cs_block_google_adsense_scripts',
			'name' => __( 'Google Adsense', 'consent-magic' ),
		),
		array(
			'key'  => 'cs_block_google_maps_scripts',
			'name' => __( 'Google maps', 'consent-magic' ),
		),
		array(
			'key'  => 'cs_block_googlefonts_scripts',
			'name' => __( 'Google Fonts', 'consent-magic' ),
		),
		array(
			'key'  => 'cs_block_google_captcha_scripts',
			'name' => __( 'Google Captcha', 'consent-magic' ),
		),
		array(
			'key'  => 'cs_block_fb_pixel_scripts',
			'name' => __( 'Facebook Pixel', 'consent-magic' ),
		),
		array(
			'key'  => 'cs_block_tiktok_scripts',
			'name' => __( 'TikTok Pixel', 'consent-magic' ),
		),
		array(
			'key'  => 'cs_block_twitter_scripts',
			'name' => __( 'Twitter Pixel', 'consent-magic' ),
		),
		array(
			'key'  => 'cs_block_tw_tag_scripts',
			'name' => __( 'Twitter Tag', 'consent-magic' ),
		),
		array(
			'key'  => 'cs_block_big_tag_scripts',
			'name' => __( 'Microsoft UET Tag', 'consent-magic' ),
		),
		array(
			'key'  => 'cs_block_ln_tag_scripts',
			'name' => __( 'Linkedin Tag', 'consent-magic' ),
		),
		array(
			'key'  => 'cs_block_pin_tag_scripts',
			'name' => __( 'Pinterest Tag', 'consent-magic' ),
		),
		array(
			'key'  => 'cs_block_instagram_scripts',
			'name' => __( 'Instagram embed', 'consent-magic' ),
		),
		array(
			'key'  => 'cs_block_yt_embedded_scripts',
			'name' => __( 'Youtube Embedded', 'consent-magic' ),
		),
		array(
			'key'  => 'cs_block_vimeo_embedded_scripts',
			'name' => __( 'Vimeo Embedded', 'consent-magic' ),
		),
		array(
			'key'  => 'cs_block_soundcloud_scripts',
			'name' => __( 'Soundcloud embed', 'consent-magic' ),
		),
		array(
			'key'  => 'cs_block_slideshare_scripts',
			'name' => __( 'Slideshare embed', 'consent-magic' ),
		),
		array(
			'key'  => 'cs_block_hubspot_scripts',
			'name' => __( 'Hubspot Analytics', 'consent-magic' ),
		),
		array(
			'key'  => 'cs_block_matomo_scripts',
			'name' => __( 'Matomo Analytics', 'consent-magic' ),
		),
		array(
			'key'  => 'cs_block_addthis_scripts',
			'name' => __( 'Addthis widget', 'consent-magic' ),
		),
		array(
			'key'  => 'cs_block_sharethis_scripts',
			'name' => __( 'Sharethis widget', 'consent-magic' ),
		),
		array(
			'key'  => 'cs_block_hotjar_scripts',
			'name' => __( 'Hotjar', 'consent-magic' ),
		),
		array(
			'key'  => 'cs_block_reddit_pixel_scripts',
			'name' => __( 'Reddit pixel', 'consent-magic' ),
		)
	);

	if ( isPYSActivated() ) {
		$predefined_scripts[] = array(
			'key'  => 'cs_block_pys_scripts',
			'name' => __( 'PixelYourSite cookies', 'consent-magic' ),
		);
	}

	if ( isConversionExporterActivated() ) {
		$predefined_scripts[] = array(
			'key'  => 'cs_block_ce_scripts',
			'name' => __( 'Conversion Exporter cookies', 'consent-magic' ),
		);
	}

	render_predefined_scripts_block( $predefined_scripts, $options );
	?>
</div>