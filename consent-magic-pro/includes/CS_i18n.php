<?php

/**
 * Define the internationalization functionality.
 *
 * @link       https://www.pixelyoursite.com/plugins/consentmagic/
 * @since      1.0.0
 *
 * Loads and defines the internationalization files for this plugin
 * so that it is ready for translation.
 *
 * @package    ConsentMagic
 * @subpackage ConsentMagic/includes
 */

namespace ConsentMagicPro;

class CS_i18n {

	/**
	 * Load the plugin text domain for translation.
	 *
	 * @since    1.0.0
	 */
	public function load_plugin_textdomain() {

		load_plugin_textdomain( 'consent-magic', false, dirname( dirname( plugin_basename( __FILE__ ) ) ) . '/languages/' );
	}
}
