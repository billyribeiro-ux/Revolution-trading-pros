<?php

namespace ConsentMagicPro;

// If this file is called directly, abort.
if ( !defined( 'WPINC' ) ) {
	die;
}
?>

<div class="cs_preview_design_block list-item">
	<?php include CMPRO_PLUGIN_VIEWS_PATH . "admin/components/spinner.php"; ?>

    <div class="loading_body cs_preview_design_wrap">
        <div class="preview-title"><?php esc_html_e( 'Preview', 'consent-magic' ); ?>:</div>

        <div class="preview-links">
			<?php if ( $design_type == 'multi' ) : ?>
                <div>
                    <a type="button" class="link link-underline preview_color_btn"
                       data-template="bar_small">
						<?php esc_html_e( 'Small bar', 'consent-magic' ); ?>
                    </a>
                </div>

                <div>
                    <a type="button" class="link link-underline preview_color_btn"
                       data-template="bar_large">
						<?php esc_html_e( 'Large bar', 'consent-magic' ); ?>
                    </a>
                </div>

                <div>
                    <a type="button" class="link link-underline preview_color_btn"
                       data-template="popup_small">
						<?php esc_html_e( 'Small popup', 'consent-magic' ); ?>
                    </a>
                </div>
			<?php endif; ?>

            <div>
                <a type="button" class="link link-underline preview_color_btn"
                   data-template="<?php echo $design_type == 'multi' ? 'popup_large' : 'popup_large_single'; ?>">
					<?php esc_html_e( 'Large popup', 'consent-magic' ); ?>
                </a>
            </div>
        </div>
    </div>
</div>