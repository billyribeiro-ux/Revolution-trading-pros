<?php

namespace ConsentMagicPro;

// If this file is called directly, abort.
if ( !defined( 'WPINC' ) ) {
	die;
}

// it will add a link that opens the options popup
function cm_options_shortcode( $atts, $content ) {
	$Content           = '<div class="cm_shortcode_container" style="' . esc_attr( get_shortcode_styles() ) . '">';
	$cs_hide_shortcode = ConsentMagic()->getOption( 'cs_hide_shortcode_cm_options' );
	$active_rule_id    = ConsentMagic()->cs_options[ 'active_rule_id' ];
	$metas             = get_post_meta( $active_rule_id );
	$cs_type           = isset( $metas[ '_cs_type' ][ 0 ] ) ? $metas[ '_cs_type' ][ 0 ] : false;
	if ( $cs_type == 'ask_before_tracking' || $cs_type == 'inform_and_opiout' ) {
		$Content .= '<a href="#" class="cs-info-sticky-button shortcode">' . $content . '</a>';
	} else if ( $cs_hide_shortcode == 'show' ) {
		$Content .= '<a href="#" class="cs-open-consent shortcode" data-id="' . $active_rule_id . '">' . $content . '</a>';
	}

	$Content .= '</div>';

	return $Content;
}

add_shortcode( 'cm_options', 'ConsentMagicPro\cm_options_shortcode' );

// it will add a link that opens the privacy page configured inside the plugin.
function cm_privacy_link_shortcode( $atts, $content ) {
	$wp_current_lang = get_locale();
	$policy_link = get_permalink( renderPrivacyPolicyPage( $wp_current_lang ) );
	$Content     = '<div class="cm_shortcode_container" style="' . esc_attr( get_shortcode_styles() ) . '">';
	$Content     .= '<a href="' . esc_url( $policy_link ) . '">' . $content . '</a>';
	$Content     .= '</div>';

	return $Content;
}

add_shortcode( 'cm_privacy_link', 'ConsentMagicPro\cm_privacy_link_shortcode' );

// it will display a list with cookies/scripts categories
function cm_categories_shortcode( $atts ) {
	$terms_primary = get_cookies_terms_objects( 'cs_necessary_term', true );
	$terms         = get_cookies_terms_objects( 'cs_necessary_term' );
	$fb_cat        = ConsentMagic()->getOption( ConsentMagic()->getOption( 'cs_block_fb_pixel_scripts_cat' ) . '_cat_id' );
	$Content       = '<div class="cm_shortcode_container" style="' . esc_attr( get_shortcode_styles() ) . '">';
	$Content       .= '<ul>';
	if ( $terms_primary ) {
		foreach ( $terms_primary as $term ) {
			if ( (int) get_term_meta( $term->term_id, 'cs_ignore_this_category', true ) == 0 ) {
				$Content .= '<li>' . esc_html__( $term->name, 'consent-magic' );
				if ( isPYSActivated() ) {
					if ( $fb_cat == $term->term_id ) {
						$Content .= '<ul>';
						if ( ConsentMagic()->getOption( 'cs_advanced_matching_consent_enabled' ) ) {
							$Content .= '<li>' . esc_html__( 'Advanced Matching', 'consent-magic' ) . '</li>';
						}
						if ( ConsentMagic()->getOption( 'cs_server_side_consent_enabled' ) ) {
							$Content .= '<li>' . esc_html__( 'Server-side events', 'consent-magic' ) . '</li>';
						}
						$Content .= '</ul>';
					}
				}
				$Content .= '</li>';
			}
		}
	}
	if ( $terms ) {
		foreach ( $terms as $term ) {
			if ( (int) get_term_meta( $term->term_id, 'cs_ignore_this_category', true ) == 0 ) {
				$Content .= '<li>' . esc_html__( $term->name, 'consent-magic' );
				if ( isPYSActivated() ) {
					if ( $fb_cat == $term->term_id ) {
						$Content .= '<ul>';
						if ( ConsentMagic()->getOption( 'cs_advanced_matching_consent_enabled' ) ) {
							$Content .= '<li>' . esc_html__( 'Advanced Matching', 'consent-magic' ) . '</li>';
						}
						if ( ConsentMagic()->getOption( 'cs_server_side_consent_enabled' ) ) {
							$Content .= '<li>' . esc_html__( 'Server-side events', 'consent-magic' ) . '</li>';
						}
						$Content .= '</ul>';
					}
				}
				$Content .= '</li>';
			}
		}
	}
	$Content .= '</ul>';
	$Content .= '</div>';

	return $Content;
}

add_shortcode( 'cm_categories', 'ConsentMagicPro\cm_categories_shortcode' );

add_shortcode( 'disable_cm', 'ConsentMagicPro\disable_cm_shortcode' );

// it will add a link that opens the privacy page configured inside the plugin.
function disable_cm_shortcode( $atts, $content ) {
	return '<div class="disable_cm_wrap" ><input type="hidden" class="disable_cm" /></div>';
}

// it will display a list with scan cookies/scripts
function cm_scan_shortcode( $atts ) {
	$scan_scripts = get_scan_scripts_list();
	$scan_cookies = get_scan_cookies_list();
	$Content      = '<div class="cm_shortcode_container" style="' . esc_attr( get_shortcode_styles() ) . '">';
	if ( $scan_scripts ) {
		$term_sort = getDataByKey( $scan_scripts, 'term' );
		$Content   .= '<h3 style="margin-left: 15px;">' . esc_html__( 'Scripts', 'consent-magic' ) . '</h3>';
		$Content   .= '<div style="margin-left: 15px;">';
		foreach ( $term_sort as $key => $val ) {
			$Content .= '<div><h4>' . esc_html__( $key, 'consent-magic' ) . '</h4></div>';
			$Content .= '<ul>';
			foreach ( $val as $script ) {
				$Content .= '<li><strong>' . esc_html__( $script[ 'script_name' ], 'consent-magic' ) . '</strong>';
				$Content .= '</li>';
			}
			$Content .= '</ul>';
		}
		$Content .= '</div>';
	}
	if ( $scan_cookies ) {
		$term_sort = getDataByKey( $scan_cookies, 'term' );
		$Content   .= '<h3 style="margin-left: 15px;">' . esc_html__( 'Cookies', 'consent-magic' ) . '</h3>';
		$Content   .= '<div style="margin-left: 15px;">';
		foreach ( $term_sort as $key => $val ) {
			$Content .= '<div><h4>' . esc_html__( $key, 'consent-magic' ) . '</h4></div>';
			$Content .= '<ul>';
			foreach ( $val as $cookie ) {
				$Content .= '<li><strong>' . esc_html__( $cookie[ 'cookie_name' ], 'consent-magic' ) . '</strong>';
				$Content .= '</li>';
			}
			$Content .= '</ul>';
		}
		$Content .= '</div>';
	}
	$Content .= '</div>';

	return $Content;
}

add_shortcode( 'cm_scan', 'ConsentMagicPro\cm_scan_shortcode' );

// it will display a list with scan cookies
function cm_scan_cookies_shortcode( $atts ) {
	$scan_cookies = get_scan_cookies_list();
	$Content      = '<div class="cm_shortcode_container" style="' . esc_attr( get_shortcode_styles() ) . '">';
	if ( $scan_cookies ) {
		$term_sort = getDataByKey( $scan_cookies, 'term' );
		$Content   .= '<h3 style="margin-left: 15px;">' . esc_html__( 'Cookies', 'consent-magic' ) . '</h3>';
		$Content   .= '<div style="margin-left: 15px;">';
		foreach ( $term_sort as $key => $val ) {
			$Content .= '<div><h4>' . esc_html__( $key, 'consent-magic' ) . '</h4></div>';
			$Content .= '<ul>';
			foreach ( $val as $cookie ) {
				$Content .= '<li><strong>' . esc_html__( $cookie[ 'cookie_name' ], 'consent-magic' ) . '</strong>';
				$Content .= '</li>';
			}
			$Content .= '</ul>';
		}
		$Content .= '</div>';
	}
	$Content .= '</div>';

	return $Content;
}

add_shortcode( 'cm_scan_cookies', 'ConsentMagicPro\cm_scan_cookies_shortcode' );

// it will display a list with scan scripts
function cm_scan_scripts_shortcode( $atts ) {
	$scan_scripts = get_scan_scripts_list();
	$Content      = '<div class="cm_shortcode_container" style="' . esc_attr( get_shortcode_styles() ) . '">';
	if ( $scan_scripts ) {
		$term_sort = getDataByKey( $scan_scripts, 'term' );
		$Content   .= '<h3 style="margin-left: 15px;">' . esc_html__( 'Scripts', 'consent-magic' ) . '</h3>';
		$Content   .= '<div style="margin-left: 15px;">';
		foreach ( $term_sort as $key => $val ) {
			$Content .= '<div><h4>' . esc_html__( $key, 'consent-magic' ) . '</h4></div>';
			$Content .= '<ul>';
			foreach ( $val as $script ) {
				$Content .= '<li><strong>' . esc_html__( $script[ 'script_name' ], 'consent-magic' ) . '</strong>';
				$Content .= '</li>';
			}
			$Content .= '</ul>';
		}
		$Content .= '</div>';
	}
	$Content .= '</div>';

	return $Content;
}

add_shortcode( 'cm_scan_scripts', 'ConsentMagicPro\cm_scan_scripts_shortcode' );

// it will display a list with scan cookies/scripts
function cm_scan_description_shortcode( $atts ) {
	$scan_scripts = get_scan_scripts_list();
	$scan_cookies = get_scan_cookies_list();
	$Content      = '<div class="cm_shortcode_container" style="' . esc_attr( get_shortcode_styles() ) . '">';
	if ( $scan_scripts ) {
		$term_sort = getDataByKey( $scan_scripts, 'term' );
		$Content   .= '<h3 style="margin-left: 15px;">' . esc_html__( 'Scripts', 'consent-magic' ) . '</h3>';
		$Content   .= '<div style="margin-left: 15px;">';
		foreach ( $term_sort as $key => $val ) {
			$Content .= '<div><h4>' . esc_html__( $key, 'consent-magic' ) . '</h4></div>';
			$Content .= '<ul>';
			foreach ( $val as $script ) {
				$Content .= '<li><strong>' . esc_html__( $script[ 'script_name' ], 'consent-magic' ) . '</strong>';
				$Content .= '<p>' . esc_html__( $script[ 'description' ], 'consent-magic' ) . '</p>';
				$Content .= '</li>';
			}
			$Content .= '</ul>';
		}
		$Content .= '</div>';
	}
	if ( $scan_cookies ) {
		$term_sort = getDataByKey( $scan_cookies, 'term' );
		$Content   .= '<h3 style="margin-left: 15px;">' . esc_html__( 'Cookies', 'consent-magic' ) . '</h3>';
		$Content   .= '<div style="margin-left: 15px;">';
		foreach ( $term_sort as $key => $val ) {
			$Content .= '<div><h4>' . esc_html__( $key, 'consent-magic' ) . '</h4></div>';
			$Content .= '<ul>';
			foreach ( $val as $cookie ) {
				$Content .= '<li><strong>' . esc_html__( $cookie[ 'cookie_name' ], 'consent-magic' ) . '</strong>';
				$Content .= '<p>' . esc_html__( $cookie[ 'description' ], 'consent-magic' ) . '</p>';
				$Content .= '</li>';
			}
			$Content .= '</ul>';
		}
		$Content .= '</div>';
	}
	$Content .= '</div>';

	return $Content;
}

add_shortcode( 'cm_scan_description', 'ConsentMagicPro\cm_scan_description_shortcode' );

// it will display a list with scan cookies
function cm_scan_cookies_description_shortcode( $atts ) {
	$scan_cookies = get_scan_cookies_list();
	$Content      = '<div class="cm_shortcode_container" style="' . esc_attr( get_shortcode_styles() ) . '">';
	if ( $scan_cookies ) {
		$term_sort = getDataByKey( $scan_cookies, 'term' );
		$Content   .= '<h3 style="margin-left: 15px;">' . esc_html__( 'Cookies', 'consent-magic' ) . '</h3>';
		$Content   .= '<div style="margin-left: 15px;">';
		foreach ( $term_sort as $key => $val ) {
			$Content .= '<div><h4>' . esc_html__( $key, 'consent-magic' ) . '</h4></div>';
			$Content .= '<ul>';
			foreach ( $val as $cookie ) {
				$Content .= '<li><strong>' . esc_html__( $cookie[ 'cookie_name' ], 'consent-magic' ) . '</strong>';
				$Content .= '<p>' . esc_html__( $cookie[ 'description' ], 'consent-magic' ) . '</p>';
				$Content .= '</li>';
			}
			$Content .= '</ul>';
		}
		$Content .= '</div>';
	}
	$Content .= '</div>';

	return $Content;
}

add_shortcode( 'cm_scan_cookies_description', 'ConsentMagicPro\cm_scan_cookies_description_shortcode' );

// it will display a list with scan scripts
function cm_scan_scrips_description_shortcode( $atts ) {
	$scan_scripts = get_scan_scripts_list();
	$Content      = '<div class="cm_shortcode_container" style="' . esc_attr( get_shortcode_styles() ) . '">';
	if ( $scan_scripts ) {
		$term_sort = getDataByKey( $scan_scripts, 'term' );
		$Content   .= '<h3 style="margin-left: 15px;">' . esc_html__( 'Scripts', 'consent-magic' ) . '</h3>';
		$Content   .= '<div style="margin-left: 15px;">';
		foreach ( $term_sort as $key => $val ) {
			$Content .= '<div><h4>' . esc_html__( $key, 'consent-magic' ) . '</h4></div>';
			$Content .= '<ul>';
			foreach ( $val as $script ) {
				$Content .= '<li><strong>' . esc_html__( $script[ 'script_name' ], 'consent-magic' ) . '</strong>';
				$Content .= '<p>' . esc_html__( $script[ 'description' ], 'consent-magic' ) . '</p>';
				$Content .= '</li>';
			}
			$Content .= '</ul>';
		}
		$Content .= '</div>';
	}
	$Content .= '</div>';

	return $Content;
}

add_shortcode( 'cm_scan_scrips_description', 'ConsentMagicPro\cm_scan_scrips_description_shortcode' );

/**
 * Get styles for shortcodes
 * @return string
 */
function get_shortcode_styles() {
	$active_rule_id = ConsentMagic()->get_active_rule_id();
	$styles         = '';
	if ( !empty( $active_rule_id ) ) {
		$theme_id   = get_post_meta( $active_rule_id, '_cs_theme', true );
		$text_color = get_post_meta( $theme_id, 'cs_shortcodes_text_color', true );
		$styles     = '--cs-shortcode-text-color: ' . $text_color . ';';
	}

	return $styles;
}

/**
 * Remove sticky shortcode
 */
add_shortcode( 'cm_remove_sticky', 'ConsentMagicPro\remove_sticky_shortcode' );
function remove_sticky_shortcode() {
	return '<div class="cm_remove_sticky_wrap" ><input type="hidden" class="cs_remove_sticky" /></div>';
}