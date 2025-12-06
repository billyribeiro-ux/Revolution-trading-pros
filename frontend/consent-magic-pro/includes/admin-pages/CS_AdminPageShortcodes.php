<?php

namespace ConsentMagicPro;

if ( !defined( 'ABSPATH' ) ) {
	exit; // Exit if accessed directly.
}

class CS_AdminPageShortcodes extends CS_AdminPage implements CS_Page {

	public function renderPage() {
		$this->manageCSPermissions();

		if ( isset( $_REQUEST[ '_wpnonce' ] ) && wp_verify_nonce( $_REQUEST[ '_wpnonce' ], 'cs-update-' . CMPRO_SETTINGS_FIELD ) && isset( $_POST[ 'cs_update_action' ] ) && $_POST[ 'cs_update_action' ] == 'update_admin_settings_form' ) {
			$the_options = ConsentMagic()->getCSOptions();
			foreach ( $the_options as $key => $value ) {
				if ( isset( $_POST[ 'cs' ][ $this->plugin_name ][ $key ] ) ) {
					ConsentMagic()->updateOptions( array( $key => sanitize_text_field( $_POST[ 'cs' ][ $this->plugin_name ][ $key ] ) ) );
				}
			}

			if ( !empty( $_SERVER[ 'HTTP_X_REQUESTED_WITH' ] ) && strtolower( sanitize_text_field( $_SERVER[ 'HTTP_X_REQUESTED_WITH' ] ) ) == 'xmlhttprequest' ) {
				exit();
			}
		}

		$this->renderHTML();
	}

	public function renderTitle() {
		add_action( 'init', function() {
			$this->page_title = __( 'Shortcodes & Filters', 'consent-magic' );
		} );
	}
}