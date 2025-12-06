<?php

namespace ConsentMagicPro;

// If this file is called directly, abort.
if ( !defined( 'WPINC' ) ) {
	die;
}
if ( !isset( $absolute_position ) ) {
	$absolute_position = true;
}
?>
<div class="cm-loader-wrap <?php echo $absolute_position ? 'loader-absolute' : ''; ?>">
    <img src="<?php echo esc_url( CMPRO_PLUGIN_URL . '/assets/images/loader.svg' ); ?>" class="cm-loader"
         alt="cm-loader"/>
</div>
