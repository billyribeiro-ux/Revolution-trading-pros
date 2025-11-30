<?php

namespace ConsentMagicPro;

if ( !defined( 'ABSPATH' ) ) {
	exit; // Exit if accessed directly.
}

class CS_Cookies {

	private static ?CS_Cookies $_instance = null;

	private array $cookies = array();

	private bool $cross_domain_cookies_loaded = false;

	/**
	 * $_instance CS_Cookies
	 * @return CS_Cookies|null
	 */
	public static function instance(): ?CS_Cookies {
		if ( is_null( self::$_instance ) ) {
			self::$_instance = new self();
		}

		return self::$_instance;
	}

	public function __construct() {
	}

	/**
	 * Get cookie value
	 * @param string $name
	 * @return string|null
	 */
	public function getCookie( string $name ): ?string {

		if ( !isset( $_COOKIE ) ) {
			return null;
		}

		if ( isset( $this->cookies[ $name ] ) ) {
			return $this->cookies[ $name ];
		} else {
			$cross_domain = ConsentMagic()->getOption( 'cs_cross_domain_tracking_domain' );
			$cross_domain_enabled = ConsentMagic()->getOption( 'cs_cross_domain_tracking' );
			if ( !$this->cross_domain_cookies_loaded && $cross_domain_enabled && !empty( $cross_domain )
			     && isset( $_COOKIE[ 'cs_cross_domain_categories' ] ) ) {
				$categories = json_decode( base64_decode( $_COOKIE[ 'cs_cross_domain_categories' ] ), true );
				if ( is_array( $categories ) ) {
					foreach ( $categories as $key => $category ) {
						$term = get_term_by( 'slug', $key, 'cs-cookies-category' );
						if ( $term ) {
							$this->cookies[ 'cs_enabled_cookie_term_' . $term->term_id  ] = sanitize_text_field( $category );
						}
					}
				}
			}

			$this->cross_domain_cookies_loaded = true;

			if ( !isset( $this->cookies[ $name ] ) ) {
				if ( isset( $_COOKIE[ $name ] ) ) {
					$this->cookies[ $name ] = sanitize_text_field( $_COOKIE[ $name ] );
				}
			}
		}

		if ( isset( $this->cookies[ $name ] ) ) {
			return $this->cookies[ $name ];
		}

		return null;
	}
}

/**
 * CS_Cookies function
 * @return ?CS_Cookies
 */
function CS_Cookies(): ?CS_Cookies {
	return CS_Cookies::instance();
}

CS_Cookies();