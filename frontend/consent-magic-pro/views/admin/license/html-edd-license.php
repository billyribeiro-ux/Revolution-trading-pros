<?php
namespace ConsentMagicPro;

// If this file is called directly, abort.
if ( !defined( 'WPINC' ) ) {
	die;
}

$status = ConsentMagic()->getOption( 'edd_consent_magic_pro_activated' );
?>

<div class="card card-static card-style9 license license-edd">
    <div class="card-header card-header-style3">
        <div class="d-flex justify-content-between align-items-center w-100">
            <div class="d-flex align-items-center">
                <div class="cm-logo-small mr-16">
                    <img src="<?php echo esc_url( CMPRO_PLUGIN_URL ); ?>/assets/images/cm-logo.svg" alt="cm-logo">
                </div>

                <h3 class="primary-heading-type2">
					<?php esc_html_e( 'Manage your ConsentMagic Pro license', 'consent-magic' ); ?>
                </h3>
            </div>

            <div class="deactivation-wrap deactivation-wrap-top">
				<?php if ( $status == 'Activated' || $status == 'Expired' ) : ?>
                    <div class="deactivate-button">
                        <button
                                name="deactivate"
                                class="btn btn-red btn-primary-type2 cm-license-action">
							<?php esc_attr_e( 'Deactivate License', 'consent-magic' ); ?>
                        </button>
                    </div>
				<?php endif; ?>

                <div class="license-loader">
					<?php include CMPRO_PLUGIN_VIEWS_PATH . "admin/components/spinner.php"; ?>
                </div>
            </div>
        </div>
    </div>

    <div class="card-body">
        <div class="gap-22">
            <div class="license-info">
				<?php if ( $status == 'Activated' || $status == 'Expired' ) : ?>
                    <h4 class="secondary-heading"><?php esc_html_e( 'Your license key:', 'consent-magic' ); ?></h4>
                    <p class="text-green fw-500"><?php esc_html_e( 'Activated', 'consent-magic' ); ?></p>
				<?php endif; ?>

                <h4 class="secondary-heading"><?php esc_html_e( 'Plugin version:', 'consent-magic' ); ?></h4>

                <div>
                    <span class="fw-500"><?php echo esc_html( CMPRO_LATEST_VERSION_NUMBER ); ?></span>
                </div>

                <h4 class="secondary-heading"><?php esc_html_e( 'License name:', 'consent-magic' ); ?></h4>

                <div>
                    <span class="fw-500"><?php echo esc_html( CMPRO_LICENSE_NAME ); ?></span>
                </div>

                <h4 class="secondary-heading"><?php esc_html_e( 'License type:', 'consent-magic' ); ?></h4>

                <div>
                    <span class="text-uppercase fw-500"><?php echo esc_html( CMPRO_LICENSE_TYPE ); ?></span>&nbsp;<span
                            class="fw-500"><?php esc_html_e( 'License', 'consent-magic' ); ?></span>
                </div>
            </div>

            <div>
                <h4 class="font-semibold-type2 mb-2">
					<?php esc_html_e( 'Your license key', 'consent-magic' ); ?>:
                </h4>

				<?php if ( $status == 'Activated' || $status == 'Expired' ) : ?>
                    <input type="hidden" name="action" value="cm_deactivate_license" />
                    <div class="cm-license-key">
                        <input type="hidden" id="edd_consent_magic_pro_deactivate_checkbox"
                               name="edd_consent_magic_pro_deactivate_checkbox" value="">

                        <input id="api_key"
                               class="input-full"
                               name="edd_consent_magic_pro[edd_consent_magic_pro_api_key]"
                               size="25"
                               required="required"
                               type="text"
                               placeholder="******************************************">

                        <div class="license-key-action license-loader license-loader-bottom">
                            <button
                                    name="activate"
                                    class="btn btn-green btn-primary-type2 cm-license-action">
								<?php esc_attr_e( 'Reactivate License', 'consent-magic' ); ?>
                            </button>

                            <div class="deactivation-wrap">
                                <div class="deactivate-button">
                                    <button
                                            name="deactivate"
                                            class="btn btn-red btn-primary-type2 cm-license-action">
										<?php esc_attr_e( 'Deactivate License', 'consent-magic' ); ?>
                                    </button>
                                </div>
                            </div>

							<?php include CMPRO_PLUGIN_VIEWS_PATH . "admin/components/spinner.php"; ?>
                        </div>
                    </div>
				<?php else : ?>
                    <div class="cm-license-key">
                        <input type="hidden" name="action" value="cm_validate_license" />
                        <div>
                            <input id="api_key"
                                   class="input-full"
                                   name="edd_consent_magic_pro[edd_consent_magic_pro_api_key]"
                                   size="25"
                                   required="required"
                                   type="text"
                                   value="">

                            <p class="license-description mt-4 lh-162">
								<?php esc_html_e( 'You have your license key in the order confirmation email, and you can get it from ', 'consent-magic' );
								?>
                                <a href="<?php echo esc_url( 'https://www.pixelyoursite.com/my-account' ); ?>"
                                   rel="nofollow"
                                   class="link link-small"
                                   target="_blank">
									<?php esc_html_e( 'your account', 'consent-magic' );
									?></a>
								<?php esc_html_e( ' on our website. We also sent you an email containing your login data.', 'consent-magic' );
								?>
                            </p>
                        </div>

                        <div class="license-key-action license-loader license-loader-bottom">
                            <input type="submit"
                                   name="submit"
                                   class="btn btn-primary btn-primary-type2 activate-license-action"
                                   value="<?php esc_attr_e( 'Activate License', 'consent-magic' ); ?>">

							<?php include CMPRO_PLUGIN_VIEWS_PATH . "admin/components/spinner.php"; ?>
                        </div>
                    </div>
				<?php endif; ?>

                <div class="mt-24">
					<?php render_info_message_type2( sprintf( __( 'If your don\'t have key, you can take it from %s', 'consent-magic' ), '<a href="https://www.pixelyoursite.com/my-account" rel="nofollow" target="_blank">' . __( 'here', 'consent-magic' ) . '</a>' ) ); ?>
                </div>

				<?php $error_license = get_transient( 'cm_license_notice' ); ?>
				<?php if ( !empty( $error_license ) ) : ?>
                    <div class="mt-24">
						<?php
						$message = __( 'Activation error', 'consent-magic' ) . ': ' . $error_license;
						render_critical_message( $message ); ?>
                    </div>
					<?php delete_transient( 'cm_license_notice' ); ?>
				<?php endif; ?>

				<?php $deactivation_notice = get_transient( 'cm_license_deactivation_notice' ); ?>
				<?php if ( !empty( $deactivation_notice ) ) : ?>
                    <div class="mt-24">
						<?php render_warning_message( $deactivation_notice ); ?>
                    </div>
					<?php
					delete_transient( 'cm_license_deactivation_notice' );
				endif; ?>

				<?php $success_notice = get_transient( 'cm_license_success_notice' ); ?>
				<?php if ( !empty( $success_notice ) ) : ?>
                    <div class="mt-24">
						<?php render_success_message( $success_notice ); ?>
                    </div>
					<?php
					delete_transient( 'cm_license_success_notice' );
				endif; ?>
            </div>
        </div>
    </div>
</div>
