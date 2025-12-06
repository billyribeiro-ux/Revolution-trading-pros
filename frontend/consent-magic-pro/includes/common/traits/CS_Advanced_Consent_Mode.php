<?php

namespace ConsentMagicPro;

if ( !defined( 'ABSPATH' ) ) {
	exit; // Exit if accessed directly.
}


trait CS_Advanced_Consent_Mode {
	/**
	 * Get advanced consent mode value
	 * @param $cat_id
	 * @return array
	 */
	public function get_advanced_consent_mode_value( $cat_id ) {

		$active_rule_id = ConsentMagic()->get_active_rule_id();
		$check_cat      = $this->check_category( $cat_id );

		if ( !$check_cat ) {
			return array(
				'status' => false,
				'value'  => 'granted'
			);
		}

		$cs_type            = get_post_meta( $active_rule_id, '_cs_type', true );
		$analytics_cat_id   = ConsentMagic()->getOption( 'analytics_cat_id' );
		$cs_track_analytics = get_post_meta( $active_rule_id, '_cs_track_analytics', true );
		$test_prefix        = false;
		$check              = true;

		if ( ConsentMagic()->getOption( 'cs_test_mode' ) ) {
			$test_prefix = '_test';
		}
		$viewed_cookie   = "cs_viewed_cookie_policy" . $test_prefix;
		$category_cookie = "cs_enabled_cookie_term" . $test_prefix . '_' . $cat_id;
		$native_scripts  = get_post_meta( $active_rule_id, '_cs_native_scripts', true );

		if ( CS_Cookies()->getCookie( 'cs_consent_string' ) && $cs_type == 'iab' && $native_scripts == 0 ) {
			try {
				$iab_string = json_decode( base64_decode( CS_Cookies()->getCookie( 'cs_consent_string' ) ), false );
				$purposes   = $iab_string->purposes;
				$iab_cat    = get_term_meta( $cat_id, '_cs_iab_cat', true );
				if ( !empty( $iab_cat ) ) {
					if ( $iab_cat != 0 ) {
						$v     = substr( $purposes, $iab_cat - 1, 1 );
						$check = ( $v !== '' ? $v : 1 ) == 0;
					} else {
						$check = false;
					}
				}
			} catch ( \Exception $e ) {
				error_log( 'Error: ' . $e->getMessage() );
			}
		} else {
			if ( CS_Cookies()->getCookie( $category_cookie ) && CS_Cookies()->getCookie( $viewed_cookie ) ) {
				if ( CS_Cookies()->getCookie( $category_cookie ) == 'yes' && CS_Cookies()->getCookie( $viewed_cookie ) == 'yes' ) {
					$check = false; //allowed by user then false
				}
			} else {
				if ( $cs_type == 'iab' || $cs_type == 'ask_before_tracking' ) {
					if ( (int) $cs_track_analytics === 1 && $analytics_cat_id == $cat_id ) {
						$check = false; //track analytics
					}
				} else {
					$check = false; //this is rule "inform_and_opiout" OR "just_inform"
				}
			}
		}

		return array(
			'status' => $check,
			'value'  => $check ? 'denied' : 'granted'
		);
	}

	public function check_category( $cat_id ) {
		$active_rule_id = ConsentMagic()->get_active_rule_id();
		$ignore         = get_term_meta( $cat_id, 'cs_ignore_this_category', true );
		$necessary      = get_term_by( 'slug', 'necessary', 'cs-cookies-category' );
		$necessary_id   = $necessary->term_id;

		if ( empty( $active_rule_id ) || $cat_id == 0 || $ignore == 1 || $cat_id == $necessary_id ) {
			return false;
		} else {
			return true;
		}
	}
}