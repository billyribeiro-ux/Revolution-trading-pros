<?php
/**
 * Compatibility with PixelYourSite
 * @link  https://www.pixelyoursite.com/plugins/consentmagic/
 * @since 1.0.0
 */

namespace ConsentMagicPro;

if ( !defined( 'ABSPATH' ) ) {
	exit; // Exit if accessed directly.
}
wp_cookie_constants();

class CS_PixelYourSite {

	/**
	 * $_instance
	 */
	private static $_instance = null;

	public $url;

	public $post_id;

	public $post_content;

	public $page_content;

	public $current_user;

	/**
	 * $_instance CS_PixelYourSite
	 * @return CS_PixelYourSite|null
	 */
	public static function instance() {
		if ( is_null( self::$_instance ) ) {
			self::$_instance = new self();
		}

		return self::$_instance;
	}

	public function __construct() {
		if ( !isPYSActivated() || ( is_admin() && !wp_doing_ajax() ) ) {
			return;
		}

		$active_rule_id = ConsentMagic()->get_active_rule_id();

		if ( ConsentMagic()->get_unblocked_crawlers() || empty( $active_rule_id ) ) {
			return;
		}

		if ( !$this->is_plugin_active() ) {
			return;
		}

		$cs_type = get_post_meta( $active_rule_id, '_cs_type', true );
		if ( !$cs_type ) {
			return;
		}

		$test_prefix   = ConsentMagic()->getOption( 'cs_test_mode' ) ? '_test' : '';
		$viewed_cookie = "cs_viewed_cookie_policy" . $test_prefix;

		if ( ConsentMagic()->getOption( 'cs_plugin_activation' ) != 1
		     || ConsentMagic()->getOption( 'cs_script_blocking_enabled' ) != 1
		     || ConsentMagic()->getOption( 'cs_block_pre_defined_scripts' ) != 1 ) {
			return;
		}

		$cs_track_analytics = ( ConsentMagic()->getOption( 'cs_block_pre_defined_scripts' ) == 1
		                        && !CS_Cookies()->getCookie( $viewed_cookie ) ) ? get_post_meta( $active_rule_id, '_cs_track_analytics', true ) : false;

		$native_scripts_option = get_post_meta( $active_rule_id, '_cs_native_scripts', true );
		$native_scripts        = ( $cs_type == 'iab' && $native_scripts_option == 1 ) || $cs_type != 'iab';

		if ( $this->add_main_filter() || $cs_type == 'just_inform' || !$native_scripts ) {
			return;
		}

		$script_list = CS_Script_Blocker()->get_cs_blocker_script_list();
		if ( empty( $script_list ) ) {
			return;
		}
		$analytics_cat_id = ConsentMagic()->getOption( 'analytics_cat_id' );

		foreach ( $script_list as $scriptkey => $script_data ) {

			if ( $script_data[ 'activation' ] != 1 ) {
				continue;
			}

			$category_cookie = "cs_enabled_cookie_term{$test_prefix}_{$script_data['cat_id']}";
			$ignore          = (int) get_term_meta( $script_data[ 'cat_id' ], 'cs_ignore_this_category', true );

			$enable_script  = false;
			$disable_script = false;

			if ( $script_data[ 'category_slug' ] == 'necessary' ) {
				$enable_script = true;
			} elseif ( $cs_track_analytics && $script_data[ 'cat_id' ] == $analytics_cat_id ) {
				$enable_script = true;
			} elseif ( !CS_Cookies()->getCookie( $viewed_cookie ) && $cs_type == 'inform_and_opiout' ) {
				$enable_script = true;
			} elseif ( !CS_Cookies()->getCookie( $viewed_cookie )
			           && ( $cs_type == 'ask_before_tracking' || $cs_type == 'iab' ) ) {
				$disable_script = true;
			} elseif ( CS_Cookies()->getCookie( $category_cookie ) && CS_Cookies()->getCookie( $category_cookie ) == 'no' && $ignore == 0 ) {
				$disable_script = true;
			} else {
				$enable_script = true;
			}

			if ( $enable_script ) {
				$this->add_script_filter( $scriptkey, false );
			} elseif ( $disable_script ) {
				$this->add_script_filter( $scriptkey, true );
			}
		}
	}

	private function add_script_filter( $scriptkey, $disable ) {
		$filter_map = array(
			'facebook_pixel'  => 'pys_disable_facebook_by_gdpr',
			'googleanalytics' => 'pys_disable_analytics_by_gdpr',
			'google_ads_tag'  => 'pys_disable_google_ads_by_gdpr',
			'pinterest'       => 'pys_disable_pinterest_by_gdpr',
			'bing_tag'        => 'pys_disable_bing_by_gdpr',
			'tiktok_pixel'    => 'pys_disable_tiktok_by_gdpr',
			'reddit_pixel'    => 'pys_disable_reddit_by_gdpr',
		);

		if ( isset( $filter_map[ $scriptkey ] ) ) {
			add_filter( $filter_map[ $scriptkey ], $disable ? '__return_true' : '__return_false', 10, 2 );
		}
	}

	/**
	 * Add main filter based on GDPR main cookie (accept/reject)
	 * @return bool
	 */
	private function add_main_filter(): bool {

		// get or make permalink
		$http_host   = sanitize_text_field( $_SERVER[ 'HTTP_HOST' ] ?? '' );
		$request_uri = sanitize_text_field( $_SERVER[ 'REQUEST_URI' ] ?? '' );
		$this->url   = !empty( get_the_permalink() ) ? get_the_permalink() : ( isset( $_SERVER[ 'HTTPS' ] ) ? "https" : "http" )
		                                                                     . "://" . $http_host . $request_uri;
		$url_path    = parse_url( $this->url, PHP_URL_PATH );
		if ( $url_path ) {
			$slug               = pathinfo( $url_path, PATHINFO_BASENAME );
			$this->post_id      = ConsentMagic::getCurrentPostID( $slug );
			$this->post_content = get_post( $this->post_id );
			if ( $this->post_content ) {
				$this->page_content = $this->post_content->post_content;
			}
		}


		$script_list   = CS_Script_Blocker()->get_cs_blocker_script_list();
		$ignore_action = 0;
		$necessary     = 0;
		if ( !empty( $script_list ) ) {
			foreach ( $script_list as $k => $v ) {
				$ignore = (int) get_term_meta( $v[ 'cat_id' ], 'cs_ignore_this_category', true );
				if ( $ignore == 1 ) {
					$ignore_action = 1;
					break;
				}

				if ( $v[ 'category_slug' ] == 'necessary' ) {
					$necessary = 1;
					break;
				}
			}
		}

		$test_prefix        = false;
		$cs_track_analytics = false;
		if ( ConsentMagic()->getOption( 'cs_test_mode' ) ) {
			$test_prefix = '_test';
		}
		$viewed_cookie  = "cs_viewed_cookie_policy" . $test_prefix;
		$active_rule_id = ConsentMagic()->cs_options[ 'active_rule_id' ];
		$cs_type        = get_post_meta( $active_rule_id, '_cs_type', true );

		if ( ConsentMagic()->getOption( 'cs_block_pre_defined_scripts' ) == 1
		     && !CS_Cookies()->getCookie( $viewed_cookie )
		     && $cs_type == 'ask_before_tracking' ) {
			$cs_track_analytics = get_post_meta( $active_rule_id, '_cs_track_analytics', true );
		}

		$native_scripts_option = get_post_meta( $active_rule_id, '_cs_native_scripts', true );
		$native_scripts        = ( $cs_type == 'iab' && $native_scripts_option == 1 ) || $cs_type != 'iab';

		$out_fn = '__return_true'; //block it
		$out    = true;

		if ( $cs_type
		     && $ignore_action == 0
		     && $necessary == 0
		     && ConsentMagic()->getOption( 'cs_block_pre_defined_scripts' ) == 1 ) {

			if ( !CS_Cookies()->getCookie( $viewed_cookie ) && !$cs_track_analytics ) {
				if ( $cs_type != 'just_inform' && $cs_type != 'inform_and_opiout' && $native_scripts ) {
					$out_fn = '__return_true'; //add blocking
					$out    = true;
				} else {
					$out_fn = '__return_false'; //remove blocking
					$out    = false;
				}
			} else {
				if ( CS_Cookies()->getCookie( $viewed_cookie ) && CS_Cookies()->getCookie( $viewed_cookie ) == 'no' ) {
					if ( $cs_type != 'just_inform' ) {
						$out_fn = '__return_true'; //add blocking
						$out    = true;
					} else {
						$out_fn = '__return_false'; //remove blocking
						$out    = false;
					}
				} else {
					$out_fn = '__return_false'; //remove blocking
					$out    = false;
				}
			}

		} else {
			$out_fn = '__return_false'; //remove blocking
			$out    = false;
		}

		$user         = wp_get_current_user();
		$ignore_roles = ConsentMagic()->getOption( 'cs_admin_ignore_users' );
		$unblocked_ip = ConsentMagic()->get_unblocked_user_ip();
		$ip           = get_client_ip();
		if ( isset( $user->roles ) && $user->ID !== 0 && is_array( $ignore_roles ) ) {
			if ( !empty( array_intersect( $ignore_roles, $user->roles ) ) ) {
				$out_fn = '__return_false'; //remove blocking
				$out    = false;
			}
		} else if ( isset( $user->roles ) && $user->ID == 0 && is_array( $ignore_roles ) ) {
			if ( !empty( array_intersect( $ignore_roles, array( 'visitor' ) ) ) ) {
				$out_fn = '__return_false'; //remove blocking
				$out    = false;
			}
		}

		if ( ( isset( $unblocked_ip ) && is_array( $unblocked_ip ) ) ) {
			if ( !empty( array_intersect( $unblocked_ip, array( $ip ) ) ) ) {
				$out_fn = '__return_true'; //add blocking
				$out    = true;
			} else {
				$out_fn = '__return_false'; //remove blocking
				$out    = false;
			}
		}

		if ( $this->page_content ) {
			if ( !has_shortcode( $this->page_content, 'disable_cm' ) ) {
				add_filter( 'pys_disable_by_gdpr', $out_fn, 10, 2 );
			} else {
				add_filter( 'pys_disable_by_gdpr', '__return_false', 10, 2 );
				$out = false;
			}
		} else {
			add_filter( 'pys_disable_by_gdpr', $out_fn, 10, 2 );
		}

		return $out;
	}


	/**
	 * Checks PixelYourSite plugin is active
	 * @since 1.0.0
	 */
	private function is_plugin_active() {
		if ( !function_exists( 'is_plugin_active' ) ) {
			include_once ABSPATH . 'wp-admin/includes/plugin.php';
		}

		return is_plugin_active( 'pixelyoursite-pro/pixelyoursite-pro.php' )
		       || is_plugin_active( 'pixelyoursite/facebook-pixel-master.php' );
	}
}

/**
 * @return CS_PixelYourSite
 */
function CS_PixelYourSite() {
	return CS_PixelYourSite::instance();
}

CS_PixelYourSite();