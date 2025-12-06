<?php

namespace ConsentMagicPro;

if ( !defined( 'ABSPATH' ) ) {
	exit; // Exit if accessed directly.
}

/**
 * CS_Url_Passthrough_Mode
 * URL Passthrough Mode for PYS
 */
class CS_Url_Passthrough_Mode {

	private static $_instance = null;

	public $loader;

	private $url_passthrough = false;

	/**
	 * $_instance CS_Url_Passthrough_Mode
	 * @return CS_Url_Passthrough_Mode|null
	 */
	public static function instance() {
		if ( is_null( self::$_instance ) ) {
			self::$_instance = new self();
		}

		return self::$_instance;
	}

	private function __construct() {
		if ( is_plugin_activated() ) {
			$this->loader = new CS_Loader();
			$this->loader->add_action( 'init', $this, 'add_url_passthrough_mode', 2 );
			$this->loader->run();
		}
	}

	/**
	 * Add google url_passthrough mode
	 * @return void
	 */
	public function add_url_passthrough_mode() {

		$active_rule_name = ConsentMagic()->get_active_rule_name();
		if ( ConsentMagic()->ready_to_run() && !empty( $active_rule_name ) ) {

			$url_passthrough        = false;
			$url_passthrough_ignore = false;
			$category               = get_term_by( 'slug', ConsentMagic()->getOption( 'cs_block_url_passthrough_scripts_cat' ), 'cs-cookies-category' );

			// check if category is enabled
			if ( $category ) {
				$category_id = $category->term_id;
				if ( $active_rule_name == 'iab' ) {
					$category_id = CS_IAB_Integration()->check_enabled_purpose_by_category( $category_id ) ? $category_id : 0;
				}
				$url_passthrough        = !CS_Google_Consent_Mode()->get_advanced_consent_mode_value( $category_id )[ 'status' ];
				$url_passthrough_ignore = get_term_meta( $category_id, 'cs_ignore_this_category', true ) == 1;
			}

			$blocking_enabled      = ConsentMagic::$check_disable_cm && ConsentMagic()->getOption( 'cs_block_pre_defined_scripts' ) && ConsentMagic()->getOption( 'cs_plugin_activation' ) && ConsentMagic()->getOption( 'cs_script_blocking_enabled' );
			$this->url_passthrough = $blocking_enabled && !$url_passthrough_ignore && ConsentMagic()->getOption( 'cs_block_url_passthrough_scripts' );

		} else {
			$this->url_passthrough = false;
			$url_passthrough       = false;
		}

		add_filter( 'cm_url_passthrough_mode', fn () => array(
			'enabled' => $this->url_passthrough,
			'value'   => $url_passthrough,
		) );
	}
}

/**
 * @return CS_Url_Passthrough_Mode
 */
function CS_Url_Passthrough_Mode(): CS_Url_Passthrough_Mode {
	return CS_Url_Passthrough_Mode::instance();
}