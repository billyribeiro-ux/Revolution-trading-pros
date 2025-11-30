<?php

namespace ConsentMagicPro;

// If this file is called directly, abort.
if ( !defined( 'WPINC' ) ) {
	die;
}

$cs_language_availability = ConsentMagic()->getOption( 'cs_language_availability' );
$language_list            = get_language_list();
$lang_options             = array();
foreach ( $cs_language_availability as $key => $lang ) {
	if ( isset( $language_list[ $key ] ) ) {

		$lang_options[ $key ] = array(
			'enable' => $lang,
			'name'   => $language_list[ $key ][ 'name' ],
			'label'  => $language_list[ $key ][ 'label' ],
		);
	}
}
$current_lang             = get_locale();
$cs_user_default_language = ConsentMagic()->getOption( 'cs_user_default_language' );
$cs_language_availability = ConsentMagic()->getOption( 'cs_language_availability' );
if ( isset( $cs_language_availability[ $current_lang ] ) && $cs_language_availability[ $current_lang ] == 0 ) {
	$current_lang = $cs_user_default_language;
}

?>

<div class="cards-wrapper cards-wrapper-style2 general-settings gap-24">
    <input type="hidden" name="cm-restore-default-translations" id="cm-restore-default-translations" value="">
    <input type="hidden" name="cm-renew-consent" id="cm-renew-consent" value="">
    <input type="hidden" name="cm-restart-setup" id="cm-restart-setup" value="">
    <div class="card card-static card-style9">
        <div class="card-body">
            <div class="gap-22">
                <div>
                    <div class="d-flex align-items-center mb-4 m-mb-8">
						<?php render_switcher_input( 'cs_test_mode' ); ?>
                        <h4 class="secondary-heading switcher-label"><?php esc_html_e( 'Enable test mode', 'consent-magic' ); ?></h4>
                    </div>

                    <p class="text-gray"><?php esc_html_e( 'Only Admin will see the consent dialog. You will have a button in the top bar to "Delete the test consent".', 'consent-magic' ); ?></p>
                </div>

                <div class="line"></div>

                <div class="d-flex align-items-center">
					<?php render_switcher_input( 'cs_deactivation_db_clear' ); ?>
                    <h4 class="secondary-heading switcher-label"><?php esc_html_e( 'Remove all data when deactivating the plugin', 'consent-magic' ); ?></h4>
                </div>

                <div class="line"></div>

				<?php if ( isWooCommerceActive() ) : ?>
                    <div class="d-flex align-items-center">
						<?php render_switcher_input( 'cs_store_consent_for_wc_orders' ); ?>
                        <h4 class="secondary-heading switcher-label"><?php esc_html_e( 'Store Consent for WooCommerce orders', 'consent-magic' ); ?></h4>
                    </div>

                    <div class="line"></div>

                    <div class="d-flex align-items-center">
						<?php render_switcher_input( 'cs_admin_email_consent_for_wc_orders' ); ?>
                        <h4 class="secondary-heading switcher-label"><?php esc_html_e( 'Send consent data with the WooCommerce "New Order" email', 'consent-magic' ); ?></h4>
                    </div>

                    <div class="line"></div>

                    <div class="privacy-options-text-wrap <?php echo ConsentMagic()->getOption( 'cs_customer_email_consent_for_wc_orders' ) == 1 ? '' : 'disabled'; ?>">
                        <div class="d-flex align-items-center mb-4 m-mb-8">
							<?php render_switcher_input( 'cs_customer_email_consent_for_wc_orders' ); ?>
                            <h4 class="secondary-heading switcher-label"><?php esc_html_e( 'Send consent data with the WooCommerce "Processing Order" email', 'consent-magic' ); ?></h4>
                        </div>

                        <div class="card card-style8 privacy-options-text">
                            <div class="card-header card-header-style3">
                                <div class="gap-8">
                                    <p class="text-gray"><?php esc_html_e( 'Your privacy options text', 'consent-magic' ); ?></p>
                                </div>

								<?php cardCollapseSettingsWithText(); ?>
                            </div>

                            <div class="card-body">
								<?php
								$inputs = array(
									array(
										'type'        => 'input',
										'meta'        => false,
										'key'         => 'cs_customer_email_consent_type',
										'input_title' => __( 'Consent type', 'consent-magic' ),
									),
									array(
										'type'        => 'input',
										'meta'        => false,
										'key'         => 'cs_customer_email_rule_name',
										'input_title' => __( 'Rule\'s name', 'consent-magic' ),
									),
									array(
										'type'        => 'input',
										'meta'        => false,
										'key'         => 'cs_customer_email_consent_expressed',
										'input_title' => __( 'Consent expressed', 'consent-magic' ),
									)
								);
								?>
                                <div class="language_list_block_wrap">
									<?php
									foreach ( $inputs as $input ) {
										render_input_text( null, $input[ 'key' ], $input[ 'meta' ], '', $input[ 'input_title' ] );
									}
									?>
                                </div>
                            </div>
                        </div>
                    </div>

                    <div class="line"></div>
				<?php endif; ?>

				<?php if ( isEddActive() ) : ?>
                    <div class="d-flex align-items-center">
						<?php render_switcher_input( 'cs_store_consent_for_edd_orders' ); ?>
                        <h4 class="secondary-heading switcher-label"><?php esc_html_e( 'Store Consent for Easy Digital Downloads orders', 'consent-magic' ); ?></h4>
                    </div>

                    <div class="line"></div>
				<?php endif; ?>

                <div class="pys-first-party-tracking-wrap gap-24 <?php echo !isPYSActivated() ? 'disabled' : ''; ?>">
                    <div class="pys-first-party-tracking-switchers">
                        <div class="mb-24">
                            <div class="d-flex align-items-center mb-4 m-mb-8">
								<?php render_switcher_input( 'cs_consent_for_pys' ); ?>
                                <h4 class="secondary-heading switcher-label"><?php esc_html_e( 'Pixel YourSite First-Party Tracking', 'consent-magic' ); ?></h4>
                            </div>
                            <p class="text-gray">
								<?php esc_html_e( 'PixelYourSite can track the traffic source, landing page, and UTMs and use the data for events parameters, WooCommerce, or Easy Digital Downloads first-party reports. You can configure the type of consent for this functionality.', 'consent-magic' ); ?>
                            </p>
                        </div>

                        <div class="consent_expressed_for_pys <?php echo ConsentMagic()->getOption( 'cs_consent_for_pys' ) ? '' : 'disabled'; ?>">
                            <div class="d-flex align-items-center mb-4 m-mb-8">
								<?php render_switcher_input( 'cs_track_before_consent_expressed_for_pys' ); ?>
                                <h4 class="secondary-heading switcher-label"><?php esc_html_e( 'Always track before consent is expressed', 'consent-magic' ); ?></h4>
                            </div>
                            <p class="text-gray">
								<?php esc_html_e( 'PixelYourSite will track and use this data for "Ask before tracking" before any consent is expressed by the website visitor.', 'consent-magic' ); ?>
                            </p>
                        </div>
                    </div>

					<?php if ( !isPYSActivated() ) :
						render_warning_message( sprintf( __( 'You need install and enable Pixel Your Site Plugin for these options. For more information visit site %s', 'consent-magic' ), '<a href="http://www.pixelyoursite.com/" target="_blank" rel="nofollow" class="link link-small">Pixelyoursite.com</a>' ) );
					endif; ?>

					<?php if ( !isPYSActivated() && isFbWooActivated() ) : ?>
                        <div>
                            <div class="d-flex align-items-center">
								<?php render_switcher_input( 'cs_fb_woo_capi_enabled' ); ?>
                                <h4 class="secondary-heading switcher-label"><?php esc_html_e( 'Facebook for WooCommerce - Control CAPI events', 'consent-magic' ); ?></h4>
                            </div>
							<?php if ( (int) ConsentMagic()->getOption( 'cs_enable_site_cache' ) === 0 ): ?>
                                <p class="text-gray mt-4 m-mt-8">
									<?php esc_html_e( 'If you enable this option, rules using "Ask before tracking" consent type will refresh the page after consent is expressed by the visitor.', 'consent-magic' ); ?>
                                </p>
							<?php endif; ?>
                        </div>
					<?php endif; ?>

                    <div class="line"></div>
                </div>

                <div class="d-flex align-items-center">
					<?php render_switcher_input( 'cs_block_video_personal_data' ); ?>
                    <h4 class="secondary-heading switcher-label"><?php esc_html_e( 'Show the video but block personal data tracking when required by the rule and user consent', 'consent-magic' ); ?></h4>
                </div>

                <div class="line"></div>

                <div class="cm-cache-option">
                    <div class="d-flex align-items-center mb-4 m-mb-8">
						<?php render_switcher_input( 'cs_enable_site_cache' ); ?>
                        <h4 class="secondary-heading switcher-label"><?php esc_html_e( 'Site uses cache', 'consent-magic' ); ?></h4>
                    </div>

                    <p class="text-gray mb-16">
						<?php esc_html_e( 'Enable this option if the site uses caching plugins (in particular, page caching), as well as caching on the hosting side.', 'consent-magic' ); ?>
                    </p>

					<?php render_warning_info_message( __( 'Usage note: after updating the plugin settings, you need to clear the cache.', 'consent-magic' ) ); ?>

                    <p class="text-gray mt-16">
						<?php esc_html_e( 'Additionally, you can add these exceptions to "Exclude JavaScript Files" in minification and delay options:', 'consent-magic' ); ?>
                    </p>

					<?php $code = '/wp-content/plugins/consent-magic-pro/(.*).js<br>consent-magic-js-extra';
					$code_js    = '/wp-content/plugins/consent-magic-pro/(.*).js\nconsent-magic-js-extra'; ?>

                    <div class="shortcode-wrap settings-shortcode">
                        <div class="shortcode">
                            <span class="shortcode-title">
                                <?php echo wp_kses_post( $code ) ?>
                            </span>
                            <div class="shortcode-copy"
                                 onclick="copyShortcode( '<?php echo wp_kses_post( $code_js ); ?>', this)">
								<?php renderTooltip( 'icon-content-copy' ); ?>
                            </div>
                        </div>
                    </div>

                    <p class="text-gray">
						<?php esc_html_e( 'Note: These exceptions may vary slightly depending on the caching plugin. In most cases it won\'t be necessary to add them.', 'consent-magic' ); ?>
                    </p>
                </div>

                <div class="line"></div>

                <div>
                    <div class="d-flex align-items-center mb-4 m-mb-8">
						<?php render_switcher_input( 'cs_cross_domain_tracking' ); ?>
                        <h4 class="secondary-heading switcher-label"><?php esc_html_e( 'Subdomain tracking', 'consent-magic' ); ?></h4>
                    </div>

                    <p class="text-gray mb-4"><?php esc_html_e( 'To enable the cross-domain option to work correctly, you must specify the top-level domain in the CM settings for all your sites.', 'consent-magic' ); ?></p>
                    <p class="text-gray mb-8"><?php echo wp_kses_post( __( 'For example, if your main site is <span class="text-gray-bg">yoursite.com</span> and a subsite is <span class="text-gray-bg">shop.yoursite.com</span>, you need to specify <span class="text-gray-bg">yoursite.com</span> in the settings of both sites.', 'consent-magic' ) ); ?></p>

                    <div>
                        <input type="text"
                               name="<?php echo esc_attr( "cs[" . ConsentMagic()->plugin_name . "][cs_cross_domain_tracking_domain]" ); ?>"
                               id="cs_cross_domain_tracking_domain"
                               value="<?php echo esc_attr( ConsentMagic()->getOption( 'cs_cross_domain_tracking_domain' ) ); ?>"
                               class="input-full"/>
                    </div>
                </div>

                <div class="line"></div>

                <div>
                    <div class="wp-consent-api-settings-wrap <?php echo !CS_WP_Consent_Api()->isEnableWPConsent() ? 'disabled' : ''; ?>">
                        <div class="d-flex align-items-center mb-4 m-mb-8">
							<?php render_switcher_input( 'cs_wp_consent_api_enabled' ); ?>
                            <h4 class="secondary-heading switcher-label"><?php esc_html_e( 'Enable support for WordPress Consent API', 'consent-magic' ); ?></h4>
                        </div>

                        <p class="text-gray"><?php esc_html_e( 'This option will turn ON/OFF the integration - sending values to the API', 'consent-magic' ); ?></p>
                    </div>

					<?php if ( !CS_WP_Consent_Api()->isEnableWPConsent() ) : ?>
                        <div class="mt-8">
							<?php render_warning_message( sprintf( __( 'You need install and enable WP Consent API Plugin for this option. For more information click %s', 'consent-magic' ), sprintf( '<a href="https://wordpress.org/plugins/wp-consent-api/" target="_blank" rel="nofollow" class="link link-small">%s</a>', __( 'here', 'consent-magic' ) ) ) ); ?>
                        </div>
					<?php endif; ?>
                </div>
            </div>
        </div>
    </div>

    <div class="card card-style9">
        <div class="card-header card-header-style3">
            <h3 class="primary-heading-type2">
				<?php esc_html_e( 'Translations', 'consent-magic' ); ?>
            </h3>

			<?php cardCollapseSettingsWithText( 'Show' ); ?>
        </div>

        <div class="card-body">
            <div class="gap-24">
                <p class="text-gray">
					<?php echo esc_html__( 'The content will be displayed in the following language according to the WordPress language settings. If a multilingual plugin changes the language based on the user\'s location, the rule\'s content will also use that language when possible. You can edit or add translations on ', 'consent-magic' ) . '<a href="' . esc_url( get_admin_url() . 'admin.php?page=consent-magic&tab=cs-text' ) . '" class="link">' . 'Dashboard > Text' . '</a>'; ?>
                </p>

				<?php render_info_message( __( ' Custom text and translations can be added to all rules except the IAB Rule.', 'consent-magic' ) ); ?>

                <div class="d-flex align-items-center mb-4">
					<?php render_switcher_input( 'cs_enable_translations' ); ?>
                    <h4 class="secondary-heading switcher-label"><?php esc_html_e( 'Enable Translations', 'consent-magic' ); ?></h4>
                </div>

                <div>
                    <h4 class="font-semibold-type2 mb-4"><?php esc_html_e( 'Default language', 'consent-magic' ); ?></h4>
					<?php renderSelectLanguages( 'cs_user_default_language', false, $cs_user_default_language, $lang_options, null ); ?>
                </div>

                <div class="line"></div>

                <h3 class="primary-heading">
					<?php esc_html_e( 'Select available languages to display', 'consent-magic' ); ?>:
                </h3>

                <div class="cs_languages">
					<?php
					if ( !empty( $lang_options ) ) :
						foreach ( $lang_options as $key => $lang ) :
							?>
                            <div class="cs_language_item <?php echo $key !== CMPRO_DEFAULT_LANGUAGE ? 'cs_available_lang_container' : ''; ?>"
                                 style="<?php echo ( $key == $cs_user_default_language || $key === CMPRO_DEFAULT_LANGUAGE ) ? 'opacity: 0.5;pointer-events:none;' : '' ?>">
								<?php
								$names = array(
									'cs_language_availability',
									$key
								);

								render_static_switcher_input( $names, $lang[ 'enable' ] ); ?>

                                <h4 class="switcher-label secondary-heading"><?php echo $lang[ 'label' ]; ?>
									<?php if ( $key === CMPRO_DEFAULT_LANGUAGE ) : ?>
                                        <span>
                                            <?php echo sprintf( '(%s)', esc_html__( 'Always ON', 'consent-magic' ) ); ?>
                                        </span>
									<?php else : ?>
                                        <span class="cs_language_message">
                                                    <?php echo $key == $cs_user_default_language ? sprintf( '(%s)', esc_html__( 'Default language always ON', 'consent-magic' ) ) : ''; ?>
                                                </span>
									<?php endif; ?>
                                </h4>
                            </div>
						<?php
						endforeach;
					endif;
					?>
                </div>
                <div id="cs_language_message" style="display: none">
                    (<?php esc_html_e( 'Default language always ON', 'consent-magic' ); ?>)
                </div>

                <div class="line"></div>

                <div>
                    <button type="button" class="btn btn-primary btn-primary-type2 restore_translations">
						<?php esc_html_e( 'Restore the default text', 'consent-magic' ); ?>
                    </button>
                </div>
            </div>
        </div>
    </div>

    <div class="card card-static card-style9">
        <div class="card-header card-header-style3">
            <h3 class="primary-heading-type2">
				<?php esc_html_e( 'Cookies duration', 'consent-magic' ); ?>
            </h3>
        </div>

        <div class="card-body">
            <div class="cookie-duration">
                <p class="font-semibold mr-12"><?php esc_html_e( 'Ask for consent again after ', 'consent-magic' ); ?></p>
                <div class="mr-8">
					<?php render_number_input( 'cs_expire_days', false ); ?>
                </div>
                <p class="font-semibold"><?php esc_html_e( 'days.', 'consent-magic' ); ?></p>
                <p><?php esc_html_e( 'Use 0 for indefinite.', 'consent-magic' ); ?></p>
            </div>
        </div>
    </div>

    <div class="card card-static card-style9">
        <div class="card-header card-header-style3">
            <h3 class="primary-heading-type2">
				<?php esc_html_e( 'Scanner module', 'consent-magic' ); ?>
            </h3>
        </div>

        <div class="card-body">
            <div class="d-flex align-items-center">
				<?php
				$admin_modules = ConsentMagic()->getOption( 'csmart_admin_modules' );
				if ( is_array( $admin_modules ) && array_key_exists( 'scanner', $admin_modules ) ) {
					$scanner_module = $admin_modules[ 'scanner' ];
				} else {
					$scanner_module = true;
				}

				$names = array(
					'cs_scanner_module',
				);

				render_static_switcher_input( $names, $scanner_module ); ?>
                <h4 class="secondary-heading switcher-label"><?php esc_html_e( 'Enable scanner module', 'consent-magic' ); ?></h4>
            </div>
        </div>
    </div>

    <div class="card card-static card-style9">
        <div class="card-header card-header-style3">
            <h3 class="primary-heading-type2">
				<?php esc_html_e( 'Close on Scroll', 'consent-magic' ); ?>
            </h3>
        </div>

        <div class="card-body">
            <div class="close-scroll">
                <p class="font-semibold mr-8"><?php esc_html_e( 'Close on Scroll duration ', 'consent-magic' ); ?></p>
                <div class="d-flex align-items-center mr-8">
					<?php render_number_input( 'cs_default_close_on_scroll', false ); ?>
                    <span class="font-semibold ml-8">%</span>
                </div>
            </div>
        </div>
    </div>

    <div class="card card-static card-style9">
        <div class="card-header card-header-style3">
            <h3 class="primary-heading-type2">
				<?php esc_html_e( 'Branding', 'consent-magic' ); ?>
            </h3>
        </div>

        <div class="card-body">
            <div class="gap-24">
                <div class="d-flex align-items-center">
					<?php render_switcher_input( 'cs_bar_verified_link' ); ?>
                    <h4 class="secondary-heading switcher-label"><?php esc_html_e( 'Add a "Verified by ConsentMagic" link on the consent message', 'consent-magic' ); ?></h4>
                </div>

                <div class="d-flex align-items-center">
					<?php render_switcher_input( 'cs_options_verified_link' ); ?>
                    <h4 class="secondary-heading switcher-label"><?php esc_html_e( 'Add a "Verified by ConsentMagic" link on the options popup', 'consent-magic' ); ?></h4>
                </div>

                <div class="d-flex align-items-center">
					<?php render_switcher_input( 'cs_options_single_verified_link' ); ?>
                    <h4 class="secondary-heading switcher-label"><?php esc_html_e( 'Add a "Verified by ConsentMagic" link on single-step template', 'consent-magic' ); ?></h4>
                </div>
            </div>
        </div>
    </div>

    <div class="card card-static card-style9">
        <div class="card-body">
            <div class="gap-24">
                <div class="d-flex align-items-center">
					<?php render_switcher_input( 'cs_send_important' ); ?>
                    <h4 class="secondary-heading switcher-label"><?php esc_html_e( 'Send important emails', 'consent-magic' ); ?></h4>
                </div>

                <div>
                    <h4 class="font-semibold-type2 mb-4">
						<?php esc_html_e( 'Emails', 'consent-magic' ); ?>
                    </h4>
					<?php render_multi_select_input( 'cs_send_important_emails', ConsentMagic()->getOption( 'cs_send_important_emails' ), false, 'cs_multi_select_tags', null, __( 'Click to add email', 'consent-magic' ) ); ?>
                </div>
            </div>
        </div>
    </div>

    <div class="card card-static card-style9">
        <div class="card-header card-header-style3">
            <h3 class="primary-heading-type2">
				<?php esc_html_e( 'Permissions', 'consent-magic' ); ?>
            </h3>
        </div>

        <div class="card-body">
            <div class="mb-4">
                <h4 class="font-semibold-type2 mb-4">
					<?php esc_html_e( 'Select user\'s role', 'consent-magic' ); ?>
                </h4>
				<?php render_multi_select_input( 'cs_admin_permissions', getAvailableUserRoles(), false, 'cs_multi_select', '', __( 'Click to add role', 'consent-magic' ) ); ?>
            </div>
            <p class="text-gray text-small"><?php esc_html_e( 'Administrators are always allowed.', 'consent-magic' ); ?></p>
        </div>
    </div>

    <div class="card card-static card-style9">
        <div class="card-header card-header-style3">
            <h3 class="primary-heading-type2">
				<?php esc_html_e( 'Ignore these user roles', 'consent-magic' ); ?>
            </h3>
        </div>

        <div class="card-body">
            <div class="mb-4">
                <h4 class="font-semibold-type2 mb-4">
					<?php esc_html_e( 'Select user\'s role', 'consent-magic' ); ?>
                </h4>
				<?php render_multi_select_input( 'cs_admin_ignore_users', getAvailableUserRoles(), false, 'cs_multi_select', null, __( 'Click to add role', 'consent-magic' ), true ); ?>
            </div>
            <p class="text-gray text-small"><?php esc_html_e( 'Selected roles are ignored, and these users will not see any consent messages when browsing the website.', 'consent-magic' ); ?></p>
        </div>
    </div>

    <div class="card card-static card-style9">
        <div class="card-header card-header-style3">
            <h3 class="primary-heading-type2">
				<?php esc_html_e( 'Ignore these user IP', 'consent-magic' ); ?>
            </h3>
        </div>

        <div class="card-body">
            <div class="gap-24">
                <div>
                    <h4 class="font-semibold-type2 mb-4">
						<?php esc_html_e( 'User IP', 'consent-magic' ); ?>
                    </h4>

                    <div class="add-ignore-ip">
                        <input type="text" name="new_ignore_ip"
                               id="new_ignore_ip"
                               placeholder="<?php esc_html_e( 'Click to add IP', 'consent-magic' ) ?>"
                               class="new_ignore_ip input-full" value=""/>

                        <div class="add_new_ignore_ip_wrap">
                            <button type="button" id="add_new_ignore_ip" class="btn btn-primary btn-primary-type2">
								<?php esc_html_e( 'Add new IP', "consent-magic" ); ?>
                            </button>
							<?php include CMPRO_PLUGIN_VIEWS_PATH . "admin/components/spinner.php"; ?>
                        </div>
                    </div>

                    <div class="error-message cs-error-message mt-24"
                         style="display: none;">
						<?php render_critical_message( '' ) ?>
                    </div>
                </div>

                <h3 class="primary-heading"><?php esc_html_e( "User's IP list", "consent-magic" ); ?></h3>

                <div>
                    <h4 class="font-semibold-type2 mb-8"><?php esc_html_e( 'Date range filters', "consent-magic" ); ?></h4>

                    <div class="search-ignore-ip">
                        <div class="select-wrap select-daterange-wrap">
                            <div class="select-daterange datepicker_ip">
                                <span></span>
                            </div>
                        </div>

                        <button type="button" id="btn_search_ip" class="btn btn-primary btn-primary-type2">
							<?php esc_html_e( 'Search', "consent-magic" ); ?>
                        </button>
                    </div>
                </div>

                <div class="cm-table-wrap">
                    <table id="ignore_ip_table" class="display cm-table">
                        <thead>
                        <tr>
                            <th><?php esc_html_e( 'IP', "consent-magic" ); ?></th>
                            <th><?php esc_html_e( 'Time', "consent-magic" ); ?></th>
                            <th></th>
                        </tr>
                        </thead>
                        <tfoot>
                        <tr>
                            <th><?php esc_html_e( 'IP', "consent-magic" ); ?></th>
                            <th><?php esc_html_e( 'Time', "consent-magic" ); ?></th>
                            <th></th>
                        </tr>
                        </tfoot>
                    </table>
                </div>

				<?php render_warning_info_message( __( 'IP in this list are ignored, and these users will not see any consent messages when browsing the website.', 'consent-magic' ) ); ?>

                <div class="d-flex align-items-center">
					<?php render_switcher_input( 'cs_disable_for_knonw_crawlers' ); ?>
                    <h4 class="secondary-heading switcher-label"><?php esc_html_e( 'Disable the plugin for known crawlers', 'consent-magic' ); ?></h4>
                </div>
            </div>
        </div>
    </div>

    <div class="card card-static card-style9">
        <div class="card-body no-border">
            <div class="gap-24">
                <div class="settings-block">
                    <h4 class="primary-heading-type2"><?php esc_html_e( 'Renew Consent', 'consent-magic' ); ?></h4>

                    <button type="button" class="btn btn-primary btn-primary-type2 cs_renew_consent_confirm">
						<?php esc_html_e( 'Renew Consent', 'consent-magic' ); ?>
                    </button>
                </div>

				<?php render_warning_info_message( __( 'This will "invalidate" all expressed consent, asking every visitor for consent again.', 'consent-magic' ) ); ?>
            </div>
        </div>
    </div>

    <div class="card card-static card-style9">
        <div class="card-body no-border">
            <div class="gap-24">
                <div class="settings-block">
                    <h4 class="primary-heading-type2"><?php esc_html_e( 'Restart Setup Process', 'consent-magic' ); ?></h4>

                    <button type="button" class="btn btn-primary btn-primary-type2 cs_restart_setup_confirm">
						<?php esc_html_e( 'Restart Setup Process', 'consent-magic' ); ?>
                    </button>
                </div>

				<?php render_warning_info_message( __( 'This option will restart the setup flow, and reset all settings to default.', 'consent-magic' ) ); ?>
            </div>
        </div>
    </div>

    <div class="card card-style9">
        <div class="card-header card-header-style3">
            <h3 class="primary-heading-type2">
				<?php esc_html_e( 'Export/Import settings', 'consent-magic' ); ?>
            </h3>

			<?php cardCollapseSettingsWithText( 'Show' ); ?>
        </div>

        <div class="card-body">
            <div class="gap-24">
                <div>
                    <p class="font-semibold-type2"><?php esc_html_e( 'Select the settings to export:', 'consent-magic' ); ?></p>

                    <div id="cs_export_settings" class="mt-18">
						<?php
						render_checkbox_input( 'cs_export_all_settings', true, false, null, false, false, 'All settings' );
						render_checkbox_input( 'cs_export_general_settings', true, false, null, false, false, 'General settings' );
						//(including all settings, blocking settings, consent proof storing, scanner settings)
						render_checkbox_input( 'cs_export_iab_settings', true, false, null, false, false, 'IAB settings' );
						render_checkbox_input( 'cs_export_geolocation_settings', true, false, null, false, false, 'Geolocation settings' );
						render_checkbox_input( 'cs_export_default_rules', true, false, null, false, false, 'Default rules' );
						render_checkbox_input( 'cs_export_custom_rules', true, false, null, false, false, 'Custom rules' );
						render_checkbox_input( 'cs_export_predefined_scripts', true, false, null, false, false, 'Pre-defined scripts' );
						render_checkbox_input( 'cs_export_custom_scripts', true, false, null, false, false, 'Custom scripts' );
						render_checkbox_input( 'cs_export_design_settings', true, false, null, false, false, 'Design templates and settings' );
						render_checkbox_input( 'cs_export_script_types', true, false, null, false, false, 'Cookie/Script types with translations' );
						render_checkbox_input( 'cs_export_text', true, false, null, false, false, 'Text with translations' );
						?>
                    </div>
                </div>

                <div class="line"></div>

                <div>
                    <div class="cs_export_settings_buttons">
						<?php include_once CMPRO_PLUGIN_VIEWS_PATH . "admin/buttons/admin-settings-export-settings-button.php"; ?>
						<?php include_once CMPRO_PLUGIN_VIEWS_PATH . "admin/buttons/admin-settings-import-settings-button.php"; ?>
                    </div>

                    <div class="cs_settings_message">
						<?php render_info_message( '' ); ?>
                    </div>
                </div>
            </div>
        </div>
    </div>
</div>

<?php include CMPRO_PLUGIN_VIEWS_PATH . "admin/components/modal-restore-translations.php"; ?>
<?php include CMPRO_PLUGIN_VIEWS_PATH . "admin/components/modal-renew-consent.php"; ?>
<?php include CMPRO_PLUGIN_VIEWS_PATH . "admin/components/modal-restart-setup.php"; ?>
<?php include CMPRO_PLUGIN_VIEWS_PATH . "admin/components/modal-deactivation-db-clear.php"; ?>
