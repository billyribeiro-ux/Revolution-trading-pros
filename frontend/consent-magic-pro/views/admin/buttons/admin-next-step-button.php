<?php

namespace ConsentMagicPro;


// If this file is called directly, abort.
if ( !defined( 'WPINC' ) ) {
	die;
}

/**
 * @var  $current_step
 */
?>

<button type="button" class="btn btn-primary btn-primary-type2 btn_next_step">
	<?php echo sprintf( esc_html__( 'Go to step %d', 'consent-magic' ), $current_step + 1 ); ?>
</button>