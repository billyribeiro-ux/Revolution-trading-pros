<?php

namespace ConsentMagicPro;

if ( !defined( 'ABSPATH' ) ) {
	exit; // Exit if accessed directly.
}

class CS_AdminPageLicense extends CS_AdminPage implements CS_Page {

	public function renderPage() {
		$this->manageCSPermissions();
		settings_errors();
		$this->renderHTML();
	}

	public function renderTitle() {
		add_action( 'init', function() {
			$this->page_title = __( 'License', 'consent-magic' );
		} );
	}
}