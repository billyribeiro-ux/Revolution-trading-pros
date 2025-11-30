<?php

namespace ConsentMagicPro;

// If this file is called directly, abort.
if ( !defined( 'WPINC' ) ) {
	die;
}
?>
<div class="cards-wrapper cards-wrapper-style2 gap-22">
    <h3 class="primary-heading">
		<?php esc_html_e( 'PixelYourSite', 'consent-magic' ); ?>
    </h3>

    <div>
        <div class="mb-4">
			<?php if ( isPYSActivated() ) : ?>
                <p class="fw-500"><?php esc_html_e( 'Active - we detected that you use the PixelYourSite plugin.', 'consent-magic' ); ?></p>
			<?php else :
				render_warning_message( esc_html__( 'PixelYourSite plugin is not active. Please install and activate the PixelYourSite plugin to use this feature.', 'consent-magic' ) );
			endif; ?>
        </div>

        <p class="text-gray"><?php esc_html_e( 'Learn how Facebook Conversion API and Advance Matching work:', 'consent-magic' ); ?>
            <a href="https://www.youtube.com/watch?v=PsKdCkKNeLU"
               target="_blank" class="link"><?php esc_html_e( 'watch video', 'consent-magic' ); ?></a></p>
    </div>

    <div class="line"></div>

    <div class="gap-22 <?php echo !isPYSActivated() ? 'disabled-container' : ''; ?>">
        <div class="d-flex align-items-center">
			<?php render_switcher_input( 'cs_advanced_matching_consent_enabled' ); ?>
            <h4 class="secondary-heading switcher-label"><?php esc_html_e( 'Advanced Matching Consent in the Options Popup', 'consent-magic' ); ?></h4>
        </div>

        <p class="text-gray">
			<?php esc_html_e( 'If you enable this option, the user will have a dedicated Advanced Matching opt-in. This will control the Advanced Matching data sent by the PixelYourSite plugin. It has no effect on the Advanced Matching data automatically collected by the Facebook Pixel.', 'consent-magic' ); ?>
        </p>

		<?php render_language_block_textarea_ajax( 'cs_advanced_matching_description', __( 'Advanced Matching Consent in the Options Popup', 'consent-magic' ), __( 'Description (used in the Options pop-up)', 'consent-magic' ) ); ?>

        <div class="line"></div>

        <div class="d-flex align-items-center">
			<?php render_switcher_input( 'cs_server_side_consent_enabled' ); ?>
            <h4 class="secondary-heading switcher-label"><?php esc_html_e( 'Conversion API (CAPI) Consent in the Options Popup', 'consent-magic' ); ?></h4>
        </div>

        <p class="text-gray">
			<?php esc_html_e( 'If you enable this option, the user will see a dedicated Conversion API category in the Options Popup.', 'consent-magic' ); ?>
        </p>

		<?php render_language_block_textarea_ajax( 'cs_server_side_consent_description', __( 'Conversion API (CAPI) Consent in the Options Popup', 'consent-magic' ), __( 'Description (used in the Options pop-up)', 'consent-magic' ) ); ?>

        <div class="line"></div>

		<?php render_warning_info_message( __( '*The Facebook Pixel will always follow the consent given for its category. These options just add an extra layer of informed consent.', 'consent-magic' ) ); ?>
    </div>
</div>