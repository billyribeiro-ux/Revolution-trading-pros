<?php

namespace ConsentMagicPro;

// If this file is called directly, abort.
if ( !defined( 'WPINC' ) ) {
	die;
}

$unassigned    = get_term_by( 'slug', 'unassigned', 'cs-cookies-category' );
$unassigned_id = $unassigned->term_id;
$options       = get_cookies_terms_list( $unassigned_id );
?>

<div data-id="pre-defined-scripts-bar" class="pre-defined-scripts-bar">
    <div class="cards-wrapper cards-wrapper-style2 gap-22">
        <div class="card card-static card-style3">
            <div class="card-header card-header-with-tabs cs_tab_header cs_tab_header_draggable">
                <ul class="cs_tab cs_tab__script_block">
                    <li data-target="scanner-cookies-bar"><a><?php esc_html_e( 'Scanner', 'consent-magic' ); ?></a></li>
                    <li data-target="pre-defined-scripts-bar">
                        <a><?php esc_html_e( 'Pre-defined Scripts', 'consent-magic' ); ?></a></li>
                    <li data-target="manual-scripts-bar">
                        <a><?php esc_html_e( 'Custom Scripts', 'consent-magic' ); ?></a></li>
                    <li data-target="settings-bar"><a><?php esc_html_e( 'Categories', 'consent-magic' ); ?></a></li>
                    <li data-target="facebook-bar"><a><?php esc_html_e( 'Facebook', 'consent-magic' ); ?></a></li>
                </ul>
            </div>

            <div class="card-body cs_tab_container">
                <div class="cs_tab_content" data-id="pre-defined-scripts-bar" style="display:block;">
					<?php
					//pre-defined-scripts-bar
					include_once CMPRO_PLUGIN_VIEWS_PATH . "admin/main/script-blocking/pre-defined-scripts-bar.php";
					?>
                </div>

                <div class="cs_tab_content" data-id="scanner-cookies-bar">
					<?php
					//scanner-cookies-bar
					include_once CMPRO_PLUGIN_VIEWS_PATH . "admin/main/script-blocking/scanner-cookies-bar.php";
					?>
                </div>

                <div class="cs_tab_content" data-id="manual-scripts-bar">
					<?php
					//manual-scripts-bar
					include_once CMPRO_PLUGIN_VIEWS_PATH . "admin/main/script-blocking/manual-scripts-bar.php";
					?>
                </div>

                <div class="cs_tab_content" data-id="facebook-bar">
					<?php
					//facebook bar
					include_once CMPRO_PLUGIN_VIEWS_PATH . "admin/main/script-blocking/facebook-bar.php";
					?>
                </div>

                <div class="cs_tab_content" data-id="settings-bar">
					<?php
					//settings-bar
					include_once CMPRO_PLUGIN_VIEWS_PATH . "admin/main/script-blocking/settings-bar.php";
					?>
                </div>
            </div>
        </div>
    </div>
</div>