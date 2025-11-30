<?php

namespace ConsentMagicPro;

if ( !defined( 'ABSPATH' ) ) {
	exit; // Exit if accessed directly.
}

class CS_AdminPageProofStatistics extends CS_AdminPage implements CS_Page {

	public function renderPage() {
		$this->manageCSPermissions();
		$this->renderHTML();
	}

	public function renderTitle() {
		add_action( 'init', function() {
			$this->page_title = __( 'Statistics', 'consent-magic' );
		} );
	}
}