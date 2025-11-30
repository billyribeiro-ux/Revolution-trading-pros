<?php

/**
 * The admin-specific functionality of the plugin.
 * Defines the plugin name, version, and two examples hooks for how to
 * enqueue the admin-specific stylesheet and JavaScript.
 * @package    ConsentMagic PRO
 * @subpackage ConsentMagic PRO/includes
 */

namespace ConsentMagicPro;

class CS_Admin {

	/**
	 * The ID of this plugin.
	 */
	public $plugin_name;

	/**
	 * The version of this plugin.
	 */
	public $version;

	public $cs_options = array();

	public $data_key;

	public $wc_am_api_key_key;

	public $wc_am_instance_key;

	public $data;

	public $wc_am_instance_id;

	public $wc_am_domain;

	public $wc_am_activated_key;

	public $wc_am_deactivate_checkbox_key;

	public $edd_deactivate_checkbox_key;

	public $product_name;

	public $url_activate;

	public $edd_license_key;

	public $edd_activated_key;

	private $product_id = '';

	protected static $instance;

	public $limit;

	public $offset;

	private $current_lang;

	public static $default_rules = array(
		'cs_gdpr_rule',
		'cs_rest_of_world_rule',
		'cs_iab_rule',
		'cs_ldu_rule'
	);

	private $adminPagesSlugs = array(
		'consent-magic',
		'cs-proof-consent',
		'cs-iab',
		'cs-geolocation',
		'cs-proof-statistics',
		'cs-settings',
		'cs-additionals',
		'cs-license'
	);

	private $adminPages = array(
		'main',
		'settings',
		'iab',
		'geolocation',
		'proof-consent',
		'proof-statistics',
		'shortcodes',
		'license',
	);

	/**
	 * Initialize the class and set its properties.
	 */
	public function __construct( $plugin_name = null, $version = null ) {
		$this->plugin_name        = $plugin_name;
		$this->version            = $version;
		$this->current_lang       = get_locale();
		$cs_user_default_language = ConsentMagic()->getOption( 'cs_user_default_language' );
		$cs_language_availability = ConsentMagic()->getOption( 'cs_language_availability' );
		if ( isset( $cs_language_availability[ $this->current_lang ] )
		     && $cs_language_availability[ $this->current_lang ] == 0 ) {
			$this->current_lang = $cs_user_default_language;
		}

		if ( CMPRO_LICENSE_TYPE == 'edd' ) {

			$this->data_key                    = CMPRO_LICENSE_TYPE . '_' . CMPRO_LICENSE_PLUGIN_NAME;
			$this->product_name                = CMPRO_LICENSE_NAME;
			$this->url_activate                = 'https://www.pixelyoursite.com';
			$this->edd_deactivate_checkbox_key = $this->data_key . '_deactivate_checkbox';
			$this->edd_license_key             = CMPRO_LICENSE_TYPE . '_license_key';
			$this->edd_activated_key           = $this->data_key . '_activated';
		} elseif ( CMPRO_LICENSE_TYPE == 'woo' ) {
			$this->data_key                      = 'wc_am_client_consent_magic_pro';
			$this->wc_am_api_key_key             = $this->data_key . '_api_key';
			$this->wc_am_instance_key            = $this->data_key . '_instance';
			$this->wc_am_instance_id             = ConsentMagic()->getOption( $this->wc_am_instance_key );
			$this->wc_am_domain                  = str_ireplace( array(
				'http://',
				'https://'
			), '', home_url() ); // blog domain name
			$this->wc_am_activated_key           = $this->data_key . '_activated';
			$this->wc_am_deactivate_checkbox_key = $this->data_key . '_deactivate_checkbox';
			$this->product_id                    = ConsentMagic()->getOption( 'wc_am_product_id_consent_magic_pro' );
			$this->data                          = ConsentMagic()->getOption( $this->data_key );
		}

		$this->adminPages[ 'main' ]             = new CS_AdminPageMain( $this->plugin_name, 'main', $this );
		$this->adminPages[ 'settings' ]         = new CS_AdminPageSettings( $this->plugin_name, 'settings', $this );
		$this->adminPages[ 'iab' ]              = new CS_AdminPageIab( $this->plugin_name, 'iab', $this );
		$this->adminPages[ 'geolocation' ]      = new CS_AdminPageGeolocation( $this->plugin_name, 'geolocation' );
		$this->adminPages[ 'proof-consent' ]    = new CS_AdminPageProofConsent( $this->plugin_name, 'proofconsent', $this );
		$this->adminPages[ 'proof-statistics' ] = new CS_AdminPageProofStatistics( $this->plugin_name, 'statistics', $this );
		$this->adminPages[ 'shortcodes' ]       = new CS_AdminPageShortcodes( $this->plugin_name, 'shortcodes', $this );
		$this->adminPages[ 'license' ]          = new CS_AdminPageLicense( $this->plugin_name, 'license', $this );

		$this->cs_cron_check_status_edd();

		if ( wp_doing_ajax() ) {
			add_action( 'wp_ajax_cs_preview_show', array(
				$this,
				'cs_preview_show'
			) );

			add_action( 'wp_ajax_preview_list_show', array(
				$this,
				'cs_preview_list_show'
			) );

			add_action( 'wp_ajax_preview_design_show', array(
				$this,
				'cs_preview_design_show'
			) );

			add_action( 'wp_ajax_cs_preview_size_show', array(
				$this,
				'cs_preview_size_show'
			) );

			add_action( 'wp_ajax_cs_preview_size_single_show', array(
				$this,
				'cs_preview_size_single_show'
			) );

			add_action( 'wp_ajax_cs_preview_color_show', array(
				$this,
				'cs_preview_color_show'
			) );

			add_action( 'wp_ajax_item_sort', array(
				$this,
				'cs_save_item_order'
			), 200 );

			add_action( 'wp_ajax_predefined_rule_enable_update', array(
				$this,
				'cs_predefined_rule_enable_update'
			), 200 );

			add_action( 'wp_ajax_no_ip_rule_update', array(
				$this,
				'cs_no_ip_rule_update'
			), 200 );

			add_action( 'wp_ajax_cs_update_primary_rule', array(
				$this,
				'cs_update_primary_rule'
			), 200 );

			add_action( 'wp_ajax_cs_force_update', array(
				$this,
				'cs_force_update'
			), 200 );

			add_action( 'wp_ajax_cs_instance_update', array(
				$this,
				'cs_instance_update'
			), 200 );

			add_action( 'wp_ajax_get_proof_consent_data', array(
				$this,
				'get_proof_consent_data'
			), 200 );

			add_action( 'wp_ajax_get_ignore_ip_table', array(
				$this,
				'get_ignore_ip_table'
			), 200 );

			add_action( 'wp_ajax_remove_ignore_ip_table', array(
				$this,
				'remove_ignore_ip_table'
			), 200 );

			add_action( 'wp_ajax_cs_new_ignore_ip', array(
				$this,
				'cs_new_ignore_ip'
			), 200 );

			add_action( 'wp_ajax_get_manual_added_scripts', array(
				$this,
				'get_manual_added_scripts'
			), 200 );

			add_action( 'wp_ajax_get_scan_cookies', array(
				$this,
				'get_scan_cookies'
			), 200 );

			add_action( 'wp_ajax_get_scan_scripts', array(
				$this,
				'get_scan_scripts'
			), 200 );

			add_action( 'wp_ajax_cs_save_scan_data', array(
				$this,
				'save_scan_data'
			), 200 );

			add_action( 'wp_ajax_get_proof_rules_stats_data', array(
				$this,
				'get_proof_rules_stats_data'
			), 200 );

			add_action( 'wp_ajax_get_proof_cs_type_stats_data', array(
				$this,
				'get_proof_cs_type_stats_data'
			), 200 );

			add_action( 'wp_ajax_get_proof_global_stats_data', array(
				$this,
				'get_proof_global_stats_data'
			), 200 );

			add_action( 'wp_ajax_cs_export_proof_consent', array(
				$this,
				'cs_export_proof_consent'
			), 200 );

			add_action( 'wp_ajax_cs_export_proof_consent_done', array(
				$this,
				'cs_export_proof_consent_done'
			), 200 );

			add_action( 'wp_ajax_cs_update_scan_page', array(
				$this,
				'cs_update_scan_page'
			), 200 );

			add_action( 'wp_ajax_cs_load_langs', array(
				$this,
				'cs_load_langs'
			), 200 );

			add_action( 'wp_ajax_cs_export_settings', array(
				$this,
				'cs_export_settings'
			), 200 );

			add_action( 'wp_ajax_cs_import_settings', array(
				$this,
				'cs_import_settings'
			), 200 );
		}

		if ( is_plugin_activated() && ConsentMagic()->check_first_flow() ) {
			add_action( 'admin_bar_menu', array(
				$this,
				'cs_admin_bar_delete_test_button'
			), 200 );
			add_action( 'admin_bar_menu', array(
				$this,
				'cs_admin_bar_clear_test_consent'
			), 200 );

			add_action( 'admin_footer', function() {
				echo "<div class='cs_preview_container cs_admin_preview_container " . ( !is_rtl() ? '' : 'rtl' ) . "'>
						    <div class='preview_wrap'></div>
					 </div>";
			} );
		}

		// hook that function onto our scheduled event:
		add_action( 'cs_db_cron_update_hook', array(
			$this,
			'cs_force_update'
		), 200 );
		add_action( 'cs_db_cron_update_hook_one_time', array(
			$this,
			'cs_force_update'
		), 200, 1 );

		add_action( 'init', function() {
			add_filter( 'cron_schedules', array(
				$this,
				'custom_cron_job_recurrence'
			), 200 );
		} );

		add_action( 'admin_enqueue_scripts', array(
			$this,
			'load_media_files'
		), 200 );

		if ( CMPRO_LICENSE_TYPE == 'edd' ) {
			add_action( 'cs_cron_check_status_edd_hook', array(
				$this,
				'cs_cron_check_edd_status'
			), 200 );
		} elseif ( CMPRO_LICENSE_TYPE == 'woo' ) {
			add_action( 'cs_cron_check_status_hook', array(
				$this,
				'cs_cron_check_status'
			), 200 );
		}

	}

	public function load_media_files() {
		wp_enqueue_media();
	}

	/**
	 * Register the stylesheets for the admin area.
	 */
	public function enqueue_styles() {

		/**
		 * This function is provided for demonstration purposes only.
		 * An instance of this class should be passed to the run() function
		 * defined in CS_Loader as all of the hooks are defined
		 * in that particular class.
		 * The ConsentMagic will then create the relationship
		 * between the defined hooks and the functions defined in this
		 * class.
		 */
		if ( in_array( getCurrentAdminPage(), $this->adminPagesSlugs ) ) {
			wp_enqueue_style( 'datatables', CMPRO_PLUGIN_URL
			                                . "assets/css/datatables.min.css", array(), CMPRO_LATEST_VERSION_NUMBER );
			wp_enqueue_style( 'daterangepicker', CMPRO_PLUGIN_URL
			                                     . "assets/css/datarangepicker.min.css", array(), CMPRO_LATEST_VERSION_NUMBER );
			wp_enqueue_style( 'wp-color-picker' );
			wp_enqueue_style( 'code-editor' );
			wp_enqueue_style( 'CS-Style', CMPRO_PLUGIN_URL
			                              . "assets/css/style-admin.min.css", array(), CMPRO_LATEST_VERSION_NUMBER );
			wp_enqueue_style( $this->plugin_name . '-select2', CMPRO_PLUGIN_URL
			                                                   . "assets/css/select2.min.css", array(), CMPRO_LATEST_VERSION_NUMBER );
			wp_enqueue_style( 'cm-noto-sans-font', 'https://fonts.googleapis.com/css2?family=Noto+Sans:ital,wdth,wght@0,62.5..100,100..900;1,62.5..100,100..900&display=swap', [], null );
		}
	}

	/**
	 * Register the JavaScript for the admin area.
	 */
	public function enqueue_scripts() {

		/**
		 * This function is provided for demonstration purposes only.
		 * An instance of this class should be passed to the run() function
		 * defined in CS_Loader as all of the hooks are defined
		 * in that particular class.
		 * The ConsentMagic will then create the relationship
		 * between the defined hooks and the functions defined in this
		 * class.
		 */
		wp_enqueue_script( 'CS-Magic-notice', CMPRO_PLUGIN_URL . 'assets/scripts/cs-notice.min.js', array(
			'jquery'
		), null, false );

		wp_localize_script( 'CS-Magic-notice', 'cmpro_ajax', array(
			'ajaxurl' => admin_url( 'admin-ajax.php' ),
			'nonce'   => wp_create_nonce( 'cs-ajax-nonce' )
		) );

		if ( in_array( getCurrentAdminPage(), $this->adminPagesSlugs ) ) {

			//Reregister tinymce
			wp_register_script( 'wp-tinymce', includes_url( 'js/tinymce/tinymce.min.js' ), array(), get_bloginfo( 'version' ) );
			wp_enqueue_script( 'wp-tinymce' );

			wp_enqueue_script( 'moment' );
			wp_enqueue_script( 'datarangepicker', CMPRO_PLUGIN_URL
			                                      . 'assets/scripts/datarangepicker.min.js', array( 'jquery' ), CMPRO_LATEST_VERSION_NUMBER, true );
			wp_enqueue_script( 'data-tables', CMPRO_PLUGIN_URL
			                                  . 'assets/scripts/data-tables.min.js', array( 'jquery' ), CMPRO_LATEST_VERSION_NUMBER, true );
			wp_enqueue_script( 'code-editor' );
			wp_enqueue_script( 'CS-Select2', CMPRO_PLUGIN_URL
			                                 . 'assets/scripts/select2.min.js', array( 'jquery' ), CMPRO_LATEST_VERSION_NUMBER, true );
			wp_enqueue_script( 'CS-jquery-ui', CMPRO_PLUGIN_URL
			                                   . 'assets/scripts/jquery-ui.min.js', array( 'jquery' ), CMPRO_LATEST_VERSION_NUMBER, true );
			wp_enqueue_script( 'CS-Magic', CMPRO_PLUGIN_URL . 'assets/scripts/cs-admin.min.js', array(
				'jquery',
				'wp-color-picker',
				'wp-tinymce'
			), time(), time() );
			wp_enqueue_script( 'CS-Magic-Tables', CMPRO_PLUGIN_URL . 'assets/scripts/cs-tables-functions.min.js', array(
				'jquery'
			), CMPRO_LATEST_VERSION_NUMBER, false );
			$cs_settings[ 'codeEditor' ] = wp_enqueue_code_editor( [ 'type' => 'text/html' ] );
			wp_localize_script( 'code-editor', 'cs_settings', $cs_settings );

			if ( ConsentMagic()->check_first_flow()
			     && is_plugin_activated()
			     && get_current_screen()->id == 'consentmagic_page_cs-iab' ) {

				wp_enqueue_script( 'CS-dayjs', CMPRO_PLUGIN_URL
				                               . 'assets/scripts/dayjs.min.js', array(), CMPRO_LATEST_VERSION_NUMBER, true );
				wp_enqueue_script( 'CS-dayjs-duration', CMPRO_PLUGIN_URL
				                                        . 'assets/scripts/dayjs-duration.min.js', array(), CMPRO_LATEST_VERSION_NUMBER, true );
				wp_localize_script( 'CS-Magic', 'CS_Admin_Data', array(
					'cs_vendor_list'  => CS_IAB_Integration()->get_vendor_list(),
					'cs_translations' => array(
						'cs_iab_name'                => __( 'Name', 'consent-magic' ),
						'cs_iab_domain'              => __( 'Domain', 'consent-magic' ),
						'cs_iab_purposes'            => __( 'Purposes', 'consent-magic' ),
						'cs_iab_expiry'              => __( 'Expiry', 'consent-magic' ),
						'cs_iab_type'                => __( 'Type', 'consent-magic' ),
						'cs_iab_cookie_details'      => __( 'Cookie details', 'consent-magic' ),
						'cs_iab_nodata'              => __( 'No data', 'consent-magic' ),
						'cs_iab_privacy_policy'      => __( 'Privacy policy', 'consent-magic' ),
						'cs_iab_legitimate_interest' => __( 'Legitimate interest', 'consent-magic' ),
						'cs_iab_days'                => __( '% day(s)', 'consent-magic' ),
						'cs_iab_special_purposes'    => __( 'Special purposes', 'consent-magic' ),
						'cs_iab_features'            => __( 'Features', 'consent-magic' ),
						'cs_iab_special_features'    => __( 'Special features', 'consent-magic' ),
						'cs_iab_data_categories'     => __( 'Data categories', 'consent-magic' ),
						'cs_iab_cookie_refreshed'    => __( 'Cookie expiry may be refreshed', 'consent-magic' ),
						'cs_iab_storage_methods'     => __( 'Storage methods or accessing', 'consent-magic' ),
						'cs_iab_cookies_and_others'  => __( 'Cookies and others', 'consent-magic' ),
						'cs_iab_other_methods'       => __( 'Other methods', 'consent-magic' ),
						'cs_iab_consent_preferences' => __( 'Consent preferences', 'consent-magic' ),
						'cs_iab_show_cookie_details' => __( 'Show cookie details', 'consent-magic' ),
						'cs_iab_years'               => __( '% year(s)', 'consent-magic' ),
						'cs_iab_months'              => __( '% month(s)', 'consent-magic' ),
						'cs_iab_hours'               => __( '% hour(s)', 'consent-magic' ),
						'cs_iab_minutes'             => __( '%minutes(s)', 'consent-magic' ),
					),
					'cs_iab'          => array(
						'current_iab_lang' => CS_IAB_Integration()->get_current_iab_lang(),
					),
				) );
			}
		}
	}

	function cs_export_proof_consent() {

		check_ajax_referer( 'cs-ajax-nonce', 'nonce_code' );
		$this->manageCSPermissionsAjax();

		global $wpdb;
		$table        = $wpdb->prefix . 'cs_proof_consent';
		$submit       = ( !empty( $_POST[ 'submit' ] ) ) ? sanitize_text_field( $_POST[ 'submit' ] ) : false;
		$this->offset = intval( sanitize_text_field( $_POST[ 'offset' ] ) );
		$this->limit  = intval( sanitize_text_field( $_POST[ 'limit' ] ) );

		$step_data = array();

		$export_keys = array(
			'UUID'         => 'uuid',
			'Time'         => 'created_at',
			'URL'          => 'url',
			'IP'           => 'ip',
			'User'         => 'email',
			'User profile' => 'profile',
			'Rule'         => 'rule',
			'Consent type' => 'consent_type',
			'Consents'     => 'category',
			'Action'       => 'current_action',
		);

		if ( $this->limit == 'undefined' || $this->limit == 0 ) {
			$this->limit = 100;
			$this->clear_export_proof_consent( $export_keys, 'export_proof_consent.csv' );
		}

		$query_from    = $wpdb->prepare( 'FROM %1$s WHERE 1=1 ', $table );
		$consent_array = $wpdb->get_results( $wpdb->prepare( 'SELECT * %1$s LIMIT %2$d OFFSET %3$d', $query_from, $this->limit, $this->offset ) );

		$total_consent_number = $wpdb->get_var( $wpdb->prepare( 'SELECT COUNT(*) FROM %1$s', $table ) );

		$array = [];
		foreach ( $consent_array as $item ) {
			$array[ 'UUID' ]         = $item->uuid;
			$array[ 'Time' ]         = $item->created_at;
			$array[ 'URL' ]          = $item->url;
			$array[ 'IP' ]           = $item->ip;
			$array[ 'User' ]         = $item->email;
			$array[ 'User profile' ] = wp_strip_all_tags( $item->profile );
			$array[ 'Rule' ]         = $item->rule;
			$array[ 'Consent type' ] = $item->consent_type;
			$array[ 'Consents' ]     = strip_tags( $item->category );
			$array[ 'Action' ]       = $item->current_action;
			array_push( $step_data, $array );
		}

		$new_offset = $this->offset + $this->limit;

		if ( ( $total_consent_number - $new_offset ) < $this->limit ) {
			$this->limit = $total_consent_number - $new_offset;
		}

		$data_result = array(
			"limit"                 => "$this->limit",
			"offset"                => "$new_offset",
			"total_products_number" => $total_consent_number,
			"loop"                  => ( $new_offset < $total_consent_number ) ? "1" : "0",
			"step_data"             => $step_data,
			"file_url"              => $this->export_proof_consent( $step_data, $export_keys, 'export_proof_consent.csv' )
			                           . '?ver=' . time(),
		);

		if ( ( $new_offset >= $total_consent_number ) && ( $submit == 'true' ) ) {
			$table  = $wpdb->prefix . 'cs_proof_consent';
			$delete = $wpdb->query( $wpdb->prepare( 'DELETE FROM %1$s', $table ) );
			$this->renew_consent_run();
		}

		wp_send_json( $data_result );
		wp_die();
	}

	function cs_export_proof_consent_done() {
		check_ajax_referer( 'cs-ajax-nonce', 'nonce_code' );
		$this->manageCSPermissionsAjax();
		unlink( CMPRO_PLUGIN_PATH . 'export_proof_consent.csv' );
		wp_die();
	}

	function clear_export_proof_consent( $export_keys, $file_name ) {

		$file     = CMPRO_PLUGIN_PATH . $file_name;
		$file_url = CMPRO_PLUGIN_URL . $file_name;
		# Generate CSV data from array
		$fh = fopen( $file, 'w' );
		# to use memory instead
		# write out the headers
		fputcsv( $fh, $export_keys, ";", '"', '' );
		fclose( $fh );

		return $file_url;
	}

	function export_proof_consent( $items, $export_keys, $file_name ) {

		$file     = CMPRO_PLUGIN_PATH . $file_name;
		$file_url = CMPRO_PLUGIN_URL . $file_name;
		# Generate CSV data from array
		$fh = fopen( $file, 'a+' );
		# to use memory instead

		# write out the headers
		foreach ( $items as $item ) {
			unset( $csv_line );
			foreach ( $export_keys as $key => $value ) {
				if ( isset( $item[ $key ] ) ) {
					$csv_line[] = $item[ $key ];
				}
			}
			if ( isset( $csv_line ) ) {
				fputcsv( $fh, $csv_line, ";", '"', '' );
			}
		}
		fclose( $fh );

		return $file_url;
	}

	function get_proof_consent_data() {
		check_ajax_referer( 'cs-ajax-nonce', 'nonce_code' );
		$this->manageCSPermissionsAjax();

		array_walk( $_REQUEST, array(
			ConsentMagic(),
			'cs_sanitize_array'
		) );
		$request_escaped = $_REQUEST;

		global $wpdb;
		$table_name = $wpdb->prefix . 'cs_proof_consent';

		$cs_proof_consent = $wpdb->get_results( $wpdb->prepare( 'SELECT * FROM %1$s', $table_name ) );

		if ( !empty( $cs_proof_consent ) ) {
			foreach ( $cs_proof_consent as $item ) {
				$item->details_button = cardCollapseSettingsWithText( 'Details', 'Close', false, true );
				$item->category       = '<div class="consent-types">' . $item->category . '</div>';
			}
		}

		$json_data = array(
			"draw" => intval( ( isset( $request_escaped[ "draw" ] ) ? $request_escaped[ "draw" ] : 0 ) ),
			"data" => $cs_proof_consent
		);

		echo json_encode( $json_data );
		wp_die(); //to remove that 0 response
	}

	function get_ignore_ip_table() {
		check_ajax_referer( 'cs-ajax-nonce', 'nonce_code', false );
		$this->manageCSPermissionsAjax();

		array_walk( $_REQUEST, array(
			ConsentMagic(),
			'cs_sanitize_array'
		) );
		$request_escaped = $_REQUEST;

		global $wpdb;
		$table_name = $wpdb->prefix . 'cs_unblock_ip';

		## Date search value
		$searchByFromdate = sanitize_text_field( $_POST[ 'searchByFromdate' ] );
		$searchByTodate   = sanitize_text_field( $_POST[ 'searchByTodate' ] );

		$searchByFromdate_sql = gmdate( 'Y-m-d H:i:s', strtotime( str_replace( '-', '/', $searchByFromdate ) ) );
		$searchByTodate_sql   = gmdate( 'Y-m-d H:i:s', strtotime( str_replace( '-', '/', $searchByTodate )
		                                                          . ' +1 day' ) );

		$table_data = array();
		## Total number of records without filtering
		if ( $searchByFromdate != '' && $searchByTodate != '' ) {
			$ips                   = $wpdb->get_results( $wpdb->prepare( 'SELECT * FROM %1$s WHERE 1 AND `created_at` <= \'%2$s\' AND `created_at` >= \'%3$s\'', $table_name, $searchByTodate_sql, $searchByFromdate_sql ) );
			$totalRecords          = count( $ips );
			$totalRecordwithFilter = count( $ips );
		} else {
			$ips                   = $wpdb->get_results( $wpdb->prepare( 'SELECT * FROM %1$s', $table_name ) );
			$totalRecords          = count( $ips );
			$totalRecordwithFilter = count( $ips );
		}
		if ( $ips ) {
			foreach ( $ips as $ip ) {
				$id = $ip->id;
				if ( $searchByFromdate != '' && $searchByTodate != '' ) {
					$data         = $wpdb->get_results( $wpdb->prepare( 'SELECT * FROM %1$s WHERE `id` = %2$d AND `created_at` <= \'%3$s\' AND `created_at` >= \'%4$s\'', $table_name, $id, $searchByTodate_sql, $searchByFromdate_sql ) );
					$table_data[] = array(
						'id'         => $data[ 0 ]->id,
						'ip'         => $data[ 0 ]->ip,
						'created_at' => $data[ 0 ]->created_at,
					);
				} else {
					$data         = $wpdb->get_results( $wpdb->prepare( 'SELECT * FROM %1$s WHERE `id` = %2$d', $table_name, $id ) );
					$table_data[] = array(
						'id'         => $data[ 0 ]->id,
						'ip'         => $data[ 0 ]->ip,
						'created_at' => $data[ 0 ]->created_at,
					);
				}
			}
		}

		$json_data = array(
			"draw"                 => intval( ( isset( $request_escaped[ "draw" ] ) ? $request_escaped[ "draw" ] : 0 ) ),
			"iTotalRecords"        => $totalRecords,
			"iTotalDisplayRecords" => $totalRecordwithFilter,
			"aaData"               => $table_data
		);

		echo json_encode( $json_data );
		wp_die(); //to remove that 0 response
	}

	function remove_ignore_ip_table() {
		check_ajax_referer( 'cs-ajax-nonce', 'nonce_code', false );
		$this->manageCSPermissionsAjax();

		global $wpdb;
		$table_name = $wpdb->prefix . 'cs_unblock_ip';
		$id         = sanitize_text_field( $_POST[ 'id' ] );

		$ips = $wpdb->get_results( $wpdb->prepare( 'SELECT * FROM %1$s WHERE `id` = %2$d', $table_name, $id ) );

		if ( $ips ) {
			$wpdb->get_results( $wpdb->prepare( 'DELETE FROM %1$s WHERE `id` = %2$d', $table_name, $id ) );
		}

		wp_die(); //to remove that 0 response
	}

	function cs_new_ignore_ip() {
		check_ajax_referer( 'cs-ajax-nonce', 'nonce_code', false );
		$this->manageCSPermissionsAjax();

		global $wpdb;
		$table_name      = $wpdb->prefix . 'cs_unblock_ip';
		$ignore_ip       = isset( $_POST[ 'ignore_ip' ] ) ? sanitize_text_field( $_POST[ 'ignore_ip' ] ) : false;
		$check_duplicate = false;

		if ( $ignore_ip ) {
			$like            = '%' . $wpdb->esc_like( $ignore_ip ) . '%';
			$check_duplicate = $wpdb->get_results( $wpdb->prepare( 'SELECT * FROM %1$s WHERE `ip` LIKE \'%2$s\'', $table_name, $like ) );
		}

		if ( $check_duplicate ) {
			wp_send_json_error();
		} else {
			$data = array(
				'ip'         => $ignore_ip,
				'created_at' => current_time( 'mysql' ),
			);
			$wpdb->insert( $table_name, $data );
			wp_send_json_success();
		}

		wp_die();
	}

	function get_scan_cookies() {
		check_ajax_referer( 'cs-ajax-nonce', 'nonce_code', false );
		$this->manageCSPermissionsAjax();

		array_walk( $_REQUEST, array(
			ConsentMagic(),
			'cs_sanitize_array'
		) );
		$request_escaped = $_REQUEST;

		$table_data           = get_scan_cookies_list();
		$result_data          = array();
		$temp_data_unassigned = array();
		$temp_data_other      = array();

		if ( $table_data ) {

			foreach ( $table_data as $item ) {
				if ( $item[ 'term_slug' ] === 'unassigned' ) {
					$temp_data_unassigned[] = $item;
				} else {
					$temp_data_other[] = $item;
				}
			}
		}

		if ( $temp_data_unassigned ) {
			foreach ( $temp_data_unassigned as $item ) {
				array_push( $result_data, $item );
			}
		}

		if ( $temp_data_other ) {
			foreach ( $temp_data_other as $item ) {
				array_push( $result_data, $item );
			}
		}

		$totalRecords          = count( $table_data );
		$totalRecordwithFilter = count( $table_data );

		$json_data = array(
			"draw"                 => intval( ( isset( $request_escaped[ "draw" ] ) ? $request_escaped[ "draw" ] : 0 ) ),
			"iTotalRecords"        => $totalRecords,
			"iTotalDisplayRecords" => $totalRecordwithFilter,
			"aaData"               => $result_data
		);

		echo json_encode( $json_data );
		wp_die(); //to remove that 0 response
	}

	function get_scan_scripts() {
		check_ajax_referer( 'cs-ajax-nonce', 'nonce_code', false );
		$this->manageCSPermissionsAjax();

		array_walk( $_REQUEST, array(
			ConsentMagic(),
			'cs_sanitize_array'
		) );
		$request_escaped = $_REQUEST;

		$table_data = get_scan_scripts_list();

		$totalRecords          = count( $table_data );
		$totalRecordwithFilter = count( $table_data );

		$json_data = array(
			"draw"                 => intval( ( isset( $request_escaped[ "draw" ] ) ? $request_escaped[ "draw" ] : 0 ) ),
			"iTotalRecords"        => $totalRecords,
			"iTotalDisplayRecords" => $totalRecordwithFilter,
			"aaData"               => $table_data
		);

		echo json_encode( $json_data );
		wp_die(); //to remove that 0 response
	}

	function cs_update_scan_page() {
		check_ajax_referer( 'cs_scanner', 'security' );
		$this->manageCSPermissionsAjax();

		$cs_scan_existing_page = sanitize_text_field( $_POST[ 'cs_scan_existing_page' ] );

		ConsentMagic()->updateOptions( array( 'cs_auto_scan_type' => $cs_scan_existing_page ) );

		$return = array(
			'message' => 'cs_auto_scan_type updated',
			'ID'      => $cs_scan_existing_page
		);

		wp_send_json_success( $return );
		wp_die();
	}

	function save_scan_data() {
		check_ajax_referer( 'cs-ajax-nonce', 'nonce_code' );
		$this->manageCSPermissionsAjax();

		$cookie_id  = isset( $_POST[ 'cookie_id' ] ) ? sanitize_text_field( $_POST[ 'cookie_id' ] ) : null;
		$script_id  = isset( $_POST[ 'script_id' ] ) ? sanitize_text_field( $_POST[ 'script_id' ] ) : null;
		$descr_text = isset( $_POST[ 'descr_text' ] ) ? wp_kses_post( wp_unslash( $_POST[ 'descr_text' ] ) ) : null;

		if ( $cookie_id ) {
			$data_arr = array(
				"description" => $descr_text
			);
			$update   = update_scan_cookie_data( $cookie_id, $data_arr );
		}

		if ( $script_id ) {
			$data_arr = array(
				"description" => $descr_text
			);
			$update   = update_scan_script_data( $script_id, $data_arr );

			if ( !$update ) {
				$template_data = array(
					'ID'         => $script_id,
					'meta_input' => array(
						'cs_default_script_desc' => $descr_text
					)
				);

				wp_update_post( $template_data );
			}
		}

		$response = array( 'success' => true );

		wp_send_json_success( $response );
		wp_die();
	}

	function get_manual_added_scripts() {
		check_ajax_referer( 'cs-ajax-nonce', 'nonce_code', false );
		$this->manageCSPermissionsAjax();

		array_walk( $_REQUEST, array(
			ConsentMagic(),
			'cs_sanitize_array'
		) );
		$request_escaped = $_REQUEST;

		$table_data = get_custom_scripts_list();

		$totalRecords          = count( $table_data );
		$totalRecordwithFilter = count( $table_data );

		$json_data = array(
			"draw"                 => intval( ( isset( $request_escaped[ "draw" ] ) ? $request_escaped[ "draw" ] : 0 ) ),
			"iTotalRecords"        => $totalRecords,
			"iTotalDisplayRecords" => $totalRecordwithFilter,
			"aaData"               => $table_data
		);

		echo json_encode( $json_data );
		wp_die(); //to remove that 0 response
	}

	function get_proof_rules_stats_data() {
		check_ajax_referer( 'cs-ajax-nonce', 'nonce_code', false );
		$this->manageCSPermissionsAjax();

		array_walk( $_REQUEST, array(
			ConsentMagic(),
			'cs_sanitize_array'
		) );
		$request_escaped = $_REQUEST;

		global $wpdb;
		$table_name = $wpdb->prefix . 'cs_stats_consent';

		## Date search value
		$searchByFromdate = sanitize_text_field( $_POST[ 'searchByFromdate' ] );
		$searchByTodate   = sanitize_text_field( $_POST[ 'searchByTodate' ] );

		$searchByFromdate_sql = gmdate( 'Y-m-d H:i:s', strtotime( str_replace( '-', '/', $searchByFromdate ) ) );
		$searchByTodate_sql   = gmdate( 'Y-m-d H:i:s', strtotime( str_replace( '-', '/', $searchByTodate )
		                                                          . ' +1 day' ) );

		$table_data = array();
		## Total number of records without filtering
		if ( $searchByFromdate != '' && $searchByTodate != '' ) {
			$proof_rules = $wpdb->get_results( $wpdb->prepare( 'SELECT DISTINCT `rule_id` FROM %1$s WHERE 1 AND `created_at` <= \'%2$s\' AND `created_at` >= \'%3$s\'', $table_name, $searchByTodate_sql, $searchByFromdate_sql ) );
		} else {
			$proof_rules = $wpdb->get_results( $wpdb->prepare( 'SELECT DISTINCT `rule_id` FROM %1$s', $table_name ) );
		}
		$totalRecords          = count( $proof_rules );
		$totalRecordwithFilter = count( $proof_rules );

		if ( $proof_rules ) {
			foreach ( $proof_rules as $rule ) {
				$rule_id   = $rule->rule_id;
				$rule_name = get_the_title( $rule_id );

				if ( $searchByFromdate != '' && $searchByTodate != '' ) {
					$table_data[] = array(
						'rule_name'       => $rule_name,
						'allow_all'       => $wpdb->get_var( $wpdb->prepare( 'SELECT COUNT(*) FROM %1$s 
	                                    WHERE `rule_id` = \'%2$d\' 
	                                    AND `current_action` LIKE \'%3$s\'
	                                    AND `created_at` <= \'%4$s\' 
	                                    AND `created_at` >= \'%5$s\'', $table_name, $rule_id, '%'
						                                                                      . $wpdb->esc_like( 'Allow all' )
						                                                                      . '%', $searchByTodate_sql, $searchByFromdate_sql ) ),
						'disallow_all'    => $wpdb->get_var( $wpdb->prepare( 'SELECT COUNT(*) FROM %1$s 
	                                    WHERE `rule_id` = \'%2$d\' 
	                                    AND `current_action` LIKE \'%3$s\'
	                                    AND `created_at` <= \'%4$s\' 
	                                    AND `created_at` >= \'%5$s\'', $table_name, $rule_id, '%'
						                                                                      . $wpdb->esc_like( 'Disable all' )
						                                                                      . '%', $searchByTodate_sql, $searchByFromdate_sql ) ),
						'close_on_scroll' => $wpdb->get_var( $wpdb->prepare( 'SELECT COUNT(*) FROM %1$s 
	                                    WHERE `rule_id` = \'%2$d\' 
	                                    AND `current_action` LIKE \'%3$s\'
	                                    AND `created_at` <= \'%4$s\' 
	                                    AND `created_at` >= \'%5$s\'', $table_name, $rule_id, '%'
						                                                                      . $wpdb->esc_like( 'Close on Scroll' )
						                                                                      . '%', $searchByTodate_sql, $searchByFromdate_sql ) ),
						'confirm_choices' => $wpdb->get_var( $wpdb->prepare( 'SELECT COUNT(*) FROM %1$s 
	                                    WHERE `rule_id` = \'%2$d\' 
	                                    AND `current_action` LIKE \'%3$s\'
	                                    AND `created_at` <= \'%4$s\' 
	                                    AND `created_at` >= \'%5$s\'', $table_name, $rule_id, '%'
						                                                                      . $wpdb->esc_like( 'Confirm my choices' )
						                                                                      . '%', $searchByTodate_sql, $searchByFromdate_sql ) ),
						'close_consent'   => $wpdb->get_var( $wpdb->prepare( 'SELECT COUNT(*) FROM %1$s 
	                                    WHERE `rule_id` = \'%2$d\' 
	                                    AND `current_action` LIKE \'%3$s\'
	                                    AND `created_at` <= \'%4$s\' 
	                                    AND `created_at` >= \'%5$s\'', $table_name, $rule_id, '%'
						                                                                      . $wpdb->esc_like( 'Close consent' )
						                                                                      . '%', $searchByTodate_sql, $searchByFromdate_sql ) ),
						'close_opt_popup' => $wpdb->get_var( $wpdb->prepare( 'SELECT COUNT(*) FROM %1$s 
	                                    WHERE `rule_id` = \'%2$d\' 
	                                    AND `current_action` LIKE \'%3$s\'
	                                    AND `created_at` <= \'%4$s\' 
	                                    AND `created_at` >= \'%5$s\'', $table_name, $rule_id, '%'
						                                                                      . $wpdb->esc_like( 'Close Options Popup' )
						                                                                      . '%', $searchByTodate_sql, $searchByFromdate_sql ) ),
					);
				} else {
					$table_data[] = array(
						'rule_name'       => $rule_name,
						'allow_all'       => $wpdb->get_var( $wpdb->prepare( 'SELECT COUNT(*) FROM %1$s 
	                                    WHERE `rule_id` = \'%2$d\' 
	                                    AND `current_action` LIKE \'%3$s\'', $table_name, $rule_id, '%'
						                                                                            . $wpdb->esc_like( 'Allow all' )
						                                                                            . '%' ) ),
						'disallow_all'    => $wpdb->get_var( $wpdb->prepare( 'SELECT COUNT(*) FROM %1$s 
	                                    WHERE `rule_id` = \'%2$d\' 
	                                    AND `current_action` LIKE \'%3$s\'', $table_name, $rule_id, '%'
						                                                                            . $wpdb->esc_like( 'Disable all' )
						                                                                            . '%' ) ),
						'close_on_scroll' => $wpdb->get_var( $wpdb->prepare( 'SELECT COUNT(*) FROM %1$s 
	                                    WHERE `rule_id` = \'%2$d\' 
	                                    AND `current_action` LIKE \'%3$s\'', $table_name, $rule_id, '%'
						                                                                            . $wpdb->esc_like( 'Close on Scroll' )
						                                                                            . '%' ) ),
						'confirm_choices' => $wpdb->get_var( $wpdb->prepare( 'SELECT COUNT(*) FROM %1$s 
	                                    WHERE `rule_id` = \'%2$d\' 
	                                    AND `current_action` LIKE \'%3$s\'', $table_name, $rule_id, '%'
						                                                                            . $wpdb->esc_like( 'Confirm my choices' )
						                                                                            . '%' ) ),
						'close_consent'   => $wpdb->get_var( $wpdb->prepare( 'SELECT COUNT(*) FROM %1$s 
	                                    WHERE `rule_id` = \'%2$d\' 
	                                    AND `current_action` LIKE \'%3$s\'', $table_name, $rule_id, '%'
						                                                                            . $wpdb->esc_like( 'Close consent' )
						                                                                            . '%' ) ),
						'close_opt_popup' => $wpdb->get_var( $wpdb->prepare( 'SELECT COUNT(*) FROM %1$s 
	                                    WHERE `rule_id` = \'%2$d\' 
	                                    AND `current_action` LIKE \'%3$s\'', $table_name, $rule_id, '%'
						                                                                            . $wpdb->esc_like( 'Close Options Popup' )
						                                                                            . '%' ) ),
					);
				}
			}
		}

		$json_data = array(
			"draw"                 => intval( ( isset( $request_escaped[ "draw" ] ) ? $request_escaped[ "draw" ] : 0 ) ),
			"iTotalRecords"        => $totalRecords,
			"iTotalDisplayRecords" => $totalRecordwithFilter,
			"aaData"               => $table_data
		);

		echo json_encode( $json_data );
		wp_die(); //to remove that 0 response
	}

	function get_proof_cs_type_stats_data() {
		check_ajax_referer( 'cs-ajax-nonce', 'nonce_code', false );
		$this->manageCSPermissionsAjax();

		array_walk( $_REQUEST, array(
			ConsentMagic(),
			'cs_sanitize_array'
		) );
		$request_escaped = $_REQUEST;

		global $wpdb;
		$table_name            = $wpdb->prefix . 'cs_stats_consent';
		$totalRecords          = 3;
		$totalRecordwithFilter = 3;
		## Date search value
		$searchByFromdate = sanitize_text_field( $_POST[ 'searchByFromdate' ] );
		$searchByTodate   = sanitize_text_field( $_POST[ 'searchByTodate' ] );

		$searchByFromdate_sql = gmdate( 'Y-m-d H:i:s', strtotime( str_replace( '-', '/', $searchByFromdate ) ) );
		$searchByTodate_sql   = gmdate( 'Y-m-d H:i:s', strtotime( str_replace( '-', '/', $searchByTodate )
		                                                          . ' +1 day' ) );
		$proof_type           = array(
			'Just inform',
			'Inform and Opt-out',
			'Ask before tracking',
			'IAB',
		);
		$table_data           = array();

		if ( $proof_type ) {
			foreach ( $proof_type as $rule ) {

				if ( $searchByFromdate != '' && $searchByTodate != '' ) {
					$table_data[] = array(
						'consent_type'    => $rule,
						'allow_all'       => $wpdb->get_var( $wpdb->prepare( 'SELECT COUNT(*) FROM %1$s
                					WHERE `consent_type` = \'%2$s\'
                					AND `current_action` LIKE \'%3$s\'
                					AND `created_at` <= \'%4$s\'
                					AND `created_at` >= \'%5$s\'', $table_name, $rule, '%'
						                                                               . $wpdb->esc_like( 'Allow all' )
						                                                               . '%', $searchByTodate_sql, $searchByFromdate_sql ) ),
						'disallow_all'    => $wpdb->get_var( $wpdb->prepare( 'SELECT COUNT(*) FROM %1$s
                					WHERE `consent_type` = \'%2$s\'
                					AND `current_action` LIKE \'%3$s\'
                					AND `created_at` <= \'%4$s\'
                					AND `created_at` >= \'%5$s\'', $table_name, $rule, '%'
						                                                               . $wpdb->esc_like( 'Disable all' )
						                                                               . '%', $searchByTodate_sql, $searchByFromdate_sql ) ),
						'close_on_scroll' => $wpdb->get_var( $wpdb->prepare( 'SELECT COUNT(*) FROM %1$s
                					WHERE `consent_type` = \'%2$s\'
                					AND `current_action` LIKE \'%3$s\'
                					AND `created_at` <= \'%4$s\'
                					AND `created_at` >= \'%5$s\'', $table_name, $rule, '%'
						                                                               . $wpdb->esc_like( 'Close on Scroll' )
						                                                               . '%', $searchByTodate_sql, $searchByFromdate_sql ) ),
						'confirm_choices' => $wpdb->get_var( $wpdb->prepare( 'SELECT COUNT(*) FROM %1$s
                					WHERE `consent_type` = \'%2$s\'
                					AND `current_action` LIKE \'%3$s\'
                					AND `created_at` <= \'%4$s\'
                					AND `created_at` >= \'%5$s\'', $table_name, $rule, '%'
						                                                               . $wpdb->esc_like( 'Confirm my choices' )
						                                                               . '%', $searchByTodate_sql, $searchByFromdate_sql ) ),
						'close_consent'   => $wpdb->get_var( $wpdb->prepare( 'SELECT COUNT(*) FROM %1$s
                					WHERE `consent_type` = \'%2$s\'
                					AND `current_action` LIKE \'%3$s\'
                					AND `created_at` <= \'%4$s\'
                					AND `created_at` >= \'%5$s\'', $table_name, $rule, '%'
						                                                               . $wpdb->esc_like( 'Close consent' )
						                                                               . '%', $searchByTodate_sql, $searchByFromdate_sql ) ),
						'close_opt_popup' => $wpdb->get_var( $wpdb->prepare( 'SELECT COUNT(*) FROM %1$s
                					WHERE `consent_type` = \'%2$s\'
                					AND `current_action` LIKE \'%3$s\'
                					AND `created_at` <= \'%4$s\'
                					AND `created_at` >= \'%5$s\'', $table_name, $rule, '%'
						                                                               . $wpdb->esc_like( 'Close Options Popup' )
						                                                               . '%', $searchByTodate_sql, $searchByFromdate_sql ) ),
					);
				} else {
					$table_data[] = array(
						'consent_type'    => $rule,
						'allow_all'       => $wpdb->get_var( $wpdb->prepare( 'SELECT COUNT(*) FROM %1$s 
                					WHERE `consent_type` = \'%2$s\' 
                					AND `current_action` LIKE \'%3$s\'', $table_name, $rule, '%'
						                                                                     . $wpdb->esc_like( 'Allow all' )
						                                                                     . '%' ) ),
						'disallow_all'    => $wpdb->get_var( $wpdb->prepare( 'SELECT COUNT(*) FROM %1$s 
                					WHERE `consent_type` = \'%2$s\' 
                					AND `current_action` LIKE \'%3$s\'', $table_name, $rule, '%'
						                                                                     . $wpdb->esc_like( 'Disable all' )
						                                                                     . '%' ) ),
						'close_on_scroll' => $wpdb->get_var( $wpdb->prepare( 'SELECT COUNT(*) FROM %1$s 
                					WHERE `consent_type` = \'%2$s\' 
                					AND `current_action` LIKE \'%3$s\'', $table_name, $rule, '%'
						                                                                     . $wpdb->esc_like( 'Close on Scroll' )
						                                                                     . '%' ) ),
						'confirm_choices' => $wpdb->get_var( $wpdb->prepare( 'SELECT COUNT(*) FROM %1$s 
                					WHERE `consent_type` = \'%2$s\' 
                					AND `current_action` LIKE \'%3$s\'', $table_name, $rule, '%'
						                                                                     . $wpdb->esc_like( 'Confirm my choices' )
						                                                                     . '%' ) ),
						'close_consent'   => $wpdb->get_var( $wpdb->prepare( 'SELECT COUNT(*) FROM %1$s 
                					WHERE `consent_type` = \'%2$s\' 
                					AND `current_action` LIKE \'%3$s\'', $table_name, $rule, '%'
						                                                                     . $wpdb->esc_like( 'Close consent' )
						                                                                     . '%' ) ),
						'close_opt_popup' => $wpdb->get_var( $wpdb->prepare( 'SELECT COUNT(*) FROM %1$s 
                					WHERE `consent_type` = \'%2$s\' 
                					AND `current_action` LIKE \'%3$s\'', $table_name, $rule, '%'
						                                                                     . $wpdb->esc_like( 'Close Options Popup' )
						                                                                     . '%' ) ),
					);
				}
			}
		}

		$json_data = array(
			"draw"                 => intval( ( isset( $request_escaped[ "draw" ] ) ? $request_escaped[ "draw" ] : 0 ) ),
			"iTotalRecords"        => $totalRecords,
			"iTotalDisplayRecords" => $totalRecordwithFilter,
			"aaData"               => $table_data
		);

		echo json_encode( $json_data );
		wp_die(); //to remove that 0 response
	}

	function get_proof_global_stats_data() {
		check_ajax_referer( 'cs-ajax-nonce', 'nonce_code', false );
		$this->manageCSPermissionsAjax();

		array_walk( $_REQUEST, array(
			ConsentMagic(),
			'cs_sanitize_array'
		) );
		$request_escaped = $_REQUEST;

		global $wpdb;
		$table_name            = $wpdb->prefix . 'cs_stats_consent';
		$totalRecords          = 4;
		$totalRecordwithFilter = 4;
		## Date search value
		$searchByFromdate = sanitize_text_field( $_POST[ 'searchByFromdate' ] );
		$searchByTodate   = sanitize_text_field( $_POST[ 'searchByTodate' ] );

		$searchByFromdate_sql = gmdate( 'Y-m-d H:i:s', strtotime( str_replace( '-', '/', $searchByFromdate ) ) );
		$searchByTodate_sql   = gmdate( 'Y-m-d H:i:s', strtotime( str_replace( '-', '/', $searchByTodate )
		                                                          . ' +1 day' ) );
		$actions              = array(
			'Allow All',
			'Disable all',
			'Close on scroll',
			'Confirm my choices',
			'Close consent',
			'Close Options Popup'
		);
		$table_data           = array();

		if ( $actions ) {
			foreach ( $actions as $action ) {

				if ( $searchByFromdate != '' && $searchByTodate != '' ) {
					$table_data[] = array(
						'action' => $action,
						'count'  => $wpdb->get_var( $wpdb->prepare( 'SELECT COUNT(*) FROM %1$s
                					WHERE `current_action` LIKE \'%2$s\'
                					AND `created_at` <= \'%3$s\'
                					AND `created_at` >= \'%4$s\'', $table_name, '%' . $wpdb->esc_like( $action )
						                                                        . '%', $searchByTodate_sql, $searchByFromdate_sql ) ),
					);
				} else {
					$table_data[] = array(
						'action' => $action,
						'count'  => $wpdb->get_var( $wpdb->prepare( 'SELECT COUNT(*) FROM %1$s
                					WHERE `current_action` LIKE \'%2$s\'', $table_name, '%' . $wpdb->esc_like( $action )
						                                                                . '%' ) ),
					);
				}
			}
		}

		$json_data = array(
			"draw"                 => intval( ( isset( $request_escaped[ "draw" ] ) ? $request_escaped[ "draw" ] : 0 ) ),
			"iTotalRecords"        => $totalRecords,
			"iTotalDisplayRecords" => $totalRecordwithFilter,
			"aaData"               => $table_data
		);

		echo json_encode( $json_data );
		wp_die(); //to remove that 0 response
	}

	function cs_cron_check_status() {

		if ( CMPRO_LICENSE_TYPE == 'edd' ) {
			$license_status = $this->edd_license_key_status();
			db_cron_update();

			if ( isset( $license_status[ 'success' ] ) && isset( $license_status[ 'license' ] ) ) {
				if ( $license_status[ 'success' ] === false && $license_status[ 'license' ] === 'invalid' ) {
					ConsentMagic()->updateOptions( array( $this->edd_activated_key => 'Deactivated' ) );
					ConsentMagic()->updateOptions( array( $this->edd_deactivate_checkbox_key => 'on' ) );
					ConsentMagic()->updateOptions( array( $this->edd_license_key => '' ) );
				}
			}
		} elseif ( CMPRO_LICENSE_TYPE == 'woo' ) {
			$license_status = $this->license_key_status();
			db_cron_update();

			if ( isset( $license_status[ 'status_check' ] ) && $license_status[ 'status_check' ] == 'inactive' ) {
				ConsentMagic()->updateOptions( array( 'wc_am_client_consent_magic_pro_activated' => 'Canceled' ) );
			}
			if ( isset( $license_status[ 'code' ] ) && $license_status[ 'code' ] == '100' ) {
				ConsentMagic()->updateOptions( array( 'wc_am_client_consent_magic_pro_activated' => 'Expired' ) );
			}
		}
	}

	/**
	 * Deletes all data if plugin deactivated
	 */
	public function cs_instance_update() {

		check_ajax_referer( 'cs-ajax-nonce', 'nonce_code' );
		$this->manageCSPermissionsAjax();

		ConsentMagic()->updateOptions( array( 'wc_am_client_consent_magic_pro_instance' => wp_generate_password( 12, false ) ) );
		ConsentMagic()->deleteOption( 'wc_am_client_consent_magic_pro_activated' );
		ConsentMagic()->deleteOption( 'wc_am_client_consent_magic_pro' );
		ConsentMagic()->deleteOption( 'wc_am_client_consent_magic_pro_deactivate_checkbox' );
		ConsentMagic()->deleteOption( 'edd_license_key' );
		ConsentMagic()->deleteOption( 'edd_consent_magic_pro' );
		ConsentMagic()->deleteOption( 'edd_consent_magic_pro_activated' );
		ConsentMagic()->deleteOption( 'edd_consent_magic_pro_deactivate_checkbox' );
		wp_die();
	}

	// Custom Cron Recurrences
	function custom_cron_job_recurrence( $schedules ) {

		$schedules[ 'daily' ] = array(
			'display'  => esc_html__( 'Once a week', 'consent-magic' ),
			'interval' => 86400,
		);

		$schedules[ 'weekly' ] = array(
			'display'  => esc_html__( 'Once a week', 'consent-magic' ),
			'interval' => 604800,
		);

		$schedules[ 'monthly' ] = array(
			'display'  => esc_html__( 'Once a month', 'consent-magic' ),
			'interval' => 2635200,
		);

		$schedules[ 'one_half_year' ] = array(
			'display'  => esc_html__( 'Once time in half a year', 'consent-magic' ),
			'interval' => 15768000,
		);

		$schedules[ 'yearly' ] = array(
			'display'  => esc_html__( 'Once a year', 'consent-magic' ),
			'interval' => 31536000,
		);

		return $schedules;
	}

	public function get_plugin_options() {

		return ConsentMagic()->cs_options;
	}

	function cs_save_item_order() {
		// Check nonce:
		check_admin_referer( 'cs-update-' . CMPRO_SETTINGS_FIELD );
		$this->manageCSPermissionsAjax();

		$order_post = ( isset( $_POST[ 'order' ] )
		                && !empty( (int) $_POST[ 'order' ] ) ) ? sanitize_text_field( $_POST[ 'order' ] ) : array();

		$order = explode( ',', $order_post );

		$counter = 0;
		foreach ( $order as $item_id ) {
			update_post_meta( $item_id, '_cs_order', $counter );
			$counter++;
		}

		$data = array(
			'order' => $order
		);
		wp_send_json_success( $data );
		wp_die();
	}

	function cs_predefined_rule_enable_update() {
		// Check nonce:
		check_admin_referer( 'cs-update-' . CMPRO_SETTINGS_FIELD );
		$this->manageCSPermissionsAjax();

		$id = ( isset( $_POST[ 'id' ] )
		        && !empty( (int) $_POST[ 'id' ] ) ) ? (int) sanitize_text_field( $_POST[ 'id' ] ) : null;

		$active = ( isset( $_POST[ 'active' ] )
		            && !empty( $_POST[ 'active' ] ) ) ? sanitize_text_field( $_POST[ 'active' ] ) : 0;

		update_post_meta( $id, '_cs_enable_rule', "$active" );
		$data = array(
			'id'     => $id,
			'active' => $active
		);
		wp_send_json_success( $data );
		wp_die();
	}

	function cs_no_ip_rule_update() {
		// Check nonce:
		check_admin_referer( 'cs-update-' . CMPRO_SETTINGS_FIELD );
		$this->manageCSPermissionsAjax();

		$id = ( isset( $_POST[ 'id' ] )
		        && !empty( (int) $_POST[ 'id' ] ) ) ? (int) sanitize_text_field( $_POST[ 'id' ] ) : null;

		$active = ( isset( $_POST[ 'active' ] )
		            && !empty( $_POST[ 'active' ] ) ) ? sanitize_text_field( $_POST[ 'active' ] ) : 0;

		update_post_meta( $id, '_cs_no_ip_rule', "$active" );
		$data = array(
			'id'     => $id,
			'active' => $active
		);
		wp_send_json_success( $data );
		wp_die();
	}

	function cs_update_primary_rule() {
		// Check nonce:
		check_admin_referer( 'cs-update-' . CMPRO_SETTINGS_FIELD );

		foreach ( $_POST as $item_name => $val ) {
			if ( isset( $_POST[ 'cs_rule_id' ] )
			     && get_post_meta( sanitize_text_field( $_POST[ 'cs_rule_id' ] ), sanitize_text_field( $item_name ) ) ) {
				update_post_meta( sanitize_text_field( $_POST[ 'cs_rule_id' ] ), sanitize_text_field( $item_name ), sanitize_text_field( $val ) );
			}
		}

		wp_send_json_success();
	}

	function cs_admin_bar_delete_test_button( $wp_admin_bar ) {
		$cs_options = $this->get_plugin_options();
		if ( $cs_options[ 'cs_test_mode' ] ) {
			$wp_admin_bar->add_menu( array(
				'id'    => 'delete_test_consent',
				'title' => __( 'Disable ConsentMagic test mode', 'consent-magic' ),
				'href'  => wp_nonce_url( admin_url( 'admin.php?page=cs-settings' )
				                         . '&cs_update_action=cs_test_mode_disable', 'cs-update-'
				                                                                     . CMPRO_SETTINGS_FIELD ),
			) );
		}
	}

	function cs_admin_bar_clear_test_consent( $wp_admin_bar ) {
		$cs_options = $this->get_plugin_options();
		if ( $cs_options[ 'cs_test_mode' ] ) {
			$wp_admin_bar->add_menu( array(
				'id'    => 'cs_test_consent_clear',
				'title' => __( 'Delete test consent', 'consent-magic' ),
				'href'  => wp_nonce_url( admin_url( 'admin.php?page=cs-settings' )
				                         . '&cs_update_action=cs_test_consent_clear', 'cs-update-'
				                                                                      . CMPRO_SETTINGS_FIELD ),
			) );
		}
	}

	function cs_force_update( $first_time = false ) {

		if ( !wp_doing_cron() ) {
			check_ajax_referer( 'cs-ajax-nonce', 'nonce_code' );
			$this->manageCSPermissionsAjax();
		}

		$data = array(
			'message' => '',
			'type'    => 'success'
		);

		if ( ( (int) ConsentMagic()->getOption( 'cs_geolocation' ) === 1
		       && ConsentMagic()->getOption( 'cs_geo_activated' ) == true )
		     || $first_time ) {
			$data = $this->cs_force_update_runner();
		} else {

			if ( (int) ConsentMagic()->getOption( 'cs_geolocation' ) === 0 ) {
				$data = array(
					'message' => esc_html__( 'Geolocation is deactivated. Please, enable it.', 'consent-magic' ),
					'type'    => 'error'
				);
			} elseif ( ConsentMagic()->getOption( 'cs_geo_activated' ) == false ) {
				$data = array(
					'message' => esc_html__( 'The MaxMind license key or Account ID is invalid.', 'consent-magic' ),
					'type'    => 'error'
				);
			}
		}

		if ( wp_doing_ajax() ) {
			wp_send_json_success( $data );
			wp_die();
		}
	}

	/**
	 * Force update geolocation DB runner
	 * @return array
	 */
	public function cs_force_update_runner() {
		$cs_geo  = new CS_Geoip;
		$message = array(
			'message' => '',
			'type'    => 'success'
		);
		foreach ( $cs_geo->databases as $database ) {
			$result = $cs_geo->update_database( $database );
			if ( is_wp_error( $result ) ) {
				$message = array(
					'message' => $result->get_error_message( 'geo_error' ),
					'type'    => 'error'
				);
				break;
			} else {
				$message[ 'message' ] = esc_html__( 'Geo Database updated successfully', 'consent-magic' );
			}
		}

		return $message;
	}

	/**
	 * Registers menu options
	 * Hooked into admin_menu
	 */
	public function admin_menu() {
		global $submenu;

		if ( is_plugin_activated() ) {
			add_menu_page( 'ConsentMagic', 'ConsentMagic', 'manage_cs', 'consent-magic', array(
				$this->adminPages[ 'main' ],
				'renderPage'
			), CMPRO_PLUGIN_URL . '/assets/images/cm-logo.png' );

			add_submenu_page( 'consent-magic', 'Settings', 'Settings', 'manage_cs', 'cs-settings', array(
				$this->adminPages[ 'settings' ],
				'renderPage'
			) );

			add_submenu_page( 'consent-magic', 'IAB Settings', 'IAB Settings', 'manage_cs', 'cs-iab', array(
				$this->adminPages[ 'iab' ],
				'renderPage'
			) );

			add_submenu_page( 'consent-magic', 'Geolocation', 'Geolocation', 'manage_cs', 'cs-geolocation', array(
				$this->adminPages[ 'geolocation' ],
				'renderPage'
			) );

			add_submenu_page( 'consent-magic', 'Proof of consent storage', 'Consent Proof', 'manage_cs', 'cs-proof-consent', array(
				$this->adminPages[ 'proof-consent' ],
				'renderPage'
			) );

			add_submenu_page( 'consent-magic', 'Proof of consent Statistics', 'Statistics', 'manage_cs', 'cs-proof-statistics', array(
				$this->adminPages[ 'proof-statistics' ],
				'renderPage',
			) );

			add_submenu_page( 'consent-magic', 'Shortcodes & Filters', 'Shortcodes & Filters', 'manage_cs', 'cs-additionals', array(
				$this->adminPages[ 'shortcodes' ],
				'renderPage',
			) );

			add_submenu_page( 'consent-magic', 'License', 'License', 'manage_cs', 'cs-license', array(
				$this->adminPages[ 'license' ],
				'renderPage',
			) );

			// rename first submenu item
			if ( isset( $submenu[ 'consent-magic' ] ) ) {
				$submenu[ 'consent-magic' ][ 0 ][ 0 ] = 'Dashboard';
			}
		} else {
			add_menu_page( 'ConsentMagic', 'ConsentMagic', 'manage_cs', 'cs-license', array(
				$this->adminPages[ 'license' ],
				'renderPage',
			), CMPRO_PLUGIN_URL . '/assets/images/cm-logo.png' );

			// core admin pages
			$this->adminPagesSlugs = array(
				'cs-license'
			);
		}
	}

	/**
	 * Returns the API Key status by querying the Status API function from the WooCommerce API Manager on the server.
	 * @return array|mixed|object
	 */
	public function license_key_status() {

		$status = $this->status();

		return !empty( $status ) ? json_decode( $this->status(), true ) : $status;
	}

	public function edd_license_key_status() {

		$status = $this->edd_status();

		return !empty( $status ) ? json_decode( $this->edd_status(), true ) : $status;
	}

	/**
	 * Sends the status check request to the API Manager.
	 * @return bool|string
	 */
	public function status() {

		if ( empty( $this->data[ $this->wc_am_api_key_key ] ) ) {
			return '';
		}

		$defaults = array(
			'wc_am_action' => 'status',
			'api_key'      => $this->data[ $this->wc_am_api_key_key ],
			'product_id'   => $this->product_id,
			'instance'     => $this->wc_am_instance_id,
			'object'       => $this->wc_am_domain
		);

		$target_url = esc_url_raw( $this->create_software_api_url( $defaults ) );
		$request    = wp_safe_remote_post( $target_url );

		if ( is_wp_error( $request ) || wp_remote_retrieve_response_code( $request ) != 200 ) {
			// Request failed
			return '';
		}

		return wp_remote_retrieve_body( $request );
	}

	public function edd_status() {

		$api_params = array(
			'edd_action' => 'check_license',
			'license'    => ConsentMagic()->getOption( $this->edd_license_key ),
			'item_name'  => $this->product_name,
			'url'        => home_url()
		);

		$request = wp_remote_post( $this->url_activate, array(
			'timeout'   => 15,
			'sslverify' => false,
			'body'      => $api_params
		) );

		if ( is_wp_error( $request ) || wp_remote_retrieve_response_code( $request ) != 200 ) {
			// Request failed
			return '';
		}

		return wp_remote_retrieve_body( $request );
	}

	/**
	 * Builds the URL containing the API query string for activation, deactivation, and status requests.
	 * @param array $args
	 * @return string
	 */
	public function create_software_api_url( $args ) {
		return add_query_arg( 'wc-api', 'wc-am-api', esc_url( 'https://www.pixelyoursite.com/' ) ) . '&'
		       . http_build_query( $args );
	}

	function cs_preview_show() {
		// Check nonce:
		check_admin_referer( 'cs-update-' . CMPRO_SETTINGS_FIELD );
		$this->manageCSPermissionsAjax();

		$id                      = isset( $_POST[ 'cs_theme' ] ) ? sanitize_text_field( $_POST[ 'cs_theme' ] ) : '';
		$post_id                 = isset( $_POST[ 'cs_rule_id' ] ) ? sanitize_text_field( $_POST[ 'cs_rule_id' ] ) : '';
		$cs_bars_type            = isset( $_POST[ 'cs_bar_type' ] ) ? sanitize_text_field( $_POST[ 'cs_bar_type' ] ) : '';
		$cs_bars_position        = isset( $_POST[ 'cs_bar_position' ] ) ? sanitize_text_field( $_POST[ 'cs_bar_position' ] ) : '';
		$cs_type                 = isset( $_POST[ 'cs_type' ] ) ? sanitize_text_field( $_POST[ 'cs_type' ] ) : '';
		$cs_sticky               = isset( $_POST[ 'cs_sticky' ] ) ? sanitize_text_field( $_POST[ 'cs_sticky' ] ) : '';
		$cs_hide_close_btn       = isset( $_POST[ 'cs_hide_close_btn' ] ) ? sanitize_text_field( $_POST[ 'cs_hide_close_btn' ] ) : '';
		$cs_hide_close_btn_admin = true;
		$cs_deny_all_btn         = isset( $_POST[ 'cs_deny_all_btn' ] ) ? sanitize_text_field( $_POST[ 'cs_deny_all_btn' ] ) : '';
		$cs_custom_button_btn    = isset( $_POST[ 'cs_custom_button' ] ) ? sanitize_text_field( $_POST[ 'cs_custom_button' ] ) : '';
		$cs_privacy_link         = isset( $_POST[ 'cs_privacy' ] ) ? sanitize_text_field( $_POST[ 'cs_privacy' ] ) : '';
		$cs_design_type          = isset( $_POST[ 'cs_design_type' ] ) ? sanitize_text_field( $_POST[ 'cs_design_type' ] ) : '';

		$active_rule_id = !empty( $post_id ) ? $post_id : ConsentMagic()->get_preview_rule_id();

		$cs_preview_popup_admin = true;
		ob_start();

		echo '<div id="cs_preview_popup">';

		if ( $cs_type == 'iab' || $cs_design_type == 'single' ) {
			include_once CMPRO_PLUGIN_VIEWS_PATH . 'templates/single/cs_single_design.php';
		} else {
			if ( $cs_bars_type == 'bar_small' || $cs_bars_type == 'bar_large' ) {
				include CMPRO_PLUGIN_VIEWS_PATH . 'templates/multi/cs-bar.php';
			} else if ( $cs_bars_type == 'popup_small' || $cs_bars_type == 'popup_large' ) {
				include CMPRO_PLUGIN_VIEWS_PATH . 'templates/multi/cs_popup.php';
			}
		}

		echo '</div>';

		add_filter( 'safe_style_css', array(
			$this,
			'add_style_option'
		) );

		$this->add_preview_styles();
		remove_filter( 'safe_style_css', array(
			$this,
			'add_style_option'
		) );

		$html = ob_get_contents();
		ob_end_clean();
		wp_send_json_success( array( 'html' => $html ) );
		wp_die(); // ajax call must die to avoid trailing 0 in your response
	}

	function cs_preview_list_show() {
		// Check nonce:
		check_admin_referer( 'cs-update-' . CMPRO_SETTINGS_FIELD );
		$this->manageCSPermissionsAjax();

		ob_start();
		$active_rule_id       = isset( $_POST[ 'cs_rule_id' ] ) ? sanitize_text_field( $_POST[ 'cs_rule_id' ] ) : '';
		$rule_obj             = get_post_meta( $active_rule_id );
		$id                   = $rule_obj[ '_cs_theme' ][ 0 ];
		$cs_design_type       = $rule_obj[ '_cs_design_type' ][ 0 ];
		$cs_bars_type         = $rule_obj[ '_cs_bars_type' ][ 0 ];
		$cs_bars_position     = $rule_obj[ '_cs_bars_position' ][ 0 ];
		$cs_type              = $rule_obj[ '_cs_type' ][ 0 ];
		$cs_privacy_link      = $rule_obj[ '_cs_privacy_link' ][ 0 ];
		$cs_deny_all_btn      = $rule_obj[ '_cs_deny_all_btn' ][ 0 ];
		$cs_custom_button_btn = $rule_obj[ '_cs_custom_button' ][ 0 ];

		$cs_preview_popup_admin = true;
		echo '<div id="cs_preview_popup">';

		$cs_hide_close_btn       = $rule_obj[ '_cs_hide_close_btn' ][ 0 ];
		$cs_hide_close_btn_admin = true;
		if ( $cs_design_type == 'single' ) {
			include_once CMPRO_PLUGIN_VIEWS_PATH . 'templates/single/cs_single_design.php';
		} else {
			if ( $cs_bars_type == 'bar_small' || $cs_bars_type == 'bar_large' ) {
				include CMPRO_PLUGIN_VIEWS_PATH . 'templates/multi/cs-bar.php';
			} else if ( $cs_bars_type == 'popup_small' || $cs_bars_type == 'popup_large' ) {
				include CMPRO_PLUGIN_VIEWS_PATH . 'templates/multi/cs_popup.php';
			}
		}

		echo '</div>';

		add_filter( 'safe_style_css', array(
			$this,
			'add_style_option'
		) );

		$this->add_preview_styles();
		remove_filter( 'safe_style_css', array(
			$this,
			'add_style_option'
		) );

		$html = ob_get_contents();
		ob_end_clean();
		wp_send_json_success( array( 'html' => $html ) );
		wp_die(); // ajax call must die to avoid trailing 0 in your response
	}

	/**
	 * Preview design show
	 * @return void
	 */
	function cs_preview_design_show() {
		// Check nonce:
		check_admin_referer( 'cs-update-' . CMPRO_SETTINGS_FIELD );
		$this->manageCSPermissionsAjax();

		ob_start();
		$active_rule_id = ConsentMagic()->get_preview_rule_id();
		$rule_obj       = get_post_meta( $active_rule_id );
		$cs_type        = $rule_obj[ '_cs_type' ][ 0 ];
		$cs_bars_type   = isset( $_POST[ 'cs_bars_type' ] ) ? sanitize_title( $_POST[ 'cs_bars_type' ] ) : $rule_obj[ '_cs_bars_type' ][ 0 ];

		if ( $cs_bars_type == 'popup_large_single' && $cs_type == 'just_inform' ) {
			$rule_id = ConsentMagic()->get_active_rule_for_single_design();
			if ( $rule_id ) {
				$active_rule_id = $rule_id;
				$cs_type        = get_post_meta( $active_rule_id, '_cs_type', true );
			}
		}

		$id                   = isset( $_POST[ 'cs_theme' ] ) ? sanitize_title( $_POST[ 'cs_theme' ] ) : $rule_obj[ '_cs_theme' ][ 0 ];
		$cs_design_type       = $rule_obj[ '_cs_design_type' ][ 0 ];
		$cs_bars_position     = $rule_obj[ '_cs_bars_position' ][ 0 ];
		$cs_privacy_link      = $rule_obj[ '_cs_privacy_link' ][ 0 ];
		$cs_deny_all_btn      = $rule_obj[ '_cs_deny_all_btn' ][ 0 ];
		$cs_custom_button_btn = $rule_obj[ '_cs_custom_button' ][ 0 ];

		$cs_preview_popup_admin = true;
		echo '<div id="cs_preview_popup">';

		$cs_hide_close_btn       = $rule_obj[ '_cs_hide_close_btn' ][ 0 ];
		$cs_hide_close_btn_admin = true;

		if ( $cs_bars_type == 'popup_large_single' ) {
			if ( $cs_type == 'just_inform' ) {
				esc_html_e( 'There are no configured rules for single step design', 'consent-magic' );
			} else {
				include_once CMPRO_PLUGIN_VIEWS_PATH . 'templates/single/cs_single_design.php';
			}
		} else {
			if ( $cs_bars_type == 'bar_small' || $cs_bars_type == 'bar_large' ) {
				include CMPRO_PLUGIN_VIEWS_PATH . 'templates/multi/cs-bar.php';
			} else if ( $cs_bars_type == 'popup_small' || $cs_bars_type == 'popup_large' ) {
				include CMPRO_PLUGIN_VIEWS_PATH . 'templates/multi/cs_popup.php';
			}
		}

		echo '</div>';

		add_filter( 'safe_style_css', array(
			$this,
			'add_style_option'
		) );

		$this->add_preview_styles();
		remove_filter( 'safe_style_css', array(
			$this,
			'add_style_option'
		) );

		$html = ob_get_contents();
		ob_end_clean();
		wp_send_json_success( array( 'html' => $html ) );
		wp_die(); // ajax call must die to avoid trailing 0 in your response
	}

	function cs_preview_size_show() {
		// Check nonce:
		check_admin_referer( 'cs-update-' . CMPRO_SETTINGS_FIELD );
		$this->manageCSPermissionsAjax();

		ob_start();
		$active_rule_id          = ConsentMagic()->get_preview_rule_id();
		$metas                   = get_post_meta( $active_rule_id );
		$id                      = $metas[ '_cs_theme' ][ 0 ];
		$cs_type                 = $metas[ '_cs_type' ][ 0 ];
		$cs_privacy_link         = $metas[ '_cs_privacy_link' ][ 0 ];
		$cs_bars_position        = $metas[ '_cs_bars_position' ][ 0 ];
		$cs_deny_all_btn         = $metas[ '_cs_deny_all_btn' ][ 0 ];
		$cs_hide_close_btn       = $metas[ '_cs_hide_close_btn' ][ 0 ];
		$cs_hide_close_btn_admin = true;
		$cs_custom_button_btn    = $metas[ '_cs_custom_button' ][ 0 ];
		$cs_bars_type            = isset( $_POST[ 'bar_type' ] ) ? sanitize_text_field( $_POST[ 'bar_type' ] ) : '';

		$min_height = isset( $_POST[ 'min_height' ] ) ? sanitize_text_field( $_POST[ 'min_height' ] ) : '';
		$pd_top     = isset( $_POST[ 'pd_top' ] ) ? sanitize_text_field( $_POST[ 'pd_top' ] ) : '';
		$pd_bottom  = isset( $_POST[ 'pd_bottom' ] ) ? sanitize_text_field( $_POST[ 'pd_bottom' ] ) : '';
		$pd_left    = isset( $_POST[ 'pd_left' ] ) ? sanitize_text_field( $_POST[ 'pd_left' ] ) : '';
		$pd_right   = isset( $_POST[ 'pd_right' ] ) ? sanitize_text_field( $_POST[ 'pd_right' ] ) : '';

		$font_s = isset( $_POST[ 'font_s' ] ) ? sanitize_text_field( $_POST[ 'font_s' ] ) : '';
		$font_w = isset( $_POST[ 'font_w' ] ) ? sanitize_text_field( $_POST[ 'font_w' ] ) : '';

		$btn_pd_t   = isset( $_POST[ 'btn_pd_t' ] ) ? sanitize_text_field( $_POST[ 'btn_pd_t' ] ) : '';
		$btn_pd_b   = isset( $_POST[ 'btn_pd_b' ] ) ? sanitize_text_field( $_POST[ 'btn_pd_b' ] ) : '';
		$btn_pd_l   = isset( $_POST[ 'btn_pd_l' ] ) ? sanitize_text_field( $_POST[ 'btn_pd_l' ] ) : '';
		$btn_pd_r   = isset( $_POST[ 'btn_pd_r' ] ) ? sanitize_text_field( $_POST[ 'btn_pd_r' ] ) : '';
		$btn_mg_t   = isset( $_POST[ 'btn_mg_t' ] ) ? sanitize_text_field( $_POST[ 'btn_mg_t' ] ) : '';
		$btn_mg_b   = isset( $_POST[ 'btn_mg_b' ] ) ? sanitize_text_field( $_POST[ 'btn_mg_b' ] ) : '';
		$btn_mg_l   = isset( $_POST[ 'btn_mg_l' ] ) ? sanitize_text_field( $_POST[ 'btn_mg_l' ] ) : '';
		$btn_mg_r   = isset( $_POST[ 'btn_mg_r' ] ) ? sanitize_text_field( $_POST[ 'btn_mg_r' ] ) : '';
		$btn_font_s = isset( $_POST[ 'btn_font_s' ] ) ? sanitize_text_field( $_POST[ 'btn_font_s' ] ) : '';
		$btn_font_w = isset( $_POST[ 'btn_font_w' ] ) ? sanitize_text_field( $_POST[ 'btn_font_w' ] ) : '';

		$op_min_height = isset( $_POST[ 'op_min_height' ] ) ? sanitize_text_field( $_POST[ 'op_min_height' ] ) : '';
		$op_pd_top     = isset( $_POST[ 'op_pd_top' ] ) ? sanitize_text_field( $_POST[ 'op_pd_top' ] ) : '';
		$op_pd_bottom  = isset( $_POST[ 'op_pd_bottom' ] ) ? sanitize_text_field( $_POST[ 'op_pd_bottom' ] ) : '';
		$op_pd_left    = isset( $_POST[ 'op_pd_left' ] ) ? sanitize_text_field( $_POST[ 'op_pd_left' ] ) : '';
		$op_pd_right   = isset( $_POST[ 'op_pd_right' ] ) ? sanitize_text_field( $_POST[ 'op_pd_right' ] ) : '';

		$op_font_s = isset( $_POST[ 'op_font_s' ] ) ? sanitize_text_field( $_POST[ 'op_font_s' ] ) : '';
		$op_font_w = isset( $_POST[ 'op_font_w' ] ) ? sanitize_text_field( $_POST[ 'op_font_w' ] ) : '';

		$op_btn_pd_t   = isset( $_POST[ 'op_btn_pd_t' ] ) ? sanitize_text_field( $_POST[ 'op_btn_pd_t' ] ) : '';
		$op_btn_pd_b   = isset( $_POST[ 'op_btn_pd_b' ] ) ? sanitize_text_field( $_POST[ 'op_btn_pd_b' ] ) : '';
		$op_btn_pd_l   = isset( $_POST[ 'op_btn_pd_l' ] ) ? sanitize_text_field( $_POST[ 'op_btn_pd_l' ] ) : '';
		$op_btn_pd_r   = isset( $_POST[ 'op_btn_pd_r' ] ) ? sanitize_text_field( $_POST[ 'op_btn_pd_r' ] ) : '';
		$op_btn_mg_t   = isset( $_POST[ 'op_btn_mg_t' ] ) ? sanitize_text_field( $_POST[ 'op_btn_mg_t' ] ) : '';
		$op_btn_mg_b   = isset( $_POST[ 'op_btn_mg_b' ] ) ? sanitize_text_field( $_POST[ 'op_btn_mg_b' ] ) : '';
		$op_btn_mg_l   = isset( $_POST[ 'op_btn_mg_l' ] ) ? sanitize_text_field( $_POST[ 'op_btn_mg_l' ] ) : '';
		$op_btn_mg_r   = isset( $_POST[ 'op_btn_mg_r' ] ) ? sanitize_text_field( $_POST[ 'op_btn_mg_r' ] ) : '';
		$op_btn_font_s = isset( $_POST[ 'op_btn_font_s' ] ) ? sanitize_text_field( $_POST[ 'op_btn_font_s' ] ) : '';
		$op_btn_font_w = isset( $_POST[ 'op_btn_font_w' ] ) ? sanitize_text_field( $_POST[ 'op_btn_font_w' ] ) : '';

		$op_title_f_s    = isset( $_POST[ 'op_title_f_s' ] ) ? sanitize_text_field( $_POST[ 'op_title_f_s' ] ) : '';
		$op_title_f_w    = isset( $_POST[ 'op_title_f_w' ] ) ? sanitize_text_field( $_POST[ 'op_title_f_w' ] ) : '';
		$op_subtitle_f_s = isset( $_POST[ 'op_subtitle_f_s' ] ) ? sanitize_text_field( $_POST[ 'op_subtitle_f_s' ] ) : '';
		$op_subtitle_f_w = isset( $_POST[ 'op_subtitle_f_w' ] ) ? sanitize_text_field( $_POST[ 'op_subtitle_f_w' ] ) : '';

		$cs_preview_size_show   = true;
		$cs_preview_popup_admin = true;

		if ( $cs_bars_type == 'bar_small' || $cs_bars_type == 'bar_large' ) {
			include_once CMPRO_PLUGIN_VIEWS_PATH . 'templates/multi/cs-bar.php';
		} else if ( $cs_bars_type == 'popup_small' || $cs_bars_type == 'popup_large' ) {
			include_once CMPRO_PLUGIN_VIEWS_PATH . 'templates/multi/cs_popup.php';
		}

		add_filter( 'safe_style_css', array(
			$this,
			'add_style_option'
		) );

		$this->add_preview_styles();
		remove_filter( 'safe_style_css', array(
			$this,
			'add_style_option'
		) );

		$html = ob_get_contents();
		ob_end_clean();
		wp_send_json_success( array( 'html' => $html ) );
		wp_die(); // ajax call must die to avoid trailing 0 in your response
	}

	/**
	 * Preview for single step design
	 * @return void
	 */
	function cs_preview_size_single_show() {
		// Check nonce:
		check_admin_referer( 'cs-update-' . CMPRO_SETTINGS_FIELD );
		$this->manageCSPermissionsAjax();

		ob_start();
		$active_rule_id = ConsentMagic()->get_preview_rule_id();
		$cs_type        = get_post_meta( $active_rule_id, '_cs_type', true );
		if ( $cs_type == 'just_inform' ) {
			$rule_id = ConsentMagic()->get_active_rule_for_single_design();
			if ( $rule_id ) {
				$active_rule_id = $rule_id;
				$cs_type        = get_post_meta( $active_rule_id, '_cs_type', true );
			}
		}

		$metas                   = get_post_meta( $active_rule_id );
		$id                      = $metas[ '_cs_theme' ][ 0 ];
		$cs_privacy_link         = $metas[ '_cs_privacy_link' ][ 0 ];
		$cs_bars_position        = $metas[ '_cs_bars_position' ][ 0 ];
		$cs_deny_all_btn         = $metas[ '_cs_deny_all_btn' ][ 0 ];
		$cs_hide_close_btn       = $metas[ '_cs_hide_close_btn' ][ 0 ];
		$cs_hide_close_btn_admin = true;
		$cs_custom_button_btn    = $metas[ '_cs_custom_button' ][ 0 ];
		$cs_bars_type            = $metas[ '_cs_bars_type' ][ 0 ];

		$op_pd_t            = ( isset( $_POST[ 'op_pd_t' ] ) ? sanitize_text_field( $_POST[ 'op_pd_t' ] ) : '' ) ?: 0;
		$op_pd_r            = ( isset( $_POST[ 'op_pd_r' ] ) ? sanitize_text_field( $_POST[ 'op_pd_r' ] ) : '' ) ?: 0;
		$op_pd_b            = ( isset( $_POST[ 'op_pd_b' ] ) ? sanitize_text_field( $_POST[ 'op_pd_b' ] ) : '' ) ?: 0;
		$op_pd_l            = ( isset( $_POST[ 'op_pd_l' ] ) ? sanitize_text_field( $_POST[ 'op_pd_l' ] ) : '' ) ?: 0;
		$op_font_s          = isset( $_POST[ 'op_font_s' ] ) ? sanitize_text_field( $_POST[ 'op_font_s' ] ) : '';
		$op_font_w          = isset( $_POST[ 'op_font_w' ] ) ? sanitize_text_field( $_POST[ 'op_font_w' ] ) : '';
		$op_second_font_s   = isset( $_POST[ 'op_second_font_s' ] ) ? sanitize_text_field( $_POST[ 'op_second_font_s' ] ) : '';
		$op_second_font_w   = isset( $_POST[ 'op_second_font_w' ] ) ? sanitize_text_field( $_POST[ 'op_second_font_w' ] ) : '';
		$op_title_f_s       = isset( $_POST[ 'op_title_f_s' ] ) ? sanitize_text_field( $_POST[ 'op_title_f_s' ] ) : '';
		$op_title_f_w       = isset( $_POST[ 'op_title_f_w' ] ) ? sanitize_text_field( $_POST[ 'op_title_f_w' ] ) : '';
		$op_subtitle_f_s    = isset( $_POST[ 'op_subtitle_f_s' ] ) ? sanitize_text_field( $_POST[ 'op_subtitle_f_s' ] ) : '';
		$op_subtitle_f_w    = isset( $_POST[ 'op_subtitle_f_w' ] ) ? sanitize_text_field( $_POST[ 'op_subtitle_f_w' ] ) : '';
		$op_subsubtitle_f_s = isset( $_POST[ 'op_subsubtitle_f_s' ] ) ? sanitize_text_field( $_POST[ 'op_subsubtitle_f_s' ] ) : '';
		$op_subsubtitle_f_w = isset( $_POST[ 'op_subsubtitle_f_w' ] ) ? sanitize_text_field( $_POST[ 'op_subsubtitle_f_w' ] ) : '';
		$op_btn_pd_t        = ( isset( $_POST[ 'op_btn_pd_t' ] ) ? sanitize_text_field( $_POST[ 'op_btn_pd_t' ] ) : '' ) ?: 0;
		$op_btn_pd_r        = ( isset( $_POST[ 'op_btn_pd_r' ] ) ? sanitize_text_field( $_POST[ 'op_btn_pd_r' ] ) : '' ) ?: 0;
		$op_btn_pd_b        = ( isset( $_POST[ 'op_btn_pd_b' ] ) ? sanitize_text_field( $_POST[ 'op_btn_pd_b' ] ) : '' ) ?: 0;
		$op_btn_pd_l        = ( isset( $_POST[ 'op_btn_pd_l' ] ) ? sanitize_text_field( $_POST[ 'op_btn_pd_l' ] ) : '' ) ?: 0;
		$op_btn_mg_t        = ( isset( $_POST[ 'op_btn_mg_t' ] ) ? sanitize_text_field( $_POST[ 'op_btn_mg_t' ] ) : '' ) ?: 0;
		$op_btn_mg_b        = ( isset( $_POST[ 'op_btn_mg_b' ] ) ? sanitize_text_field( $_POST[ 'op_btn_mg_b' ] ) : '' ) ?: 0;
		$op_btn_mg_l        = ( isset( $_POST[ 'op_btn_mg_l' ] ) ? sanitize_text_field( $_POST[ 'op_btn_mg_l' ] ) : '' ) ?: 0;
		$op_btn_mg_r        = ( isset( $_POST[ 'op_btn_mg_r' ] ) ? sanitize_text_field( $_POST[ 'op_btn_mg_r' ] ) : '' ) ?: 0;
		$op_btn_font_s      = isset( $_POST[ 'op_btn_font_s' ] ) ? sanitize_text_field( $_POST[ 'op_btn_font_s' ] ) : '';
		$op_btn_font_w      = isset( $_POST[ 'op_btn_font_w' ] ) ? sanitize_text_field( $_POST[ 'op_btn_font_w' ] ) : '';

		$cs_preview_size_show   = true;
		$cs_preview_popup_admin = true;

		if ( $cs_type == 'just_inform' ) {
			esc_html_e( 'There are no configured rules for single step design', 'consent-magic' );
		} else {
			include_once CMPRO_PLUGIN_VIEWS_PATH . 'templates/single/cs_single_design.php';
		}

		add_filter( 'safe_style_css', array(
			$this,
			'add_style_option'
		) );

		$this->add_preview_styles();
		remove_filter( 'safe_style_css', array(
			$this,
			'add_style_option'
		) );

		$html = ob_get_contents();
		ob_end_clean();
		wp_send_json_success( array( 'html' => $html ) );
		wp_die(); // ajax call must die to avoid trailing 0 in your response
	}

	function cs_preview_color_show() {
		// Check nonce:
		check_admin_referer( 'cs-update-' . CMPRO_SETTINGS_FIELD );
		$this->manageCSPermissionsAjax();

		ob_start();
		$active_rule_id = ConsentMagic()->get_preview_rule_id();
		$cs_bars_type   = isset( $_POST[ 'bar_type' ] ) ? sanitize_text_field( $_POST[ 'bar_type' ] ) : '';
		$cs_type        = get_post_meta( $active_rule_id, '_cs_type', true );

		if ( $cs_bars_type == 'popup_large_single' && $cs_type == 'just_inform' ) {
			$rule_id = ConsentMagic()->get_active_rule_for_single_design();
			if ( $rule_id ) {
				$active_rule_id = $rule_id;
				$cs_type        = get_post_meta( $active_rule_id, '_cs_type', true );
			}
		}

		$metas                   = get_post_meta( $active_rule_id );
		$id                      = null;
		$cs_type                 = $metas[ '_cs_type' ][ 0 ];
		$cs_privacy_link         = $metas[ '_cs_privacy_link' ][ 0 ];
		$cs_bars_position        = $metas[ '_cs_bars_position' ][ 0 ];
		$cs_deny_all_btn         = $metas[ '_cs_deny_all_btn' ][ 0 ];
		$cs_custom_button_btn    = $metas[ '_cs_custom_button' ][ 0 ];
		$cs_hide_close_btn       = $metas[ '_cs_hide_close_btn' ][ 0 ];
		$cs_hide_close_btn_admin = true;

		$style_array = array(
			'cs_backend_color'                    => array( isset( $_POST[ 'cs_backend_color' ] ) ? sanitize_text_field( $_POST[ 'cs_backend_color' ] ) : '' ),
			'cs_border_style'                     => array( isset( $_POST[ 'cs_border_style' ] ) ? sanitize_text_field( $_POST[ 'cs_border_style' ] ) : '' ),
			'cs_border_weight'                    => array( isset( $_POST[ 'cs_border_weight' ] ) ? sanitize_text_field( $_POST[ 'cs_border_weight' ] ) : '' ),
			'cs_border_color'                     => array( isset( $_POST[ 'cs_border_color' ] ) ? sanitize_text_field( $_POST[ 'cs_border_color' ] ) : '' ),
			'cs_text_block_bg'                    => array( isset( $_POST[ 'cs_text_block_bg' ] ) ? sanitize_text_field( $_POST[ 'cs_text_block_bg' ] ) : '' ),
			'cs_text_color'                       => array( isset( $_POST[ 'cs_text_color' ] ) ? sanitize_text_field( $_POST[ 'cs_text_color' ] ) : '' ),
			'cs_links_color'                      => array( isset( $_POST[ 'cs_links_color' ] ) ? sanitize_text_field( $_POST[ 'cs_links_color' ] ) : '' ),
			'cs_titles_text_color'                => array( isset( $_POST[ 'cs_titles_text_color' ] ) ? sanitize_text_field( $_POST[ 'cs_titles_text_color' ] ) : '' ),
			'cs_subtitles_text_color'             => array( isset( $_POST[ 'cs_subtitles_text_color' ] ) ? sanitize_text_field( $_POST[ 'cs_subtitles_text_color' ] ) : '' ),
			'cs_accept_all_buttons_bg'            => array( isset( $_POST[ 'cs_accept_all_buttons_bg' ] ) ? sanitize_text_field( $_POST[ 'cs_accept_all_buttons_bg' ] ) : '' ),
			'cs_accept_all_buttons_text_color'    => array( isset( $_POST[ 'cs_accept_all_buttons_text_color' ] ) ? sanitize_text_field( $_POST[ 'cs_accept_all_buttons_text_color' ] ) : '' ),
			'cs_custom_button_buttons_bg'         => array( isset( $_POST[ 'cs_custom_button_buttons_bg' ] ) ? sanitize_text_field( $_POST[ 'cs_custom_button_buttons_bg' ] ) : '' ),
			'cs_custom_button_buttons_text_color' => array( isset( $_POST[ 'cs_custom_button_buttons_text_color' ] ) ? sanitize_text_field( $_POST[ 'cs_custom_button_buttons_text_color' ] ) : '' ),
			'cs_deny_all_buttons_bg'              => array( isset( $_POST[ 'cs_deny_all_buttons_bg' ] ) ? sanitize_text_field( $_POST[ 'cs_deny_all_buttons_bg' ] ) : '' ),
			'cs_deny_all_buttons_text_color'      => array( isset( $_POST[ 'cs_deny_all_buttons_text_color' ] ) ? sanitize_text_field( $_POST[ 'cs_deny_all_buttons_text_color' ] ) : '' ),
			'cs_options_buttons_text_color'       => array( isset( $_POST[ 'cs_options_buttons_text_color' ] ) ? sanitize_text_field( $_POST[ 'cs_options_buttons_text_color' ] ) : '' ),
			'cs_options_buttons_bg'               => array( isset( $_POST[ 'cs_options_buttons_bg' ] ) ? sanitize_text_field( $_POST[ 'cs_options_buttons_bg' ] ) : '' ),
			'cs_confirm_buttons_bg'               => array( isset( $_POST[ 'cs_confirm_buttons_bg' ] ) ? sanitize_text_field( $_POST[ 'cs_confirm_buttons_bg' ] ) : '' ),
			'cs_confirm_buttons_text_color'       => array( isset( $_POST[ 'cs_confirm_buttons_text_color' ] ) ? sanitize_text_field( $_POST[ 'cs_confirm_buttons_text_color' ] ) : '' ),
			'cs_sticky_bg'                        => array( isset( $_POST[ 'cs_sticky_bg' ] ) ? sanitize_text_field( $_POST[ 'cs_sticky_bg' ] ) : '' ),
			'cs_sticky_link_color'                => array( isset( $_POST[ 'cs_sticky_link_color' ] ) ? sanitize_text_field( $_POST[ 'cs_sticky_link_color' ] ) : '' ),
			'cs_logo'                             => array( isset( $_POST[ 'cs_logo' ] ) ? sanitize_text_field( $_POST[ 'cs_logo' ] ) : '' ),
			'cs_position_vertical_list'           => array( isset( $_POST[ 'cs_position_vertical_list' ] ) ? sanitize_text_field( $_POST[ 'cs_position_vertical_list' ] ) : '' ),
			'cs_position_horizontal_list'         => array( isset( $_POST[ 'cs_position_horizontal_list' ] ) ? sanitize_text_field( $_POST[ 'cs_position_horizontal_list' ] ) : '' ),
			'cs_logo_size'                        => array( isset( $_POST[ 'cs_logo_size' ] ) ? sanitize_text_field( $_POST[ 'cs_logo_size' ] ) : '' ),
			'cs_cat_color'                        => array( isset( $_POST[ 'cs_cat_color' ] ) ? sanitize_text_field( $_POST[ 'cs_cat_color' ] ) : '' ),
			'cs_active_toggle_color'              => array( isset( $_POST[ 'cs_active_toggle_color' ] ) ? sanitize_text_field( $_POST[ 'cs_active_toggle_color' ] ) : '' ),
			'cs_active_toggle_text_color'         => array( isset( $_POST[ 'cs_active_toggle_text_color' ] ) ? sanitize_text_field( $_POST[ 'cs_active_toggle_text_color' ] ) : '' ),
			'cs_shortcodes_text_color'            => array( isset( $_POST[ 'cs_shortcodes_text_color' ] ) ? sanitize_text_field( $_POST[ 'cs_shortcodes_text_color' ] ) : '' ),
			'cs_tab_buttons_bg'                   => array( isset( $_POST[ 'cs_tab_buttons_bg' ] ) ? sanitize_text_field( $_POST[ 'cs_tab_buttons_bg' ] ) : '' ),
			'cs_tab_buttons_text_color'           => array( isset( $_POST[ 'cs_tab_buttons_text_color' ] ) ? sanitize_text_field( $_POST[ 'cs_tab_buttons_text_color' ] ) : '' ),
		);

		$cs_preview_popup_admin = true;
		echo '<div id="cs_preview_popup">';

		if ( $cs_bars_type == 'popup_large_single' ) {
			if ( $cs_type == 'just_inform' ) {
				esc_html_e( 'There are no configured rules for single step design', 'consent-magic' );
			} else {
				include_once CMPRO_PLUGIN_VIEWS_PATH . 'templates/single/cs_single_design.php';
			}
		} else {
			if ( $cs_bars_type == 'bar_small' || $cs_bars_type == 'bar_large' ) {
				include CMPRO_PLUGIN_VIEWS_PATH . 'templates/multi/cs-bar.php';
			} else if ( $cs_bars_type == 'popup_small' || $cs_bars_type == 'popup_large' ) {
				include CMPRO_PLUGIN_VIEWS_PATH . 'templates/multi/cs_popup.php';
			}
		}

		echo '</div>';
		add_filter( 'safe_style_css', array(
			$this,
			'add_style_option'
		) );
		$this->add_preview_styles();
		remove_filter( 'safe_style_css', array(
			$this,
			'add_style_option'
		) );

		$html = ob_get_contents();
		ob_end_clean();
		wp_send_json_success( array( 'html' => $html ) );
		wp_die(); // ajax call must die to avoid trailing 0 in your response

	}

	function renew_consent_run() {
		$consent_version = ConsentMagic()->cs_get_consent_version();
		if ( !empty( $consent_version ) ) {
			$consent_version = $consent_version + 1;
			ConsentMagic()->updateOptions( array( 'cs_consent_version' => $consent_version ) );
		}
	}

	public function register_custom_post_type() {
		$labels = array(
			'name'               => esc_html__( 'ConsentMagic', 'consent-magic' ),
			'all_items'          => esc_html__( 'Rules List', 'consent-magic' ),
			'singular_name'      => esc_html__( 'Rule', 'consent-magic' ),
			'add_new'            => esc_html__( 'Add New', 'consent-magic' ),
			'add_new_item'       => esc_html__( 'Add New Rule', 'consent-magic' ),
			'edit_item'          => esc_html__( 'Edit Rule', 'consent-magic' ),
			'new_item'           => esc_html__( 'New Rule', 'consent-magic' ),
			'view_item'          => esc_html__( 'View Rule', 'consent-magic' ),
			'search_items'       => esc_html__( 'Search Rules', 'consent-magic' ),
			'not_found'          => esc_html__( 'Nothing found', 'consent-magic' ),
			'not_found_in_trash' => esc_html__( 'Nothing found in Trash', 'consent-magic' ),
			'parent_item_colon'  => ''
		);
		$args   = array(
			'labels'              => $labels,
			'public'              => false,
			'publicly_queryable'  => false,
			'exclude_from_search' => true,
			'show_ui'             => false,
			'query_var'           => false,
			'rewrite'             => true,
			'show_in_menu'        => false,
			'menu_icon'           => '',
			'capabilities'        => array(
				'publish_posts'       => 'manage_cs',
				'edit_posts'          => 'manage_cs',
				'edit_others_posts'   => 'manage_cs',
				'delete_posts'        => 'manage_cs',
				'delete_others_posts' => 'manage_cs',
				'read_private_posts'  => 'manage_cs',
				'edit_post'           => 'manage_cs',
				'delete_post'         => 'manage_cs',
				'read_post'           => 'manage_cs',
			),
			'taxonomies'          => array( 'cs-category' ),
			'hierarchical'        => true,
			'menu_position'       => null,
			'supports'            => array(
				'title',
				'editor',
				'page-attributes'
			)
		);
		register_post_type( CMPRO_POST_TYPE, $args );

		$labels = array(
			'name' => esc_html__( 'CS Templates', 'consent-magic' ),
		);
		$args   = array(
			'labels'              => $labels,
			'public'              => false,
			'publicly_queryable'  => false,
			'exclude_from_search' => true,
			'show_ui'             => false,
			'query_var'           => false,
			'rewrite'             => true,
			'show_in_menu'        => false,
			'menu_icon'           => '',
			'capabilities'        => array(
				'publish_posts'       => 'manage_cs',
				'edit_posts'          => 'manage_cs',
				'edit_others_posts'   => 'manage_cs',
				'delete_posts'        => 'manage_cs',
				'delete_others_posts' => 'manage_cs',
				'read_private_posts'  => 'manage_cs',
				'edit_post'           => 'manage_cs',
				'delete_post'         => 'manage_cs',
				'read_post'           => 'manage_cs',
			),
			'menu_position'       => null,
			'supports'            => array(
				'title',
				'editor',
				'page-attributes'
			)
		);
		register_post_type( CMPRO_TEMPLATE_POST_TYPE, $args );

		$labels = array(
			'name'               => esc_html__( 'CS Scripts', 'consent-magic' ),
			'all_items'          => esc_html__( 'Scripts List', 'consent-magic' ),
			'singular_name'      => esc_html__( 'Script', 'consent-magic' ),
			'add_new'            => esc_html__( 'Add New', 'consent-magic' ),
			'add_new_item'       => esc_html__( 'Add New Script', 'consent-magic' ),
			'edit_item'          => esc_html__( 'Edit Script', 'consent-magic' ),
			'new_item'           => esc_html__( 'New Script', 'consent-magic' ),
			'view_item'          => esc_html__( 'View Script', 'consent-magic' ),
			'search_items'       => esc_html__( 'Search Scripts', 'consent-magic' ),
			'not_found'          => esc_html__( 'Nothing found', 'consent-magic' ),
			'not_found_in_trash' => esc_html__( 'Nothing found in Trash', 'consent-magic' ),
			'parent_item_colon'  => ''
		);
		$args   = array(
			'labels'              => $labels,
			'public'              => false,
			'publicly_queryable'  => false,
			'exclude_from_search' => true,
			'show_ui'             => false,
			'query_var'           => false,
			'rewrite'             => true,
			'show_in_menu'        => false,
			'menu_icon'           => '',
			'capabilities'        => array(
				'publish_posts'       => 'manage_cs',
				'edit_posts'          => 'manage_cs',
				'edit_others_posts'   => 'manage_cs',
				'delete_posts'        => 'manage_cs',
				'delete_others_posts' => 'manage_cs',
				'read_private_posts'  => 'manage_cs',
				'edit_post'           => 'manage_cs',
				'delete_post'         => 'manage_cs',
				'read_post'           => 'manage_cs',
			),
			'taxonomies'          => array( 'cs-cookies-category' ),
			'hierarchical'        => true,
			'menu_position'       => null,
			'supports'            => array(
				'title',
				'editor',
				'page-attributes'
			)
		);
		register_post_type( CMPRO_POST_TYPE_SCRIPTS, $args );

		$labels = array(
			'name'               => esc_html__( 'CS Cookies', 'consent-magic' ),
			'all_items'          => esc_html__( 'Cookies List', 'consent-magic' ),
			'singular_name'      => esc_html__( 'Cookie', 'consent-magic' ),
			'add_new'            => esc_html__( 'Add New', 'consent-magic' ),
			'add_new_item'       => esc_html__( 'Add New Cookie', 'consent-magic' ),
			'edit_item'          => esc_html__( 'Edit Cookie', 'consent-magic' ),
			'new_item'           => esc_html__( 'New Cookie', 'consent-magic' ),
			'view_item'          => esc_html__( 'View Cookie', 'consent-magic' ),
			'search_items'       => esc_html__( 'Search Cookies', 'consent-magic' ),
			'not_found'          => esc_html__( 'Nothing found', 'consent-magic' ),
			'not_found_in_trash' => esc_html__( 'Nothing found in Trash', 'consent-magic' ),
			'parent_item_colon'  => ''
		);
		$args   = array(
			'labels'              => $labels,
			'public'              => false,
			'publicly_queryable'  => false,
			'exclude_from_search' => true,
			'show_ui'             => false,
			'query_var'           => false,
			'rewrite'             => true,
			'show_in_menu'        => false,
			'menu_icon'           => '',
			'capabilities'        => array(
				'publish_posts'       => 'manage_cs',
				'edit_posts'          => 'manage_cs',
				'edit_others_posts'   => 'manage_cs',
				'delete_posts'        => 'manage_cs',
				'delete_others_posts' => 'manage_cs',
				'read_private_posts'  => 'manage_cs',
				'edit_post'           => 'manage_cs',
				'delete_post'         => 'manage_cs',
				'read_post'           => 'manage_cs',
			),
			'taxonomies'          => array( 'cs-cookies-category' ),
			'hierarchical'        => true,
			'menu_position'       => null,
			'supports'            => array(
				'title',
				'editor',
				'page-attributes'
			)
		);
		register_post_type( CMPRO_POST_TYPE_COOKIES, $args );
	}

	/**
	 * Registering cookie category
	 */
	public function create_taxonomy() {
		register_taxonomy( 'cs-category', CMPRO_POST_TYPE, array(
			'label'              => esc_html__( 'Rules Category', 'consent-magic' ),
			'rewrite'            => array( 'slug' => 'cs-category' ),
			'hierarchical'       => true,
			'show_in_menu'       => true,
			'show_ui'            => true,
			'query_var'          => true,
			'capabilities'       => array(
				'publish_posts'       => 'manage_cs',
				'edit_posts'          => 'manage_cs',
				'edit_others_posts'   => 'manage_cs',
				'delete_posts'        => 'manage_cs',
				'delete_others_posts' => 'manage_cs',
				'read_private_posts'  => 'manage_cs',
				'edit_post'           => 'manage_cs',
				'delete_post'         => 'manage_cs',
				'read_post'           => 'manage_cs',
			),
			'publicly_queryable' => false
		) );

		register_taxonomy( 'cs-cookies-category', array(
			CMPRO_POST_TYPE_COOKIES,
			CMPRO_POST_TYPE_SCRIPTS
		), array(
			'label'              => esc_html__( 'Scripts Category', 'consent-magic' ),
			'rewrite'            => array( 'slug' => 'cs-cookies-category' ),
			'hierarchical'       => true,
			'show_in_menu'       => true,
			'show_ui'            => true,
			'query_var'          => true,
			'capabilities'       => array(
				'publish_posts'       => 'manage_cs',
				'edit_posts'          => 'manage_cs',
				'edit_others_posts'   => 'manage_cs',
				'delete_posts'        => 'manage_cs',
				'delete_others_posts' => 'manage_cs',
				'read_private_posts'  => 'manage_cs',
				'edit_post'           => 'manage_cs',
				'delete_post'         => 'manage_cs',
				'read_post'           => 'manage_cs',
			),
			'publicly_queryable' => false
		) );

		global $sitepress;
		$wpml_default_lang = 'en';
		$wpml_current_lang = 'en';
		if ( function_exists( 'icl_object_id' ) && $sitepress ) {
			$wpml_default_lang = $sitepress->get_default_language();
			$wpml_current_lang = apply_filters( 'wpml_current_language', NULL );;
		}
		if ( $wpml_default_lang == $wpml_current_lang ) {
			$term = get_term_by( 'slug', 'rules', 'cs-category' );
			if ( @ !$term->term_id ) {
				wp_insert_term( 'Rules', 'cs-category', array(
					'description' => '',
					'slug'        => 'rules'
				) );
			}

			$term = get_term_by( 'slug', 'necessary', 'cs-cookies-category' );
			if ( !$term ) {
				$cid = wp_insert_term( 'Necessary', 'cs-cookies-category', array(
					'slug' => 'necessary'
				) );
				if ( !is_wp_error( $cid ) ) {
					$description = array(
						'name'  => 'Necessary',
						'descr' => 'These cookies and scripts are necessary for the website to function and cannot be switched off. They are usually only set in response to actions made by you which amount to a request for services, suchas setting your privacy preferences, logging in or filling in forms. You can set your browser to block oralert you about these cookies, but some parts of the site will not then work. These cookies do not store any personally identifiable information.',
					);
					// Get term_id, set default as 0 if not set
					$cat_id = isset( $cid[ 'term_id' ] ) ? $cid[ 'term_id' ] : 0;
					add_term_meta( $cat_id, 'cs_primary_term', 'primary', true );
					add_term_meta( $cat_id, 'cs_necessary_term', 'primary', true );
					add_term_meta( $cat_id, 'cs_ignore_this_category', '0', true );
					ConsentMagic()->updateLangOptions( 'necessary', $description, CMPRO_DEFAULT_LANGUAGE, 'term' );
					ConsentMagic()->updateOptions( array( 'necessary_cat_id' => $cat_id ) );
				} else {
					// Trouble in Paradise:
					echo esc_html( $cid->get_error_message() );
				}
			}

			$term = get_term_by( 'slug', 'analytics', 'cs-cookies-category' );
			if ( !$term ) {
				$cid = wp_insert_term( 'Analytics', 'cs-cookies-category', array(
					'slug' => 'analytics'
				) );
				if ( !is_wp_error( $cid ) ) {
					require_once CMPRO_PLUGIN_PATH . 'includes/modules/iab_integration/CS_IAB_Integration.php';
					$iab_categories = CS_IAB_Integration()->get_default_script_categories();
					$description    = array(
						'name'  => 'Analytics',
						'descr' => 'These cookies and scripts allow us to count visits and traffic sources, so we can measure and improve the performance of our site. They help us know which pages are the most and least popular and see how visitors move around the site. All information these cookies collect is aggregated and therefore anonymous. If you do not allow these cookies and scripts, we will not know when you have visited our site.',
					);
					// Get term_id, set default as 0 if not set
					$cat_id = isset( $cid[ 'term_id' ] ) ? $cid[ 'term_id' ] : 0;
					add_term_meta( $cat_id, 'cs_primary_term', 'primary', true );
					add_term_meta( $cat_id, 'cs_necessary_term', 'custom', true );
					add_term_meta( $cat_id, 'cs_ignore_this_category', '0', true );
					ConsentMagic()->updateLangOptions( 'analytics', $description, CMPRO_DEFAULT_LANGUAGE, 'term' );
					add_term_meta( $cat_id, '_cs_iab_cat', $iab_categories[ 'analytics' ], true );
					ConsentMagic()->updateOptions( array( 'analytics_cat_id' => $cat_id ) );
				} else {
					// Trouble in Paradise:
					echo esc_html( $cid->get_error_message() );
				}
			}

			$term = get_term_by( 'slug', 'marketing', 'cs-cookies-category' );
			if ( !$term ) {
				$cid = wp_insert_term( 'Marketing', 'cs-cookies-category', array(
					'slug' => 'marketing'
				) );
				if ( !is_wp_error( $cid ) ) {
					require_once CMPRO_PLUGIN_PATH . 'includes/modules/iab_integration/CS_IAB_Integration.php';
					$iab_categories = CS_IAB_Integration()->get_default_script_categories();
					$description    = array(
						'name'  => 'Marketing',
						'descr' => 'These cookies and scripts  may be set through our site by our advertising partners. They may be used by those companies to build a profile of your interests and show you relevant adverts on other sites. They do not store directly personal information, but are based on uniquely identifying your browser and internet device. If you do not allow these cookies and scripts, you will experience less targeted advertising.',
					);
					// Get term_id, set default as 0 if not set
					$cat_id = isset( $cid[ 'term_id' ] ) ? $cid[ 'term_id' ] : 0;
					add_term_meta( $cat_id, 'cs_primary_term', 'primary', true );
					add_term_meta( $cat_id, 'cs_necessary_term', 'custom', true );
					add_term_meta( $cat_id, 'cs_ignore_this_category', '0', true );
					ConsentMagic()->updateLangOptions( 'marketing', $description, CMPRO_DEFAULT_LANGUAGE, 'term' );
					add_term_meta( $cat_id, '_cs_iab_cat', $iab_categories[ 'marketing' ], true );
					ConsentMagic()->updateOptions( array( 'marketing_cat_id' => $cat_id ) );
				} else {
					// Trouble in Paradise:
					echo esc_html( $cid->get_error_message() );
				}
			}

			$term = get_term_by( 'slug', 'googlefonts', 'cs-cookies-category' );
			if ( !$term ) {
				$cid = wp_insert_term( 'Google Fonts', 'cs-cookies-category', array(
					'slug' => 'googlefonts'
				) );
				if ( !is_wp_error( $cid ) ) {
					require_once CMPRO_PLUGIN_PATH . 'includes/modules/iab_integration/CS_IAB_Integration.php';
					$iab_categories = CS_IAB_Integration()->get_default_script_categories();
					$description    = array(
						'name'  => 'Google Fonts',
						'descr' => 'Google Fonts is a font embedding service library. Google Fonts are stored on Google\'s CDN. The Google Fonts API is designed to limit the collection, storage, and use of end-user data to only what is needed to serve fonts efficiently. Use of Google Fonts API is unauthenticated. No cookies are sent by website visitors to the Google Fonts API. Requests to the Google Fonts API are made to resource-specific domains, such as fonts.googleapis.com or fonts.gstatic.com. This means your font requests are separate from and don\'t contain any credentials you send to google.com while using other Google services that are authenticated, such as Gmail.',
					);
					// Get term_id, set default as 0 if not set
					$cat_id = isset( $cid[ 'term_id' ] ) ? $cid[ 'term_id' ] : 0;
					add_term_meta( $cat_id, 'cs_primary_term', 'primary', true );
					add_term_meta( $cat_id, 'cs_necessary_term', 'custom', true );
					add_term_meta( $cat_id, 'cs_ignore_this_category', '1', true );
					ConsentMagic()->updateLangOptions( 'googlefonts', $description, CMPRO_DEFAULT_LANGUAGE, 'term' );
					add_term_meta( $cat_id, '_cs_iab_cat', $iab_categories[ 'googlefonts' ], true );
					ConsentMagic()->updateOptions( array( 'googlefonts_cat_id' => $cat_id ) );
				} else {
					// Trouble in Paradise:
					echo esc_html( $cid->get_error_message() );
				}
			}

			$term = get_term_by( 'slug', 'unassigned', 'cs-cookies-category' );
			if ( !$term ) {
				$cid = wp_insert_term( 'Unassigned', 'cs-cookies-category', array(
					'slug' => 'unassigned'
				) );
				if ( !is_wp_error( $cid ) ) {
					require_once CMPRO_PLUGIN_PATH . 'includes/modules/iab_integration/CS_IAB_Integration.php';
					$iab_categories = CS_IAB_Integration()->get_default_script_categories();
					$description    = array(
						'name'  => 'Unassigned',
						'descr' => 'These cookies and scripts  may be set through our site by our advertising partners. They may be used by those companies to build a profile of your interests and show you relevant adverts on other sites. They do not store directly personal information, but are based on uniquely identifying your browser and internet device. If you do not allow these cookies and scripts, you will experience less targeted advertising.',
					);
					// Get term_id, set default as 0 if not set
					$cat_id = isset( $cid[ 'term_id' ] ) ? $cid[ 'term_id' ] : 0;
					add_term_meta( $cat_id, 'cs_primary_term', 'primary', true );
					add_term_meta( $cat_id, 'cs_necessary_term', 'custom', true );
					add_term_meta( $cat_id, 'cs_ignore_this_category', '1', true );
					ConsentMagic()->updateLangOptions( 'unassigned', $description, CMPRO_DEFAULT_LANGUAGE, 'term' );
					add_term_meta( $cat_id, '_cs_iab_cat', $iab_categories[ 'default' ], true );
					ConsentMagic()->updateOptions( array( 'unassigned_cat_id' => $cat_id ) );
				} else {
					// Trouble in Paradise:
					echo esc_html( $cid->get_error_message() );
				}
			}

			$term = get_term_by( 'slug', 'embedded_video', 'cs-cookies-category' );
			if ( !$term ) {
				$cid = wp_insert_term( 'Embedded Videos', 'cs-cookies-category', array(
					'slug' => 'embedded_video'
				) );
				if ( !is_wp_error( $cid ) ) {
					require_once CMPRO_PLUGIN_PATH . 'includes/modules/iab_integration/CS_IAB_Integration.php';
					$iab_categories = CS_IAB_Integration()->get_default_script_categories();
					$description    = array(
						'name'  => 'Embedded Videos',
						'descr' => 'These cookies and scripts may be set through our site by external video hosting services likeYouTube or Vimeo. They may be used to deliver video content on our website. It’s possible for the video provider to build a profile of your interests and show you relevant adverts on this or other websites. They do not store directly personal information, but are based on uniquely identifying your browser and internet device. If you do not allow these cookies or scripts it is possible that embedded video will not function as expected.',
					);
					// Get term_id, set default as 0 if not set
					$cat_id = isset( $cid[ 'term_id' ] ) ? $cid[ 'term_id' ] : 0;
					add_term_meta( $cat_id, 'cs_primary_term', 'primary', true );
					add_term_meta( $cat_id, 'cs_necessary_term', 'custom', true );
					add_term_meta( $cat_id, 'cs_ignore_this_category', '0', true );
					ConsentMagic()->updateLangOptions( 'embedded_video', $description, CMPRO_DEFAULT_LANGUAGE, 'term' );
					add_term_meta( $cat_id, '_cs_iab_cat', $iab_categories[ 'embedded_video' ], true );
					ConsentMagic()->updateOptions( array( 'embedded_video_cat_id' => $cat_id ) );
				} else {
					// Trouble in Paradise:
					echo esc_html( $cid->get_error_message() );
				}
			}
		}

		//check for new options
		if ( ConsentMagic()->getOption( 'cs_check_script_cat_settings' ) != CMPRO_LATEST_VERSION_NUMBER ) {
			$args         = array(
				'taxonomy'   => 'cs-cookies-category',
				'hide_empty' => false,
			);
			$script_types = get_terms( $args );
			global $wpdb;

			if ( !empty( $script_types ) ) {
				$translation_table = $wpdb->prefix . 'cs_translations';
				foreach ( $script_types as $script_type ) {
					// Check if the term has a translation
					$has_translation = $wpdb->get_var( $wpdb->prepare( "
                                SELECT COUNT(*) 
                                    FROM $translation_table 
                                    WHERE `option` = %s 
                                      AND category = 1", $script_type->slug ) );
					if ( !$has_translation ) {
						// Insert default translation for the term
						$default = ConsentMagic()->getDefaultTranslation( $script_type->slug, CMPRO_DEFAULT_LANGUAGE, true );
						$name    = $value = '';
						if ( is_array( $default ) ) {
							$name  = $default[ 'name' ] ?? '';
							$value = $default[ 'descr' ] ?? '';
						}

						$description = array(
							'name'  => $name,
							'descr' => $value,
						);

						ConsentMagic()->updateLangOptions( $script_type->slug, $description, CMPRO_DEFAULT_LANGUAGE, 'term' );
					}
				}
			}

			require_once CMPRO_PLUGIN_PATH . 'includes/modules/iab_integration/CS_IAB_Integration.php';
			require_once CMPRO_PLUGIN_PATH . 'includes/modules/wp_consent_api/CS_WP_Consent_Api.php';
			$iab_default_cats  = CS_IAB_Integration()->get_default_script_categories();
			$wp_api_categories = CS_WP_Consent_Api()->getWPConsentAPIDefaultCategories();
			$ignore            = array();
			$term              = get_term_by( 'slug', 'unassigned', 'cs-cookies-category' );
			$ignore[]          = $term->term_id;
			$scripts           = get_cookies_terms_objects( null, false, $ignore );
			if ( !empty( $scripts ) ) {
				foreach ( $scripts as $script ) {

					//Update iab_cat in scripts category
					$iab_cat = get_term_meta( $script->term_id, '_cs_iab_cat', true );
					if ( $iab_cat === '' && $script->slug !== 'necessary' ) {
						$val = $iab_default_cats[ $script->slug ] ?? $iab_default_cats[ 'default' ];
						update_term_meta( $script->term_id, '_cs_iab_cat', $val );
					}

					//Update WP Consent API categories
					$wp_api_cat = get_term_meta( $script->term_id, '_cs_wp_consent_api_cat', true );
					if ( $wp_api_cat === '' ) {
						$val = $wp_api_categories[ $script->slug ] ?? $wp_api_categories[ 'default' ];
						update_term_meta( $script->term_id, '_cs_wp_consent_api_cat', $val );
					}
				}
			}

			ConsentMagic()->updateOptions( array( 'cs_check_script_cat_settings' => CMPRO_LATEST_VERSION_NUMBER ) );
		}
	}

	/**
	 * Insert Dummy post
	 */
	public function cs_insert_cookie_lists() {

		$not_enough_rule = false;
		foreach ( static::$default_rules as $default_rule ) {
			if ( !$this->cs_post_exists_by_slug( $default_rule, CMPRO_POST_TYPE ) ) {
				$not_enough_rule = true;
			}
		}

		if ( $not_enough_rule ) {

			$cs_light_theme_id = get_page_id_by_path( 'cs_light_theme', '', CMPRO_TEMPLATE_POST_TYPE );

			$default_cookies = array(
				array(
					'post_title'                            => 'IAB Rule',
					'slug'                                  => 'cs_iab_rule',
					'post_content'                          => '',
					'post_category'                         => 'rules',
					'cs_type'                               => 'iab',
					//ask_before_tracking, just_inform, inform_and_opiout, iab
					'cs_sticky'                             => 1,
					'cs_smart_sticky'                       => 0,
					'cs_smart_sticky_mobile'                => 0,
					'cs_custom_button'                      => 0,
					// 1 - sticky, 0 - not-sticky
					'cs_mobile_side_sticky'                 => 0,
					// 1 - true, 0 - false
					'cs_hide_close_btn'                     => 1,
					// 1 - true, 0 - false
					'cs_deny_consent_for_close'             => 0,
					// 1 - true, 0 - false
					'cs_deny_all_btn'                       => 1,
					// 1 - show, 0 - hide
					'cs_bars_position'                      => 'bottom',
					// bottom, top
					'cs_bars_type'                          => 'popup_large',
					// bar_small, bar_large, popup_small, popup_large
					'cs_privacy_link'                       => 1,
					// 1 - on, 0 - off
					'cs_close_on_scroll'                    => 0,
					// 1 - on, 0 - off
					'cs_theme'                              => $cs_light_theme_id,
					'cs_design_type'                        => 'single',
					'cs_top_push'                           => 0,
					// 1 - on, 0 - off
					'cs_predefined_rule'                    => 1,
					// 1 - on, 0 - off
					'cs_target'                             => 'GDPR,AT,BE,BG,CH,CY,CZ,DK,EE,FI,FR,DE,GR,HU,HR,IE,IT,IS,LI,LT,LU,LV,MT,NL,NO,PL,PT,RO,SE,SI,SK,ES,GB',
					'cs_us_states_target'                   => '',
					'cs_use_meta_ldu'                       => 0,
					'cs_google_consent_mode'                => 1,
					'cs_bing_consent_mode'                  => 1,
					'cs_reddit_ldu'                         => 0,
					'cs_order'                              => 3,
					'cs_enable_rule'                        => 0,
					'cs_no_ip_rule'                         => 0,
					'cs_track_analytics'                    => 0,
					'cs_custom_text'                        => 0,
					'cs_block_content'                      => 1,
					'cs_rule_status'                        => 'publish',
					'cs_refresh_after_consent'              => 1,
					'excluded_from_consent_storing'         => 0,
					'cs_showing_rule_until_express_consent' => 0,
					'cs_native_scripts'                     => 1,
				),
				array(
					'post_title'                            => 'GDPR Rule',
					'slug'                                  => 'cs_gdpr_rule',
					'post_content'                          => '',
					'post_category'                         => 'rules',
					'cs_type'                               => 'ask_before_tracking',
					//ask_before_tracking, just_inform, inform_and_opiout, iab
					'cs_sticky'                             => 1,
					'cs_smart_sticky'                       => 0,
					'cs_smart_sticky_mobile'                => 0,
					'cs_custom_button'                      => 0,
					// 1 - sticky, 0 - not-sticky
					'cs_mobile_side_sticky'                 => 0,
					// 1 - true, 0 - false
					'cs_hide_close_btn'                     => 1,
					// 1 - true, 0 - false
					'cs_deny_consent_for_close'             => 0,
					// 1 - true, 0 - false
					'cs_deny_all_btn'                       => 1,
					// 1 - show, 0 - hide
					'cs_bars_position'                      => 'bottom',
					// bottom, top
					'cs_bars_type'                          => 'bar_large',
					// bar_small, bar_large, popup_small, popup_large
					'cs_privacy_link'                       => 1,
					// 1 - on, 0 - off
					'cs_close_on_scroll'                    => 0,
					// 1 - on, 0 - off
					'cs_theme'                              => $cs_light_theme_id,
					'cs_design_type'                        => 'multi',
					'cs_top_push'                           => 0,
					// 1 - on, 0 - off
					'cs_predefined_rule'                    => 1,
					// 1 - on, 0 - off
					'cs_target'                             => 'GDPR,AT,BE,BG,CH,CY,CZ,DK,EE,FI,FR,DE,GR,HU,HR,IE,IT,IS,LI,LT,LU,LV,MT,NL,NO,PL,PT,RO,SE,SI,SK,ES,GB',
					'cs_us_states_target'                   => '',
					'cs_use_meta_ldu'                       => 0,
					'cs_google_consent_mode'                => 1,
					'cs_bing_consent_mode'                  => 1,
					'cs_reddit_ldu'                         => 0,
					'cs_order'                              => 0,
					'cs_enable_rule'                        => 1,
					'cs_no_ip_rule'                         => 1,
					'cs_track_analytics'                    => 0,
					'cs_custom_text'                        => 0,
					'cs_block_content'                      => 1,
					'cs_rule_status'                        => 'publish',
					'cs_refresh_after_consent'              => 1,
					'excluded_from_consent_storing'         => 0,
					'cs_showing_rule_until_express_consent' => 0,
					'cs_native_scripts'                     => 0,
				),
				array(
					'post_title'                            => 'Limited Data Use (LDU) Rule',
					'slug'                                  => 'cs_ldu_rule',
					'post_content'                          => '',
					'post_category'                         => 'rules',
					'cs_type'                               => 'inform_and_opiout',
					//ask_before_tracking, just_inform, inform_and_opiout, iab
					'cs_sticky'                             => 0,
					'cs_smart_sticky'                       => 0,
					'cs_smart_sticky_mobile'                => 0,
					'cs_custom_button'                      => 0,
					// 1 - sticky, 0 - not-sticky
					'cs_mobile_side_sticky'                 => 0,
					// 1 - true, 0 - false
					'cs_hide_close_btn'                     => 0,
					// 1 - true, 0 - false
					'cs_deny_consent_for_close'             => 0,
					// 1 - true, 0 - false
					'cs_deny_all_btn'                       => 0,
					// 1 - show, 0 - hide
					'cs_bars_position'                      => 'bottom',
					// bottom, top
					'cs_bars_type'                          => 'bar_small',
					// bar_small, bar_large, popup_small, popup_large
					'cs_privacy_link'                       => 1,
					// 1 - on, 0 - off
					'cs_close_on_scroll'                    => 0,
					// 1 - on, 0 - off
					'cs_theme'                              => $cs_light_theme_id,
					'cs_design_type'                        => 'multi',
					'cs_top_push'                           => 0,
					// 1 - on, 0 - off
					'cs_predefined_rule'                    => 1,
					// 1 - on, 0 - off
					'cs_target'                             => 'US',
					'cs_us_states_target'                   => 'US_CA,US_CO,US_CT,US_DE,US_FL,US_MT,US_NE,US_NH,US_NJ,US_OR,US_ТХ',
					'cs_use_meta_ldu'                       => 1,
					'cs_google_consent_mode'                => 1,
					'cs_bing_consent_mode'                  => 1,
					'cs_reddit_ldu'                         => 1,
					'cs_order'                              => 1,
					'cs_enable_rule'                        => 1,
					'cs_no_ip_rule'                         => 0,
					'cs_track_analytics'                    => 0,
					'cs_custom_text'                        => 0,
					'cs_block_content'                      => 0,
					'cs_rule_status'                        => 'publish',
					'cs_refresh_after_consent'              => 0,
					'excluded_from_consent_storing'         => 0,
					'cs_showing_rule_until_express_consent' => 0,
					'cs_native_scripts'                     => 0,
				),
				array(
					'post_title'                            => 'Rest of the world Rule',
					'slug'                                  => 'cs_rest_of_world_rule',
					'post_content'                          => '',
					'post_category'                         => 'rules',
					'cs_type'                               => 'just_inform',
					//ask_before_tracking, just_inform, inform_and_opiout, iab
					'cs_sticky'                             => 0,
					'cs_smart_sticky'                       => 0,
					'cs_smart_sticky_mobile'                => 0,
					'cs_custom_button'                      => 0,
					// 1 - sticky, 0 - not-sticky
					'cs_mobile_side_sticky'                 => 0,
					// 1 - true, 0 - false
					'cs_hide_close_btn'                     => 0,
					// 1 - true, 0 - false
					'cs_deny_consent_for_close'             => 0,
					// 1 - true, 0 - false
					'cs_deny_all_btn'                       => 0,
					// 1 - show, 0 - hide
					'cs_bars_position'                      => 'bottom',
					// bottom, top
					'cs_bars_type'                          => 'bar_small',
					// bar_small, bar_large, popup_small, popup_large
					'cs_privacy_link'                       => 1,
					// 1 - on, 0 - off
					'cs_close_on_scroll'                    => 1,
					// 1 - on, 0 - off
					'cs_theme'                              => $cs_light_theme_id,
					'cs_design_type'                        => 'multi',
					'cs_top_push'                           => 0,
					// 1 - on, 0 - off
					'cs_predefined_rule'                    => 1,
					// 1 - on, 0 - off
					'cs_target'                             => 'NONGDPR,AD,AF,AG,AI,AL,DZ,AM,AO,AQ,AR,AS,AU,AW,AX,AZ,BA,BB,BD,BF,BH,BI,BJ,BL,BM,BN,BO,BQ,BR,BS,BT,BV,BW,BY,BZ,CA,CC,CD,CF,CG,CI,CK,CL,CM,CN,CO,CR,CU,CV,CW,CX,KH,KM,TD,KY,DJ,DM,DO,EZ,SV,SZ,EG,ER,GQ,ET,FG,FK,FO,GF,PF,TF,GA,GD,GE,GG,GH,GI,GL,GM,GN,GP,GT,GU,GW,GY,HK,VA,HM,HN,HT,ID,IL,IM,IN,IO,IQ,IR,JE,JM,JO,JP,KE,KG,KI,KN,KP,KR,KW,KZ,LA,LB,LR,LS,LY,MA,MC,MD,ME,MG,MH,ML,MM,MN,MO,MQ,MR,MS,MU,MV,MW,MX,MY,FM,YT,MZ,NM,NC,NE,NF,NG,NI,NO,NP,NR,NU,NZ,MK,MP,OM,PA,PE,PG,PH,PK,PN,PR,PS,PW,PY,QA,RE,RU,RW,PM,RS,SA,SB,SC,SD,SG,SH,SJ,SL,SM,SN,SO,SR,SS,ST,GS,MF,LC,LK,SX,SY,VC,WS,ZA,TC,TG,TH,TJ,TK,TL,TM,TN,TO,TR,TT,TV,TW,TZ,UA,UG,AE,UM,US,UY,UZ,VE,VG,VI,VN,VU,EH,WF,YE,ZM,ZW',
					'cs_us_states_target'                   => 'US_AL,US_АК,US_AZ,US_AR,US_CA,US_CO,US_CT,US_DE,US_DC,US_FL,US_GA,US_HI,US_ID,US_IL,US_IN,US_IA,US_KS,US_KY,US_LA,US_ME,US_MD,US_МА,US_MI,US_MN,US_MS,US_МО,US_MT,US_NE,US_NV,US_NH,US_NJ,US_NM,US_NY,US_NC,US_ND,US_ОН,US_ОК,US_OR,US_РА,US_RI,US_SC,US_SD,US_TN,US_ТХ,US_UT,US_VT,US_VA,US_WA,US_WV,US_WI,US_WY',
					'cs_use_meta_ldu'                       => 0,
					'cs_google_consent_mode'                => 0,
					'cs_bing_consent_mode'                  => 0,
					'cs_reddit_ldu'                         => 0,
					'cs_order'                              => 2,
					'cs_enable_rule'                        => 0,
					'cs_no_ip_rule'                         => 0,
					'cs_track_analytics'                    => 0,
					'cs_custom_text'                        => 0,
					'cs_block_content'                      => 0,
					'cs_rule_status'                        => 'publish',
					'cs_refresh_after_consent'              => 0,
					'excluded_from_consent_storing'         => 0,
					'cs_showing_rule_until_express_consent' => 0,
					'cs_native_scripts'                     => 0,
				),
			);

			$unassigned    = get_term_by( 'slug', 'unassigned', 'cs-cookies-category' );
			$unassigned_id = $unassigned->term_id;
			$categories    = get_cookies_terms_objects( null, false, $unassigned_id );

			$categories_options = array();
			if ( !empty( $categories ) ) {
				foreach ( $categories as $category ) {
					$categories_options[ '_cs_smart_sticky_' . $category->term_id ]        = '1';
					$categories_options[ '_cs_smart_sticky_mobile_' . $category->term_id ] = '1';
					$categories_options[ '_cs_custom_button_' . $category->term_id ]       = '1';
				}
			}

			foreach ( $default_cookies as $cookie_data ) {
				if ( !$this->cs_post_exists_by_slug( $cookie_data[ 'slug' ], CMPRO_POST_TYPE ) ) {

					$category = get_term_by( 'slug', $cookie_data[ 'post_category' ], 'cs-category' );
					if ( $category && is_object( $category ) ) {

						$category_id = $category->term_id;
						$cookie_data = array(
							'post_type'     => CMPRO_POST_TYPE,
							'post_title'    => $cookie_data[ 'post_title' ],
							'post_name'     => $cookie_data[ 'slug' ],
							'post_content'  => $cookie_data[ 'post_content' ],
							'post_category' => array( $category_id ),
							'post_status'   => 'publish',
							'ping_status'   => 'closed',
							'post_author'   => 1,
							'meta_input'    => array(
								'_cs_type'                               => $cookie_data[ 'cs_type' ],
								'_cs_sticky'                             => $cookie_data[ 'cs_sticky' ],
								'_cs_smart_sticky'                       => $cookie_data[ 'cs_smart_sticky' ],
								'_cs_smart_sticky_mobile'                => $cookie_data[ 'cs_smart_sticky_mobile' ],
								'_cs_mobile_side_sticky'                 => $cookie_data[ 'cs_mobile_side_sticky' ],
								'_cs_custom_button'                      => $cookie_data[ 'cs_custom_button' ],
								'_cs_bars_position'                      => $cookie_data[ 'cs_bars_position' ],
								'_cs_bars_type'                          => $cookie_data[ 'cs_bars_type' ],
								'_cs_top_push'                           => $cookie_data[ 'cs_top_push' ],
								'_cs_privacy_link'                       => $cookie_data[ 'cs_privacy_link' ],
								'_cs_close_on_scroll'                    => $cookie_data[ 'cs_close_on_scroll' ],
								'_cs_theme'                              => $cookie_data[ 'cs_theme' ],
								'_cs_design_type'                        => $cookie_data[ 'cs_design_type' ],
								'_cs_deny_all_btn'                       => $cookie_data[ 'cs_deny_all_btn' ],
								'_cs_hide_close_btn'                     => $cookie_data[ 'cs_hide_close_btn' ],
								'_cs_deny_consent_for_close'             => $cookie_data[ 'cs_deny_consent_for_close' ],
								'_cs_predefined_rule'                    => $cookie_data[ 'cs_predefined_rule' ],
								'_cs_target'                             => $cookie_data[ 'cs_target' ],
								'_cs_us_states_target'                   => $cookie_data[ 'cs_us_states_target' ],
								'_cs_use_meta_ldu'                       => $cookie_data[ 'cs_use_meta_ldu' ],
								'_cs_google_consent_mode'                => $cookie_data[ 'cs_google_consent_mode' ],
								'_cs_bing_consent_mode'                  => $cookie_data[ 'cs_bing_consent_mode' ],
								'_cs_reddit_ldu'                         => $cookie_data[ 'cs_reddit_ldu' ],
								'_cs_custom_text'                        => $cookie_data[ 'cs_custom_text' ],
								'_cs_order'                              => $cookie_data[ 'cs_order' ],
								'_cs_enable_rule'                        => $cookie_data[ 'cs_enable_rule' ],
								'_cs_no_ip_rule'                         => $cookie_data[ 'cs_no_ip_rule' ],
								'_cs_track_analytics'                    => $cookie_data[ 'cs_track_analytics' ],
								'_cs_block_content'                      => $cookie_data[ 'cs_block_content' ],
								'_cs_refresh_after_consent'              => $cookie_data[ 'cs_refresh_after_consent' ],
								'_excluded_from_consent_storing'         => $cookie_data[ 'excluded_from_consent_storing' ],
								'_cs_showing_rule_until_express_consent' => $cookie_data[ 'cs_showing_rule_until_express_consent' ],
								'_cs_native_scripts'                     => $cookie_data[ 'cs_native_scripts' ],
							)
						);

						$cookie_data[ 'meta_input' ] = array_merge( $cookie_data[ 'meta_input' ], $categories_options );
						$post_id                     = wp_insert_post( $cookie_data );

						wp_set_object_terms( $post_id, $cookie_data[ 'post_category' ], 'cs-category' );
					}
				}
			}
		}

		//check for new options
		if ( ConsentMagic()->getOption( 'cs_check_rule_settings' ) != CMPRO_LATEST_VERSION_NUMBER ) {

			$args  = array(
				'post_type'      => CMPRO_POST_TYPE,
				'post_status'    => 'publish',
				'posts_per_page' => -1,
				'order'          => 'ASC',
			);
			$rules = get_posts( $args );

			$unassigned              = get_term_by( 'slug', 'unassigned', 'cs-cookies-category' );
			$unassigned_id           = $unassigned->term_id;
			$options                 = get_cookies_terms_objects( null, false, $unassigned_id );
			$check_settings          = ConsentMagic()->getOption( 'cs_check_rule_settings' );
			$google_consent_mode_old = ConsentMagic()->getOption( 'cs_google_consent_mode' );

			if ( $rules ) {
				foreach ( $rules as $rule ) {
					$smart_sticky = get_post_meta( $rule->ID, '_cs_smart_sticky', true );
					if ( $smart_sticky === '' ) {
						update_post_meta( $rule->ID, '_cs_smart_sticky', 0 );
						foreach ( $options as $option ) {
							update_post_meta( $rule->ID, '_cs_smart_sticky_' . $option->term_id, 1 );
						}
					}
					$smart_sticky_mobile = get_post_meta( $rule->ID, '_cs_smart_sticky_mobile', true );
					if ( $smart_sticky_mobile === '' ) {
						update_post_meta( $rule->ID, '_cs_smart_sticky_mobile', 0 );
						foreach ( $options as $option ) {
							update_post_meta( $rule->ID, '_cs_smart_sticky_mobile_' . $option->term_id, 1 );
						}
					}
					$custom_button = get_post_meta( $rule->ID, '_cs_custom_button', true );
					if ( $custom_button === '' ) {
						update_post_meta( $rule->ID, '_cs_custom_button', 0 );
						foreach ( $options as $option ) {
							update_post_meta( $rule->ID, '_cs_custom_button_' . $option->term_id, 1 );
						}
					}

					$deny_consent_for_close = get_post_meta( $rule->ID, '_cs_deny_consent_for_close', true );
					if ( $deny_consent_for_close === '' ) {
						update_post_meta( $rule->ID, '_cs_deny_consent_for_close', 0 );
					}

					$cs_design_type = get_post_meta( $rule->ID, '_cs_design_type', true );
					if ( $cs_design_type === '' ) {
						update_post_meta( $rule->ID, '_cs_design_type', 'multi' );
					}

					$native_scripts = get_post_meta( $rule->ID, '_cs_native_scripts', true );
					if ( $native_scripts === '' ) {
						update_post_meta( $rule->ID, '_cs_native_scripts', 0 );
					}

					$cs_type = get_post_meta( $rule->ID, '_cs_type', true );

					//add US states
					if ( version_compare( $check_settings, '4.1.0.8', '<=' ) && $rule->post_name != 'cs_ldu_rule' ) {
						$country_targets = get_post_meta( $rule->ID, '_cs_target', true );
						$states          = '';
						if ( strpos( $country_targets, 'CCPA' ) !== false ) {
							$states          = 'US_CA';
							$country_targets .= ',US';
							update_post_meta( $rule->ID, '_cs_target', $country_targets );
						} elseif ( strpos( $country_targets, 'US' ) !== false ) {
							$states = 'US_AL,US_АК,US_AZ,US_AR,US_CA,US_CO,US_CT,US_DE,US_DC,US_FL,US_GA,US_HI,US_ID,US_IL,US_IN,US_IA,US_KS,US_KY,US_LA,US_ME,US_MD,US_МА,US_MI,US_MN,US_MS,US_МО,US_MT,US_NE,US_NV,US_NH,US_NJ,US_NM,US_NY,US_NC,US_ND,US_ОН,US_ОК,US_OR,US_РА,US_RI,US_SC,US_SD,US_TN,US_ТХ,US_UT,US_VT,US_VA,US_WA,US_WV,US_WI,US_WY';
						}
						update_post_meta( $rule->ID, '_cs_us_states_target', $states );
						update_post_meta( $rule->ID, '_cs_use_meta_ldu', 0 );

						if ( $cs_type == 'inform_and_opiout'
						     || $cs_type == 'ask_before_tracking'
						     || $cs_type == 'iab' ) {
							update_post_meta( $rule->ID, '_cs_google_consent_mode', $google_consent_mode_old );
						} else {
							update_post_meta( $rule->ID, '_cs_google_consent_mode', 0 );
						}

						//remove CCPA rule
						if ( $rule->post_name == 'cs_ccpa_rule' ) {
							wp_delete_post( $rule->ID, true );
						}
					}

					//Bing consent mode
					$bing_consent_mode = get_post_meta( $rule->ID, '_cs_bing_consent_mode', true );
					if ( $bing_consent_mode === '' ) {
						if ( $cs_type == 'inform_and_opiout'
						     || $cs_type == 'ask_before_tracking'
						     || $cs_type == 'iab' ) {
							update_post_meta( $rule->ID, '_cs_bing_consent_mode', 1 );
						} else {
							update_post_meta( $rule->ID, '_cs_bing_consent_mode', 0 );
						}
					}

					//Reddit consent mode
					$reddit_consent_mode = get_post_meta( $rule->ID, '_cs_reddit_ldu', true );
					if ( $reddit_consent_mode === '' ) {
						if ( $cs_type == 'inform_and_opiout' ) {
							update_post_meta( $rule->ID, '_cs_reddit_ldu', 1 );
						} else {
							update_post_meta( $rule->ID, '_cs_reddit_ldu', 0 );
						}
					}
				}
			}

			ConsentMagic()->updateOptions( array( 'cs_check_rule_settings' => CMPRO_LATEST_VERSION_NUMBER ) );
		}
	}

	public function cs_insert_templates_lists() {

		//check template changes for current plugin version
		if ( ConsentMagic()->getOption( 'cs_check_design' ) != CMPRO_LATEST_VERSION_NUMBER ) {

			$white                        = '#ffffff';
			$buttons_text_color           = '#212121';
			$accept_all_buttons_bg        = '#E16B43';
			$custom_button_buttons_bg     = '#F0F0F0';
			$deny_all_buttons_bg          = '#F0F0F0';
			$links_color                  = '#005BD3';
			$cs_tab_buttons_bg            = "#E7ECF1";
			$cs_tab_buttons_text_color    = "#4F5E71";
			$text_color                   = '#5D5D5D';
			$text_color_type2             = '#d1d1d1';
			$tittles_text_color           = '#252A31';
			$tittles_text_color_type2     = '#acb9cb';
			$cs_active_toggle_color       = '#0172CB';
			$cs_active_toggle_color_type2 = '#259fff';
			$black                        = '#000000';

			$general_templates = array();
			$default           = array(
				array(
					'post_title'                          => 'Light',
					'slug'                                => 'cs_light_theme',
					'cs_backend_color'                    => $white,
					'cs_border_style'                     => 'solid',
					'cs_border_weight'                    => '0',
					'cs_border_color'                     => '#333333',
					'cs_text_block_bg'                    => '#F3F7FB',
					'cs_text_color'                       => $text_color,
					'cs_links_color'                      => $links_color,
					'cs_titles_text_color'                => $tittles_text_color,
					'cs_subtitles_text_color'             => $text_color,
					'cs_accept_all_buttons_bg'            => $accept_all_buttons_bg,
					'cs_accept_all_buttons_text_color'    => $white,
					'cs_custom_button_buttons_bg'         => $custom_button_buttons_bg,
					'cs_custom_button_buttons_text_color' => $buttons_text_color,
					'cs_deny_all_buttons_bg'              => $custom_button_buttons_bg,
					'cs_deny_all_buttons_text_color'      => $buttons_text_color,
					'cs_options_buttons_bg'               => $custom_button_buttons_bg,
					'cs_options_buttons_text_color'       => $buttons_text_color,
					'cs_confirm_buttons_bg'               => $custom_button_buttons_bg,
					'cs_confirm_buttons_text_color'       => $buttons_text_color,
					'cs_sticky_bg'                        => $white,
					'cs_sticky_link_color'                => $buttons_text_color,
					'cs_logo'                             => ' ',
					'cs_logo_size'                        => 'thumbnail',
					'cs_position_vertical_list'           => 'top',
					'cs_position_horizontal_list'         => 'left',
					'cs_cat_color'                        => '#219653',
					'cs_active_toggle_color'              => $cs_active_toggle_color,
					'cs_active_toggle_text_color'         => $white,
					'cs_main_template'                    => 'main',
					'cs_shortcodes_text_color'            => $deny_all_buttons_bg,
					'cs_tab_buttons_bg'                   => $cs_tab_buttons_bg,
					'cs_tab_buttons_text_color'           => $cs_tab_buttons_text_color,
				),
				array(
					'post_title'                          => 'Dark',
					'slug'                                => 'cs_dark_theme',
					'cs_backend_color'                    => '#333333',
					'cs_border_style'                     => 'solid',
					'cs_border_weight'                    => '0',
					'cs_border_color'                     => $deny_all_buttons_bg,
					'cs_text_block_bg'                    => '#494949',
					'cs_text_color'                       => $white,
					'cs_links_color'                      => $cs_active_toggle_color_type2,
					'cs_titles_text_color'                => $tittles_text_color_type2,
					'cs_subtitles_text_color'             => $tittles_text_color_type2,
					'cs_accept_all_buttons_bg'            => $accept_all_buttons_bg,
					'cs_accept_all_buttons_text_color'    => $white,
					'cs_custom_button_buttons_bg'         => $black,
					'cs_custom_button_buttons_text_color' => $white,
					'cs_deny_all_buttons_bg'              => $black,
					'cs_deny_all_buttons_text_color'      => $white,
					'cs_options_buttons_bg'               => $black,
					'cs_options_buttons_text_color'       => $white,
					'cs_confirm_buttons_bg'               => $black,
					'cs_confirm_buttons_text_color'       => $white,
					'cs_sticky_bg'                        => $black,
					'cs_sticky_link_color'                => $white,
					'cs_logo'                             => ' ',
					'cs_logo_size'                        => 'thumbnail',
					'cs_position_vertical_list'           => 'top',
					'cs_position_horizontal_list'         => 'left',
					'cs_cat_color'                        => '#219653',
					'cs_active_toggle_color'              => $cs_active_toggle_color_type2,
					'cs_active_toggle_text_color'         => $white,
					'cs_main_template'                    => '',
					'cs_shortcodes_text_color'            => $deny_all_buttons_bg,
					'cs_tab_buttons_bg'                   => '#515151',
					'cs_tab_buttons_text_color'           => '#dfdfdf',
				),
				array(
					'post_title'                          => 'Orange',
					'slug'                                => 'cs_orange_theme',
					'cs_backend_color'                    => $accept_all_buttons_bg,
					'cs_border_style'                     => 'solid',
					'cs_border_weight'                    => '0',
					'cs_border_color'                     => $deny_all_buttons_bg,
					'cs_text_block_bg'                    => '#a75b55',
					'cs_text_color'                       => $white,
					'cs_links_color'                      => $links_color,
					'cs_titles_text_color'                => $text_color_type2,
					'cs_subtitles_text_color'             => $text_color_type2,
					'cs_accept_all_buttons_bg'            => '#472323',
					'cs_accept_all_buttons_text_color'    => $white,
					'cs_custom_button_buttons_bg'         => $accept_all_buttons_bg,
					'cs_custom_button_buttons_text_color' => $white,
					'cs_deny_all_buttons_bg'              => $accept_all_buttons_bg,
					'cs_deny_all_buttons_text_color'      => $white,
					'cs_options_buttons_bg'               => $accept_all_buttons_bg,
					'cs_options_buttons_text_color'       => $white,
					'cs_confirm_buttons_bg'               => $accept_all_buttons_bg,
					'cs_confirm_buttons_text_color'       => $white,
					'cs_sticky_bg'                        => $accept_all_buttons_bg,
					'cs_sticky_link_color'                => $white,
					'cs_logo'                             => ' ',
					'cs_logo_size'                        => 'thumbnail',
					'cs_position_vertical_list'           => 'top',
					'cs_position_horizontal_list'         => 'left',
					'cs_cat_color'                        => '#EBC686',
					'cs_active_toggle_color'              => '#673AB7',
					'cs_active_toggle_text_color'         => $white,
					'cs_main_template'                    => '',
					'cs_shortcodes_text_color'            => $deny_all_buttons_bg,
					'cs_tab_buttons_bg'                   => $cs_tab_buttons_bg,
					'cs_tab_buttons_text_color'           => $cs_tab_buttons_text_color,
				),
			);

			foreach ( $default as $template_data ) {
				$meta = array(
					'cs_backend_color'                    => $template_data[ 'cs_backend_color' ],
					'cs_border_style'                     => $template_data[ 'cs_border_style' ],
					'cs_border_weight'                    => $template_data[ 'cs_border_weight' ],
					'cs_border_color'                     => $template_data[ 'cs_border_color' ],
					'cs_text_block_bg'                    => $template_data[ 'cs_text_block_bg' ],
					'cs_text_color'                       => $template_data[ 'cs_text_color' ],
					'cs_links_color'                      => $template_data[ 'cs_links_color' ],
					'cs_titles_text_color'                => $template_data[ 'cs_titles_text_color' ],
					'cs_subtitles_text_color'             => $template_data[ 'cs_subtitles_text_color' ],
					'cs_accept_all_buttons_bg'            => $template_data[ 'cs_accept_all_buttons_bg' ],
					'cs_accept_all_buttons_text_color'    => $template_data[ 'cs_accept_all_buttons_text_color' ],
					'cs_custom_button_buttons_bg'         => $template_data[ 'cs_custom_button_buttons_bg' ],
					'cs_custom_button_buttons_text_color' => $template_data[ 'cs_custom_button_buttons_text_color' ],
					'cs_deny_all_buttons_bg'              => $template_data[ 'cs_deny_all_buttons_bg' ],
					'cs_deny_all_buttons_text_color'      => $template_data[ 'cs_deny_all_buttons_text_color' ],
					'cs_options_buttons_text_color'       => $template_data[ 'cs_options_buttons_text_color' ],
					'cs_options_buttons_bg'               => $template_data[ 'cs_options_buttons_bg' ],
					'cs_confirm_buttons_bg'               => $template_data[ 'cs_confirm_buttons_bg' ],
					'cs_confirm_buttons_text_color'       => $template_data[ 'cs_confirm_buttons_text_color' ],
					'cs_sticky_bg'                        => $template_data[ 'cs_sticky_bg' ],
					'cs_sticky_link_color'                => $template_data[ 'cs_sticky_link_color' ],
					'cs_logo'                             => $template_data[ 'cs_logo' ],
					'cs_logo_size'                        => $template_data[ 'cs_logo_size' ],
					'cs_position_vertical_list'           => $template_data[ 'cs_position_vertical_list' ],
					'cs_position_horizontal_list'         => $template_data[ 'cs_position_horizontal_list' ],
					'cs_cat_color'                        => $template_data[ 'cs_cat_color' ],
					'cs_active_toggle_color'              => $template_data[ 'cs_active_toggle_color' ],
					'cs_active_toggle_text_color'         => $template_data[ 'cs_active_toggle_text_color' ],
					'cs_main_template'                    => $template_data[ 'cs_main_template' ],
					'cs_shortcodes_text_color'            => $template_data[ 'cs_shortcodes_text_color' ],
					'cs_tab_buttons_bg'                   => $template_data[ 'cs_tab_buttons_bg' ],
					'cs_tab_buttons_text_color'           => $template_data[ 'cs_tab_buttons_text_color' ],
				);

				if ( !$this->cs_post_exists_by_slug( $template_data[ 'slug' ], CMPRO_TEMPLATE_POST_TYPE ) ) {

					$cookie_data = array(
						'post_type'   => CMPRO_TEMPLATE_POST_TYPE,
						'post_title'  => $template_data[ 'post_title' ],
						'post_name'   => $template_data[ 'slug' ],
						'post_status' => 'publish',
						'ping_status' => 'closed',
						'post_author' => 1,
						'meta_input'  => $meta
					);
					$id          = wp_insert_post( $cookie_data );

					$general_templates[] = $id;

				} else {
					$template_id = get_post_id_by_slug( $template_data[ 'slug' ], CMPRO_TEMPLATE_POST_TYPE );
					if ( $template_id ) {
						foreach ( $meta as $key => $value ) {
							update_post_meta( $template_id, $key, $value );
						}
						$general_templates[] = $template_id;
					}
				}
			}

			$default_meta = array(
				'cs_backend_color',
				'cs_border_style',
				'cs_border_weight',
				'cs_border_color',
				'cs_text_block_bg',
				'cs_text_color',
				'cs_links_color',
				'cs_titles_text_color',
				'cs_subtitles_text_color',
				'cs_accept_all_buttons_bg',
				'cs_accept_all_buttons_text_color',
				'cs_custom_button_buttons_bg',
				'cs_custom_button_buttons_text_color',
				'cs_deny_all_buttons_bg',
				'cs_deny_all_buttons_text_color',
				'cs_options_buttons_text_color',
				'cs_options_buttons_bg',
				'cs_confirm_buttons_bg',
				'cs_confirm_buttons_text_color',
				'cs_sticky_bg',
				'cs_sticky_link_color',
				'cs_logo',
				'cs_logo_size',
				'cs_position_vertical_list',
				'cs_position_horizontal_list',
				'cs_cat_color',
				'cs_active_toggle_color',
				'cs_active_toggle_text_color',
				'cs_main_template',
				'cs_shortcodes_text_color',
				'cs_tab_buttons_bg',
				'cs_tab_buttons_text_color',
			);

			//check fields for user templates
			$all_templates = array_column( get_custom_theme_list(), 'option_value' );
			foreach ( $all_templates as $template ) {
				if ( !in_array( $template, $general_templates ) ) {
					foreach ( $default_meta as $value ) {
						if ( empty( get_post_meta( $template, $value, true ) ) ) {
							update_post_meta( $template, $value, ConsentMagic()->getOption( $value ) );
						}
					}
					update_post_meta( $template, 'cs_main_template', '' );
				}
			}

			ConsentMagic()->updateOptions( array( 'cs_check_design' => CMPRO_LATEST_VERSION_NUMBER ) );
		}
	}

	public function cs_post_exists_by_slug( $post_name, $post_type ) {
		global $wpdb;
		if ( $wpdb->query( $wpdb->prepare( "SELECT post_name FROM $wpdb->posts WHERE post_name = %s AND post_type = %s", $post_name, $post_type ) ) ) {
			return true;
		} else {
			return false;
		}
	}

	public function cs_cron_check_status_edd() {
		if ( !wp_next_scheduled( 'cs_cron_check_status_edd_hook' ) ) {
			wp_schedule_event( time(), 'daily', 'cs_cron_check_status_edd_hook' );
		}
	}

	public function cs_cron_check_edd_status() {
		if ( !class_exists( 'ConsentMagicPro\CS_EDD_Updater' ) ) {

			require_once CMPRO_PLUGIN_PATH . 'includes/CS_EDD_Updater.php';

			$license_edd = new CS_EDD_License_manager( 'consent-magic', CMPRO_LICENSE_TYPE, CMPRO_LATEST_VERSION_NUMBER, CMPRO_LICENSE_NAME );
			$license_key = ConsentMagic()->getOption( $license_edd->edd_license_key );

			$plugin_updater = new CS_EDD_Updater( $license_edd->api_url, CMPRO_PLUGIN_FILENAME, array(
				'version'   => CMPRO_LATEST_VERSION_NUMBER,
				'license'   => $license_key,
				'item_name' => CMPRO_LICENSE_NAME,
				'author'    => CMPRO_LICENSE_NAME
			) );
			set_transient( $license_edd->edd_check_for_update_key, $license_edd->edd_check_for_update_key, DAY_IN_SECONDS );

		}
	}

	/**
	 * Check geolocation status and out message
	 * Out message for cache notification
	 * Out message for geolocation update notification
	 * @return void
	 */
	public function cs_admin_messages() {
		if ( ConsentMagic()->getOption( 'cs_check_flow' ) ) {
			$url = admin_url( basename( sanitize_text_field( $_SERVER[ 'REQUEST_URI' ] ) ) );
			if ( strpos( $url, '?' ) === false ) {
				$url .= '?';
			} else {
				$url .= '&';
			}

			if ( ( (int) ConsentMagic()->getOption( 'cs_geolocation' ) === 0
			       || (int) ConsentMagic()->getOption( 'cs_geo_activated' ) === 0 ) ) {
				$warning = '<div class="notice notice-warning cm-fixed-notice" style="position: relative">';
				$msg     = '<p>'
				           . esc_html__( 'Consent Magic - Geolocation not configured. Please visit the ', 'consent-magic' )
				           . '<a href="' . admin_url( 'admin.php?page=cs-geolocation' ) . '" class="notice-link">'
				           . esc_html__( 'Geolocation settings page', 'consent-magic' ) . '</a>'
				           . esc_html__( ' and configure geolocation.', 'consent-magic' ) . '</p>';
				$button  = '<a href="' . esc_attr( $url )
				           . 'cs_geo_disable_message"><button type="button" class="notice-dismiss"></button></a>';

				if ( !get_user_meta( get_current_user_id(), 'cs_geo_enabled', true ) ) {
					echo $warning . $msg . $button . '</div>';
				}
			}

			if ( (int) ConsentMagic()->getOption( 'cs_geolocation' ) === 1
			     && (int) ConsentMagic()->getOption( 'cs_geo_activated' ) === 1
			     && defined( 'DISABLE_WP_CRON' )
			     && DISABLE_WP_CRON
			     && !get_user_meta( get_current_user_id(), 'cs_cron_message', true ) ) {
				$cs_geo = new CS_Geoip();
				if ( $cs_geo->check_database_paths() ) {
					echo '<div class="notice notice-warning cm-fixed-notice" style="position: relative"><p>'
					     . sprintf( esc_html__( 'Your ConsentMagic geolocation data base needs a manual download: %sclick to proceed%s.', 'consent-magic' ), '<a href="'
					                                                                                                                                         . admin_url( 'admin.php?page=cs-geolocation' )
					                                                                                                                                         . '" class="notice-link">', '</a>' )
					     . '</p><a href="' . esc_attr( $url )
					     . 'cs_cron_disable_message"><button type="button" class="notice-dismiss"></button></a></div>';
				}
			}

			if ( get_current_screen()->parent_base == 'consent-magic'
			     && (int) ConsentMagic()->getOption( 'cs_enable_site_cache' ) === 1
			     && !get_user_meta( get_current_user_id(), 'cs_cache_message', true ) ) {
				echo '<div class="notice notice-warning cm-fixed-notice" style="position: relative"><p>'
				     . esc_html__( 'Consent Magic - Usage cache: after updating the plugin settings, you need to clear the cache.', 'consent-magic' )
				     . '</p><a href="' . esc_attr( $url )
				     . 'cs_cache_disable_message"><button type="button" class="notice-dismiss"></button></a></div>';
			}
		}
	}

	/**
	 * delete notice for current user
	 * delete notice for cache message
	 * delete notice for geolocation update
	 * @return void
	 */
	public function cs_close_messages() {
		if ( isset( $_GET[ 'cs_geo_disable_message' ] ) ) {
			add_user_meta( get_current_user_id(), 'cs_geo_enabled', 'true', true );
		}

		if ( isset( $_GET[ 'cs_cache_disable_message' ] ) ) {
			add_user_meta( get_current_user_id(), 'cs_cache_message', 'true', true );
		}

		if ( isset( $_GET[ 'cs_cron_disable_message' ] ) ) {
			add_user_meta( get_current_user_id(), 'cs_cron_message', 'true', true );
		}
	}

	//load additional languages via ajax
	function cs_load_langs() {

		check_ajax_referer( 'cs-ajax-nonce', 'nonce_code' );
		$this->manageCSPermissionsAjax();

		$lang_list_out = array();

		if ( isset( $_POST[ 'current_lang' ] ) && isset( $_POST[ 'key' ] ) && isset( $_POST[ 'term' ] ) ) {

			$current_lang = sanitize_text_field( $_POST[ 'current_lang' ] );
			$key          = sanitize_text_field( $_POST[ 'key' ] );
			$translations = array();

			if ( $_POST[ 'term' ] == 1 ) {
				$term = get_term_by( 'id', $key, 'cs-cookies-category' );
				if ( $term ) {
					$result       = ConsentMagic()->getTranslations( $term->slug, 'term' );
					$translations = array_reduce( $result, function( $carry, $item ) {
						$carry[ $item[ 'language' ] ] = [
							'descr' => $item[ 'value' ],
							'name'  => $item[ 'category_name' ]
						];

						return $carry;
					}, [] );
				}
			} else {
				$translations = ConsentMagic()->getTranslations( $key );
			}

			if ( !empty( $translations ) ) {

				$language_availability = ConsentMagic()->getOption( 'cs_language_availability' );

				if ( isset( $translations[ $current_lang ] ) ) {
					unset( $translations[ $current_lang ] );
				}

				if ( !empty( $translations ) ) {
					foreach ( $translations as $k => $translation ) {
						$lang_list_out[ $k ] = array(
							'enable' => $language_availability[ $k ] ?? 1,
							'value'  => $translation
						);
					}
				}
			}
			$message = '';
			if ( empty( $lang_list_out ) ) {
				$message = '<div class="language-message">';
				ob_start();
				render_warning_message( __( 'Additional languages not found', 'consent-magic' ) );
				$message .= ob_get_clean();
				ob_end_clean();
				$message .= '</div>';
			}
		} else {
			$message = '<div class="language-message">';
			ob_start();
			render_warning_message( __( 'An error occurred in the request', 'consent-magic' ) );
			$message .= ob_get_clean();
			ob_end_clean();
			$message .= '</div>';
		}

		wp_send_json_success( array(
			'message'   => $message,
			'lang_list' => $lang_list_out,
		) );

		wp_die();
	}

	public function cs_migrate_to_pro() {

		$gdpr_rule = get_post_id_by_slug_from_db( 'cs_gdpr_rule' );
		if ( $gdpr_rule ) {
			$args = array(
				'ID'         => $gdpr_rule,
				'post_title' => 'GDPR Rule',
			);
			wp_update_post( $args );
			update_post_meta( $gdpr_rule, '_cs_deny_all_btn', 1 );
			update_post_meta( $gdpr_rule, '_cs_target', 'GDPR,AD,AT,BE,BG,CY,CZ,DK,EE,FI,FR,DE,GR,HU,IE,IT,LT,LU,LV,MT,NL,NO,PL,PT,RO,SE,SI,SK,ES,GB' );
			update_post_meta( $gdpr_rule, '_cs_no_ip_rule', 1 );
			update_post_meta( $gdpr_rule, '_excluded_from_consent_storing', 0 );
			update_post_meta( $gdpr_rule, '_cs_custom_text', 0 );
		}

		$ldu_rule = get_post_id_by_slug_from_db( 'cs_ldu_rule' );
		if ( $ldu_rule ) {
			$args = array(
				'ID'         => $ldu_rule,
				'post_title' => 'Limited Data Use (LDU) Rule',
			);
			wp_update_post( $args );
			update_post_meta( $ldu_rule, '_cs_deny_all_btn', 0 );
			update_post_meta( $ldu_rule, '_cs_target', 'US' );
			update_post_meta( $ldu_rule, '_cs_us_states_target', 'US_CA,US_CO,US_CT,US_DE,US_FL,US_MT,US_NE,US_NH,US_NJ,US_OR,US_ТХ' );
			update_post_meta( $ldu_rule, '_cs_no_ip_rule', 0 );
			update_post_meta( $ldu_rule, '_excluded_from_consent_storing', 0 );
			update_post_meta( $ldu_rule, '_cs_custom_text', 0 );
		}

		$rest_rule = get_post_id_by_slug_from_db( 'cs_rest_of_world_rule' );
		if ( $rest_rule ) {
			$args = [
				'ID'         => $rest_rule,
				'post_title' => 'Rest of the world Rule',
			];
			wp_update_post( $args );
			update_post_meta( $rest_rule, '_cs_deny_all_btn', 0 );
			update_post_meta( $rest_rule, '_cs_target', 'NONGDPR,AF,AG,AI,AL,DZ,AM,AO,AQ,AR,AS,AU,AW,AX,AZ,BA,BB,BD,BF,BH,BI,BJ,BL,BM,BN,BO,BQ,BR,BS,BT,BV,BW,BY,BZ,CA,CC,CD,CF,CG,CI,CK,CL,CM,CN,CO,CR,CU,CV,CW,CX,HR,KH,KM,TD,KY,DJ,DM,DO,EZ,SV,SZ,EG,ER,GQ,ET,FG,FK,FO,GF,PF,TF,GA,GD,GE,GG,GH,GI,GL,GM,GN,GP,GT,GU,GW,GY,HK,VA,HM,HN,HT,ID,IL,IM,IN,IO,IQ,IR,IS,JE,JM,JO,JP,KE,KG,KI,KN,KP,KR,KW,KZ,LA,LB,LI,LR,LS,LY,MA,MC,MD,ME,MG,MH,ML,MM,MN,MO,MQ,MR,MS,MU,MV,MW,MX,MY,FM,YT,MZ,NM,NC,NE,NF,NG,NI,NO,NP,NR,NU,NZ,MK,MP,OM,PA,PE,PG,PH,PK,PN,PR,PS,PW,PY,QA,RE,RU,RW,PM,RS,SA,SB,SC,SD,SG,SH,SJ,SL,SM,SN,SO,SR,SS,ST,GS,MF,CH,LC,LK,SX,SY,VC,WS,ZA,TC,TG,TH,TJ,TK,TL,TM,TN,TO,TR,TT,TV,TW,TZ,UA,UG,AE,UM,US,UY,UZ,VE,VG,VI,VN,VU,EH,WF,YE,ZM,ZW' );
			update_post_meta( $rest_rule, '_cs_no_ip_rule', 0 );
			update_post_meta( $rest_rule, '_excluded_from_consent_storing', 0 );
			update_post_meta( $rest_rule, '_cs_custom_text', 0 );
		}

		ConsentMagic()->deleteOption( 'cs_free' );
	}

	public function add_style_option( $styles ) {
		$styles[] = 'display';

		return $styles;
	}

	/**
	 * Export settings
	 * @return void
	 */
	function cs_export_settings() {

		// Check nonce:
		check_ajax_referer( 'cs-ajax-nonce', 'nonce_code' );
		$this->manageCSPermissionsAjax();

		require_once CMPRO_PLUGIN_PATH . '/includes/functions/cs-settings-keys.php';

		$settings_income = array();

		if ( isset( $_POST[ 'script_types' ] ) && $_POST[ 'script_types' ] === 'true' ) {
			$settings_income[] = 'script_types';
		}
		if ( isset( $_POST[ 'design_settings' ] ) && $_POST[ 'design_settings' ] === 'true' ) {
			$settings_income[] = 'design_settings';
		}
		if ( isset( $_POST[ 'general_settings' ] ) && $_POST[ 'general_settings' ] === 'true' ) {
			$settings_income[] = 'general_settings';
		}
		if ( isset( $_POST[ 'geolocation_settings' ] ) && $_POST[ 'geolocation_settings' ] === 'true' ) {
			$settings_income[] = 'geolocation_settings';
		}
		if ( isset( $_POST[ 'predefined_scripts' ] ) && $_POST[ 'predefined_scripts' ] === 'true' ) {
			$settings_income[] = 'predefined_scripts';
		}
		if ( isset( $_POST[ 'custom_scripts' ] ) && $_POST[ 'custom_scripts' ] === 'true' ) {
			$settings_income[] = 'custom_scripts';
		}
		if ( isset( $_POST[ 'text' ] ) && $_POST[ 'text' ] === 'true' ) {
			$settings_income[] = 'text';
		}
		if ( isset( $_POST[ 'iab_settings' ] ) && $_POST[ 'iab_settings' ] === 'true' ) {
			$settings_income[] = 'iab_settings';
		}
		if ( isset( $_POST[ 'default_rules' ] ) && $_POST[ 'default_rules' ] === 'true' ) {
			$settings_income[] = 'default_rules';
		}
		if ( isset( $_POST[ 'custom_rules' ] ) && $_POST[ 'custom_rules' ] === 'true' ) {
			$settings_income[] = 'custom_rules';
		}

		$settings_keys = get_cm_settings_keys( $settings_income );
		$langs_keys    = get_language_keys();
		$settings      = array();

		foreach ( $settings_keys as $part => $settings_part ) {
			if ( empty( $settings[ $part ] ) ) {
				$settings[ $part ] = array();
			}

			switch ( $part ) {
				case 'cm_options' :
					foreach ( $settings_part as $k => $setting ) {

						if ( $k == 'cs_ignore_users_ip' ) {
							global $wpdb;
							$table_name              = $wpdb->prefix . 'cs_unblock_ip';
							$ips                     = $wpdb->get_results( $wpdb->prepare( 'SELECT `ip`, `created_at` FROM %1$s', $table_name ), ARRAY_A );
							$settings[ $part ][ $k ] = $ips;
						} elseif ( $k == 'cmpro_iab_settings' ) {
							$value                   = ConsentMagic()->getOption( $k );
							$settings[ $part ][ $k ] = json_encode( $value );
						} else {
							$settings[ $part ][ $k ] = ConsentMagic()->getOption( $k );
						}
					}
					break;

				case 'rules' :
					foreach ( $settings_part as $k => $setting_part ) {
						$default_rules = array();
						foreach ( static::$default_rules as $default_rule ) {
							$default_rules[] = get_page_id_by_path( $default_rule, '', CMPRO_POST_TYPE );
						}

						$unassigned    = get_term_by( 'slug', 'unassigned', 'cs-cookies-category' );
						$unassigned_id = $unassigned->term_id;
						$scripts       = get_cookies_terms_objects( null, false, $unassigned_id );

						$args = array(
							'post_type'      => CMPRO_POST_TYPE,
							'post_status'    => 'publish',
							'posts_per_page' => -1,
						);

						if ( $k === 'default' ) {
							$args[ 'post__in' ] = $default_rules;
						} else {
							$args[ 'post__not_in' ] = $default_rules;
						}

						$loop_posts = new \WP_Query( $args );
						if ( !$loop_posts->have_posts() ) {
							break;
						} else {
							while ( $loop_posts->have_posts() ) {
								$loop_posts->the_post();
								$rule = array(
									'post_data' => array(
										'post_title' => $loop_posts->post->post_title,
										'post_name'  => $loop_posts->post->post_name,
										'post_id'    => $loop_posts->post->ID
									),
									'post_meta' => array(),
								);
								foreach ( $setting_part as $s => $setting ) {
									if ( $s == '_cs_smart_sticky_'
									     || $s == '_cs_smart_sticky_mobile_'
									     || $s == '_cs_custom_button_' ) {
										$values = array();
										foreach ( $scripts as $script ) {
											$values[ $script->slug ] = get_post_meta( $loop_posts->post->ID, $s
											                                                                 . $script->term_id, true );
										}
										$rule[ 'post_meta' ][ $s ] = $values;

									} elseif ( $s == '_cs_theme' ) {
										$template_id = get_post_meta( $loop_posts->post->ID, $s, true );
										$template    = get_post( $template_id );
										if ( !empty( $template ) && !is_wp_error( $template ) ) {
											$rule[ 'post_meta' ][ $s ] = $template->post_name;
										} else {
											$rule[ 'post_meta' ][ $s ] = '';
										}
									} else {
										$rule[ 'post_meta' ][ $s ] = get_post_meta( $loop_posts->post->ID, $s, true );
									}
								}

								$settings[ $part ][] = $rule;
							}
						}
					}
					break;

				case 'script_types' :
					$args         = array(
						'taxonomy'   => 'cs-cookies-category',
						'hide_empty' => false,
					);
					$script_types = get_terms( $args );

					if ( $script_types && !is_wp_error( $script_types ) ) {
						foreach ( $script_types as $script_type ) {
							$script = array(
								'term_data' => array(
									'term_title' => $script_type->name,
									'term_slug'  => $script_type->slug,
									'term_id'    => $script_type->term_id,
								),
								'term_meta' => array(),
							);

							foreach ( $settings_part as $s => $setting ) {
								$script[ 'term_meta' ][ $s ] = get_term_meta( $script_type->term_id, $s, true );
							}

							$settings[ $part ][] = $script;
						}
					}
					break;

				case 'design' :
					$cs_light_theme  = get_page_id_by_path( 'cs_light_theme', '', CMPRO_TEMPLATE_POST_TYPE );
					$cs_dark_theme   = get_page_id_by_path( 'cs_dark_theme', '', CMPRO_TEMPLATE_POST_TYPE );
					$cs_orange_theme = get_page_id_by_path( 'cs_orange_theme', '', CMPRO_TEMPLATE_POST_TYPE );

					$args = array(
						'post_type'      => CMPRO_TEMPLATE_POST_TYPE,
						'post_status'    => 'any',
						'posts_per_page' => -1,
						'post__not_in'   => array(
							$cs_light_theme,
							$cs_dark_theme,
							$cs_orange_theme
						)
					);

					$loop_posts = new \WP_Query( $args );
					if ( !$loop_posts->have_posts() ) {
						$settings[ $part ][ 'design_templates' ] = array();
					} else {
						while ( $loop_posts->have_posts() ) {
							$loop_posts->the_post();
							$template = array(
								'post_data' => array(
									'post_title' => $loop_posts->post->post_title,
									'post_name'  => $loop_posts->post->post_name
								),
								'post_meta' => array(),
							);
							foreach ( $settings_part[ 'design_templates' ] as $k => $setting ) {

								if ( $k == 'cs_logo' ) {
									$image_id     = get_post_meta( $loop_posts->post->ID, $k, true );
									$image_encode = '';
									if ( !empty( $image_id ) ) {
										$img          = wp_get_attachment_image_src( $image_id, 'full' );
										$image_size   = getimagesize( $img[ 0 ] );
										$image_data   = base64_encode( file_get_contents( $img[ 0 ] ) );
										$image_encode = "data:{$image_size['mime']};base64,{$image_data}";
									}

									$template[ 'post_meta' ][ $k ] = $image_encode;

								} else {
									$template[ 'post_meta' ][ $k ] = get_post_meta( $loop_posts->post->ID, $k, true );
								}
							}

							$settings[ $part ][ 'design_templates' ][] = $template;
						}
					}

					foreach ( $settings_part[ 'adjust_sizes' ] as $k => $setting ) {
						$settings[ $part ][ 'adjust_sizes' ][ $k ] = ConsentMagic()->getOption( $k );
					}
					break;

				case 'custom_scripts' :
					$args       = get_list_query_args( CMPRO_POST_TYPE_SCRIPTS );
					$loop_posts = new \WP_Query( $args );

					while ( $loop_posts->have_posts() ) {
						$loop_posts->the_post();
						$id            = get_the_ID();
						$term_list     = wp_get_object_terms( get_the_ID(), 'cs-cookies-category' );
						$custom_script = array(
							'script_data' => array(
								'script_title' => $loop_posts->post->post_title,
								'script_name'  => $loop_posts->post->post_name,
								'enabled'      => ConsentMagic()->getOption( 'cs_' . sanitize_title( get_the_title() )
								                                             . '_' . $id . '_script_enable' ),
								'term_slug'    => $term_list[ 0 ]->slug,
							),
							'script_meta' => array()
						);

						foreach ( $settings_part as $k => $setting ) {
							$custom_script[ 'script_meta' ][ $k ] = get_post_meta( $id, $k, true );
						}

						$settings[ $part ][] = $custom_script;
					}
					break;

				default :
					break;
			}
		}

		global $wpdb;
		$table_name = $wpdb->prefix . 'cs_translations';

		foreach ( $langs_keys as $p => $langs_part ) {
			foreach ( $langs_part as $k => $lang_setting ) {
				switch ( $p ) {
					case 'cm_options' :
						$sql = $wpdb->prepare( "
                            SELECT *
                                FROM $table_name
                                WHERE `option` = %s
                                    ", $k );

						$languages    = $wpdb->get_results( $sql, ARRAY_A );
						$translations = array();
						if ( !empty( $languages ) ) {
							foreach ( $languages as $language ) {
								$lang                  = $language[ 'language' ];
								$translations[ $lang ] = $language[ 'value' ];

							}
						}

						$settings[ $p ][ $k ] = $translations;
						break;

					case 'rules' :
						if ( !empty( $settings[ $p ] ) ) {
							foreach ( $settings[ $p ] as &$rule ) {
								$sql = $wpdb->prepare( "
                                        SELECT *
                                            FROM $table_name
                                            WHERE `option` = %s
                                                AND `meta` = 1 
                                                AND `meta_id` = %d
                                                ", $k, $rule[ 'post_data' ][ 'post_id' ] );

								$languages    = $wpdb->get_results( $sql, ARRAY_A );
								$translations = array();
								if ( !empty( $languages ) ) {
									foreach ( $languages as $language ) {
										$lang                  = $language[ 'language' ];
										$translations[ $lang ] = $language[ 'value' ];

									}
								}

								$rule[ 'post_meta' ][ $k ] = $translations;
							}
						}
						break;

					case 'script_types' :
						if ( !empty( $settings[ $p ] ) ) {
							foreach ( $settings[ $p ] as &$script ) {
								$sql = $wpdb->prepare( "
                                        SELECT *
                                            FROM $table_name
                                            WHERE `option` = %s
                                                AND `category` = 1
                                                ", $script[ 'term_data' ][ 'term_slug' ] );

								$languages    = $wpdb->get_results( $sql, ARRAY_A );
								$translations = array();
								if ( !empty( $languages ) ) {
									foreach ( $languages as $language ) {
										$lang                  = $language[ 'language' ];
										$translations[ $lang ] = array(
											'name'  => $language[ 'category_name' ],
											'descr' => $language[ 'value' ],
										);
									}
								}

								$script[ 'term_meta' ][ $k ] = $translations;
							}
						}
						break;

					default :
						break;

				}
			}
		}

		$settings = json_encode( $settings );

		wp_send_json( $settings );
	}

	/**
	 * Import settings
	 * @return void
	 */
	function cs_import_settings() {

		// Check nonce:
		check_ajax_referer( 'cs-ajax-nonce', 'nonce_code' );
		$this->manageCSPermissionsAjax();

		if ( isset( $_FILES[ 'cs_import_settings' ] ) ) {
			if ( $_FILES[ 'cs_import_settings' ][ 'size' ] == 0 ) {
				wp_send_json_error( __( 'File is empty', 'consent-magic' ) );
				wp_die();

				return;
			}
			if ( $_FILES[ 'cs_import_settings' ][ 'type' ] != "application/json" ) {
				wp_send_json_error( __( sprintf( 'File has wrong format %s', $_FILES[ 'cs_import_settings' ][ 'type' ] ), 'consent-magic' ) );
				wp_die();

				return;
			}
			$content = file_get_contents( $_FILES[ 'cs_import_settings' ][ 'tmp_name' ] );
			$data    = json_decode( $content, true );

			if ( empty( $data ) ) {
				wp_send_json_error( __( 'Invalid file', 'consent-magic' ) );
				wp_die();

				return;
			}

			require_once CMPRO_PLUGIN_PATH . '/includes/functions/cs-settings-keys.php';
			$langs_keys = get_language_keys();

			foreach ( $data as $part => $settings_part ) {
				switch ( $part ) {
					case 'cm_options' :
					case 'wp_options' :
						foreach ( $settings_part as $key => $setting ) {
							$key = sanitize_text_field( $key );
							if ( is_array( $setting ) ) {
								array_walk( $setting, array(
									ConsentMagic(),
									'cs_sanitize_array'
								) );
							} else {
								$setting = sanitize_text_field( $setting );
							}

							if ( $key == 'cs_ignore_users_ip' ) {
								if ( !empty( $setting ) ) {
									foreach ( $setting as $ip_group ) {
										global $wpdb;
										$table_name = $wpdb->prefix . 'cs_unblock_ip';
										$ip         = sanitize_text_field( $ip_group[ 'ip' ] );
										$created_at = sanitize_text_field( $ip_group[ 'created_at' ] );
										$like       = '%' . $wpdb->esc_like( $ip ) . '%';

										if ( $isset_ip = $wpdb->get_var( $wpdb->prepare( 'SELECT `id` FROM %1$s WHERE `ip` LIKE \'%2$s\'', $table_name, $like ) ) ) {
											$sql = $wpdb->prepare( 'UPDATE %1$s SET created_at=\'%2$s\' WHERE id=%3$s', $table_name, $created_at, $isset_ip );
										} else {
											$sql_arr[] = $wpdb->prepare( "(%s,%s)", $ip, $created_at );
											$sql       = "INSERT INTO `$table_name` (`ip`,`created_at`) VALUES "
											             . implode( ',', $sql_arr );
										}

										$wpdb->query( $sql );
									}
								}
							} elseif ( $key == 'cmpro_iab_settings' ) {
								$setting = json_decode( $setting );
								if ( is_object( $setting ) ) {
									$setting->vendorListVersion = 0;
								} else {
									$setting = null;
								}
								ConsentMagic()->updateOptions( array( $key => $setting ) );
							} else {
								if ( isset( $langs_keys[ $part ] ) && array_key_exists( $key, $langs_keys[ $part ] ) ) {
									foreach ( $setting as $lang => $lang_value ) {
										ConsentMagic()->updateLangOptions( $key, $lang_value, $lang );
									}
								} else {
									ConsentMagic()->updateOptions( array( $key => $setting ) );
								}
							}
						}
						break;

					case 'rules' :
						if ( !empty( $settings_part ) ) {

							$unassigned    = get_term_by( 'slug', 'unassigned', 'cs-cookies-category' );
							$unassigned_id = $unassigned->term_id;
							$scripts       = get_cookies_terms_objects( null, false, $unassigned_id );
							$scripts_slugs = array();
							foreach ( $scripts as $script ) {
								$scripts_slugs[] = $script->slug;
							}

							foreach ( $settings_part as $rule ) {

								$rule_meta    = array();
								$translations = array();

								foreach ( $rule[ 'post_meta' ] as $key => $setting ) {
									$key = sanitize_text_field( $key );
									if ( is_array( $setting ) ) {
										array_walk( $setting, array(
											ConsentMagic(),
											'cs_sanitize_array'
										) );
									} else {
										$setting = sanitize_text_field( $setting );
									}

									if ( isset( $langs_keys[ $part ] )
									     && array_key_exists( $key, $langs_keys[ $part ] ) ) {

										foreach ( $setting as $lang => $lang_value ) {
											if ( !isset( $translations[ $key ] ) ) {
												$translations[ $key ] = array();
											}

											$translations[ $key ][ $lang ] = $lang_value;
										}
									} else {
										if ( $key == '_cs_smart_sticky_'
										     || $key == '_cs_smart_sticky_mobile_'
										     || $key == '_cs_custom_button_' ) {
											foreach ( $setting as $key_script_in => $script_in ) {
												if ( ( $i = array_search( $key_script_in, $scripts_slugs ) )
												     !== false ) {
													$rule_meta[ $key . $scripts[ $i ]->term_id ] = $script_in;
												}
											}
										} elseif ( $key == '_cs_theme' ) {
											if ( $post = get_page_by_path( $setting, OBJECT, CMPRO_TEMPLATE_POST_TYPE ) ) {
												$rule_meta[ $key ] = $post->ID;
											} else {
												$rule_meta[ $key ] = 'cs_light_theme';
											}
										} else {
											$rule_meta[ $key ] = $setting;
										}
									}
								}

								$post_name = sanitize_text_field( $rule[ 'post_data' ][ 'post_name' ] );

								if ( $post = get_page_by_path( $post_name, OBJECT, CMPRO_POST_TYPE ) ) {
									$rule_data = array(
										'ID'         => $post->ID,
										'meta_input' => $rule_meta
									);

									$rule_id = wp_update_post( $rule_data );
								} else {
									$rule_data = array(
										'post_type'   => CMPRO_POST_TYPE,
										'post_title'  => sanitize_text_field( $rule[ 'post_data' ][ 'post_title' ] ),
										'post_name'   => $post_name,
										'post_status' => 'publish',
										'ping_status' => 'closed',
										'post_author' => 1,
										'meta_input'  => $rule_meta
									);

									$rule_id = wp_insert_post( $rule_data );
								}

								if ( !is_wp_error( $rule_id ) && !empty( $rule_id ) && !empty( $translations ) ) {
									foreach ( $translations as $key => $settings ) {
										foreach ( $settings as $lang => $lang_value ) {
											ConsentMagic()->updateLangOptions( $key, $lang_value, $lang, 'meta', $rule_id );
										}
									}
								}
							}
						}
						break;

					case 'script_types' :
						$update_consent = false;
						if ( !empty( $settings_part ) ) {
							foreach ( $settings_part as $script ) {

								$script_meta  = array();
								$translations = array();

								foreach ( $script[ 'term_meta' ] as $key => $setting ) {
									$key = sanitize_text_field( $key );
									if ( is_array( $setting ) ) {
										array_walk( $setting, array(
											ConsentMagic(),
											'cs_sanitize_array'
										) );
									} else {
										$setting = sanitize_text_field( $setting );
									}

									if ( isset( $langs_keys[ $part ] )
									     && array_key_exists( $key, $langs_keys[ $part ] ) ) {

										foreach ( $setting as $lang => $lang_value ) {
											if ( !isset( $translations[ $key ] ) ) {
												$translations[ $key ] = array();
											}

											$translations[ $key ][ $lang ] = $lang_value;
										}
									} else {
										$script_meta[ $key ] = $setting;
									}
								}

								$term_slug = sanitize_text_field( $script[ 'term_data' ][ 'term_slug' ] );
								$term_name = sanitize_text_field( $script[ 'term_data' ][ 'term_title' ] );
								$term      = get_term_by( 'slug', $term_slug, 'cs-cookies-category' );
								if ( !$term ) {
									$term_id = wp_insert_term( $term_name, 'cs-cookies-category', array(
										'slug' => $term_slug
									) );

									if ( !is_wp_error( $term_id ) ) {
										$cat_id = $term_id[ 'term_id' ] ?? 0;
									} else {
										$cat_id = 0;
									}
									$update_consent = true;
								} else {
									$cat_id = $term->term_id;
								}

								foreach ( $script_meta as $key => $meta ) {
									if ( $key == 'cs_ignore_this_category' ) {
										$old_value = get_term_meta( $cat_id, $key, true );
										if ( $old_value != $meta ) {
											$update_consent = true;
										}
									}

									update_term_meta( $cat_id, $key, $meta );
								}

								if ( !empty( $cat_id ) && !empty( $translations ) ) {
									foreach ( $translations as $settings ) {
										foreach ( $settings as $lang => $lang_value ) {
											ConsentMagic()->updateLangOptions( $term_slug, $lang_value, $lang, 'term' );
										}
									}
								}
							}
						}

						if ( $update_consent ) {
							$this->renew_consent_run();
						}

						break;

					case 'design' :
						if ( !empty( $settings_part[ 'design_templates' ] ) ) {
							foreach ( $settings_part[ 'design_templates' ] as $template ) {
								if ( !empty( $template ) ) {
									$template_meta = array();
									foreach ( $template[ 'post_meta' ] as $key => $setting ) {
										$key = sanitize_text_field( $key );
										if ( is_array( $setting ) ) {
											array_walk( $setting, array(
												ConsentMagic(),
												'cs_sanitize_array'
											) );
										} else {
											$setting = sanitize_text_field( $setting );
										}

										if ( $key == 'cs_logo' ) {
											$logo_id = '';
											if ( !empty( $setting ) ) {
												$logo_id = save_image( $setting, 'logo' );
											}
											$template_meta[ $key ] = $logo_id;
										} else {
											$template_meta[ $key ] = $setting;
										}
									}

									$post_name = sanitize_text_field( $template[ 'post_data' ][ 'post_name' ] );

									if ( $post = get_page_by_path( $post_name, OBJECT, CMPRO_TEMPLATE_POST_TYPE ) ) {
										$template_data = array(
											'ID'         => $post->ID,
											'meta_input' => $template_meta
										);

										wp_update_post( $template_data );
									} else {
										$template_data = array(
											'post_type'   => CMPRO_TEMPLATE_POST_TYPE,
											'post_title'  => sanitize_text_field( $template[ 'post_data' ][ 'post_title' ] ),
											'post_name'   => $post_name,
											'post_status' => 'publish',
											'ping_status' => 'closed',
											'post_author' => 1,
											'meta_input'  => $template_meta
										);

										wp_insert_post( $template_data );
									}
								}
							}
						}

						foreach ( $settings_part[ 'adjust_sizes' ] as $key => $setting ) {
							$key = sanitize_text_field( $key );
							ConsentMagic()->updateOptionsDesign( array( $key => sanitize_text_field( $setting ) ) );
						}

						break;

					case 'custom_scripts' :
						if ( !empty( $settings_part ) ) {
							foreach ( $settings_part as $script ) {
								$script_meta = array();
								foreach ( $script[ 'script_meta' ] as $key => $setting ) {
									$key = sanitize_text_field( $key );
									if ( is_array( $setting ) ) {
										array_walk( $setting, array(
											ConsentMagic(),
											'cs_sanitize_array'
										) );
									} else {
										$setting = sanitize_text_field( $setting );
									}

									$script_meta[ $key ] = $setting;
								}

								$script_slug  = sanitize_text_field( $script[ 'script_data' ][ 'term_slug' ] );
								$script_name  = sanitize_text_field( $script[ 'script_data' ][ 'script_name' ] );
								$script_title = sanitize_text_field( $script[ 'script_data' ][ 'script_title' ] );
								$term         = get_term_by( 'slug', $script_slug, 'cs-cookies-category' );
								if ( !$term ) {
									$term = get_term_by( 'slug', 'necessary', 'cs-cookies-category' );
								}

								if ( $post = get_page_by_path( $script_name, OBJECT, CMPRO_POST_TYPE_SCRIPTS ) ) {
									$script_data = array(
										'ID'         => $post->ID,
										'meta_input' => $script_meta
									);
									$post_id     = $post->ID;

									wp_update_post( $script_data );
								} else {
									$script_data = array(
										'post_type'   => CMPRO_POST_TYPE_SCRIPTS,
										'post_title'  => $script_title,
										'post_name'   => $script_name,
										'post_status' => 'publish',
										'ping_status' => 'closed',
										'post_author' => 1,
										'meta_input'  => $script_meta
									);

									$post_id = wp_insert_post( $script_data );
									if ( is_wp_error( $post_id ) ) {
										$post_id = null;
									}
								}

								wp_set_object_terms( $post_id, $term->term_id, 'cs-cookies-category' );
								ConsentMagic()->updateOptions( array(
									'cs_' . $script_title . '_' . $post_id
									. '_script_enable' => sanitize_text_field( $script[ 'script_data' ][ 'enabled' ] )
								) );
								ConsentMagic()->updateOptions( array(
									'cs_' . $post_id . '_script_cat' => $term->slug
								) );
							}
						}
						break;

					default :
						break;
				}
			}

			wp_send_json_success( __( 'Import completed', 'consent-magic' ) );
		} else {
			wp_send_json_error( __( 'File not found', 'consent-magic' ) );
		}

		wp_die();
	}

	private function add_preview_styles() {
		$style_url = CMPRO_PLUGIN_URL . "assets/css/style-public.min.css?ver=" . CMPRO_LATEST_VERSION_NUMBER;
		?>
        <script>
			(
				function () {
					if ( !document.querySelector( 'link[href="<?php echo esc_url( $style_url ); ?>"]' ) ) {
						let link = document.createElement( 'link' );
						link.rel = 'stylesheet';
						link.href = "<?php echo esc_url( $style_url ); ?>";
						document.head.appendChild( link );
					}
				}
			)();
        </script>
		<?php
	}

	public function manageCSPermissionsAjax() {
		// Lock out non-admins:
		if ( !current_user_can( 'manage_cs' ) ) {
			wp_send_json_error();
			wp_die( esc_html__( 'You do not have sufficient permission to perform this operation', 'consent-magic' ) );
		}
	}
}
