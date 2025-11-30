<?php

namespace ConsentMagicPro;

// If this file is called directly, abort.
if ( !defined( 'WPINC' ) ) {
	die;
}
?>
<a class="btn btn-primary btn-primary-type2 d-inline-block"
   href="<?php echo esc_url( get_admin_url( null, 'admin.php?page=consent-magic' ) . '&design=' . $_GET[ 'tab' ] . '&design_template=add' ); ?>">
	<?php esc_html_e( 'New template', 'consent-magic' ); ?>
</a>