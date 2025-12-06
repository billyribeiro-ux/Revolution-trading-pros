<?php

namespace ConsentMagicPro;

// If this file is called directly, abort.
if ( !defined( 'WPINC' ) ) {
	die;
}

?>

<div class="templates-bar-wrap">
	<?php ConsentMagic()->cs_get_template_list( 'cs_orange_theme', $type ); ?>
	<?php ConsentMagic()->cs_get_template_list( 'cs_light_theme', $type ); ?>
	<?php ConsentMagic()->cs_get_template_list( 'cs_dark_theme', $type ); ?>
	<?php ConsentMagic()->cs_get_template_list( false, $type ); ?>
    <div>
		<?php
		include CMPRO_PLUGIN_VIEWS_PATH . "admin/buttons/admin-create-new-template-button.php";
		?>
    </div>
</div>