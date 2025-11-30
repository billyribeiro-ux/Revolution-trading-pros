<?php

/**
 * Blocking Facebook For Woocommerce Actions
 * @link  https://www.pixelyoursite.com/plugins/consentmagic/
 * @since 1.9.2
 */

namespace ConsentMagicPro;

if ( !defined( 'ABSPATH' ) ) {
	exit; // Exit if accessed directly.
}

class CS_Facebook_For_Woocommerce {

	private static $_instance = null;

	/**
	 * $_instance CS_Facebook_For_Woocommerce
	 * @return CS_Facebook_For_Woocommerce|null
	 */
	public static function instance() {
		if ( is_null( self::$_instance ) ) {
			self::$_instance = new self();
		}

		return self::$_instance;
	}

	/**
	 * @constructor
	 */
	public function __construct() {

		//if bot or not active rule - script not working
		if ( ConsentMagic()->get_unblocked_crawlers() || empty( ConsentMagic()->get_active_rule_id() ) ) {
			return;
		}

		if ( ( ConsentMagic()->getOption( 'cs_plugin_activation' ) == 1 && ConsentMagic()->getOption( 'cs_script_blocking_enabled' ) == 1 ) || ( ConsentMagic()->getOption( 'cs_enable_site_cache' ) == 1 && ConsentMagic()->getOption( 'cs_plugin_activation' ) == 1 ) ) {
			if ( $this->is_plugin_active() && !isPYSActivated() && !is_admin() ) {

				$status = $this->check_blocking_status();

				// check cache & enabled control CAPI events for FB
				if ( (int) ConsentMagic()->getOption( 'cs_enable_site_cache' ) === 1 ) {

					if ( (int) ConsentMagic()->getOption( 'cs_fb_woo_capi_enabled' ) === 1 && !$status ) {

						add_action( 'woocommerce_after_single_product', array(
							$this,
							'cs_remove_action'
						), 1 );
						add_action( 'woocommerce_after_shop_loop', array(
							$this,
							'cs_remove_action'
						), 1 );
						add_action( 'pre_get_posts', array(
							$this,
							'cs_remove_action'
						), 1 );
						add_action( 'woocommerce_redirect_single_search_result', array(
							$this,
							'cs_remove_action'
						), 1 );
						add_action( 'woocommerce_add_to_cart', array(
							$this,
							'cs_remove_action'
						), 1 );
						add_action( 'woocommerce_ajax_added_to_cart', array(
							$this,
							'cs_remove_action'
						), 1 );
						add_action( 'woocommerce_add_to_cart_redirect', array(
							$this,
							'cs_remove_action'
						), 1 );
						add_action( 'woocommerce_ajax_added_to_cart', array(
							$this,
							'cs_remove_action'
						), 1 );
						add_action( 'woocommerce_after_cart', array(
							$this,
							'cs_remove_action'
						), 1 );
						add_action( 'woocommerce_after_checkout_form', array(
							$this,
							'cs_remove_action'
						), 1 );
						add_action( 'woocommerce_blocks_checkout_enqueue_data', array(
							$this,
							'cs_remove_action'
						), 1 );
						add_action( 'woocommerce_checkout_update_order_meta', array(
							$this,
							'cs_remove_action'
						), 1 );
						add_action( 'woocommerce_thankyou', array(
							$this,
							'cs_remove_action'
						), 1 );
						add_action( 'woocommerce_store_api_checkout_update_order_meta', array(
							$this,
							'cs_remove_action'
						), 1 );
						add_action( 'woocommerce_blocks_checkout_update_order_meta', array(
							$this,
							'cs_remove_action'
						), 1 );
						add_action( '__experimental_woocommerce_blocks_checkout_update_order_meta', array(
							$this,
							'cs_remove_action'
						), 1 );
						add_action( 'wpcf7_contact_form', array(
							$this,
							'cs_remove_action'
						), 1 );
						add_action( 'shutdown', array(
							$this,
							'cs_remove_action'
						), 1 );
					}

				} elseif ( (int) ConsentMagic()->getOption( 'cs_fb_woo_capi_enabled' ) === 1 ) {
					add_filter( 'facebook_for_woocommerce_integration_pixel_enabled', $status ? '__return_true' : '__return_false' );
				}
			}
		}
	}

	/**
	 * Remove actions from plugin
	 * @return void
	 */
	public function cs_remove_action() {

		$this->execute_removing_action( 'WC_Facebookcommerce_EventsTracker' );

	}

	/**
	 * Execute removing action from plugin
	 * @return void
	 */
	public function execute_removing_action( $class_name, $action_name = null ) {

		global $wp_filter;

		if ( null === $action_name ) {
			$action_name = current_action();
		}

		$hooks     = $wp_filter[ $action_name ];
		$callbacks = $hooks->callbacks;

		foreach ( $callbacks as $priority => $actions ) {
			foreach ( $actions as $action ) {
				$function = $action[ 'function' ];
				if ( is_array( $function ) && is_object( $function[ 0 ] ) && ( $class_name === get_class( $function[ 0 ] ) ) ) {
					remove_action( $action_name, $function, $priority );
				}
			}
		}
	}


	/**
	 * check block/unblock status for Facebook Woocommerce
	 * @return bool
	 */
	public function check_blocking_status() {

		$active_rule_id = ConsentMagic()->cs_options[ 'active_rule_id' ];

		$cs_type = get_post_meta( $active_rule_id, '_cs_type', true );
		if ( $cs_type ) {
			$test_prefix        = false;
			$cs_track_analytics = false;
			if ( ConsentMagic()->getOption( 'cs_test_mode' ) ) {
				$test_prefix = '_test';
			}
			$viewed_cookie = "cs_viewed_cookie_policy" . $test_prefix;
			if ( ConsentMagic()->getOption( 'cs_block_pre_defined_scripts' ) == 1 && !CS_Cookies()->getCookie( $viewed_cookie ) ) {
				$cs_track_analytics = get_post_meta( $active_rule_id, '_cs_track_analytics', true );
			}

			if ( ( ConsentMagic()->getOption( 'cs_plugin_activation' ) == 1 && ConsentMagic()->getOption( 'cs_script_blocking_enabled' ) == 1 ) || ( ConsentMagic()->getOption( 'cs_enable_site_cache' ) == 1 && ConsentMagic()->getOption( 'cs_plugin_activation' ) == 1 ) ) {

				$native_scripts_option = get_post_meta( $active_rule_id, '_cs_native_scripts', true );
				$native_scripts        = ( $cs_type == 'iab' && $native_scripts_option == 1 ) || $cs_type != 'iab';

				if ( $cs_type != 'just_inform' && $native_scripts ) {

					$script_list = CS_Script_Blocker()->get_cs_blocker_script_list();
					if ( !empty( $script_list ) ) {
						$analytics_cat_id = ConsentMagic()->getOption( 'analytics_cat_id' );
						foreach ( $script_list as $script_key => $script_item ) {

							if ( $script_key == 'facebook_pixel' ) {
								$category_cookie = "cs_enabled_cookie_term" . $test_prefix . '_' . $script_item[ 'cat_id' ];
								$ignore          = (int) get_term_meta( $script_item[ 'cat_id' ], 'cs_ignore_this_category', true );

								if ( $cs_track_analytics && $script_item[ 'cat_id' ] == $analytics_cat_id ) {
									//enable tracking
									return true;

								} else if ( !CS_Cookies()->getCookie( $viewed_cookie ) && ( $cs_type == 'inform_and_opiout' ) ) {
									// first time enable tracking for inform_and_opiout
									return true;

								} else {
									//user is disabled the checkbox
									if ( ( (CS_Cookies()->getCookie( $category_cookie ) && CS_Cookies()->getCookie( $category_cookie ) == 'no' && $ignore == 0 ) ) || !CS_Cookies()->getCookie( $viewed_cookie ) ) {
										//block tracking
										return false;
									}
								}
							}
						}
					}
				}
			}
		}

		return true;
	}

	/**
	 * Check activate plugin
	 * @return bool
	 */
	public function is_plugin_active() {
		if ( is_plugin_active( 'facebook-for-woocommerce/facebook-for-woocommerce.php' ) ) {
			return true;
		} else {
			return false;
		}
	}
}

function CS_Facebook_For_Woocommerce() {
	return CS_Facebook_For_Woocommerce::instance();
}

CS_Facebook_For_Woocommerce();
