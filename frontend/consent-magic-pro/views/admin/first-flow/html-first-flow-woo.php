<?php

namespace ConsentMagicPro;

// If this file is called directly, abort.
if ( !defined( 'WPINC' ) ) {
	die;
}

?>
<div class="first-flow">
    <div class="cs-modal-admin">
        <div class="cs-modal-dialog" role="document">
            <div class="cs-modal-content cs_settings_form">
                <div class="cs-modal-body">
                    <div class="steps-wrap">
						<?php if ( ConsentMagic()->getOption( 'wc_am_client_consent_magic_pro_activated' ) !== 'Activated' ) { ?>
                            <div id="step_one" class="step-wrap">
                                <form action="<?php echo esc_url( admin_url( 'admin-post.php' ) ); ?>" method="post">
                                    <div class="step">
                                        <h3 class="heading primary-heading"><?php esc_html_e( 'Activate ConsentMagic\'s license', 'consent-magic' ); ?></h3>

                                        <div>
                                            <p class="text-gray fw-500 mb-24">
												<?php esc_html_e( 'You have your license key in the order confirmation email, and you can get it from ', 'consent-magic' ); ?>
                                                <a href="<?php echo esc_url( 'https://www.pixelyoursite.com/my-account' ); ?>"
                                                   style="text-transform: none;"
                                                   rel="nofollow"
                                                   class="link"
                                                   target="_blank"><?php esc_html_e( 'your account', 'consent-magic' ); ?></a>
												<?php esc_html_e( ' on our website. We also sent you an email containing your login data.', 'consent-magic' ); ?>
                                            </p>

                                            <div class="gap-16">
                                                <div class="license-data">
                                                    <input type="hidden" name="action" value="cm_validate_license" />
                                                    <h4 class="font-semibold"><?php esc_html_e( 'Plugin version:', 'consent-magic' ); ?></h4>

                                                    <div>
                                                        <span><?php echo esc_html( CMPRO_LATEST_VERSION_NUMBER ); ?></span>
                                                    </div>

                                                    <h4 class="font-semibold"><?php esc_html_e( 'License name:', 'consent-magic' ); ?></h4>

                                                    <div>
                                                        <span><?php echo esc_html( CMPRO_LICENSE_NAME ); ?></span>
                                                    </div>

                                                    <h4 class="font-semibold"><?php esc_html_e( 'License type:', 'consent-magic' ); ?></h4>

                                                    <div>
                                                        <span class="text-uppercase"><?php echo CMPRO_LICENSE_TYPE; ?></span>
                                                        <span><?php esc_html_e( ' License', 'consent-magic' ); ?></span>
                                                    </div>
                                                </div>

												<?php $error_license = get_transient( 'cm_license_notice' );
												$license_data        = ConsentMagic()->getOption( 'wc_am_client_consent_magic_pro' );
												$license             = '';
												if ( $license_data ) {
													if ( is_array( $license_data ) && isset( $license_data[ 'wc_am_client_consent_magic_pro_api_key' ] ) ) {
														$license = $license_data[ 'wc_am_client_consent_magic_pro_api_key' ];
													}
												}
												?>

                                                <div>
                                                    <h4 class="font-semibold mb-4"><?php esc_html_e( 'Your license key:', 'consent-magic' ); ?></h4>
                                                    <input id="api_key"
                                                           name="wc_am_client_consent_magic_pro[wc_am_client_consent_magic_pro_api_key]"
                                                           class="input-full <?php echo !empty( $error_license ) ? 'input-error' : ''; ?>"
                                                           size="25" type="text"
                                                           value="<?php echo esc_attr( $license ); ?>">

													<?php if ( !empty( $error_license ) ) : ?>
                                                        <div class="mt-4">
                                                            <span class="error-msg text-small"><?php esc_html_e( 'Activation error', 'consent-magic' ); ?>: <?php echo wp_kses_post( $error_license ); ?></span>
                                                        </div>
													<?php endif; ?>
                                                </div>

                                                <div>
                                                    <h4 class="font-semibold mb-4"><?php esc_html_e( 'You have your Product ID in the order confirmation email, and you can get it from your account on our website:', 'consent-magic' ); ?></h4>

                                                    <input id="api_key"
                                                           name="wc_am_product_id_consent_magic_pro"
                                                           class="input-full"
                                                           size="25" type="text"
                                                           value="<?php echo esc_attr( ConsentMagic()->getOption( 'wc_am_product_id_consent_magic_pro' ) ); ?>">
                                                </div>
                                            </div>
                                        </div>

                                        <div class="step-buttons">
                                            <input type="submit" name="submit" id="submit"
                                                   class="btn btn-sm btn-primary btn-primary-type2"
                                                   value="<?php esc_attr_e( 'Activate license', 'consent-magic' ); ?>">
                                        </div>
                                    </div>
                                </form>
                            </div>
						<?php } else { ?>
                            <div id="step_one" class="step-wrap">
                                <div class="step">
                                    <h3 class="heading primary-heading"><?php esc_html_e( 'Activate ConsentMagic\'s license', 'consent-magic' ); ?></h3>

                                    <div>
                                        <p class="text-gray fw-500 mb-24"><?php echo sprintf( esc_html__( 'Your license is now %sactivated%s, please continue to the next step.', 'consent-magic' ), '<span class="text-green">', '</span>' ); ?></p>

                                        <div class="license-data">
                                            <h4 class="font-semibold"><?php esc_html_e( 'Plugin version:', 'consent-magic' ); ?></h4>

                                            <div>
                                                <span><?php echo esc_html( CMPRO_LATEST_VERSION_NUMBER ); ?></span>
                                            </div>

                                            <h4 class="font-semibold"><?php esc_html_e( 'License name:', 'consent-magic' ); ?></h4>

                                            <div>
                                                <span><?php echo esc_html( CMPRO_LICENSE_NAME ); ?></span>
                                            </div>

                                            <h4 class="font-semibold"><?php esc_html_e( 'License type:', 'consent-magic' ); ?></h4>

                                            <div>
                                                <span class="text-uppercase"><?php echo esc_html( CMPRO_LICENSE_TYPE ); ?></span>
                                                <span><?php esc_html_e( ' License', 'consent-magic' ); ?></span>
                                            </div>
                                        </div>
                                    </div>

                                    <div class="step-buttons">
                                        <button class="btn btn-sm btn-primary btn-primary-type2 btn_second_step"> <?php esc_html_e( 'Continue', 'consent-magic' ); ?>
                                        </button>
                                    </div>
                                </div>
                            </div>

							<?php include_once 'html-first-flow-modal.php'; ?>
						<?php } ?>
                    </div>
                </div>
            </div>
        </div>
    </div>
</div>
