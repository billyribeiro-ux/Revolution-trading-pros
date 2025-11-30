<?php

namespace ConsentMagicPro;

// If this file is called directly, abort.
if ( !defined( 'WPINC' ) ) {
	die;
}

$page_options = get_existing_pages();
ConsentMagic()->check_policy_page();
$fonts = array(
	array(
		'option_value' => 'arial',
		'option_name'  => esc_html__( 'Arial', 'consent-magic' )
	),
	array(
		'option_value' => 'verdana',
		'option_name'  => esc_html__( 'Verdana', 'consent-magic' )
	),
	array(
		'option_value' => 'times',
		'option_name'  => esc_html__( 'Times New Roman', 'consent-magic' )
	)
);

$rules = array(
	'cs_gdpr_rule'          => array(
		'id'      => get_post_id_by_slug( 'cs_gdpr_rule' ),
		'label'   => esc_html__( 'GDPR Rule', 'consent-magic' ),
		'default' => true,
	),
	'cs_ldu_rule'           => array(
		'id'      => get_post_id_by_slug( 'cs_ldu_rule' ),
		'label'   => esc_html__( 'Limited Data Use (LDU) Rule', 'consent-magic' ),
		'default' => true,
	),
	'cs_rest_of_world_rule' => array(
		'id'      => get_post_id_by_slug( 'cs_rest_of_world_rule' ),
		'label'   => esc_html__( 'Rest of the world rule', 'consent-magic' ),
		'default' => true,
	),
	'cs_iab_rule'           => array(
		'id'      => get_post_id_by_slug( 'cs_iab_rule' ),
		'label'   => esc_html__( 'IAB rule', 'consent-magic' ),
		'default' => false,
	),
);

?>
<form method="post" action="<?php echo esc_url( get_admin_url( null, 'admin.php?page=consent-magic' ) ); ?>"
      id="cs_step_form" class="cs-modal-content" style="display: none;">

	<?php
	// Set nonce:
	if ( function_exists( 'wp_create_nonce' ) ) {
		$nonce_value = wp_create_nonce( 'cs-update-' . CMPRO_SETTINGS_FIELD );
		echo '<input type="hidden" id="cm_nonce" name="_wpnonce" value="' . esc_attr( $nonce_value ) . '">';
	}
	$current_step = 1;
	?>

    <input type="hidden" name="cs_update_action" value="" id="cs_update_action"/>
    <div class="step-wrap" id="step_second">
        <div class="step">
            <div class="step-title">
                <h3 class="heading primary-heading"><?php esc_html_e(
						'Configure the default rules',
						'consent-magic'
					); ?></h3>
				<?php render_stepper( $current_step ); ?>
            </div>

            <div>
                <div class="mb-24">
                    <p class="text-gray fw-500"><?php esc_html_e(
							'We recommend keeping the default rules active. You can edit them, turned them ON or OFF, or create new rules anytime you want.',
							'consent-magic'
						); ?></p>
                </div>

                <div class="gap-16">
					<?php
					foreach ( $rules as $key => $rule ) : ?>
                        <div class="switcher-wrap">
							<?php render_switcher_input(
								$key,
								false,
								false,
								null,
								false,
								false,
								false,
								$rule[ 'default' ]
							); ?>
                            <h4 class="switcher-label secondary-heading"><?php echo esc_html( $rule[ 'label' ] ) ?></h4>
                        </div>
					<?php endforeach;
					?>
                </div>
            </div>

            <div class="line"></div>

            <div class="step-buttons step-button-2">
				<?php
				//next step button
				include CMPRO_PLUGIN_VIEWS_PATH . "admin/buttons/admin-next-step-button.php";
				?>
            </div>
        </div>
    </div>

	<?php $current_step++; ?>

    <div class="step-wrap" id="step_third">
        <div class="step">
            <div class="step-title">
                <h3 class="heading primary-heading"><?php esc_html_e( 'Select the top rule:', 'consent-magic' ); ?></h3>
				<?php render_stepper( $current_step ); ?>
            </div>

            <div>
                <p class="text-gray fw-500 mb-24"><?php esc_html_e(
						'The top rule will be the only rule active until you complete the geolocation settings. You can re-order rules at any time.',
						'consent-magic'
					); ?></p>

                <div class="gap-16">
					<?php
					foreach ( $rules as $rule ) {
						render_radio_input( 'cs_active_rule_id_first', $rule[ 'id' ], $rule[ 'label' ] );
					}
					?>
                </div>
            </div>

            <div class="line"></div>

            <div class="step-buttons">
				<?php
				//previous step button
				include CMPRO_PLUGIN_VIEWS_PATH . "admin/buttons/admin-previous-step-button.php";
				//next step button
				include CMPRO_PLUGIN_VIEWS_PATH . "admin/buttons/admin-next-step-button.php";
				?>
            </div>
        </div>
    </div>

	<?php $current_step++; ?>

    <div class="step-wrap" id="step_fourth">
        <div class="step">
            <div class="step-title">
                <h3 class="heading primary-heading"><?php esc_html_e( 'Privacy policy page', 'consent-magic' ); ?></h3>
				<?php render_stepper( $current_step ); ?>
            </div>

            <div>
                <h4 class="font-semibold mb-4"><?php esc_html_e( 'Select an existing page', 'consent-magic' ); ?></h4>

                <div class="mb-24">
					<?php
					$page_id = renderPrivacyPolicyPage( get_locale() );

					$names = array(
						'cs_policy_existing_page',
						CMPRO_DEFAULT_LANGUAGE,
					);

					renderSelectInputStatic(
						$names,
						$page_id,
						$page_options,
						false,
						'js-control-element',
						'full'
					);
					?>
                </div>

                <p class="text-gray fw-500"><?php esc_html_e(
						'You can add additional pages later in the Privacy Policy settings.',
						'consent-magic'
					); ?></p>
            </div>

            <div class="line"></div>

            <div class="step-buttons">
				<?php
				//previous step button
				include CMPRO_PLUGIN_VIEWS_PATH . "admin/buttons/admin-previous-step-button.php";
				//next step button
				include CMPRO_PLUGIN_VIEWS_PATH . "admin/buttons/admin-next-step-button.php";
				?>
            </div>
        </div>
    </div>

	<?php $current_step++; ?>

    <div class="step-wrap" id="step_fifth">
        <div class="step">
            <div class="step-title">
                <h3 class="heading primary-heading"><?php esc_html_e( 'Script blocking', 'consent-magic' ); ?></h3>
				<?php render_stepper( $current_step ); ?>
            </div>

            <div>
                <p class="text-gray fw-500 mb-24"><?php esc_html_e(
						"We recommend keeping it active. When necessary, the plugin automatically blocks the most common scripts according to rules and user's preferences.",
						'consent-magic'
					); ?></p>

                <div class="switcher-wrap">
					<?php render_switcher_input( 'cs_script_blocking_enabled' ); ?>
                    <h4 class="switcher-label secondary-heading"><?php esc_html_e(
							'Enable Script Blocking',
							'consent-magic'
						); ?></h4>
                </div>
            </div>

            <div class="line"></div>

            <div class="step-buttons">
				<?php
				//previous step button
				include CMPRO_PLUGIN_VIEWS_PATH . "admin/buttons/admin-previous-step-button.php";
				//next step button
				include CMPRO_PLUGIN_VIEWS_PATH . "admin/buttons/admin-next-step-button.php";
				?>
            </div>
        </div>
    </div>

	<?php $current_step++; ?>

    <div class="step-wrap" id="step_sixth">
        <div class="step">

            <input type="hidden" name="cs[consent-magic][cs_check_flow]" value="1"
                   checked="checked" id="cs_consent-magic_cs_check_flow"
                   class="primary-switch-input"
                   data-target="cs_consent-magic_cs_check_flow_panel">

            <div class="step-title">
                <h3 class="heading primary-heading"><?php esc_html_e(
						'Configure Google Fonts',
						'consent-magic'
					); ?></h3>
				<?php render_stepper( $current_step ); ?>
            </div>

            <div class="gap-24">
                <p class="text-gray fw-500"><?php esc_html_e(
						'Inform your users about the use of Google Fonts and let them opt out of using them. When Google Fonts are rejected, they will be replaced by another font type. You can change these settings from the plugin\'s Scrips & Cookies > Categories.
                                            This option works for rules using "Inform and Opt-out" and "Ask before tracking" consent types. ',
						'consent-magic'
					); ?></p>

                <div>
                    <h4 class="font-semibold mb-4"><?php esc_html_e(
							'Select the fallback font: ',
							'consent-magic'
						); ?></h4>
					<?php renderSelectInput( 'default_font', $fonts, true ); ?>
                </div>

                <div class="switcher-wrap">
					<?php render_switcher_input( 'cs_block_googlefonts_enabled' ); ?>
                    <h4 class="switcher-label secondary-heading"><?php esc_html_e(
							'Enable Google Fonts blocking',
							'consent-magic'
						); ?></h4>
                </div>
            </div>

            <div class="line"></div>

            <div class="step-buttons">
				<?php
				//previous step button
				include CMPRO_PLUGIN_VIEWS_PATH . "admin/buttons/admin-previous-step-button.php";
				?>

                <div class="first-flow-save-button">
					<?php
					//spinner
					$absolute_position = false;
					include CMPRO_PLUGIN_VIEWS_PATH . "admin/components/spinner.php";

					//save button
					include CMPRO_PLUGIN_VIEWS_PATH . "admin/buttons/admin-first-flow-save-button.php";
					?>
                </div>
            </div>
        </div>
    </div>
</form>