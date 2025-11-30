<?php

/**
 * The core plugin class.
 * This is used to define internationalization, admin-specific hooks, and
 * public-facing site hooks.
 * Also maintains the unique identifier of this plugin as well as the current
 * version of the plugin.
 * @since      1.0.0
 * @package    ConsentMagic
 * @subpackage ConsentMagic/includes
 */

namespace ConsentMagicPro;

class ConsentMagic extends CS_Options {

	/**
	 * $_instance
	 */
	private static $_instance = null;

	/**
	 * The loader that's responsible for maintaining and registering all hooks that power
	 * the plugin.
	 */
	protected $loader;

	public array $status = array();

	public $cs_track_analytics = 0;

	private $active_rule_id;

	public static $unblocked_user_ip = false;

	public static $check_unblocked_crawlers;

	public static bool $check_disable_cm = false;

	public array $containers_names;

	public static $run = null;

	/**
	 * Get singleton instance of ConsentMagic
	 * @return ?ConsentMagic
	 */
	public static function instance(): ?ConsentMagic {
		if ( is_null( self::$_instance ) ) {
			self::$_instance = new self();
		}

		return self::$_instance;
	}

	/**
	 * Constructor sets up core plugin properties and containers
	 */
	public function __construct() {

		parent::__construct();

		$this->containers_names = array(
			'cs_cookie_bar_container' => generate_name(),
		);
	}

	/**
	 * Initialize plugin: load dependencies, set locale, define hooks
	 * @return void
	 */
	public function init(): void {
		$this->load_dependencies();

		if ( is_plugin_activated() && $this->check_first_flow() ) {
			if ( is_multisite() && !defined( 'COOKIEHASH' ) ) {
				wp_cookie_constants();
			}

			$cs_check_tables = $this->getOption( 'cs_check_unblock_ip_tables' );
			if ( !$cs_check_tables ) {
				$this->check_tables();
			}

			$this->get_unblocked_user_ip();
			$this->check_unblocked_crawlers();
			$this->set_active_rule_id();
			$this->define_plugin_options();

			if ( (int) $this->getOption( 'cs_geolocation' ) === 1 &&
			     (int) $this->getOption( 'cs_geo_activated' ) === 1 ) {
				$cs_geo = new CS_Geoip;
				$cs_geo->check_databases();
			}
		}

		$this->set_locale();
		$this->define_admin_hooks();
	}

	/**
	 * Define and populate plugin options
	 * @return void
	 */
	public function define_plugin_options(): void {
		$this->cs_options[ 'active_rule_id' ]               = $this->get_active_rule_id();
		$this->cs_options[ 'cs_test_mode' ]                 = $this->getOption( 'cs_test_mode' );
		$this->cs_options[ 'test_prefix' ]                  = '';
		$this->cs_options[ 'cs_track_analytics' ]           = 0;
		$this->cs_options[ 'analytics_cat_id' ]             = $this->getOption( 'analytics_cat_id' );
		$this->cs_options[ 'cs_type' ]                      = get_post_meta( $this->cs_options[ 'active_rule_id' ], '_cs_type', true );
		$this->cs_options[ 'cs_plugin_activation' ]         = $this->getOption( 'cs_plugin_activation' );
		$this->cs_options[ 'allowed_roles' ]                = $this->getOption( 'cs_admin_permissions' );
		$this->cs_options[ 'cs_block_pre_defined_scripts' ] = $this->getOption( 'cs_block_pre_defined_scripts' );
		$this->cs_options[ 'cs_script_blocking_enabled' ]   = $this->getOption( 'cs_script_blocking_enabled' );

		if ( $this->cs_options[ 'cs_test_mode' ] ) {
			$this->cs_options[ 'test_prefix' ] = '_test';
		}

		$this->cs_options[ 'viewed_cookie' ] = "cs_viewed_cookie_policy" . $this->cs_options[ 'test_prefix' ];

		if ( $this->cs_options[ 'cs_block_pre_defined_scripts' ] == 1 &&
		     !CS_Cookies()->getCookie( $this->cs_options[ 'viewed_cookie' ] ) ) {
			$this->cs_options[ 'cs_track_analytics' ] = get_post_meta( $this->cs_options[ 'active_rule_id' ], '_cs_track_analytics', true );
		}
		$this->cs_track_analytics = $this->cs_options[ 'cs_track_analytics' ];

	}

	/**
	 * Load the required dependencies for this plugin.
	 * Include the following files that make up the plugin:
	 * - CS_Loader. Orchestrates the hooks of the plugin.
	 * - CS_i18n. Defines internationalization functionality.
	 * - CS_Admin. Defines all hooks for the admin area.
	 * - CS_Public. Defines all hooks for the public side of the site.
	 * Create an instance of the loader which will be used to register the hooks
	 * with WordPress.
	 * @return void
	 */
	private function load_dependencies(): void {

		/**
		 * The class responsible for orchestrating the actions and filters of the
		 * core plugin.
		 */
		require_once plugin_dir_path( dirname( __FILE__ ) ) . 'includes/CS_Loader.php';

		/**
		 * The class responsible for defining internationalization functionality
		 * of the plugin.
		 */
		require_once plugin_dir_path( dirname( __FILE__ ) ) . 'includes/CS_i18n.php';

		/**
		 * The class responsible for defining all actions that occur in the admin area.
		 */
		require_once plugin_dir_path( dirname( __FILE__ ) ) . 'includes/CS_Admin.php';
		require_once plugin_dir_path( dirname( __FILE__ ) ) . 'includes/admin-pages/CS_Page.php';
		require_once plugin_dir_path( dirname( __FILE__ ) ) . 'includes/admin-pages/CS_AdminPage.php';
		require_once plugin_dir_path( dirname( __FILE__ ) ) . 'includes/admin-pages/CS_AdminPageMain.php';
		require_once plugin_dir_path( dirname( __FILE__ ) ) . 'includes/admin-pages/CS_AdminPageSettings.php';
		require_once plugin_dir_path( dirname( __FILE__ ) ) . 'includes/admin-pages/CS_AdminPageIab.php';
		require_once plugin_dir_path( dirname( __FILE__ ) ) . 'includes/admin-pages/CS_AdminPageGeolocation.php';
		require_once plugin_dir_path( dirname( __FILE__ ) ) . 'includes/admin-pages/CS_AdminPageProofConsent.php';
		require_once plugin_dir_path( dirname( __FILE__ ) ) . 'includes/admin-pages/CS_AdminPageProofStatistics.php';
		require_once plugin_dir_path( dirname( __FILE__ ) ) . 'includes/admin-pages/CS_AdminPageShortcodes.php';
		require_once plugin_dir_path( dirname( __FILE__ ) ) . 'includes/admin-pages/CS_AdminPageLicense.php';

		/**
		 * The class responsible for defining all actions that occur in the public-facing
		 * side of the site.
		 */

		require_once plugin_dir_path( dirname( __FILE__ ) ) . 'includes/CS_Public.php';

		/**
		 * The class responsible for defining all actions that occur in the integrations
		 * side of the site.
		 */
		require_once plugin_dir_path( dirname( __FILE__ ) ) . 'includes/modules/geoip2/CS_Geoip2_Module.php';

		$this->loader = new CS_Loader();
	}

	/**
	 * Define the locale for this plugin for internationalization.
	 * Uses the CS_i18n class in order to set the domain and to register the hook
	 * with WordPress.
	 * @return void
	 */
	private function set_locale(): void {

		$plugin_i18n = new CS_i18n();
		$this->loader->add_action( 'plugins_loaded', $plugin_i18n, 'load_plugin_textdomain' );
	}

	/**
	 * Register all of the hooks related to the admin area functionality
	 * of the plugin.
	 * @return void
	 */
	private function define_admin_hooks(): void {

		$plugin_admin = new CS_Admin( $this->get_plugin_name(), $this->get_version() );

		/**
		 * loading admin modules
		 */
		$this->loader->add_action( 'init', $this, 'manageAdminPermissions' );
		$this->loader->add_action( 'init', $plugin_admin, 'create_taxonomy', 1 );
		$this->loader->add_action( 'init', $plugin_admin, 'register_custom_post_type', 1 );

		$this->loader->add_action( 'init', $plugin_admin, 'cs_insert_templates_lists', 1 );
		$this->loader->add_action( 'init', $plugin_admin, 'cs_insert_cookie_lists', 1 );

		if ( (int) $this->getOption( 'cs_free' ) === 1 && is_plugin_activated() ) {
			$this->loader->add_action( 'init', $plugin_admin, 'cs_migrate_to_pro', 1 );
		}

		$this->loader->add_action( 'admin_menu', $plugin_admin, 'admin_menu', 11 ); /* Adding admin menu */

		$this->loader->add_action( 'admin_enqueue_scripts', $plugin_admin, 'enqueue_styles' );
		$this->loader->add_action( 'admin_enqueue_scripts', $plugin_admin, 'enqueue_scripts' );

		if ( is_plugin_activated() ) {

			//if bot - script not working
			if ( $this->check_first_flow() && !self::$check_unblocked_crawlers ) {
				if ( !is_active_supreme_modules_pro_for_divi() ) {
					$priority = 11;
				} else {
					$priority = 30;
				}
				$plugin_public = new CS_Public( $this->getOption( 'cs_enable_site_cache' ) );
				$this->loader->add_action( 'init', $plugin_public, 'public_init' );
				$this->loader->add_action( 'wp_enqueue_scripts', $plugin_public, 'enqueue_styles' );
				$this->loader->add_action( 'wp_enqueue_scripts', $plugin_public, 'enqueue_scripts', $priority );
				$this->loader->add_action( 'admin_notices', $plugin_admin, 'cs_admin_messages' );
				$this->loader->add_action( 'admin_init', $plugin_admin, 'cs_close_messages' );
				$this->loader->add_action( 'init', $this, 'check_policy_page', 1 );

				if ( self::$unblocked_user_ip !== false ) {
					self::$check_disable_cm = $plugin_public->cs_check_disable_cm( self::$unblocked_user_ip );
				} else {
					self::$check_disable_cm = $plugin_public->cs_check_disable_cm( array() );
				}
			}
		}

		$this->cs_check_settings( $plugin_admin );

		//check load translations for current plugin version
		$this->loadTranslations();
	}

	/**
	 * Get post ID by slug
	 * @param string $slug
	 * @return int|null
	 */
	public static function getCurrentPostID( string $slug ): ?int {
		static $cached_id = null;
		if ( $cached_id !== null ) {
			return $cached_id;
		}

		global $wpdb;
		$post_id      = $wpdb->get_var( $wpdb->prepare( "SELECT ID FROM $wpdb->posts WHERE post_name = %s", $slug ) );

		return $cached_id = $post_id;
	}


	/**
	 * Set active rule ID
	 * @return void
	 */
	private function set_active_rule_id(): void {

		if ( isset( $_GET[ 'cs_scan_pages' ] ) && $_GET[ 'cs_scan_pages' ] == 1 ) {
			$this->active_rule_id = false;

			return;
		}

		if ( isset( $_POST[ 'action' ] ) &&
		     ( $_POST[ 'action' ] == 'cs_preview_show' || $_POST[ 'action' ] == 'preview_list_show' ) &&
		     isset( $_POST[ 'cs_rule_id' ] ) ) {
			$this->active_rule_id = $this->sanitize_text_field( $_POST[ 'cs_rule_id' ] );

			return;
		}

		if ( empty( $this->active_rule_id ) ) {
			if ( ( (int) $this->getOption( 'cs_geolocation' ) === 0 ||
			       (int) $this->getOption( 'cs_geo_activated' ) === 0 ) ) {
				$args_posts = array(
					'post_type'      => CMPRO_POST_TYPE,
					'post_status'    => 'publish',
					'meta_query'     => [
						[
							'key'   => '_cs_enable_rule',
							'value' => 1,
						]
					],
					'posts_per_page' => 1,
					'order'          => 'ASC',
					'meta_key'       => '_cs_order',
					'orderby'        => 'meta_value_num',
					'meta_type'      => 'NUMERIC',
				);
			} else {

				$cs_geo          = new CS_Geoip();
				$user_country    = $cs_geo->cs_check_country();
				$user_state_meta = array();

				if ( $user_country == 'US' ) {
					$user_state_meta = array(
						'key'     => '_cs_us_states_target',
						'value'   => 'US_' . $cs_geo->get_state(),
						'compare' => 'LIKE'
					);
				}

				if ( empty( $user_country ) ) {
					$args_posts = array(
						'post_type'      => CMPRO_POST_TYPE,
						'post_status'    => 'publish',
						'meta_query'     => array(
							'relation' => 'AND',
							array(
								'key'   => '_cs_no_ip_rule',
								'value' => 1,
							),
							array(
								'key'   => '_cs_enable_rule',
								'value' => 1,
							)
						),
						'posts_per_page' => 1,
						'order'          => 'ASC',
						'meta_key'       => '_cs_order',
						'orderby'        => 'meta_value_num',
						'meta_type'      => 'NUMERIC',
					);
					$the_post   = get_posts( $args_posts );
					if ( empty( $the_post ) ) {
						$args_posts = array(
							'post_type'      => CMPRO_POST_TYPE,
							'post_status'    => 'publish',
							'meta_query'     => array(
								array(
									'key'   => '_cs_enable_rule',
									'value' => 1,
								)
							),
							'posts_per_page' => 1,
							'order'          => 'ASC',
							'meta_key'       => '_cs_order',
							'orderby'        => 'meta_value_num',
							'meta_type'      => 'NUMERIC',
						);
					}
				} else {
					$args_posts = array(
						'post_type'      => CMPRO_POST_TYPE,
						'post_status'    => 'publish',
						'meta_query'     => array(
							'relation' => 'AND',
							array(
								'key'     => '_cs_target',
								'value'   => $user_country,
								'compare' => 'LIKE'
							),
							$user_state_meta,
							array(
								'key'   => '_cs_enable_rule',
								'value' => 1,
							)
						),
						'posts_per_page' => 1,
						'order'          => 'ASC',
						'meta_key'       => '_cs_order',
						'orderby'        => 'meta_value_num',
						'meta_type'      => 'NUMERIC',
					);
				}
			}

			$rule = get_posts( $args_posts );

			if ( !empty( $rule ) ) {
				$post_id = $rule[ 0 ]->ID;
			} else {
				$post_id = '';
			}

			$this->active_rule_id = $post_id;
		}
	}

	/**
	 * Run the loader to execute all of the hooks with WordPress.
	 * @return void
	 */
	public function run(): void {

		//run all the hooks with WordPress
		$this->loader->run();

		if ( $this->getOption( 'cs_store_consent_for_wc_orders' ) ) {
			add_action( 'add_meta_boxes', array(
				$this,
				'woo_add_order_meta_boxes'
			) );

			add_action( 'edd_view_order_details_main_after', array(
				$this,
				'add_edd_order_details'
			) );
		}

		add_action( 'woocommerce_init', function() {
			add_action( 'woocommerce_email_customer_details', array(
				$this,
				'cs_send_admin_order_consent'
			), 80, 4 );
		} );
	}

	/**
	 * Ensure required database tables exist
	 * @return bool True on success
	 */
	public function check_tables(): bool {
		global $wpdb;
		require_once ABSPATH . 'wp-admin/includes/upgrade.php';
		//creating unlock users ip list table ========================
		$table_name_ip = $wpdb->prefix . 'cs_unblock_ip';
		$like          = '%' . $wpdb->esc_like( $table_name_ip ) . '%';
		$search_query  = $wpdb->prepare( "SHOW TABLES LIKE %s", $like );
		if ( !$wpdb->get_results( $search_query, ARRAY_N ) ) {
			$create_table_sql = "CREATE TABLE `$table_name_ip`(
			    `id` INT NOT NULL AUTO_INCREMENT,
			    `ip` VARCHAR(250) NOT NULL,
			    `created_at` DATETIME NOT NULL DEFAULT '0000-00-00 00:00:00',
			    PRIMARY KEY(`id`)
			);";
			dbDelta( $create_table_sql );
		}
		$this->updateOptions( array( 'cs_check_unblock_ip_tables' => true ) );

		return true;
	}

	/**
	 * Render consent info in WooCommerce emails
	 * @param \WC_Order $order
	 * @param bool      $sent_to_admin
	 * @param bool      $plain_text
	 * @param object    $email
	 * @return bool
	 */
	public function cs_send_admin_order_consent( $order, $sent_to_admin, $plain_text, $email ): bool {

		if ( in_array( $email->id, [ 'customer_processing_order' ] ) ) {
			if ( $this->getOption( "cs_customer_email_consent_for_wc_orders" ) ) {
				$orderId = $order->get_id();
				echo "<h2 style='text-align: center'>" .
				     esc_html( $this->getOption( "cs_admin_email_privacy_title" ) ) .
				     "</h2>";
				include_once CMPRO_PLUGIN_VIEWS_PATH . 'admin/order/html-order-meta-box.php';
			}
		}
		if ( $sent_to_admin ) {
			if ( $this->getOption( "cs_admin_email_consent_for_wc_orders" ) ) {
				$orderId = $order->get_id();
				echo "<h2 style='text-align: center'>" . esc_html__( 'ConsentMagic', 'consent-magic' ) . "</h2>";
				echo sprintf( esc_html__( 'We only send the consent related data in the New Order email that you get. You can remove it by opening the ConsentMagic plugin\'s %s settings page %s', 'consent-magic' ), "<a href='" .
				                                                                                                                                                                                                       esc_url( admin_url( "admin.php?page=cs-settings" ) ) .
				                                                                                                                                                                                                       "' target='_blank'>", '</a>' );
				echo '<br>';
				echo sprintf( esc_html__( 'You can also send this privacy info to your clients. It can increase their confidence in your brand and build trust. Turin it on from the ConsentMagic %s settings page %s', 'consent-magic' ), "<a href='" .
				                                                                                                                                                                                                                           esc_url( admin_url( "admin.php?page=cs-settings" ) ) .
				                                                                                                                                                                                                                           "' target='_blank'>", '</a>.' );

				include_once CMPRO_PLUGIN_VIEWS_PATH . 'admin/order/html-order-meta-box.php';
			}
		}

		return false;
	}

	/**
	 * Render ConsentMagic meta box in WooCommerce order screen
	 * @param \WC_Order|\WP_Post $order
	 * @return void
	 */
	public function woo_render_order_fields( $order ): void {

		$orderId = ( $order instanceof \WP_Post ) ? $order->ID : $order->get_id();

		include_once CMPRO_PLUGIN_VIEWS_PATH . 'admin/order/html-order-box.php';
	}

	/**
	 * Register WooCommerce order meta boxes for ConsentMagic
	 * @return void
	 */
	public function woo_add_order_meta_boxes(): void {

		$screen = isWooUseHPStorage() ? wc_get_page_screen_id( 'shop-order' ) : 'shop_order';

		add_meta_box( 'cs_enrich_fields_woo', esc_html__( 'ConsentMagic', 'consent-magic' ), array(
			$this,
			"woo_render_order_fields"
		), $screen );
	}

	/**
	 * Grant or revoke manage_cs capability based on allowed roles
	 * @return void
	 */
	function manageAdminPermissions(): void {
		global $wp_roles;

		$roles = $this->getOption( 'cs_admin_permissions' );

		foreach ( $wp_roles->roles as $role => $options ) {

			if ( in_array( $role, $roles ) || $role == 'administrator' ) {
				$wp_roles->add_cap( $role, 'manage_cs' );
			} else {
				$wp_roles->remove_cap( $role, 'manage_cs' );
			}
		}
	}

	/**
	 * Ensure privacy policy page option is set
	 * @return void
	 */
	function check_policy_page(): void {
		$privacy_option = $this->getOption( 'cs_policy_existing_page' );
		if ( !$privacy_option || $privacy_option == '' ) {
			$privacy_policy_page = get_option( 'wp_page_for_privacy_policy' );
			if ( $privacy_policy_page ) {
				$this->updateOptions( array( 'cs_policy_existing_page' => array( CMPRO_DEFAULT_LANGUAGE => $privacy_policy_page ) ) );
			} else {
				$this->updateOptions( array( 'cs_policy_existing_page' => array( CMPRO_DEFAULT_LANGUAGE => 1 ) ) );
			}
		}
	}

	/**
	 * Get current consent version number
	 * @return int
	 */
	public function cs_get_consent_version(): int {
		$current_consent_version = 1;
		$consent_version         = $this->getOption( 'cs_consent_version' );
		if ( !empty( $consent_version ) ) {
			$current_consent_version = (int) $consent_version;
		}

		return $current_consent_version;
	}

	/**
	 * Get test consent version number
	 * @return int
	 */
	public function cs_get_consent_version_test(): int {
		$current_consent_version = 1;
		$consent_version         = $this->getOption( 'cs_consent_version_test' );
		if ( !empty( $consent_version ) ) {
			$current_consent_version = (int) $consent_version;
		}

		return $current_consent_version;
	}

	/**
	 * Resolve post by geolocation settings
	 * @return int|false Post ID or false
	 */
	function cs_get_post_by_geo() {

		if ( ( (int) $this->getOption( 'cs_geolocation' ) === 0 ||
		       (int) $this->getOption( 'cs_geo_activated' ) === 0 ) ) {
			$args_posts = array(
				'post_type'      => CMPRO_POST_TYPE,
				'post_status'    => 'publish',
				'meta_query'     => array(
					array(
						'key'   => '_cs_enable_rule',
						'value' => 1,
					)
				),
				'posts_per_page' => 1,
				'order'          => 'ASC',
				'meta_key'       => '_cs_order',
				'orderby'        => 'meta_value_num',
				'meta_type'      => 'NUMERIC',
			);
		} else {
			$cs_geo = ( new CS_Geoip() );

			$user_country    = $cs_geo->cs_check_country();
			$user_state_meta = array();

			$p_id = null;

			if ( $user_country == 'US' ) {
				$user_state_meta = array(
					'key'     => '_cs_us_states_target',
					'value'   => 'US_' . $cs_geo->get_state(),
					'compare' => 'LIKE'
				);
			}

			$args_posts = array(
				'post_type'      => CMPRO_POST_TYPE,
				'post_status'    => 'publish',
				'meta_query'     => array(
					'relation' => 'AND',
					array(
						'key'     => '_cs_target',
						'value'   => $user_country,
						'compare' => 'LIKE'
					),
					$user_state_meta,
					array(
						'key'   => '_cs_enable_rule',
						'value' => 1,
					)
				),
				'posts_per_page' => 1,
				'order'          => 'ASC',
				'meta_key'       => '_cs_order',
				'orderby'        => 'meta_value_num',
				'meta_type'      => 'NUMERIC',
			);
		}

		$loop_posts = new \WP_Query( $args_posts );
		if ( !$loop_posts->have_posts() ) {
			return false;
		} else {
			while ( $loop_posts->have_posts() ) {
				$loop_posts->the_post();
				$p_id = get_the_ID();
			}
		}
		wp_reset_postdata();

		return $p_id;
	}

	/**
	 * Build HTML preview list of available templates
	 * @param string|false $post_slug
	 * @param string       $type multi|single
	 * @return string|false
	 */
	function cs_get_template_list( $post_slug = false, $type = 'multi' ) {
		$cs_light_theme_id = get_page_id_by_path( 'cs_light_theme', '', CMPRO_TEMPLATE_POST_TYPE );
		$cs_dark_theme     = get_page_id_by_path( 'cs_dark_theme', '', CMPRO_TEMPLATE_POST_TYPE );
		$cs_orange_theme   = get_page_id_by_path( 'cs_orange_theme', '', CMPRO_TEMPLATE_POST_TYPE );

		$active_rule_id = $this->cs_get_post_by_geo();

		if ( $post_slug ) {
			$args_posts = array(
				'post_type'      => CMPRO_TEMPLATE_POST_TYPE,
				'post_status'    => 'any',
				'name'           => $post_slug,
				'posts_per_page' => 1,
			);
		} else {
			$args_posts = array(
				'post_type'      => CMPRO_TEMPLATE_POST_TYPE,
				'post_status'    => 'any',
				'posts_per_page' => -1,
				'post__not_in'   => [
					$cs_light_theme_id,
					$cs_dark_theme,
					$cs_orange_theme
				]
			);
		}

		$loop_posts = new \WP_Query( $args_posts );
		if ( !$loop_posts->have_posts() ) {
			return false;
		} else {
			$active_rule_id = $this->get_active_rule_id();
			if ( !empty( $active_rule_id ) ) {
				$metas                = get_post_meta( $active_rule_id );
				$cs_deny_all_btn      = $metas[ '_cs_deny_all_btn' ][ 0 ];
				$cs_custom_button_btn = $metas[ '_cs_custom_button' ][ 0 ];
				$cs_hide_close_btn    = $metas[ '_cs_hide_close_btn' ][ 0 ];
				$design               = $metas[ '_cs_design_type' ][ 0 ];
			}

			$design_link = $type == 'single' ? 'cs-single-step-design' : 'cs-multi-step-design';

			while ( $loop_posts->have_posts() ) {
				$loop_posts->the_post();
				$id               = $loop_posts->post->ID;
				$cs_privacy_link  = '1';
				$cs_bars_position = 'bottom';
				$cs_type          = 'ask_before_tracking';
				$style_array      = get_post_meta( $id );

				$edit_link = get_admin_url( null, 'admin.php?page=consent-magic' ) .
				             '&design=' .
				             $design_link .
				             '&design_template=edit&template_id=' .
				             $id;
				?>
                <div class="card card-static card-style8">
					<?php if ( !$post_slug ) : ?>
                        <div class="card-header card-header-style2 d-flex justify-content-between align-items-center">
                            <h3 class="primary-heading-type2"><?php echo esc_html( get_the_title( $id ) ); ?></h3>

                            <div class="action-buttons design-action-buttons">
                                <button type="button" data-id="<?php echo esc_attr( $id ); ?>"
                                        class="button-link link-delete template_delete_btn">
									<?php esc_html_e( 'Delete', 'consent-magic' ); ?>
                                </button>

                                <a href="<?php echo esc_url( $edit_link ); ?>"
                                   class="button-link link-edit">
									<?php esc_html_e( 'Edit', 'consent-magic' ); ?>
                                </a>

                            </div>
                        </div>
					<?php else : ?>
                        <div class="card-header card-header-style3">
                            <h3 class="primary-heading-type2"><?php echo esc_html( get_the_title( $id ) ); ?></h3>
                        </div>
					<?php endif; ?>

                    <div class="card-body">
                        <div class="cs_preview_design_block">
							<?php include CMPRO_PLUGIN_VIEWS_PATH . "admin/components/spinner.php"; ?>

                            <div class="loading_body cs_preview_design_wrap">
                                <div class="preview-title"><?php esc_html_e( 'Preview', 'consent-magic' ); ?>:</div>

                                <div class="preview-links">
									<?php if ( $type === 'multi' ) : ?>
										<?php $cs_bars_type = 'bar_small'; ?>
                                        <div>
                                            <a type="button" class="link link-underline preview_action_design lh-162"
                                               data-template="bar_small" data-theme="<?php echo esc_attr( $id ); ?>">
												<?php esc_html_e( 'Small bar', 'consent-magic' ); ?>
                                            </a>
                                        </div>

										<?php
										$cs_bars_type = 'bar_large'; ?>
                                        <div>
                                            <a type="button" class="link link-underline preview_action_design lh-162"
                                               data-template="bar_large" data-theme="<?php echo esc_attr( $id ); ?>">
												<?php esc_html_e( 'Large bar', 'consent-magic' ); ?>
                                            </a>
                                        </div>
										<?php

										$cs_bars_type = 'popup_small'; ?>
                                        <div>
                                            <a type="button" class="link link-underline preview_action_design lh-162"
                                               data-template="popup_small" data-theme="<?php echo esc_attr( $id ); ?>">
												<?php esc_html_e( 'Small popup', 'consent-magic' ); ?>
                                            </a>
                                        </div>
									<?php endif;

									$cs_bars_type = $type === 'multi' ? 'popup_large' : 'popup_large_single'; ?>
                                    <div>
                                        <a type="button" class="link link-underline preview_action_design lh-162"
                                           data-template="<?php echo esc_attr( $cs_bars_type ); ?>"
                                           data-theme="<?php echo esc_attr( $id ); ?>">
											<?php esc_html_e( 'Large popup', 'consent-magic' ); ?>
                                        </a>
                                    </div>
                                </div>
                            </div>
                        </div>
                    </div>
                </div>
				<?php
			}
		}
	}

	/**
	 * Get cached active rule ID
	 * @return int|string
	 */
	public function get_active_rule_id() {

		if ( empty( $this->active_rule_id ) ) {
			$this->set_active_rule_id();
		}

		return $this->active_rule_id;
	}

	/**
	 * Get active rule type meta value
	 * @return string
	 */
	public function get_active_rule_name(): string {
		$active_rule_id = $this->get_active_rule_id();

		return get_post_meta( $active_rule_id, '_cs_type', true );
	}

	/**
	 * Resolve preview rule ID using geolocation if enabled
	 * @return int|false
	 */
	public function get_preview_rule_id() {

		if ( ( (int) $this->getOption( 'cs_geolocation' ) === 0 ||
		       (int) $this->getOption( 'cs_geo_activated' ) === 0 ) ) {
			$args_posts = array(
				'post_type'      => CMPRO_POST_TYPE,
				'post_status'    => 'publish',
				'meta_query'     => array(
					array(
						'key'   => '_cs_enable_rule',
						'value' => 1,
					)
				),
				'posts_per_page' => 1,
				'order'          => 'ASC',
				'meta_key'       => '_cs_order',
				'orderby'        => 'meta_value_num',
				'meta_type'      => 'NUMERIC',
			);
		} else {

			$cs_geo          = ( new CS_Geoip() );
			$user_country    = $cs_geo->cs_check_country();
			$user_state_meta = array();

			if ( $user_country == 'US' ) {
				$user_state_meta = array(
					'key'     => '_cs_us_states_target',
					'value'   => 'US_' . $cs_geo->get_state(),
					'compare' => 'LIKE'
				);
			}

			$args_posts = array(
				'post_type'      => CMPRO_POST_TYPE,
				'post_status'    => 'publish',
				'meta_query'     => array(
					'relation' => 'AND',
					array(
						'key'     => '_cs_target',
						'value'   => $user_country,
						'compare' => 'LIKE'
					),
					$user_state_meta,
				),
				'posts_per_page' => 1,
				'order'          => 'ASC',
				'meta_key'       => '_cs_order',
				'orderby'        => 'meta_value_num',
				'meta_type'      => 'NUMERIC',
			);
		}

		$rule = new \WP_Query( $args_posts );
		if ( !$rule->have_posts() ) {
			return false;
		} else {
			$rule->the_post();

			return $rule->post->ID;
		}
	}

	/**
	 * Get active rule for single step design
	 * @return false|int
	 */
	public function get_active_rule_for_single_design() {
		$args_posts = array(
			'post_type'      => CMPRO_POST_TYPE,
			'post_status'    => 'publish',
			'meta_query'     => array(
				'relation' => 'AND',
				array(
					'key'   => '_cs_enable_rule',
					'value' => 1,
				),
				array(
					'relation' => 'OR',
					array(
						'key'   => '_cs_type',
						'value' => 'ask_before_tracking',
					),
					array(
						'key'   => '_cs_type',
						'value' => 'inform_and_opiout',
					),
					array(
						'key'   => '_cs_type',
						'value' => 'iab',
					),
				)
			),
			'posts_per_page' => 1,
			'order'          => 'ASC',
			'meta_key'       => '_cs_order',
			'orderby'        => 'meta_value_num',
			'meta_type'      => 'NUMERIC',
		);

		$rule = new \WP_Query( $args_posts );
		if ( !$rule->have_posts() ) {
			return false;
		} else {
			$rule->the_post();

			return $rule->post->ID;
		}
	}

	/**
	 * Load unblocked user IPs from DB on first call
	 * @return array
	 */
	public function get_unblocked_user_ip(): array {
		if ( self::$unblocked_user_ip === false ) {
			global $wpdb;
			$table_name = $wpdb->prefix . 'cs_unblock_ip';

			$ip_list     = $wpdb->get_results( "SELECT `ip` FROM `{$table_name}`", ARRAY_N );
			$ip_list_new = array();

			if ( $ip_list ) {
				foreach ( $ip_list as $ip ) {
					$ip_list_new[] = $ip[ 0 ];
				}
			}

			self::$unblocked_user_ip = $ip_list_new;
		}

		return self::$unblocked_user_ip;
	}

	/**
	 * Detect known crawler user agents and disable blocker for them
	 * @return void
	 */
	public function check_unblocked_crawlers(): void {

		if ( self::$check_unblocked_crawlers === null ) {
			$enabled_option = $this->getOption( 'cs_disable_for_knonw_crawlers' );
			if ( (int) $enabled_option === 1 ) {

				$bots = array(
					'YandexBot',
					'YandexAccessibilityBot',
					'YandexMobileBot',
					'YandexDirectDyn',
					'YandexScreenshotBot',
					'YandexImages',
					'YandexVideo',
					'YandexVideoParser',
					'YandexMedia',
					'YandexBlogs',
					'YandexFavicons',
					'YandexWebmaster',
					'YandexPagechecker',
					'YandexImageResizer',
					'YandexAdNet',
					'YandexDirect',
					'YaDirectFetcher',
					'YandexCalendar',
					'YandexSitelinks',
					'YandexMetrika',
					'YandexNews',
					'YandexNewslinks',
					'YandexCatalog',
					'YandexAntivirus',
					'YandexMarket',
					'YandexVertis',
					'YandexForDomain',
					'YandexSpravBot',
					'YandexSearchShop',
					'YandexMedianaBot',
					'YandexOntoDB',
					'YandexOntoDBAPI',
					'Googlebot',
					'Googlebot-Image',
					'Googlebot-News',
					'Googlebot-Video',
					'Mediapartners-Google',
					'AdsBot-Google',
					'Chrome-Lighthouse',
					'Lighthouse',
					'Mail.RU_Bot',
					'bingbot',
					'Accoona',
					'ia_archiver',
					'Ask Jeeves',
					'OmniExplorer_Bot',
					'W3C_Validator',
					'WebAlta',
					'YahooFeedSeeker',
					'Yahoo!',
					'Ezooms',
					'Tourlentabot',
					'MJ12bot',
					'AhrefsBot',
					'SearchBot',
					'SiteStatus',
					'Nigma.ru',
					'Baiduspider',
					'Statsbot',
					'SISTRIX',
					'AcoonBot',
					'findlinks',
					'proximic',
					'OpenindexSpider',
					'statdom.ru',
					'Exabot',
					'Spider',
					'SeznamBot',
					'oBot',
					'C-T bot',
					'Updownerbot',
					'Snoopy',
					'heritrix',
					'Yeti',
					'DomainVader',
					'DCPbot',
					'PaperLiBot',
					'APIs-Google',
					'AdsBot-Google-Mobile',
					'AdsBot-Google-Mobile',
					'AdsBot-Google-Mobile-Apps',
					'FeedFetcher-Google',
					'Google-Read-Aloud',
					'DuplexWeb-Google',
					'Storebot-Google'
				);

				foreach ( $bots as $bot ) {
					if ( isset( $_SERVER[ 'HTTP_USER_AGENT' ] ) &&
					     stripos( $this->sanitize_text_field( $_SERVER[ 'HTTP_USER_AGENT' ] ), $bot ) !== false ) {
						self::$check_unblocked_crawlers = true; //if bot - blocker not working
					}
				}
			} else {
				self::$check_unblocked_crawlers = false; //if disabled option - run standart blocker
			}
		}
	}

	/**
	 * Get current crawler bypass state
	 * @return bool|null
	 */
	public function get_unblocked_crawlers() {
		return self::$check_unblocked_crawlers;
	}

	/**
	 * Build minimum recommended options array based on active rule and terms
	 * @return array
	 */
	public function get_minimum_recommended_options(): array {
		$unassigned          = get_term_by( 'slug', 'unassigned', 'cs-cookies-category' );
		$unassigned_id       = $unassigned->term_id;
		$options             = get_cookies_terms_objects( null, false, $unassigned_id );
		$active_rule_id      = $this->get_active_rule_id();
		$minimum_recommended = array();

		if ( !empty( $options ) ) {
			foreach ( $options as $option ) {
				$ignore_category = get_term_meta( $option->term_id, 'cs_ignore_this_category', true );
				if ( !$ignore_category ) {
					$value                                   = (int) get_post_meta( $active_rule_id, '_cs_custom_button_' .
					                                                                                 $option->term_id, true );
					$minimum_recommended[ $option->term_id ] = $value;

					if ( $option->slug === 'marketing' ) {
						$minimum_recommended[ 'cs_enabled_server_side' ] = $minimum_recommended[ 'cs_enabled_advanced_matching' ] = $value;
					}
				}
			}
		}

		return $minimum_recommended;
	}

	/**
	 * Get generated container element names
	 * @return array
	 */
	public function get_containers_names(): array {
		return $this->containers_names;
	}

	/**
	 * Render EDD order details meta box
	 * @param int $order_id
	 * @return void
	 */
	function add_edd_order_details( $order_id ): void {
		echo '<div class="postbox" style="padding: 15px;">
    		<h3 class="hndle"><span>' . esc_html__( 'Consent Magic Pro', 'consent-magic' ) . '</span></h3>';
		include_once CMPRO_PLUGIN_VIEWS_PATH . 'admin/order/html-edd-order-box.php';

		echo '</div>';
	}

	/**
	 * Check ready to run the plugin
	 * @return bool
	 */
	public function ready_to_run(): ?bool {

		if ( self::$run === null ) {
			if ( ( isset( $_GET[ 'et_fb' ] ) && sanitize_text_field( $_GET[ 'et_fb' ] ) == 1 ) ||
			     ( isset( $_GET[ 'bricks' ] ) && sanitize_text_field( $_GET[ 'bricks' ] ) == 'run' ) ||
			     isset( $_GET[ 'ct_builder' ] ) ||
			     ( isset( $_GET[ 'fb-edit' ] ) && sanitize_text_field( $_GET[ 'fb-edit' ] ) == 1 ) ||
			     ( isset( $_GET[ 'builder' ] ) &&
			       isset( $_GET[ 'builder_id' ] ) &&
			       sanitize_text_field( $_GET[ 'builder' ] ) == 'true' ) ||
			     ( isset( $_GET[ 'action' ] ) && $_GET[ 'action' ] == 'elementor' ) ) {
				return self::$run = false;
			}
			if ( ConsentMagic()->getOption( 'cs_test_mode' ) ) {
				$user          = wp_get_current_user();
				$allowed_roles = $this->getOption( 'cs_admin_permissions' );
				if ( empty( array_intersect( $allowed_roles, $user->roles ) ) ) {
					return self::$run = false;
				}
			}
		} else {
			return self::$run;
		}

		return self::$run = true;
	}

	/**
	 * Update available language for translations
	 * @param $income_langs
	 * @return void
	 */
	public function update_language_availability( $income_langs ): void {
		if ( !empty( $income_langs ) ) {
			$language_availability = $this->getOption( 'cs_language_availability' );
			foreach ( $income_langs as $lang ) {
				if ( !isset( $language_availability[ $lang ] ) || $lang == CMPRO_DEFAULT_LANGUAGE ) {
					$language_availability[ $lang ] = '1';
				}
			}
			ksort( $language_availability );
			$this->updateOptions( array( 'cs_language_availability' => $language_availability ) );
		}
	}

	public function check_first_flow() {
		return $this->getOption( 'cs_check_flow' ) == '1';
	}
}

/**
 * Helper to get ConsentMagic singleton instance
 * @return ConsentMagic|null
 */
function ConsentMagic(): ?ConsentMagic {
	return ConsentMagic::instance();
}