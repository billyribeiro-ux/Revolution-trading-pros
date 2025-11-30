<?php
namespace ConsentMagicPro;

// If this file is called directly, abort.
if ( !defined( 'WPINC' ) ) {
	die;
}

$license_type = CMPRO_LICENSE_TYPE; // 'edd' or 'woo'
?>

<div class="container-middle">
    <form action="<?php echo esc_url( admin_url( 'admin-post.php' ) ); ?>" method="post"
          data-type="<?php echo ( $license_type == 'edd' ) ? 'edd' : 'wc_am_client'; ?>"
          id="cs_settings_form" class="cm-settings-form">
		<?php
		// Set nonce:
		if ( function_exists( 'wp_create_nonce' ) ) {
			$nonce_value = wp_create_nonce( 'cs-update-' . CMPRO_SETTINGS_FIELD );
			echo '<input type="hidden" id="cm_nonce" name="_wpnonce" value="' . esc_attr( $nonce_value ) . '">';
		}
        ?>

        <div class="cs-tab-content">
			<?php
			include_once CMPRO_PLUGIN_VIEWS_PATH . "admin/license/html-{$license_type}-license.php";
			?>
        </div>
    </form>
</div>