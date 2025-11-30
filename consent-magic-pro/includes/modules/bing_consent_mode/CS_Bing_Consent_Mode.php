<?php

namespace ConsentMagicPro;

if ( !defined( 'ABSPATH' ) ) {
	exit; // Exit if accessed directly.
}

/**
 * CS_Bing_Consent_Mode
 * Bing Consent Mode
 */
class CS_Bing_Consent_Mode {

	use CS_Advanced_Consent_Mode;

	private static $_instance = null;

	public $loader;

	private $enable_blocking = false;

	private $consent_mode = null;

	private $ad_storage = false;

	/**
	 * $_instance CS_Bing_Consent_Mode
	 * @return CS_Bing_Consent_Mode|null
	 */
	public static function instance() {
		if ( is_null( self::$_instance ) ) {
			self::$_instance = new self();
		}

		return self::$_instance;
	}

	private function __construct() {

		// TODO Order consent for Bing

		if ( is_plugin_activated() ) {
			$this->enable_blocking = ConsentMagic::$check_disable_cm && ConsentMagic()->getOption( 'cs_plugin_activation' );

			if ( $this->enabled_bing_consent_mode() ) {
				$this->loader = new CS_Loader();
				$this->loader->add_action( 'init', $this, 'add_bing_consent_mode', 2 );
				$this->loader->run();
			}
		}
	}

	public function enabled_blocking() {
		return $this->enable_blocking;
	}

	/**
	 * Add google consent mode
	 * @return void
	 */
	public function add_bing_consent_mode(): void {
		if ( $this->enabled_blocking() ) {
			$active_rule_name  = ConsentMagic()->get_active_rule_name();
			$ad_storage        = 'granted';
			$ad_storage_ignore = false;

			$category = get_term_by( 'slug', ConsentMagic()->getOption( 'cs_block_big_tag_scripts_cat' ), 'cs-cookies-category' );
			if ( $category ) {
				$category_id = $category->term_id;
				if ( $active_rule_name == 'iab' ) {
					$category_id = CS_IAB_Integration()->check_enabled_purpose_by_category( $category_id ) ? $category_id : 0;
				}
				$ad_storage        = $this->get_advanced_consent_mode_value( $category_id )[ 'value' ];
				$ad_storage_ignore = get_term_meta( $category_id, 'cs_ignore_this_category', true ) == 1;
			}

			$blocking_enabled = ConsentMagic()->ready_to_run();
			$this->ad_storage = (bool) apply_filters( 'cm_bing_ad_storage_mode', $blocking_enabled && !$ad_storage_ignore && ConsentMagic()->getOption( 'cs_block_big_tag_scripts' ) );
		} else {
			$this->ad_storage = true;
			$ad_storage       = 'granted';
		}

		add_filter( 'cm_bing_consent_mode', function() use ( $ad_storage ) {
			return array(
				'ad_storage' => array(
					'enabled' => $this->ad_storage,
					'value'   => $ad_storage,
				),
			);
		} );
	}


	public function enabled_bing_consent_mode() {

		if ( $this->consent_mode !== null ) {
			return $this->consent_mode;
		}

		$consent_mode   = false;
		$active_rule_id = ConsentMagic()->get_active_rule_id();

		if ( ConsentMagic()->ready_to_run() && !empty( $active_rule_id ) ) {
			$cs_type = get_post_meta( $active_rule_id, '_cs_type', true );
			if ( $cs_type == 'just_inform' ) {
				$consent_mode = true;
			} else {
				$consent_mode = get_post_meta( $active_rule_id, '_cs_bing_consent_mode', true );
			}
		}
		$this->consent_mode = $consent_mode;

		return $consent_mode;
	}

	public function get_bing_consent_mode() {

		$active_rule_id = ConsentMagic()->get_active_rule_id();
		$cs_type        = ( !empty( $active_rule_id ) ) ? get_post_meta( $active_rule_id, '_cs_type', true ) : 'false';

		$bing_consent_mode = apply_filters( 'cm_bing_consent_mode', array() );

		if ( isPYSActivated() && $this->enabled_bing_consent_mode() ) {
			$pys_ad_storage_mode = has_filter( 'pys_bing_ad_storage_mode' );

			$bing_consent_mode[ 'ad_storage' ][ 'filter' ] = $pys_ad_storage_mode;
			$bing_consent_mode[ 'ad_storage' ][ 'value' ]  = $pys_ad_storage_mode ? ( apply_filters( 'pys_bing_ad_storage_mode', true ) ? 'granted' : 'denied' ) : $bing_consent_mode[ 'ad_storage' ][ 'value' ];
		} else {
			$bing_consent_mode[ 'ad_storage' ][ 'filter' ] = 0;
		}

		$ad_storage_category = get_term_by( 'slug', ConsentMagic()->getOption( 'cs_block_big_tag_scripts_cat' ), 'cs-cookies-category' )->term_id;

		if ( $this->enable_blocking && $this->check_category( $ad_storage_category ) && ConsentMagic()->getOption( 'cs_block_big_tag_scripts' ) ) {
			if ( $cs_type == 'iab' ) {
				$bing_consent_mode[ 'ad_storage' ][ 'category' ] = CS_IAB_Integration()->check_enabled_purpose_by_category( $ad_storage_category ) ? $ad_storage_category : 0;
			} else {
				$bing_consent_mode[ 'ad_storage' ][ 'category' ] = $ad_storage_category;
			}
		} else {
			$bing_consent_mode[ 'ad_storage' ][ 'category' ] = 0;
		}

		return $bing_consent_mode;
	}

	public function check_pys_version() {
		if ( isBingActive() && defined( 'PYS_BING_VERSION' ) && version_compare( PYS_BING_VERSION, '3.5.3.1', '>=' ) ) {
			if ( isPYSProActivated() && defined( 'PYS_VERSION' ) ) {
				return version_compare( PYS_VERSION, '11.3.0.3', '>=' );
			}

			if ( isPYSFreeActivated() && defined( 'PYS_FREE_VERSION' ) ) {
				return version_compare( PYS_FREE_VERSION, '10.2.0.3', '>=' );
			}
		}

		return false;
	}
}

/**
 * @return CS_Bing_Consent_Mode
 */
function CS_Bing_Consent_Mode(): CS_Bing_Consent_Mode {
	return CS_Bing_Consent_Mode::instance();
}