<?php

namespace ConsentMagicPro;

// If this file is called directly, abort.
if ( !defined( 'WPINC' ) ) {
	die;
}
?>
<form method="post" enctype="multipart/form-data">
    <input type="hidden" id="cs_import_settings_nonce"
           value="<?php echo wp_create_nonce( "import_events_file_nonce" ) ?>"/>
    <input type="file" id="cs_import_settings" name="cs_import_settings" accept="application/json"/>
    <label for="cs_import_settings" class="cs_import_settings_label">
        <a type="button" class="btn btn-secondary">
			<?php esc_html_e( 'Import settings', 'consent-magic' ); ?>
        </a>
    </label>
</form>
