<?php

namespace ConsentMagicPro;

// If this file is called directly, abort.
if ( !defined( 'WPINC' ) ) {
	die;
}
?>
<div data-id="<?php echo esc_attr( $target_id ); ?>">
    <div class="cards-wrapper cards-wrapper-style2 gap-22 dashboard">
        <div class="card card-static card-style3">
            <div class="card-header card-header-style2">
                <div class="header-full-width">
                    <h3 class="primary-heading"><?php esc_html_e( 'Rules', 'consent-magic' ); ?></h3>

                    <a href="<?php echo esc_url( get_admin_url( null, 'admin.php?page=consent-magic' ) . '&new_rule' ); ?>"
                       class="btn btn-primary btn-primary-type2 with-icon"><i
                                class="icon-plus"></i> <?php esc_html_e( 'Add New Rule', 'consent-magic' ); ?></a>
                </div>
            </div>

            <div class="card-body cm-sortable-body">
                <div class="gap-24">
					<?php render_notice_message( __( 'A visitor will be targeted by a single rule, based on location.', 'consent-magic' ) ); ?>

                    <div>
                        <p class="text-gray text-extra-small fw-500 mb-4"><?php esc_html_e( 'Drag & Drop to configure rule\'s priority', 'consent-magic' ) ?>
                            :</p>

                        <div class="cm-sortable">
							<?php get_rules_by_meta_key( '_cs_predefined_rule' ); ?>
                        </div>
                    </div>
                </div>
            </div>
        </div>

        <div class="card card-static card-style3">
            <div class="card-header card-header-style2">
                <div class="d-flex align-items-center">
                    <h3 class="primary-heading"><?php esc_html_e( 'Main settings', 'consent-magic' ); ?></h3>
                </div>
            </div>

            <div class="card-body">
                <div class="gap-24">
                    <div>
                        <div class="d-flex align-items-center mb-4 m-mb-8">
						    <?php render_switcher_input( 'cs_plugin_activation' ); ?>
                            <h4 class="secondary-heading switcher-label"><?php esc_html_e( 'Enable consent', 'consent-magic' ); ?></h4>
                        </div>

                        <p class="text-gray"><?php esc_html_e( 'This option enables or disables the plugin entirely. When turned off, the plugin will not function, and no messages or actions will be applied.', 'consent-magic' ); ?></p>
                    </div>

                    <div class="line"></div>

                    <div class="gap-24 general-sub-settings">
                        <div>
                            <div class="d-flex align-items-center mb-4 m-mb-8">
							    <?php render_switcher_input( 'cs_script_blocking_enabled' ); ?>
                                <h4 class="secondary-heading switcher-label"><?php esc_html_e( 'Enable Script Blocking', 'consent-magic' ); ?></h4>
                            </div>

                            <p class="text-gray"><?php esc_html_e( 'This sub-option controls script blocking according to the user’s preferences. It only works when ‘Enable Consent’ is turned on. If disabled, the plugin will display messages but will not block any scripts.', 'consent-magic' ); ?></p>
                        </div>

                        <div class="d-flex">
                            <a class="btn btn-primary btn-primary-type2"
                               href="<?php echo esc_url( get_admin_url( null, 'admin.php?page=consent-magic&tab=cs-script-blocking' ) ); ?>"><?php esc_html_e( 'View Settings', 'consent-magic' ); ?></a>
                        </div>
                    </div>
                </div>
            </div>
        </div>
    </div>
</div>
