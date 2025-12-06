<?php

namespace ConsentMagicPro;

if ( !defined( 'ABSPATH' ) ) {
	exit; // Exit if accessed directly.
}

class CS_AdminPageGeolocation extends CS_AdminPage implements CS_Page {

	public function __construct( $plugin_name, $page_slug, $admin = null ) {
		parent::__construct( $plugin_name, $page_slug, $admin );

		add_action( 'wp_ajax_cs_check_geo_license', array(
			$this,
			'cs_check_geo_license'
		), 200 );
	}

	public function renderPage() {

		$this->manageCSPermissions();

		$this->updateAdminSettingsHandle();

		if ( ConsentMagic()->getOption( 'cs_geolocation' ) && ConsentMagic()->getOption( 'cs_geo_activated' ) && ConsentMagic()->getOption( 'cs_often_update' ) != 'never' ) {
			db_cron_update();
		} else {
			db_cron_deactivate();
		}

		$this->renderHTML();
	}

	public function renderTitle() {
		add_action( 'init', function() {
			$this->page_title = __( 'Geolocation', 'consent-magic' );
		} );
	}

	private function updateAdminSettingsHandle() {
		if ( isset( $_POST[ 'cs_update_action' ] ) && $_POST[ 'cs_update_action' ] == 'update_admin_settings_form' ) {

			// Check nonce:
			check_admin_referer( 'cs-update-' . CMPRO_SETTINGS_FIELD );

			if ( isset( $_POST[ 'cs' ][ $this->plugin_name ][ 'cs_geolocation' ] ) ) {
				ConsentMagic()->updateOptions( array( 'cs_geolocation' => sanitize_text_field( $_POST[ 'cs' ][ $this->plugin_name ][ 'cs_geolocation' ] ) ) );
			}

			if ( isset( $_POST[ 'cs' ][ $this->plugin_name ][ 'cs_often_update' ] ) ) {
				$old_cs_often_update = ConsentMagic()->getOption( 'cs_often_update' );
				if ( $old_cs_often_update != $_POST[ 'cs' ][ $this->plugin_name ][ 'cs_often_update' ] ) {
					ConsentMagic()->updateOptions( array( 'cs_often_update' => sanitize_text_field( $_POST[ 'cs' ][ $this->plugin_name ][ 'cs_often_update' ] ) ) );
					if ( $old_cs_often_update == 'never' ) {
						db_cron_change_recurrence();
					} else {
						db_cron_change_recurrence( true );
					}
				}
			}

			$this->renderSuccessMessage();
		}
	}

	function cs_check_geo_license() {

		// Check nonce:
		check_admin_referer( 'cs-update-' . CMPRO_SETTINGS_FIELD );
		$this->manageCSPermissions();

		if ( isset( $_POST[ 'license_action' ] ) ) {

			if ( $_POST[ 'license_action' ] == 1 ) {
				if ( !empty( $_POST[ 'key' ] ) && !empty( $_POST[ 'account_id' ] ) ) {
					$cs_geo     = new CS_Geoip;
					$key        = sanitize_text_field( $_POST[ 'key' ] );
					$account_id = sanitize_text_field( $_POST[ 'account_id' ] );
					ConsentMagic()->updateOptions( array( 'cs_geo_licence_key' => sanitize_text_field( $key ) ) );
					ConsentMagic()->updateOptions( array( 'cs_geo_licence_account_id' => sanitize_text_field( $_POST[ 'account_id' ] ) ) );
					$validate = $cs_geo->cs_validate_geo_license_key( $key, $account_id );

					if ( is_wp_error( $validate ) ) {
						ConsentMagic()->updateOptions( array( 'cs_geo_activated' => false ) );
						$message = $validate->get_error_message( 'geo_error' );
						$status  = false;
					} else {
						ConsentMagic()->updateOptions( array( 'cs_geo_activated' => true ) );
						$message = esc_html__( 'License key activated.' );
						$status  = true;
					}

					if ( $status ) {
						db_cron_update_one_time( true );
					} else {
						db_cron_deactivate();
					}
				} else {
					$message = esc_html__( 'License key or Account ID is empty.', 'consent-magic' );
					$status  = false;
				}
			} else {
				ConsentMagic()->updateOptions( array( 'cs_geo_activated' => false ) );
				ConsentMagic()->updateOptions( array( 'cs_geo_licence_key' => '' ) );
				ConsentMagic()->updateOptions( array( 'cs_geo_licence_account_id' => '' ) );
				$message = esc_html__( 'License key deactivated.' );
				db_cron_deactivate();
				$status = false;
			}
			$license_action = sanitize_text_field( $_POST[ 'license_action' ] );

		} else {
			$message        = esc_html__( 'An error occurred, please try again.' );
			$status         = false;
			$license_action = '1';
		}

		wp_send_json_success( array(
			'status'         => $status,
			'message'        => $message,
			'license_action' => $license_action,
		) );
		wp_die();
	}
}