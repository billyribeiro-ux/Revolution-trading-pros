<?php

namespace ConsentMagicPro;

if ( !defined( 'ABSPATH' ) ) {
	exit; // Exit if accessed directly.
}

/**
 * CS_Conversion_Exporter
 * Conversion Exporter module
 */
class CS_Conversion_Exporter {

	private static ?CS_Conversion_Exporter $_instance = null;

	public CS_Loader $loader;

	/**
	 * $_instance CS_Conversion_Exporter
	 * @return CS_Conversion_Exporter|null
	 */
	public static function instance(): ?CS_Conversion_Exporter {
		if ( is_null( self::$_instance ) ) {
			self::$_instance = new self();
		}

		return self::$_instance;
	}

	private function __construct() {
		if ( is_plugin_activated() && isConversionExporterActivated() ) {
			$this->loader = new CS_Loader();
			$this->loader->add_action( 'init', $this, 'add_consent_mode_conversion_exporter', 2 );
			$this->loader->run();
		}
	}

	/**
	 * Check if blocking is enabled
	 * @return bool
	 */
	public function enabled_blocking(): bool {
		static $cached_blocking = null;
		if ( $cached_blocking !== null ) {
			return $cached_blocking;
		}

		$active_rule_id = ConsentMagic()->get_active_rule_id();

		if ( ConsentMagic()->get_unblocked_crawlers() || !ConsentMagic()->ready_to_run()
		     || !CS_Integrations::check_showing()
		     || empty( $active_rule_id ) ) {
			return true;
		}

		$cs_type = get_post_meta( $active_rule_id, '_cs_type', true );
		if ( !$cs_type ) {
			return true;
		}

		if ( ConsentMagic()->getOption( 'cs_plugin_activation' ) != 1
		     || ConsentMagic()->getOption( 'cs_script_blocking_enabled' ) != 1
		     || ConsentMagic()->getOption( 'cs_block_pre_defined_scripts' ) != 1 ) {
			return true;
		}

		$ce_cat = get_term_by( 'slug', ConsentMagic()->getOption( 'cs_block_ce_scripts_cat' ), 'cs-cookies-category' );
		if ( !$ce_cat || is_wp_error( $ce_cat ) ) {
			return true;
		}


		$test_prefix = false;
		if ( ConsentMagic()->getOption( 'cs_test_mode' ) ) {
			$test_prefix = '_test';
		}
		$viewed_cookie      = "cs_viewed_cookie_policy" . $test_prefix;
		$cs_track_analytics = get_post_meta( $active_rule_id, '_cs_track_analytics', true );
		$analytics_cat_id   = ConsentMagic()->getOption( 'analytics_cat_id' );
		$category_cookie    = "cs_enabled_cookie_term" . $test_prefix . '_' . $ce_cat->term_id;
		$necessary_cat_id   = ConsentMagic()->getOption( 'necessary_cat_id' );
		$ignore             = (int) get_term_meta( $ce_cat->term_id, 'cs_ignore_this_category', true );
		$cache              = ConsentMagic()->getOption( 'cs_enable_site_cache' );

		if ( $cache != 0 ) {
			$blocking = true; //if cache enabled - block
		} else {
			if ( $necessary_cat_id == $ce_cat->term_id || $ignore == 1 ) {
				$blocking = false; //if necessary category - do not block
			} else {
				if ( CS_Cookies()->getCookie( $category_cookie ) && CS_Cookies()->getCookie( $viewed_cookie ) ) {
					if ( CS_Cookies()->getCookie( $category_cookie ) == 'yes' && CS_Cookies()->getCookie( $viewed_cookie ) == 'yes' ) {
						$blocking = false;
						//allowed by user then false
					} else {
						$blocking = true; //not allowed by user then true
					}
				} else {
					if ( $cs_type == 'iab' || $cs_type == 'ask_before_tracking' ) {
						if ( (int) $cs_track_analytics === 1 && $analytics_cat_id == $ce_cat->term_id ) {
							$blocking = false; //default it is false - need answer before track analytic
						} else {
							$blocking = true; //default it is true so blocks the code
						}
					} else {
						$blocking = false; //this is rule "inform_and_opiout" OR "just_inform"
					}
				}
			}
		}

		return $cached_blocking = $blocking;
	}

	/**
	 * Add cookie blocking
	 * @return void
	 */
	public function add_consent_mode_conversion_exporter(): void {

		$enabled_blocking = $this->enabled_blocking();
		add_filter( 'cnvex_cookies_consent', fn () => !$enabled_blocking );
	}
}

/**
 * @return ?CS_Conversion_Exporter
 */
function CS_Conversion_Exporter(): ?CS_Conversion_Exporter {
	return CS_Conversion_Exporter::instance();
}