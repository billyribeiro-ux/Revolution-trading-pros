<?php

namespace ConsentMagicPro;

if ( !defined( 'ABSPATH' ) ) {
	exit; // Exit if accessed directly.
}

use ConsentMagicPro\GuzzleHttp\Cookie\CookieJar;

class CS_Scanner extends CS_Scanner_Module {

	public $version;

	public function __construct() {

		// Cache scanner tables status to avoid duplicate checks
		$this->tables_ready = (bool) ConsentMagic()->getOption( 'cs_check_scanner_tables' );

		add_action( 'wp_ajax_cs_scanner', array(
			$this,
			'ajax_scanner'
		) );

	}

	/**
	 * Ajax callback which handles the ajax calls from scanner.
	 * @return void
	 */
	public function ajax_scanner() {
		if ( !current_user_can( 'manage_cs' ) ) {
			wp_die( esc_html__( 'You do not have sufficient permission to perform this operation', 'consent-magic' ) );
		}
		check_ajax_referer( 'cs_scanner', 'security' );

		$out = array(
			'response' => false,
			'message'  => esc_html__( 'Unable to handle your request.', 'consent-magic' ),
		);
		if ( isset( $_POST[ 'cs_scanner_action' ] ) ) {

			$cs_scan_action = sanitize_text_field( wp_unslash( $_POST[ 'cs_scanner_action' ] ) );

			$allowed_actions = array(
				'cs_get_pages',
				'scan_pages',
				'cs_bulk_scan',
			);

			if ( in_array( $cs_scan_action, $allowed_actions, true ) && method_exists( $this, $cs_scan_action ) ) {
				$out = $this->{$cs_scan_action}();
			}
		}
		echo wp_json_encode( $out );
		exit();
	}

	/**
	 * Retrieves pages from the site for scanning
	 * @return array
	 */

	public function cs_get_pages() {

		check_ajax_referer( 'cs_scanner', 'security' );

		$cs_scan_existing_page = ConsentMagic()->getOption( 'cs_auto_scan_type' );

		if ( $cs_scan_existing_page === 'scan_all_pages' ) {
			$out = $this->get_scan_all_pages();
		} else {
			$out = $this->get_scan_key_pages();
		}

		return $out;
	}

	public function get_scan_key_pages() {
		global $wpdb;
		$page_limit = 10;
		$out        = array(
			'log'     => array(),
			'total'   => 0,
			'scan_id' => 0,
			'status'  => false,
		);
		$sql        = $this->get_scan_pages_query();

		$sql_new  = $wpdb->prepare( "SELECT post_name,post_title,post_type,ID,guid $sql ORDER BY post_date DESC LIMIT %d", $page_limit );
		$sql_old  = $wpdb->prepare( "SELECT post_name,post_title,post_type,ID,guid $sql ORDER BY post_date ASC LIMIT %d", $page_limit );
		$sql_rand = $wpdb->prepare( "SELECT post_name,post_title,post_type,ID,guid $sql ORDER BY RAND() LIMIT %d", $page_limit );

		$data_new  = $wpdb->get_results( $sql_new, ARRAY_A ); // phpcs:ignore WordPress.DB.DirectDatabaseQuery,WordPress.DB.PreparedSQL.NotPrepared
		$data_old  = $wpdb->get_results( $sql_old, ARRAY_A ); // phpcs:ignore WordPress.DB.DirectDatabaseQuery,WordPress.DB.PreparedSQL.NotPrepared
		$data_rand = $wpdb->get_results( $sql_rand, ARRAY_A ); // phpcs:ignore WordPress.DB.DirectDatabaseQuery,WordPress.DB.PreparedSQL.NotPrepared

		if ( !empty( $data_new ) ) {
			foreach ( $data_new as $value ) {
				$out[ 'urls' ][] = get_permalink( $value[ 'ID' ] );
			}
		}

		if ( !empty( $data_old ) ) {
			foreach ( $data_old as $value ) {
				$out[ 'urls' ][] = get_permalink( $value[ 'ID' ] );
			}
		}

		if ( !empty( $data_rand ) ) {
			foreach ( $data_rand as $value ) {
				$out[ 'urls' ][] = get_permalink( $value[ 'ID' ] );
			}
		}

		$total = 0;
		if ( !empty( $out[ 'urls' ] ) ) {
			$out[ 'urls' ] = array_unique( $out[ 'urls' ] );
			$total         = count( $out[ 'urls' ] );
		}

		$scan_id = $this->create_scan_entry( $total );

		$data_arr = array(
			'current_action' => 'cs_get_pages',
			'status'         => 1,
			'total_url'      => $total,
		);

		$this->update_scan_entry( $data_arr, $scan_id );
		$out[ 'status' ] = true;

		$out[ 'scan_id' ] = $scan_id;
		$out[ 'total' ]   = $total;

		return $out;
	}

	public function get_scan_all_pages() {
		global $wpdb;
		$page_limit = 100;

		$out = array(
			'log'     => array(),
			'total'   => 0,
			'limit'   => $page_limit,
			'scan_id' => 0,
			'status'  => false,
		);

		$sql = $this->get_scan_pages_query();

		$sql = $wpdb->prepare( "SELECT post_name,post_title,post_type,ID,guid $sql ORDER BY post_type='page' DESC LIMIT %d", $page_limit );

		$data = $wpdb->get_results( $sql, ARRAY_A ); // phpcs:ignore WordPress.DB.DirectDatabaseQuery,WordPress.DB.PreparedSQL.NotPrepared

		if ( !empty( $data ) ) {
			foreach ( $data as $value ) {
				$out[ 'urls' ][] = get_permalink( $value[ 'ID' ] );
			}
		}

		$total = 0;
		if ( !empty( $out[ 'urls' ] ) ) {
			$out[ 'urls' ] = array_unique( $out[ 'urls' ] );
			$total         = count( $out[ 'urls' ] );
		}

		$scan_id          = $this->create_scan_entry( $total );
		$out[ 'scan_id' ] = $scan_id;
		$out[ 'total' ]   = $total;

		$data_arr = array(
			'current_action' => 'cs_get_pages',
			'status'         => 1,
			'total_url'      => $total,
		);

		$this->update_scan_entry( $data_arr, $scan_id );
		$out[ 'status' ] = true;
		$out[ 'total' ]  = $total;

		return $out;
	}

	public function cs_get_all_url( $page_limit = null, $page_offset = 0 ) {

		global $wpdb;
		$sql    = $this->get_scan_pages_query();
		$offset = $wpdb->prepare( '%d', (int) $page_offset );
		$limit  = $wpdb->prepare( '%d', (int) $page_limit );

		$limits = $limit > 0 ? 'LIMIT ' . $limit . ' OFFSET ' . $offset : '';

		if ( $page_limit ) {
			$sql = 'SELECT post_name,post_title,post_type,ID,guid' . $sql . " ORDER BY post_type='page' DESC $limits";
		} else {
			$sql = 'SELECT post_name,post_title,post_type,ID,guid' . $sql . " ORDER BY post_type='page' DESC ";
		}
		$out  = array();
		$data = $wpdb->get_results( $sql, ARRAY_A ); // phpcs:ignore WordPress.DB.DirectDatabaseQuery,WordPress.DB.PreparedSQL.NotPrepared

		if ( !empty( $data ) ) {
			foreach ( $data as $value ) {
				$permalink = get_permalink( $value[ 'ID' ] );
				if ( strpos( $permalink, '?' ) !== false ) {
					$out[] = $permalink . '&cs_scan_pages=1';
				} else {
					$out[] = $permalink . '?cs_scan_pages=1';
				}
			}
		}

		return $out;
	}

	/**
	 * Returns the current host
	 * @param string $url URL of a page or post.
	 * @return string
	 */
	private function cs_get_host( $url ) {
		$parsed_url = wp_parse_url( $url );
		$site_host  = isset( $parsed_url[ 'host' ] ) ? $parsed_url[ 'host' ] : '';

		return $site_host;
	}

	/**
	 * Returns the query to get the pages to be scanned
	 * @return string
	 */
	public function get_scan_pages_query() {
		global $wpdb;

		$post_table = $wpdb->prefix . 'posts';
		$post_types = get_post_types( array(
			'public' => true,
		) );
		unset( $post_types[ 'attachment' ] );
		unset( $post_types[ 'revision' ] );
		unset( $post_types[ 'custom_css' ] );
		unset( $post_types[ 'customize_changeset' ] );
		unset( $post_types[ 'user_request' ] );

		$sql = " FROM $post_table WHERE post_type IN('" .
		       implode( "','", $post_types ) .
		       "') AND post_status='publish' ";

		return $sql;
	}

	/**
	 * Perform a bulk scan request.
	 * @return void
	 */
	public function cs_bulk_scan() {

		check_ajax_referer( 'cs_scanner', 'security' );

		$scan_id = isset( $_POST[ 'scan_id' ] ) ? sanitize_text_field( wp_unslash( $_POST[ 'scan_id' ] ) ) : 0;

		$total = isset( $_POST[ 'total' ] ) ? sanitize_text_field( wp_unslash( $_POST[ 'total' ] ) ) : 0;

		$cs_scan_url_per_request = ConsentMagic()->getOption( 'cs_scan_url_per_request' );

		$page_offset = isset( $_POST[ 'page_offset' ] ) ? sanitize_text_field( wp_unslash( $_POST[ 'page_offset' ] ) ) : 0;

		if ( $page_offset == 0 ) {
			global $wpdb;
			$script_table = $wpdb->prefix . $this->scripts_table;
			$update       = $wpdb->prepare( 'UPDATE `%1$s` SET `script_enabled` = 0', $script_table );
			$wpdb->query( $update );

			$cookie_table = $wpdb->prefix . $this->cookies_table;
			$update       = $wpdb->prepare( 'UPDATE `%1$s` SET `cookie_enabled` = 0', $cookie_table );
			$wpdb->query( $update );
		}

		$urls = $this->cs_get_all_url( $cs_scan_url_per_request, $page_offset );
		$data = array();

		$needle = $this->cs_scripts_pattern();
		$needle = $this->get_custom_scripts_list( $needle );

		$matches         = array();
		$cookies_pattern = $this->get_cookies_pattern();
		$cookie_data     = array();
		$script_data     = array();

		if ( isPYSActivated() ) {
			$pys = $this->cs_get_pys_pattent();

			foreach ( $pys as $key => $val ) {
				switch ( $key ) {
					case 'pys_pinterest' :
						$configured_pixel = isPinterestActive() == '1' &&
						                    function_exists( 'PixelYourSite\Pinterest' ) &&
						                    \PixelYourSite\Pinterest()->configured();
						break;
					case 'pys_bing' :
						$configured_pixel = isBingActive() == '1' &&
						                    function_exists( 'PixelYourSite\Bing' ) &&
						                    \PixelYourSite\Bing()->configured();
						break;
					case 'pys_tiktok' :
						$configured_pixel = function_exists( 'PixelYourSite\Tiktok' ) &&
						                    \PixelYourSite\Tiktok()->configured();
						break;
					case 'pys_facebook' :
						$configured_pixel = function_exists( 'PixelYourSite\Facebook' ) &&
						                    \PixelYourSite\Facebook()->configured();
						break;
					case 'pys_ga' :
						$configured_pixel = function_exists( 'PixelYourSite\GA' ) && \PixelYourSite\GA()->configured();
						break;
					case 'pys_google_ads' :
						$configured_pixel = function_exists( 'PixelYourSite\Ads' ) &&
						                    \PixelYourSite\Ads()->configured();
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
						$matches = array_merge( $matches, $matches_fonts[ 0 ] );
					}
					if ( !empty( $matches_data ) ) {
						$matches = array_merge( $matches, $matches_data[ 0 ] );
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
									'value'          => trim( sanitize_text_field( $cookie->getValue() ) ),
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
								$option_key = isset( $val[ 'option_key' ] ) ? 'cs_block_' .
								                                              $val[ 'option_key' ] .
								                                              '_scripts_cat' : $val[ 'option_name' ];
								$term       = get_term_by( 'slug', ConsentMagic()->getOption( $option_key ), 'cs-cookies-category' );
								if ( !$term ) {
									$option_key = isset( $val[ 'script_id' ] ) ? 'cs_' .
									                                             $val[ 'script_id' ] .
									                                             '_script_cat' : $val[ 'option_key' ];
									$term       = get_term_by( 'slug', ConsentMagic()->getOption( $option_key ), 'cs-cookies-category' );
								}
								if ( isset( $val[ 'script_custom' ] ) ) {
									$script_data[] = array(
										'script_name'    => $key,
										'script_slug'    => $val[ 'option_key' ],
										'script_body'    => 'custom',
										'category_id'    => $term->term_id,
										'description'    => isset( $val[ 'description' ] ) ? $val[ 'description' ] : '',
										'parent_cat_key' => isset( $val[ 'parent_cat_key' ] ) ? $val[ 'parent_cat_key' ] : '',
										'script_enabled' => 1,
									);
								} else {
									$script_data[] = array(
										'script_name'    => $key,
										'script_slug'    => $val[ 'option_key' ],
										'script_body'    => $value,
										'category_id'    => $term->term_id,
										'description'    => isset( $val[ 'description' ] ) ? $val[ 'description' ] : '',
										'parent_cat_key' => isset( $val[ 'parent_cat_key' ] ) ? $val[ 'parent_cat_key' ] : '',
										'script_enabled' => 1,
									);
								}
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
						} else {
							foreach ( $val[ 'data' ] as $sub_val ) {
								$check = strripos( $value, $sub_val );
								if ( $check !== false ) {
									$option_key = isset( $val[ 'option_key' ] ) ? 'cs_block_' .
									                                              $val[ 'option_key' ] .
									                                              '_scripts_cat' : $val[ 'option_name' ];
									$term       = get_term_by( 'slug', ConsentMagic()->getOption( $option_key ), 'cs-cookies-category' );
									if ( !$term ) {
										$option_key = isset( $val[ 'script_id' ] ) ? 'cs_' .
										                                             $val[ 'script_id' ] .
										                                             '_script_cat' : $val[ 'option_key' ];
										$term       = get_term_by( 'slug', ConsentMagic()->getOption( $option_key ), 'cs-cookies-category' );
									}
									if ( isset( $val[ 'script_custom' ] ) ) {
										$script_data[] = array(
											'script_name'    => $key,
											'script_slug'    => $val[ 'option_key' ],
											'script_body'    => 'custom',
											'category_id'    => $term->term_id,
											'description'    => isset( $val[ 'description' ] ) ? $val[ 'description' ] : '',
											'parent_cat_key' => isset( $val[ 'parent_cat_key' ] ) ? $val[ 'parent_cat_key' ] : '',
											'script_enabled' => 1
										);
									} else {
										$script_data[] = array(
											'script_name'    => $key,
											'script_slug'    => $val[ 'option_key' ],
											'script_body'    => $value,
											'category_id'    => $term->term_id,
											'description'    => isset( $val[ 'description' ] ) ? $val[ 'description' ] : '',
											'parent_cat_key' => isset( $val[ 'parent_cat_key' ] ) ? $val[ 'parent_cat_key' ] : '',
											'script_enabled' => 1
										);
									}
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
					}
				}
			}

			$cookie_data = $this->unique_multidim_array( $cookie_data, 'cookie_name' );
		}


		$old_cookie  = $this->get_scan_cookies( 0, 0, false );
		$bind_data   = array(
			'description',
			'category'
		);
		$cookie_data = $this->bind_data( $old_cookie, $cookie_data, $bind_data, 'cookie_name' );
		$this->save_cookie_data( $cookie_data );

		$this->save_script_data( $script_data );

		$data_arr = array(
			'current_action' => 'cs_bulk_scan',
			'current_offset' => -1,
			'status'         => 1,
		);
		$this->update_scan_entry( $data_arr, $scan_id );
		$page_offset_new = $page_offset + $cs_scan_url_per_request;

		if ( $page_offset_new < $total ) {
			$data[ 'page_offset' ] = $page_offset + $cs_scan_url_per_request;
		} else {
			$data[ 'page_offset' ] = 'false';
		}

		$data[ 'scan_id' ] = $scan_id;
		$data[ 'total' ]   = $total;

		$data[ 'title' ]   = esc_html__( 'Scanning initiated successfully', 'consent-magic' );
		$data[ 'message' ] = esc_html__( 'It might take a few minutes to a few hours to complete the scanning of your website. This depends on the number of pages to scan and the website speed. Once the scanning is complete, we will notify you by email.', 'consent-magic' );
		$data[ 'html' ]    = esc_html__( 'Scanning initiated successfully', 'consent-magic' );

		wp_send_json_success( $data );
	}

	public function cs_scripts_pattern() {

		$wp_content = str_replace( ABSPATH, '', WP_PLUGIN_DIR );

		return array(
			'Hotjar'                    => array(
				'option_key'     => 'hotjar',
				'data'           => 'static.hotjar.com/c/hotjar-',
				'description'    => ConsentMagic()->getOption( 'cs_block_hotjar_scripts_descr' ),
				'parent_cat_key' => 'cs_block_hotjar_scripts_cat',
				'parent_script'  => 'hotjar'
			),
			'Instagram'                 => array(
				'option_key'     => 'instagram',
				'data'           => array(
					'www.instagram.com/embed.js',
					'www.instagram.com/embed.js',
					'api.instagram.com/oembed',
					'src:www.instagram.com/p'
				),
				'description'    => ConsentMagic()->getOption( 'cs_block_instagram_scripts_descr' ),
				'parent_cat_key' => 'cs_block_instagram_scripts_cat',
				'parent_script'  => 'instagram'
			),
			'Facebook Pixel'            => array(
				'option_key'     => 'fb_pixel',
				'data'           => array(
					'connect.facebook.net/en_US/fbevents.js',
					'connect.facebook.net/signals/config/',
					'connect.facebook.net/signals/plugins/',
					'fjs',
					'facebook-jssdk',
					'connect.facebook.net/en_US/fbevents.js',
					'connect.facebook.net/signals/config/',
					'fjs',
					'facebook-jssdk',
					'connect.facebook.net/en_US/fbevents.js',
					'connect.facebook.net/signals/config/',
					'connect.facebook.net/signals/plugins/',
					'fbq',
					'fjs',
					'facebook-jssdk',
					'src:facebook.com/tr',
					'connect.facebook.net'
				),
				'description'    => ConsentMagic()->getOption( 'cs_block_fb_pixel_scripts_descr' ),
				'parent_cat_key' => 'cs_block_fb_pixel_scripts_cat',
				'parent_script'  => 'fb_pixel'
			),
			'Google Analytics'          => array(
				'option_key'     => 'google_analytics',
				'data'           => array(
					'www.googletagmanager.com/ns.html?id=GTM-',
					$wp_content . '/woocommerce-google-adwords-conversion-tracking-tag/js/public/google_ads.js',
					'googletagmanager.com/gtag/js',
					'www.google-analytics.com/analytics.js',
					'www.google-analytics.com/analytics.js',
					'google-analytics.com/ga.js',
					'www.googletagmanager.com/gtm',
					'stats.g.doubleclick.net/dc.js',
					'window.ga=window.ga',
					'_getTracker',
					'__gaTracker',
					'wooptpmDataLayer',
					'gtag',
					'GoogleAnalyticsObject'
				),
				'description'    => ConsentMagic()->getOption( 'cs_block_google_analytics_scripts_descr' ),
				'parent_cat_key' => 'cs_block_google_analytics_scripts_cat',
				'parent_script'  => 'google_analytics'
			),
			'Google Publisher Tag'      => array(
				'option_key'     => 'google_ads_tag',
				'data'           => array(
					'www.googletagmanager.com/ns.html?id=GTM-',
					'www.googletagservices.com/tag/js/gpt.js',
					'www.googleadservices.com/pagead/conversion.js',
					$wp_content . '/woocommerce-google-adwords-conversion-tracking-tag/js/public/google_ads.js',
					'googletagmanager.com/gtag/js',
					'googletag.pubads',
					'googletag.enableServices',
					'googletag.display',
					'wooptpmDataLayer',
					'www.googletagservices.com/tag/js/gpt.js',
					'www.googleadservices.com/pagead/conversion.js',
					'www.googletagmanager.com/gtm',
					'gtag',
					'src:pubads.g.doubleclick.net/gampad',
					'src:googleads.g.doubleclick.net/pagead'
				),
				'description'    => ConsentMagic()->getOption( 'cs_block_google_tag_manager_scripts_descr' ),
				'parent_cat_key' => 'cs_block_google_tag_manager_scripts_cat',
				'parent_script'  => 'google_ads_tag'
			),
			'Google Adsense'            => array(
				'option_key'     => 'google_adsense',
				'data'           => array(
					'pagead2.googlesyndication.com/pagead/js/adsbygoogle.js',
					'adsbygoogle.js'
				),
				'description'    => ConsentMagic()->getOption( 'cs_block_google_adsense_scripts_descr' ),
				'parent_cat_key' => 'cs_block_google_adsense_scripts_cat',
				'parent_script'  => 'google_adsense'
			),
			'Google maps'               => array(
				'option_key'     => 'google_maps',
				'data'           => array(
					'maps.googleapis.com/maps/api',
					'maps.googleapis.com/maps/api',
					'google.map',
					'initMap',
					'src:www.google.com/maps/embed',
					'src:maps.google.com/maps'
				),
				'description'    => ConsentMagic()->getOption( 'cs_block_google_maps_scripts_descr' ),
				'parent_cat_key' => 'cs_block_google_maps_scripts_cat',
				'parent_script'  => 'google_maps'
			),
			'Google fonts'              => array(
				'option_key'     => 'googlefonts',
				'data'           => array(
					'fonts.googleapis.com',
					'https://fonts.googleapis.com',
				),
				'description'    => ConsentMagic()->getOption( 'cs_block_googlefonts_scripts_descr' ),
				'parent_cat_key' => 'cs_block_googlefonts_scripts_cat',
				'parent_script'  => 'googlefonts'
			),
			'Twitter widget'            => array(
				'option_key'     => 'tw_tag',
				'data'           => array(
					'platform.twitter.com/widgets.js',
					'platform.twitter.com/widgets.js',
					'twitter-wjs',
					'twttr.widgets',
					'twttr.events',
					'twttr.ready',
					'window.twttr',
					'twq'
				),
				'description'    => ConsentMagic()->getOption( 'cs_block_tw_tag_scripts_descr' ),
				'parent_cat_key' => 'cs_block_tw_tag_scripts_cat',
				'parent_script'  => 'tw_tag'
			),
			'Twitter Pixel'             => array(
				'option_key'     => 'twitter',
				'data'           => array(
					'static.ads-twitter.com'
				),
				'description'    => ConsentMagic()->getOption( 'cs_block_twitter_scripts_descr' ),
				'parent_cat_key' => 'cs_block_twitter_scripts_cat',
				'parent_script'  => 'twitter'
			),
			'Microsoft UET'             => array(
				'option_key'     => 'big_tag',
				'data'           => array(
					'bat.bing.com/bat.js',
					'bat.bing.com/bat.js',
					'window.uetq=window.uetq',
				),
				'description'    => ConsentMagic()->getOption( 'cs_block_big_tag_scripts_descr' ),
				'parent_cat_key' => 'cs_block_big_tag_scripts_cat',
				'parent_script'  => 'big_tag'
			),
			'Linkedin widget/Analytics' => array(
				'option_key'     => 'ln_tag',
				'data'           => array(
					'platform.linkedin.com/in.js',
					'platform.linkedin.com/in.js',
					'snap.licdn.com/li.lms-analytics/insight.min.js',
					'_linkedin_partner_id',
					'src:dc.ads.linkedin.com/collect/'
				),
				'description'    => ConsentMagic()->getOption( 'cs_block_ln_tag_scripts_descr' ),
				'parent_cat_key' => 'cs_block_ln_tag_scripts_cat',
				'parent_script'  => 'ln_tag'
			),
			'Pinterest widget'          => array(
				'option_key'     => 'pin_tag',
				'data'           => array(
					'assets.pinterest.com/js/pinit.js',
					'assets.pinterest.com/js/pinit.js',
					'pintrk',
					'window.pintrk',
					's.pinimg.com/ct/core.js',
					'src:ct.pinterest.com/v3/'
				),
				'description'    => ConsentMagic()->getOption( 'cs_block_pin_tag_scripts_desc' ),
				'parent_cat_key' => 'cs_block_pin_tag_scripts_cat',
				'parent_script'  => 'pin_tag'
			),
			'Youtube embed'             => array(
				'option_key'     => 'yt_embedded',
				'data'           => array(
					'www.youtube.com/embed',
					'www.youtube.com/player_api',
					'www.youtube.com/player_api',
					'onYouTubePlayerAPIReady',
					'YT.Player',
					'onYouTubeIframeAPIReady',
					'www.youtube.com/iframe_api',
					'src:www.youtube.com/embed',
					'src:youtu.be',
					'data:www.youtube.com/embed',
					'src:www.youtube.com/embed',
					'src:www.youtube.com/embed'
				),
				'description'    => ConsentMagic()->getOption( 'cs_block_yt_embedded_scripts_descr' ),
				'parent_cat_key' => 'cs_block_yt_embedded_scripts_cat',
				'parent_script'  => 'yt_embedded'
			),
			'Vimeo embed'               => array(
				'option_key'     => 'vimeo_embedded',
				'data'           => array(
					'player.vimeo.com/api/player.js',
					'www.vimeo.com/api/oembed',
					'player.vimeo.com/api/player.js',
					'Vimeo.Player',
					'new Player',
					'src:player.vimeo.com/video',
					'player.vimeo.com/video'
				),
				'description'    => ConsentMagic()->getOption( 'cs_block_vimeo_embedded_scripts_descr' ),
				'parent_cat_key' => 'cs_block_vimeo_embedded_scripts_cat',
				'parent_script'  => 'vimeo_embedded'
			),
			'Hubspot Analytics'         => array(
				'option_key'     => 'hubspot',
				'data'           => 'js.hs-scripts.com',
				'description'    => ConsentMagic()->getOption( 'cs_block_hubspot_scripts_descr' ),
				'parent_cat_key' => 'cs_block_hubspot_scripts_cat',
				'parent_script'  => 'hubspot'
			),
			'Matomo Analytics'          => array(
				'option_key'     => 'matomo',
				'data'           => array(
					'matomo.js',
					'_paq.push',
					'_mtm.push'
				),
				'description'    => ConsentMagic()->getOption( 'cs_block_matomo_scripts_descr' ),
				'parent_cat_key' => 'cs_block_matomo_scripts_cat',
				'parent_script'  => 'matomo'
			),
			'Addthis widget'            => array(
				'option_key'     => 'addthis',
				'data'           => array(
					's7.addthis.com/js',
					'addthis_widget'
				),
				'description'    => ConsentMagic()->getOption( 'cs_block_addthis_scripts_descr' ),
				'parent_cat_key' => 'cs_block_addthis_scripts_cat',
				'parent_script'  => 'addthis'
			),
			'Sharethis widget'          => array(
				'option_key'     => 'sharethis',
				'data'           => array(
					'platform-api.sharethis.com/js/sharethis.js',
					'sharethis.js'
				),
				'description'    => ConsentMagic()->getOption( 'cs_block_sharethis_scripts_descr' ),
				'parent_cat_key' => 'cs_block_sharethis_scripts_cat',
				'parent_script'  => 'sharethis'
			),
			'Soundcloud embed'          => array(
				'option_key'     => 'soundcloud',
				'data'           => array(
					'connect.soundcloud.com',
					'SC.initialize',
					'SC.get',
					'SC.connectCallback',
					'SC.connect',
					'SC.put',
					'SC.stream',
					'SC.Recorder',
					'SC.upload',
					'SC.oEmbed',
					'soundcloud.com',
					'src:w.soundcloud.com/player',
					'src:api.soundcloud.com'
				),
				'description'    => ConsentMagic()->getOption( 'cs_block_soundcloud_scripts_descr' ),
				'parent_cat_key' => 'cs_block_soundcloud_scripts_cat',
				'parent_script'  => 'soundcloud'
			),
			'Slideshare embed'          => array(
				'option_key'     => 'slideshare',
				'data'           => array(
					'www.slideshare.net/api/oembed',
					'src:www.slideshare.net/slideshow'
				),
				'description'    => ConsentMagic()->getOption( 'cs_block_slideshare_scripts_descr' ),
				'parent_cat_key' => 'cs_block_slideshare_scripts_cat',
				'parent_script'  => 'slideshare'
			),
			'Tik-tok pixel'             => array(
				'option_key'     => 'tiktok',
				'data'           => array(
					'analytics.tiktok.com/i18n/pixel/',
					'ttq.load',
					'ttq.page',
					'ttq',
					'w.TiktokAnalyticsObject',
					'ttq.methods',
					'ttq.instance',
				),
				'description'    => ConsentMagic()->getOption( 'cs_block_tiktok_scripts_descr' ),
				'parent_cat_key' => 'cs_block_tiktok_scripts_cat',
				'parent_script'  => 'tiktok'
			),

			'Google captcha' => array(
				'option_key'     => 'google_captcha',
				'data'           => array(
					'gstatic.com/recaptcha',
					'google.com/recaptcha',
				),
				'description'    => ConsentMagic()->getOption( 'cs_block_google_captcha_scripts_descr' ),
				'parent_cat_key' => 'cs_block_google_captcha_scripts_cat',
				'parent_script'  => 'google_captcha'
			),

			'Google captcha Contact form 7' => array(
				'option_key'     => 'google_captcha',
				'data'           => array(
					$wp_content . '/contact-form-7/modules/recaptcha/index.js',
				),
				'description'    => ConsentMagic()->getOption( 'cs_block_google_captcha_scripts_descr' ),
				'parent_cat_key' => 'cs_block_google_captcha_scripts_cat',
				'parent_script'  => 'google_captcha'
			),

			'Reddit pixel'          => array(
				'option_key'     => 'reddit_tag',
				'data'           => array(
					'redditstatic.com/ads/pixel.js',
				),
				'description'    => ConsentMagic()->getOption( 'cs_block_reddit_pixel_scripts_descr' ),
				'parent_cat_key' => 'cs_block_reddit_pixel_scripts_cat',
				'parent_script'  => 'reddit_tag'
			),
		);
	}

	public function cs_get_pys_pattent() {

		return array(
			'pys_tiktok'     => array(
				'script_name'    => 'TikTok',
				'option_key'     => 'pys_tiktok',
				'parent_cat_key' => 'cs_block_tiktok_scripts_cat',
				'description'    => ConsentMagic()->getOption( 'cs_block_tiktok_scripts_descr' ),
				'parent_script'  => 'tiktok'
			),
			'pys_facebook'   => array(
				'script_name'    => 'Facebook Pixel',
				'option_key'     => 'pys_facebook',
				'parent_cat_key' => 'cs_block_fb_pixel_scripts_cat',
				'description'    => ConsentMagic()->getOption( 'cs_block_fb_pixel_scripts_descr' ),
				'parent_script'  => 'fb_pixel'
			),
			'pys_ga'         => array(
				'script_name'    => 'Google Analytics',
				'option_key'     => 'pys_ga',
				'parent_cat_key' => 'cs_block_google_analytics_scripts_cat',
				'description'    => ConsentMagic()->getOption( 'cs_block_google_analytics_scripts_descr' ),
				'parent_script'  => 'google_analytics'
			),
			'pys_google_ads' => array(
				'script_name'    => 'Google Ads Tag',
				'option_key'     => 'pys_google_ads',
				'parent_cat_key' => 'cs_block_google_adsense_scripts_cat',
				'description'    => ConsentMagic()->getOption( 'cs_block_google_adsense_scripts_descr' ),
				'parent_script'  => 'google_ads_tag'
			),
			'pys_bing'       => array(
				'script_name'    => 'Microsoft UET',
				'option_key'     => 'pys_bing',
				'parent_cat_key' => 'cs_block_big_tag_scripts_cat',
				'description'    => ConsentMagic()->getOption( 'cs_block_big_tag_scripts_descr' ),
				'parent_script'  => 'big_tag'
			),
			'pys_pinterest'  => array(
				'script_name'    => 'Pinterest Tag',
				'option_key'     => 'pys_pinterest',
				'parent_cat_key' => 'cs_block_pin_tag_scripts_cat',
				'description'    => ConsentMagic()->getOption( 'cs_block_pin_tag_scripts_desc' ),
				'parent_script'  => 'pin_tag'
			),
			'pys_reddit'  => array(
				'script_name'    => 'Reddit Tag',
				'option_key'     => 'pys_reddit',
				'parent_cat_key' => 'cs_block_reddit_pixel_scripts_cat',
				'description'    => ConsentMagic()->getOption( 'cs_block_reddit_pixel_scripts_descr' ),
				'parent_script'  => 'reddit'
			),
		);
	}

	public function get_custom_scripts_list( $needle ) {

		global $wpdb;
		$querystr  = $wpdb->prepare( "SELECT * FROM $wpdb->posts WHERE $wpdb->posts.post_type = %s ", 'cs-scripts' );
		$pageposts = $wpdb->get_results( $querystr );

		foreach ( $pageposts as $script ) {
			$needle[ "$script->post_title" ] = array(
				'option_name'    => 'cs_' . $script->ID . '_script_cat',
				'option_key'     => 'cs_' . $script->ID . '_script_cat',
				'data'           => explode( ",", get_post_meta( $script->ID, 'cs_default_script_js_heedle', true ) ),
				'script_id'      => $script->ID,
				'script_custom'  => 'custom',
				'parent_script'  => '',
				'parent_cat_key' => '',
				'description'    => get_post_meta( $script->ID, 'cs_default_script_desc', true ),
			);
		}

		return $needle;
	}

	public function unique_multidim_array( $array, $key ) {
		$temp_array = array();
		$i          = 0;
		$key_array  = array();

		if ( !empty( $array ) ) {
			foreach ( $array as $val ) {
				if ( !in_array( $val[ $key ], $key_array ) ) {
					$key_array[ $i ]  = $val[ $key ];
					$temp_array[ $i ] = $val;
				}
				$i++;
			}
		}

		return $temp_array;
	}

	/**
	 * Bind data to new array data
	 * @param $array_old
	 * @param $array_new
	 * @param $bind_data
	 * @param $key
	 * @return mixed
	 */
	public function bind_data( $array_old, $array_new, $bind_data, $key ) {
		if ( !empty( $array_new ) && !empty( $array_old ) ) {
			$array_keys = array_column( $array_old, $key );
			foreach ( $array_new as &$val ) {
				$i = array_search( $val[ $key ], $array_keys );
				if ( $i !== false ) {
					foreach ( $bind_data as $data ) {
						$val[ $data ] = $array_old[ $i ][ $data ];
					}
				}
			}
			unset( $val );
		}

		return $array_new;
	}

	/**
	 * Save cookie data to cookies table
	 * @param array $cookie_data Array of data.
	 * @return \WP_Error|void
	 */
	public function save_cookie_data( $cookie_data ) {

		$scan_id = $this->get_last_scan_id();

		if ( $cookie_data ) {

			if ( $scan_id !== false ) {

				foreach ( $cookie_data as $data ) {
					$cookies = $data;
					if ( !empty( $cookies ) ) {
						$this->insert_cookies( $scan_id, $cookies );
					}
				}
			}
		}
		$total_cookies = $this->get_last_scan_new_cookies( $scan_id );
		$data_arr      = array(
			'current_action' => 'cs_bulk_scan',
			'total_cookies'  => $total_cookies
		);
		$this->update_scan_entry( $data_arr, $scan_id );
	}

	/**
	 * Save script data to scripts table
	 * @param array $script_data Array of data.
	 * @return void
	 */
	public function save_script_data( array $script_data ) {

		$scan_id = $this->get_last_scan_id();

		if ( !empty( $script_data ) ) {
			if ( $scan_id !== false ) {
				foreach ( $script_data as $data ) {
					$scripts     = $data;
					$category_id = ( $data[ 'category_id' ] ?? '' );
					if ( !empty( $scripts ) ) {
						$this->insert_scripts( $scan_id, $scripts, $category_id );
					}
				}
			}
		}
		$total_scripts = $this->get_last_scan_new_scripts( $scan_id );
		$data_arr      = array(
			'current_action' => 'cs_bulk_scan',
			'total_scripts'  => $total_scripts
		);
		$this->update_scan_entry( $data_arr, $scan_id );
	}

	/**
	 * Filters the URL
	 * @param string $permalink Permalink of a page or post.
	 * @return string
	 */
	public function filter_url( $permalink ) {
		$url_arr = explode( '/', $permalink );
		$end     = trim( end( $url_arr ) );
		if ( '' !== $end ) {
			$url_end_arr = explode( '.', $end );
			if ( count( $url_end_arr ) > 1 ) {
				$end_end = trim( end( $url_end_arr ) );
				if ( $end_end != '' ) {
					$allowed = array(
						'html',
						'htm',
						'shtml',
						'php'
					);
					if ( !in_array( $end_end, $allowed, true ) ) {
						return false;
					}
				}
			}
		}

		return true;
	}
}

new CS_Scanner();