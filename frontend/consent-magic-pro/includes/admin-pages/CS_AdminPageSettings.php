<?php

namespace ConsentMagicPro;

if ( !defined( 'ABSPATH' ) ) {
	exit; // Exit if accessed directly.
}

class CS_AdminPageSettings extends CS_AdminPage implements CS_Page {

	public function renderPage() {

		$this->manageCSPermissions();

		$this->disableTestModeHandle();
		$this->clearTestConsentHandle();
		$this->updateAdminSettingsHandle();

		if ( isset( $_POST[ 'cm-restore-default-translations' ] ) && $_POST[ 'cm-restore-default-translations' ] == 1 ) {
			// Check nonce:
			check_admin_referer( 'cs-update-' . CMPRO_SETTINGS_FIELD );
			ConsentMagic()->loadDefaultTranslations();
			$this->renderSuccessMessage();
		}

		if ( isset( $_POST[ 'cm-renew-consent' ] ) && $_POST[ 'cm-renew-consent' ] == 1 ) {
			// Check nonce:
			check_admin_referer( 'cs-update-' . CMPRO_SETTINGS_FIELD );
			$this->admin->renew_consent_run();
		}

		$this->restartSetupHandle();
		$this->renderHTML();
	}

	public function renderTitle() {
		add_action( 'init', function() {
			$this->page_title = __( 'Settings', 'consent-magic' );
		} );
	}

	private function disableTestModeHandle() {
		// Get options:
		$the_options = ConsentMagic()->getCSOptions();

		if ( isset( $_GET[ 'cs_update_action' ] ) && $_GET[ 'cs_update_action' ] == 'cs_test_mode_disable' ) {
			// Check nonce:
			check_admin_referer( 'cs-update-' . CMPRO_SETTINGS_FIELD );

			ConsentMagic()->updateOptions( array( 'cs_test_mode' => false ) );
			$this->renderSuccessMessage();

			wp_redirect( remove_query_arg( 'cs_update_action', false ) );
		}
	}

	private function clearTestConsentHandle() {
		if ( isset( $_GET[ 'cs_update_action' ] ) && $_GET[ 'cs_update_action' ] == 'cs_test_consent_clear' ) {
			// Check nonce:
			check_admin_referer( 'cs-update-' . CMPRO_SETTINGS_FIELD );
			$consent_version_test = ConsentMagic()->cs_get_consent_version_test();
			if ( !empty( $consent_version_test ) ) {
				$consent_version_test = $consent_version_test + 1;
				ConsentMagic()->updateOptions( array( 'cs_consent_version_test' => $consent_version_test ) );
			}

			$this->renderSuccessMessage();

			wp_redirect( remove_query_arg( 'cs_update_action', false ) );
		}
	}

	private function updateAdminSettingsHandle() {
		if ( isset( $_POST[ 'cs_update_action' ] ) && $_POST[ 'cs_update_action' ] == 'update_admin_settings_form' ) {
			check_admin_referer( 'cs-update-' . CMPRO_SETTINGS_FIELD );

			$post_data = $_POST[ 'cs' ][ $this->plugin_name ];
			array_walk( $post_data, array(
				ConsentMagic(),
				'cs_sanitize_array'
			) );

			$permissions      = ( isset( $post_data[ 'cs_admin_permissions' ] ) && is_array( $post_data[ 'cs_admin_permissions' ] ) ) ? $post_data[ 'cs_admin_permissions' ] : array();
			$permissions[]    = 'administrator';
			$permissions_save = array();
			foreach ( $permissions as $permission ) {
				if ( !empty( $permission ) ) {
					$permissions_save[] = $permission;
				}
			}

			$ignore_users      = ( isset( $post_data[ 'cs_admin_ignore_users' ] ) && is_array( $post_data[ 'cs_admin_ignore_users' ] ) ) ? $post_data[ 'cs_admin_ignore_users' ] : array();
			$ignore_users_save = array();
			foreach ( $ignore_users as $ignore_user ) {
				if ( !empty( $ignore_user ) ) {
					$ignore_users_save[] = $ignore_user;
				}
			}

			$ignore_users_ip = isset( $post_data[ 'ignore_users_ip' ] ) ? sanitize_text_field( $post_data[ 'ignore_users_ip' ] ) : '';
			$ignore_users_ip = explode( ",", $ignore_users_ip );

			$emails      = ( isset( $post_data[ 'cs_send_important_emails' ] ) && is_array( $post_data[ 'cs_send_important_emails' ] ) ) ? $post_data[ 'cs_send_important_emails' ] : array();
			$emails_save = array();

			if ( !empty( $emails ) ) {
				foreach ( $emails as $email ) {
					if ( is_email( $email ) ) {
						$emails_save[] = $email;
					}
				}
			}

			if ( isset( $post_data[ 'text' ] ) ) {
				foreach ( $post_data[ 'text' ] as $option => $languages ) {
					ConsentMagic()->clearLangOptions( $option );
					$update_langs = array();
					foreach ( $languages as $key_lang => $language ) {
						$description = wp_kses_post( wp_unslash( $_POST[ 'cs' ][ $this->plugin_name ][ 'text' ][ $option ][ $key_lang ] ) );
						if ( !empty( $description ) ) {
							ConsentMagic()->updateLangOptions( $option, $description, $key_lang );
							$update_langs[] = $key_lang;
						}
					}

					ConsentMagic()->update_language_availability( $update_langs );
				}
			}

			// Get options:
			$the_options = ConsentMagic()->getCSOptions();

			foreach ( $the_options as $key => $value ) {
				if ( $key == 'cs_admin_permissions' ) {
					ConsentMagic()->updateOptions( array( 'cs_admin_permissions' => $permissions_save ) );
				} else if ( $key == 'cs_send_important_emails' ) {
					ConsentMagic()->updateOptions( array( 'cs_send_important_emails' => $emails_save ) );
				} else if ( $key == 'cs_admin_ignore_users' ) {
					ConsentMagic()->updateOptions( array( 'cs_admin_ignore_users' => $ignore_users_save ) );
				} else if ( $key == 'cs_admin_ignore_users_ip' ) {
					ConsentMagic()->updateOptions( array( 'cs_admin_ignore_users_ip' => $ignore_users_ip ) );
				} else if ( $key == 'cs_scanner_module' && isset( $post_data[ $key ] ) ) {
					$csmart_admin_modules = ConsentMagic()->getOption( 'csmart_admin_modules' );
					if ( $csmart_admin_modules === false ) {
						$csmart_admin_modules = array();
					}
					$csmart_admin_modules[ 'scanner' ] = ( $post_data[ 'cs_scanner_module' ] == true ) ? 1 : 0;
					ConsentMagic()->updateOptions( array( 'csmart_admin_modules' => $csmart_admin_modules ) );
				} else if ( $key == 'cs_cross_domain_tracking_domain' ) {
					$needle = $post_data[ $key ] ?? null;
					$needle = cs_sanitize_url( $needle );
					ConsentMagic()->updateOptions( array( 'cs_cross_domain_tracking_domain' => $needle ) );
				} else if ( $key == 'cs_language_availability' ) {
					$income_langs = $post_data[ $key ];
					if ( is_array( $income_langs ) ) {
						array_walk( $income_langs, array(
							ConsentMagic(),
							'cs_sanitize_array'
						) );

						$language_availability = ConsentMagic()->getOption( 'cs_language_availability' );
						foreach ( $income_langs as $k => $lang ) {
							if ( $k != CMPRO_DEFAULT_LANGUAGE ) {
								$language_availability[ $k ] = $lang;
							}
						}
						ksort( $language_availability );
						ConsentMagic()->updateOptions( array( 'cs_language_availability' => $language_availability ) );
					}
				} else {
					if ( isset( $post_data[ $key ] ) ) {
						ConsentMagic()->updateOptions( array( $key => $post_data[ $key ] ) );
					}
				}
			}

			$this->renderSuccessMessage();
		}
	}

	private function restartSetupHandle() {
		//Restart Setup Process
		if ( isset( $_POST[ 'cm-restart-setup' ] ) && $_POST[ 'cm-restart-setup' ] == 1 ) {
			// Check nonce:
			check_admin_referer( 'cs-update-' . CMPRO_SETTINGS_FIELD );

			if ( ConsentMagic()->getOption( 'cs_deactivation_db_clear' ) == 1 ) {
				require_once CMPRO_PLUGIN_PATH . 'includes/CS_Uninstall.php';
				require_once CMPRO_PLUGIN_PATH . 'includes/CS_Activator.php';

				CS_Uninstall::deactivation( false );

				ConsentMagic()->check_tables();
				( new CS_Activator )->activate();

				//install tables
				require_once CMPRO_PLUGIN_PATH . '/includes/modules/scanner/CS_Scanner_Module.php';
				$scanner_module = new CS_Scanner_Module;
				$scanner_module->install_tables();

				$this->admin->create_taxonomy();
				$this->admin->cs_insert_templates_lists();
			} else {

				$id = get_post_id_by_slug( 'cs_gdpr_rule' );
				wp_delete_post( $id, true );
				$id = get_post_id_by_slug( 'cs_ldu_rule' );
				wp_delete_post( $id, true );
				$id = get_post_id_by_slug( 'cs_rest_of_world_rule' );
				wp_delete_post( $id, true );
				$id = get_post_id_by_slug( 'cs_iab_rule' );
				wp_delete_post( $id, true );

				ConsentMagic()->deleteOption( 'cs_policy_existing_page' );
				ConsentMagic()->deleteOption( 'cs_script_blocking_enabled' );
				ConsentMagic()->updateOptions( array( 'default_font' => '' ) );
				ConsentMagic()->updateOptions( array( 'cs_check_flow' => false ) );
			}

			$check_settings = array(
				'cs_check_settings' ,
				'cs_check_translations',
				'cs_check_script_cat_settings',
				'cs_check_rule_settings',
				'cs_check_design',
			);
			foreach ( $check_settings as $option ) {
				ConsentMagic()->deleteOption( $option );
			}

			$this->admin->cs_insert_templates_lists();
			$this->admin->cs_insert_cookie_lists();

			$this->renderSuccessMessage();
		}
	}
}