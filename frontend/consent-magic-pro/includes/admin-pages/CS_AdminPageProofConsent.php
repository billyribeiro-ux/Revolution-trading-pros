<?php

namespace ConsentMagicPro;

if ( !defined( 'ABSPATH' ) ) {
	exit; // Exit if accessed directly.
}

class CS_AdminPageProofConsent extends CS_AdminPage implements CS_Page {

	public function renderPage() {

		$this->manageCSPermissions();
		$this->updateAdminSettingsHandle();
		$this->deleteProofConsentHandle();

		$this->renderHTML();
	}

	public function renderTitle() {
		add_action( 'init', function() {
			$this->page_title = __( 'Consent Proof', 'consent-magic' );
		} );
	}

	private function updateAdminSettingsHandle() {
		if ( isset( $_POST[ 'cs_update_action' ] ) && $_POST[ 'cs_update_action' ] == 'update_admin_settings_form' ) {
			// Check nonce:
			check_admin_referer( 'cs-update-' . CMPRO_SETTINGS_FIELD );

			$the_options = ConsentMagic()->getCSOptions();

			foreach ( $the_options as $key => $value ) {
				if ( isset( $_POST[ 'cs' ][ $this->plugin_name ][ $key ] ) ) {
					switch ( $key ) {
						case 'cs_send_proof_deleting_email':
							$email = sanitize_email( $_POST[ 'cs' ][ $this->plugin_name ][ $key ] );
							if ( !empty( $email ) ) {
								ConsentMagic()->updateOptions( array( $key => $email ) );
							}
							break;
						default:
							ConsentMagic()->updateOptions( array( $key => sanitize_text_field( $_POST[ 'cs' ][ $this->plugin_name ][ $key ] ) ) );
							break;
					}
				}
			}

			$this->renderSuccessMessage();
		}
	}

	private function deleteProofConsentHandle() {
		if ( isset( $_POST[ 'cm-delete-proof-consent' ] ) && $_POST[ 'cm-delete-proof-consent' ] == '1' ) {
			// Check nonce:
			check_admin_referer( 'cs-update-' . CMPRO_SETTINGS_FIELD );

			global $wpdb;
			$table  = $wpdb->prefix . 'cs_proof_consent';
			$delete = $wpdb->query( $wpdb->prepare( 'DELETE FROM %1$s', $table ) );
			$this->admin->renew_consent_run();
			$this->renderSuccessMessage();
		}
	}
}