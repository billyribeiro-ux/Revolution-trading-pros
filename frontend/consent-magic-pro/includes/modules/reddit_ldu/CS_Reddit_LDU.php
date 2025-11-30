<?php

namespace ConsentMagicPro;

if ( !defined( 'ABSPATH' ) ) {
	exit; // Exit if accessed directly.
}

/**
 * CS_Reddit_LDU
 * Reddit Consent Mode
 */
class CS_Reddit_LDU {

	private static $_instance = null;

	public $loader;

	/**
	 * $_instance CS_Reddit_LDU
	 * @return CS_Reddit_LDU|null
	 */
	public static function instance(): ?CS_Reddit_LDU {
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
		$this->loader->add_action( 'init', $this, 'add_reddit_ldu_mode' );
		$this->loader->run();
	}

	/**
	 * Add Reddit ldu mode
	 * @return void
	 */
	public function add_reddit_ldu_mode(): void {

		if ( !$this->check_pys_version() ) {
			return;
		}

		$active_rule_id = ConsentMagic()->get_active_rule_id();

		if ( ConsentMagic()->get_unblocked_crawlers() || empty( $active_rule_id ) || !ConsentMagic()->ready_to_run() ) {
			return;
		}

		$test_prefix = ConsentMagic()->getOption( 'cs_test_mode' ) ? '_test' : '';
		$viewed_cookie = "cs_viewed_cookie_policy" . $test_prefix;

		if ( ConsentMagic()->getOption( 'cs_plugin_activation' ) != 1 ) {
			return;
		}

		$cs_type = get_post_meta( $active_rule_id, '_cs_type', true );

		if ( $cs_type == 'just_inform' || $cs_type == 'iab'
		     || ConsentMagic()->getOption(
				'cs_block_reddit_pixel_scripts'
			) != 1 ) {
			return;
		}

		$cs_track_analytics = ( ConsentMagic()->getOption( 'cs_block_pre_defined_scripts' ) == 1
		                        && !CS_Cookies()->getCookie( $viewed_cookie ) ) ? get_post_meta(
			$active_rule_id,
			'_cs_track_analytics',
			true
		) : false;
		$reddit_cat_slug    = ConsentMagic()->getOption( 'cs_block_reddit_pixel_scripts_cat' );
		$reddit_cat_id      = ConsentMagic()->getOption(
			ConsentMagic()->getOption( 'cs_block_reddit_pixel_scripts_cat' ) . '_cat_id'
		);
		$analytics_cat_id   = ConsentMagic()->getOption( 'analytics_cat_id' );

		$category_cookie = "cs_enabled_cookie_term{$test_prefix}_{$reddit_cat_id}";
		$ignore          = (int) get_term_meta( $reddit_cat_id, 'cs_ignore_this_category', true );

		if ( $reddit_cat_slug == 'necessary' ) {
			$enable_ldu = false;
		} elseif ( $cs_track_analytics && $reddit_cat_id == $analytics_cat_id ) {
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
			add_filter( 'pys_reddit_ldu_mode', fn () => $this->get_reddit_ldu(), 100 );
		}
	}

	/**
	 * Get Reddit Limited Data Use
	 * @return false|mixed
	 */
	private function get_reddit_ldu() {
		$active_rule_id = ConsentMagic()->get_active_rule_id();
		if ( empty( $active_rule_id ) ) {
			return false;
		}

		return (bool) get_post_meta( $active_rule_id, '_cs_reddit_ldu', true );
	}

	/**
	 * Check if Reddit Limited Data Use is enabled
	 * @return bool
	 */
	private function enabled_reddit_ldu(): bool {
		$enabled = ConsentMagic::$check_disable_cm && ConsentMagic()->getOption( 'cs_plugin_activation' );

		$active_rule_id = ConsentMagic()->get_active_rule_id();
		$cs_type        = get_post_meta( $active_rule_id, '_cs_type', true );
		if ( $enabled && ( $cs_type == 'iab' || $cs_type == 'just_inform' ) ) {
			$enabled = false;
		}

		return $enabled;
	}

	/**
	 * Check PYS version
	 * @return bool
	 */
	public function check_pys_version(): bool {
		if ( isRedditActive() && defined( 'PYS_REDDIT_VERSION' ) ) {
			if ( isPYSProActivated() && defined( 'PYS_VERSION' ) ) {
				return version_compare( PYS_VERSION, '12.3.0', '>=' );
			}

			if ( isPYSFreeActivated() && defined( 'PYS_FREE_VERSION' ) ) {
				return version_compare( PYS_FREE_VERSION, '11.1.4', '>=' );
			}
		}

		return false;
	}
}

/**
 * @return CS_Reddit_LDU
 */
function CS_Reddit_LDU(): CS_Reddit_LDU {
	return CS_Reddit_LDU::instance();
}