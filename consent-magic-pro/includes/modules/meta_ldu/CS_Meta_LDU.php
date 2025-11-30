<?php

namespace ConsentMagicPro;

if ( !defined( 'ABSPATH' ) ) {
	exit; // Exit if accessed directly.
}

/**
 * CS_Meta_LDU
 * Meta Limited Data Use
 */
class CS_Meta_LDU {

	private static $_instance = null;

	public $loader;

	/**
	 * $_instance CS_Meta_LDU
	 * @return CS_Meta_LDU|null
	 */
	public static function instance() {
		if ( is_null( self::$_instance ) ) {
			self::$_instance = new self();
		}

		return self::$_instance;
	}

	private function __construct() {

		if ( !isPYSActivated() || ( is_admin() && !wp_doing_ajax() ) ) {
			return;
		}

		$this->loader = new CS_Loader();
		$this->loader->add_action( 'init', $this, 'add_meta_ldu_mode' );
		$this->loader->run();
	}

	/**
	 * Add meta ldu mode
	 * @return void
	 */
	public function add_meta_ldu_mode() {

		if ( !$this->check_pys_version() ) {
			return;
		}

		$active_rule_id = ConsentMagic()->get_active_rule_id();

		if ( ConsentMagic()->get_unblocked_crawlers() || empty( $active_rule_id ) || !ConsentMagic()->ready_to_run() ) {
			return;
		}

		$test_prefix   = ConsentMagic()->getOption( 'cs_test_mode' ) ? '_test' : '';
		$viewed_cookie = "cs_viewed_cookie_policy" . $test_prefix;

		if ( ConsentMagic()->getOption( 'cs_plugin_activation' ) != 1 ) {
			return;
		}

		$cs_type = get_post_meta( $active_rule_id, '_cs_type', true );

		if ( $cs_type == 'just_inform' || $cs_type == 'iab' || ConsentMagic()->getOption( 'cs_block_fb_pixel_scripts' ) != 1 ) {
			return;
		}

		$cs_track_analytics = ( ConsentMagic()->getOption( 'cs_block_pre_defined_scripts' ) == 1 && !CS_Cookies()->getCookie( $viewed_cookie ) ) ? get_post_meta( $active_rule_id, '_cs_track_analytics', true ) : false;
		$meta_cat_slug      = ConsentMagic()->getOption( 'cs_block_fb_pixel_scripts_cat' );
		$meta_cat_id        = ConsentMagic()->getOption( ConsentMagic()->getOption( 'cs_block_fb_pixel_scripts_cat' ) . '_cat_id' );
		$analytics_cat_id   = ConsentMagic()->getOption( 'analytics_cat_id' );

		$category_cookie = "cs_enabled_cookie_term{$test_prefix}_{$meta_cat_id}";
		$ignore          = (int) get_term_meta( $meta_cat_id, 'cs_ignore_this_category', true );

		if ( $meta_cat_slug == 'necessary' ) {
			$enable_ldu = false;
		} elseif ( $cs_track_analytics && $meta_cat_id == $analytics_cat_id ) {
			$enable_ldu = false;
		} elseif ( !CS_Cookies()->getCookie( $viewed_cookie ) && $cs_type == 'inform_and_opiout' ) {
			$enable_ldu = false;
		} elseif ( !CS_Cookies()->getCookie( $viewed_cookie ) && $cs_type == 'ask_before_tracking' ) {
			$enable_ldu = true;
		} elseif ( CS_Cookies()->getCookie( $category_cookie ) && CS_Cookies()->getCookie( $category_cookie ) == 'no'
		           && $ignore == 0 ) {
			$enable_ldu = true;
		} else {
			$enable_ldu = false;
		}

		if ( $enable_ldu ) {
			add_filter( 'pys_meta_ldu_mode', fn () => $this->get_meta_ldu(), 100 );
		}
	}

	/**
	 * Get Meta Limited Data Use
	 * @return false|mixed
	 */
	private function get_meta_ldu() {
		$active_rule_id = ConsentMagic()->get_active_rule_id();
		if ( empty( $active_rule_id ) ) {
			return false;
		}

		return (bool) get_post_meta( $active_rule_id, '_cs_use_meta_ldu', true );
	}

	/**
	 * Check if Meta Limited Data Use is enabled
	 * @return bool
	 */
	private function enabled_meta_ldu() {
		$enabled = ConsentMagic::$check_disable_cm && ConsentMagic()->getOption( 'cs_plugin_activation' );

		$active_rule_id = ConsentMagic()->get_active_rule_id();
		$cs_type        = get_post_meta( $active_rule_id, '_cs_type', true );
		if ( $enabled && ( $cs_type == 'iab' || $cs_type == 'just_inform' ) ) {
			$enabled = false;
		}

		return $enabled;
	}

	/**
	 * Check if PixelYourSite plugin installed and activated
	 * @return bool
	 */
	public function check_pys_version() {
		if ( isPYSProActivated() && defined( 'PYS_VERSION' ) ) {
			return version_compare( PYS_VERSION, '11.2.1', '>=' );
		}

		if ( isPYSFreeActivated() && defined( 'PYS_FREE_VERSION' ) ) {
			return version_compare( PYS_FREE_VERSION, '10.1.0', '>=' );
		}

		return false;
	}
}

/**
 * @return CS_Meta_LDU
 */
function CS_Meta_LDU(): CS_Meta_LDU {
	return CS_Meta_LDU::instance();
}