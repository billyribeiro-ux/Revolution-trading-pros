<?php

namespace ConsentMagicPro;

// If this file is called directly, abort.
if ( !defined( 'WPINC' ) ) {
	die;
}

use ConsentMagicPro\GuzzleHttp\Cookie\CookieJar;

//`Coming Soon Page & Maintenance Mode by SeedProd` is active then disable script blocker
if ( class_exists( 'SEED_CSP4' ) ) {
	$seed_csp4_option = get_option( 'seed_csp4_settings_content' );
	if ( $seed_csp4_option && $seed_csp4_option[ 'status' ] > 0 ) {
		return;
	}
}

class CS_Scanner_Module {

	/**
	 * Table name for the san history table.
	 * @var string
	 */
	public string $scan_table = 'cs_scan';

	/**
	 * Table name for the scanned cookies table.
	 * @var string
	 */
	public string $cookies_table = 'cs_scan_cookies';

	/**
	 * Table name for the scanned scripts table.
	 * @var string
	 */
	public string $scripts_table = 'cs_scan_scripts';

	public array $status_labels;

	public $last_scan_data;

	protected ?bool $tables_ready = null; // Indicates if scanner tables exist; set after successful check_tables() to avoid duplicate SHOW TABLES

	public function __construct() {

		$this->status_labels = array(
			1 => 'Completed',
			2 => 'Failed',
		);
		// Cache scanner tables status to avoid duplicate checks
		$this->tables_ready = (bool) ConsentMagic()->getOption( 'cs_check_scanner_tables' );

		if ( is_admin() ) {
			add_action( 'init', array(
				$this,
				'init'
			) );
		}

		/* creating necessary table for script blocker  */
		register_activation_hook( CMPRO_PLUGIN_FILENAME, array(
			$this,
			'cs_activator'
		) );

		if ( is_plugin_activated() ) {
			$this->cs_frontend_module();

			// hook that function onto our scheduled event:
			add_action( 'cs_cron_bulck_scan_hook', array(
				$this,
				'cs_cron_bulck_scan'
			), 200 );
			add_action( 'init', function() {
				add_filter( 'cron_schedules', array(
					$this,
					'custom_cron_job_recurrence'
				), 200 );
			} );
		}
	}

	public function init(): void {

		require_once ABSPATH . 'wp-admin/includes/upgrade.php';

		$this->tables_ready = ConsentMagic()->getOption( 'cs_check_scanner_tables' );

		if ( !$this->tables_ready ) {
			if ( $this->check_tables() ) {
				$this->install_tables();
			}
		}

		$params = array(
			'nonces'      => array(
				'cs_scanner' => wp_create_nonce( 'cs_scanner' ),
			),
			'ajax_url'    => admin_url( 'admin-ajax.php' ),
			'scan_status' => (int) $this->check_scan_status() === 1,
			'labels'      => array(
				'scanned'             => esc_html__( 'Scanned', 'consent-magic' ),
				'finished'            => esc_html__( 'Scanning completed.', 'consent-magic' ),
				'scanning'            => esc_html__( 'Scanning pages...', 'consent-magic' ),
				'error'               => esc_html__( 'Error', 'consent-magic' ),
				'refreshing'          => esc_html__( 'Refreshing....', 'consent-magic' ),
				'reload_page'         => esc_html__( 'Error !!! Please reload the page to see cookie list.', 'consent-magic' ),
				'success'             => esc_html__( 'Success', 'consent-magic' ),
				'thankyou'            => esc_html__( 'Thank you', 'consent-magic' ),
				'total_urls_scanned'  => esc_html__( 'Total URLs scanned', 'consent-magic' ),
				'total_cookies_found' => esc_html__( 'Total Cookies found', 'consent-magic' ),
				'page_fetch_error'    => esc_html__( 'Could not fetch the URLs, please try again', 'consent-magic' ),
			),
		);

		wp_enqueue_script( 'cs_scanner', plugin_dir_url( __FILE__ )
		                                 . 'js/scanner.js', array(), CMPRO_LATEST_VERSION_NUMBER, true );
		wp_localize_script( 'cs_scanner', 'cs_scanner', $params );
	}

	public function cs_cron_bulck_scan(): void {

		$cs_scanner_module     = new CS_Scanner();
		$cookies_pattern       = $this->get_cookies_pattern();
		$cs_scan_existing_page = ConsentMagic()->getOption( 'cs_auto_scan_type' );
		$total                 = 30;

		if ( $cs_scan_existing_page === 'scan_all_pages' ) {
			$total = $cs_scanner_module->get_scan_all_pages()[ 'total' ];
		}

		$scan_id     = $this->create_scan_entry( $total );
		$page_limit  = $total;
		$page_offset = 0;
		$urls        = $cs_scanner_module->cs_get_all_url( $page_limit, $page_offset );

		$needle      = $cs_scanner_module->cs_scripts_pattern();
		$needle      = $cs_scanner_module->get_custom_scripts_list( $needle );
		$matches     = array();
		$cookie_data = array();
		$script_data = array();

		global $wpdb;
		$script_table = $wpdb->prefix . $this->scripts_table;
		$update       = $wpdb->prepare( 'UPDATE `%1$s` SET `script_enabled` = 0', $script_table );
		$wpdb->query( $update );

		$cookie_table = $wpdb->prefix . $this->cookies_table;
		$update       = $wpdb->prepare( 'UPDATE `%1$s` SET `cookie_enabled` = 0', $cookie_table );
		$wpdb->query( $update );

		if ( isPYSActivated() ) {
			$pys = $cs_scanner_module->cs_get_pys_pattent();

			foreach ( $pys as $key => $val ) {

				switch ( $key ) {
					case 'pys_pinterest' :
						$configured_pixel = isPinterestActive() == '1' && function_exists( 'PixelYourSite\Pinterest' )
						                    && \PixelYourSite\Pinterest()->configured();
						break;
					case 'pys_bing' :
						$configured_pixel = isBingActive() == '1' && function_exists( 'PixelYourSite\Bing' )
						                    && \PixelYourSite\Bing()->configured();
						break;
					case 'pys_tiktok' :
						$configured_pixel = function_exists( 'PixelYourSite\Tiktok' )
						                    && \PixelYourSite\Tiktok()->configured();
						break;
					case 'pys_facebook' :
						$configured_pixel = function_exists( 'PixelYourSite\Facebook' )
						                    && \PixelYourSite\Facebook()->configured();
						break;
					case 'pys_ga' :
						$configured_pixel = function_exists( 'PixelYourSite\GA' ) && \PixelYourSite\GA()->configured();
						break;
					case 'pys_google_ads' :
						$configured_pixel = function_exists( 'PixelYourSite\Ads' ) && \PixelYourSite\Ads()->configured();
						break;
					default :
						$configured_pixel = false;
				}

				if ( $configured_pixel ) {
					$term          = get_term_by( 'slug', ConsentMagic()->getOption( $val[ 'parent_cat_key' ] ), 'cs-cookies-category' );
					$script_data[] = array(
						'script_name'    => $val[ 'script_name' ],
						'script_slug'    => $val[ 'option_key' ],
						'category_id'    => $term->term_id,
						'script_body'    => 'custom',
						'description'    => $val[ 'description' ],
						'parent_cat_key' => $val[ 'parent_cat_key' ],
						'script_enabled' => 1,
					);
					foreach ( $cookies_pattern as $ckey => $cval ) {
						if ( $val[ 'parent_script' ] === $cval[ 'parent_script' ] ) {
							$cookie_data[] = array(
								'cookie_name'    => $ckey,
								'value'          => '',
								'domain'         => '',
								'expires'        => $cval[ 'expiry' ],
								'max-age'        => '',
								'path'           => '',
								'secure'         => '',
								'discard'        => '',
								'description'    => '',
								'category'       => '',
								'cookie_enabled' => 1,
							);
						}
					}
				}
			}
		}

		if ( !empty( $urls ) ) {
			foreach ( $urls as $url ) {
				$response = wp_remote_get( $url );

				if ( is_wp_error( $response ) ) {
					error_log( print_r( $response->get_error_message(), true ) );
				} elseif ( wp_remote_retrieve_response_code( $response ) === 200 ) {

					$html_all = wp_remote_retrieve_body( $response );
					preg_match_all( '#<script(.*?)<\/script>#is', $html_all, $matches_data );
					preg_match_all( '/<iframe.*src=\"(.*)\".*><\/iframe>/isU', $html_all, $matches_data_iframe );
					preg_match_all( '/<link.*fonts\.googleapis\.com\/css.*?[\/]?>/', $html_all, $matches_fonts );
					if ( !empty( $matches_fonts ) ) {
						$matches = array_merge( $matches, $matches_data[ 0 ] );
					}
					if ( !empty( $matches_data ) ) {
						$matches = array_merge( $matches, $matches_fonts[ 0 ] );
					}
					if ( !empty( $matches_data_iframe ) ) {
						$matches = array_merge( $matches, $matches_data_iframe[ 0 ] );
					}

					if ( !empty( $_COOKIE ) ) {
						$jar = CookieJar::fromArray( $_COOKIE, $url );
						foreach ( $_COOKIE as $key => $val ) {
							$cookie = $jar->getCookieByName( $key );
							if ( $cookie ) {
								$cookie_data[] = array(
									'cookie_name'    => sanitize_text_field( $cookie->getName() ),
									'value'          => sanitize_text_field( $cookie->getValue() ),
									'domain'         => sanitize_text_field( $cookie->getDomain() ),
									'expires'        => sanitize_text_field( $cookie->getExpires() ),
									'max-age'        => sanitize_text_field( $cookie->getMaxAge() ),
									'path'           => sanitize_text_field( $cookie->getPath() ),
									'secure'         => sanitize_text_field( $cookie->getSecure() ),
									'discard'        => sanitize_text_field( $cookie->getDiscard() ),
									'description'    => '',
									'category'       => '',
									'cookie_enabled' => 1,
								);
							}
						}
					}
				}
			}
			if ( !empty( $matches ) ) {
				$matches = array_unique( $matches );
				foreach ( $matches as $value ) {
					foreach ( $needle as $key => $val ) {
						if ( !is_array( $val[ 'data' ] ) ) {
							$check = strripos( $value, $val[ 'data' ] );
							if ( $check !== false ) {
								$option_key = isset( $val[ 'option_key' ] ) ? 'cs_block_' . $val[ 'option_key' ]
								                                              . '_scripts_cat' : $val[ 'option_name' ];
								$term       = get_term_by( 'slug', ConsentMagic()->getOption( $option_key ), 'cs-cookies-category' );
								if ( !$term ) {
									$option_key = isset( $val[ 'script_id' ] ) ? 'cs_' . $val[ 'script_id' ]
									                                             . '_script_cat' : $val[ 'option_key' ];
									$term       = get_term_by( 'slug', ConsentMagic()->getOption( $option_key ), 'cs-cookies-category' );
								}
								if ( isset( $val[ 'script_custom' ] ) ) {
									$script_data[] = array(
										'script_name'    => $key,
										'script_slug'    => $val[ 'option_key' ],
										'script_body'    => 'custom',
										'category_id'    => $term->term_id,
										'description'    => $val[ 'description' ],
										'parent_cat_key' => $val[ 'parent_cat_key' ],
										'script_enabled' => 1,
									);
								} else {
									$script_data[] = array(
										'script_name'    => $key,
										'script_slug'    => $val[ 'option_key' ],
										'script_body'    => $value,
										'category_id'    => $term->term_id,
										'description'    => $val[ 'description' ],
										'parent_cat_key' => $val[ 'parent_cat_key' ],
										'script_enabled' => 1,
									);
								}
								foreach ( $cookies_pattern as $ckey => $cval ) {
									if ( $val[ 'parent_cat_key' ] == $cval[ 'parent_cat_key' ] ) {
										$cookie_data[] = array(
											'cookie_name'    => $ckey,
											'value'          => '',
											'domain'         => '',
											'expires'        => $cval[ 'expiry' ],
											'max-age'        => '',
											'path'           => '',
											'secure'         => '',
											'discard'        => '',
											'description'    => '',
											'category'       => '',
											'cookie_enabled' => 1,
										);
									}
								}
							}
						} else {
							foreach ( $val[ 'data' ] as $sub_val ) {
								$check = strripos( $value, $sub_val );
								if ( $check !== false ) {
									$option_key = isset( $val[ 'option_key' ] ) ? 'cs_block_' . $val[ 'option_key' ]
									                                              . '_scripts_cat' : $val[ 'option_name' ];
									$term       = get_term_by( 'slug', ConsentMagic()->getOption( $option_key ), 'cs-cookies-category' );
									if ( !$term ) {
										$option_key = isset( $val[ 'script_id' ] ) ? 'cs_' . $val[ 'script_id' ]
										                                             . '_script_cat' : $val[ 'option_key' ];
										$term       = get_term_by( 'slug', ConsentMagic()->getOption( $option_key ), 'cs-cookies-category' );
									}
									if ( isset( $val[ 'script_custom' ] ) ) {
										$script_data[] = array(
											'script_name'    => $key,
											'script_slug'    => $val[ 'option_key' ],
											'script_body'    => 'custom',
											'category_id'    => $term->term_id,
											'description'    => $val[ 'description' ],
											'parent_cat_key' => $val[ 'parent_cat_key' ],
											'script_enabled' => 1,
										);
									} else {
										$script_data[] = array(
											'script_name'    => $key,
											'script_slug'    => $val[ 'option_key' ],
											'script_body'    => $value,
											'category_id'    => $term->term_id,
											'description'    => $val[ 'description' ],
											'parent_cat_key' => $val[ 'parent_cat_key' ],
											'script_enabled' => 1,
										);
									}
									foreach ( $cookies_pattern as $ckey => $cval ) {
										if ( $val[ 'parent_cat_key' ] == $cval[ 'parent_cat_key' ] ) {
											$cookie_data[] = array(
												'cookie_name'    => $ckey,
												'value'          => '',
												'domain'         => '',
												'expires'        => $cval[ 'expiry' ],
												'max-age'        => '',
												'path'           => '',
												'secure'         => '',
												'discard'        => '',
												'description'    => '',
												'category'       => '',
												'cookie_enabled' => 1,
											);
										}
									}
								}
							}
						}
					}
				}
			}

			$cookie_data = $cs_scanner_module->unique_multidim_array( $cookie_data, 'cookie_name' );
		}

		$old_cookie  = $this->get_scan_cookies( 0, 0, false );
		$bind_data   = array(
			'description',
			'category'
		);
		$cookie_data = $cs_scanner_module->bind_data( $old_cookie, $cookie_data, $bind_data, 'cookie_name' );
		$cs_scanner_module->save_cookie_data( $cookie_data );

		$cs_scanner_module->save_script_data( $script_data );


		$data_arr = array(
			'current_action' => 'cs_bulk_scan',
			'current_offset' => -1,
			'status'         => 1,
		);

		$this->update_scan_entry( $data_arr, $scan_id );

		if ( ConsentMagic()->getOption( 'cs_auto_scan_email_enabled' ) == 1 ) {
			$last_scan = $this->get_last_scan();
			if ( $last_scan ) {
				$email = sanitize_email( ConsentMagic()->getOption( 'cs_auto_scan_email' ) );
				if ( !empty( $email ) ) {
					$subject = esc_html__( 'New auto-scan is completed', 'consent-magic' ) . ' - '
					           . current_time( 'mysql' );
					$headers = array( "From: <$email>", "Content-Type: text/html; charset=UTF-8" );
					$message = '<p>' . esc_html__( 'Hi,', 'consent-magic' ) . '</p>
<p>' . esc_html__( 'ConsentMagic performed a new auto-scan on ', 'consent-magic' ) . get_site_url() . '</p>
<p>' . esc_html__( 'Detected cookies ', 'consent-magic' ) . $last_scan[ 'total_cookies' ] . ', '
					           . esc_html__( 'detected scripts ', 'consent-magic' ) . $last_scan[ 'total_scripts' ]
					           . ', ' . esc_html__( 'total scanned URLs ', 'consent-magic' ) . $last_scan[ 'total_url' ]
					           . '</p>
<p>'
					           . sprintf( esc_html__( 'For more details about your scans, %s go to your website %s', 'consent-magic' ), '<a href="'
					                                                                                                                    . admin_url( 'admin.php?page=consent-magic&tab=cs-script-blocking' )
					                                                                                                                    . '" target="_blank">', '</a>' )
					           . '</p>';
					$sent    = wp_mail( $email, $subject, $message, $headers );
					if ( $sent ) {
						error_log( 'message sent!' );
					} else {
						error_log( 'message not sent!' );
					}
				}
			}
		}

		wp_die();
	}

	public function cs_scan_cron_update(): bool {
		$cs_auto_scan_interval = ConsentMagic()->getOption( 'cs_auto_scan_interval' );
		if ( $cs_auto_scan_interval ) {
			if ( is_plugin_activated() ) {
				if ( !wp_next_scheduled( 'cs_cron_bulck_scan_hook' ) ) {
					if ( $cs_auto_scan_interval == "once_a_week" ) {
						wp_schedule_event( time(), 'weekly', 'cs_cron_bulck_scan_hook' );
					} else if ( $cs_auto_scan_interval == "once_a_month" ) {
						wp_schedule_event( time(), 'monthly', 'cs_cron_bulck_scan_hook' );
					} else {
						$this->cs_scan_cron_deactivate();
					}
				}
			}
		}

		return false;
	}

	// unschedule event upon plugin deactivation
	public function cs_scan_cron_deactivate(): void {
		if ( wp_next_scheduled( 'cs_cron_bulck_scan_hook' ) ) {
			wp_clear_scheduled_hook( 'cs_cron_bulck_scan_hook' );
		}
	}

	// Custom Cron Recurrences
	public function custom_cron_job_recurrence( $schedules ): array {
		$schedules[ 'monthly' ] = array(
			'display'  => esc_html__( 'Once a month', 'consent-magic' ),
			'interval' => 2635200,
		);
		$schedules[ 'weekly' ]  = array(
			'display'  => esc_html__( 'Once a week', 'consent-magic' ),
			'interval' => 604800,
		);

		return $schedules;
	}

	public function cs_activator(): void {
		global $wpdb;

		require_once ABSPATH . 'wp-admin/includes/upgrade.php';

		if ( is_multisite() ) {
			$blog_ids = $wpdb->get_col( "SELECT blog_id FROM $wpdb->blogs" ); // phpcs:ignore WordPress.DB.DirectDatabaseQuery,WordPress.DB.PreparedSQL.NotPrepared
			foreach ( $blog_ids as $blog_id ) {
				switch_to_blog( $blog_id );
				$this->install_tables();
				restore_current_blog();
			}
		} else {
			$this->install_tables();
		}
	}

	public function cs_frontend_module(): void {
		$cs_auto_scan_interval = ConsentMagic()->getOption( 'cs_auto_scan_interval' );
		if ( $cs_auto_scan_interval !== 'never' ) {
			$this->cs_scan_cron_update();
		} else {
			$this->cs_scan_cron_deactivate();
		}
		require_once plugin_dir_path( __FILE__ ) . 'CS_Scanner.php';
	}

	/**
	 * Check whether table exist or not
	 * @param string $table_name table name.
	 * @return bool
	 */
	public function table_exists( string $table_name ): bool {
		global $wpdb;
		if ( $wpdb->get_var( $wpdb->prepare( 'SHOW TABLES LIKE %s', $wpdb->esc_like( $table_name ) ) )
		     === $table_name ) {
			return true;
		}

		return false;
	}

	/**
	 * Install necessary tables
	 * @return void
	 */
	public function install_tables(): void {
		global $wpdb;

		$charset_collate = $wpdb->get_charset_collate();

		$table_name = $wpdb->prefix . $this->scan_table;

		if ( false === $this->table_exists( $table_name ) ) {
			$create_table_sql = "CREATE TABLE `$table_name`(
			    `id_cs_scan` INT NOT NULL AUTO_INCREMENT,
			    `status` INT NOT NULL DEFAULT '0',
			    `created_at` DATETIME NOT NULL DEFAULT '0000-00-00 00:00:00',
			    `total_url` INT NOT NULL DEFAULT '0',
			    `total_cookies` INT NOT NULL DEFAULT '0',
			    `total_scripts` INT NOT NULL DEFAULT '0',
			    `current_action` VARCHAR(50) NOT NULL,
			    `current_offset` INT NOT NULL DEFAULT '0',
			    PRIMARY KEY(`id_cs_scan`)
            ) $charset_collate;";

			dbDelta( $create_table_sql );
		}

		// Creates a table to store all the scanned cookies.
		$table_name = $wpdb->prefix . $this->cookies_table;

		if ( false === $this->table_exists( $table_name ) ) {
			$create_table_sql = "CREATE TABLE `$table_name`(
			    `id_cs_scan_cookies` INT NOT NULL AUTO_INCREMENT,
			    `id_cs_scan` INT NOT NULL DEFAULT '0',
			    `cookie_enabled` INT NOT NULL DEFAULT '0',
			    `cookie_name` VARCHAR(255) NOT NULL,
			    `cookie_parent_cat_key` VARCHAR(85) NOT NULL,
			    `expiry` VARCHAR(255) NOT NULL,
			    `type` VARCHAR(255) NOT NULL,
			    `category` VARCHAR(255) NOT NULL,
                `category_id` INT NOT NULL,
                `description` TEXT NULL DEFAULT '',
                `created_at` DATETIME NOT NULL DEFAULT '0000-00-00 00:00:00',
			    PRIMARY KEY(`id_cs_scan_cookies`),
			    UNIQUE `cookie` (`cookie_name`)
            )";

			$this->insert_scanner_tables( $create_table_sql, $charset_collate );
		}

		// Creates a table to store all the scanned scripts.
		$table_name = $wpdb->prefix . $this->scripts_table;

		if ( false === $this->table_exists( $table_name ) ) {
			$create_table_sql = "CREATE TABLE `$table_name`(
			    `id_cs_scan_scripts` INT NOT NULL AUTO_INCREMENT,
			    `id_cs_scan` INT NOT NULL DEFAULT '0',
			    `script_name` VARCHAR(85) NOT NULL,
			    `script_parent_cat_key` VARCHAR(85) NOT NULL,
                `category_id` INT NOT NULL,
                `script_body` VARCHAR(255) NOT NULL,
                `description` TEXT NULL DEFAULT '',
                `created_at` DATETIME NOT NULL DEFAULT '0000-00-00 00:00:00',
			    PRIMARY KEY(`id_cs_scan_scripts`),
			    UNIQUE `scripts` (`script_name`)
            )";

			$this->insert_scanner_tables( $create_table_sql, $charset_collate );
		}
	}

	/**
	 * Recursive function to insert scanner tables no matter what error has occurred
	 * @param string  $sql    sql query.
	 * @param string  $prop   property value.
	 * @param integer $status current status fail or success.
	 * @return boolean
	 */
	private function insert_scanner_tables( $sql, $prop = '', $status = 0 ) {

		global $wpdb;
		dbDelta( $sql . ' ' . $prop );
		if ( $wpdb->last_error ) {
			$status++;
			if ( 1 === $status ) {
				$prop = '';
			} elseif ( 2 === $status ) {
				$prop = 'ENGINE = MyISAM CHARACTER SET utf8 COLLATE utf8_general_ci';
			} else {

				return true;
			}
			$this->insert_scanner_tables( $sql, $prop, $status );
		} else {

			return true;
		}
	}

	/**
	 * Check if all the tables are inserted
	 * @return bool
	 */
	protected function check_tables(): bool {
		global $wpdb;

		$scanner_tables = array(
			$this->scan_table,
			$this->cookies_table,
			$this->scripts_table,
		);
		foreach ( $scanner_tables as $table ) {
			$table_name = $wpdb->prefix . $table;

			$like = '%' . $wpdb->esc_like( $table_name ) . '%';

			if ( !$wpdb->get_results( $wpdb->prepare( "SHOW TABLES LIKE %s", $like ), ARRAY_N ) ) { // phpcs:ignore WordPress.DB.DirectDatabaseQuery,WordPress.DB.PreparedSQL.NotPrepared
				return false;
			}
		}
		ConsentMagic()->updateOptions( array( 'cs_check_scanner_tables' => true ) );

		return true;
	}

	/**
	 * Insert a new scan entry to the scanner table
	 * @param array $data_arr Array of data.
	 * @param int   $scan_id  scan ID.
	 * @return bool
	 */
	protected function update_scan_entry( array $data_arr, int $scan_id ): bool {
		global $wpdb;
		$scan_table = $wpdb->prefix . $this->scan_table;
		if ( $wpdb->update( $scan_table, $data_arr, array( 'id_cs_scan' => $scan_id ) ) ) { // phpcs:ignore WordPress.DB.DirectDatabaseQuery,WordPress.DB.PreparedSQL.NotPrepared
			return true;
		} else {
			return false;
		}
	}

	/**
	 * Get current scan status text
	 * @param [type] $status current status of the scan.
	 * @return string
	 */
	public function get_scan_status_text( $status ): string {

		return isset( $this->status_labels[ $status ] ) ? $this->status_labels[ $status ] : esc_html__( 'Unknown', 'consent-magic' );
	}

	/**
	 * Return the last scan results
	 * @return ?array
	 */
	public function get_last_scan(): ?array {

		if ( $this->last_scan_data === null ) {
			global $wpdb;
			$scan_table = $wpdb->prefix . $this->scan_table;
			$data       = array();
			if ( $this->tables_ready ) {
				$data = $wpdb->get_row( $wpdb->prepare( 'SELECT * FROM `%1$s` ORDER BY id_cs_scan DESC LIMIT 1', $scan_table ), ARRAY_A ); // phpcs:ignore WordPress.DB.DirectDatabaseQuery,WordPress.DB.PreparedSQL.NotPrepared
			}

			return $data;
		} else {
			return $this->last_scan_data;
		}
	}

	/**
	 * Return the current scan status progress, failed , or success.
	 * @return integer
	 */
	public function check_scan_status(): int {
		$last_scan = $this->get_last_scan();
		$status    = $last_scan[ 'status' ] ?? 0;

		return intval( $status );
	}

	/**
	 * Return the identified cookies after the scanning
	 * @param [type]  $scan_id scan ID.
	 * @param integer $offset offset number.
	 * @param integer $limit  page limit if pagination is used.
	 * @return array
	 */
	public function get_scan_cookies( $offset = 0, $limit = 100, $only_enabled = true ): array {
		global $wpdb;

		$cookies_table = $wpdb->prefix . $this->cookies_table;

		$offset = $wpdb->prepare( '%d', $offset );
		$limit  = $wpdb->prepare( '%d', $limit );

		$sql     = $wpdb->prepare( 'SELECT * FROM %1$s ' . ( $only_enabled ? 'WHERE `cookie_enabled` = 1' : '' )
		                           . ' ORDER BY id_cs_scan_cookies DESC', $cookies_table ) . ( $limit
		                                                                                       > 0 ? " LIMIT $offset,$limit" : '' );
		$cookies = $wpdb->get_results( $sql, ARRAY_A ); // phpcs:ignore WordPress.DB.DirectDatabaseQuery,WordPress.DB.PreparedSQL.NotPrepared

		return $cookies;
	}

	/**
	 * Return the identified cookies after the scanning
	 * @param integer $offset offset number.
	 * @param integer $limit  page limit if pagination is used.
	 * @return array
	 */
	public function get_scan_scripts( int $offset = 0, int $limit = 100 ): array {
		global $wpdb;

		$scripts_table = $wpdb->prefix . $this->scripts_table;

		$offset = $wpdb->prepare( '%d', $offset );
		$limit  = $wpdb->prepare( '%d', $limit );

		$sql     = $wpdb->prepare( 'SELECT * FROM %1$s WHERE `script_enabled` = 1 ORDER BY id_cs_scan_scripts DESC', $scripts_table )
		           . ( $limit > 0 ? " LIMIT $offset,$limit" : '' );
		$scripts = $wpdb->get_results( $sql, ARRAY_A ); // phpcs:ignore WordPress.DB.DirectDatabaseQuery,WordPress.DB.PreparedSQL.NotPrepared

		return $scripts;
	}

	/**
	 * Return HTML table structure for listing cookies
	 * @param [type] $cookies The cookie list.
	 * @return string
	 */
	public function create_cookies_table( $cookies ): string {

		$count = 1;
		$html  = '<table class="cli-table">';
		$html  .= '<thead>';
		$html  .= '<th style="width: 6%;">' . esc_html__( 'Sl.No:', 'consent-magic' ) . '</th>';
		$html  .= '<th>' . esc_html__( 'Cookie Name', 'consent-magic' ) . '</th>';
		$html  .= '<th style="width:15%;" >' . esc_html__( 'Duration', 'consent-magic' ) . '</th>';
		$html  .= '<th style="width:15%;" >' . esc_html__( 'Category', 'consent-magic' ) . '</th>';
		$html  .= '<th style="width:40%;" >' . esc_html__( 'Description', 'consent-magic' ) . '</th>';
		$html  .= '</thead>';
		$html  .= '<tbody>';

		if ( isset( $cookies ) && is_array( $cookies ) && count( $cookies ) > 0 ) :
			foreach ( $cookies as $cookie ) :
				$html .= '<tr>';
				$html .= '<td>' . $count . '</td>';
				$html .= '<td>' . $cookie[ 'id' ] . '</td>';
				$html .= '<td>' . $cookie[ 'expiry' ] . '</td>';
				$html .= '<td>' . $cookie[ 'category' ] . '</td>';
				$html .= '<td>' . $cookie[ 'description' ] . '</td>';
				$html .= '</tr>';
				$count++;
			endforeach;
		else :
			$html .= '<tr><td class="colspanchange" colspan="5" style="text-align:center" >'
			         . esc_html__( 'Your cookie list is empty', 'consent-magic' ) . '</td></tr>';
		endif;

		$html .= '</tbody>';
		$html .= '</table>';

		return $html;
	}

	/**
	 * Get the last scan ID
	 * @return int
	 */
	public function get_last_scan_id() {
		$last_scan = $this->get_last_scan();
		$scan_id   = $last_scan[ 'id_cs_scan' ] ?? false;

		return $scan_id;
	}

	/**
	 * Get the last scan cookies count
	 * @param $scan_id
	 * @return int
	 */
	public function get_last_scan_new_cookies( $scan_id ): int {

		global $wpdb;

		$cookies_table = $wpdb->prefix . $this->cookies_table;

		$cookies = $wpdb->get_results( $wpdb->prepare( 'SELECT id_cs_scan_cookies FROM %1$s WHERE id_cs_scan = %2$s ORDER BY id_cs_scan_cookies ASC', $cookies_table, $scan_id ), ARRAY_A ); // phpcs:ignore WordPress.DB.DirectDatabaseQuery,WordPress.DB.PreparedSQL.NotPrepared

		return count( $cookies );
	}

	/**
	 * Get the last scan scripts count
	 * @param $scan_id
	 * @return int
	 */
	public function get_last_scan_new_scripts( $scan_id ): int {

		global $wpdb;

		$scripts_table = $wpdb->prefix . $this->scripts_table;

		$scripts = $wpdb->get_results( $wpdb->prepare( 'SELECT id_cs_scan_scripts FROM %1$s WHERE id_cs_scan = %2$s ORDER BY id_cs_scan_scripts ASC', $scripts_table, $scan_id ), ARRAY_A ); // phpcs:ignore WordPress.DB.DirectDatabaseQuery,WordPress.DB.PreparedSQL.NotPrepared

		return count( $scripts );
	}

	/**
	 * Insert a new scan entry to the table
	 * @param integer $total_url Total URL count.
	 * @return int
	 */
	protected function create_scan_entry( int $total_url = 0 ) {
		global $wpdb;

		$scan_table = $wpdb->prefix . $this->scan_table;
		$data_arr   = array(
			'created_at'     => gmdate( 'Y-m-d H:i:s' ),
			'total_url'      => absint( $total_url ),
			'total_cookies'  => 0,
			'total_scripts'  => 0,
			'current_action' => 'cs_get_pages',
			'status'         => 1,
		);
		if ( $wpdb->insert( $scan_table, $data_arr ) ) {
			return $wpdb->insert_id;
		} else {
			return '0';
		}
	}

	/**
	 * Insert the scanned Cookies to the corresponding table
	 * @param int   $scan_id     scan Id.
	 * @param array $cookie_data scanned cookies.
	 * @return void
	 */
	protected function insert_cookies( int $scan_id, array $cookie_data ): void {
		global $wpdb;
		$cookie_table   = $wpdb->prefix . $this->cookies_table;
		$sql            = "INSERT IGNORE INTO `$cookie_table` (`id_cs_scan`,`cookie_name`,`cookie_enabled`,`expiry`,`cookie_parent_cat_key`,`type`,`category`,`category_id`,`description`, `created_at`) VALUES ";
		$sql_arr        = array();
		$created_at     = gmdate( 'Y-m-d H:i:s' );
		$cookie_name    = isset( $cookie_data[ 'cookie_name' ] ) ? esc_sql( sanitize_text_field( $cookie_data[ 'cookie_name' ] ) ) : '';
		$description    = isset( $cookie_data[ 'description' ] ) ? esc_sql( sanitize_textarea_field( $cookie_data[ 'description' ] ) ) : '';
		$expiry         = isset( $cookie_data[ 'expires' ] ) ? esc_sql( sanitize_text_field( $cookie_data[ 'expires' ] ) ) : '';
		$cookie_enabled = isset( $cookie_data[ 'cookie_enabled' ] ) ? esc_sql( sanitize_text_field( $cookie_data[ 'cookie_enabled' ] ) ) : 1;
		$parent_cat_key = 'necessary';
		$type           = isset( $cookie_data[ 'type' ] ) ? esc_sql( sanitize_text_field( $cookie_data[ 'type' ] ) ) : '';
		$category       = !empty( $cookie_data[ 'category' ] ) ? $cookie_data[ 'category' ] : 'unassigned';
		$category_id    = get_term_by( 'slug', $category, 'cs-cookies-category' );
		if ( is_wp_error( $category_id ) || !$category_id ) {
			$category_id = get_term_by( 'slug', 'unassigned', 'cs-cookies-category' );
			$category    = 'unassigned';
		}
		$category_id             = $category_id->term_id;
		$google_analytics_cat    = ConsentMagic()->getOption( 'cs_block_google_analytics_scripts_cat' );
		$google_analytics_cat_id = get_term_by( 'slug', $google_analytics_cat, 'cs-cookies-category' )->term_id;

		$pys_patterns = $this->get_pys_cookies_patterns();
		$pys_cookie   = false;
		foreach ( $pys_patterns as $key => $pys_pattern ) {
			if ( str_starts_with( $cookie_name, $key ) ) {
				$pys_cookie = $key;
				break;
			}
		}

		if ( $pys_cookie !== false ) {
			$type           = $pys_patterns[ $pys_cookie ][ 'type' ];
			$description    = $pys_patterns[ $pys_cookie ][ 'description' ];
			$expiry         = $pys_patterns[ $pys_cookie ][ 'expiry' ];
			$category       = $pys_patterns[ $pys_cookie ][ 'category' ];
			$category_id    = $pys_patterns[ $pys_cookie ][ 'category_id' ]->term_id;
			$parent_cat_key = $pys_patterns[ $pys_cookie ][ 'parent_cat_key' ];
		} elseif ( stristr( $cookie_name, 'wordpress' ) !== false || stristr( $cookie_name, 'wp-' ) !== false
		           || stristr( $cookie_name, 'CS-' ) !== false
		           || stristr( $cookie_name, 'cs_' ) !== false
		           || stristr( $cookie_name, 'pys_' ) !== false
		           || stristr( $cookie_name, 'pys' ) !== false ) {
			$type        = 'necessary';
			$category_id = get_term_by( 'slug', 'necessary', 'cs-cookies-category' )->term_id;
			$category    = 'necessary';
			$description = !empty( $description ) ? $description : 'Default wordpress cookie. Always ON.';
		} else if ( stristr( $cookie_name, '_ga' ) !== false ) {
			$category_id = ( !empty( $category ) && $category != 'unassigned' ) ? $category : $google_analytics_cat_id;
			$category    = ( !empty( $category ) && $category != 'unassigned' ) ? $category : $google_analytics_cat;
		} else {
			$cookies_arr = $this->get_cookies_pattern();
			foreach ( $cookies_arr as $key => $val ) {
				if ( $cookie_name == $key ) {
					$type           = $val[ 'type' ];
					$description    = !empty( $description ) ? $description : $val[ 'description' ];
					$expiry         = $val[ 'expiry' ];
					$category       = ( !empty( $category )
					                    && $category != 'unassigned' ) ? $category : $val[ 'category' ];
					$category_id    = ( !empty( $category )
					                    && $category != 'unassigned' ) ? $category_id : $val[ 'category_id' ];
					$parent_cat_key = $val[ 'parent_cat_key' ];
				}
			}
		}

		$sql_arr[] = $wpdb->prepare( "(%s,%s,%s,%s,%s,%s,%s,%s,%s,%s)", $scan_id, $cookie_name, $cookie_enabled, $expiry, $parent_cat_key, $type, $category, $category_id, $description, $created_at );
		$sql       = $sql . implode( ',', $sql_arr )
		             . $wpdb->prepare( " ON DUPLICATE KEY UPDATE id_cs_scan = %s, cookie_name = %s, cookie_enabled = %d, expiry = %s, cookie_parent_cat_key = %s, type = %s, category = %s, category_id = %d, description = %s", $scan_id, $cookie_name, $cookie_enabled, $expiry, $parent_cat_key, $type, $category, $category_id, $description );
		$wpdb->query( $sql );
	}

	public function get_cookies_pattern(): array {
		$google_analytics_cat                = ConsentMagic()->getOption( 'cs_block_google_analytics_scripts_cat' );
		$google_analytics_cat_id             = get_term_by( 'slug', $google_analytics_cat, 'cs-cookies-category' );
		$google_google_ads_tag_cat           = ConsentMagic()->getOption( 'cs_block_google_ads_tag_scripts_cat' );
		$google_google_ads_tag_cat_id        = get_term_by( 'slug', $google_google_ads_tag_cat, 'cs-cookies-category' );
		$google_google_adsense_tag_cat       = ConsentMagic()->getOption( 'cs_block_google_adsense_scripts_cat' );
		$google_google_adsense_tag_cat_id    = get_term_by( 'slug', $google_google_adsense_tag_cat, 'cs-cookies-category' );
		$cs_block_fb_pixel_scripts_cat       = ConsentMagic()->getOption( 'cs_block_fb_pixel_scripts_cat' );
		$cs_block_fb_pixel_cat_id            = get_term_by( 'slug', $cs_block_fb_pixel_scripts_cat, 'cs-cookies-category' );
		$cs_block_big_tag_scripts_cat        = ConsentMagic()->getOption( 'cs_block_big_tag_scripts_cat' );
		$cs_block_big_tag_cat_id             = get_term_by( 'slug', $cs_block_big_tag_scripts_cat, 'cs-cookies-category' );
		$cs_block_pin_tag_scripts_cat        = ConsentMagic()->getOption( 'cs_block_pin_tag_scripts_cat' );
		$cs_block_pin_tag_cat_id             = get_term_by( 'slug', $cs_block_pin_tag_scripts_cat, 'cs-cookies-category' );
		$cs_block_yt_embedded_scripts_cat    = ConsentMagic()->getOption( 'cs_block_yt_embedded_scripts_cat' );
		$cs_block_yt_embedded_cat_id         = get_term_by( 'slug', $cs_block_yt_embedded_scripts_cat, 'cs-cookies-category' );
		$cs_block_vimeo_embedded_scripts_cat = ConsentMagic()->getOption( 'cs_block_vimeo_embedded_scripts_cat' );
		$cs_block_vimeo_embedded_cat_id      = get_term_by( 'slug', $cs_block_vimeo_embedded_scripts_cat, 'cs-cookies-category' );
		$cs_block_soundcloud_scripts_cat     = ConsentMagic()->getOption( 'cs_block_soundcloud_scripts_cat' );
		$cs_block_soundcloud_cat_id          = get_term_by( 'slug', $cs_block_soundcloud_scripts_cat, 'cs-cookies-category' );
		$cs_block_hubspot_scripts_cat        = ConsentMagic()->getOption( 'cs_block_hubspot_scripts_cat' );
		$cs_block_hubspot_cat_id             = get_term_by( 'slug', $cs_block_hubspot_scripts_cat, 'cs-cookies-category' );
		$cs_block_hotjar_scripts_cat         = ConsentMagic()->getOption( 'cs_block_hotjar_scripts_cat' );
		$cs_block_hotjar_cat_id              = get_term_by( 'slug', $cs_block_hotjar_scripts_cat, 'cs-cookies-category' );
		$cs_block_matomo_scripts_cat         = ConsentMagic()->getOption( 'cs_block_matomo_scripts_cat' );
		$cs_block_matomo_cat_id              = get_term_by( 'slug', $cs_block_matomo_scripts_cat, 'cs-cookies-category' );
		$cs_block_addthis_scripts_cat        = ConsentMagic()->getOption( 'cs_block_addthis_scripts_cat' );
		$cs_block_addthis_cat_id             = get_term_by( 'slug', $cs_block_addthis_scripts_cat, 'cs-cookies-category' );
		$cs_block_sharethis_scripts_cat      = ConsentMagic()->getOption( 'cs_block_sharethis_scripts_cat' );
		$cs_block_sharethis_cat_id           = get_term_by( 'slug', $cs_block_sharethis_scripts_cat, 'cs-cookies-category' );
		$cs_block_ln_tag_scripts_cat         = ConsentMagic()->getOption( 'cs_block_ln_tag_scripts_cat' );
		$cs_block_ln_tag_scripts_cat_id      = get_term_by( 'slug', $cs_block_ln_tag_scripts_cat, 'cs-cookies-category' );
		$cs_block_tiktok_scripts_cat         = ConsentMagic()->getOption( 'cs_block_tiktok_scripts_cat' );
		$cs_block_tiktok_scripts_cat_id      = get_term_by( 'slug', $cs_block_tiktok_scripts_cat, 'cs-cookies-category' );
		$cs_block_twitter_scripts_cat        = ConsentMagic()->getOption( 'cs_block_twitter_scripts_cat' );
		$cs_block_twitter_scripts_cat_id     = get_term_by( 'slug', $cs_block_twitter_scripts_cat, 'cs-cookies-category' );

		$cookies_arr = array(
			'_ga'                                       => array(
				'parent_script'  => 'google_analytics',
				'expiry'         => '2 year',
				'type'           => 'persistent',
				'description'    => 'Allows you to differentiate between users.',
				'category'       => $google_analytics_cat,
				'category_id'    => $google_analytics_cat_id,
				'parent_cat_key' => 'cs_block_google_analytics_scripts_cat'
			),
			'_gid'                                      => array(
				'parent_script'  => 'google_analytics',
				'expiry'         => '24 hours',
				'type'           => 'persistent',
				'description'    => 'Allows you to differentiate between users.',
				'category'       => $google_analytics_cat,
				'category_id'    => $google_analytics_cat_id,
				'parent_cat_key' => 'cs_block_google_analytics_scripts_cat'
			),
			'_gat'                                      => array(
				'parent_script'  => 'google_analytics',
				'expiry'         => '1 minute',
				'type'           => 'session',
				'description'    => 'Limits the frequency of requests.',
				'category'       => $google_analytics_cat,
				'category_id'    => $google_analytics_cat_id,
				'parent_cat_key' => 'cs_block_google_analytics_scripts_cat'
			),
			'AMP_TOKEN'                                 => array(
				'parent_script'  => 'google_analytics',
				'expiry'         => '1 year',
				'type'           => 'persistent',
				'description'    => 'Contains a token that can be used to get the Client-ID from the AMP service. Other possible values: disabling the function, active request, or an error in obtaining the Client-ID from the AMP service.',
				'category'       => $google_analytics_cat,
				'category_id'    => $google_analytics_cat_id,
				'parent_cat_key' => 'cs_block_google_analytics_scripts_cat'
			),
			'__utma'                                    => array(
				'parent_script'  => 'google_analytics',
				'expiry'         => '2 year',
				'type'           => 'persistent',
				'description'    => 'Allows you to distinguish between users and sessions. Thrown when the JavaScript library is executed, if there are no existing __utma cookies. Updated every time data is sent to Google Analytics.',
				'category'       => $google_analytics_cat,
				'category_id'    => $google_analytics_cat_id,
				'parent_cat_key' => 'cs_block_google_analytics_scripts_cat'
			),
			'__utmt'                                    => array(
				'parent_script'  => 'google_analytics',
				'expiry'         => '10 minute',
				'type'           => 'persistent',
				'description'    => 'Limits the frequency of requests.',
				'category'       => $google_analytics_cat,
				'category_id'    => $google_analytics_cat_id,
				'parent_cat_key' => 'cs_block_google_analytics_scripts_cat'
			),
			'__utmb'                                    => array(
				'parent_script'  => 'google_analytics',
				'expiry'         => '30 minute',
				'type'           => 'persistent',
				'description'    => 'Used to define new sessions / visits. Thrown when the JavaScript library is executed if there are no existing __utmb cookies. Updated every time data is sent to Google Analytics.',
				'category'       => $google_analytics_cat,
				'category_id'    => $google_analytics_cat_id,
				'parent_cat_key' => 'cs_block_google_analytics_scripts_cat'
			),
			'__utmc'                                    => array(
				'parent_script'  => 'google_analytics',
				'expiry'         => '0 minute',
				'type'           => 'session',
				'description'    => 'Not used in ga.js. Installed for interoperability with urchin.js. Previously worked in conjunction with the __utmb cookie to determine whether a user should start a new session or visit.',
				'category'       => $google_analytics_cat,
				'category_id'    => $google_analytics_cat_id,
				'parent_cat_key' => 'cs_block_google_analytics_scripts_cat'
			),
			'__utmz'                                    => array(
				'parent_script'  => 'google_analytics',
				'expiry'         => '6 months',
				'type'           => 'persistent',
				'description'    => 'Stores information about the source of traffic or campaign, allowing you to understand where the user came to the site from. Generated when the library is run and updated every time data is submitted to Google Analytics.',
				'category'       => $google_analytics_cat,
				'category_id'    => $google_analytics_cat_id,
				'parent_cat_key' => 'cs_block_google_analytics_scripts_cat'
			),
			'__utmv'                                    => array(
				'parent_script'  => 'google_analytics',
				'expiry'         => '2 year',
				'type'           => 'persistent',
				'description'    => 'Stores data about a visitor level custom variable. Thrown when a developer uses the _setCustomVar method with a visitor-level custom variable. Also used by the _setVar method, which is no longer supported. Updated every time data is sent to Google Analytics.',
				'category'       => $google_analytics_cat,
				'category_id'    => $google_analytics_cat_id,
				'parent_cat_key' => 'cs_block_google_analytics_scripts_cat'
			),
			'__utmx'                                    => array(
				'parent_script'  => 'google_analytics',
				'expiry'         => '18 months',
				'type'           => 'persistent',
				'description'    => 'Determines if the user took part in the experiment.',
				'category'       => $google_analytics_cat,
				'category_id'    => $google_analytics_cat_id,
				'parent_cat_key' => 'cs_block_google_analytics_scripts_cat'
			),
			'__utmxx'                                   => array(
				'parent_script'  => 'google_analytics',
				'expiry'         => '18 months',
				'type'           => 'persistent',
				'description'    => 'Determines when the experiment in which the user participated expires.',
				'category'       => $google_analytics_cat,
				'category_id'    => $google_analytics_cat_id,
				'parent_cat_key' => 'cs_block_google_analytics_scripts_cat'
			),
			'_gaexp'                                    => array(
				'parent_script'  => 'google_analytics',
				'expiry'         => '90 days',
				'type'           => 'persistent',
				'description'    => 'Determines when the experiment expires and whether the user participated in it.',
				'category'       => $google_analytics_cat,
				'category_id'    => $google_analytics_cat_id,
				'parent_cat_key' => 'cs_block_google_analytics_scripts_cat'
			),
			'_ga_'                                      => array(
				'parent_script'  => 'google_analytics',
				'expiry'         => '2 year',
				'type'           => 'persistent',
				'description'    => 'Allows to save session state.',
				'category'       => $google_analytics_cat,
				'category_id'    => $google_analytics_cat_id,
				'parent_cat_key' => 'cs_block_google_analytics_scripts_cat'
			),
			'_gac_gb_'                                  => array(
				'parent_script'  => 'google_analytics',
				'expiry'         => '90 days',
				'type'           => 'persistent',
				'description'    => 'Contains data related to the campaign. Once a link is established between your Analytics and Google Ads accounts, the Google Ads conversion tags hosted on your site will receive the data from the cookie, unless you disable this option.',
				'category'       => $google_analytics_cat,
				'category_id'    => $google_analytics_cat_id,
				'parent_cat_key' => 'cs_block_google_analytics_scripts_cat'
			),
			'_gac_'                                     => array(
				'parent_script'  => 'google_analytics',
				'expiry'         => '',
				'type'           => 'persistent',
				'description'    => 'Contains information about the campaign for the user. Once a link is established between your Analytics and Google Ads accounts, the Google Ads conversion tags hosted on your site will receive the data from the cookie, unless you disable this option.',
				'category'       => $google_analytics_cat,
				'category_id'    => $google_analytics_cat_id,
				'parent_cat_key' => 'cs_block_google_analytics_scripts_cat'
			),
			'_opt_awcid'                                => array(
				'parent_script'  => 'google_analytics',
				'expiry'         => '24 hours',
				'type'           => 'persistent',
				'description'    => 'Used for campaigns that are linked to Google Ads customer IDs.',
				'category'       => $google_analytics_cat,
				'category_id'    => $google_analytics_cat_id,
				'parent_cat_key' => 'cs_block_google_analytics_scripts_cat'
			),
			'_opt_awmid'                                => array(
				'parent_script'  => 'google_analytics',
				'expiry'         => '24 hours',
				'type'           => 'persistent',
				'description'    => 'Used for campaigns linked to Google Ads Campaign IDs.',
				'category'       => $google_analytics_cat,
				'category_id'    => $google_analytics_cat_id,
				'parent_cat_key' => 'cs_block_google_analytics_scripts_cat'
			),
			'_opt_awgid'                                => array(
				'parent_script'  => 'google_analytics',
				'expiry'         => '24 hours',
				'type'           => 'persistent',
				'description'    => 'Used for campaigns associated with Google Ads ad group IDs.',
				'category'       => $google_analytics_cat,
				'category_id'    => $google_analytics_cat_id,
				'parent_cat_key' => 'cs_block_google_analytics_scripts_cat'
			),
			'_opt_awkid'                                => array(
				'parent_script'  => 'google_analytics',
				'expiry'         => '24 hours',
				'type'           => 'persistent',
				'description'    => 'Used for campaigns linked to Google Ads Criteria IDs.',
				'category'       => $google_analytics_cat,
				'category_id'    => $google_analytics_cat_id,
				'parent_cat_key' => 'cs_block_google_analytics_scripts_cat'
			),
			'_opt_utmc'                                 => array(
				'parent_script'  => 'google_analytics',
				'expiry'         => '24 hours',
				'type'           => 'persistent',
				'description'    => 'Retains the last query parameter utm_campaign.',
				'category'       => $google_analytics_cat,
				'category_id'    => $google_analytics_cat_id,
				'parent_cat_key' => 'cs_block_google_analytics_scripts_cat'
			),
			'_opt_expid'                                => array(
				'parent_script'  => 'google_analytics',
				'expiry'         => '10 Seconds',
				'type'           => 'persistent',
				'description'    => 'Thrown when a redirect experiment is running. Stores the experiment ID, variation ID, and referral source to the redirected page.',
				'category'       => $google_analytics_cat,
				'category_id'    => $google_analytics_cat_id,
				'parent_cat_key' => 'cs_block_google_analytics_scripts_cat'
			),
			'utm_source'                                => array(
				'parent_script'  => 'google_analytics',
				'expiry'         => 'expires immediately',
				'type'           => 'Statistics',
				'description'    => 'to Provide parameters to URLs to identify the campaigns that refer traffic.',
				'category'       => $google_analytics_cat,
				'category_id'    => $google_analytics_cat_id,
				'parent_cat_key' => 'cs_block_google_analytics_scripts_cat'
			),
			'utm_campaign'                              => array(
				'parent_script'  => 'google_analytics',
				'expiry'         => 'expires immediately',
				'type'           => 'Statistics',
				'description'    => 'to Provide parameters to URLs to identify the campaigns that refer traffic.',
				'category'       => $google_analytics_cat,
				'category_id'    => $google_analytics_cat_id,
				'parent_cat_key' => 'cs_block_google_analytics_scripts_cat'
			),
			'_gat_gtag_UA_'                             => array(
				'parent_script'  => 'google_analytics',
				'expiry'         => '1 hour',
				'type'           => 'Marketing/Tracking',
				'description'    => 'to store and track conversions.',
				'category'       => $google_analytics_cat,
				'category_id'    => $google_analytics_cat_id,
				'parent_cat_key' => 'cs_block_google_analytics_scripts_cat'
			),
			'gtag_logged_in'                            => array(
				'parent_script'  => 'google_analytics',
				'expiry'         => '1 hour',
				'type'           => 'Marketing/Tracking',
				'description'    => '',
				'category'       => $google_analytics_cat,
				'category_id'    => $google_analytics_cat_id,
				'parent_cat_key' => 'cs_block_google_analytics_scripts_cat'
			),
			'__utm.gif '                                => array(
				'parent_script'  => 'google_analytics',
				'expiry'         => 'session',
				'type'           => 'Statistics',
				'description'    => 'to store browser details.',
				'category'       => $google_analytics_cat,
				'category_id'    => $google_analytics_cat_id,
				'parent_cat_key' => 'cs_block_google_analytics_scripts_cat'
			),
			'UTMD_'                                     => array(
				'parent_script'  => 'google_analytics',
				'expiry'         => 'various',
				'type'           => 'Statistics',
				'description'    => 'to store and count pageviews.',
				'category'       => $google_analytics_cat,
				'category_id'    => $google_analytics_cat_id,
				'parent_cat_key' => 'cs_block_google_analytics_scripts_cat'
			),
			'_dc_gtm_UA'                                => array(
				'parent_script'  => 'google_analytics',
				'expiry'         => '1 minute',
				'type'           => 'Statistics',
				'description'    => 'to store number of service requests.',
				'category'       => $google_analytics_cat,
				'category_id'    => $google_analytics_cat_id,
				'parent_cat_key' => 'cs_block_google_analytics_scripts_cat'
			),
			'_utm'                                      => array(
				'parent_script'  => 'google_analytics',
				'expiry'         => 'at least one session',
				'type'           => 'Statistics',
				'description'    => 'to store and track visits across websites.',
				'category'       => $google_analytics_cat,
				'category_id'    => $google_analytics_cat_id,
				'parent_cat_key' => 'cs_block_google_analytics_scripts_cat'
			),
			'_gac_UA'                                   => array(
				'parent_script'  => 'google_analytics',
				'expiry'         => '90 days',
				'type'           => 'Statistics',
				'description'    => 'to store and count pageviews.',
				'category'       => $google_analytics_cat,
				'category_id'    => $google_analytics_cat_id,
				'parent_cat_key' => 'cs_block_google_analytics_scripts_cat'
			),
			'__utmd'                                    => array(
				'parent_script'  => 'google_analytics',
				'expiry'         => '1 second',
				'type'           => 'Statistics',
				'description'    => 'to store and track visitor journeys through the site and classifies them into groups.',
				'category'       => $google_analytics_cat,
				'category_id'    => $google_analytics_cat_id,
				'parent_cat_key' => 'cs_block_google_analytics_scripts_cat'
			),
			'__gads'                                    => array(
				'parent_script'  => 'google_ads_tag',
				'expiry'         => '13 months',
				'type'           => 'Marketing/Tracking',
				'description'    => 'To provide ad delivery or retargeting.',
				'category'       => $google_google_ads_tag_cat,
				'category_id'    => $google_google_ads_tag_cat_id,
				'parent_cat_key' => 'cs_block_google_ads_tag_scripts_cat'
			),
			'google_adsense_settings'                   => array(
				'parent_script'  => 'google_ads_tag',
				'expiry'         => 'persistent',
				'type'           => 'Marketing/Tracking',
				'description'    => 'to provide ad delivery or retargeting.',
				'category'       => $google_google_ads_tag_cat,
				'category_id'    => $google_google_ads_tag_cat_id,
				'parent_cat_key' => 'cs_block_google_ads_tag_scripts_cat'
			),
			'AID'                                       => array(
				'parent_script'  => 'google_ads_tag',
				'expiry'         => '13 months EEA UK / 540 days elsewhere',
				'type'           => 'Analytics, Advertising',
				'description'    => '',
				'category'       => $google_google_ads_tag_cat,
				'category_id'    => $google_google_ads_tag_cat_id,
				'parent_cat_key' => 'cs_block_google_ads_tag_scripts_cat'
			),
			'TAID'                                      => array(
				'parent_script'  => 'google_ads_tag',
				'expiry'         => '14 days',
				'type'           => 'Analytics, Advertising',
				'description'    => '',
				'category'       => $google_google_ads_tag_cat,
				'category_id'    => $google_google_ads_tag_cat_id,
				'parent_cat_key' => 'cs_block_google_ads_tag_scripts_cat'
			),
			'GED_PLAYLIST_ACTIVITY'                     => array(
				'parent_script'  => 'google_adsense',
				'expiry'         => 'Session',
				'type'           => 'Marketing/Tracking',
				'description'    => 'Allow Google DoubleClick to track website usage and help serve the most relevant ads to you',
				'category'       => $google_google_adsense_tag_cat,
				'category_id'    => $google_google_adsense_tag_cat_id,
				'parent_cat_key' => 'cs_block_google_adsense_scripts_cat'
			),
			'google_pub_config '                        => array(
				'parent_script'  => 'google_adsense',
				'expiry'         => 'persistent',
				'type'           => 'Marketing/Tracking',
				'description'    => 'to provide ad delivery or retargeting.',
				'category'       => $google_google_adsense_tag_cat,
				'category_id'    => $google_google_adsense_tag_cat_id,
				'parent_cat_key' => 'cs_block_google_adsense_scripts_cat'
			),
			'google_ama_settings '                      => array(
				'parent_script'  => 'google_adsense',
				'expiry'         => 'persistent',
				'type'           => 'Marketing/Tracking',
				'description'    => 'to store performed actions on the website.',
				'category'       => $google_google_adsense_tag_cat,
				'category_id'    => $google_google_adsense_tag_cat_id,
				'parent_cat_key' => 'cs_block_google_adsense_scripts_cat'
			),
			'_gcl_au'                                   => array(
				'parent_script'  => 'google_adsense',
				'expiry'         => 'persistent',
				'type'           => 'Marketing/Tracking',
				'description'    => 'to store and track conversions.',
				'category'       => $google_google_adsense_tag_cat,
				'category_id'    => $google_google_adsense_tag_cat_id,
				'parent_cat_key' => 'cs_block_google_adsense_scripts_cat'
			),
			'NID'                                       => array(
				'parent_script'  => 'google_adsense',
				'expiry'         => '6 months',
				'type'           => 'Security, Analytics, Functionality, Advertising',
				'description'    => '',
				'category'       => $google_google_adsense_tag_cat,
				'category_id'    => $google_google_adsense_tag_cat_id,
				'parent_cat_key' => 'cs_block_google_adsense_scripts_cat'
			),
			'DSID'                                      => array(
				'parent_script'  => 'google_adsense',
				'expiry'         => '2 weeks',
				'type'           => 'Security, Functionality, Advertising',
				'description'    => '',
				'category'       => $google_google_adsense_tag_cat,
				'category_id'    => $google_google_adsense_tag_cat_id,
				'parent_cat_key' => 'cs_block_google_adsense_scripts_cat'
			),
			'test_cookie'                               => array(
				'parent_script'  => 'google_adsense',
				'expiry'         => '15 minutes',
				'type'           => 'Functionality',
				'description'    => '',
				'category'       => $google_google_adsense_tag_cat,
				'category_id'    => $google_google_adsense_tag_cat_id,
				'parent_cat_key' => 'cs_block_google_adsense_scripts_cat'
			),
			'id'                                        => array(
				'parent_script'  => 'google_adsense',
				'expiry'         => 'OPT_OUT: fixed expiration (year 2030/11/09), non-OPT_OUT: 13 months EEA UK / 24 months elsewhere',
				'type'           => 'Functionality, Advertising',
				'description'    => '',
				'category'       => $google_google_adsense_tag_cat,
				'category_id'    => $google_google_adsense_tag_cat_id,
				'parent_cat_key' => 'cs_block_google_adsense_scripts_cat'
			),
			'ACLK_DATA'                                 => array(
				'parent_script'  => 'google_adsense',
				'expiry'         => '5 minutes',
				'type'           => 'Advertising',
				'description'    => '',
				'category'       => $google_google_adsense_tag_cat,
				'category_id'    => $google_google_adsense_tag_cat_id,
				'parent_cat_key' => 'cs_block_google_adsense_scripts_cat'
			),
			'pm_sess'                                   => array(
				'parent_script'  => 'google_adsense',
				'expiry'         => '30 minutes',
				'type'           => 'Security, Functionality',
				'description'    => '',
				'category'       => $google_google_adsense_tag_cat,
				'category_id'    => $google_google_adsense_tag_cat_id,
				'parent_cat_key' => 'cs_block_google_adsense_scripts_cat'
			),
			'pm_sess_NNN'                               => array(
				'parent_script'  => 'google_adsense',
				'expiry'         => '30 minutes',
				'type'           => 'Security, Functionality',
				'description'    => 'doubleclick.net, google.com',
				'category'       => $google_google_adsense_tag_cat,
				'category_id'    => $google_google_adsense_tag_cat_id,
				'parent_cat_key' => 'cs_block_google_adsense_scripts_cat'
			),
			'aboutads_sessNNN'                          => array(
				'parent_script'  => 'google_adsense',
				'expiry'         => '30 minutes',
				'type'           => 'Security, Functionality',
				'description'    => 'doubleclick.net, google.com',
				'category'       => $google_google_adsense_tag_cat,
				'category_id'    => $google_google_adsense_tag_cat_id,
				'parent_cat_key' => 'cs_block_google_adsense_scripts_cat'
			),
			'FPAU'                                      => array(
				'parent_script'  => 'google_adsense',
				'expiry'         => '90 days',
				'type'           => 'Analytics, Advertising',
				'description'    => 'Set from partner domain',
				'category'       => $google_google_adsense_tag_cat,
				'category_id'    => $google_google_adsense_tag_cat_id,
				'parent_cat_key' => 'cs_block_google_adsense_scripts_cat'
			),
			'ANID'                                      => array(
				'parent_script'  => 'google_adsense',
				'expiry'         => '13 months EEA UK / 24 months elsewhere',
				'type'           => 'Advertising',
				'description'    => 'google.com and local variations, e.g. google.de',
				'category'       => $google_google_adsense_tag_cat,
				'category_id'    => $google_google_adsense_tag_cat_id,
				'parent_cat_key' => 'cs_block_google_adsense_scripts_cat'
			),
			'RUL'                                       => array(
				'parent_script'  => 'google_adsense',
				'expiry'         => '12 months',
				'type'           => 'Advertising',
				'description'    => 'doubleclick.net',
				'category'       => $google_google_adsense_tag_cat,
				'category_id'    => $google_google_adsense_tag_cat_id,
				'parent_cat_key' => 'cs_block_google_adsense_scripts_cat'
			),
			'FPGCLAW'                                   => array(
				'parent_script'  => 'google_adsense',
				'expiry'         => '90 days',
				'type'           => 'Analytics, Advertising',
				'description'    => 'Set from partner domain',
				'category'       => $google_google_adsense_tag_cat,
				'category_id'    => $google_google_adsense_tag_cat_id,
				'parent_cat_key' => 'cs_block_google_adsense_scripts_cat'
			),
			'FPGCLGB'                                   => array(
				'parent_script'  => 'google_adsense',
				'expiry'         => '90 days',
				'type'           => 'Analytics, Advertising',
				'description'    => 'Set from partner domain',
				'category'       => $google_google_adsense_tag_cat,
				'category_id'    => $google_google_adsense_tag_cat_id,
				'parent_cat_key' => 'cs_block_google_adsense_scripts_cat'
			),
			'_gcl_gb'                                   => array(
				'parent_script'  => 'google_adsense',
				'expiry'         => '90 days',
				'type'           => 'Analytics, Advertising',
				'description'    => 'Set from partner domain',
				'category'       => $google_google_adsense_tag_cat,
				'category_id'    => $google_google_adsense_tag_cat_id,
				'parent_cat_key' => 'cs_block_google_adsense_scripts_cat'
			),
			'_gcl_aw'                                   => array(
				'parent_script'  => 'google_adsense',
				'expiry'         => '90 days',
				'type'           => 'Analytics, Advertising',
				'description'    => 'Set from partner domain',
				'category'       => $google_google_adsense_tag_cat,
				'category_id'    => $google_google_adsense_tag_cat_id,
				'parent_cat_key' => 'cs_block_google_adsense_scripts_cat'
			),
			'1P_JAR'                                    => array(
				'parent_script'  => 'google_adsense',
				'expiry'         => '30 days',
				'type'           => 'Advertising',
				'description'    => 'google.com and local variations, e.g. google.de',
				'category'       => $google_google_adsense_tag_cat,
				'category_id'    => $google_google_adsense_tag_cat_id,
				'parent_cat_key' => 'cs_block_google_adsense_scripts_cat'
			),
			'Conversion'                                => array(
				'parent_script'  => 'google_adsense',
				'expiry'         => '90 days',
				'type'           => 'Advertising',
				'description'    => 'www.googleadservices.com/pagead/conversion/',
				'category'       => $google_google_adsense_tag_cat,
				'category_id'    => $google_google_adsense_tag_cat_id,
				'parent_cat_key' => 'cs_block_google_adsense_scripts_cat'
			),
			'VISITOR_INFO1_LIVE__k'                     => array(
				'parent_script'  => 'google_adsense',
				'expiry'         => '180 days',
				'type'           => 'Security, Advertising',
				'description'    => 'youtube.com',
				'category'       => $google_google_adsense_tag_cat,
				'category_id'    => $google_google_adsense_tag_cat_id,
				'parent_cat_key' => 'cs_block_google_adsense_scripts_cat'
			),
			'VISITOR_INFO1_LIVE__default'               => array(
				'parent_script'  => 'google_adsense',
				'expiry'         => '180 days',
				'type'           => 'Security, Advertising',
				'description'    => 'youtube.com',
				'category'       => $google_google_adsense_tag_cat,
				'category_id'    => $google_google_adsense_tag_cat_id,
				'parent_cat_key' => 'cs_block_google_adsense_scripts_cat'
			),
			'_js_datr'                                  => array(
				'parent_script'  => 'fb_pixel',
				'expiry'         => '2 years',
				'type'           => 'Preferences',
				'description'    => 'to store user preferences',
				'category'       => $cs_block_fb_pixel_scripts_cat,
				'category_id'    => $cs_block_fb_pixel_cat_id,
				'parent_cat_key' => 'cs_block_fb_pixel_scripts_cat'
			),
			'_fbc'                                      => array(
				'parent_script'  => 'fb_pixel',
				'expiry'         => '2 years',
				'type'           => 'Marketing/Tracking',
				'description'    => 'to store last visit.',
				'category'       => $cs_block_fb_pixel_scripts_cat,
				'category_id'    => $cs_block_fb_pixel_cat_id,
				'parent_cat_key' => 'cs_block_fb_pixel_scripts_cat'
			),
			'fbm'                                       => array(
				'parent_script'  => 'fb_pixel',
				'expiry'         => '1 year',
				'type'           => 'Marketing/Tracking',
				'description'    => 'to store account details.',
				'category'       => $cs_block_fb_pixel_scripts_cat,
				'category_id'    => $cs_block_fb_pixel_cat_id,
				'parent_cat_key' => 'cs_block_fb_pixel_scripts_cat'
			),
			'xs'                                        => array(
				'parent_script'  => 'fb_pixel',
				'expiry'         => '3 months',
				'type'           => 'Marketing/Tracking',
				'description'    => 'to store a unique session ID.',
				'category'       => $cs_block_fb_pixel_scripts_cat,
				'category_id'    => $cs_block_fb_pixel_cat_id,
				'parent_cat_key' => 'cs_block_fb_pixel_scripts_cat'
			),
			'wd'                                        => array(
				'parent_script'  => 'fb_pixel',
				'expiry'         => '1 week',
				'type'           => 'Functional',
				'description'    => 'to read screen resolution.',
				'category'       => $cs_block_fb_pixel_scripts_cat,
				'category_id'    => $cs_block_fb_pixel_cat_id,
				'parent_cat_key' => 'cs_block_fb_pixel_scripts_cat'
			),
			'fr'                                        => array(
				'parent_script'  => 'fb_pixel',
				'expiry'         => '3 months',
				'type'           => 'Marketing/Tracking',
				'description'    => 'to provide ad delivery or retargeting.',
				'category'       => $cs_block_fb_pixel_scripts_cat,
				'category_id'    => $cs_block_fb_pixel_cat_id,
				'parent_cat_key' => 'cs_block_fb_pixel_scripts_cat'
			),
			'act'                                       => array(
				'parent_script'  => 'fb_pixel',
				'expiry'         => '90 days',
				'type'           => 'Marketing/Tracking',
				'description'    => 'to Store logged in users.',
				'category'       => $cs_block_fb_pixel_scripts_cat,
				'category_id'    => $cs_block_fb_pixel_cat_id,
				'parent_cat_key' => 'cs_block_fb_pixel_scripts_cat'
			),
			'_fbp'                                      => array(
				'parent_script'  => 'fb_pixel',
				'expiry'         => '3 months',
				'type'           => 'Marketing/Tracking',
				'description'    => 'to store and track visits across websites.',
				'category'       => $cs_block_fb_pixel_scripts_cat,
				'category_id'    => $cs_block_fb_pixel_cat_id,
				'parent_cat_key' => 'cs_block_fb_pixel_scripts_cat'
			),
			'datr'                                      => array(
				'parent_script'  => 'fb_pixel',
				'expiry'         => '2 years',
				'type'           => 'Marketing/Tracking',
				'description'    => 'to provide fraud prevention.',
				'category'       => $cs_block_fb_pixel_scripts_cat,
				'category_id'    => $cs_block_fb_pixel_cat_id,
				'parent_cat_key' => 'cs_block_fb_pixel_scripts_cat'
			),
			'c_user'                                    => array(
				'parent_script'  => 'fb_pixel',
				'expiry'         => '30 days',
				'type'           => 'Marketing/Tracking',
				'description'    => 'to store a unique user ID.',
				'category'       => $cs_block_fb_pixel_scripts_cat,
				'category_id'    => $cs_block_fb_pixel_cat_id,
				'parent_cat_key' => 'cs_block_fb_pixel_scripts_cat'
			),
			'csm'                                       => array(
				'parent_script'  => 'fb_pixel',
				'expiry'         => '90 days',
				'type'           => 'Functional',
				'description'    => 'to provide fraud prevention.',
				'category'       => $cs_block_fb_pixel_scripts_cat,
				'category_id'    => $cs_block_fb_pixel_cat_id,
				'parent_cat_key' => 'cs_block_fb_pixel_scripts_cat'
			),
			'sb'                                        => array(
				'parent_script'  => 'fb_pixel',
				'expiry'         => '2 years',
				'type'           => 'Functional',
				'description'    => 'to store browser details.',
				'category'       => $cs_block_fb_pixel_scripts_cat,
				'category_id'    => $cs_block_fb_pixel_cat_id,
				'parent_cat_key' => 'cs_block_fb_pixel_scripts_cat'
			),
			'actppresence'                              => array(
				'parent_script'  => 'fb_pixel',
				'expiry'         => 'session',
				'type'           => 'Functional',
				'description'    => 'to store and track if the browser tab is active.',
				'category'       => $cs_block_fb_pixel_scripts_cat,
				'category_id'    => $cs_block_fb_pixel_cat_id,
				'parent_cat_key' => 'cs_block_fb_pixel_scripts_cat'
			),
			'_fbm_'                                     => array(
				'parent_script'  => 'fb_pixel',
				'expiry'         => '1 year',
				'type'           => 'Marketing/Tracking',
				'description'    => 'to store account details.',
				'category'       => $cs_block_fb_pixel_scripts_cat,
				'category_id'    => $cs_block_fb_pixel_cat_id,
				'parent_cat_key' => 'cs_block_fb_pixel_scripts_cat'
			),
			'_uetvid_exp'                               => array(
				'parent_script'  => 'big_tag',
				'expiry'         => 'Persistent',
				'type'           => 'Marketing/Tracking',
				'description'    => 'Contains the expiry-date for the cookie with corresponding name.',
				'category'       => $cs_block_big_tag_scripts_cat,
				'category_id'    => $cs_block_big_tag_cat_id,
				'parent_cat_key' => 'cs_block_big_tag_scripts_cat'
			),
			'_uetvid'                                   => array(
				'parent_script'  => 'big_tag',
				'expiry'         => '16 days',
				'type'           => 'Marketing/Tracking',
				'description'    => 'to store and track visits across websites.',
				'category'       => $cs_block_big_tag_scripts_cat,
				'category_id'    => $cs_block_big_tag_cat_id,
				'parent_cat_key' => 'cs_block_big_tag_scripts_cat'
			),
			'_uetsid'                                   => array(
				'parent_script'  => 'big_tag',
				'expiry'         => '1 day',
				'type'           => 'Marketing/Tracking',
				'description'    => 'to store and track visits across websites.',
				'category'       => $cs_block_big_tag_scripts_cat,
				'category_id'    => $cs_block_big_tag_cat_id,
				'parent_cat_key' => 'cs_block_big_tag_scripts_cat'
			),
			'_uetsid_exp'                               => array(
				'parent_script'  => 'big_tag',
				'expiry'         => 'Persistent',
				'type'           => 'Marketing/Tracking',
				'description'    => 'Contains the expiry-date for the cookie with corresponding name.',
				'category'       => $cs_block_big_tag_scripts_cat,
				'category_id'    => $cs_block_big_tag_cat_id,
				'parent_cat_key' => 'cs_block_big_tag_scripts_cat'
			),
			'_uetmsclkid'                               => array(
				'parent_script'  => 'big_tag',
				'expiry'         => '',
				'type'           => 'Functional',
				'description'    => 'This is the Microsoft Click ID, which is used to improve the accuracy of conversion tracking.',
				'category'       => $cs_block_big_tag_scripts_cat,
				'category_id'    => $cs_block_big_tag_cat_id,
				'parent_cat_key' => 'cs_block_big_tag_scripts_cat'
			),
			'_pinterest_sess'                           => array(
				'parent_script'  => 'pin_tag',
				'expiry'         => '1 year',
				'type'           => '',
				'description'    => 'is the Pinterest login cookie. It contains user ID(s), authentication token(s) and timestamps. If the person is logged out, authentication tokens are deleted but we leave the cookie present. We use the logged out user ID(s) to optimize the person’s experience and measurement.',
				'category'       => $cs_block_pin_tag_scripts_cat,
				'category_id'    => $cs_block_pin_tag_cat_id,
				'parent_cat_key' => 'cs_block_pin_tag_scripts_cat'
			),
			'_pinterest_ct'                             => array(
				'parent_script'  => 'pin_tag',
				'expiry'         => 'Session',
				'type'           => '',
				'description'    => '_pinterest_ct and _pinterest_ct_rt are identical. They contain a user ID and the timestamp at which the cookie was created.',
				'category'       => $cs_block_pin_tag_scripts_cat,
				'category_id'    => $cs_block_pin_tag_cat_id,
				'parent_cat_key' => 'cs_block_pin_tag_scripts_cat'
			),
			'_pinterest_ct_rt'                          => array(
				'parent_script'  => 'pin_tag',
				'expiry'         => 'Session',
				'type'           => '',
				'description'    => '_pinterest_ct and _pinterest_ct_rt are identical. They contain a user ID and the timestamp at which the cookie was created.',
				'category'       => $cs_block_pin_tag_scripts_cat,
				'category_id'    => $cs_block_pin_tag_cat_id,
				'parent_cat_key' => 'cs_block_pin_tag_scripts_cat'
			),
			'_epik'                                     => array(
				'parent_script'  => 'pin_tag',
				'expiry'         => '1 year',
				'type'           => '',
				'description'    => 'is placed by the JavaScript tag based on information sent from Pinterest with promoted traffic to help identify the user.',
				'category'       => $cs_block_pin_tag_scripts_cat,
				'category_id'    => $cs_block_pin_tag_cat_id,
				'parent_cat_key' => 'cs_block_pin_tag_scripts_cat'
			),
			'_derived_epik'                             => array(
				'parent_script'  => 'pin_tag',
				'expiry'         => '1 year',
				'type'           => '',
				'description'    => 'is placed by the Pinterest tag when a match is identified when no cookies are present, such as Enhanced Match.',
				'category'       => $cs_block_pin_tag_scripts_cat,
				'category_id'    => $cs_block_pin_tag_cat_id,
				'parent_cat_key' => 'cs_block_pin_tag_scripts_cat'
			),
			'_pin_unauth'                               => array(
				'parent_script'  => 'pin_tag',
				'expiry'         => '1 year',
				'type'           => 'Marketing/Tracking',
				'description'    => 'is a first party cookie which groups actions for users who cannot be identified by Pinterest.',
				'category'       => $cs_block_pin_tag_scripts_cat,
				'category_id'    => $cs_block_pin_tag_cat_id,
				'parent_cat_key' => 'cs_block_pin_tag_scripts_cat'
			),
			'_pinterest_ct_ua'                          => array(
				'parent_script'  => 'pin_tag',
				'expiry'         => '',
				'type'           => '',
				'description'    => 'is identical to _pin_unauth, but as a third party cookie.',
				'category'       => $cs_block_pin_tag_scripts_cat,
				'category_id'    => $cs_block_pin_tag_cat_id,
				'parent_cat_key' => 'cs_block_pin_tag_scripts_cat'
			),
			'_pinterest_pfob'                           => array(
				'parent_script'  => 'pin_tag',
				'expiry'         => 'Persistent',
				'type'           => '',
				'description'    => 'Cookie for Pinterest functionality',
				'category'       => $cs_block_pin_tag_scripts_cat,
				'category_id'    => $cs_block_pin_tag_cat_id,
				'parent_cat_key' => 'cs_block_pin_tag_scripts_cat'
			),
			'PREF'                                      => array(
				'parent_script'  => 'yt_embedded',
				'expiry'         => '8 months',
				'type'           => 'Marketing/Tracking',
				'description'    => 'to store user preferences.',
				'category'       => $cs_block_yt_embedded_scripts_cat,
				'category_id'    => $cs_block_yt_embedded_cat_id,
				'parent_cat_key' => 'cs_block_yt_embedded_scripts_cat'
			),
			'GPS'                                       => array(
				'parent_script'  => 'yt_embedded',
				'expiry'         => 'session',
				'type'           => 'Marketing/Tracking',
				'description'    => 'to store location data.',
				'category'       => $cs_block_yt_embedded_scripts_cat,
				'category_id'    => $cs_block_yt_embedded_cat_id,
				'parent_cat_key' => 'cs_block_yt_embedded_scripts_cat'
			),
			'VISITOR_INFO1_LIVE'                        => array(
				'parent_script'  => 'yt_embedded',
				'expiry'         => '6 months',
				'type'           => 'Functional',
				'description'    => 'to provide bandwidth estimations.',
				'category'       => $cs_block_yt_embedded_scripts_cat,
				'category_id'    => $cs_block_yt_embedded_cat_id,
				'parent_cat_key' => 'cs_block_yt_embedded_scripts_cat'
			),
			'YSC'                                       => array(
				'parent_script'  => 'yt_embedded',
				'expiry'         => 'session',
				'type'           => 'Marketing/Tracking',
				'description'    => 'to Store and track interaction.',
				'category'       => $cs_block_yt_embedded_scripts_cat,
				'category_id'    => $cs_block_yt_embedded_cat_id,
				'parent_cat_key' => 'cs_block_yt_embedded_scripts_cat'
			),
			'vuid'                                      => array(
				'parent_script'  => 'vimeo_embedded',
				'expiry'         => '2 years',
				'type'           => 'Statistics',
				'description'    => "to store the user usage history.",
				'category'       => $cs_block_vimeo_embedded_scripts_cat,
				'category_id'    => $cs_block_vimeo_embedded_cat_id,
				'parent_cat_key' => 'cs_block_vimeo_embedded_scripts_cat'
			),
			'__utmt_player'                             => array(
				'parent_script'  => 'vimeo_embedded',
				'expiry'         => '10 minutes',
				'type'           => 'Statistics',
				'description'    => 'to store and track audience reach.',
				'category'       => $cs_block_vimeo_embedded_scripts_cat,
				'category_id'    => $cs_block_vimeo_embedded_cat_id,
				'parent_cat_key' => 'cs_block_vimeo_embedded_scripts_cat'
			),
			'sclocale'                                  => array(
				'parent_script'  => 'soundcloud',
				'expiry'         => '1 year',
				'type'           => 'Functional',
				'description'    => 'to store language settings.',
				'category'       => $cs_block_soundcloud_scripts_cat,
				'category_id'    => $cs_block_soundcloud_cat_id,
				'parent_cat_key' => 'cs_block_soundcloud_scripts_cat'
			),
			'sc_anonymous_id'                           => array(
				'parent_script'  => 'soundcloud',
				'expiry'         => '10 years',
				'type'           => 'Functional',
				'description'    => 'to provide functions across pages.',
				'category'       => $cs_block_soundcloud_scripts_cat,
				'category_id'    => $cs_block_soundcloud_cat_id,
				'parent_cat_key' => 'cs_block_soundcloud_scripts_cat'
			),
			'__hstc'                                    => array(
				'parent_script'  => 'hubspot',
				'expiry'         => '13 months',
				'type'           => 'Analytics',
				'description'    => 'The main cookie for tracking visitors. It contains the domain, utk, initial timestamp (first visit), last timestamp (last visit), current timestamp (this visit), and session number (increments for each subsequent session).',
				'category'       => $cs_block_hubspot_scripts_cat,
				'category_id'    => $cs_block_hubspot_cat_id,
				'parent_cat_key' => 'cs_block_hubspot_scripts_cat'
			),
			'hubspotutk'                                => array(
				'parent_script'  => 'hubspot',
				'expiry'         => '13 months',
				'type'           => 'Analytics',
				'description'    => "This cookie keeps track of a visitor's identity. It is passed to HubSpot on form submission and used when deduplicating contacts. It contains an opaque GUID to represent the current visitor.",
				'category'       => $cs_block_hubspot_scripts_cat,
				'category_id'    => $cs_block_hubspot_cat_id,
				'parent_cat_key' => 'cs_block_hubspot_scripts_cat'
			),
			'__hssc'                                    => array(
				'parent_script'  => 'hubspot',
				'expiry'         => '30 minutes',
				'type'           => 'Analytics',
				'description'    => 'This cookie keeps track of sessions. This is used to determine if HubSpot should increment the session number and timestamps in the __hstc cookie. It contains the domain, viewCount (increments each pageView in a session), and session start timestamp.',
				'category'       => $cs_block_hubspot_scripts_cat,
				'category_id'    => $cs_block_hubspot_cat_id,
				'parent_cat_key' => 'cs_block_hubspot_scripts_cat'
			),
			'__hssrc'                                   => array(
				'parent_script'  => 'hubspot',
				'expiry'         => 'session',
				'type'           => 'Analytics',
				'description'    => 'Whenever HubSpot changes the session cookie, this cookie is also set to determine if the visitor has restarted their browser. If this cookie does not exist when HubSpot manages cookies, it is considered a new session. It contains the value "1" when present.',
				'category'       => $cs_block_hubspot_scripts_cat,
				'category_id'    => $cs_block_hubspot_cat_id,
				'parent_cat_key' => 'cs_block_hubspot_scripts_cat'
			),
			'messagesUtk'                               => array(
				'parent_script'  => 'hubspot',
				'expiry'         => '13 months',
				'type'           => 'Functionality',
				'description'    => "This cookie is used to recognize visitors who chat with you via the chatflows tool. If the visitor leaves your site before they're added as a contact, they will have this cookie associated with their browser. If you chat with a visitor who later returns to your site in the same cookied browser, the chatflows tool will load their conversation history. The cookie is controlled by the Consent to collect chat cookies setting in your chatflow. If this setting is disabled, the cookie is controlled by the Consent to process setting in your chatflow. HubSpot will not drop the messagesUtk cookie for visitors who have been identified through the Visitor Identification API. The analytics cookie banner will not be impacted. This cookie will be specific to a subdomain and will not carry over to other subdomains. For example, the cookie dropped for info.example.com will not apply to the visitor when they visit www.example.com, and vice versa. It contains an opaque GUID to represent the current chat user.",
				'category'       => $cs_block_hubspot_scripts_cat,
				'category_id'    => $cs_block_hubspot_cat_id,
				'parent_cat_key' => 'cs_block_hubspot_scripts_cat'
			),
			'_hjIncludedInSample'                       => array(
				'parent_script'  => 'hotjar',
				'expiry'         => 'session',
				'type'           => 'Statistics (anonymous)',
				'description'    => 'to store anonymized statistics.',
				'category'       => $cs_block_hotjar_scripts_cat,
				'category_id'    => $cs_block_hotjar_cat_id,
				'parent_cat_key' => 'cs_block_hotjar_scripts_cat'
			),
			'_hjSessionRejected'                        => array(
				'parent_script'  => 'hotjar',
				'expiry'         => 'session',
				'type'           => 'Functional',
				'description'    => 'to provide load balancing functionality.',
				'category'       => $cs_block_hotjar_scripts_cat,
				'category_id'    => $cs_block_hotjar_cat_id,
				'parent_cat_key' => 'cs_block_hotjar_scripts_cat'
			),
			'_hjAbsoluteSessionInProgress'              => array(
				'parent_script'  => 'hotjar',
				'expiry'         => 'session',
				'type'           => 'Statistics',
				'description'    => 'to store unique visits.',
				'category'       => $cs_block_hotjar_scripts_cat,
				'category_id'    => $cs_block_hotjar_cat_id,
				'parent_cat_key' => 'cs_block_hotjar_scripts_cat'
			),
			'_hjRecordingLastActivity'                  => array(
				'parent_script'  => 'hotjar',
				'expiry'         => 'session',
				'type'           => 'Statistics',
				'description'    => 'to store performed actions on the website.',
				'category'       => $cs_block_hotjar_scripts_cat,
				'category_id'    => $cs_block_hotjar_cat_id,
				'parent_cat_key' => 'cs_block_hotjar_scripts_cat'
			),
			'_hjMinimizedTestersWidgets'                => array(
				'parent_script'  => 'hotjar',
				'expiry'         => '1 year',
				'type'           => 'Functional',
				'description'    => 'to store if a message has been dismissed.',
				'category'       => $cs_block_hotjar_scripts_cat,
				'category_id'    => $cs_block_hotjar_cat_id,
				'parent_cat_key' => 'cs_block_hotjar_scripts_cat'
			),
			'_hjUserId'                                 => array(
				'parent_script'  => 'hotjar',
				'expiry'         => 'session',
				'type'           => 'Functional',
				'description'    => 'to store a unique user ID.',
				'category'       => $cs_block_hotjar_scripts_cat,
				'category_id'    => $cs_block_hotjar_cat_id,
				'parent_cat_key' => 'cs_block_hotjar_scripts_cat'
			),
			'hjViewportId'                              => array(
				'parent_script'  => 'hotjar',
				'expiry'         => 'session',
				'type'           => 'Statistics',
				'description'    => 'to store a unique session ID.',
				'category'       => $cs_block_hotjar_scripts_cat,
				'category_id'    => $cs_block_hotjar_cat_id,
				'parent_cat_key' => 'cs_block_hotjar_scripts_cat'
			),
			'_hjShownFeedbackMessage'                   => array(
				'parent_script'  => 'hotjar',
				'expiry'         => '1 year',
				'type'           => 'Functional',
				'description'    => 'to store if a message has been shown.',
				'category'       => $cs_block_hotjar_scripts_cat,
				'category_id'    => $cs_block_hotjar_cat_id,
				'parent_cat_key' => 'cs_block_hotjar_scripts_cat'
			),
			'_hjid'                                     => array(
				'parent_script'  => 'hotjar',
				'expiry'         => '1 year',
				'type'           => 'Statistics',
				'description'    => 'to store a unique user ID.',
				'category'       => $cs_block_hotjar_scripts_cat,
				'category_id'    => $cs_block_hotjar_cat_id,
				'parent_cat_key' => 'cs_block_hotjar_scripts_cat'
			),
			'_hjDoneTestersWidgets'                     => array(
				'parent_script'  => 'hotjar',
				'expiry'         => '1 year',
				'type'           => 'Functional',
				'description'    => 'to store performed actions on the website.',
				'category'       => $cs_block_hotjar_scripts_cat,
				'category_id'    => $cs_block_hotjar_cat_id,
				'parent_cat_key' => 'cs_block_hotjar_scripts_cat'
			),
			'_hjMinimizedPolls'                         => array(
				'parent_script'  => 'hotjar',
				'expiry'         => '1 year',
				'type'           => 'Functional',
				'description'    => 'to store if a message has been dismissed.',
				'category'       => $cs_block_hotjar_scripts_cat,
				'category_id'    => $cs_block_hotjar_cat_id,
				'parent_cat_key' => 'cs_block_hotjar_scripts_cat'
			),
			'_hjDonePolls'                              => array(
				'parent_script'  => 'hotjar',
				'expiry'         => '1 year',
				'type'           => 'Functional',
				'description'    => 'to store performed actions on the website.',
				'category'       => $cs_block_hotjar_scripts_cat,
				'category_id'    => $cs_block_hotjar_cat_id,
				'parent_cat_key' => 'cs_block_hotjar_scripts_cat'
			),
			'_hjClosedSurveyInvites'                    => array(
				'parent_script'  => 'hotjar',
				'expiry'         => '1 year',
				'type'           => 'Functional',
				'description'    => 'to store if a message has been shown.',
				'category'       => $cs_block_hotjar_scripts_cat,
				'category_id'    => $cs_block_hotjar_cat_id,
				'parent_cat_key' => 'cs_block_hotjar_scripts_cat'
			),
			'__atrfs'                                   => array(
				'parent_script'  => 'addthis',
				'expiry'         => 'session',
				'type'           => 'Marketing/Tracking',
				'description'    => 'to provide functions across pages.',
				'category'       => $cs_block_addthis_scripts_cat,
				'category_id'    => $cs_block_addthis_cat_id,
				'parent_cat_key' => 'cs_block_addthis_scripts_cat'
			),
			'_at.hist.'                                 => array(
				'parent_script'  => 'addthis',
				'expiry'         => 'persistent',
				'type'           => 'Statistics',
				'description'    => "to store the user usage history.",
				'category'       => $cs_block_addthis_scripts_cat,
				'category_id'    => $cs_block_addthis_cat_id,
				'parent_cat_key' => 'cs_block_addthis_scripts_cat'
			),
			'at-lojson-cache-ra-'                       => array(
				'parent_script'  => 'addthis',
				'expiry'         => 'persistent',
				'type'           => 'Marketing/Tracking',
				'description'    => 'to provide functions across pages.',
				'category'       => $cs_block_addthis_scripts_cat,
				'category_id'    => $cs_block_addthis_cat_id,
				'parent_cat_key' => 'cs_block_addthis_scripts_cat'
			),
			'ouid'                                      => array(
				'parent_script'  => 'addthis',
				'expiry'         => '1 year',
				'type'           => 'Marketing/Tracking',
				'description'    => 'to store a unique user ID.',
				'category'       => $cs_block_addthis_scripts_cat,
				'category_id'    => $cs_block_addthis_cat_id,
				'parent_cat_key' => 'cs_block_addthis_scripts_cat'
			),
			'na_tc'                                     => array(
				'parent_script'  => 'addthis',
				'expiry'         => '2 years',
				'type'           => 'Marketing/Tracking',
				'description'    => 'to store dynamic variables from the browser.',
				'category'       => $cs_block_addthis_scripts_cat,
				'category_id'    => $cs_block_addthis_cat_id,
				'parent_cat_key' => 'cs_block_addthis_scripts_cat'
			),
			'na_id'                                     => array(
				'parent_script'  => 'addthis',
				'expiry'         => '1 year',
				'type'           => 'Marketing/Tracking',
				'description'    => "to store the user usage history.",
				'category'       => $cs_block_addthis_scripts_cat,
				'category_id'    => $cs_block_addthis_cat_id,
				'parent_cat_key' => 'cs_block_addthis_scripts_cat'
			),
			'mus'                                       => array(
				'parent_script'  => 'addthis',
				'expiry'         => '1 year',
				'type'           => 'Marketing/Tracking',
				'description'    => 'to provide functions across pages.',
				'category'       => $cs_block_addthis_scripts_cat,
				'category_id'    => $cs_block_addthis_cat_id,
				'parent_cat_key' => 'cs_block_addthis_scripts_cat'
			),
			'loc'                                       => array(
				'parent_script'  => 'addthis',
				'expiry'         => '13 months',
				'type'           => 'Marketing/Tracking',
				'description'    => 'to store location data.',
				'category'       => $cs_block_addthis_scripts_cat,
				'category_id'    => $cs_block_addthis_cat_id,
				'parent_cat_key' => 'cs_block_addthis_scripts_cat'
			),
			'cw_id'                                     => array(
				'parent_script'  => 'addthis',
				'expiry'         => '1 month',
				'type'           => 'Marketing/Tracking',
				'description'    => "to store the user usage history.",
				'category'       => $cs_block_addthis_scripts_cat,
				'category_id'    => $cs_block_addthis_cat_id,
				'parent_cat_key' => 'cs_block_addthis_scripts_cat'
			),
			'uvc'                                       => array(
				'parent_script'  => 'addthis',
				'expiry'         => '13 months',
				'type'           => 'Marketing/Tracking',
				'description'    => 'to store and track visits across websites.',
				'category'       => $cs_block_addthis_scripts_cat,
				'category_id'    => $cs_block_addthis_cat_id,
				'parent_cat_key' => 'cs_block_addthis_scripts_cat'
			),
			'uid'                                       => array(
				'parent_script'  => 'addthis',
				'expiry'         => '1 year',
				'type'           => 'Statistics',
				'description'    => 'to store a unique user ID.',
				'category'       => $cs_block_addthis_scripts_cat,
				'category_id'    => $cs_block_addthis_cat_id,
				'parent_cat_key' => 'cs_block_addthis_scripts_cat'
			),
			'at-lojson-cache-wp-'                       => array(
				'parent_script'  => 'addthis',
				'expiry'         => 'persistent',
				'type'           => 'Marketing/Tracking',
				'description'    => 'to provide functions across pages.',
				'category'       => $cs_block_addthis_scripts_cat,
				'category_id'    => $cs_block_addthis_cat_id,
				'parent_cat_key' => 'cs_block_addthis_scripts_cat'
			),
			'__atuvc'                                   => array(
				'parent_script'  => 'addthis',
				'expiry'         => '13 months',
				'type'           => 'Functional',
				'description'    => 'to store performed actions on the website.',
				'category'       => $cs_block_addthis_scripts_cat,
				'category_id'    => $cs_block_addthis_cat_id,
				'parent_cat_key' => 'cs_block_addthis_scripts_cat'
			),
			'__atuvs'                                   => array(
				'parent_script'  => 'addthis',
				'expiry'         => '1 year',
				'type'           => 'Functional',
				'description'    => 'to store performed actions on the website.',
				'category'       => $cs_block_addthis_scripts_cat,
				'category_id'    => $cs_block_addthis_cat_id,
				'parent_cat_key' => 'cs_block_addthis_scripts_cat'
			),
			'_pk_testcookie'                            => array(
				'parent_script'  => 'matomo',
				'expiry'         => 'session',
				'type'           => 'Statistics (anonymous)',
				'description'    => 'to store and count pageviews.',
				'category'       => $cs_block_matomo_scripts_cat,
				'category_id'    => $cs_block_matomo_cat_id,
				'parent_cat_key' => 'cs_block_matomo_scripts_cat'
			),
			'MATOMO_SESSID'                             => array(
				'parent_script'  => 'matomo',
				'expiry'         => 'session',
				'type'           => 'Functional',
				'description'    => 'to provide fraud prevention.',
				'category'       => $cs_block_matomo_scripts_cat,
				'category_id'    => $cs_block_matomo_cat_id,
				'parent_cat_key' => 'cs_block_matomo_scripts_cat'
			),
			'_pk_ses'                                   => array(
				'parent_script'  => 'matomo',
				'expiry'         => 'session',
				'type'           => 'Statistics (anonymous)',
				'description'    => 'to store a unique session ID.',
				'category'       => $cs_block_matomo_scripts_cat,
				'category_id'    => $cs_block_matomo_cat_id,
				'parent_cat_key' => 'cs_block_matomo_scripts_cat'
			),
			'_pk_id'                                    => array(
				'parent_script'  => 'matomo',
				'expiry'         => '13 months',
				'type'           => 'Statistics (anonymous)',
				'description'    => 'to store a unique user ID.',
				'category'       => $cs_block_matomo_scripts_cat,
				'category_id'    => $cs_block_matomo_cat_id,
				'parent_cat_key' => 'cs_block_matomo_scripts_cat'
			),
			'_pk_ref'                                   => array(
				'parent_script'  => 'matomo',
				'expiry'         => '6 months',
				'type'           => 'Statistics (anonymous)',
				'description'    => "to store referrer ID's.",
				'category'       => $cs_block_matomo_scripts_cat,
				'category_id'    => $cs_block_matomo_cat_id,
				'parent_cat_key' => 'cs_block_matomo_scripts_cat'
			),
			'__stid'                                    => array(
				'parent_script'  => 'sharethis',
				'expiry'         => '1 week',
				'type'           => 'Marketing/Tracking',
				'description'    => 'to store and track audience reach.',
				'category'       => $cs_block_sharethis_scripts_cat,
				'category_id'    => $cs_block_sharethis_cat_id,
				'parent_cat_key' => 'cs_block_sharethis_scripts_cat'
			),
			'__unam'                                    => array(
				'parent_script'  => 'sharethis',
				'expiry'         => '2 years',
				'type'           => 'Marketing/Tracking',
				'description'    => 'to store a unique session ID.',
				'category'       => $cs_block_sharethis_scripts_cat,
				'category_id'    => $cs_block_sharethis_cat_id,
				'parent_cat_key' => 'cs_block_sharethis_scripts_cat'
			),
			'_stgmap'                                   => array(
				'parent_script'  => 'sharethis',
				'expiry'         => '1 week',
				'type'           => 'Marketing/Tracking',
				'description'    => 'to store and track audience reach.',
				'category'       => $cs_block_sharethis_scripts_cat,
				'category_id'    => $cs_block_sharethis_cat_id,
				'parent_cat_key' => 'cs_block_sharethis_scripts_cat'
			),
			'_stamap'                                   => array(
				'parent_script'  => 'sharethis',
				'expiry'         => '1 week',
				'type'           => 'Marketing/Tracking',
				'description'    => 'to store and track audience reach.',
				'category'       => $cs_block_sharethis_scripts_cat,
				'category_id'    => $cs_block_sharethis_cat_id,
				'parent_cat_key' => 'cs_block_sharethis_scripts_cat'
			),
			'stacxiommap'                               => array(
				'parent_script'  => 'sharethis',
				'expiry'         => '1 week',
				'type'           => 'Marketing/Tracking',
				'description'    => 'to store and track audience reach.',
				'category'       => $cs_block_sharethis_scripts_cat,
				'category_id'    => $cs_block_sharethis_cat_id,
				'parent_cat_key' => 'cs_block_sharethis_scripts_cat'
			),
			'stdlxmap'                                  => array(
				'parent_script'  => 'sharethis',
				'expiry'         => '1 week',
				'type'           => 'Marketing/Tracking',
				'description'    => 'to store and track audience reach.',
				'category'       => $cs_block_sharethis_scripts_cat,
				'category_id'    => $cs_block_sharethis_cat_id,
				'parent_cat_key' => 'cs_block_sharethis_scripts_cat'
			),
			'AnalyticsSyncHistory'                      => array(
				'parent_script'  => 'ln_tag',
				'expiry'         => '30 days',
				'type'           => '',
				'description'    => 'Used to store information about the time a sync with the lms_analytics cookie took place for users in the Designated Countries',
				'category'       => $cs_block_ln_tag_scripts_cat,
				'category_id'    => $cs_block_ln_tag_scripts_cat_id,
				'parent_cat_key' => 'cs_block_ln_tag_scripts_cat'
			),
			'lms_analytics'                             => array(
				'parent_script'  => 'ln_tag',
				'expiry'         => '30 days',
				'type'           => '',
				'description'    => 'Used to identify LinkedIn Members in the Designated Countries for analytics',
				'category'       => $cs_block_ln_tag_scripts_cat,
				'category_id'    => $cs_block_ln_tag_scripts_cat_id,
				'parent_cat_key' => 'cs_block_ln_tag_scripts_cat'
			),
			'li_sugr'                                   => array(
				'parent_script'  => 'ln_tag',
				'expiry'         => '90 days',
				'type'           => '',
				'description'    => "Used to make a probabilistic match of a user identity outside the Designated Countries",
				'category'       => $cs_block_ln_tag_scripts_cat,
				'category_id'    => $cs_block_ln_tag_scripts_cat_id,
				'parent_cat_key' => 'cs_block_ln_tag_scripts_cat'
			),
			'U'                                         => array(
				'parent_script'  => 'ln_tag',
				'expiry'         => '3 months',
				'type'           => '',
				'description'    => 'Browser Identifier for users outside the Designated Countries',
				'category'       => $cs_block_ln_tag_scripts_cat,
				'category_id'    => $cs_block_ln_tag_scripts_cat_id,
				'parent_cat_key' => 'cs_block_ln_tag_scripts_cat'
			),
			'_guid'                                     => array(
				'parent_script'  => 'ln_tag',
				'expiry'         => '90 days',
				'type'           => '',
				'description'    => 'Used to identify a LinkedIn Member for advertising through Google Ads',
				'category'       => $cs_block_ln_tag_scripts_cat,
				'category_id'    => $cs_block_ln_tag_scripts_cat_id,
				'parent_cat_key' => 'cs_block_ln_tag_scripts_cat'
			),
			'BizographicsOptOut'                        => array(
				'parent_script'  => 'ln_tag',
				'expiry'         => '10 years',
				'type'           => '',
				'description'    => 'Determine opt-out status for 3rd party tracking',
				'category'       => $cs_block_ln_tag_scripts_cat,
				'category_id'    => $cs_block_ln_tag_scripts_cat_id,
				'parent_cat_key' => 'cs_block_ln_tag_scripts_cat'
			),
			'li_giant'                                  => array(
				'parent_script'  => 'ln_tag',
				'expiry'         => '7 days',
				'type'           => '',
				'description'    => 'Indirect indentifier for groups of LinkedIn Members used for conversion tracking',
				'category'       => $cs_block_ln_tag_scripts_cat,
				'category_id'    => $cs_block_ln_tag_scripts_cat_id,
				'parent_cat_key' => 'cs_block_ln_tag_scripts_cat'
			),
			'UserMatchHistory'                          => array(
				'parent_script'  => 'ln_tag',
				'expiry'         => '30 days',
				'type'           => '',
				'description'    => 'LinkedIn Ads ID syncing',
				'category'       => $cs_block_ln_tag_scripts_cat,
				'category_id'    => $cs_block_ln_tag_scripts_cat_id,
				'parent_cat_key' => 'cs_block_ln_tag_scripts_cat'
			),
			'li_oatml'                                  => array(
				'parent_script'  => 'ln_tag',
				'expiry'         => '30 days',
				'type'           => '',
				'description'    => 'Used to identify LinkedIn Members off LinkedIn for advertising and analytics outside the Designated Countries and, for a limited time, advertising in the Designated Countries',
				'category'       => $cs_block_ln_tag_scripts_cat,
				'category_id'    => $cs_block_ln_tag_scripts_cat_id,
				'parent_cat_key' => 'cs_block_ln_tag_scripts_cat'
			),
			'lms_ads'                                   => array(
				'parent_script'  => 'ln_tag',
				'expiry'         => '30 days',
				'type'           => '',
				'description'    => 'Used to identify LinkedIn Members off LinkedIn in the Designated Countries for advertising',
				'category'       => $cs_block_ln_tag_scripts_cat,
				'category_id'    => $cs_block_ln_tag_scripts_cat_id,
				'parent_cat_key' => 'cs_block_ln_tag_scripts_cat'
			),
			'li_fat_id'                                 => array(
				'parent_script'  => 'ln_tag',
				'expiry'         => '30 days',
				'type'           => '',
				'description'    => 'Member indirect identifier for Members for conversion tracking, retargeting, analytics',
				'category'       => $cs_block_ln_tag_scripts_cat,
				'category_id'    => $cs_block_ln_tag_scripts_cat_id,
				'parent_cat_key' => 'cs_block_ln_tag_scripts_cat'
			),
			'A3'                                        => array(
				'parent_script'  => 'ln_tag',
				'expiry'         => '1 year',
				'type'           => '',
				'description'    => 'Ads targeting cookie for Yahoo',
				'category'       => $cs_block_ln_tag_scripts_cat,
				'category_id'    => $cs_block_ln_tag_scripts_cat_id,
				'parent_cat_key' => 'cs_block_ln_tag_scripts_cat'
			),
			'anj'                                       => array(
				'parent_script'  => 'ln_tag',
				'expiry'         => '3 months',
				'type'           => '',
				'description'    => 'Ads targeting cookie for AppNexus',
				'category'       => $cs_block_ln_tag_scripts_cat,
				'category_id'    => $cs_block_ln_tag_scripts_cat_id,
				'parent_cat_key' => 'cs_block_ln_tag_scripts_cat'
			),
			'uuids'                                     => array(
				'parent_script'  => 'ln_tag',
				'expiry'         => '3 months',
				'type'           => '',
				'description'    => 'Used by ad nexus analytics by AppNexus',
				'category'       => $cs_block_ln_tag_scripts_cat,
				'category_id'    => $cs_block_ln_tag_scripts_cat_id,
				'parent_cat_key' => 'cs_block_ln_tag_scripts_cat'
			),
			'ELOQUA'                                    => array(
				'parent_script'  => 'ln_tag',
				'expiry'         => '2 years',
				'type'           => '',
				'description'    => 'Used for tracking by ELOQUA',
				'category'       => $cs_block_ln_tag_scripts_cat,
				'category_id'    => $cs_block_ln_tag_scripts_cat_id,
				'parent_cat_key' => 'cs_block_ln_tag_scripts_cat'
			),
			'ELQSTATUS'                                 => array(
				'parent_script'  => 'ln_tag',
				'expiry'         => '2 years',
				'type'           => '',
				'description'    => 'Used for tracking by ELOQUA',
				'category'       => $cs_block_ln_tag_scripts_cat,
				'category_id'    => $cs_block_ln_tag_scripts_cat_id,
				'parent_cat_key' => 'cs_block_ln_tag_scripts_cat'
			),
			'dpm'                                       => array(
				'parent_script'  => 'ln_tag',
				'expiry'         => '6 months',
				'type'           => '',
				'description'    => 'Sends events to Adobe Audience Manager',
				'category'       => $cs_block_ln_tag_scripts_cat,
				'category_id'    => $cs_block_ln_tag_scripts_cat_id,
				'parent_cat_key' => 'cs_block_ln_tag_scripts_cat'
			),
			'dextp'                                     => array(
				'parent_script'  => 'ln_tag',
				'expiry'         => '6 months',
				'type'           => '',
				'description'    => 'Records last time a data synchronization call was performed',
				'category'       => $cs_block_ln_tag_scripts_cat,
				'category_id'    => $cs_block_ln_tag_scripts_cat_id,
				'parent_cat_key' => 'cs_block_ln_tag_scripts_cat'
			),
			'demdex'                                    => array(
				'parent_script'  => 'ln_tag',
				'expiry'         => '6 months',
				'type'           => '',
				'description'    => 'Visitor identification',
				'category'       => $cs_block_ln_tag_scripts_cat,
				'category_id'    => $cs_block_ln_tag_scripts_cat_id,
				'parent_cat_key' => 'cs_block_ln_tag_scripts_cat'
			),
			'dst'                                       => array(
				'parent_script'  => 'ln_tag',
				'expiry'         => '6 months',
				'type'           => '',
				'description'    => 'Logs when there is an error sending data to destination. Used as part of Adobe Audience Manager',
				'category'       => $cs_block_ln_tag_scripts_cat,
				'category_id'    => $cs_block_ln_tag_scripts_cat_id,
				'parent_cat_key' => 'cs_block_ln_tag_scripts_cat'
			),
			'lnkd'                                      => array(
				'parent_script'  => 'ln_tag',
				'expiry'         => '6 months',
				'type'           => '',
				'description'    => 'Sends event to Adobe Audience Manager',
				'category'       => $cs_block_ln_tag_scripts_cat,
				'category_id'    => $cs_block_ln_tag_scripts_cat_id,
				'parent_cat_key' => 'cs_block_ln_tag_scripts_cat'
			),
			'aam_uuid'                                  => array(
				'parent_script'  => 'ln_tag',
				'expiry'         => '30 days',
				'type'           => '',
				'description'    => 'Set for ID sync for Adobe Audience Manager',
				'category'       => $cs_block_ln_tag_scripts_cat,
				'category_id'    => $cs_block_ln_tag_scripts_cat_id,
				'parent_cat_key' => 'cs_block_ln_tag_scripts_cat'
			),
			'MUID'                                      => array(
				'parent_script'  => 'ln_tag',
				'expiry'         => '1 year',
				'type'           => '',
				'description'    => 'Bing Tracking',
				'category'       => $cs_block_ln_tag_scripts_cat,
				'category_id'    => $cs_block_ln_tag_scripts_cat_id,
				'parent_cat_key' => 'cs_block_ln_tag_scripts_cat'
			),
			'MR'                                        => array(
				'parent_script'  => 'ln_tag',
				'expiry'         => '6 months',
				'type'           => '',
				'description'    => 'Bing Tracking',
				'category'       => $cs_block_ln_tag_scripts_cat,
				'category_id'    => $cs_block_ln_tag_scripts_cat_id,
				'parent_cat_key' => 'cs_block_ln_tag_scripts_cat'
			),
			'MR2'                                       => array(
				'parent_script'  => 'ln_tag',
				'expiry'         => '6 months',
				'type'           => '',
				'description'    => 'Bing Tracking',
				'category'       => $cs_block_ln_tag_scripts_cat,
				'category_id'    => $cs_block_ln_tag_scripts_cat_id,
				'parent_cat_key' => 'cs_block_ln_tag_scripts_cat'
			),
			'IDE'                                       => array(
				'parent_script'  => 'ln_tag',
				'expiry'         => '2 years',
				'type'           => '',
				'description'    => 'Ads targeting cookie for DoubleClick',
				'category'       => $cs_block_ln_tag_scripts_cat,
				'category_id'    => $cs_block_ln_tag_scripts_cat_id,
				'parent_cat_key' => 'cs_block_ln_tag_scripts_cat'
			),
			'personalization_id'                        => array(
				'parent_script'  => 'ln_tag',
				'expiry'         => '2 years',
				'type'           => '',
				'description'    => 'Ads targeting cookie for Twitter',
				'category'       => $cs_block_ln_tag_scripts_cat,
				'category_id'    => $cs_block_ln_tag_scripts_cat_id,
				'parent_cat_key' => 'cs_block_ln_tag_scripts_cat'
			),
			'GUC'                                       => array(
				'parent_script'  => 'ln_tag',
				'expiry'         => '5 months',
				'type'           => '',
				'description'    => 'Yahoo Conversion Tracking',
				'category'       => $cs_block_ln_tag_scripts_cat,
				'category_id'    => $cs_block_ln_tag_scripts_cat_id,
				'parent_cat_key' => 'cs_block_ln_tag_scripts_cat'
			),
			'B'                                         => array(
				'parent_script'  => 'ln_tag',
				'expiry'         => '1 year',
				'type'           => '',
				'description'    => 'Yahoo Conversion Tracking',
				'category'       => $cs_block_ln_tag_scripts_cat,
				'category_id'    => $cs_block_ln_tag_scripts_cat_id,
				'parent_cat_key' => 'cs_block_ln_tag_scripts_cat'
			),
			'_gcl_dc'                                   => array(
				'parent_script'  => 'ln_tag',
				'expiry'         => '3 months',
				'type'           => '',
				'description'    => 'Used through Google Campaign Manager and DV 360 to understand user interaction with the site and advertising',
				'category'       => $cs_block_ln_tag_scripts_cat,
				'category_id'    => $cs_block_ln_tag_scripts_cat_id,
				'parent_cat_key' => 'cs_block_ln_tag_scripts_cat'
			),
			'brwsr'                                     => array(
				'parent_script'  => 'ln_tag',
				'expiry'         => '2 years',
				'type'           => '',
				'description'    => 'Affiliate Marketing Cookie for LinkedIn',
				'category'       => $cs_block_ln_tag_scripts_cat,
				'category_id'    => $cs_block_ln_tag_scripts_cat_id,
				'parent_cat_key' => 'cs_block_ln_tag_scripts_cat'
			),
			'ABSELB'                                    => array(
				'parent_script'  => 'ln_tag',
				'expiry'         => '2 years',
				'type'           => '',
				'description'    => 'Load Balancer Cookie for affiliate marketing',
				'category'       => $cs_block_ln_tag_scripts_cat,
				'category_id'    => $cs_block_ln_tag_scripts_cat_id,
				'parent_cat_key' => 'cs_block_ln_tag_scripts_cat'
			),
			'IRLD'                                      => array(
				'parent_script'  => 'ln_tag',
				'expiry'         => '2 years',
				'type'           => '',
				'description'    => 'Affiliate Marketing Cookie for LinkedIn',
				'category'       => $cs_block_ln_tag_scripts_cat,
				'category_id'    => $cs_block_ln_tag_scripts_cat_id,
				'parent_cat_key' => 'cs_block_ln_tag_scripts_cat'
			),
			'_gac_UA-62256447-1'                        => array(
				'parent_script'  => 'ln_tag',
				'expiry'         => '3 months',
				'type'           => '',
				'description'    => 'Google Analytics cookie',
				'category'       => $cs_block_ln_tag_scripts_cat,
				'category_id'    => $cs_block_ln_tag_scripts_cat_id,
				'parent_cat_key' => 'cs_block_ln_tag_scripts_cat'
			),
			'ASPSESSIONID'                              => array(
				'parent_script'  => 'ln_tag',
				'expiry'         => 'Session',
				'type'           => '',
				'description'    => 'Used for tracking conversion during member sign up process',
				'category'       => $cs_block_ln_tag_scripts_cat,
				'category_id'    => $cs_block_ln_tag_scripts_cat_id,
				'parent_cat_key' => 'cs_block_ln_tag_scripts_cat'
			),
			'SERVERID'                                  => array(
				'parent_script'  => 'ln_tag',
				'expiry'         => 'Session',
				'type'           => '',
				'description'    => 'Load balancer cookie for Ad Pepper conversion tracking',
				'category'       => $cs_block_ln_tag_scripts_cat,
				'category_id'    => $cs_block_ln_tag_scripts_cat_id,
				'parent_cat_key' => 'cs_block_ln_tag_scripts_cat'
			),
			'tuuid'                                     => array(
				'parent_script'  => 'ln_tag',
				'expiry'         => '13 months',
				'type'           => '',
				'description'    => 'Stores randomly generated number for recurring visitors for Ad Pepper conversation tracking',
				'category'       => $cs_block_ln_tag_scripts_cat,
				'category_id'    => $cs_block_ln_tag_scripts_cat_id,
				'parent_cat_key' => 'cs_block_ln_tag_scripts_cat'
			),
			'tuuid_lu'                                  => array(
				'parent_script'  => 'ln_tag',
				'expiry'         => '12 months',
				'type'           => '',
				'description'    => 'Stores randomly generated number for recurring visitors for Ad Pepper conversation tracking',
				'category'       => $cs_block_ln_tag_scripts_cat,
				'category_id'    => $cs_block_ln_tag_scripts_cat_id,
				'parent_cat_key' => 'cs_block_ln_tag_scripts_cat'
			),
			'c'                                         => array(
				'parent_script'  => 'ln_tag',
				'expiry'         => '12 months',
				'type'           => '',
				'description'    => 'Stores a timestamp for conversion tracking',
				'category'       => $cs_block_ln_tag_scripts_cat,
				'category_id'    => $cs_block_ln_tag_scripts_cat_id,
				'parent_cat_key' => 'cs_block_ln_tag_scripts_cat'
			),
			'fl_inst'                                   => array(
				'parent_script'  => 'ln_tag',
				'expiry'         => '1 week',
				'type'           => '',
				'description'    => 'Identifies whether a browser accepts flash',
				'category'       => $cs_block_ln_tag_scripts_cat,
				'category_id'    => $cs_block_ln_tag_scripts_cat_id,
				'parent_cat_key' => 'cs_block_ln_tag_scripts_cat'
			),
			'pvc*'                                      => array(
				'parent_script'  => 'ln_tag',
				'expiry'         => '13 months',
				'type'           => '',
				'description'    => 'Used for Advertising Engagement tracking through Ad Pepper',
				'category'       => $cs_block_ln_tag_scripts_cat,
				'category_id'    => $cs_block_ln_tag_scripts_cat_id,
				'parent_cat_key' => 'cs_block_ln_tag_scripts_cat'
			),
			'pcc*'                                      => array(
				'parent_script'  => 'ln_tag',
				'expiry'         => '13 months',
				'type'           => '',
				'description'    => 'Used for Advertising Engagement tracking through Ad Pepper',
				'category'       => $cs_block_ln_tag_scripts_cat,
				'category_id'    => $cs_block_ln_tag_scripts_cat_id,
				'parent_cat_key' => 'cs_block_ln_tag_scripts_cat'
			),
			'trc'                                       => array(
				'parent_script'  => 'ln_tag',
				'expiry'         => '13 months',
				'type'           => '',
				'description'    => 'Used for Advertising Engagement tracking through Ad Pepper',
				'category'       => $cs_block_ln_tag_scripts_cat,
				'category_id'    => $cs_block_ln_tag_scripts_cat_id,
				'parent_cat_key' => 'cs_block_ln_tag_scripts_cat'
			),
			'ad2'                                       => array(
				'parent_script'  => 'ln_tag',
				'expiry'         => '13 months',
				'type'           => '',
				'description'    => 'Used for tracking ad viewability and brand safety through Ad Pepper',
				'category'       => $cs_block_ln_tag_scripts_cat,
				'category_id'    => $cs_block_ln_tag_scripts_cat_id,
				'parent_cat_key' => 'cs_block_ln_tag_scripts_cat'
			),
			'barometric[cuid]'                          => array(
				'parent_script'  => 'ln_tag',
				'expiry'         => '1 year',
				'type'           => '',
				'description'    => 'Used for Veritone/Barometric Podcast Conversion tracking on both linkedin.com (product) and microsites (business.linkedin.com)',
				'category'       => $cs_block_ln_tag_scripts_cat,
				'category_id'    => $cs_block_ln_tag_scripts_cat_id,
				'parent_cat_key' => 'cs_block_ln_tag_scripts_cat'
			),
			'tluid'                                     => array(
				'parent_script'  => 'ln_tag',
				'expiry'         => '90 days',
				'type'           => '',
				'description'    => 'Used to uniquely identify users across webpages for advertising purposes',
				'category'       => $cs_block_ln_tag_scripts_cat,
				'category_id'    => $cs_block_ln_tag_scripts_cat_id,
				'parent_cat_key' => 'cs_block_ln_tag_scripts_cat'
			),
			'queryString'                               => array(
				'parent_script'  => 'ln_tag',
				'expiry'         => '30 days',
				'type'           => '',
				'description'    => 'This cookie is used to persist marketing tracking parameters',
				'category'       => $cs_block_ln_tag_scripts_cat,
				'category_id'    => $cs_block_ln_tag_scripts_cat_id,
				'parent_cat_key' => 'cs_block_ln_tag_scripts_cat'
			),
			'SID'                                       => array(
				'parent_script'  => 'ln_tag',
				'expiry'         => 'Session',
				'type'           => '',
				'description'    => 'Used to determine what a visitor is doing before they convert on a LinkedIn microsite',
				'category'       => $cs_block_ln_tag_scripts_cat,
				'category_id'    => $cs_block_ln_tag_scripts_cat_id,
				'parent_cat_key' => 'cs_block_ln_tag_scripts_cat'
			),
			'VID'                                       => array(
				'parent_script'  => 'ln_tag',
				'expiry'         => '1 year',
				'type'           => '',
				'description'    => 'ID associated with a visitor to a LinkedIn microsite which is used to determine conversions for lead gen purposes',
				'category'       => $cs_block_ln_tag_scripts_cat,
				'category_id'    => $cs_block_ln_tag_scripts_cat_id,
				'parent_cat_key' => 'cs_block_ln_tag_scripts_cat'
			),
			'UID'                                       => array(
				'parent_script'  => 'ln_tag',
				'expiry'         => '720 days',
				'type'           => '',
				'description'    => 'Cookie used for market and user research',
				'category'       => $cs_block_ln_tag_scripts_cat,
				'category_id'    => $cs_block_ln_tag_scripts_cat_id,
				'parent_cat_key' => 'cs_block_ln_tag_scripts_cat'
			),
			'UIDR'                                      => array(
				'parent_script'  => 'ln_tag',
				'expiry'         => '720 days',
				'type'           => '',
				'description'    => 'Cookie used for market and user research',
				'category'       => $cs_block_ln_tag_scripts_cat,
				'category_id'    => $cs_block_ln_tag_scripts_cat_id,
				'parent_cat_key' => 'cs_block_ln_tag_scripts_cat'
			),
			'AMCVS_14215E3D5995C57C0A495C55%40AdobeOrg' => array(
				'parent_script'  => 'ln_tag',
				'expiry'         => 'Session',
				'type'           => '',
				'description'    => 'Indicates the start of a session for Adobe Experience Cloud',
				'category'       => $cs_block_ln_tag_scripts_cat,
				'category_id'    => $cs_block_ln_tag_scripts_cat_id,
				'parent_cat_key' => 'cs_block_ln_tag_scripts_cat'
			),
			'AMCV_14215E3D5995C57C0A495C55%40AdobeOrg'  => array(
				'parent_script'  => 'ln_tag',
				'expiry'         => '180 days',
				'type'           => '',
				'description'    => 'Unique Identifier for Adobe Experience Cloud',
				'category'       => $cs_block_ln_tag_scripts_cat,
				'category_id'    => $cs_block_ln_tag_scripts_cat_id,
				'parent_cat_key' => 'cs_block_ln_tag_scripts_cat'
			),
			's_cc'                                      => array(
				'parent_script'  => 'ln_tag',
				'expiry'         => 'Session',
				'type'           => '',
				'description'    => 'Used to determine if cookies are enabled for Adobe Analytics',
				'category'       => $cs_block_ln_tag_scripts_cat,
				'category_id'    => $cs_block_ln_tag_scripts_cat_id,
				'parent_cat_key' => 'cs_block_ln_tag_scripts_cat'
			),
			's_sq'                                      => array(
				'parent_script'  => 'ln_tag',
				'expiry'         => 'Session',
				'type'           => '',
				'description'    => 'Used to store information about the previous link that was clicked on by the user by Adobe Analytics',
				'category'       => $cs_block_ln_tag_scripts_cat,
				'category_id'    => $cs_block_ln_tag_scripts_cat_id,
				'parent_cat_key' => 'cs_block_ln_tag_scripts_cat'
			),
			's_vi'                                      => array(
				'parent_script'  => 'ln_tag',
				'expiry'         => '180 days',
				'type'           => '',
				'description'    => 'Unique identifier for Adobe Analytics',
				'category'       => $cs_block_ln_tag_scripts_cat,
				'category_id'    => $cs_block_ln_tag_scripts_cat_id,
				'parent_cat_key' => 'cs_block_ln_tag_scripts_cat'
			),
			's_fid'                                     => array(
				'parent_script'  => 'ln_tag',
				'expiry'         => '180 days',
				'type'           => '',
				'description'    => 'Unique identifier for Adobe Analytics',
				'category'       => $cs_block_ln_tag_scripts_cat,
				'category_id'    => $cs_block_ln_tag_scripts_cat_id,
				'parent_cat_key' => 'cs_block_ln_tag_scripts_cat'
			),
			'ki_r'                                      => array(
				'parent_script'  => 'ln_tag',
				'expiry'         => '5 years',
				'type'           => '',
				'description'    => 'Stores the initial page referrer when available for targeting purposes',
				'category'       => $cs_block_ln_tag_scripts_cat,
				'category_id'    => $cs_block_ln_tag_scripts_cat_id,
				'parent_cat_key' => 'cs_block_ln_tag_scripts_cat'
			),
			'ki_s'                                      => array(
				'parent_script'  => 'ln_tag',
				'expiry'         => '5 years',
				'type'           => '',
				'description'    => 'Stores the current state of any survey the user has viewed or interacted with',
				'category'       => $cs_block_ln_tag_scripts_cat,
				'category_id'    => $cs_block_ln_tag_scripts_cat_id,
				'parent_cat_key' => 'cs_block_ln_tag_scripts_cat'
			),
			'ki_t'                                      => array(
				'parent_script'  => 'ln_tag',
				'expiry'         => '5 years',
				'type'           => '',
				'description'    => 'Stores the survey timestamps and view counts',
				'category'       => $cs_block_ln_tag_scripts_cat,
				'category_id'    => $cs_block_ln_tag_scripts_cat_id,
				'parent_cat_key' => 'cs_block_ln_tag_scripts_cat'
			),
			'ki_u'                                      => array(
				'parent_script'  => 'ln_tag',
				'expiry'         => '5 years',
				'type'           => '',
				'description'    => 'Stores a random UID to associate with survey responses',
				'category'       => $cs_block_ln_tag_scripts_cat,
				'category_id'    => $cs_block_ln_tag_scripts_cat_id,
				'parent_cat_key' => 'cs_block_ln_tag_scripts_cat'
			),
			's_plt'                                     => array(
				'parent_script'  => 'ln_tag',
				'expiry'         => 'Session',
				'type'           => '',
				'description'    => 'Tracks the time that the previous page took to load',
				'category'       => $cs_block_ln_tag_scripts_cat,
				'category_id'    => $cs_block_ln_tag_scripts_cat_id,
				'parent_cat_key' => 'cs_block_ln_tag_scripts_cat'
			),
			's_tslv'                                    => array(
				'parent_script'  => 'ln_tag',
				'expiry'         => '6 months',
				'type'           => '',
				'description'    => 'Used to retain and fetch time since last visit in Adobe Analytics',
				'category'       => $cs_block_ln_tag_scripts_cat,
				'category_id'    => $cs_block_ln_tag_scripts_cat_id,
				'parent_cat_key' => 'cs_block_ln_tag_scripts_cat'
			),
			's_ppv'                                     => array(
				'parent_script'  => 'ln_tag',
				'expiry'         => 'Session',
				'type'           => '',
				'description'    => 'Used by Adobe Analytics to retain and fetch what percentage of a page was viewed',
				'category'       => $cs_block_ln_tag_scripts_cat,
				'category_id'    => $cs_block_ln_tag_scripts_cat_id,
				'parent_cat_key' => 'cs_block_ln_tag_scripts_cat'
			),
			's_pltp'                                    => array(
				'parent_script'  => 'ln_tag',
				'expiry'         => 'Session',
				'type'           => '',
				'description'    => 'Provides page name value (URL) for use by Adobe Analytics',
				'category'       => $cs_block_ln_tag_scripts_cat,
				'category_id'    => $cs_block_ln_tag_scripts_cat_id,
				'parent_cat_key' => 'cs_block_ln_tag_scripts_cat'
			),
			's_ips'                                     => array(
				'parent_script'  => 'ln_tag',
				'expiry'         => '2 years',
				'type'           => '',
				'description'    => 'Tracks percent of page viewed',
				'category'       => $cs_block_ln_tag_scripts_cat,
				'category_id'    => $cs_block_ln_tag_scripts_cat_id,
				'parent_cat_key' => 'cs_block_ln_tag_scripts_cat'
			),
			's_tp'                                      => array(
				'parent_script'  => 'ln_tag',
				'expiry'         => '2 years',
				'type'           => '',
				'description'    => 'Tracks percent of page viewed',
				'category'       => $cs_block_ln_tag_scripts_cat,
				'category_id'    => $cs_block_ln_tag_scripts_cat_id,
				'parent_cat_key' => 'cs_block_ln_tag_scripts_cat'
			),
			'liveagent_ptid'                            => array(
				'parent_script'  => 'ln_tag',
				'expiry'         => '10 years',
				'type'           => '',
				'description'    => 'Used to enable live chat functionality',
				'category'       => $cs_block_ln_tag_scripts_cat,
				'category_id'    => $cs_block_ln_tag_scripts_cat_id,
				'parent_cat_key' => 'cs_block_ln_tag_scripts_cat'
			),
			'liveagent_sid'                             => array(
				'parent_script'  => 'ln_tag',
				'expiry'         => 'Session',
				'type'           => '',
				'description'    => 'Used to enable live chat functionality',
				'category'       => $cs_block_ln_tag_scripts_cat,
				'category_id'    => $cs_block_ln_tag_scripts_cat_id,
				'parent_cat_key' => 'cs_block_ln_tag_scripts_cat'
			),
			'liveagent_vc'                              => array(
				'parent_script'  => 'ln_tag',
				'expiry'         => '10 years',
				'type'           => '',
				'description'    => 'Used to enable live chat functionality',
				'category'       => $cs_block_ln_tag_scripts_cat,
				'category_id'    => $cs_block_ln_tag_scripts_cat_id,
				'parent_cat_key' => 'cs_block_ln_tag_scripts_cat'
			),
			'sharebox-suggestion'                       => array(
				'parent_script'  => 'ln_tag',
				'expiry'         => 'Session',
				'type'           => '',
				'description'    => 'Displays a banner that provides help text to first time users of the Elevate share box',
				'category'       => $cs_block_ln_tag_scripts_cat,
				'category_id'    => $cs_block_ln_tag_scripts_cat_id,
				'parent_cat_key' => 'cs_block_ln_tag_scripts_cat'
			),
			'at_check'                                  => array(
				'parent_script'  => 'ln_tag',
				'expiry'         => 'Session',
				'type'           => '',
				'description'    => 'Used to determine if a visitor has accepted the use of cookies for Adobe Target',
				'category'       => $cs_block_ln_tag_scripts_cat,
				'category_id'    => $cs_block_ln_tag_scripts_cat_id,
				'parent_cat_key' => 'cs_block_ln_tag_scripts_cat'
			),
			'mbox'                                      => array(
				'parent_script'  => 'ln_tag',
				'expiry'         => '180 days',
				'type'           => '',
				'description'    => 'Used by Adobe Target to analyze the relevance of online content',
				'category'       => $cs_block_ln_tag_scripts_cat,
				'category_id'    => $cs_block_ln_tag_scripts_cat_id,
				'parent_cat_key' => 'cs_block_ln_tag_scripts_cat'
			),
			'recent_history'                            => array(
				'parent_script'  => 'ln_tag',
				'expiry'         => '90 days',
				'type'           => '',
				'description'    => 'Used to remember URLs visited by the guest to show the pages back where they left off',
				'category'       => $cs_block_ln_tag_scripts_cat,
				'category_id'    => $cs_block_ln_tag_scripts_cat_id,
				'parent_cat_key' => 'cs_block_ln_tag_scripts_cat'
			),
			'liveagent_chatted'                         => array(
				'parent_script'  => 'ln_tag',
				'expiry'         => '10 years',
				'type'           => '',
				'description'    => 'Used to enable live chat functionality',
				'category'       => $cs_block_ln_tag_scripts_cat,
				'category_id'    => $cs_block_ln_tag_scripts_cat_id,
				'parent_cat_key' => 'cs_block_ln_tag_scripts_cat'
			),
			'liveagent_oref'                            => array(
				'parent_script'  => 'ln_tag',
				'expiry'         => '10 years',
				'type'           => '',
				'description'    => 'Used to enable live chat functionality',
				'category'       => $cs_block_ln_tag_scripts_cat,
				'category_id'    => $cs_block_ln_tag_scripts_cat_id,
				'parent_cat_key' => 'cs_block_ln_tag_scripts_cat'
			),
			'li_cc'                                     => array(
				'parent_script'  => 'ln_tag',
				'expiry'         => '1 week',
				'type'           => '',
				'description'    => "Used to ensure a user phone number is inputted in China",
				'category'       => $cs_block_ln_tag_scripts_cat,
				'category_id'    => $cs_block_ln_tag_scripts_cat_id,
				'parent_cat_key' => 'cs_block_ln_tag_scripts_cat'
			),
			'lss_bundle_viewer'                         => array(
				'parent_script'  => 'ln_tag',
				'expiry'         => '1 month',
				'type'           => '',
				'description'    => 'Stores consent when a user agrees to view a Smartlinks link',
				'category'       => $cs_block_ln_tag_scripts_cat,
				'category_id'    => $cs_block_ln_tag_scripts_cat_id,
				'parent_cat_key' => 'cs_block_ln_tag_scripts_cat'
			),
			'interstitial_page_reg_oauth_url'           => array(
				'parent_script'  => 'ln_tag',
				'expiry'         => '1 day',
				'type'           => '',
				'description'    => 'Stores the referring page to ensure the Authentication screen displays correctly',
				'category'       => $cs_block_ln_tag_scripts_cat,
				'category_id'    => $cs_block_ln_tag_scripts_cat_id,
				'parent_cat_key' => 'cs_block_ln_tag_scripts_cat'
			),
			'tk_ai'                                     => array(
				'parent_script'  => 'tiktok',
				'expiry'         => 'session',
				'type'           => 'Statistics',
				'description'    => 'to store a unique user ID',
				'category'       => $cs_block_tiktok_scripts_cat,
				'category_id'    => $cs_block_tiktok_scripts_cat_id,
				'parent_cat_key' => 'cs_block_tiktok_scripts_cat'
			),
			'twtr_pixel_opt_in'                         => array(
				'parent_script'  => 'twitter',
				'expiry'         => 'session',
				'type'           => 'persistent',
				'description'    => '',
				'category'       => $cs_block_twitter_scripts_cat,
				'category_id'    => $cs_block_twitter_scripts_cat_id,
				'parent_cat_key' => 'cs_block_twitter_scripts_cat'
			),
		);

		return $cookies_arr;
	}

	/**
	 * Get PYS cookie patterns for it detecting
	 * @return array[]
	 */
	public function get_pys_cookies_patterns(): array {

		$pys_cat    = ConsentMagic()->getOption( 'cs_block_pys_scripts_cat' );
		$pys_cat_id = get_term_by( 'slug', $pys_cat, 'cs-cookies-category' );

		$cookies_arr = array(
			'pys_start_session'      => array(
				'parent_script'  => 'pys',
				'expiry'         => 'session',
				'type'           => 'pys',
				'description'    => 'Session start flag',
				'category'       => $pys_cat,
				'category_id'    => $pys_cat_id,
				'parent_cat_key' => 'cs_block_pys_scripts_cat'
			),
			'pys_session_limit'      => array(
				'parent_script'  => 'pys',
				'expiry'         => '60 days',
				'type'           => 'pys',
				'description'    => 'Session limit duration',
				'category'       => $pys_cat,
				'category_id'    => $pys_cat_id,
				'parent_cat_key' => 'cs_block_pys_scripts_cat'
			),
			'pbid'                   => array(
				'parent_script'  => 'pys',
				'expiry'         => '180 days',
				'type'           => 'pys',
				'description'    => 'Unique external ID',
				'category'       => $pys_cat,
				'category_id'    => $pys_cat_id,
				'parent_cat_key' => 'cs_block_pys_scripts_cat'
			),
			'pys_first_visit'        => array(
				'parent_script'  => 'pys',
				'expiry'         => '7 days',
				'type'           => 'pys',
				'description'    => 'First visit flag',
				'category'       => $pys_cat,
				'category_id'    => $pys_cat_id,
				'parent_cat_key' => 'cs_block_pys_scripts_cat'
			),
			'pysTrafficSource'       => array(
				'parent_script'  => 'pys',
				'expiry'         => '7 days',
				'type'           => 'pys',
				'description'    => 'Source URL data',
				'category'       => $pys_cat,
				'category_id'    => $pys_cat_id,
				'parent_cat_key' => 'cs_block_pys_scripts_cat'
			),
			'pys_landing_page'       => array(
				'parent_script'  => 'pys',
				'expiry'         => '7 days',
				'type'           => 'pys',
				'description'    => 'Landing page data',
				'category'       => $pys_cat,
				'category_id'    => $pys_cat_id,
				'parent_cat_key' => 'cs_block_pys_scripts_cat'
			),
			'last_pysTrafficSource'  => array(
				'parent_script'  => 'pys',
				'expiry'         => '7 days',
				'type'           => 'pys',
				'description'    => 'Last visit source URL data',
				'category'       => $pys_cat,
				'category_id'    => $pys_cat_id,
				'parent_cat_key' => 'cs_block_pys_scripts_cat'
			),
			'last_pys_landing_page'  => array(
				'parent_script'  => 'pys',
				'expiry'         => '7 days',
				'type'           => 'pys',
				'description'    => 'Last visit landing page data',
				'category'       => $pys_cat,
				'category_id'    => $pys_cat_id,
				'parent_cat_key' => 'cs_block_pys_scripts_cat'
			),
			'pys_advanced_form_data' => array(
				'parent_script'  => 'pys',
				'expiry'         => '300 days',
				'type'           => 'pys',
				'description'    => 'Event advanced form data',
				'category'       => $pys_cat,
				'category_id'    => $pys_cat_id,
				'parent_cat_key' => 'cs_block_pys_scripts_cat'
			),
			'pys_fb_event_id'        => array(
				'parent_script'  => 'pys',
				'expiry'         => 'session',
				'type'           => 'pys',
				'description'    => 'Facebook event ID',
				'category'       => $pys_cat,
				'category_id'    => $pys_cat_id,
				'parent_cat_key' => 'cs_block_pys_scripts_cat'
			),
			'select_prod_list'       => array(
				'parent_script'  => 'pys',
				'expiry'         => '1 day',
				'type'           => 'pys',
				'description'    => 'Product list for Woocommerce',
				'category'       => $pys_cat,
				'category_id'    => $pys_cat_id,
				'parent_cat_key' => 'cs_block_pys_scripts_cat'
			),
			'productlist'            => array(
				'parent_script'  => 'pys',
				'expiry'         => '1 day',
				'type'           => 'pys',
				'description'    => 'Product list data Woocommmerce add to cart',
				'category'       => $pys_cat,
				'category_id'    => $pys_cat_id,
				'parent_cat_key' => 'cs_block_pys_scripts_cat'
			),
			'last_pys_'              => array(
				'parent_script'  => 'pys',
				'expiry'         => '7 days',
				'type'           => 'pys',
				'description'    => 'Data for other last UTM parameters',
				'category'       => $pys_cat,
				'category_id'    => $pys_cat_id,
				'parent_cat_key' => 'cs_block_pys_scripts_cat'
			),
			'hide_tag_'              => array(
				'parent_script'  => 'pys',
				'expiry'         => 'variable',
				'type'           => 'pys',
				'description'    => 'Flag to hide a tag',
				'category'       => $pys_cat,
				'category_id'    => $pys_cat_id,
				'parent_cat_key' => 'cs_block_pys_scripts_cat'
			),
			'pys'                    => array(
				'parent_script'  => 'pys',
				'expiry'         => 'variable',
				'type'           => 'pys',
				'description'    => 'Data for other UTM parameters',
				'category'       => $pys_cat,
				'category_id'    => $pys_cat_id,
				'parent_cat_key' => 'cs_block_pys_scripts_cat'
			)
		);

		return $cookies_arr;
	}

	/**
	 * Insert the scanned Scripts to the corresponding table
	 * @param int   $scan_id      scan Id.
	 * @param array $scripts_data scanned scripts.
	 * @param       $category_id
	 * @return void
	 */
	protected function insert_scripts( int $scan_id, array $scripts_data, $category_id ): void {
		global $wpdb;
		$script_table   = $wpdb->prefix . $this->scripts_table;
		$script_name    = isset( $scripts_data[ 'script_name' ] ) ? esc_sql( sanitize_text_field( $scripts_data[ 'script_name' ] ) ) : '';
		$script_slug    = isset( $scripts_data[ 'script_slug' ] ) ? esc_sql( sanitize_text_field( $scripts_data[ 'script_slug' ] ) ) : '';
		$created_at     = gmdate( 'Y-m-d H:i:s' );
		$script_body    = isset( $scripts_data[ 'script_body' ] ) ? esc_sql( esc_js( $scripts_data[ 'script_body' ] ) ) : '';
		$description    = isset( $scripts_data[ 'description' ] ) ? esc_sql( sanitize_textarea_field( $scripts_data[ 'description' ] ) ) : '';
		$parent_cat_key = isset( $scripts_data[ 'parent_cat_key' ] ) ? esc_sql( sanitize_text_field( $scripts_data[ 'parent_cat_key' ] ) ) : '';
		$script_enabled = (int) ( isset( $scripts_data[ 'script_enabled' ] ) ? esc_sql( sanitize_text_field( $scripts_data[ 'script_enabled' ] ) ) : 1 );

		$sql       = "INSERT IGNORE INTO `$script_table` ( `id_cs_scan`,`script_name`, `script_slug`, `category_id`,`script_body`,`script_parent_cat_key`,`description`, `script_enabled`, `created_at` ) VALUES ";
		$sql_arr[] = $wpdb->prepare( "(%s,%s,%s,%s,%s,%s,%s,%d,%s)", $scan_id, $script_name, $script_slug, $category_id, $script_body, $parent_cat_key, $description, $script_enabled, $created_at );
		$sql       = $sql . implode( ',', $sql_arr )
		             . $wpdb->prepare( " ON DUPLICATE KEY UPDATE id_cs_scan = %s, script_name = %s, script_slug = %s, category_id = %s, script_body = %s, script_parent_cat_key = %s, description = %s, script_enabled = %d", $scan_id, $script_name, $script_slug, $category_id, $script_body, $parent_cat_key, $description, $script_enabled );
		$wpdb->query( $sql );
	}

	/**
	 * Update scan PYS category when it changed
	 * @param $category
	 * @return void
	 */
	public function update_scan_pys_category( $category ): void {
		global $wpdb;
		$cookie_table = $wpdb->prefix . $this->cookies_table;
		$sql          = $wpdb->prepare( 'SELECT * FROM `%1$s`', $cookie_table );
		$cookies      = $wpdb->get_results( $sql, ARRAY_A );
		$term_cat     = get_term_by( 'slug', $category, 'cs-cookies-category' );
		if ( !empty( $cookies ) && $term_cat ) {
			foreach ( $cookies as $cookie ) {
				if ( $cookie[ 'type' ] == 'pys' ) {
					$cookie[ 'category' ]    = $category;
					$cookie[ 'category_id' ] = $term_cat->term_id;
					$this->insert_cookies( $cookie[ 'id_cs_scan' ], $cookie );
				}
			}
		}
	}
}

new CS_Scanner_Module();