<?php

namespace ConsentMagicPro;

// If this file is called directly, abort.
if ( !defined( 'WPINC' ) ) {
	die;
}

if ( !class_exists( 'ConsentMagicPro\CS_Scanner' ) ) {
	render_critical_message( sprintf( esc_html__( 'Scanner module is disabled. To continue using the scanner, turn on option ', 'consent-magic' ) . '<a href="%s" class="link">' . esc_html__( '"Scanner module" in Settings', 'consent-magic' ) . '</a>', esc_url( get_admin_url( null, 'admin.php?page=cs-settings' ) ) ) );

	return;
}

$per_request = array(
	'1'  => '1',
	'2'  => '2',
	'3'  => '3',
	'4'  => '4',
	'5'  => '5',
	'6'  => '6',
	'7'  => '7',
	'8'  => '8',
	'9'  => '9',
	'10' => '10',
);

$scan_interval = array(
	'once_a_week'  => __( 'Once a week', 'consent-magic' ),
	'once_a_month' => __( 'Once a month', 'consent-magic' ),
	'never'        => __( 'Never', 'consent-magic' ),
);

$auto_scan_type = array(
	'scan_all_pages' => __( 'Scan all pages', 'consent-magic' ),
	'scan_key_pages' => __( 'Scan key pages', 'consent-magic' )
);

$cs_scanner = new CS_Scanner;

$last_scan = $cs_scanner->get_last_scan();

$table_data         = get_scan_cookies_list();
$unassigned_counter = 0;

if ( $table_data ) {
	foreach ( $table_data as $item ) {
		if ( $item[ 'term_slug' ] === 'unassigned' ) {
			$unassigned_counter++;
		}
	}
}

?>
<div class="gap-24">
    <div>
        <h4 class="mb-4 fw-500 lh-134"><?php esc_html_e( 'The scan will detect pre-defined scripts, custom scripts and cookies. You can use shortcodes to display the results to your website visitors.', 'consent-magic' ); ?></h4>
        <p class="text-gray lh-134"><?php esc_html_e( 'Learn how the scanner work:', 'consent-magic' ); ?> <a
                    href="https://www.youtube.com/watch?v=Dpj-qSNb3eE"
                    target="_blank" class="link"><?php esc_html_e( 'watch video', 'consent-magic' ); ?></a></p>
    </div>

    <div class="line"></div>

	<?php if ( $last_scan ) : ?>
        <div class="scanned-scripts">
            <i class="icon-check"></i>
            <span class="scanned-label">
                <?php esc_html_e( 'Last scan ' . $last_scan[ 'created_at' ] . ', ' . $last_scan[ 'total_url' ] . ' scanned URLs, ' . $last_scan[ 'total_scripts' ] . ' scripts, ' . $last_scan[ 'total_cookies' ] . ' cookies detected', 'consent-magic' ); ?>
            </span>
        </div>

        <div class="line"></div>
	<?php endif; ?>

    <h3 class="primary-heading-type2"><?php esc_html_e( 'Scan Settings', 'consent-magic' ); ?></h3>

    <div>
        <p class="text-gray mb-12">
			<?php esc_html_e( 'Newly detected cookies will go under the Unassigned category, and you can manually assign them to the correct category. You can manage categories from ', 'consent-magic' ); ?>
            <a href="#" data-target="settings-bar"
               class="link open_target_tab"><?php esc_html_e( 'here', 'consent-magic' ); ?></a>.
        </p>

        <p class="text-gray mb-4">
			<?php esc_html_e( 'You can change pre-defined scripts category from the ', 'consent-magic' ); ?>
            <a
                    href="#" data-target="pre-defined-scripts-bar"
                    class="link open_target_tab"><?php esc_html_e( 'Pre-defined scripts page', 'consent-magic' ); ?></a>.
        </p>

        <p class="text-gray">
			<?php esc_html_e( 'You can change custom scripts category from the ', 'consent-magic' ); ?><a
                    href="#"
                    data-target="manual-scripts-bar"
                    class="link open_target_tab"><?php esc_html_e( 'Custom scripts page', 'consent-magic' ); ?></a>.
        </p>
    </div>

    <div class="line"></div>

    <div class="scan-settings-wrap">
        <div class="scan-setting-item">
            <h4 class="font-semibold-type2 mb-8"><?php esc_html_e( 'Auto scan type', 'consent-magic' ); ?></h4>
			<?php renderSelectInput( 'cs_auto_scan_type', $auto_scan_type, true ); ?>
        </div>

        <div class="scan-setting-item">
            <h4 class="font-semibold-type2 mb-8"><?php esc_html_e( 'Cookie scanner URL per request', 'consent-magic' ); ?></h4>
			<?php renderSelectInput( 'cs_scan_url_per_request', $per_request, true ); ?>
        </div>

        <div class="scan-setting-item">
            <h4 class="font-semibold-type2 mb-8"><?php esc_html_e( 'Auto scan', 'consent-magic' ); ?></h4>
			<?php renderSelectInput( 'cs_auto_scan_interval', $scan_interval, true ); ?>
        </div>
    </div>

    <div>
        <div class="d-flex align-items-center mb-12">
			<?php render_switcher_input( 'cs_auto_scan_email_enabled' ); ?>
            <h4 class="secondary-heading switcher-label"><?php esc_html_e( 'When a new auto-scan is completed, email to this address:', 'consent-magic' ); ?></h4>
        </div>

		<?php renderProofDeletingEmail( 'cs_auto_scan_email' ); ?>
    </div>

    <div class="line"></div>

    <div class="run-scan-wrap">
        <div class="run-scan">
            <div class="run-scan-button-wrap">
                <button type="button"
                        class="btn btn-primary btn-primary-type2 cs_cookies_scan"><?php esc_html_e( 'Run scan now', 'consent-magic' ); ?></button>
				<?php include CMPRO_PLUGIN_VIEWS_PATH . "admin/components/spinner.php"; ?>
            </div>

            <p class="fw-500">
				<?php esc_html_e( 'You can run a scan now if you made any significant changes to your website.', 'consent-magic' ); ?>
            </p>
        </div>

        <p class="cm-scan-bar text-small text-gray"></p>
    </div>

    <div class="line"></div>
    <h3 class="primary-heading-type2"><?php esc_html_e( 'Scripts', 'consent-magic' ); ?></h3>

    <div>
        <p class="text-gray mb-8">
			<?php esc_html_e( "The plugin will control pre-defined and custom scripts according to the rules' settings and customers' preferences.", 'consent-magic' ); ?>
        </p>

        <p class="text-gray">
			<?php esc_html_e( 'These are the ', 'consent-magic' ); ?><a href="#" data-target="pre-defined-scripts-bar"
                                                                        class="open_target_tab link"><?php esc_html_e( 'pre-defined', 'consent-magic' ); ?></a>
			<?php esc_html_e( ' and ', 'consent-magic' ); ?><a href="#" data-target="manual-scripts-bar"
                                                               class="open_target_tab link"><?php esc_html_e( 'custom', 'consent-magic' ); ?></a>
			<?php esc_html_e( ' scripts we detected on your site:', 'consent-magic' ); ?>
        </p>
    </div>

    <div class="cm-table-wrap">
        <table id="custom_scripts" class="display cm-table">
            <thead>
            <tr>
                <th><?php esc_html_e( 'Script name', 'consent-magic' ); ?></th>
                <th><?php esc_html_e( 'Script category', 'consent-magic' ); ?></th>
                <th></th>
            </tr>
            </thead>
            <tfoot>
            <tr>
                <th><?php esc_html_e( 'Script name', 'consent-magic' ); ?></th>
                <th><?php esc_html_e( 'Script category', 'consent-magic' ); ?></th>
                <th></th>
            </tr>
            </tfoot>
        </table>
    </div>

    <div class="line"></div>
    <h3 class="primary-heading-type2"><?php esc_html_e( 'Cookies', 'consent-magic' ); ?></h3>
    <p class="text-gray lh-162"><?php esc_html_e( 'You can show the list with your cookies using the dedicated shortcodes.', 'consent-magic' ); ?></p>

	<?php if ( $unassigned_counter > 0 ) : ?>
		<?php render_warning_info_message( __( "You have $unassigned_counter Unassigned cookies. Assign them to the correct category and update settings.", 'consent-magic' ) ); ?>
	<?php endif; ?>

    <div class="cm-table-wrap">
        <table id="custom_cookies" class="display cm-table">
            <thead>
            <tr>
                <th><?php esc_html_e( 'Cookie name', 'consent-magic' ); ?></th>
                <th><?php esc_html_e( 'Cookie category', 'consent-magic' ); ?></th>
                <th></th>
            </tr>
            </thead>
            <tfoot>
            <tr>
                <th><?php esc_html_e( 'Cookie name', 'consent-magic' ); ?></th>
                <th><?php esc_html_e( 'Cookie category', 'consent-magic' ); ?></th>
                <th></th>
            </tr>
            </tfoot>
        </table>
    </div>

    <div class="line"></div>
    <h3 class="primary-heading-type2"><?php esc_html_e( 'Shortcodes', 'consent-magic' ); ?></h3>

    <div class="shortcodes-wrap">
		<?php $shortcodes = array(
			array(
				'code'        => '[cm_scan]',
				'description' => __( 'It will show all cookies and scripts detected by the last scan.', 'consent-magic' ),
			),
			array(
				'code'        => '[cm_scan_cookies]',
				'description' => __( 'It will show all cookies detected by the last scan.', 'consent-magic' ),
			),
			array(
				'code'        => '[cm_scan_scripts]',
				'description' => __( 'It will show all scripts detected by the last scan.', 'consent-magic' ),
			),
			array(
				'code'        => '[cm_scan_description]',
				'description' => __( 'It will show all cookies and scripts detected by the last scan and their descriptions.', 'consent-magic' ),
			),
			array(
				'code'        => '[cm_scan_cookies_description]',
				'description' => __( 'It will show all cookies detected by the last scan and their descriptions.', 'consent-magic' ),
			),
			array(
				'code'        => '[cm_scan_scrips_description]',
				'description' => __( 'It will show all scripts detected by the last scan and their descriptions.', 'consent-magic' ),
			),
		);

		renderShortcodeBlock( $shortcodes );

		?>
    </div>
</div>
