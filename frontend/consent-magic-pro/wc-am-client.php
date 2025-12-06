<?php

/**
 * The WooCommerce API Manager PHP Client Library is designed to be droppped into a WordPress plugin or theme.
 * This version is designed to be used with the WooCommerce API Manager version 2.x.
 * Intellectual Property rights, and copyright, reserved by Todd Lahman, LLC as allowed by law include,
 * but are not limited to, the working concept, function, and behavior of this software,
 * the logical code structure and expression as written.
 * @version       2.8
 * @author        Todd Lahman LLC https://toddlahman.com/
 * @copyright     Copyright (c) Todd Lahman LLC (support@toddlahman.com)
 * @package       WooCommerce API Manager plugin and theme library
 * @license       Copyright Todd Lahman LLC
 */

namespace ConsentMagicPro;

defined( 'ABSPATH' ) || exit;

if ( !class_exists( 'WC_AM_Client_2_8' ) ) {
	class WC_AM_Client_2_8 {

		/**
		 * Class args
		 * @var string
		 */
		private $api_url          = '';
		private $data_key         = '';
		private $file             = '';
		private $plugin_name      = '';
		private $plugin_or_theme  = '';
		private $product_id       = '';
		private $slug             = '';
		private $software_title   = '';
		private $software_version = '';
		private $text_domain      = ''; // For language translation.

		/**
		 * Class properties.
		 * @var string
		 */
		private $data                              = array();
		private $identifier                        = '';
		private $no_product_id                     = false;
		private $product_id_chosen                 = 0;
		private $wc_am_activated_key               = '';
		private $wc_am_activation_tab_key          = '';
		private $wc_am_api_key_key                 = '';
		private $wc_am_deactivate_checkbox_key     = '';
		private $wc_am_deactivation_tab_key        = '';
		private $wc_am_auto_update_key             = '';
		private $wc_am_domain                      = '';
		private $wc_am_instance_id                 = '';
		private $wc_am_instance_key                = '';
		private $wc_am_menu_tab_activation_title   = '';
		private $wc_am_menu_tab_deactivation_title = '';
		private $wc_am_plugin_name                 = '';
		private $wc_am_product_id                  = '';
		private $wc_am_renew_license_url           = '';
		private $wc_am_settings_menu_title         = '';
		private $wc_am_settings_title              = '';
		private $wc_am_software_version            = '';

		public function __construct( $file, $product_id, $software_version, $plugin_or_theme, $api_url, $software_title = '', $text_domain = '' ) {
			$this->no_product_id   = empty( $product_id );
			$this->plugin_or_theme = esc_attr( strtolower( $plugin_or_theme ) );

			if ( $this->no_product_id ) {
				$this->identifier        = $this->plugin_or_theme == 'plugin' ? dirname( untrailingslashit( plugin_basename( $file ) ) ) : basename( dirname( plugin_basename( $file ) ) );
				$product_id              = strtolower( str_ireplace( array(
					' ',
					'_',
					'&',
					'?',
					'-'
				), '_', $this->identifier ) );
				$this->wc_am_product_id  = 'wc_am_product_id_' . $product_id;
				$this->product_id_chosen = ConsentMagic()->getOption( $this->wc_am_product_id );
			} else {
				/**
				 * Preserve the value of $product_id to use for API requests. Pre 2.0 product_id is a string, and >= 2.0 is an integer.
				 */
				if ( is_int( $product_id ) ) {
					$this->product_id = absint( $product_id );
				} else {
					$this->product_id = esc_attr( $product_id );
				}
			}

			// If the product_id was not provided, but was saved by the customer, used the saved product_id.
			if ( empty( $this->product_id ) && !empty( $this->product_id_chosen ) ) {
				$this->product_id = $this->product_id_chosen;
			}

			$this->file             = $file;
			$this->software_title   = esc_attr( $software_title );
			$this->software_version = esc_attr( $software_version );
			$this->api_url          = esc_url( $api_url );
			$this->text_domain      = esc_attr( $text_domain );
			/**
			 * If the product_id is a pre 2.0 string, format it to be used as an option key, otherwise it will be an integer if >= 2.0.
			 */
			$this->data_key            = 'wc_am_client_' . strtolower( str_ireplace( array(
					' ',
					'_',
					'&',
					'?',
					'-'
				), '_', $product_id ) );
			$this->wc_am_activated_key = $this->data_key . '_activated';

			if ( is_admin() ) {
				if ( !empty( $this->plugin_or_theme ) && $this->plugin_or_theme == 'theme' ) {
					add_action( 'admin_init', array(
						$this,
						'activation'
					) );
				}

				if ( !empty( $this->plugin_or_theme ) && $this->plugin_or_theme == 'plugin' ) {
					register_activation_hook( $this->file, array(
						$this,
						'activation'
					) );
				}

				// Check for external connection blocking
				add_action( 'admin_notices', array(
					$this,
					'check_external_blocking'
				) );

				/**
				 * Set all data defaults here
				 */
				$this->wc_am_api_key_key  = $this->data_key . '_api_key';
				$this->wc_am_instance_key = $this->data_key . '_instance';

				/**
				 * Set all admin menu data
				 */
				$this->wc_am_deactivate_checkbox_key     = $this->data_key . '_deactivate_checkbox';
				$this->wc_am_activation_tab_key          = $this->data_key . '_dashboard';
				$this->wc_am_deactivation_tab_key        = $this->data_key . '_deactivation';
				$this->wc_am_auto_update_key             = $this->data_key . '_auto_update';
				$this->wc_am_settings_menu_title         = $this->software_title . esc_html__( ' Activation', $this->text_domain );
				$this->wc_am_settings_title              = $this->software_title . esc_html__( ' API Key Activation', $this->text_domain );
				$this->wc_am_menu_tab_activation_title   = esc_html__( 'API Key Activation', $this->text_domain );
				$this->wc_am_menu_tab_deactivation_title = esc_html__( 'API Key Deactivation', $this->text_domain );

				/**
				 * Set all software update data here
				 */
				$this->data                    = ConsentMagic()->getOption( $this->data_key );
				$this->wc_am_plugin_name       = $this->plugin_or_theme == 'plugin' ? untrailingslashit( plugin_basename( $this->file ) ) : basename( dirname( plugin_basename( $file ) ) ); // same as plugin slug. if a theme use a theme name like 'twentyeleven'
				$this->wc_am_renew_license_url = $this->api_url . 'my-account'; // URL to renew an API Key. Trailing slash in the upgrade_url is required.
				$this->wc_am_instance_id       = ConsentMagic()->getOption( $this->wc_am_instance_key ); // Instance ID (unique to each blog activation)

				if ( empty( $this->wc_am_instance_id ) ) {
					$token = $this->generate_token();
					ConsentMagic()->updateOptions(  array( $this->wc_am_instance_key => $token ) );
					$this->wc_am_instance_id = $token;
				}

				add_action( 'admin_post_cm_validate_license', function() {
					$this->validate_options();
					wp_safe_redirect( admin_url( 'admin.php?page=cs-license' ) );
				} );

				add_action( 'admin_post_cm_deactivate_license', function() {
					$this->deactivation();
					wp_safe_redirect( admin_url( 'admin.php?page=cs-license' ) );
				} );

				/**
				 * Some web hosts have security policies that block the : (colon) and // (slashes) in http://,
				 * so only the host portion of the URL can be sent. For example the host portion might be
				 * www.example.com or example.com. http://www.example.com includes the scheme http,
				 * and the host www.example.com.
				 * Sending only the host also eliminates issues when a client site changes from http to https,
				 * but their activation still uses the original scheme.
				 * To send only the host, use a line like the one below:
				 * $this->wc_am_domain = str_ireplace( array( 'http://', 'https://' ), '', home_url() ); // blog domain name
				 */
				$this->wc_am_domain           = str_ireplace( array(
					'http://',
					'https://'
				), '', home_url() ); // blog domain name
				$this->wc_am_software_version = $this->software_version; // The software version

				/**
				 * Check for software updates
				 */
				$this->check_for_update();

				if ( !empty( $this->wc_am_activated_key ) && ConsentMagic()->getOption( $this->wc_am_activated_key ) != 'Activated' && ConsentMagic()->getOption( $this->wc_am_activated_key ) != 'Canceled' && ConsentMagic()->getOption( $this->wc_am_activated_key ) != 'Expired' ) {
					add_action( 'admin_notices', array(
						$this,
						'inactive_notice'
					) );
					$this->cs_db_cron_deactivate();
				} else if ( ConsentMagic()->getOption( $this->wc_am_activated_key ) == 'Canceled' ) {
					add_action( 'admin_notices', array(
						$this,
						'canceled_notice'
					) );
					$this->cs_db_cron_deactivate();
				} else if ( ConsentMagic()->getOption( $this->wc_am_activated_key ) == 'Expired' ) {
					add_action( 'admin_notices', array(
						$this,
						'expired_notice'
					) );
					$this->cs_db_cron_deactivate();
				}

				$this->cs_cron_check_status();

				/**
				 * Makes auto updates available if WP >= 5.5.
				 * @since 2.8
				 */
				$this->try_automatic_updates();

				if ( $this->plugin_or_theme == 'plugin' ) {
					add_filter( 'plugin_auto_update_setting_html', array(
						$this,
						'auto_update_message'
					), 10, 3 );
				}
			}
		}

		public function cs_db_cron_deactivate() {
			if ( wp_next_scheduled( 'cs_db_cron_update_hook' ) ) {
				wp_clear_scheduled_hook( 'cs_db_cron_update_hook' );
			}
		}

		public function cs_cron_check_status() {
			if ( !wp_next_scheduled( 'cs_cron_check_status_hook' ) ) {
				wp_schedule_event( time(), 'daily', 'cs_cron_check_status_hook' );
			}

			return false;
		}


		/**
		 *  Tries auto updates.
		 * @since 2.8
		 */
		public function try_automatic_updates() {
			global $wp_version;

			if ( version_compare( $wp_version, '5.5', '>=' ) ) {

				if ( $this->plugin_or_theme == 'plugin' ) {
					add_filter( 'auto_update_plugin', array(
						$this,
						'maybe_auto_update'
					), 10, 2 );
				} elseif ( $this->plugin_or_theme == 'theme' ) {
					add_filter( 'auto_update_theme', array(
						$this,
						'maybe_auto_update'
					), 10, 2 );
				}
			}
		}

		/**
		 * Tries to set auto updates.
		 * @param bool|null $update
		 * @param object    $item
		 * @return bool
		 * @since 2.8
		 */
		public function maybe_auto_update( $update, $item ) {
			if ( strpos( $this->wc_am_plugin_name, '.php' ) !== 0 ) {
				$slug = dirname( $this->wc_am_plugin_name );
			} else {
				$slug = $this->wc_am_plugin_name;
			}

			if ( isset( $item->slug ) && $item->slug == $slug ) {
				if ( $this->is_auto_update_disabled() ) {
					return false;
				}

				if ( !$this->get_api_key_status() ) {
					return false;
				}

				return true;
			}

			return $update;
		}

		/**
		 * Checks if auto updates are disabled.
		 * @return bool
		 * @since 2.8
		 */
		public function is_auto_update_disabled() {
			/*
			 * WordPress will not offer to update if background updates are disabled.
			 * WordPress background updates are disabled if file changes are not allowed.
			 */
			if ( defined( '\DISALLOW_FILE_MODS' ) && \DISALLOW_FILE_MODS ) {
				return true;
			}

			if ( defined( '\WP_INSTALLING' ) ) {
				return true;
			}

			$wp_updates_disabled = defined( '\AUTOMATIC_UPDATER_DISABLED' ) && \AUTOMATIC_UPDATER_DISABLED;

			/**
			 * Overrides the WordPress AUTOMATIC_UPDATER_DISABLED constant.
			 * @param bool $wp_updates_disabled true if disables.  false otherwise.
			 */
			$wp_updates_disabled = apply_filters( 'automatic_updater_disabled', $wp_updates_disabled );

			if ( $wp_updates_disabled ) {
				return true;
			}

			// Return true if this plugin or theme background update is disabled.

			return false;
		}

		/**
		 * Filter the auto-update message on the plugins page.
		 * Plugin updates stored in 'auto_update_plugins' array.
		 * @param string $html        HTML of the auto-update message.
		 * @param string $plugin_file Plugin file.
		 * @param array  $plugin_data Plugin details.
		 * @return mixed|string
		 * @see   'wp-admin/includes/class-wp-plugins-list-table.php'
		 * @since 2.8
		 */
		public function auto_update_message( $html, $plugin_file, $plugin_data ) {
			if ( $this->wc_am_plugin_name == $plugin_file ) {
				global $status, $page;

				if ( !$this->get_api_key_status() ) {
					return esc_html__( 'Auto-updates unavailable.', $this->text_domain );
				}

				$auto_updates = (array) get_site_option( 'auto_update_plugins', array() );
				$html         = array();

				if ( !empty( $plugin_data[ 'auto-update-forced' ] ) ) {
					if ( $plugin_data[ 'auto-update-forced' ] ) {
						// Forced on.
						$text = __( 'Auto-updates enabled' );
					} else {
						$text = __( 'Auto-updates disabled' );
					}

					$action     = 'unavailable';
					$time_class = ' hidden';
				} elseif ( in_array( $plugin_file, $auto_updates, true ) ) {
					$text       = __( 'Disable auto-updates' );
					$action     = 'disable';
					$time_class = '';
				} else {
					$text       = __( 'Enable auto-updates' );
					$action     = 'enable';
					$time_class = ' hidden';
				}

				$query_args = array(
					'action'        => "{$action}-auto-update",
					'plugin'        => $plugin_file,
					'paged'         => $page,
					'plugin_status' => $status,
				);

				$url = add_query_arg( $query_args, 'plugins.php' );

				if ( 'unavailable' === $action ) {
					$html[] = '<span class="label">' . esc_html( $text ) . '</span>';
				} else {
					$html[] = sprintf( '<a href="%s" class="toggle-auto-update aria-button-if-js" data-wp-action="%s">', wp_nonce_url( $url, 'updates' ), esc_html( $action ) );

					$html[] = '<span class="dashicons dashicons-update spin hidden" aria-hidden="true"></span>';
					$html[] = '<span class="label">' . esc_html( $text ) . '</span>';
					$html[] = '</a>';
				}

				if ( !empty( $plugin_data[ 'update' ] ) ) {
					$html[] = sprintf( '<div class="auto-update-time%s">%s</div>', $time_class, wp_get_auto_update_message() );
				}

				$html = implode( '', $html );
			}

			return $html;
		}

		/**
		 * Generate the default data.
		 */
		public function activation() {
			$instance_exists = ConsentMagic()->getOption( $this->wc_am_instance_key );

			if ( ConsentMagic()->getOption( $this->data_key ) === false || $instance_exists === false ) {
				if ( $instance_exists === false ) {
					ConsentMagic()->updateOptions( array(  $this->wc_am_instance_key => wp_generate_password( 12, false ) ) );
				}

				ConsentMagic()->updateOptions( array(  $this->wc_am_deactivate_checkbox_key => 'on' ) );
				ConsentMagic()->updateOptions( array(  $this->wc_am_activated_key => 'Deactivated' ) );
				$this->cs_db_cron_deactivate();
			}
		}

		/**
		 * Deletes all data if plugin deactivated
		 */
		public function uninstall() {
			/**
			 * @since 2.5.1
			 * Filter wc_am_client_uninstall_disable
			 * If set to false uninstall() method will be disabled.
			 */
			if ( apply_filters( 'wc_am_client_uninstall_disable', true ) ) {
				global $blog_id;

				$this->license_key_deactivation();

                foreach (
                    array(
                        $this->wc_am_instance_key,
                        $this->wc_am_deactivate_checkbox_key,
                        $this->wc_am_activated_key
                    ) as $option
                ) {

                    ConsentMagic()->deleteOption( $option );
                }
			}
		}

		/**
		 * Deactivates the license on the API server
		 */
		public function license_key_deactivation() {
			$activation_status = ConsentMagic()->getOption( $this->wc_am_activated_key );
			$api_key           = $this->data[ $this->wc_am_api_key_key ];

			$args = array(
				'api_key' => $api_key,
			);

			if ( !empty( $api_key ) && $activation_status == 'Activated' ) {
				if ( empty( $this->deactivate( $args ) ) ) {
					add_settings_error( 'not_deactivated_text', 'not_deactivated_error', esc_html__( 'The API Key could not be deactivated. Use the API Key Deactivation tab to manually deactivate the API Key before activating a new API Key. If all else fails, go to Plugins, then deactivate and reactivate this plugin, or if a theme change themes, then change back to this theme, then go to the Settings for this plugin/theme and enter the API Key information again to activate it. Also check the My Account dashboard to see if the API Key for this site was still active before the error message was displayed.', $this->text_domain ), 'updated' );
				}
			}
		}

		/**
		 * Displays an inactive notice when the software is inactive.
		 */
		public function inactive_notice() { ?>
			<?php
			/**
			 * @since 2.5.1
			 * Filter wc_am_client_inactive_notice_override
			 * If set to false inactive_notice() method will be disabled.
			 */ ?>
			<?php if ( apply_filters( 'wc_am_client_inactive_notice_override', true ) ) { ?>
				<?php if ( !current_user_can( 'manage_cs' ) ) {
					return;
				} ?>
				<?php if ( isset( $_GET[ 'page' ] ) && $_GET[ 'page' ] == 'cs-license' || isset( $_GET[ 'page' ] ) && $_GET[ 'page' ] == 'consent-magic' ) {
					return;
				} ?>
                <div class="notice notice-error cm-fixed-notice">
                    <p><?php echo '<strong>' . esc_html__( 'License not installed:', 'consent-magic' ) . '</strong>' . sprintf( esc_html__( 'Activate the license for your %s %s: %sclick to activate%s.', 'consent-magic' ), esc_attr( $this->software_title ), esc_attr( $this->plugin_or_theme ), '<a href="' . esc_url( admin_url( 'admin.php?page=cs-license' ) ) . '">', '</a>' ); ?></p>
                </div>
			<?php }
		}

		/**
		 * Displays an inactive notice when the software is expired.
		 */
		public function expired_notice() { ?>
			<?php
			/**
			 * @since 2.5.1
			 * Filter wc_am_client_expired_notice_override
			 * If set to false expired_notice() method will be disabled.
			 */ ?>
			<?php if ( apply_filters( 'wc_am_client_expired_notice_override', true ) ) { ?>
				<?php if ( !current_user_can( 'manage_cs' ) ) {
					return;
				} ?>
				<?php if ( isset( $_GET[ 'page' ] ) && $this->wc_am_activation_tab_key == $_GET[ 'page' ] ) {
					return;
				} ?>
                <div class="notice notice-error cm-fixed-notice">
                    <p><?php echo '<strong>' . esc_html__( 'License not installed:', 'consent-magic' ) . '</strong>' . sprintf( esc_html__( 'Your %s %s license is expired, please renew it. %sLogin to your account to renew.%s.', 'consent-magic' ), esc_attr( $this->software_title ), esc_attr( $this->plugin_or_theme ), '<a href="' . esc_url( admin_url( 'admin.php?page=cs-license' ) ) . '">', '</a>' ); ?></p>
                </div>
			<?php }
		}

		/**
		 * Displays an inactive notice when the software is canceled.
		 */
		public function canceled_notice() { ?>
			<?php
			/**
			 * @since 2.5.1
			 * Filter wc_am_client_canceled_notice_override
			 * If set to false canceled_notice() method will be disabled.
			 */ ?>
			<?php if ( apply_filters( 'wc_am_client_canceled_notice_override', true ) ) { ?>
				<?php if ( !current_user_can( 'manage_cs' ) ) {
					return;
				} ?>
				<?php if ( isset( $_GET[ 'page' ] ) && $this->wc_am_activation_tab_key == $_GET[ 'page' ] ) {
					return;
				} ?>
                <div class="notice notice-error cm-fixed-notice">
                    <p><?php echo '<strong>' . esc_html__( 'Canceled:', 'consent-magic' ) . '</strong>' . sprintf( esc_html__( 'Please replace %s %s license key with a valid one. %sClick to buy a new license.%s.', 'consent-magic' ), esc_attr( $this->software_title ), esc_attr( $this->plugin_or_theme ), '<a href="' . esc_url( 'https://www.pixelyoursite.com/plugins/consentmagic' ) . '" rel="nofollow">', '</a>' ); ?></p>
                </div>
			<?php }
		}

		/**
		 * Check for external blocking contstant.
		 */
		public function check_external_blocking() {
			// show notice if external requests are blocked through the WP_HTTP_BLOCK_EXTERNAL constant
			if ( defined( '\WP_HTTP_BLOCK_EXTERNAL' ) && \WP_HTTP_BLOCK_EXTERNAL === true ) {
				// check if our API endpoint is in the allowed hosts
				$host = wp_parse_url( $this->api_url, PHP_URL_HOST );

				if ( !defined( '\WP_ACCESSIBLE_HOSTS' ) || stristr( \WP_ACCESSIBLE_HOSTS, $host ) === false ) {
					?>
                    <div class="notice cm-fixed-notice notice-error">
                        <p><?php printf( __( '<b>Warning!</b> You\'re blocking external requests which means you won\'t be able to get %s updates. Please add %s to %s.' ), esc_html( $this->software_title ), '<strong>' . esc_html( $host ) . '</strong>', '<code>WP_ACCESSIBLE_HOSTS</code>' ); ?></p>
                    </div>
					<?php
				}
			}
		}

		// Provides text for api key section
		public function wc_am_api_key_text() {
		}

		// Returns the API Key status from the WooCommerce API Manager on the server
		public function wc_am_api_key_status() {
			if ( $this->get_api_key_status() ) {
				$license_status_check = esc_html__( 'Activated', $this->text_domain );
				ConsentMagic()->updateOptions( array(  $this->wc_am_activated_key => 'Activated' ) );
				ConsentMagic()->updateOptions( array( $this->wc_am_deactivate_checkbox_key => 'off' ) );
			} else {
				$license_status_check = esc_html__( 'Deactivated', $this->text_domain );
			}

			echo esc_attr( $license_status_check );
		}

		/**
		 * Returns the API Key status by querying the Status API function from the WooCommerce API Manager on the server.
		 * @return array|mixed|object
		 */
		public function license_key_status() {
			$status = $this->status();

			return !empty( $status ) ? json_decode( $this->status(), true ) : $status;
		}

		/**
		 * Returns true if the API Key status is Activated.
		 * @param bool $live Do not set to true if using to activate software. True is for live status checks after activation.
		 * @return bool
		 * @since 2.1
		 */
		public function get_api_key_status( $live = false ) {
			/**
			 * Real-time result.
			 * @since 2.5.1
			 */
			if ( $live ) {
				$license_status = $this->license_key_status();

				return !empty( $license_status ) && !empty( $license_status[ 'data' ][ 'activated' ] ) && $license_status[ 'data' ][ 'activated' ];
			}

			/**
			 * If $live === false.
			 * Stored result when first activating software.
			 */
			return ConsentMagic()->getOption( $this->wc_am_activated_key ) == 'Activated';
		}

		// Returns API Key text field
		public function wc_am_api_key_field() {
			if ( !empty( $this->data[ $this->wc_am_api_key_key ] ) ) {
				echo "<input id='api_key' name='" . esc_attr( $this->data_key ) . "[" . esc_attr( $this->wc_am_api_key_key ) . "]' size='25' type='text' value='" . esc_attr( $this->data[ $this->wc_am_api_key_key ] ) . "' />";
			} else {
				echo "<input id='api_key' name='" . esc_attr( $this->data_key ) . "[" . esc_attr( $this->wc_am_api_key_key ) . "]' size='25' type='text' value='' />";
			}
		}

		/**
		 * @since 2.3
		 */
		public function wc_am_product_id_field() {
			$product_id = ConsentMagic()->getOption( $this->wc_am_product_id );

			if ( !empty( $product_id ) ) {
				$this->product_id = $product_id;
			}

			if ( !empty( $product_id ) ) {
				echo "<input id='product_id' name='" . esc_attr( $this->wc_am_product_id ) . "' size='25' type='text' value='" . absint( $this->product_id ) . "' />";
			} else {
				echo "<input id='product_id' name='" . esc_attr( $this->wc_am_product_id ) . "' size='25' type='text' value='' />";
			}
		}

		/**
		 * Sanitizes and validates all input and output for Dashboard
		 * @since 2.0
		 */
		public function validate_options() {

			if ( !isset( $_POST[ 'wc_am_client_consent_magic_pro' ][ $this->wc_am_api_key_key ] ) ) {
				return;
			}

			$api_key = trim( sanitize_text_field( $_POST[ 'wc_am_client_consent_magic_pro' ][ $this->wc_am_api_key_key ] ) );

			// Load existing options, validate, and update with changes from input before returning
			if ( !$this->data ) {
				$this->data[ "wc_am_client_consent_magic_pro_api_key" ] = $api_key;
			}

			$options                             = $this->data;
			$options[ $this->wc_am_api_key_key ] = $api_key;
			$activation_status                   = ConsentMagic()->getOption( $this->wc_am_activated_key );
			$checkbox_status                     = ConsentMagic()->getOption( $this->wc_am_deactivate_checkbox_key );
			$current_api_key                     = $this->data[ $this->wc_am_api_key_key ];

            ConsentMagic()->updateOptions( array(  $this->data_key => $options ) );
			/**
			 * @since 2.3
			 */
			if ( $this->no_product_id ) {
				$new_product_id = ( isset( $_REQUEST[ $this->wc_am_product_id ] ) ) ? absint( sanitize_text_field( $_REQUEST[ $this->wc_am_product_id ] ) ) : '';

				if ( !empty( $new_product_id ) ) {
					ConsentMagic()->updateOptions( array(  $this->wc_am_product_id => $new_product_id ) );
					$this->product_id = $new_product_id;
				}
			}

            if ( $activation_status == 'Deactivated' || $activation_status == '' || $api_key == '' || $checkbox_status == 'on' || $current_api_key != $api_key ) {
                /**
                 * If this is a new key, and an existing key already exists in the database,
                 * try to deactivate the existing key before activating the new key.
                 */
                if ( !empty( $current_api_key ) && $current_api_key != $api_key ) {
                    $this->replace_license_key( $current_api_key );
                }

                $args = array(
                    'api_key' => $api_key,
                );

                $activation_result = $this->activate( $args );

                if ( !empty( $activation_result ) ) {
                    $activate_results = json_decode( $activation_result, true );

                    if ( $activate_results[ 'success' ] === true && $activate_results[ 'activated' ] === true ) {
                        $message = sprintf( __( '%s activated. ', $this->text_domain ), esc_attr( $this->software_title ) ) . esc_attr( "{$activate_results['message']}." );
                        add_settings_error( 'activate_text', 'activate_msg', esc_html( $message ), 'updated' );
                        ConsentMagic()->updateOptions( array(  $this->wc_am_activated_key => 'Activated' ) );
                        ConsentMagic()->updateOptions( array(  $this->wc_am_deactivate_checkbox_key => 'off' ) );
                        set_transient( 'cm_license_success_notice', esc_html( $message ), 30 );
                    }

                    if ( $activate_results == false && !empty( $this->data ) && !empty( $this->wc_am_activated_key ) ) {
                        $error = __( 'Connection failed to the License Key API server. Try again later. There may be a problem on your server preventing outgoing requests, or the store is blocking your request to activate the plugin/theme.', $this->text_domain );
                        add_settings_error( 'api_key_check_text', 'api_key_check_error', esc_html( $error ) );
                        ConsentMagic()->updateOptions( array( $this->wc_am_activated_key => 'Deactivated' ) );
                        $this->cs_db_cron_deactivate();
                        set_transient( 'cm_license_notice', esc_html( $error ), 30 );
                    }

                    if ( isset( $activate_results[ 'data' ][ 'error_code' ] ) && !empty( $this->data ) && !empty( $this->wc_am_activated_key ) ) {
                        add_settings_error( 'wc_am_client_error_text', 'wc_am_client_error', esc_html( $activate_results[ 'data' ][ 'error' ] ) );
                        ConsentMagic()->updateOptions( array( $this->wc_am_activated_key => 'Deactivated' ) );
                        set_transient( 'cm_license_notice', esc_html( $activate_results[ 'data' ][ 'error' ] ), 30 );
                    }
                }
            } // End Plugin Activation
		}

		// Deactivates the API Key to allow key to be used on another blog
		public function deactivation() {
			$activation_status = ConsentMagic()->getOption( $this->wc_am_activated_key );

			$args = array(
				'api_key' => $this->data[ $this->wc_am_api_key_key ],
			);

			if ( !empty( $this->data[ $this->wc_am_api_key_key ] ) && $activation_status == 'Activated' ) {
				// deactivates API Key activation
				$deactivation_result = $this->deactivate( $args );

				if ( !empty( $deactivation_result ) ) {
					$activate_results = json_decode( $deactivation_result, true );

					if ( $activate_results[ 'success' ] === true && $activate_results[ 'deactivated' ] === true ) {
						if ( !empty( $this->wc_am_activated_key ) ) {
							$message = __( sprintf( '%s deactivated is successfully.', $this->software_title ), 'consent-magic' ) . ' ' . esc_attr( "{$activate_results['activations_remaining']}." );
							ConsentMagic()->updateOptions( array( $this->wc_am_activated_key => 'Deactivated' ) );
							add_settings_error( 'wc_am_deactivate_text', 'deactivate_msg', esc_html( $message ), 'updated' );
							set_transient( 'cm_license_deactivation_notice', esc_html( $message ), 30 );
							$this->cs_db_cron_deactivate();
						}

						return;
					}

					if ( isset( $activate_results[ 'data' ][ 'error_code' ] ) && !empty( $this->data ) && !empty( $this->wc_am_activated_key ) ) {
						$error = $activate_results[ 'data' ][ 'error' ];
						add_settings_error( 'wc_am_client_error_text', 'wc_am_client_error', esc_html( $error ) );
						ConsentMagic()->updateOptions( array(  $this->wc_am_activated_key => 'Deactivated' ) );
						$this->cs_db_cron_deactivate();
						set_transient( 'cm_license_notice', esc_html( $error ), 30 );
					}
				}
			}
		}

		/**
		 * Deactivate the current API Key before activating the new API Key
		 * @param string $current_api_key
		 */
		public function replace_license_key( $current_api_key ) {
			$args = array(
				'api_key' => $current_api_key,
			);

			$this->deactivate( $args );
		}

		public function wc_am_deactivate_textarea() {
			echo '<input type="checkbox" id="' . esc_attr( $this->wc_am_deactivate_checkbox_key ) . '" name="' . esc_attr( $this->wc_am_deactivate_checkbox_key ) . '" value="on"';
			echo checked( ConsentMagic()->getOption( $this->wc_am_deactivate_checkbox_key ), 'on' );
			echo '/>';
			?>
            <span class="description"><?php esc_html_e( 'Deactivates an API Key so it can be used on another blog.', $this->text_domain ); ?></span>
			<?php
		}

		/**
		 * Builds the URL containing the API query string for activation, deactivation, and status requests.
		 * @param array $args
		 * @return string
		 */
		public function create_software_api_url( $args ) {
			return add_query_arg( 'wc-api', 'wc-am-api', $this->api_url ) . '&' . http_build_query( $args );
		}

		/**
		 * Sends the request to activate to the API Manager.
		 * @param array $args
		 * @return string
		 */
		public function activate( $args ) {
			if ( empty( $args ) ) {
				add_settings_error( 'not_activated_text', 'not_activated_error', esc_html__( 'The API Key is missing from the deactivation request.', $this->text_domain ), 'updated' );

				return '';
			}

			$defaults = array(
				'wc_am_action'     => 'activate',
				'product_id'       => $this->product_id,
				'instance'         => $this->wc_am_instance_id,
				'object'           => $this->wc_am_domain,
				'software_version' => $this->wc_am_software_version
			);

			$args       = wp_parse_args( $defaults, $args );
			$target_url = esc_url_raw( $this->create_software_api_url( $args ) );
			$request    = wp_safe_remote_post( $target_url );

			if ( is_wp_error( $request ) || wp_remote_retrieve_response_code( $request ) != 200 ) {

				$status_code = wp_remote_retrieve_response_code( $request );
				if ( $status_code == 403 || $status_code == 415 ) {
					$ip_list = getAllServerIps();
					$message = __(
						           "The request may have been blocked by our firewall. Please try again later. If the problem persists, contact our support and provide the following IP addresses: ",
						           'consent-magic'
					           ) . $ip_list;

					set_transient( 'cm_license_notice', $message, 60 );
					add_settings_error( 'not_activated_text', 'not_activated_error', $message, 'updated' );

					return '';
				}

				// Request failed
				$error = __( 'The API Key activation could not be completed due to an unknown error possibly on the store server. The activation results were empty.', $this->text_domain );
				add_settings_error( 'not_activated_empty_response_text', 'not_activated_empty_response_error', esc_html( $error ), 'updated' );
				set_transient( 'cm_license_notice', esc_html( $error ), 30 );
                return '';
			}

			return wp_remote_retrieve_body( $request );
		}

		/**
		 * Sends the request to deactivate to the API Manager.
		 * @param array $args
		 * @return string
		 */
		public function deactivate( $args ) {
			if ( empty( $args ) ) {
				add_settings_error( 'not_deactivated_text', 'not_deactivated_error', esc_html__( 'The API Key is missing from the deactivation request.', $this->text_domain ), 'updated' );

				return '';
			}

			$defaults = array(
				'wc_am_action' => 'deactivate',
				'product_id'   => $this->product_id,
				'instance'     => $this->wc_am_instance_id,
				'object'       => $this->wc_am_domain
			);

			$args       = wp_parse_args( $defaults, $args );
			$target_url = esc_url_raw( $this->create_software_api_url( $args ) );
			$request    = wp_safe_remote_post( $target_url );

			if ( is_wp_error( $request ) || wp_remote_retrieve_response_code( $request ) != 200 ) {
				$status_code = wp_remote_retrieve_response_code( $request );
				if ( $status_code == 403 || $status_code == 415 ) {
					$ip_list = getAllServerIps();
					$message = __(
						           "The request may have been blocked by our firewall. Please try again later. If the problem persists, contact our support and provide the following IP addresses: ",
						           'consent-magic'
					           ) . $ip_list;

					set_transient( 'cm_license_notice', $message, 60 );

					return '';
				}


				// Request failed
				$error = __( 'The API Key activation could not be completed due to an unknown error possibly on the store server The activation results were empty.', $this->text_domain );
				add_settings_error( 'not_deactivated_empty_response_text', 'not_deactivated_empty_response_error', esc_html( $error ), 'updated' );
				set_transient( 'cm_license_notice', esc_html( $error ), 30 );
				return '';
			}

			return wp_remote_retrieve_body( $request );
		}

		/**
		 * Sends the status check request to the API Manager.
		 * @return bool|string
		 */
		public function status() {
			if ( empty( $this->data[ $this->wc_am_api_key_key ] ) ) {
				return '';
			}

			$defaults = array(
				'wc_am_action' => 'status',
				'api_key'      => $this->data[ $this->wc_am_api_key_key ],
				'product_id'   => $this->product_id,
				'instance'     => $this->wc_am_instance_id,
				'object'       => $this->wc_am_domain
			);

			$target_url = esc_url_raw( $this->create_software_api_url( $defaults ) );
			$request    = wp_safe_remote_post( $target_url );

			if ( is_wp_error( $request ) || wp_remote_retrieve_response_code( $request ) != 200 ) {
				$status_code = wp_remote_retrieve_response_code( $request );
				if ( $status_code == 403 || $status_code == 415 ) {
					$ip_list = getAllServerIps();
					$message = __(
						           "The request may have been blocked by our firewall. Please try again later. If the problem persists, contact our support and provide the following IP addresses: ",
						           'consent-magic'
					           ) . $ip_list;

					set_transient( 'cm_license_notice', $message, 60 );

					return '';
				}

				// Request failed
				return '';
			}

			return wp_remote_retrieve_body( $request );
		}

		/**
		 * Check for software updates.
		 */
		public function check_for_update() {
			$this->plugin_name = $this->wc_am_plugin_name;

			// Slug should be the same as the plugin/theme directory name
			if ( strpos( $this->plugin_name, '.php' ) !== 0 ) {
				$this->slug = dirname( $this->plugin_name );
			} else {
				$this->slug = $this->plugin_name;
			}

			/*********************************************************************
			 * The plugin and theme filters should not be active at the same time
			 *********************************************************************/ /**
			 * More info:
			 * function set_site_transient moved from wp-includes/functions.php
			 * to wp-includes/option.php in WordPress 3.4
			 * set_site_transient() contains the pre_set_site_transient_{$transient} filter
			 * {$transient} is either update_plugins or update_themes
			 * Transient data for plugins and themes exist in the Options table:
			 * _site_transient_update_themes
			 * _site_transient_update_plugins
			 */

			// uses the flag above to determine if this is a plugin or a theme update request
			if ( $this->plugin_or_theme == 'plugin' ) {
				/**
				 * Plugin Updates
				 */
				add_filter( 'pre_set_site_transient_update_plugins', array(
					$this,
					'update_check'
				) );
				// Check For Plugin Information to display on the update details page
				add_filter( 'plugins_api', array(
					$this,
					'information_request'
				), 10, 3 );
			}
		}

		/**
		 * Sends and receives data to and from the server API
		 * @param array $args
		 * @return bool|string $response
		 * @since  2.0
		 */
		public function send_query( $args ) {
			$target_url = esc_url_raw( add_query_arg( 'wc-api', 'wc-am-api', $this->api_url ) . '&' . http_build_query( $args ) );
			$request    = wp_safe_remote_post( $target_url );

			if ( is_wp_error( $request ) || wp_remote_retrieve_response_code( $request ) != 200 ) {
				$status_code = wp_remote_retrieve_response_code( $request );
				if ( $status_code == 403 || $status_code == 415 ) {
					$ip_list = getAllServerIps();
					$message = __(
						           "The request may have been blocked by our firewall. Please try again later. If the problem persists, contact our support and provide the following IP addresses: ",
						           'consent-magic'
					           ) . $ip_list;

					set_transient( 'cm_license_notice', $message, 60 );
				}

				return false;
			}

			$response = wp_remote_retrieve_body( $request );

			return !empty( $response ) ? $response : false;
		}

		/**
		 * Check for updates against the remote server.
		 * @param object $transient
		 * @return object $transient
		 * @since  2.0
		 */
		public function update_check( $transient ) {
			global $pagenow;

			if ( empty( $transient->checked ) ) {
				return $transient;
			}

			if ( 'plugins.php' == $pagenow && is_multisite() ) {
				return $transient;
			}

			if ( !empty( $transient->response ) && !empty( $transient->response[ $this->plugin_name ] ) ) {
				return $transient;
			}

			$args = array(
				'wc_am_action' => 'update',
				'slug'         => $this->slug,
				'plugin_name'  => $this->plugin_name,
				'version'      => $this->wc_am_software_version,
				'product_id'   => $this->product_id,
				'api_key'      => !empty( $this->data[ $this->wc_am_api_key_key ] ) ? $this->data[ $this->wc_am_api_key_key ] : '',
				'instance'     => $this->wc_am_instance_id,
			);

			// Check for a plugin update
			$response = json_decode( $this->send_query( $args ), true );
			// Displays an admin error message in the WordPress dashboard

			if ( isset( $response[ 'data' ][ 'error_code' ] ) ) {
				$error = esc_html( $response[ 'data' ][ 'error' ] );
				add_settings_error( 'wc_am_client_error_text', 'wc_am_client_error', $error );
				set_transient( 'cm_license_notice', $error, 30 );
			}

			if ( $response !== false && $response[ 'success' ] === true ) {
				// New plugin version from the API
				$new_ver = (string) $response[ 'data' ][ 'package' ][ 'new_version' ];
				// Current installed plugin version
				$curr_ver = (string) $this->wc_am_software_version;

				$package = array(
					'id'             => $response[ 'data' ][ 'package' ][ 'id' ],
					'slug'           => $response[ 'data' ][ 'package' ][ 'slug' ],
					'plugin'         => $response[ 'data' ][ 'package' ][ 'plugin' ],
					'new_version'    => $response[ 'data' ][ 'package' ][ 'new_version' ],
					'url'            => $response[ 'data' ][ 'package' ][ 'url' ],
					'tested'         => $response[ 'data' ][ 'package' ][ 'tested' ],
					'package'        => $response[ 'data' ][ 'package' ][ 'package' ],
					'upgrade_notice' => $response[ 'data' ][ 'package' ][ 'upgrade_notice' ],
				);

				if ( isset( $new_ver ) && isset( $curr_ver ) ) {
					if ( version_compare( $new_ver, $curr_ver, '>' ) ) {
						$transient->response[ $this->plugin_name ] = (object) $package;
						unset( $transient->no_update[ $this->plugin_name ] );
					}
				}
			}

			return $transient;
		}

		/**
		 * API request for informatin.
		 * If `$action` is 'query_plugins' or 'plugin_information', an object MUST be passed.
		 * If `$action` is 'hot_tags` or 'hot_categories', an array should be passed.
		 * @param false|object|array $result The result object or array. Default false.
		 * @param string             $action The type of information being requested from the Plugin Install API.
		 * @param object             $args
		 * @return object
		 */
		public function information_request( $result, $action, $args ) {
			// Check if this plugins API is about this plugin
			if ( isset( $args->slug ) ) {
				if ( $args->slug != $this->slug ) {
					return $result;
				}
			} else {
				return $result;
			}

			$args = array(
				'wc_am_action' => 'plugininformation',
				'plugin_name'  => $this->plugin_name,
				'version'      => $this->wc_am_software_version,
				'product_id'   => $this->product_id,
				'api_key'      => !empty( $this->data[ $this->wc_am_api_key_key ] ) ? $this->data[ $this->wc_am_api_key_key ] : '',
				'instance'     => $this->wc_am_instance_id,
				'object'       => $this->wc_am_domain,
			);

			$response = unserialize( $this->send_query( $args ) );

			if ( isset( $response ) && is_object( $response ) && $response !== false ) {
				return $response;
			}

			return $result;
		}

		function generate_token( $length = 8 ) {
			$characters       = 'abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ1234567890';
			$charactersLength = strlen( $characters );
			$randomString     = '';
			for ( $i = 0; $i < $length; $i++ ) {
				$randomString .= $characters[ random_int( 0, $charactersLength - 1 ) ];
			}

			return $randomString;
		}

	}
}