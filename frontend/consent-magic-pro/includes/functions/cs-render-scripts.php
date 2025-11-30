<?php

namespace ConsentMagicPro;

// If this file is called directly, abort.
if ( !defined( 'WPINC' ) ) {
	die;
}

function render_predefined_scripts_block( $scripts, $categories ) {
	if ( !empty( $scripts ) ) : ?>
        <div class="pre-defined-scripts-wrap">
			<?php foreach ( $scripts as $script ) : ?>
                <div class="card card-style6 disable-control">
                    <div class="card-header card-header-style5 pre-defined-scripts-header">
                        <div class="switcher disable-control-switcher">
							<?php render_switcher_input( $script[ 'key' ] ); ?>
                            <h4 class="switcher-label"><?php echo esc_html( $script[ 'name' ] ) ?></h4>
                        </div>

                        <div class="categories">
							<?php renderSelectInput( "{$script['key']}_cat", $categories, true, false, 'small' ); ?>
                        </div>

						<?php cardCollapseSettingsWithText( 'Details' ); ?>
                    </div>

                    <div class="card-body">
                        <h4 class="font-semibold mb-4">
							<?php esc_html_e( 'Description', 'consent-magic' ); ?>:
                        </h4>

						<?php renderTextarea( "{$script['key']}_descr", '', '', true ); ?>
                    </div>
                </div>
			<?php endforeach; ?>
        </div>
	<?php endif;
}