<?php

namespace ConsentMagicPro;

if ( !defined( 'ABSPATH' ) ) {
	exit; // Exit if accessed directly.
}

interface CS_Page {

	public function __construct( $plugin_name, $page_slug, $admin = null );

	public function renderPage();

	public function renderTitle();

	public function renderSuccessMessage();

	public function manageCSPermissions();

}