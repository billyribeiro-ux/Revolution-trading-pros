<?php

/**
 * Integrations plugin compatibility.
 * @link       https://www.pixelyoursite.com/plugins/consentmagic/
 * @since      1.0.0
 * @package    ConsentMagic
 * @subpackage ConsentMagic/includes
 */

namespace ConsentMagicPro;

class CS_Integrations {

	/**
	 * scripts list, Script folder and main file must be same as that of script name
	 * Please check the `register_scripts` method for more details
	 */

	private $common_modules = array(
		'traits' => array(
			'CS_Advanced_Consent_Mode',
			'CS_Throttle',
		),
	);

	private $blockers = array(
		'CS_SBlocker_Module'
	);

	private $third_party_scripts = array(
		'CS_PixelYourSite',
		'CS_Elementor',
		'CS_Facebook_For_Woocommerce'
	);

	public static $existing_scripts = array();

	/**
	 * admin module list, Module folder and main file must be same as that of module name
	 * Please check the `admin_modules` method for more details
	 */
	private $admin_modules = array(
		array(
			'title'  => 'scanner',
			'module' => 'CS_Scanner_Module'
		)
	);

	private $modules = array(
		array(
			'title'     => 'iab_integration',
			'module'    => 'CS_IAB_Integration',
			'construct' => false
		),
		array(
			'title'     => 'google_consent_mode',
			'module'    => 'CS_Google_Consent_Mode',
			'construct' => true
		),
		array(
			'title'     => 'bing_consent_mode',
			'module'    => 'CS_Bing_Consent_Mode',
			'construct' => true
		),
		array(
			'title'     => 'reddit_ldu',
			'module'    => 'CS_Reddit_LDU',
			'construct' => true
		),
		array(
			'title'     => 'url_passthrough_mode',
			'module'    => 'CS_Url_Passthrough_Mode',
			'construct' => true
		),
		array(
			'title'     => 'meta_ldu',
			'module'    => 'CS_Meta_LDU',
			'construct' => true
		),
		array(
			'title'     => 'wp_consent_api',
			'module'    => 'CS_WP_Consent_Api',
			'construct' => false
		),
		array(
			'title'     => 'conversion_exporter',
			'module'    => 'CS_Conversion_Exporter',
			'construct' => true
		),
	);

	public static $existing_modules = array();

	/**
	 * Initialize the class and set its properties.
	 * @since    1.0.0
	 */
	public function __construct() {
		$this->register_common_modules();
		$this->register_scripts();
		self::$existing_modules = array();
		$this->admin_modules();
	}

	public function register_common_modules() {
		foreach ( $this->common_modules as $name => $modules ) {
			foreach ( $modules as $module ) {
				$path = plugin_dir_path( __FILE__ ) . "common/$name/$module.php";
				if ( file_exists( $path ) ) {
					require_once $path;
				}

			}
		}
	}

	/**
	 * Registering integrations scripts
	 */
	public function register_scripts() {

		//register modules
		foreach ( $this->modules as $module ) {
			$module_title = $module[ 'title' ];
			$module_item  = $module[ 'module' ];
			$module_file  = plugin_dir_path( __FILE__ ) . "modules/$module_title/$module_item.php";
			if ( file_exists( $module_file ) ) {
				require_once $module_file;

				if ( $module[ 'construct' ] ) {
					call_user_func( [ "\ConsentMagicPro\\$module_item", 'instance' ] );
				}
			}
		}

		//loop through script list and include its file
		if ( ConsentMagic()->ready_to_run() && $this::check_showing() ) {
			$scripts = array();
			foreach ( $this->blockers as $blocker ) {
				$blocker_file = plugin_dir_path( __FILE__ ) . "scripts/cs-script-blocker/$blocker.php";
				if ( file_exists( $blocker_file ) ) {
					$scripts[] = $blocker;
					require_once $blocker_file;
				}
			}
			self::$existing_scripts = $scripts; //this is for module_exits checking

			//include third party scripts
			if ( !empty( ConsentMagic()->get_active_rule_id() ) ) {
				foreach ( $this->third_party_scripts as $script ) {
					$script_file = plugin_dir_path( __FILE__ ) . "scripts/third-party/$script.php";
					if ( file_exists( $script_file ) ) {
						require_once $script_file;
					}
				}
			}
		}
	}

	public static function check_showing() {
		static $cached_show = null;
		if ( $cached_show !== null ) {
			return $cached_show;
		}

		$show         = false;
		$page_content = null;

		//if bot - script not working
		if ( ConsentMagic()->get_unblocked_crawlers() ) {
			return $cached_show = $show;
		}

		$user         = wp_get_current_user();
		$ignore_roles = ConsentMagic()->getOption( 'cs_admin_ignore_users' );
		// get or make permalink
		$HTTP_REFERER = isset( $_SERVER[ 'HTTP_REFERER' ] ) ? sanitize_text_field( $_SERVER[ 'HTTP_REFERER' ] ) : false;

		if ( isset( $HTTP_REFERER ) && strstr( $HTTP_REFERER, 'wp-admin' ) ) {
			$show = true;
		} else {
			$url = $HTTP_REFERER;
			// get post_id using url/permalink
			global $wp_rewrite, $wp;
			if ( $wp_rewrite === null || !isset( $wp->public_query_vars ) ) {
				$url_path = parse_url( $url, PHP_URL_PATH );
				$slug     = pathinfo( $url_path ?: '', PATHINFO_BASENAME );
				$post_id = ConsentMagic::getCurrentPostID( $slug );

			} else {
				if ( function_exists( 'icl_object_id' ) ) {
					$current_language = apply_filters( 'wpml_current_language', null );
					$post_id = apply_filters( 'wpml_object_id', get_the_id(), get_post_type(), true, $current_language );
				} else {
					$post_id = url_to_postid( $url );
				}
			}

			$post_content = get_post( $post_id );
			if ( $post_content ) {
				$page_content = $post_content->post_content;
			}

			$home_url = get_home_url();
			if ( empty( $page_content ) || !has_shortcode( $page_content, 'disable_cm' ) || untrailingslashit( $home_url ) == untrailingslashit( $url ) ) {
				$show = true;
			}
		}

		$unblocked_ip = ConsentMagic()->get_unblocked_user_ip();
		$ip           = get_client_ip();

		if ( ( isset( $ignore_roles ) && is_array( $ignore_roles ) && isset( $user->roles ) && array_intersect( $ignore_roles, $user->roles ) ) || ( isset( $ignore_roles ) && is_array( $ignore_roles ) && isset( $user ) && array_intersect( $ignore_roles, array( 'visitor' ) ) && $user->ID == 0 ) || ( isset( $unblocked_ip ) && is_array( $unblocked_ip ) && array_intersect( $unblocked_ip, array( $ip ) ) ) ) {
			$show = false;
		}

		return $cached_show = $show;
	}

	/**
	 * Registers admin modules
	 */
	public function admin_modules() {
		$csmart_admin_modules = ConsentMagic()->getOption( 'csmart_admin_modules' );
		if ( empty( $csmart_admin_modules ) ) {
			$csmart_admin_modules = array();
		}

		//loop through module list and include its file
		foreach ( $this->admin_modules as $module ) {
			$module_title = $module[ 'title' ];
			$is_active    = 1;
			if ( isset( $csmart_admin_modules[ $module_title ] ) ) {
				$is_active = $csmart_admin_modules[ $module_title ]; //checking module status
			} else {
				$csmart_admin_modules[ $module_title ] = 1; //default status is active
			}
			$module_item = $module[ 'module' ];
			$module_file = plugin_dir_path( __FILE__ ) . "modules/$module_title/$module_item.php";
			if ( file_exists( $module_file ) && $is_active == 1 ) {
				self::$existing_modules[] = $module[ 'title' ]; //this is for module_exits checking
				require_once $module_file;
			} else {
				$csmart_admin_modules[ $module_title ] = 0;
			}
		}

		$out = array();
		if ( !empty( $this->admin_modules ) ) {
			$modules_names = array_column( $this->admin_modules, 'title' );
			foreach ( $csmart_admin_modules as $k => $m ) {
				if ( in_array( $k, $modules_names ) ) {
					$out[ $k ] = $m;
				}
			}
		}
		ConsentMagic()->updateOptions( array(  'csmart_admin_modules' => $out ) );
	}
}

new CS_Integrations();