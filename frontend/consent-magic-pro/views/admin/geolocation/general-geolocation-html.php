<?php

namespace ConsentMagicPro;

// If this file is called directly, abort.
if ( !defined( 'WPINC' ) ) {
	die;
}

$geo_enable           = ConsentMagic()->getOption( 'cs_geolocation' );
$class_settings       = ( (int) $geo_enable === 0 ) ? 'geo_disable' : '';
$cs_often_update_list = ConsentMagic()->getOption( 'cs_often_update_list' );
$activated            = (int) ConsentMagic()->getOption( 'cs_geo_activated' );

?>
<div class="cards-wrapper cards-wrapper-style2 cs_geolocation gap-8">
    <div class="card card-static card-style9">
        <div class="card-body">
            <div class="gap-24">
                <div>
                    <h4 class="primary-heading-type2 mb-8"><?php esc_html_e( 'Geolocation settings', 'consent-magic' ); ?></h4>
                    <p class="text-gray"> <?php echo sprintf( esc_html__( 'Learn how to configure geolocation: ', 'consent-magic' ) . '<a href="%s" target="_blank" class="link">' . esc_html__( 'watch video', 'consent-magic' ) . '</a>', 'https://www.youtube.com/watch?v=9S2GZUWPKg0' ) ?></p>
                </div>

                <div class="line"></div>

                <div>
                    <h4 class="font-semibold-type2 mb-4">
						<?php esc_html_e( 'MaxMind Account ID', 'consent-magic' ); ?>:
                    </h4>
					<?php renderTextInput( 'cs_geo_licence_account_id', null, false, false, false, 'MaxMind Account ID', 'js-control-element' ); ?>
                </div>

                <div>
                    <h4 class="font-semibold-type2 mb-4">
						<?php esc_html_e( 'License key', 'consent-magic' ); ?>:

						<?php if ( $activated === 1 ) : ?>
                            <span class="cs_geo_license_message cs_geo_license_message_activated">
                                    <?php esc_html_e( 'Activated', 'consent-magic' ); ?>
                                </span>
						<?php else: ?>
                            <span class="cs_geo_license_message">
                                    <?php esc_html_e( 'Deactivated', 'consent-magic' ); ?>
                                </span>
						<?php endif; ?>
                    </h4>

                    <div class="mb-4">
						<?php if ( $activated === 1 ) : ?>
                            <input id="cs_cs_geo_licence_key"
                                   name="cs[consent-magic][cs_geo_licence_key]"
                                   size="25" type="text"
                                   class="input-full"
                                   value="************">
						<?php else:
							renderTextInput( 'cs_geo_licence_key', null, false, false, false, 'License key', 'js-control-element' );
						endif; ?>
                    </div>

                    <p class="text-gray text-small lh-162"><?php esc_html_e( 'After creating the license key, you need to wait up to 5 minutes before activating it.', 'consent-magic' ) ?></p>
                </div>

                <div class="geolocation-buttons">
                    <div class="geolocation-buttons-inner">
                        <button type="button" class="btn btn-primary btn-primary-type2 cs_activate_geo_license"
                                data-action="<?php echo ( $activated === 1 ) ? 0 : 1; ?>"
                                data-deactivate="<?php esc_html_e( 'Deactivate', 'consent-magic' ); ?>"
                                data-activate="<?php esc_html_e( 'Activate', 'consent-magic' ); ?>"
                        >
							<?php if ( $activated === 1 ) {
								esc_html_e( 'Deactivate', 'consent-magic' );
							} else {
								esc_html_e( 'Activate', 'consent-magic' );
							} ?>
                        </button>
						<?php include CMPRO_PLUGIN_VIEWS_PATH . "admin/components/spinner.php"; ?>
                    </div>
                </div>

                <div class="line"></div>

                <div class="d-flex align-items-center">
					<?php render_switcher_input( 'cs_geolocation' ); ?>
                    <h4 class="secondary-heading switcher-label"><?php esc_html_e( 'Enable geolocation', 'consent-magic' ); ?></h4>
                </div>

                <div class="geolocation-settings gap-24 <?php echo !ConsentMagic()->getOption( 'cs_geolocation' ) ? 'disabled' : ''; ?>">
                    <div>
                        <h4 class="font-semibold-type2 mb-4">
							<?php esc_html_e( 'Database file path', 'consent-magic' ); ?>:
                        </h4>

                        <input type="text" name="cs_geo_db_file_path" id="cs_geo_db_file_path"
                               class="input-full"
                               value="<?php echo trailingslashit( str_replace( '\\', '/', CMPRO_PLUGIN_PATH ) ) . trailingslashit( ConsentMagic()->getOption( 'cs_geo_db_file_path' ) ); ?>"
                               disabled/>
                    </div>

                    <div class="line"></div>

                    <div>
                        <h4 class="font-semibold-type2 mb-4"><?php esc_html_e( 'How often to download the files options:', 'consent-magic' ); ?></h4>

                        <div>
							<?php renderSelectInput( 'cs_often_update', $cs_often_update_list, true ); ?>
                        </div>
                    </div>

                    <div class="line"></div>

                    <div class="geolocation-buttons">
                        <div class="geolocation-buttons-inner">
                            <button type="button" class="btn btn-primary btn-primary-type2 cs_force_update">
								<?php esc_html_e( 'Force update database', 'consent-magic' ); ?>
                            </button>
							<?php include CMPRO_PLUGIN_VIEWS_PATH . "admin/components/spinner.php"; ?>
                        </div>

                        <div class="cs_message_db_update">
                            <div class="message geolocation-warning-message mt-16" style="display: none;">
								<?php render_warning_message( '' ); ?>
                            </div>

                            <div class="message geolocation-info-message mt-16" style="display: none;">
								<?php render_info_message( '' ); ?>
                            </div>
                        </div>
                    </div>
                </div>
            </div>
        </div>
    </div>

    <div class="card card-style9">
        <div class="card-header card-header-style3">
            <h3 class="primary-heading-type2">
				<?php esc_html_e( 'Instruction for creating MaxMind license key', 'consent-magic' ); ?>
            </h3>

			<?php
			if ( $activated === 1 ) {
				cardCollapseSettingsWithText( 'Show' );
			} else {
				cardCollapseSettingsWithText( 'Collapse', 'Show' );
			}
			?>
        </div>

        <div class="card-body" style="<?php echo $activated === 0 ? 'display: block' : ''; ?>">
            <div class="gap-24">
                <h4 class="primary-heading-type2"><?php esc_html_e( 'MaxMind Geolocation Integration', 'consent-magic' ); ?></h4>

                <div>
                    <p class="text-gray fw-500 mb-16"><?php esc_html_e( 'Create an account on MaxMind:', 'consent-magic' ); ?></p>

                    <ul class="cm-list-style">
                        <li><?php echo esc_html__( 'Create an ', 'consent-magic' ) . '<a href="https://www.maxmind.com/en/geolite2/signup" target="_blank" class="link">' . esc_html__( 'MaxMind Geolocation account', 'consent-magic' ) . '</a>:'; ?></li>
                        <li><?php esc_html_e( 'On the registration page, complete the form with your information. If you’re not sure what to choose as "Industry" and "Intended use" fields you can fill as "eCommerce" and "Enforcing digital rights" respectively.', 'consent-magic' ); ?></li>
                        <li><?php echo esc_html__( 'After submitting you’ll get an email with a link to set your password shortly. ', 'consent-magic' ) . esc_html__( 'Click on that link and choose a password.', 'consent-magic' ); ?></li>
                    </ul>
                </div>

                <div class="img-wrap">
                    <img src="<?php echo esc_url( CMPRO_PLUGIN_URL . 'assets/images/instructions/geolocation-image1.png' ) ?>"
                         alt="instructions"/>
                    <img src="<?php echo esc_url( CMPRO_PLUGIN_URL . 'assets/images/instructions/geolocation-image2.png' ) ?>"
                         alt="instructions"/>
                </div>

                <div>
                    <p class="text-gray fw-500 mb-16"><?php esc_html_e( 'Generate your MaxMind license key:', 'consent-magic' ); ?></p>

                    <ul class="cm-list-style">
                        <li><?php echo esc_html__( 'Go to: ', 'consent-magic' ) . '<a href="https://www.maxmind.com/en/account" target="_blank" class="link">https://www.maxmind.com/en/account</a>:'; ?></li>
                        <li><?php esc_html_e( 'Select the "Manage License Keys" tab in your user account dashboard.', 'consent-magic' ); ?></li>
                        <li><?php esc_html_e( 'Select "Generate new license key".', 'consent-magic' ); ?></li>
                    </ul>
                </div>

                <div class="img-wrap">
                    <img src="<?php echo esc_url( CMPRO_PLUGIN_URL . 'assets/images/instructions/geolocation-image3.png' ) ?>"
                         alt="instructions"/>
                </div>

                <ul class="cm-list-style">
                    <li><?php esc_html_e( 'You will be asked if you want to use the old license type. Select No', 'consent-magic' ); ?></li>
                </ul>

                <div class="img-wrap">
                    <img src="<?php echo esc_url( CMPRO_PLUGIN_URL . 'assets/images/instructions/geolocation-image4.png' ) ?>"
                         alt="instructions"/>
                </div>

                <ul class="cm-list-style">
                    <li><?php esc_html_e( 'Copy the license key and paste it inside ConsentMagic settings.', 'consent-magic' ); ?></li>
                </ul>

                <div class="img-wrap">
                    <img src="<?php echo esc_url( CMPRO_PLUGIN_URL . 'assets/images/instructions/geolocation-image5.png' ) ?>"
                         alt="instructions"/>
                </div>
            </div>
        </div>
    </div>
</div>