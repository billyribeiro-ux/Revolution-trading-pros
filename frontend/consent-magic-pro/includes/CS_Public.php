<?php

/**
 * The public-facing functionality of the plugin.
 * @link       https://www.pixelyoursite.com/plugins/consentmagic/
 * @since      1.0.0
 * @package    ConsentMagic
 * @subpackage ConsentMagic/includes
 */

namespace ConsentMagicPro;

class CS_Public {

	use CS_Throttle;

	public static $cookie_list_arr = null;

	public static $disable = null;

	public $url;

	public $post_id;

	public $post_content;

	public $page_content;

	private $cache;

	private $current_lang;

	/**
	 * Initialize the class and set its properties.
	 */
	public function __construct( $cache ) {

		$this->page_content = '';

		if ( (int) $cache === 1 ) {
			$this->cache = true;
		} else {
			$this->cache = false;
		}

		add_action( 'woocommerce_new_order', array(
			$this,
			'cs_save_order_consent'
		) );

		add_filter( 'edd_payment_meta', array(
			$this,
			'cs_save_edd_order_consent'
		) );

		if ( wp_doing_ajax() ) {
			add_action( 'wp_ajax_preview_shortcode_show', array(
				$this,
				'cs_preview_shortcode_show'
			) );
			add_action( 'wp_ajax_nopriv_preview_shortcode_show', array(
				$this,
				'cs_preview_shortcode_show'
			) );

			add_action( 'wp_ajax_insert_proof', array(
				$this,
				'cs_insert_proof'
			) );
			add_action( 'wp_ajax_nopriv_insert_proof', array(
				$this,
				'cs_insert_proof'
			) );

			add_action( 'wp_ajax_proof_show_count_update', array(
				$this,
				'cs_proof_show_count_update'
			) );
			add_action( 'wp_ajax_nopriv_proof_show_count_update', array(
				$this,
				'cs_proof_show_count_update'
			) );
			add_action( 'wp_ajax_nopriv_cs_get_active_data', array(
				$this,
				'cs_get_active_data'
			) );
			add_action( 'wp_ajax_cs_get_active_data', array(
				$this,
				'cs_get_active_data'
			) );
			add_action( 'wp_ajax_nopriv_cs_get_active_vendor_list', array(
				$this,
				'cs_get_active_vendor_list'
			) );
			add_action( 'wp_ajax_cs_get_active_vendor_list', array(
				$this,
				'cs_get_active_vendor_list'
			) );
			add_action( 'wp_ajax_nopriv_cs_update_sticky_button', array(
				$this,
				'cs_update_sticky_button'
			) );
			add_action( 'wp_ajax_cs_update_sticky_button', array(
				$this,
				'cs_update_sticky_button'
			) );
		}

		//Add exclusion for banner for WPRocket
		add_action( 'init', array(
			$this,
			'add_wprocket_exclusions'
		) );

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
		$show = $this->check_disable();
		if ( $show ) {
			$term = get_term_by( 'slug', 'googlefonts', 'cs-cookies-category' );
			if ( $term ) {
				$googlefonts_enabled = get_term_meta( $term->term_id, 'cs_ignore_this_category', true );
			} else {
				$googlefonts_enabled = '1';
			}

			wp_enqueue_style(
				'cm-noto-sans-font',
				'https://fonts.googleapis.com/css2?family=Noto+Sans:ital,wdth,wght@0,62.5..100,100..900;1,62.5..100,100..900&display=swap',
				[],
				null
			);

			wp_enqueue_style(
				ConsentMagic()->plugin_name,
				CMPRO_PLUGIN_URL . "assets/css/style-public.min.css",
				array(),
				ConsentMagic()->get_version(),
				'all'
			);
			if ( $googlefonts_enabled === '0' || $googlefonts_enabled === '' ) {
				wp_enqueue_style(
					ConsentMagic()->plugin_name . '-font-css',
					CMPRO_PLUGIN_URL . "assets/fonts/" . ConsentMagic()->getOption( 'default_font' )
					. "/stylesheet.css",
					array(),
					ConsentMagic()->get_version(),
					'all'
				);
				if ( (int) ConsentMagic()->getOption( 'cs_enable_site_cache' ) === 1 ) {
					add_filter( 'body_class', array( $this, 'add_body_class' ) );
				}
			}
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
		if ( !is_admin() ) {
			$show = $this->check_disable();
			if ( $show ) {
				$cs_options = $this->get_plugin_options();

				if ( $cs_options[ 'cs_plugin_activation' ] ) {
					$in_footer = false;
					if ( is_active_supreme_modules_pro_for_divi() ) {
						global $wp_scripts;
						if ( array_key_exists( 'dsm-typed', $wp_scripts->registered )
						     && array_key_exists( 'dsm-typing-effect', $wp_scripts->registered ) ) {
							$in_footer = true;
						}
					}

					wp_enqueue_script(
						ConsentMagic()->plugin_name,
						CMPRO_PLUGIN_URL . 'assets/scripts/cs-public.min.js',
						array( 'jquery' ),
						time(),
						$in_footer
					);
					wp_localize_script( ConsentMagic()->plugin_name, 'CS_Data', $this->get_cs_data() );
					wp_localize_script( ConsentMagic()->plugin_name, 'cs_log_object', array(
						'ajaxurl' => admin_url( 'admin-ajax.php' ),
					) );
				}
			}
		}
	}

	public function get_cs_data() {

		$cs_user_default_language = ConsentMagic()->getOption( 'cs_user_default_language' );
		if ( !$this->current_lang ) {
			$this->current_lang = get_locale();

			$cs_language_availability = ConsentMagic()->getOption( 'cs_language_availability' );
			if ( isset( $cs_language_availability[ $this->current_lang ] )
			     && $cs_language_availability[ $this->current_lang ] == 0 ) {
				$this->current_lang = $cs_user_default_language;
			}
		}

		$show            = $this->check_disable();
		$cs_cookie_datas = array();
		if ( $show ) {
			$cs_options = $this->get_plugin_options();

			if ( $cs_options[ 'cs_plugin_activation' ] ) {
				global $sitepress;
				$wp_current_lang = explode( '_', $this->current_lang );
				$wp_current_lang = $wp_current_lang[ 0 ];
				if ( function_exists( 'icl_object_id' ) && $sitepress ) {
					$wp_current_lang = apply_filters( 'wpml_current_language', NULL );
				}

				$translations = array(
					'cs_iab_name'                => '',
					'cs_iab_domain'              => '',
					'cs_iab_purposes'            => '',
					'cs_iab_expiry'              => '',
					'cs_iab_type'                => '',
					'cs_iab_cookie_details'      => '',
					'cs_iab_years'               => '',
					'cs_iab_months'              => '',
					'cs_iab_days'                => '',
					'cs_iab_hours'               => '',
					'cs_iab_minutes'             => '',
					'cs_iab_legitimate_interest' => '',
					'cs_iab_privacy_policy'      => '',
					'cs_iab_special_purposes'    => '',
					'cs_iab_features'            => '',
					'cs_iab_special_features'    => '',
					'cs_iab_data_categories'     => '',
					'cs_iab_storage_methods'     => '',
					'cs_iab_cookies_and_others'  => '',
					'cs_iab_other_methods'       => '',
					'cs_iab_consent_preferences' => '',
					'cs_iab_cookie_refreshed'    => '',
					'cs_iab_show_cookie_details' => '',
					'cs_iab_nodata'              => '',
					'cs_iab_btn_text_allow'      => '',
				);

				foreach ( $translations as $key => $translation ) {
					if ( ConsentMagic()->getOption( 'cs_enable_translations' ) == 1 ) {
						$translations[ $key ] = ConsentMagic()->getLangOption( $key, $this->current_lang );
						if ( empty( $translations[ $key ] ) ) {
							$translations[ $key ] = ConsentMagic()->getLangOption( $key, $cs_user_default_language );
							if ( empty( $translations[ $key ] ) ) {
								$translations[ $key ] = ConsentMagic()->getLangOption( $key, CMPRO_DEFAULT_LANGUAGE );
							}
						}
					} else {
						$translations[ $key ] = ConsentMagic()->getLangOption( $key, CMPRO_DEFAULT_LANGUAGE );
					}
				}

				$cs_cookie_list           = $this->get_cookie_list();
				$cookies_by_category      = $this->get_cookie_by_category();
				$non_necessary_cookie_ids = $this->get_non_necessary_cookie_ids();
				$ajax_nonce               = wp_create_nonce( "cs-ajax-public-nonce" );
				$expire                   = ConsentMagic()->getOption( 'cs_expire_days' );
				if ( $expire == 0 ) {
					$expire = 18250;
				}
				$necessary_cat_id    = ConsentMagic()->getOption( 'necessary_cat_id' );
				$active_rule_id      = ConsentMagic()->get_active_rule_id();
				$cs_active_rule_name = ( !empty( $active_rule_id ) ) ? get_post_meta(
					$active_rule_id,
					'_cs_type',
					true
				) : 'false';
				if ( isFbWooActivated() && !isPYSActivated()
				     && (int) ConsentMagic()->getOption( 'cs_fb_woo_capi_enabled' ) === 1
				     && (int) ConsentMagic()->getOption( 'cs_enable_site_cache' ) === 0
				     && $cs_active_rule_name != 'just_inform' ) {
					$refresh_after_consent = 1;
				} else {
					$refresh_after_consent = get_post_meta( $active_rule_id, '_cs_refresh_after_consent', true );
				}

				$cs_run_scripts   = ConsentMagic()->ready_to_run() && !empty( $active_rule_id ) ? 1 : 0;
				$blocking_enabled = $cs_run_scripts && ConsentMagic()->getOption( 'cs_block_pre_defined_scripts' )
				                    && ConsentMagic()->getOption( 'cs_plugin_activation' )
				                    && ConsentMagic()->getOption( 'cs_script_blocking_enabled' );

				$iab = array(
					'enabled'        => ( CS_IAB_Integration()->enabled() && $cs_run_scripts ) ? 1 : 0,
					'native_scripts' => get_post_meta( $active_rule_id, '_cs_native_scripts', true ),
				);
				if ( $iab[ 'enabled' ] ) {
					$current_iab_lang = CS_IAB_Integration()->get_current_iab_lang();
					$iab              = array_merge( $iab, array(
						'cmp_id'                    => CMPRO_CMP_ID,
						'cmp_version'               => CMPRO_CMP_VERSION,
						'legitimate_purposes'       => CS_IAB_Integration()->get_legitimate_purposes(),
						'active_vendors'            => CS_IAB_Integration()->get_active_vendors(),
						'active_additional_vendors' => CS_IAB_Integration()->get_active_additional_vendors(),
						'current_iab_lang'          => $current_iab_lang,
						'vendor_list_version'       => CS_IAB_Integration()->get_vendor_list_version(),
						'vendor_list_url'           => CMPRO_PLUGIN_URL
						                               . 'includes/modules/iab_integration/json/vendor_list_'
						                               . $current_iab_lang . '.json'
					) );
				}

				// Options for url_passthrough and Meta Limited Data Use
				if ( isPYSActivated() && function_exists( '\PixelYourSite\GA' ) ) {

					//Passthrough
					$pys_filter_enabled = \PixelYourSite\GA()->getOption( 'url_passthrough_filter' );
					if ( $pys_filter_enabled ) {
						$pys_url_passthrough_mode = has_filter( 'pys_url_passthrough_mode' );
					} else {
						$pys_url_passthrough_mode = false;
					}

					$url_passthrough_mode             = apply_filters( 'cm_url_passthrough_mode', array(
						'enabled' => '',
						'value'   => '',
					) );
					$url_passthrough_mode[ 'filter' ] = $pys_url_passthrough_mode;
					$url_passthrough_mode[ 'value' ]  = $pys_url_passthrough_mode ? apply_filters(
						'pys_url_passthrough_mode',
						\PixelYourSite\GA()->getOption( 'url_passthrough' )
					) : $url_passthrough_mode[ 'value' ];

				} else {
					$url_passthrough_mode = array(
						'filter'  => false,
						'value'   => false,
						'enabled' => false,
					);
				}

				$youtube_cat = get_term_by(
					'slug',
					ConsentMagic()->getOption( 'cs_block_yt_embedded_scripts_cat' ),
					'cs-cookies-category'
				)->term_id;
				$cache       = ( (int) ConsentMagic()->getOption( 'cs_enable_site_cache' ) === 1 ) ? 1 : 0;

				if ( ConsentMagic()->ready_to_run() && CS_Integrations::check_showing() ) {
					$block_embedded_video = CS_Script_Blocker()->checkBlockingEmbeddedVideo(
						$youtube_cat,
						$active_rule_id,
						$cache
					);
				} else {
					$block_embedded_video = true;
				}

				$conversion_exporter = array(
					'load_cookies' => apply_filters( 'cnvex_cookies_consent', true ),
				);

				$cs_cookie_datas = array(
					'nn_cookie_ids'                             => !empty( $non_necessary_cookie_ids ) ? $non_necessary_cookie_ids : array(),
					'non_necessary_cookies'                     => !empty( $cookies_by_category ) ? $cookies_by_category : array(),
					'cookielist'                                => !empty( $cs_cookie_list ) ? $cs_cookie_list : array(),
					'ajax_url'                                  => admin_url( 'admin-ajax.php' ),
					'current_lang'                              => $wp_current_lang,
					'security'                                  => $ajax_nonce,
					'consentVersion'                            => ConsentMagic()->getOption(
						'cs_test_mode'
					) ? ConsentMagic()->cs_get_consent_version_test() : ConsentMagic()->cs_get_consent_version(),
					'cs_cookie_domain'                          => ConsentMagic()->getOption(
						'cs_cross_domain_tracking'
					) ? ConsentMagic()->getOption( 'cs_cross_domain_tracking_domain' ) : '',
					'privacy_length'                            => apply_filters( 'cs_privacy_overview_length', 250 ),
					'cs_expire_days'                            => $expire,
					'cs_script_cat'                             => array(
						'facebook'            => ( $blocking_enabled
						                           && ConsentMagic()->getOption(
								'cs_block_fb_pixel_scripts'
							) ) ? get_term_by(
							'slug',
							ConsentMagic()->getOption( 'cs_block_fb_pixel_scripts_cat' ),
							'cs-cookies-category'
						)->term_id : 0,
						'analytics'           => ( $blocking_enabled
						                           && ConsentMagic()->getOption(
								'cs_block_google_analytics_scripts'
							) ) ? get_term_by(
							'slug',
							ConsentMagic()->getOption( 'cs_block_google_analytics_scripts_cat' ),
							'cs-cookies-category'
						)->term_id : 0,
						'gads'                => ( $blocking_enabled
						                           && ConsentMagic()->getOption(
								'cs_block_google_ads_tag_scripts'
							) ) ? get_term_by(
							'slug',
							ConsentMagic()->getOption( 'cs_block_google_ads_tag_scripts_cat' ),
							'cs-cookies-category'
						)->term_id : 0,
						'pinterest'           => ( $blocking_enabled
						                           && ConsentMagic()->getOption(
								'cs_block_pin_tag_scripts'
							) ) ? get_term_by(
							'slug',
							ConsentMagic()->getOption( 'cs_block_pin_tag_scripts_cat' ),
							'cs-cookies-category'
						)->term_id : 0,
						'bing'                => ( $blocking_enabled
						                           && ConsentMagic()->getOption(
								'cs_block_big_tag_scripts'
							) ) ? get_term_by(
							'slug',
							ConsentMagic()->getOption( 'cs_block_big_tag_scripts_cat' ),
							'cs-cookies-category'
						)->term_id : 0,
						'adsense'             => ( $blocking_enabled
						                           && ConsentMagic()->getOption(
								'cs_block_google_adsense_scripts'
							) ) ? get_term_by(
							'slug',
							ConsentMagic()->getOption( 'cs_block_google_adsense_scripts_cat' ),
							'cs-cookies-category'
						)->term_id : 0,
						'hubspot'             => ( $blocking_enabled
						                           && ConsentMagic()->getOption(
								'cs_block_hubspot_scripts'
							) ) ? get_term_by(
							'slug',
							ConsentMagic()->getOption( 'cs_block_hubspot_scripts_cat' ),
							'cs-cookies-category'
						)->term_id : 0,
						'matomo'              => ( $blocking_enabled
						                           && ConsentMagic()->getOption(
								'cs_block_matomo_scripts'
							) ) ? get_term_by(
							'slug',
							ConsentMagic()->getOption( 'cs_block_matomo_scripts_cat' ),
							'cs-cookies-category'
						)->term_id : 0,
						'maps'                => ( $blocking_enabled
						                           && ConsentMagic()->getOption(
								'cs_block_google_maps_scripts'
							) ) ? get_term_by(
							'slug',
							ConsentMagic()->getOption( 'cs_block_google_maps_scripts_cat' ),
							'cs-cookies-category'
						)->term_id : 0,
						'addthis'             => ( $blocking_enabled
						                           && ConsentMagic()->getOption(
								'cs_block_addthis_scripts'
							) ) ? get_term_by(
							'slug',
							ConsentMagic()->getOption( 'cs_block_addthis_scripts_cat' ),
							'cs-cookies-category'
						)->term_id : 0,
						'sharethis'           => ( $blocking_enabled
						                           && ConsentMagic()->getOption(
								'cs_block_sharethis_scripts'
							) ) ? get_term_by(
							'slug',
							ConsentMagic()->getOption( 'cs_block_sharethis_scripts_cat' ),
							'cs-cookies-category'
						)->term_id : 0,
						'soundcloud'          => ( $blocking_enabled
						                           && ConsentMagic()->getOption(
								'cs_block_soundcloud_scripts'
							) ) ? get_term_by(
							'slug',
							ConsentMagic()->getOption( 'cs_block_soundcloud_scripts_cat' ),
							'cs-cookies-category'
						)->term_id : 0,
						'slideshare'          => ( $blocking_enabled
						                           && ConsentMagic()->getOption(
								'cs_block_slideshare_scripts'
							) ) ? get_term_by(
							'slug',
							ConsentMagic()->getOption( 'cs_block_slideshare_scripts_cat' ),
							'cs-cookies-category'
						)->term_id : 0,
						'instagram'           => ( $blocking_enabled
						                           && ConsentMagic()->getOption(
								'cs_block_instagram_scripts'
							) ) ? get_term_by(
							'slug',
							ConsentMagic()->getOption( 'cs_block_instagram_scripts_cat' ),
							'cs-cookies-category'
						)->term_id : 0,
						'hotjar'              => ( $blocking_enabled
						                           && ConsentMagic()->getOption(
								'cs_block_hotjar_scripts'
							) ) ? get_term_by(
							'slug',
							ConsentMagic()->getOption( 'cs_block_hotjar_scripts_cat' ),
							'cs-cookies-category'
						)->term_id : 0,
						'tiktok'              => ( $blocking_enabled
						                           && ConsentMagic()->getOption(
								'cs_block_tiktok_scripts'
							) ) ? get_term_by(
							'slug',
							ConsentMagic()->getOption( 'cs_block_tiktok_scripts_cat' ),
							'cs-cookies-category'
						)->term_id : 0,
						'twitter'             => ( $blocking_enabled
						                           && ConsentMagic()->getOption(
								'cs_block_twitter_scripts'
							) ) ? get_term_by(
							'slug',
							ConsentMagic()->getOption( 'cs_block_twitter_scripts_cat' ),
							'cs-cookies-category'
						)->term_id : 0,
						'youtube'             => ( $blocking_enabled
						                           && ConsentMagic()->getOption(
								'cs_block_yt_embedded_scripts'
							) ) ? $youtube_cat : 0,
						'googlefonts'         => ( $blocking_enabled
						                           && ConsentMagic()->getOption(
								'cs_block_googlefonts_scripts'
							) ) ? get_term_by(
							'slug',
							ConsentMagic()->getOption( 'cs_block_googlefonts_scripts_cat' ),
							'cs-cookies-category'
						)->term_id : 0,
						'google_captcha'      => ( $blocking_enabled
						                           && ConsentMagic()->getOption(
								'cs_block_google_captcha_scripts'
							) ) ? get_term_by(
							'slug',
							ConsentMagic()->getOption( 'cs_block_google_captcha_scripts_cat' ),
							'cs-cookies-category'
						)->term_id : 0,
						'reddit'              => ( $blocking_enabled
						                           && ConsentMagic()->getOption(
								'cs_block_reddit_pixel_scripts'
							) ) ? get_term_by(
							'slug',
							ConsentMagic()->getOption( 'cs_block_reddit_pixel_scripts_cat' ),
							'cs-cookies-category'
						)->term_id : 0,
						'pys'                 => ( !empty( ConsentMagic()->getOption( 'cs_block_pys_scripts' ) )
						                           && !empty(
							ConsentMagic()->getOption(
								'cs_block_pre_defined_scripts'
							)
							) ) ? get_term_by(
							'slug',
							ConsentMagic()->getOption( 'cs_block_pys_scripts_cat' ),
							'cs-cookies-category'
						)->term_id : 0,
						'conversion_exporter' => ( !empty( ConsentMagic()->getOption( 'cs_block_ce_scripts' ) )
						                           && !empty(
							ConsentMagic()->getOption(
								'cs_block_pre_defined_scripts'
							)
							) ) ? get_term_by(
							'slug',
							ConsentMagic()->getOption( 'cs_block_ce_scripts_cat' ),
							'cs-cookies-category'
						)->term_id : 0,

					),
					'cs_proof_expire'                           => ( ConsentMagic()->getOption(
							'cs_stored_consent_for'
						) == '' ) ? $expire : ConsentMagic()->getOption(
						'cs_stored_consent_for'
					),
					'cs_default_close_on_scroll'                => ConsentMagic()->getOption(
						'cs_default_close_on_scroll'
					),
					'cs_track_analytics'                        => $cs_options[ 'cs_track_analytics' ],
					'test_prefix'                               => $cs_options[ 'test_prefix' ],
					'cs_refresh_after_consent'                  => $refresh_after_consent,
					'cs_consent_for_pys'                        => ( !isPYSActivated()
					                                                 || empty(
					                                                 ConsentMagic()->getOption(
						                                                 'cs_consent_for_pys'
					                                                 )
					                                                 ) ) ? 0 : 1,
					'cs_track_before_consent_expressed_for_pys' => ( !isPYSActivated()
					                                                 || empty(
					                                                 ConsentMagic()->getOption(
						                                                 'cs_track_before_consent_expressed_for_pys'
					                                                 )
					                                                 )
					                                                 || empty(
					                                                 ConsentMagic()->getOption(
						                                                 'cs_consent_for_pys'
					                                                 )
					                                                 ) ) ? 0 : 1,
					'cs_video_placeholder_text'                 => wp_kses_post(
						apply_filters( 'cm_video_placeholder', '' )
					),
					'cs_google_consent_mode'                    => CS_Google_Consent_Mode()->get_google_consent_mode(),
					'cs_google_consent_mode_enabled'            => CS_Google_Consent_Mode(
					)->enabled_google_consent_mode(),
					'cs_bing_consent_mode'                      => CS_Bing_Consent_Mode()->get_bing_consent_mode(),
					'cs_bing_consent_mode_enabled'              => CS_Bing_Consent_Mode()->enabled_bing_consent_mode(),
					'cs_reddit_ldu_mode'                        => (int) apply_filters( 'pys_reddit_ldu_mode', 0 ),
					'cs_url_passthrough_mode'                   => $url_passthrough_mode,
					'cs_meta_ldu_mode'                          => (int) apply_filters( 'pys_meta_ldu_mode', 0 ),
					'cs_block_video_personal_data'              => ConsentMagic()->getOption(
						'cs_block_video_personal_data'
					),
					'cs_necessary_cat_id'                       => $necessary_cat_id ? $necessary_cat_id : 0,
					'cs_cache_label'                            => time(),
					'cs_cache_enabled'                          => $cache,
					'cs_active_rule'                            => $active_rule_id,
					'cs_active_rule_name'                       => $cs_active_rule_name,
					'cs_showing_rule_until_express_consent'     => (int) get_post_meta(
						$active_rule_id,
						'_cs_showing_rule_until_express_consent',
						true
					),
					'cs_minimum_recommended'                    => ConsentMagic()->get_minimum_recommended_options(),
					'cs_deny_consent_for_close'                 => get_post_meta(
						$active_rule_id,
						'_cs_deny_consent_for_close',
						true
					),
					'cs_run_scripts'                            => $cs_run_scripts,
					'cs_iab'                                    => $iab,
					'cs_translations'                           => $translations,
					'cs_design_type'                            => get_post_meta(
						$active_rule_id,
						'_cs_design_type',
						true
					),
					'cs_embedded_video'                         => array(
						'block' => $blocking_enabled && ConsentMagic()->getOption( 'cs_block_yt_embedded_scripts' )
						           && $block_embedded_video,
					),
					'cs_wp_consent_api'                         => CS_WP_Consent_Api()->getSettings(),
					'cs_conversion_exporter'                    => $conversion_exporter,
					'version'                                   => CMPRO_LATEST_VERSION_NUMBER
				);
			}
		}

		return $cs_cookie_datas;
	}

	public function get_plugin_options() {

		return ConsentMagic()->cs_options;
	}

	function cs_insert_proof() {

		check_ajax_referer( 'cs-ajax-public-nonce', 'nonce_code' );

		//check throttle
		if ( !$this->checkThrottle( 'cs_insert_proof' ) ) {
			wp_send_json_error();
			wp_die();
		}

		$cs_email_before_delete_consent = ConsentMagic()->getOption( 'cs_email_before_delete_consent' );
		$rule_id                        = isset( $_POST[ 'rule' ] ) ? sanitize_text_field( $_POST[ 'rule' ] ) : null;
		$excluded_from_consent_storing  = get_post_meta( $rule_id, '_excluded_from_consent_storing', true );
		$cs_type                        = isset( $_POST[ 'cs_type' ] ) ? sanitize_text_field(
			$_POST[ 'cs_type' ]
		) : null;
		global $wpdb;
		$table          = $wpdb->prefix . 'cs_proof_consent';
		$terms_primary  = get_cookies_terms_objects( 'cs_primary_term', true );
		$terms          = get_cookies_terms_objects( 'cs_primary_term' );
		$current_action = isset( $_POST[ 'user_react' ] ) ? sanitize_text_field( $_POST[ 'user_react' ] ) : null;
		$url            = isset( $_POST[ 'url' ] ) ? esc_url_raw( $_POST[ 'url' ] ) : null;
		$cs_type_db     = get_cs_type_name( $cs_type );
		$current_action = get_current_action( $current_action );
		$native_scripts = get_post_meta( $rule_id, '_cs_native_scripts', true );
		$category       = '';

		if ( $cs_type == 'iab' && CS_Cookies()->getCookie( 'cs_consent_string' ) ) {
			try {
				$iab_string  = json_decode(
					base64_decode( CS_Cookies()->getCookie( 'cs_consent_string' ) ),
					false
				);
				$purposes    = $iab_string->purposes;
				$purposes    = str_split( $purposes );
				$vendor_list = CS_IAB_Integration()->get_vendor_list();

				foreach ( $purposes as $i => $purpose ) {
					if ( $purpose == 1 ) {
						if ( isset( $vendor_list->purposes->{(int) $i + 1} ) ) {
							$category .= '<span class="table_cat">' . $vendor_list->purposes->{(int) $i + 1}->name
							             . '</span> ';
						}
					}
				}
			} catch ( \Exception $e ) {
				error_log( 'Error: ' . $e->getMessage() );
			}
		}

		if ( $cs_type != 'iab' || $native_scripts == 1 ) {
			$category .= '<span class="table_cat">Necessary</span> ';

			if ( !empty( $terms_primary ) ) {
				foreach ( $terms_primary as $term ) {
					$category_cookie = "cs_enabled_cookie_term_" . $term->term_id;
					$ignore          = (int) get_term_meta( $term, 'cs_ignore_this_category', true );
					if ( ( CS_Cookies()->getCookie( $category_cookie ) && CS_Cookies()->getCookie( $category_cookie ) == 'yes'
					       && $ignore == 0 ) ) {
						$category .= '<span class="table_cat">' . $term->name . '</span> ';
					}
				}
			}

			if ( !empty( $terms ) ) {
				foreach ( $terms as $term ) {
					$category_cookie = "cs_enabled_cookie_term_" . $term->term_id;
					$ignore          = (int) get_term_meta( $term, 'cs_ignore_this_category', true );
					if ( ( CS_Cookies()->getCookie( $category_cookie ) && CS_Cookies()->getCookie( $category_cookie ) == 'yes'
					       && $ignore == 0 ) ) {
						$category .= '<span class="table_cat">' . $term->name . '</span> ';
					}
				}
			}
		}

		$data_arr_statistics = array(
			'current_action' => $current_action,
			'consent_type'   => $cs_type_db,
			'rule'           => get_the_title( $rule_id ),
			'rule_id'        => $rule_id,
			'created_at'     => current_time( 'mysql' ),
		);

		$cs_check_tables = ConsentMagic()->getOption( 'cs_check_proof_tables' );

		if ( !$cs_check_tables ) {
			$this->check_tables();
			$cs_check_tables = true;
		}

		if ( $cs_check_tables ) {
			$wpdb->insert( $wpdb->prefix . 'cs_stats_consent', $data_arr_statistics );
		}

		if ( ConsentMagic()->getOption( 'cs_proof_consent_enable' ) == 1 && $cs_check_tables
		     && $excluded_from_consent_storing == 0
		     && $current_action !== 'Close consent'
		     && $current_action !== 'Close Options Popup' ) {

			if ( ( $cs_type == 'just_inform' && ConsentMagic()->getOption( 'cs_proof_just_inform' ) == 1 )
			     || ( $cs_type == 'inform_and_opiout'
			          && ConsentMagic()->getOption( 'cs_proof_inform_and_optout' ) == 1 )
			     || ( $cs_type == 'ask_before_tracking'
			          && ConsentMagic()->getOption( 'cs_proof_ask_before_tracking' ) == 1 )
			     || ( $cs_type == 'iab' && ConsentMagic()->getOption( 'cs_proof_iab' ) == 1 ) ) {
				$ip = get_client_ip();
				if ( is_user_logged_in() ) {
					$user         = wp_get_current_user();
					$email        = $user->user_email;
					$user_profile = '<a href="' . admin_url( 'user-edit.php?user_id=' . $user->ID )
					                . '" target="_blank">' . admin_url( 'user-edit.php?user_id=' . $user->ID ) . '</a>';
					$user_data    = $email;
				} else {
					$user_data    = 'no associated user';
					$user_profile = 'no associated user';
				}

				$data_arr = array(
					'created_at'     => current_time( 'mysql' ),
					'current_action' => $current_action,
					'ip'             => $ip,
					'url'            => $url,
					'consent_type'   => $cs_type_db,
					'email'          => $user_data,
					'profile'        => $user_profile,
					'category'       => $category,
					'rule'           => get_the_title( $rule_id )
				);

				if ( ConsentMagic()->getOption( 'cs_proof_auto_delete' ) == 1 ) {
					$cs_proof_consent_count = $wpdb->get_var( $wpdb->prepare( 'SELECT COUNT(*) FROM %1$s', $table ) );
					$cs_proof_entries_count = ConsentMagic()->getOption( 'cs_proof_entries_count' );
					if ( $cs_proof_consent_count >= $cs_proof_entries_count ) {
						if ( $cs_email_before_delete_consent == 1 ) {
							create_email_before_delete_consent();
						} else {
							$wpdb->query( $wpdb->prepare( 'DELETE FROM %1$s', $table ) );
							renew_consent_run();
						}
					}
				}

				if ( $wpdb->insert( $table, $data_arr ) ) {
					echo $wpdb->insert_id;
				}
			}
		}

		wp_die();
	}

	public function cs_save_order_consent( $order_id ) {
		if ( isWooCommerceActive() && ConsentMagic()->getOption( 'cs_store_consent_for_wc_orders' ) == 1 ) {
			$active_rule_id = ConsentMagic()->cs_options[ 'active_rule_id' ];
			$cs_type        = get_post_meta( $active_rule_id, '_cs_type', true );
			$title          = get_the_title( $active_rule_id );
			$terms_primary  = get_cookies_terms_objects( 'cs_primary_term', true );
			$terms          = get_cookies_terms_objects( 'cs_primary_term' );
			$native_scripts = get_post_meta( $active_rule_id, '_cs_native_scripts', true );
			$no             = array();
			$yes            = array();

			if ( $cs_type == 'iab' && CS_Cookies()->getCookie('cs_consent_string' ) ) {
				try {
					$iab_string  = json_decode(
						base64_decode( CS_Cookies()->getCookie('cs_consent_string' ) ),
						false
					);
					$purposes    = $iab_string->purposes;
					$purposes    = str_split( $purposes );
					$vendor_list = CS_IAB_Integration()->get_vendor_list();

					foreach ( $purposes as $i => $purpose ) {
						if ( isset( $vendor_list->purposes->{(int) $i + 1} ) ) {
							if ( $purpose == 1 ) {
								$yes[] .= $vendor_list->purposes->{(int) $i + 1}->name;
							} else {
								$no[] .= $vendor_list->purposes->{(int) $i + 1}->name;
							}
						}
					}
				} catch ( \Exception $e ) {
					error_log( 'Error: ' . $e->getMessage() );
				}
			}


			if ( $cs_type != 'iab' || $native_scripts == 1 ) {

				$yes[] = 'Necessary';
				if ( !empty( $terms_primary ) ) {
					foreach ( $terms_primary as $term ) {
						$category_cookie = "cs_enabled_cookie_term_" . $term->term_id;
						$ignore          = (int) get_term_meta( $term, 'cs_ignore_this_category', true );
						if ( $ignore == 0 ) {
							if ( CS_Cookies()->getCookie( $category_cookie ) && CS_Cookies()->getCookie( $category_cookie ) == 'yes' ) {
								$yes[] = $term->name;
							} else {
								$no[] = $term->name;
							}
						}
					}
				}

				if ( !empty( $terms ) ) {
					foreach ( $terms as $term ) {
						$category_cookie = "cs_enabled_cookie_term_" . $term->term_id;
						$ignore          = (int) get_term_meta( $term, 'cs_ignore_this_category', true );
						if ( $ignore == 0 ) {
							if ( CS_Cookies()->getCookie( $category_cookie ) && CS_Cookies()->getCookie( $category_cookie ) == 'yes' ) {
								$yes[] = $term->name;
							} else {
								$no[] = $term->name;
							}
						}
					}
				}
			}

			$consent_mode = apply_filters( 'cm_google_consent_mode', array(
				'analytics_storage'  => array(
					'enabled' => true,
					'value'   => 'granted',
				),
				'ad_storage'         => array(
					'enabled' => true,
					'value'   => 'granted',
				),
				'ad_user_data'       => array(
					'enabled' => true,
					'value'   => 'granted',
				),
				'ad_personalization' => array(
					'enabled' => true,
					'value'   => 'granted',
				),
			) );

			if ( function_exists( 'WC' ) && version_compare( WC()->version, '3.0', '>=' ) ) {
				$order = wc_get_order( $order_id );

				$order->update_meta_data( '_cs_type', get_cs_type_name( $cs_type ) );
				$order->update_meta_data( '_cs_title', $title );
				if ( CS_Cookies()->getCookie( 'cs_user_preference' ) ) {
					$order->update_meta_data( '_cs_expressed', 'Yes' );
				} else {
					$order->update_meta_data( '_cs_expressed', 'No' );
				}

				$order->update_meta_data( '_cs_category_list', $yes );
				$order->update_meta_data( '_cs_category_yes', $yes );
				$order->update_meta_data( '_cs_category_no', $no );
				if ( CS_Google_Consent_Mode()->enabled_google_consent_mode() ) {
					$order->update_meta_data(
						'_cm_analytics_storage',
						sanitize_text_field( $consent_mode[ 'analytics_storage' ][ 'value' ] )
					);
					$order->update_meta_data(
						'_cm_ad_storage',
						sanitize_text_field( $consent_mode[ 'ad_storage' ][ 'value' ] )
					);
					$order->update_meta_data(
						'_cm_ad_user_data',
						sanitize_text_field( $consent_mode[ 'ad_user_data' ][ 'value' ] )
					);
					$order->update_meta_data(
						'_cm_ad_personalization',
						sanitize_text_field( $consent_mode[ 'ad_personalization' ][ 'value' ] )
					);
				}
				$order->save();
			} else {

				update_post_meta( $order_id, "_cs_type", get_cs_type_name( $cs_type ) );
				update_post_meta( $order_id, "_cs_title", $title );
				if ( CS_Cookies()->getCookie( 'cs_user_preference' ) ) {
					update_post_meta( $order_id, "_cs_expressed", 'Yes' );
				} else {
					update_post_meta( $order_id, "_cs_expressed", 'No' );
				}
				update_post_meta( $order_id, "_cs_category_list", $yes );
				update_post_meta( $order_id, "_cs_category_yes", $yes );
				update_post_meta( $order_id, "_cs_category_no", $no );

				if ( CS_Google_Consent_Mode()->enabled_google_consent_mode() ) {
					update_post_meta(
						$order_id,
						'_cm_analytics_storage',
						sanitize_text_field( $consent_mode[ 'analytics_storage' ][ 'value' ] )
					);
					update_post_meta(
						$order_id,
						'_cm_ad_storage',
						sanitize_text_field( $consent_mode[ 'ad_storage' ][ 'value' ] )
					);
					update_post_meta(
						$order_id,
						'_cm_ad_user_data',
						sanitize_text_field( $consent_mode[ 'ad_user_data' ][ 'value' ] )
					);
					update_post_meta(
						$order_id,
						'_cm_ad_personalization',
						sanitize_text_field( $consent_mode[ 'ad_personalization' ][ 'value' ] )
					);
				}
			}
		}
	}

	public function cs_save_edd_order_consent( $payment_data ) {
		if ( isEddActive() && ConsentMagic()->getOption( 'cs_store_consent_for_edd_orders' ) == 1 ) {
			$active_rule_id = ConsentMagic()->cs_options[ 'active_rule_id' ];
			$cs_type        = get_post_meta( $active_rule_id, '_cs_type', true );
			$title          = get_the_title( $active_rule_id );
			$terms_primary  = get_cookies_terms_objects( 'cs_primary_term', true );
			$terms          = get_cookies_terms_objects( 'cs_primary_term' );
			$native_scripts = get_post_meta( $active_rule_id, '_cs_native_scripts', true );
			$no             = array();
			$yes            = array();

			if ( $cs_type == 'iab' && CS_Cookies()->getCookie( 'cs_consent_string' ) ) {
				try {
					$iab_string  = json_decode(
						base64_decode( CS_Cookies()->getCookie( 'cs_consent_string' ) ),
						false
					);
					$purposes    = $iab_string->purposes;
					$purposes    = str_split( $purposes );
					$vendor_list = CS_IAB_Integration()->get_vendor_list();

					foreach ( $purposes as $i => $purpose ) {
						if ( isset( $vendor_list->purposes->{(int) $i + 1} ) ) {
							if ( $purpose == 1 ) {
								$yes[] .= $vendor_list->purposes->{(int) $i + 1}->name;
							} else {
								$no[] .= $vendor_list->purposes->{(int) $i + 1}->name;
							}
						}
					}
				} catch ( \Exception $e ) {
					error_log( 'Error: ' . $e->getMessage() );
				}
			}

			if ( $cs_type != 'iab' || $native_scripts == 1 ) {

				$yes[] = 'Necessary';
				if ( !empty( $terms_primary ) ) {
					foreach ( $terms_primary as $term ) {
						$category_cookie = "cs_enabled_cookie_term_" . $term->term_id;
						$ignore          = (int) get_term_meta( $term, 'cs_ignore_this_category', true );
						if ( $ignore == 0 ) {
							if ( ( CS_Cookies()->getCookie( $category_cookie ) && CS_Cookies()->getCookie( $category_cookie ) == 'yes' ) ) {
								$yes[] = $term->name;

							} else {
								$no[] = $term->name;
							}
						}
					}
				}

				if ( !empty( $terms ) ) {
					foreach ( $terms as $term ) {
						$category_cookie = "cs_enabled_cookie_term_" . $term->term_id;
						$ignore          = (int) get_term_meta( $term, 'cs_ignore_this_category', true );
						if ( $ignore == 0 ) {
							if ( ( CS_Cookies()->getCookie( $category_cookie ) && CS_Cookies()->getCookie( $category_cookie ) == 'yes' ) ) {
								$yes[] = $term->name;

							} else {
								$no[] = $term->name;
							}
						}
					}
				}
			}

			$payment_data[ '_cs_type' ]  = get_cs_type_name( $cs_type );
			$payment_data[ '_cs_title' ] = $title;
			if ( CS_Cookies()->getCookie( 'cs_user_preference' ) ) {
				$payment_data[ '_cs_expressed' ] = 'Yes';
			} else {
				$payment_data[ '_cs_expressed' ] = 'No';
			}

			$payment_data[ '_cs_category_list' ] = $yes;
			$payment_data[ '_cs_category_yes' ]  = $yes;
			$payment_data[ '_cs_category_no' ]   = $no;

			if ( CS_Google_Consent_Mode()->enabled_google_consent_mode() ) {
				$payment_data[ '_cm_analytics_storage' ]  = isset( $_REQUEST[ 'cm_analytics_storage' ] ) ? sanitize_text_field(
					$_REQUEST[ 'cm_analytics_storage' ]
				) : 0;
				$payment_data[ '_cm_ad_storage' ]         = isset( $_REQUEST[ 'cm_ad_storage' ] ) ? sanitize_text_field(
					$_REQUEST[ 'cm_ad_storage' ]
				) : 0;
				$payment_data[ '_cm_ad_user_data' ]       = isset( $_REQUEST[ 'cm_ad_user_data' ] ) ? sanitize_text_field(
					$_REQUEST[ 'cm_ad_user_data' ]
				) : 0;
				$payment_data[ '_cm_ad_personalization' ] = isset( $_REQUEST[ 'cm_ad_personalization' ] ) ? sanitize_text_field(
					$_REQUEST[ 'cm_ad_personalization' ]
				) : 0;
			}
		}

		return $payment_data;
	}

	function cs_proof_show_count_update() {
		check_ajax_referer( 'cs-ajax-public-nonce', 'nonce_code' );

		//check throttle
		if ( !$this->checkThrottle( 'cs_preview_shortcode_show' ) ) {
			wp_send_json_error();
			wp_die();
		}

		$current_count = (int) ConsentMagic()->getOption( 'cs_proof_show_count' );

		$current_count++;
		ConsentMagic()->updateOptions( array( 'cs_proof_show_count' => $current_count ) );
		wp_die();
	}

	/**
	 * checking necessary tables are installed
	 */
	protected function check_tables() {
		global $wpdb;

		require_once ABSPATH . 'wp-admin/includes/upgrade.php';
		//creating main table ========================
		$table_name   = $wpdb->prefix . 'cs_proof_consent';
		$like         = '%' . $wpdb->esc_like( $table_name ) . '%';
		$search_query = $wpdb->prepare( "SHOW TABLES LIKE %s", $like );
		if ( !$wpdb->get_results( $search_query, ARRAY_N ) ) {
			$create_table_sql = $wpdb->prepare(
				'CREATE TABLE `%1$s`(
            `uuid` INT NOT NULL AUTO_INCREMENT,
            `created_at` DATETIME NOT NULL DEFAULT \'0000-00-00 00:00:00\',
            `updated_at` DATETIME NOT NULL DEFAULT \'0000-00-00 00:00:00\',
            `url` TEXT NOT NULL,
            `ip` TEXT NOT NULL,
            `email` TEXT NOT NULL,
            `profile` TEXT NOT NULL,
            `rule` TEXT NOT NULL,
            `consent_type` VARCHAR(50) NOT NULL,
            `category` LONGTEXT NOT NULL,
            `current_action` VARCHAR(50) NOT NULL,
            PRIMARY KEY(`uuid`)
        );',
				$table_name
			);
			dbDelta( $create_table_sql );
		}

		//creating statictics table ========================
		$table_name_statistics = $wpdb->prefix . 'cs_stats_consent';
		$like                  = '%' . $wpdb->esc_like( $table_name_statistics ) . '%';
		$search_query          = $wpdb->prepare( "SHOW TABLES LIKE %s", $like );
		if ( !$wpdb->get_results( $search_query, ARRAY_N ) ) {
			$create_table_sql = $wpdb->prepare(
				'CREATE TABLE `%1$s`(
            `id` INT NOT NULL AUTO_INCREMENT,
            `uuid` INT NOT NULL,
            `current_action` VARCHAR(50) NOT NULL,
            `rule` TEXT NOT NULL,
            `rule_id` TEXT NOT NULL,
            `consent_type` VARCHAR(50) NOT NULL,
            `created_at` DATETIME NOT NULL DEFAULT \'0000-00-00 00:00:00\',
            PRIMARY KEY(`id`)
         );',
				$table_name_statistics
			);
			dbDelta( $create_table_sql );
		}

		//remove old statictics table ========================
		$table_name_old_statistics = $wpdb->prefix . 'cs_statistics_consent';
		$wpdb->query( $wpdb->prepare( 'DROP TABLE IF EXISTS %1$s;', $table_name_old_statistics ) );
		ConsentMagic()->updateOptions( array( 'cs_check_proof_tables' => true ) );

		return true;
	}

	function cs_preview_shortcode_show() {
		if ( !has_shortcode( $this->page_content, 'disable_cm' ) ) {

			//check throttle
			if ( !$this->checkThrottle( 'cs_preview_shortcode_show' ) ) {
				wp_send_json_error();
				wp_die();
			}

			check_ajax_referer( 'cs-ajax-public-nonce', 'nonce_code' );
			ob_start();
			$active_rule_id   = isset( $_POST[ 'id' ] ) ? sanitize_text_field( $_POST[ 'id' ] ) : '';
			$rule_obj         = get_post_meta( $active_rule_id );
			$id               = $rule_obj[ '_cs_theme' ][ 0 ];
			$cs_bars_type     = $rule_obj[ '_cs_bars_type' ][ 0 ];
			$cs_bars_position = $rule_obj[ '_cs_bars_position' ][ 0 ];
			$cs_type          = $rule_obj[ '_cs_type' ][ 0 ];
			$cs_privacy_link  = $rule_obj[ '_cs_privacy_link' ][ 0 ];
			$cs_top_push      = $rule_obj[ '_cs_top_push' ][ 0 ];
			$classes          = $cs_top_push ? 'cs_top_push' : '';
			$classes          .= is_rtl() ? ' rtl' : '';

			if ( $cs_type == 'ask_before_tracking' ) {
				if ( !CS_Cookies()->getCookie( 'cs_viewed_cookie_policy' ) ) {
					$cs_block_content = $rule_obj[ '_cs_block_content' ][ 0 ];
				} else {
					$cs_block_content = 0;
				}
			} elseif ( $cs_type == 'inform_and_opiout' ) {
				if ( isset( $rule_obj[ '_cs_showing_rule_until_express_consent' ][ 0 ] )
				     && $rule_obj[ '_cs_showing_rule_until_express_consent' ][ 0 ] == 1
				     && !CS_Cookies()->getCookie( 'cs_viewed_cookie_policy' ) ) {
					$cs_block_content = $rule_obj[ '_cs_block_content' ][ 0 ];
				} else {
					$cs_block_content = 0;
				}
			} else {
				if ( !CS_Cookies()->getCookie( 'CS-Magic' ) ) {
					$cs_block_content = $rule_obj[ '_cs_block_content' ][ 0 ];
				} else {
					$cs_block_content = 0;
				}
			}
			$cs_scroll_close = $rule_obj[ '_cs_close_on_scroll' ][ 0 ];
			$cs_sticky       = $rule_obj[ '_cs_sticky' ][ 0 ];
			echo '<div class="cs_preview_container ' . esc_attr( $classes ) . ' bars_position_' . esc_attr(
					$cs_bars_position
				) . ' ' . esc_attr( $cs_bars_type ) . ' cs_block_content_' . esc_attr( $cs_block_content )
			     . '" style="display: none;" data-scroll="' . esc_attr( $cs_scroll_close ) . '" data-sticky="'
			     . esc_attr( $cs_sticky ) . '">';

			echo '<div class="preview_wrap">';
			if ( $cs_bars_type == 'bar_small' || $cs_bars_type == 'bar_large' ) {
				include CMPRO_PLUGIN_VIEWS_PATH . 'templates/multi/cs-bar.php';
			} else if ( $cs_bars_type == 'popup_small' || $cs_bars_type == 'popup_large' ) {
				include CMPRO_PLUGIN_VIEWS_PATH . 'templates/multi/cs_popup.php';
			}
			echo '</div>';

			echo '</div>';
			$html = ob_get_contents();
			ob_end_clean();
			echo wp_kses_post( $html );
			wp_die(); // ajax call must die to avoid trailing 0 in your response
		}
	}

	public function public_init() {
		new CS_WP_Consent_Api();
		if ( !is_admin() && !wp_doing_cron() ) {
			$popup_content_escaped = $this->get_popup_content();
			if ( $popup_content_escaped != false ) {
				add_action( $popup_content_escaped[ 'position' ], function() use ( $popup_content_escaped ) {
					echo '<div id="cs_preview_popup">';
					echo $popup_content_escaped[ 'content' ];
					echo '</div>';
				} );

				add_action( 'wp_footer', function() use ( $popup_content_escaped ) {
					echo '<div id="cs_preview_popup_button" ' . ( is_rtl() ? 'class="rtl"' : '' ) . '>';
					echo $popup_content_escaped[ 'button' ];
					echo '</div>';
				} );
			}
		}
	}

	public function get_popup_content() {
		$show = $this->check_disable();
		if ( ConsentMagic()->getOption( 'cs_plugin_activation' ) == 1 && $show && ConsentMagic()->ready_to_run() ) {

			$active_rule_id = ConsentMagic()->get_active_rule_id();
			if ( empty( $active_rule_id ) ) {
				return false;
			}

			$metas            = get_post_meta( $active_rule_id );
			$cs_bars_position = $metas[ '_cs_bars_position' ][ 0 ];
			$cs_top_push      = $metas[ '_cs_top_push' ][ 0 ];
			if ( $cs_top_push && $cs_bars_position == "top" ) {
				$position = 'wp_head';
			} else {
				$position = 'wp_footer';
			}

			if ( $this->cache && wp_doing_ajax() ) {
				$unblocked_ip = ConsentMagic()->get_unblocked_user_ip();
				$ip           = get_client_ip();

				if ( ( isset( $unblocked_ip ) && is_array( $unblocked_ip ) ) ) {
					if ( !empty( array_intersect( $unblocked_ip, array( $ip ) ) ) ) {
						return array(
							'position' => $position,
							'content'  => '',
							'button'   => ''
						);
					}
				}
			}

			$user          = wp_get_current_user();
			$allowed_roles = ConsentMagic()->getOption( 'cs_admin_permissions' );
			$ignore_roles  = ConsentMagic()->getOption( 'cs_admin_ignore_users' );

			$content = $this->cs_output_public_script( $active_rule_id, $user, $allowed_roles, $ignore_roles );
			$button  = $this->cs_output_public_options_button( $active_rule_id, $user, $allowed_roles, $ignore_roles );

			return array(
				'position' => $position,
				'content'  => $content,
				'button'   => $button
			);

		} else {
			return false;
		}
	}

	/**
	 * Outputs the cookie control script
	 * N.B. This script MUST be output.
	 * This function should be attached to the wp_footer or wp_head action hook.
	 */
	public function cs_output_public_script( $active_rule_id, $user, $allowed_roles, $ignore_roles ) {

		$show = $this->check_disable( $ignore_roles );
		if ( $show && $active_rule_id ) {
			$metas                   = get_post_meta( $active_rule_id );
			$id                      = $metas[ '_cs_theme' ][ 0 ];
			$style_array             = get_post_meta( $id );
			$cs_bars_type            = $metas[ '_cs_bars_type' ][ 0 ];
			$cs_type                 = $metas[ '_cs_type' ][ 0 ];
			$cs_sticky               = $metas[ '_cs_sticky' ][ 0 ];
			$cs_hide_close_btn       = $metas[ '_cs_hide_close_btn' ][ 0 ];
			$cs_privacy_link         = $metas[ '_cs_privacy_link' ][ 0 ];
			$cs_deny_all_btn         = $metas[ '_cs_deny_all_btn' ][ 0 ];
			$cs_custom_button_btn    = $metas[ '_cs_custom_button' ][ 0 ];
			$cs_bars_position        = $metas[ '_cs_bars_position' ][ 0 ];
			$cs_top_push             = $metas[ '_cs_top_push' ][ 0 ];
			$cs_scroll_close         = $metas[ '_cs_close_on_scroll' ][ 0 ];
			$cs_smart_sticky_desktop = $metas[ '_cs_smart_sticky' ][ 0 ];
			$cs_smart_sticky_mobile  = $metas[ '_cs_smart_sticky_mobile' ][ 0 ];
			$smart_sticky            = '';
			$design_type             = $metas[ '_cs_design_type' ][ 0 ];

			if ( $cs_sticky ) {
				if ( $cs_smart_sticky_desktop ) {
					$smart_sticky .= $this->cs_check_cookie_categories_for_sticky(
						'desktop'
					) ? ' data-smart-sticky-desktop-show="1"' : ' data-smart-sticky-desktop-show="0"';
				}
				if ( $cs_smart_sticky_mobile ) {
					$smart_sticky .= $this->cs_check_cookie_categories_for_sticky(
						'mobile'
					) ? ' data-smart-sticky-mobile-show="1"' : ' data-smart-sticky-mobile-show="0"';
				}
			}

			if ( !CS_Cookies()->getCookie( 'cs_viewed_cookie_policy' ) ) {
				if ( $cs_type == 'ask_before_tracking' ) {
					$cs_block_content = $metas[ '_cs_block_content' ][ 0 ];
				} elseif ( $cs_type == 'inform_and_opiout'
				           && isset( $metas[ '_cs_showing_rule_until_express_consent' ][ 0 ] )
				           && $metas[ '_cs_showing_rule_until_express_consent' ][ 0 ] == 1 ) {
					$cs_block_content = $metas[ '_cs_block_content' ][ 0 ];
				} else {
					$cs_block_content = CS_Cookies()->getCookie( 'CS-Magic' ) ? 0 : $metas[ '_cs_block_content' ][ 0 ];
				}
			} else {
				$cs_block_content = 0;
			}

			$cs_mobile_side_sticky = $metas[ '_cs_mobile_side_sticky' ][ 0 ] ? 'cs_mobile_side_sticky' : '';

			$classes = $cs_top_push ? 'cs_top_push' : '';
			$classes .= is_rtl() ? ' rtl' : '';

			$out = '';
			ob_start();

			$out .= '<div class="cs_preview_container ' . esc_attr( $classes ) . ' bars_position_' . esc_attr(
					$cs_bars_position
				) . ' ' . esc_attr( $cs_bars_type ) . ' cs_block_content_' . esc_attr( $cs_block_content )
			        . '" style="display: none;" data-scroll="' . esc_attr( $cs_scroll_close ) . '" data-sticky="'
			        . esc_attr( $cs_sticky ) . '" data-smart-sticky-desktop="' . esc_attr( $cs_smart_sticky_desktop )
			        . '" data-smart-sticky-mobile="' . esc_attr( $cs_smart_sticky_mobile ) . '" ' . $smart_sticky
			        . ' data-cstype="' . esc_attr( $cs_type ) . '" data-rule="' . $active_rule_id . '">';

			$out .= '<div class="preview_wrap">';

			if ( $cs_type == 'iab' || $design_type == 'single' ) {
				include_once CMPRO_PLUGIN_VIEWS_PATH . 'templates/single/cs_single_design.php';
			} elseif ( $cs_bars_type == 'bar_small' || $cs_bars_type == 'bar_large' ) {
				include_once CMPRO_PLUGIN_VIEWS_PATH . 'templates/multi/cs-bar.php';
			} elseif ( $cs_bars_type == 'popup_small' || $cs_bars_type == 'popup_large' ) {
				include_once CMPRO_PLUGIN_VIEWS_PATH . 'templates/multi/cs_popup.php';
			}

			$out .= ob_get_contents();
			$out .= '</div>';
			$out .= '</div>';

			ob_end_clean();

			return $out;
		}
	}

	/**
	 * Outputs the cookie control options button in the footer
	 * N.B. This options button MUST be output in the footer.
	 * This function should be attached to the wp_footer action hook.
	 */
	public function cs_output_public_options_button( $active_rule_id, $user, $allowed_roles, $ignore_roles ) {
		$show = $this->check_disable( $ignore_roles );
		if ( $show && $active_rule_id ) {

			$metas                 = get_post_meta( $active_rule_id );
			$id                    = $metas[ '_cs_theme' ][ 0 ];
			$style_array           = get_post_meta( $id );
			$cs_sticky             = $metas[ '_cs_sticky' ][ 0 ];
			$cs_top_push           = $metas[ '_cs_top_push' ][ 0 ];
			$cs_scroll_close       = $metas[ '_cs_close_on_scroll' ][ 0 ];
			$cs_mobile_side_sticky = $metas[ '_cs_mobile_side_sticky' ][ 0 ] ? 'cs_mobile_side_sticky' : '';
			$cs_sticky_output      = '';
			if ( $cs_sticky ) {

				$wp_current_lang          = ( isset( $this->current_lang )
				                              && $this->current_lang ) ? $this->current_lang : get_locale();
				$cs_user_default_language = ConsentMagic()->getOption( 'cs_user_default_language' );
				$cs_language_availability = ConsentMagic()->getOption( 'cs_language_availability' );
				if ( isset( $cs_language_availability[ $wp_current_lang ] )
				     && $cs_language_availability[ $wp_current_lang ] == 0 ) {
					$wp_current_lang = $cs_user_default_language;
				}
				$custom_text = get_post_meta( $active_rule_id, '_cs_custom_text', true );

				$front_options = array(
					'_cs_btn_text_sticky_cookie' => '',
				);
				$front_options = generate_front_text(
					$front_options,
					$custom_text,
					$active_rule_id,
					$wp_current_lang,
					$cs_user_default_language
				);

				$design = $metas[ '_cs_design_type' ][ 0 ];
				if ( $design == 'single' ) {
					$min_height               = ConsentMagic()->getOption( 'cs_d_single_stp_min_height' );
					$padding_top              = ConsentMagic()->getOption( 'cs_d_single_stp_padding_top' );
					$padding_bottom           = ConsentMagic()->getOption( 'cs_d_single_stp_padding_bottom' );
					$padding_left             = ConsentMagic()->getOption( 'cs_d_single_stp_padding_left' );
					$padding_right            = ConsentMagic()->getOption( 'cs_d_single_stp_padding_right' );
					$font_size                = ConsentMagic()->getOption( 'cs_d_single_stp_font_size' );
					$font_weight              = ConsentMagic()->getOption( 'cs_d_single_stp_font_weight' );
					$desktop_sticky_position  = 'desktop_' . ConsentMagic()->getOption(
							'cs_d_single_stp_desktop_position'
						);
					$sticky_vertical_position = ConsentMagic()->getOption( 'cs_d_single_stp_sticky_position_vertical' );
				} else {
					$min_height               = ConsentMagic()->getOption( 'cs_d_stp_min_height' );
					$padding_top              = ConsentMagic()->getOption( 'cs_d_stp_padding_top' );
					$padding_bottom           = ConsentMagic()->getOption( 'cs_d_stp_padding_bottom' );
					$padding_left             = ConsentMagic()->getOption( 'cs_d_stp_padding_left' );
					$padding_right            = ConsentMagic()->getOption( 'cs_d_stp_padding_right' );
					$font_size                = ConsentMagic()->getOption( 'cs_d_stp_font_size' );
					$font_weight              = ConsentMagic()->getOption( 'cs_d_stp_font_weight' );
					$desktop_sticky_position  = 'desktop_' . ConsentMagic()->getOption( 'cs_d_stp_desktop_position' );
					$sticky_vertical_position = ConsentMagic()->getOption( 'cs_d_stp_sticky_position_vertical' );
				}

				$cs_sticky_output .= '<!--googleoff: all--><div class="cs-info-sticky ' . $desktop_sticky_position . ' '
				                     . $cs_mobile_side_sticky . ' ' . $sticky_vertical_position
				                     . '" style="min-height: ' . $min_height . 'px;">';
				$cs_sticky_output .= '<span class="cs-info-sticky-button" style="background-color: '
				                     . $style_array[ 'cs_sticky_bg' ][ 0 ] . '; color: '
				                     . $style_array[ 'cs_sticky_link_color' ][ 0 ] . '; padding:' . $padding_top . 'px '
				                     . $padding_right . 'px ' . $padding_bottom . 'px ' . $padding_left
				                     . 'px; font-size: ' . $font_size . 'px; font-weight: ' . $font_weight . ';">';
				$cs_sticky_output .= $front_options[ '_cs_btn_text_sticky_cookie' ];
				$cs_sticky_output .= '</span>';
				$cs_sticky_output .= '</div><!--googleon: all-->';

				$classes = $cs_top_push ? 'cs_top_push' : '';
				$classes .= is_rtl() ? ' rtl' : '';

				$out = '<div class="cs_preview_container ' . esc_attr( $classes )
				       . ' cs_footer_btn" style="display: none;" data-scroll="' . esc_attr( $cs_scroll_close )
				       . '" data-sticky="' . esc_attr( $cs_sticky ) . '">';
				$out .= wp_kses_post( $cs_sticky_output );
				$out .= '</div>';

				return $out;
			}
		}
	}

	public function check_disable( $unblocked_user_ip = null ) {

		if ( self::$disable === null ) {

			if ( $this->cache ) {
				$show = true;
			} else {
				$user         = wp_get_current_user();
				$ignore_roles = ConsentMagic()->getOption( 'cs_admin_ignore_users' );
				// get or make permalink
				$HTTP_REFERER = isset( $_SERVER[ 'HTTP_REFERER' ] ) ? sanitize_text_field(
					$_SERVER[ 'HTTP_REFERER' ]
				) : false;
				$show         = false;

				if ( isset( $HTTP_REFERER ) && strstr( $HTTP_REFERER, 'wp-admin' ) ) {
					$show = true;
				} else {
					$this->url = $HTTP_REFERER;
					// get post_id using url/permalink
					global $wp_rewrite, $wp;
					if ( $wp_rewrite === null || !isset( $wp->public_query_vars ) ) {
						$url_path = parse_url( $this->url, PHP_URL_PATH );
						$slug     = pathinfo( $url_path ?: '', PATHINFO_BASENAME );

						$this->post_id = ConsentMagic::getCurrentPostID( $slug );

					} else {
						if ( function_exists( 'icl_object_id' ) ) {
							$wp_current_lang = apply_filters( 'wpml_current_language', NULL );
							$this->post_id = apply_filters( 'wpml_object_id', get_the_id(), get_post_type(), true, $wp_current_lang );
						} else {
							$this->post_id = url_to_postid( $this->url );
						}
					}

					$this->post_content = get_post( $this->post_id );
					if ( $this->post_content ) {
						$this->page_content = $this->post_content->post_content;
					}

					global $wp_query;

					if ( $wp_query === null || is_home() || is_front_page() ) {
						$show = !has_shortcode( $this->page_content, 'disable_cm' );
					}
				}

				if ( $unblocked_user_ip === null ) {
					$unblocked_user_ip = ConsentMagic()->get_unblocked_user_ip();
				}

				$ip = get_client_ip();

				if ( ( isset( $ignore_roles ) && is_array( $ignore_roles ) && isset( $user->roles )
				       && array_intersect( $ignore_roles, $user->roles ) )
				     || ( isset( $ignore_roles ) && is_array( $ignore_roles ) && isset( $user )
				          && array_intersect( $ignore_roles, array( 'visitor' ) )
				          && $user->ID == 0 )
				     || ( isset( $unblocked_user_ip ) && is_array( $unblocked_user_ip )
				          && array_intersect( $unblocked_user_ip, array( $ip ) ) ) ) {
					$show = false;
				}

			}
			self::$disable = $show;

			return $show;
		} else {

			return self::$disable;
		}
	}

	/**
	 * Get the cookie list
	 */
	public static function get_cookie_list() {

		if ( self::$cookie_list_arr !== null ) {
			return self::$cookie_list_arr;
		}
		$args  = array(
			'taxonomy'   => 'cs-cookies-category',
			'hide_empty' => false,
			'meta_key'   => 'cs_primary_term',
			'orderby'    => 'meta_value',
			// use 'meta_value_num' if the value type of this meta is numeric.
			'order'      => 'DESC',
		);
		$terms = get_terms( $args );
		$posts = array();
		global $sitepress;
		$wpml_default_lang = 'en';
		$wpml_current_lang = 'en';
		if ( function_exists( 'icl_object_id' ) && $sitepress ) {
			$wpml_default_lang = $sitepress->get_default_language();
			$wpml_current_lang = apply_filters( 'wpml_current_language', NULL );
		}

		foreach ( $terms as $term ) {
			if ( is_object( $term ) ) {
				$term_term_id = $term->term_id;
				//wpml enabled and current language is not default language
				if ( function_exists( 'icl_object_id' ) && $wpml_default_lang != $wpml_current_lang ) {
					if ( version_compare( ICL_SITEPRESS_VERSION, '3.2.0' ) >= 0 ) {
						$original_term_id = apply_filters(
							'wpml_object_id',
							$term->term_id,
							'category',
							true,
							$wpml_default_lang
						);
					} else {
						$original_term_id = icl_object_id( $term->term_id, 'category', true, $wpml_default_lang );
					}
					$sitepress->switch_lang( $wpml_default_lang );
					$original_term = get_term_by( 'id', $original_term_id, 'cs-category' );
					if ( $original_term && $original_term->term_id ) {
						$term_term_id = $original_term->term_id;
					}
					$sitepress->switch_lang( $wpml_current_lang );
				}
				$posts[ $term_term_id ] = get_posts( array(
					'posts_per_page' => -1,
					'post_type'      => CMPRO_POST_TYPE,
					'taxonomy'       => $term->taxonomy,
					'term'           => $term->slug,
				) );

				$ignore                              = get_term_meta( $term->term_id, 'cs_ignore_this_category', true );
				$posts[ $term_term_id ][ 'term_id' ] = $term->term_id;
				$posts[ $term_term_id ][ 'name' ]    = $term->name;
				$posts[ $term_term_id ][ 'slug' ]    = $term->slug;
				$posts[ $term_term_id ][ 'ignore' ]  = !empty( $ignore ) ? $ignore : 0;
			}
		}
		self::$cookie_list_arr = $posts;

		return $posts;
	}

	public function get_cookie_by_category() {
		$args  = array(
			'taxonomy' => 'cs-cookies-category',
			'meta_key' => 'cs_necessary_term',
			'orderby'  => 'meta_value',
			// use 'meta_value_num' if the value type of this meta is numeric.
			'order'    => 'DESC',
		);
		$terms = get_terms( $args );
		global $sitepress;
		$wpml_default_lang = 'en';
		$wpml_current_lang = 'en';
		//wpml enabled check
		if ( function_exists( 'icl_object_id' ) && $sitepress ) {
			$wpml_default_lang = $sitepress->get_default_language();
			$wpml_current_lang = apply_filters( 'wpml_current_language', NULL );
		}
		$non_necessary_cookies = array();
		foreach ( $terms as $term ) {
			if ( is_object( $term ) ) {
				$term_slug = $term->slug;
				if ( $term_slug !== 'necessary' ) {
					if ( function_exists( 'icl_object_id' ) && $wpml_default_lang != $wpml_current_lang ) {
						if ( version_compare( ICL_SITEPRESS_VERSION, '3.2.0' ) >= 0 ) {
							$original_term_id = apply_filters(
								'wpml_object_id',
								$term->term_id,
								'category',
								true,
								$wpml_default_lang
							);
						} else {
							$original_term_id = icl_object_id( $term->term_id, 'category', true, $wpml_default_lang );
						}
						$sitepress->switch_lang( $wpml_default_lang );
						$original_term = get_term_by( 'id', $original_term_id, 'cs-cookies-category' );
						if ( $original_term && $original_term->term_id ) {
							$term_slug = $original_term->slug;
						}
						$sitepress->switch_lang( $wpml_current_lang );
					}
					$cs_cookies = array();
					$args       = array(
						'posts_per_page' => -1,
						'post_type'      => CMPRO_POST_TYPE_SCRIPTS,
						// you can change it according to your custom post type
						'post_status'    => 'any',
						'tax_query'      => array(
							array(
								'taxonomy' => 'cs-cookies-category',
								'field'    => 'slug',
								'terms'    => $term_slug
							)
						)
					);
					$results    = get_posts( $args );
					foreach ( $results as $row ) {
						$cs_cookies[] = $row->post_title;
					}
				}
				if ( !empty( $cs_cookies ) ) {
					$non_necessary_cookies[ $term_slug ] = $cs_cookies;
				}
			}
		}

		return $non_necessary_cookies;
	}

	public function get_non_necessary_cookie_ids() {

		$args = array(
			'post_type'      => CMPRO_POST_TYPE_SCRIPTS,
			'posts_per_page' => -1,
			'tax_query'      => array(
				array(
					'taxonomy' => 'cs-cookies-category',
					'field'    => 'slug',
					'terms'    => 'necessary',
					'operator' => 'NOT IN',
				)
			)

		);

		$posts = get_posts( $args );

		if ( !$posts ) {
			return;
		}
		$cookie_slugs = array();

		if ( $posts ) {
			foreach ( $posts as $post ) {
				$cookie_slugs[] = get_post_field( 'post_name', $post->ID, true );
			}
		}

		return $cookie_slugs;
	}

	/**
	 * Checking for "disable_cm" shortcode
	 */
	public function cs_check_disable_cm( $unblocked_user_ip = null ) {
		return $this->check_disable( $unblocked_user_ip );
	}

	public function cs_get_active_data() {

		check_ajax_referer( 'cs-ajax-public-nonce', 'nonce_code' );

		//check throttle
		if ( !$this->checkThrottle( 'cs_get_active_data' ) ) {
			wp_send_json_error();
			wp_die();
		}

		$active_rule_id = ConsentMagic()->get_active_rule_id();

		$force_action = '';
		$unblocked_ip = ConsentMagic()->get_unblocked_user_ip();
		$ip           = get_client_ip();

		if ( ( isset( $unblocked_ip ) && is_array( $unblocked_ip ) )
		     && !empty( array_intersect( $unblocked_ip, array( $ip ) ) ) ) {
			$force_action = 'unblock';
		}

		$this->current_lang       = !empty( $_POST[ 'lang' ] ) ? sanitize_text_field( $_POST[ 'lang' ] ) : get_locale();
		$cs_user_default_language = ConsentMagic()->getOption( 'cs_user_default_language' );
		$cs_language_availability = ConsentMagic()->getOption( 'cs_language_availability' );

		if ( isset( $cs_language_availability[ $this->current_lang ] )
		     && $cs_language_availability[ $this->current_lang ] == 0 ) {
			$this->current_lang = $cs_user_default_language;
		}

		if ( empty( $this->current_lang ) ) {
			$this->current_lang = 'en_US';
		}

		wp_send_json_success( array(
			'status'       => true,
			'active_rule'  => ( !empty( $active_rule_id ) ) ? get_post_meta(
				$active_rule_id,
				'_cs_type',
				true
			) : 'false',
			'content'      => $this->get_popup_content(),
			'cs_data'      => $this->get_cs_data(),
			'force_action' => $force_action
		) );
		wp_die();
	}

	/**
	 * Get vendor list from ajax request
	 * @return void
	 */
	public function cs_get_active_vendor_list() {

		// Check nonce:
		check_ajax_referer( 'cs-ajax-public-nonce', 'nonce_code' );

		//check throttle
		if ( !$this->checkThrottle( 'cs_get_active_vendor_list' ) ) {
			wp_send_json_error();
			wp_die();
		}

		wp_send_json_success( array(
			'status'      => true,
			'vendor_list' => json_encode( CS_IAB_Integration()->get_vendor_list() ),
		) );
		wp_die();
	}

	/**
	 * Checking whether a sticky button is displayed for each category of cookies
	 * @param $type
	 * @return bool
	 */
	public function cs_check_cookie_categories_for_sticky( $type ) {

		if ( $type == 'desktop' ) {
			$key = '_cs_smart_sticky';
		} elseif ( $type == 'mobile' ) {
			$key = '_cs_smart_sticky_mobile';
		} else {
			return true;
		}

		$active_rule_id = ConsentMagic()->get_active_rule_id();
		$sticky         = get_post_meta( $active_rule_id, $key, true );
		$show           = false;

		if ( $sticky ) {
			$unassigned    = get_term_by( 'slug', 'unassigned', 'cs-cookies-category' );
			$unassigned_id = $unassigned->term_id;
			$categories    = get_cookies_terms_objects( null, false, $unassigned_id );

			if ( !empty( $categories ) ) {
				foreach ( $categories as $category ) {
					$ignore_category = (int) get_term_meta( $category->term_id, 'cs_ignore_this_category', true );

					if ( $ignore_category === 0
					     && ( !CS_Cookies()->getCookie( 'cs_enabled_cookie_term_' . $category->term_id )
					          || ( CS_Cookies()->getCookie( 'cs_enabled_cookie_term_' . $category->term_id )
					               && CS_Cookies()->getCookie( 'cs_enabled_cookie_term_' . $category->term_id ) == 'no' ) ) ) {
						$sticky_category = get_post_meta( $active_rule_id, $key . '_' . $category->term_id, true );

						if ( $sticky_category === '1' ) {
							$show = true;
							break;
						}
					}
				}
			}
		}

		return $show;
	}

	/**
	 * Update sticky button ajax request
	 * @return void
	 */
	public function cs_update_sticky_button() {

		// Check nonce:
		check_ajax_referer( 'cs-ajax-public-nonce', 'nonce_code' );

		//check throttle
		if ( !$this->checkThrottle( 'cs_update_sticky_button' ) ) {
			wp_send_json_error();
			wp_die();
		}

		wp_send_json_success( array(
			'status'        => true,
			'sticky_button' => $this->cs_check_cookie_categories_for_sticky(
				sanitize_text_field( $_POST[ 'type' ] ) ?? ''
			),
		) );
		wp_die();
	}

	/**
	 * Add wprocket exclusions
	 * @return void
	 */
	public function add_wprocket_exclusions() {
		add_filter( 'rocket_lrc_exclusions', function( $exclusions ) {
			$exclusions[] = 'div id="cs_preview_popup"';
			$exclusions[] = 'div id="cs_preview_popup_button"';

			return $exclusions;
		} );
	}

	public function add_body_class( $classes ) {
		$classes[] = 'cm-manage-google-fonts';

		return $classes;
	}
}