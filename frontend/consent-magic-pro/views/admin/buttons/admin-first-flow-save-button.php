<?php

namespace ConsentMagicPro;

// If this file is called directly, abort.
if ( !defined( 'WPINC' ) ) {
	die;
}
?>

<button type="submit"
        name="update_admin_settings_form"
        class="btn btn-primary btn-primary-type2"
        onClick="return cs_store_settings_btn_click(this.name)">
	<?php esc_html_e( 'Finish', 'consent-magic' ); ?>
</button>