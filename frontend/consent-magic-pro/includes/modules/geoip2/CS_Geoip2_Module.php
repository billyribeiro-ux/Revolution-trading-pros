<?php

namespace ConsentMagicPro;

// If this file is called directly, abort.
if ( !defined( 'WPINC' ) ) {
	die;
}

//`Coming Soon Page & Maintenance Mode by SeedProd` is active then disable script blocker
if ( class_exists( 'SEED_CSP4' ) ) {
	$seed_csp4_option = get_option( 'seed_csp4_settings_content' );
	if ( $seed_csp4_option && $seed_csp4_option[ 'status' ] > 0 ) {
		return;
	}
}

class CS_Geoip2_Module {

	public function __construct() {

		$this->cs_frontend_module();

		/* creating necessary table for script blocker  */
		register_activation_hook( CMPRO_PLUGIN_FILENAME, array(
			__CLASS__,
			'cs_activator'
		) );
	}

	public static function cs_activator() {
		global $wpdb;

		require_once ABSPATH . 'wp-admin/includes/upgrade.php';

		if ( is_multisite() ) {
			// Get all blogs in the network and activate plugin on each one
			$blog_ids = $wpdb->get_col( $wpdb->prepare( 'SELECT blog_id FROM %1$s', $wpdb->blogs ) );
			foreach ( $blog_ids as $blog_id ) {
				switch_to_blog( $blog_id );
				restore_current_blog();
			}
		}
	}

	public function cs_frontend_module() {
		require_once plugin_dir_path( __FILE__ ) . 'CS_Geoip.php';
	}

}

new CS_Geoip2_Module();