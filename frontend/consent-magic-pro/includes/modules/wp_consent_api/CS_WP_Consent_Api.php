<?php

namespace ConsentMagicPro;

if ( !defined( 'ABSPATH' ) ) {
	exit; // Exit if accessed directly.
}

/**
 * WP Consent API Integration Class
 * Handles integration with the WordPress Consent API plugin, providing:
 * - Consent type management (opt-in/opt-out)
 * - Category mapping between ConsentMagic Pro and WP Consent API
 * - Consent logging with IP address tracking
 * - JavaScript-based real-time consent change detection
 * - Admin interface for viewing consent logs and statistics
 */
class CS_WP_Consent_Api {

	/**
	 * Singleton instance
	 * @var CS_WP_Consent_Api|null
	 */
	private static ?CS_WP_Consent_Api $_instance = null;

	/**
	 * Default category mappings from ConsentMagic Pro to WP Consent API
	 */
	private array $default_categories = array(
		'analytics'      => 'statistics',
		'marketing'      => 'marketing',
		'googlefonts'    => 'preferences',
		'embedded_video' => 'preferences',
		'necessary'      => 'functional',
		'default'        => 'marketing'
	);

	/**
	 * Get singleton instance
	 * Implements the singleton pattern to ensure only one instance of the class exists.
	 * @return ?CS_WP_Consent_Api The singleton instance
	 */
	public static function instance(): ?CS_WP_Consent_Api {
		if ( is_null( self::$_instance ) ) {
			self::$_instance = new self();
		}

		return self::$_instance;
	}

	/**
	 * Constructor
	 * Sets up WordPress hooks
	 */
	public function __construct() {
		add_action( 'init', array( $this, 'init' ), 11 );
	}

	/**
	 * Initialize WP Consent API integration
	 * @return void
	 */
	public function init(): void {
		// Check if the WP Consent API is active and add the filter to enable Consent magic pro.
		if ( $this->isEnableIntegration() ) {
			$plugin = CMPRO_PLUGIN_BASENAME;
			add_filter( "wp_consent_api_registered_{$plugin}", '__return_true' );
			add_filter( 'wp_consent_api_waitfor_consent_hook', array( $this, 'setWaitFor' ) );

			$this->setConsentType();
		}
	}

	/**
	 * Check if the WP Consent API plugin is active
	 * Determines if the WP Consent API plugin is installed and active by checking
	 * for the existence of the wp_has_consent function.
	 * @return bool True if WP Consent API is active, false otherwise
	 */
	public function isEnableWPConsent(): bool {
		return function_exists( 'wp_has_consent' );
	}

	/**
	 * Check if WP Consent API integration should be enabled
	 * @return bool True if integration should be enabled, false otherwise
	 */
	public function isEnableIntegration(): bool {
		return ( ConsentMagic()->getOption( 'cs_wp_consent_api_enabled' ) == 1 )
		       && ConsentMagic()->getOption( 'cs_plugin_activation' )
		       && $this->isEnableWPConsent()
		       && ConsentMagic()->ready_to_run()
		       && CS_Integrations::check_showing();
	}

	/**
	 * Set the consent type for WP Consent API
	 * @return void
	 */
	public function setConsentType(): void {
		$type = ConsentMagic()->get_active_rule_name() == 'inform_and_opiout' ? 'optout' : 'optin';

		add_filter( 'wp_get_consent_type', function() use ( $type ) {
			return $type;
		} );
	}

	/**
	 * Get the settings for the WP Consent API integration
	 * Builds and returns the configuration settings for WP Consent API integration,
	 * including category mappings, consent type, and enabled categories based on
	 * ConsentMagic Pro configuration and IAB settings.
	 * @return array Array containing integration settings with categories and consent type
	 */
	public function getSettings(): array {
		$wp_api_categories = $this->getWPConsentAPICategories();
		$categories        = array();

		$unassigned     = get_term_by( 'slug', 'unassigned', 'cs-cookies-category' );
		$cm_categories  = get_cookies_terms_objects_all( $unassigned->term_id );
		$consent_type   = ConsentMagic()->get_active_rule_name();
		$iab_settings   = CS_IAB_Integration()->get_settings();

		if ( !empty( $wp_api_categories ) ) {
			if ( !empty( $cm_categories ) ) {
				foreach ( $cm_categories as $category ) {

					if ( $consent_type == 'iab' ) {
						$iab_cat        = get_term_meta( $category->term_id, '_cs_iab_cat', true );
						$iab_cat_active = isset( $iab_settings->purposes->{$iab_cat} )
						                  && $iab_settings->purposes->{$iab_cat} == 1;

						if ( empty( $iab_cat ) || $iab_cat == '0' || !$iab_cat_active ) {
							continue;
						}
					}

					$cat    = get_term_meta( $category->term_id, '_cs_wp_consent_api_cat', true );
					$ignore = get_term_meta( $category->term_id, 'cs_ignore_this_category', true );
					if ( $cat !== '' && !$ignore && isset( $wp_api_categories[ $cat ] ) ) {
						if ( isset( $categories[ $cat ] ) ) {
							$categories[ $cat ][ 'ids' ][] = $category->term_id;
						} else {
							$categories[ $cat ] = array(
								'name' => $wp_api_categories[ $cat ],
								'slug' => $cat,
								'ids'  => array( $category->term_id ),
							);
						}
					}
				}
			}

			foreach ( $wp_api_categories as $k => $cat ) {
				if ( !isset( $categories[ $k ] ) ) {
					$categories[ $k ] = array(
						'name' => $cat,
						'slug' => $k,
						'ids'  => array(),
					);
				}
			}
		}

		$settings = array(
			'enabled'      => $this->isEnableIntegration(),
			'consent_type' => ConsentMagic()->get_active_rule_name() == 'ldu' ? 'optout' : 'optin',
			'categories'   => $categories
		);

		return $settings;
	}

	/**
	 * Get available WP Consent API categories
	 * @return array Associative array of category slug => formatted name
	 */
	public function getWPConsentAPICategories(): array {
		$categories = apply_filters( 'wp_consent_categories', array(
			'functional',
			'preferences',
			'statistics',
			'statistics-anonymous',
			'marketing',
		) );

		$options = array();
		foreach ( $categories as $category ) {
			$options[ $category ] = ucfirst( $category );
		}

		return $options;
	}

	/**
	 * Get default category mappings
	 * @return array Default category mappings
	 */
	public function getWPConsentAPIDefaultCategories(): array {
		return $this->default_categories;
	}

	/**
	 * Set wait-for-consent hook behavior
	 * @param bool $waitfor Current wait-for-consent setting
	 * @return bool Modified wait-for-consent setting
	 */
	public function setWaitFor( $waitfor ): bool {
		if ( ConsentMagic()->getOption( 'cs_enable_site_cache' ) === 1 ) {
			$waitfor = true;
		}

		return $waitfor;
	}
}

/**
 * Get the CS_WP_Consent_Api instance
 * @return CS_WP_Consent_Api The singleton instance
 */
function CS_WP_Consent_Api(): CS_WP_Consent_Api {
	return CS_WP_Consent_Api::instance();
}