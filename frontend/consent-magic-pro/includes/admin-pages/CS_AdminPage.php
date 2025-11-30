<?php

namespace ConsentMagicPro;

if ( !defined( 'ABSPATH' ) ) {
	exit; // Exit if accessed directly.
}

abstract class CS_AdminPage {

	public $plugin_name;

	public $page_slug = '';

	public $page_title = '';

	protected $admin;

	public function __construct( $plugin_name, $page_slug, $admin = null ) {
		$this->plugin_name = $plugin_name;
		$this->page_slug   = $page_slug;
		if ( $admin ) {
			$this->admin = $admin;
		}
		$this->renderTitle();
	}

	public function renderHTML() {
		include CMPRO_PLUGIN_VIEWS_PATH . 'admin/html-general.php';
	}

	public function renderSuccessMessage() {
		if ( !empty( $_SERVER[ 'HTTP_X_REQUESTED_WITH' ] ) && strtolower( sanitize_text_field( $_SERVER[ 'HTTP_X_REQUESTED_WITH' ] ) ) == 'xmlhttprequest' ) {
			exit();
		}
	}

	public function manageCSPermissions() {
		// Lock out non-admins:
		if ( !current_user_can( 'manage_cs' ) ) {
			wp_die( esc_html__( 'You do not have sufficient permission to perform this operation', 'consent-magic' ) );
		}
	}
}