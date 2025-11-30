<?php

namespace ConsentMagicPro;

if ( !defined( 'ABSPATH' ) ) {
	exit; // Exit if accessed directly.
}

/**
 * CS_Google_Consent_Mode
 * Google Consent Mode
 */
class CS_Google_Consent_Mode {

	use CS_Advanced_Consent_Mode;

	private static $_instance = null;

	public $loader;

	private $enable_blocking = false;

	private $consent_mode = null;

	private $ad_storage = false;

	private $analytics_storage = false;

	private $ad_user_data = false;

	private $ad_personalization = false;

	/**
	 * $_instance CS_Google_Consent_Mode
	 * @return CS_Google_Consent_Mode|null
	 */
	public static function instance() {
		if ( is_null( self::$_instance ) ) {
			self::$_instance = new self();
		}

		return self::$_instance;
	}

	private function __construct() {
		if ( is_plugin_activated() ) {
			$this->enable_blocking = ConsentMagic::$check_disable_cm && ConsentMagic()->getOption( 'cs_plugin_activation' );

			if ( $this->enabled_google_consent_mode() ) {
				$this->loader = new CS_Loader();
				$this->loader->add_action( 'init', $this, 'add_google_consent_mode', 2 );
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
	public function add_google_consent_mode() {
		if ( $this->enabled_blocking() ) {
			$active_rule_name         = ConsentMagic()->get_active_rule_name();
			$analytics_storage        = $ad_storage = $ad_user_data = $ad_personalization = 'granted';
			$analytics_storage_ignore = $ad_storage_ignore = $ad_user_data_ignore = $ad_personalization_ignore = false;

			$category = get_term_by( 'slug', ConsentMagic()->getOption( 'cs_block_analytics_storage_scripts_cat' ), 'cs-cookies-category' );
			if ( $category ) {
				$category_id = $category->term_id;
				if ( $active_rule_name == 'iab' ) {
					$category_id = CS_IAB_Integration()->check_enabled_purpose_by_category( $category_id ) ? $category_id : 0;
				}
				$analytics_storage        = $this->get_advanced_consent_mode_value( $category_id )[ 'value' ];
				$analytics_storage_ignore = get_term_meta( $category_id, 'cs_ignore_this_category', true ) == 1;
			}

			$category = get_term_by( 'slug', ConsentMagic()->getOption( 'cs_block_ad_storage_scripts_cat' ), 'cs-cookies-category' );
			if ( $category ) {
				$category_id = $category->term_id;
				if ( $active_rule_name == 'iab' ) {
					$category_id = CS_IAB_Integration()->check_enabled_purpose_by_category( $category_id ) ? $category_id : 0;
				}
				$ad_storage        = $this->get_advanced_consent_mode_value( $category_id )[ 'value' ];
				$ad_storage_ignore = get_term_meta( $category_id, 'cs_ignore_this_category', true ) == 1;
			}

			$category = get_term_by( 'slug', ConsentMagic()->getOption( 'cs_block_ad_user_data_scripts_cat' ), 'cs-cookies-category' );
			if ( $category ) {
				$category_id = $category->term_id;
				if ( $active_rule_name == 'iab' ) {
					$category_id = CS_IAB_Integration()->check_enabled_purpose_by_category( $category_id ) ? $category_id : 0;
				}
				$ad_user_data        = $this->get_advanced_consent_mode_value( $category_id )[ 'value' ];
				$ad_user_data_ignore = get_term_meta( $category_id, 'cs_ignore_this_category', true ) == 1;
			}

			$category = get_term_by( 'slug', ConsentMagic()->getOption( 'cs_block_ad_personalization_scripts_cat' ), 'cs-cookies-category' );
			if ( $category ) {
				$category_id = $category->term_id;
				if ( $active_rule_name == 'iab' ) {
					$category_id = CS_IAB_Integration()->check_enabled_purpose_by_category( $category_id ) ? $category_id : 0;
				}
				$ad_personalization        = $this->get_advanced_consent_mode_value( $category_id )[ 'value' ];
				$ad_personalization_ignore = get_term_meta( $category_id, 'cs_ignore_this_category', true ) == 1;
			}

			$blocking_enabled         = ConsentMagic()->ready_to_run();
			$this->ad_storage         = (bool) apply_filters( 'cm_ad_storage_mode', $blocking_enabled && !$ad_storage_ignore && ConsentMagic()->getOption( 'cs_block_ad_storage_scripts' ) );
			$this->analytics_storage  = (bool) apply_filters( 'cm_analytics_storage_mode', $blocking_enabled && !$analytics_storage_ignore && ConsentMagic()->getOption( 'cs_block_analytics_storage_scripts' ) );
			$this->ad_user_data       = (bool) apply_filters( 'cm_ad_user_data_mode', $blocking_enabled && !$ad_user_data_ignore && ConsentMagic()->getOption( 'cs_block_ad_user_data_scripts' ) );
			$this->ad_personalization = (bool) apply_filters( 'cm_ad_personalization_mode', $blocking_enabled && !$ad_personalization_ignore && ConsentMagic()->getOption( 'cs_block_ad_personalization_scripts' ) );

		} else {
			$this->ad_storage         = true;
			$this->analytics_storage  = true;
			$this->ad_user_data       = true;
			$this->ad_personalization = true;
			$analytics_storage        = $ad_storage = $ad_user_data = $ad_personalization = 'granted';
		}

		add_filter( 'cm_google_consent_mode', function() use ( $analytics_storage, $ad_storage, $ad_user_data, $ad_personalization ) {
			return array(
				'analytics_storage'  => array(
					'enabled' => $this->analytics_storage,
					'value'   => $analytics_storage,
				),
				'ad_storage'         => array(
					'enabled' => $this->ad_storage,
					'value'   => $ad_storage,
				),
				'ad_user_data'       => array(
					'enabled' => $this->ad_user_data,
					'value'   => $ad_user_data,
				),
				'ad_personalization' => array(
					'enabled' => $this->ad_personalization,
					'value'   => $ad_personalization,
				),
			);
		} );
	}


	public function enabled_google_consent_mode() {

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
				$consent_mode = get_post_meta( $active_rule_id, '_cs_google_consent_mode', true );
			}
		}
		$this->consent_mode = $consent_mode;

		return $consent_mode;
	}

	public function get_google_consent_mode() {
		$active_rule_id = ConsentMagic()->get_active_rule_id();
		$cs_type        = ( !empty( $active_rule_id ) ) ? get_post_meta( $active_rule_id, '_cs_type', true ) : 'false';

		$google_consent_mode = apply_filters( 'cm_google_consent_mode', array() );

		if ( isPYSActivated() && CS_Google_Consent_Mode()->enabled_google_consent_mode() ) {
			$pys_analytics_storage_mode  = has_filter( 'pys_analytics_storage_mode' );
			$pys_ad_storage_mode         = has_filter( 'pys_ad_storage_mode' );
			$pys_ad_user_data_mode       = has_filter( 'pys_ad_user_data_mode' );
			$pys_ad_personalization_mode = has_filter( 'pys_ad_personalization_mode' );

			$google_consent_mode[ 'analytics_storage' ][ 'filter' ]  = $pys_analytics_storage_mode;
			$google_consent_mode[ 'ad_storage' ][ 'filter' ]         = $pys_ad_storage_mode;
			$google_consent_mode[ 'ad_user_data' ][ 'filter' ]       = $pys_ad_user_data_mode;
			$google_consent_mode[ 'ad_personalization' ][ 'filter' ] = $pys_ad_personalization_mode;
			$google_consent_mode[ 'analytics_storage' ][ 'value' ]   = $pys_analytics_storage_mode ? ( apply_filters( 'pys_analytics_storage_mode', true ) ? 'granted' : 'denied' ) : $google_consent_mode[ 'analytics_storage' ][ 'value' ];
			$google_consent_mode[ 'ad_storage' ][ 'value' ]          = $pys_ad_storage_mode ? ( apply_filters( 'pys_ad_storage_mode', true ) ? 'granted' : 'denied' ) : $google_consent_mode[ 'ad_storage' ][ 'value' ];
			$google_consent_mode[ 'ad_user_data' ][ 'value' ]        = $pys_ad_user_data_mode ? ( apply_filters( 'pys_ad_user_data_mode', true ) ? 'granted' : 'denied' ) : $google_consent_mode[ 'ad_user_data' ][ 'value' ];
			$google_consent_mode[ 'ad_personalization' ][ 'value' ]  = $pys_ad_personalization_mode ? ( apply_filters( 'pys_ad_personalization_mode', true ) ? 'granted' : 'denied' ) : $google_consent_mode[ 'ad_personalization' ][ 'value' ];
		} else {
			$google_consent_mode[ 'analytics_storage' ][ 'filter' ]  = 0;
			$google_consent_mode[ 'ad_storage' ][ 'filter' ]         = 0;
			$google_consent_mode[ 'ad_user_data' ][ 'filter' ]       = 0;
			$google_consent_mode[ 'ad_personalization' ][ 'filter' ] = 0;
		}

		$ad_storage_category         = get_term_by( 'slug', ConsentMagic()->getOption( 'cs_block_ad_storage_scripts_cat' ), 'cs-cookies-category' )->term_id;
		$ad_storage_category         = $this->enable_blocking && $this->check_category( $ad_storage_category ) && ConsentMagic()->getOption( 'cs_block_ad_storage_scripts' ) ? $ad_storage_category : 0;
		$analytics_storage_category  = get_term_by( 'slug', ConsentMagic()->getOption( 'cs_block_analytics_storage_scripts_cat' ), 'cs-cookies-category' )->term_id;
		$analytics_storage_category  = $this->enable_blocking && $this->check_category( $analytics_storage_category ) && ConsentMagic()->getOption( 'cs_block_analytics_storage_scripts' ) ? $analytics_storage_category : 0;
		$ad_user_data_category       = get_term_by( 'slug', ConsentMagic()->getOption( 'cs_block_ad_user_data_scripts_cat' ), 'cs-cookies-category' )->term_id;
		$ad_user_data_category       = $this->enable_blocking && $this->check_category( $ad_user_data_category ) && ConsentMagic()->getOption( 'cs_block_ad_user_data_scripts' ) ? $ad_user_data_category : 0;
		$ad_personalization_category = get_term_by( 'slug', ConsentMagic()->getOption( 'cs_block_ad_personalization_scripts_cat' ), 'cs-cookies-category' )->term_id;
		$ad_personalization_category = $this->enable_blocking && $this->check_category( $ad_personalization_category ) && ConsentMagic()->getOption( 'cs_block_ad_personalization_scripts' ) ? $ad_personalization_category : 0;

		if ( $cs_type == 'iab' ) {
			$google_consent_mode[ 'ad_storage' ][ 'category' ]         = CS_IAB_Integration()->check_enabled_purpose_by_category( $ad_storage_category ) ? $ad_storage_category : 0;
			$google_consent_mode[ 'analytics_storage' ][ 'category' ]  = CS_IAB_Integration()->check_enabled_purpose_by_category( $analytics_storage_category ) ? $analytics_storage_category : 0;
			$google_consent_mode[ 'ad_user_data' ][ 'category' ]       = CS_IAB_Integration()->check_enabled_purpose_by_category( $ad_user_data_category ) ? $ad_user_data_category : 0;
			$google_consent_mode[ 'ad_personalization' ][ 'category' ] = CS_IAB_Integration()->check_enabled_purpose_by_category( $ad_personalization_category ) ? $ad_personalization_category : 0;
		} else {
			$google_consent_mode[ 'ad_storage' ][ 'category' ]         = $ad_storage_category;
			$google_consent_mode[ 'analytics_storage' ][ 'category' ]  = $analytics_storage_category;
			$google_consent_mode[ 'ad_user_data' ][ 'category' ]       = $ad_user_data_category;
			$google_consent_mode[ 'ad_personalization' ][ 'category' ] = $ad_personalization_category;
		}

		return $google_consent_mode;
	}
}

/**
 * @return CS_Google_Consent_Mode
 */
function CS_Google_Consent_Mode(): CS_Google_Consent_Mode {
	return CS_Google_Consent_Mode::instance();
}