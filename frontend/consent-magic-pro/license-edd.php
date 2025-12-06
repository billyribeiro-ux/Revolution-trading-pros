<?php

namespace ConsentMagicPro;

// If this file is called directly, abort.
if ( !defined( 'WPINC' ) ) {
	die;
}

if ( !class_exists( 'CS_EDD_License_manager' ) ) {
	class CS_EDD_License_manager {

		private $data_key                    = '';
		private $text_domain                 = '';
		private $edd_api_key                 = '';
		private $plugin_name                 = '';
		private $edd_deactivate_checkbox_key = '';
		private $edd_activated_key           = '';
		private $license_name                = '';
		private $edd_check_license_key       = '';
		private $plugin_version              = '';

		public $edd_check_for_update_key = '';
		public $api_url                  = '';
		public $edd_license_key          = '';

		public function __construct( $text_domain, $license_name, $plugin_version, $plugin_name ) {

			$this->edd_check_for_update_key = 'edd_check_for_update_key';
			$this->api_url                  = 'https://www.pixelyoursite.com';
			$this->edd_license_key          = 'edd_license_key';

			if ( is_admin() ) {

				$this->data_key                    = CMPRO_LICENSE_PLUGIN_NAME;
				$this->edd_api_key                 = CMPRO_LICENSE_TYPE . '_' . CMPRO_LICENSE_PLUGIN_NAME . '_api_key';
				$this->plugin_name                 = $plugin_name;
				$this->edd_check_license_key       = $this->data_key . '_check_license';
				$this->text_domain                 = $text_domain;
				$this->license_name                = $license_name;
				$this->plugin_version              = $plugin_version;
				$this->edd_activated_key           = $this->license_name . '_' . $this->data_key . '_activated';
				$this->edd_deactivate_checkbox_key = $this->license_name .
				                                     '_' .
				                                     $this->data_key .
				                                     '_deactivate_checkbox';

				add_action( 'admin_init', array(
					$this,
					'check_license'
				) );

				/**
				 * Check for software updates
				 */
				if ( get_transient( $this->edd_check_for_update_key ) === false &&
				     ( ConsentMagic()->getOption( 'edd_consent_magic_pro_activated' ) == 'Activated' ||
				       ConsentMagic()->getOption( 'edd_consent_magic_pro_activated' ) == 'Expired' ) ) {
					$this->update_check();
				}
				add_action( 'admin_init', array(
					$this,
					'update_check'
				), 0 );

				add_action( 'admin_post_cm_validate_license', function() {
					$this->validate_options();
					wp_safe_redirect( admin_url( 'admin.php?page=cs-license' ) );
				} );

				add_action( 'admin_post_cm_deactivate_license', function() {
					$this->deactivation();
					wp_safe_redirect( admin_url( 'admin.php?page=cs-license' ) );
				} );
			}
		}

		//check license
		public function check_license() {
			if ( get_transient( $this->edd_check_license_key ) === false &&
			     ( ConsentMagic()->getOption( 'edd_consent_magic_pro_activated' ) == 'Activated' ||
			       ConsentMagic()->getOption( 'edd_consent_magic_pro_activated' ) == 'Expired' ) ) {
				$activation_key = ConsentMagic()->getOption( $this->edd_license_key );
				$response       = $this->remote_query( $activation_key, 'check_license' );

				if ( !is_wp_error( $response ) ) {
					$status_code = wp_remote_retrieve_response_code( $response );
					if ( $status_code == 403 || $status_code == 415 ) {
						$ip_list = getAllServerIps();
						$message = __(
							           "The request may have been blocked by our firewall. Please try again later. If the problem persists, contact our support and provide the following IP addresses: ",
							           'consent-magic'
						           ) . $ip_list;

						set_transient( 'cm_license_notice', $message, 60 );

						exit;
					}

					$response = json_decode( wp_remote_retrieve_body( $response ) );
					if ( $response->success === false ||
					     $response->license === 'invalid' ||
					     $response->license === 'expired' ||
					     $response->license === 'disabled' ||
					     $response->license === 'key_mismatch' ||
					     $response->license === 'invalid_item_id' ||
					     $response->license === 'item_name_mismatch' ) {

						ConsentMagic()->updateOptions( array( $this->edd_activated_key => 'Deactivated' ) );
						ConsentMagic()->updateOptions( array( $this->edd_deactivate_checkbox_key => 'on' ) );
						ConsentMagic()->updateOptions( array( $this->edd_license_key => '' ) );

						delete_transient( $this->edd_check_license_key );
						set_transient( 'cm_license_notice', sprintf( __( 'Your license is %s', 'consent-magic' ), $response->license ), 30 );
						$this->cs_db_cron_deactivate();

						wp_safe_redirect( admin_url( '/admin.php?page=cs-license' ) );
						exit;

					} else {
						if ( $response->success === true && $response->license === 'valid' ) {
							set_transient( $this->edd_check_license_key, $this->edd_check_license_key, DAY_IN_SECONDS );
						} else {
							set_transient( $this->edd_check_license_key, $this->edd_check_license_key, 600 );
							set_transient( 'cm_license_notice', __( 'Error activation license', 'consent-magic' ), 30 );
						}
					}
				} else {
					set_transient( $this->edd_check_license_key, $this->edd_check_license_key, 600 );
					set_transient( 'cm_license_notice', __( 'Error activation license', 'consent-magic' ), 30 );
				}
			}
		}

		//check license options
		public function validate_options() {

			if ( !isset( $_POST[ 'edd_consent_magic_pro' ][ $this->edd_api_key ] ) ) {
				return;
			}

			$api_key = trim( sanitize_text_field( $_POST[ 'edd_consent_magic_pro' ][ $this->edd_api_key ] ) );
			ConsentMagic()->updateOptions( array( $this->edd_license_key => $api_key ) );

			$activation_result = $this->remote_query( $api_key, 'activate_license' );

			if ( is_wp_error( $activation_result ) || wp_remote_retrieve_response_code( $activation_result ) != 200 ) {
				$message = __( 'Connection failed to the License Key API server. Try again later. There may be a problem on your server preventing outgoing requests, or the store is blocking your request to activate the plugin/theme.', $this->text_domain );
				ConsentMagic()->updateOptions( array( $this->edd_activated_key => 'Deactivated' ) );
				ConsentMagic()->updateOptions( array( $this->edd_deactivate_checkbox_key => 'on' ) );
				add_settings_error( 'edd_error_text', 'api_key_check_error', esc_html( $message ) );
				// add error
				set_transient( 'cm_license_notice', esc_html( $message ), 30 );
			}

			$status_code = wp_remote_retrieve_response_code( $activation_result );
			if ( $status_code == 403 || $status_code == 415 ) {
				$ip_list = getAllServerIps();
				$message = __(
					           "The request may have been blocked by our firewall. Please try again later. If the problem persists, contact our support and provide the following IP addresses: ",
					           'consent-magic'
				           ) . $ip_list;

				set_transient( 'cm_license_notice', $message, 60 );

				return;
			}

			$activation_result = json_decode( wp_remote_retrieve_body( $activation_result ) );

			if ( !empty( $activation_result ) ) {
				if ( $activation_result->success === true && $activation_result->license === 'valid' ) {
					$message = __( sprintf( '%s activated. ', $this->plugin_name ), $this->text_domain ) .
					           __( 'Activation completed successfully', $this->text_domain );
					add_settings_error( 'activate_text', 'activate_msg', esc_html( $message ), 'updated' );
					ConsentMagic()->updateOptions( array( $this->edd_activated_key => 'Activated' ) );
					ConsentMagic()->updateOptions( array( $this->edd_deactivate_checkbox_key => 'off' ) );
					delete_transient( 'cm_license_notice' );
					set_transient( 'cm_license_success_notice', esc_html( $message ), 30 );
				}

				if ( $activation_result->success === false ) {
					switch ( $activation_result->error ) {

						case 'expired' :
							$message = sprintf( __( 'Your license key expired on %s.', $this->text_domain ), date_i18n( get_option( 'date_format' ), strtotime( $activation_result->expires, current_time( 'timestamp' ) ) ) );
							break;

						case 'revoked' :
							$message = __( 'Your license key has been disabled.', $this->text_domain );
							break;

						case 'missing':
						case 'invalid':
							$message = __( 'Invalid license.', $this->text_domain );
							break;

						case 'site_inactive' :
							$message = __( 'Your license is not active for this URL.', $this->text_domain );
							break;

						case 'item_name_mismatch' :
							$message = sprintf( __( 'This appears to be an invalid license key for %s.', $this->text_domain ), $this->plugin_name );
							break;

						case 'no_activations_left':
							$message = __( 'Your license key has reached its activation limit.', $this->text_domain );
							break;

						default :
							$message = __( 'An error occurred, please try again.', $this->text_domain );
							break;
					}

					add_settings_error( 'edd_error_text', 'api_key_check_error', esc_html( $message ) );
					ConsentMagic()->updateOptions( array( $this->edd_activated_key => 'Deactivated' ) );
					$this->cs_db_cron_deactivate();

					// add error
					set_transient( 'cm_license_notice', esc_html( $message ), 30 );
				}
			} else {
				$message = __( 'The API Key activation could not be completed due to an unknown error possibly on the store server The activation results were empty.', $this->text_domain );
				add_settings_error( 'not_activated_empty_response_text', 'not_activated_empty_response_error', esc_html( $message ), 'updated' );
				set_transient( 'cm_license_notice', $message, 30 );
			}
		}

		// Deactivates License
		public function deactivation() {

			if ( !isset( $_POST[ 'edd_consent_magic_pro_deactivate_checkbox' ] ) ) {
				return;
			}

			$type = $_POST[ 'edd_consent_magic_pro_deactivate_checkbox' ] == 'deactivate' ? 'deactivate' : 'reactivate';

			if ( $type == 'reactivate' && isset( $_POST[ 'edd_consent_magic_pro' ] ) ) {
				$this->validate_options();
			} else {
				$activation_status = ConsentMagic()->getOption( $this->edd_activated_key );
				$activation_key    = ConsentMagic()->getOption( $this->edd_license_key );

				if ( !empty( $activation_key ) &&
				     ( $activation_status == 'Activated' || $activation_status == 'Expired' ) ) {
					// deactivates API Key activation
					$deactivation_result = $this->remote_query( $activation_key, 'deactivate_license' );

					if ( is_wp_error( $deactivation_result ) ||
					     200 !== wp_remote_retrieve_response_code( $deactivation_result ) ) {

						$status_code = wp_remote_retrieve_response_code( $deactivation_result );
						if ( $status_code == 403 || $status_code == 415 ) {
							$ip_list = getAllServerIps();
							$message = __(
								           "The request may have been blocked by our firewall. Please try again later. If the problem persists, contact our support and provide the following IP addresses: ",
								           'consent-magic'
							           ) . $ip_list;

							set_transient( 'cm_license_notice', $message, 60 );

							return;
						}

						$message = ( is_wp_error( $deactivation_result ) &&
						             !empty( $deactivation_result->get_error_message() ) ) ? $deactivation_result->get_error_message() : __( 'An error occurred, please try again.', 'consent-magic' );
						add_settings_error( 'edd_deactivation_text', 'edd_deactivation_error', esc_html( $message ) );
						set_transient( 'cm_license_notice', $message, 30 );
					} else {
						$message = __( sprintf( '%s deactivated is successfully.', $this->plugin_name ), 'consent-magic' );
						ConsentMagic()->updateOptions( array( $this->edd_activated_key => 'Deactivated' ) );
						ConsentMagic()->updateOptions( array( $this->edd_deactivate_checkbox_key => 'on' ) );
						ConsentMagic()->updateOptions( array( $this->edd_license_key => '' ) );
						add_settings_error( 'edd_deactivation_text', 'edd_deactivation_message', esc_html( $message ), 'updated' );
						set_transient( 'cm_license_deactivation_notice', esc_html( $message ), 30 );
						$this->cs_db_cron_deactivate();
					}
				}
			}
		}

		//API query
		public function remote_query( $key, $action ) {
			$api_params = array(
				'edd_action' => $action,
				'license'    => $key,
				'item_name'  => $this->plugin_name,
				'url'        => home_url()
			);

			$response = wp_remote_post( $this->api_url, array(
				'timeout'   => 15,
				'sslverify' => false,
				'body'      => $api_params
			) );

			return $response;
		}

		//cron deactivate
		public function cs_db_cron_deactivate() {
			if ( wp_next_scheduled( 'cs_db_cron_update_hook' ) ) {
				wp_clear_scheduled_hook( 'cs_db_cron_update_hook' );
			}
		}

		//function check update
		public function update_check() {

			if ( !class_exists( 'ConsentMagicPro\CS_EDD_Updater' ) ) {
				require_once CMPRO_PLUGIN_PATH . 'includes/CS_EDD_Updater.php';
			}

			$license_key = ConsentMagic()->getOption( $this->edd_license_key );

			$plugin_updater = new CS_EDD_Updater( $this->api_url, CMPRO_PLUGIN_FILENAME, array(
				'version'   => CMPRO_LATEST_VERSION_NUMBER,
				'license'   => $license_key,
				'item_name' => CMPRO_LICENSE_NAME,
				'author'    => CMPRO_LICENSE_NAME
			) );

			set_transient( $this->edd_check_for_update_key, $this->edd_check_for_update_key, DAY_IN_SECONDS );
		}
	}
}