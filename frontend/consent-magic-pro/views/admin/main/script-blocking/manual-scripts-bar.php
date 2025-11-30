<?php

namespace ConsentMagicPro;

// If this file is called directly, abort.
if ( !defined( 'WPINC' ) ) {
	die;
}

?>

<div class="cards-wrapper cards-wrapper-style2 gap-22">
    <div class="manual-scripts-head">
        <div>
            <div class="d-flex align-items-center mb-4">
				<?php render_switcher_input( 'cs_block_manually_added_cookies' ); ?>
                <h4 class="secondary-heading switcher-label"><?php esc_html_e( 'Block manually added scripts', 'consent-magic' ); ?></h4>
            </div>

            <p class="text-gray"><?php esc_html_e( 'Learn how to add custom scripts:', 'consent-magic' ); ?> <a
                        href="https://www.youtube.com/watch?v=V9Q0bcIvlDw"
                        target="_blank" class="link"><?php esc_html_e( 'watch video', 'consent-magic' ); ?></a></p>
        </div>

        <div>
            <a href="<?php echo esc_url( get_admin_url( null, 'admin.php?page=consent-magic' ) . '&new_script' ); ?>"
               class="btn btn-primary btn-primary-type2 with-icon"><i
                        class="icon-plus"></i> <?php esc_html_e( 'Add script', 'consent-magic' ); ?></a>
        </div>
    </div>

    <div class="line"></div>
    <h3 class="primary-heading"><?php esc_html_e( 'Manually added scripts', 'consent-magic' ); ?></h3>

    <div class="cm-table-wrap">
        <table id="manual_added_scripts" class="display cm-table">
            <thead>
            <tr>
                <th colspan="2"><?php esc_html_e( 'Script name', 'consent-magic' ); ?></th>
                <th><?php esc_html_e( 'Script category', 'consent-magic' ); ?></th>
                <th></th>
            </tr>

            <tr class="tr-hidden">
                <th></th>
                <th></th>
                <th></th>
                <th></th>
            </tr>
            </thead>
            <tfoot>
            <tr>
                <th colspan="2"><?php esc_html_e( 'Script name', 'consent-magic' ); ?></th>
                <th><?php esc_html_e( 'Script category', 'consent-magic' ); ?></th>
                <th></th>
            </tr>

            <tr class="tr-hidden">
                <th></th>
                <th></th>
                <th></th>
                <th></th>
            </tr>
            </tfoot>
        </table>
    </div>
</div>