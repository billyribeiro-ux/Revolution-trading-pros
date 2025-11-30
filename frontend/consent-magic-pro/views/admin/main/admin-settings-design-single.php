<?php

namespace ConsentMagicPro;

// If this file is called directly, abort.
if ( !defined( 'WPINC' ) ) {
	die;
}

$id   = $target_id;
$type = 'single';
?>
<div data-id="<?php echo esc_attr( $target_id ); ?>" id="cs_preview_popup">
    <div class="cards-wrapper cards-wrapper-style2 gap-22">
        <div class="card card-static card-style3">
            <div class="card-header card-header-with-tabs cs_tab_header">
                <ul class="cs_tab templates_tab">
                    <li data-target="templates-bar"><a><?php esc_html_e( 'Templates', 'consent-magic' ); ?></a></li>
                    <li data-target="adjust-bar"><a><?php esc_html_e( 'Adjust sizes', 'consent-magic' ); ?></a></li>
                </ul>
            </div>

            <div class="card-body cs_tab_container">
                <div class="cs_tab_content" data-id="templates-bar" style="display:block;">
					<?php
					//templates-bar
					include CMPRO_PLUGIN_VIEWS_PATH . "admin/main/design/templates-bar.php";
					?>
                </div>

                <div class="cs_tab_content" data-id="adjust-bar">
					<?php
					//adjust-bar
					include CMPRO_PLUGIN_VIEWS_PATH . "admin/main/design/adjust-bar-single.php";
					?>
                </div>
            </div>

			<?php include CMPRO_PLUGIN_VIEWS_PATH . "admin/components/modal-delete-template.php"; ?>
        </div>
    </div>
</div>