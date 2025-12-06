<?php

namespace ConsentMagicPro;

if ( !defined( 'ABSPATH' ) ) {
	exit; // Exit if accessed directly.
}

class CS_Script_Blocker {

	/**
	 * $_instance
	 */
	private static $_instance = null;

	public $version;

	public $buffer_type = 1;

	public $cs_block_video_personal_data = false;

	public static $custom_scripts_list;

	private $front_scripts = array();

	/**
	 * $_instance CS_Script_Blocker
	 * @return CS_Script_Blocker|null
	 */
	public static function instance() {
		if ( is_null( self::$_instance ) ) {
			self::$_instance = new self();
		}

		return self::$_instance;
	}

	private function cs_check_sb_status() {
		return ConsentMagic()->getOption( 'cs_plugin_activation' ) == 1;
	}

	public function __construct( $isFirst = true ) {

		include_once ABSPATH . 'wp-admin/includes/plugin.php';

		$active_rule_id = ConsentMagic()->get_active_rule_id();
		if ( empty( $active_rule_id ) ) {
			return;
		}

		$cache                              = (int) ConsentMagic()->getOption( 'cs_enable_site_cache' );
		$this->cs_block_video_personal_data = ConsentMagic()->getOption( 'cs_block_video_personal_data' );

		$this->set_custom_scripts_list();

		$this->front_scripts = array(
			'data_layers' => array(),
		);

		if ( is_plugin_active( CMPRO_PLUGIN_BASENAME ) && get_post_meta( $active_rule_id, '_cs_type', true ) ) {

			if ( $isFirst ) {

				//if bot or not active rule - script not working
				if ( ConsentMagic()->get_unblocked_crawlers()
				     || $active_rule_id == 0
				     || !ConsentMagic()->ready_to_run() ) {
					return;
				}

				$this->render_video_placeholder();

				$user         = wp_get_current_user();
				$ignore_roles = ConsentMagic()->getOption( 'cs_admin_ignore_users' );

				$unblocked_ip = ConsentMagic()->get_unblocked_user_ip();
				$ip           = get_client_ip();

				if ( isset( $user->roles )
				     && $user->ID !== 0
				     && is_array( $ignore_roles )
				     && is_array( $unblocked_ip ) ) {
					if ( $cache != 0 ) {
						$this->cs_run_blocker();
					} else {
						if ( !array_intersect( $ignore_roles, $user->roles )
						     && !array_intersect( $unblocked_ip, array( $ip ) ) ) {
							$this->cs_run_blocker();
						}
					}

				} elseif ( isset( $user->roles )
				           && $user->ID == 0
				           && is_array( $ignore_roles )
				           && is_array( $unblocked_ip ) ) {
					if ( $cache != 0 ) {
						$this->cs_run_blocker();
					} else {
						if ( !array_intersect( $ignore_roles, array( 'visitor' ) )
						     && !array_intersect( $unblocked_ip, array( $ip ) ) ) {
							$this->cs_run_blocker();
						}
					}

				} else {
					$this->cs_run_blocker();
				}
			}
		}
	}

	/**
	 *  Run the blocker
	 */
	private function cs_run_blocker() {

		$cs_sb_enabled = $this->cs_check_sb_status();
		$is_ok         = false;

		if ( $cs_sb_enabled ) {
			$is_ok = true;
		}

		if ( $this->cs_sb_disable() ) {
			$is_ok = false;
		}

		if ( $is_ok ) {

			//check Elementor
			if ( is_plugin_active( 'elementor/elementor.php' )
			     || is_plugin_active( 'elementor-pro/elementor-pro.php' ) ) {
				if ( !class_exists( 'CS_Elementor' ) ) {
					$script_file = CMPRO_PLUGIN_SCRIPTS_PATH . "/third-party/CS_Elementor.php";
					require_once $script_file;
				}

				add_action( 'elementor/element/after_add_attributes', array(
					CS_Elementor::instance(),
					'check_elementor_widgets'
				), 200, 1 );
			}

			//checking buffer type
			$this->buffer_type = $this::cs_get_buffer_type();

			add_action( 'template_redirect', array(
				$this,
				'cs_start_sb'
			), 9 );

			if ( $this->buffer_type == 2 ) {
				remove_action( 'shutdown', 'wp_ob_end_flush_all', 1 );
				add_action( 'shutdown', array(
					$this,
					'cs_start_sb'
				), 1 );
			}
		}
	}

	/**
	 *Add Editor Support
	 * @return    boolean
	 * @since     1.0.0
	 */
	public function cs_sb_disable() {

		$disabled = false;
		if ( isset( $_GET[ 'elementor-preview' ] ) ) {
			$disabled = true;
		}

		return apply_filters( 'cs_disable_sb', $disabled );
	}

	public static function cs_get_buffer_type() {

		if ( !ConsentMagic()->getOption( 'cs_sb_buffer_type' ) ) {
			ConsentMagic()->updateOptions( array( 'cs_sb_buffer_type' => 1 ) );

			return 1;
		} else {
			return ConsentMagic()->getOption( 'cs_sb_buffer_type' );
		}
	}

	public function cs_start_sb() {

		//this is needed for Thrive Architect template redirect
		global $wp_filter;
		if ( isset( $wp_filter[ 'template_redirect' ][ 9999 ] )
		     && is_array( $wp_filter[ 'template_redirect' ][ 9999 ] ) ) {
			$run = false;
			foreach ( $wp_filter[ 'template_redirect' ][ 9999 ] as $key => $value ) {

				if ( stripos( $key, 'cs_start_sb' ) !== false ) {
					$run = true;
				}
			}

			if ( !$run ) {
				return;
			}

		} else {
			if ( !has_action( 'template_redirect', 'tcb_custom_editable_content' ) ) {

				add_action( 'template_redirect', array(
					$this,
					'cs_start_sb'
				), 9999 );

				return;
			}
		}

		ob_start();
		if ( $this->buffer_type == 1 ) {
			ob_start( array(
				$this,
				'cs_end_sb'
			), 0, PHP_OUTPUT_HANDLER_FLUSHABLE | PHP_OUTPUT_HANDLER_REMOVABLE );
		}
	}

	public function cs_takeBuffer() {

		$buffer_option = ConsentMagic()->getOption( 'cs_sb_buffer_option' );
		if ( $buffer_option ) {
			$buffer_option = ConsentMagic()->getOption( 'cs_sb_buffer_option' );
		} else {
			$buffer_option = $this::cs_decideBuffer();
			ConsentMagic()->updateOptions( array( 'cs_sb_buffer_option' => $buffer_option ) );
		}
		$buffer_escaped = '';
		if ( $buffer_option == 1 ) {
			$level = @ob_get_level();
			for ( $i = 0; $i < $level; $i++ ) {
				$buffer_escaped .= @ob_get_clean();
			}
		} else {
			$buffer_escaped = @ob_get_contents();
			@ob_end_clean();
		}
		print_r( $buffer_escaped );

		return $buffer_escaped;
	}

	public static function cs_decideBuffer() {

		$level = @ob_get_level();
		if ( version_compare( PHP_VERSION, '5.3.0' ) < 0 ) {
			$buffer_option = 1;//multi level
		} else {
			if ( $level > 1 ) {
				$buffer_option = 1;
			} else {
				$buffer_option = 2;
			}
		}

		return $buffer_option;
	}

	public function cs_end_sb( $buffer_escaped = "" ) {

		$active_rule_id = ConsentMagic()->get_active_rule_id();
		$cs_type        = get_post_meta( $active_rule_id, '_cs_type', true );

		if ( $this->buffer_type == 2 ) {
			$buffer_escaped = $this->cs_takeBuffer();
		}
		try {
			$cache              = (int) ConsentMagic()->getOption( 'cs_enable_site_cache' );
			$script_list        = $this->get_cs_blocker_script_list();
			$script_list_manual = $this->get_cs_blocker_script_list_manual();
			$thirdPartyScript   = $this->cs_getScriptPatterns();

			$scripts        = '';
			$scripts_manual = '';
			$cs_placeholder = '';
			$scripts        = apply_filters( 'cs_extend_script_blocker', $scripts );
			$scripts_manual = apply_filters( 'cs_extend_script_blocker', $scripts_manual );

			if ( $scripts && is_array( $scripts ) ) {
				foreach ( $scripts as $k => $v ) {
					$ignore = (int) get_term_meta( $v[ 'cat_id' ], 'cs_ignore_this_category', true );
					if ( $ignore == 0 ) {
						$cs_is_html_element = $cs_html_element = false;
						if ( isset( $v[ 'html_elem' ] ) ) {
							$cs_is_html_element = true;
							$cs_html_element    = $v[ 'html_elem' ];
						}
						if ( isset( $v[ 'placeholder' ] ) ) {
							$cs_placeholder = $v[ 'placeholder' ];
						}
						$thirdPartyScript[ $v[ 'id' ] ] = array(
							'label'            => esc_html__( $v[ 'label' ], 'consent-magic' ),
							'js_needle'        => $v[ 'key' ],
							'js'               => $v[ 'key' ][ 0 ],
							'cc'               => $v[ 'category' ],
							'noscript_tag'     => $v[ 'key' ],
							'has_s'            => false,
							'has_js'           => true,
							'has_js_needle'    => true,
							'has_noscript_tag' => false,
							'has_uri'          => false,
							'has_cc'           => false,
							'has_html_elem'    => $cs_is_html_element,
							'internal_cb'      => true,
							's'                => false,
							'uri'              => false,
							'html_elem'        => $cs_html_element,
							'callback'         => 'cs_automateDefault',
							'placeholder'      => $cs_placeholder,
							'slug'             => $k
						);
						$temp_scripts                   = new \stdClass();
						$temp_scripts->cliscript_status = $v[ 'status' ];
						$temp_scripts->cliscript_key    = $v[ 'id' ];
						$temp_scripts->category_slug    = $v[ 'category' ];
						$temp_scripts->loadonstart      = 0;
						array_push( $script_list, $temp_scripts );
					}
				}
			}
			if ( $scripts_manual && is_array( $scripts_manual ) ) {
				foreach ( $scripts_manual as $k => $v ) {
					$ignore = (int) get_term_meta( $v[ 'cat_id' ], 'cs_ignore_this_category', true );
					if ( $ignore == 0 ) {
						$cs_is_html_element = $cs_html_element = false;
						if ( isset( $v[ 'html_elem' ] ) ) {
							$cs_is_html_element = true;
							$cs_html_element    = $v[ 'html_elem' ];
						}
						if ( isset( $v[ 'placeholder' ] ) ) {
							$cs_placeholder = $v[ 'placeholder' ];
						}
						$thirdPartyScript[ $v[ 'id' ] ] = array(
							'label'            => esc_html__( $v[ 'label' ], 'consent-magic' ),
							'js_needle'        => $v[ 'key' ],
							'js'               => $v[ 'key' ][ 0 ],
							'cc'               => $v[ 'category' ],
							'noscript_tag'     => $v[ 'key' ],
							'has_s'            => false,
							'has_js'           => true,
							'has_js_needle'    => true,
							'has_noscript_tag' => false,
							'has_uri'          => false,
							'has_cc'           => false,
							'has_html_elem'    => $cs_is_html_element,
							'internal_cb'      => true,
							's'                => false,
							'uri'              => false,
							'html_elem'        => $cs_html_element,
							'callback'         => 'cs_automateDefault',
							'placeholder'      => $cs_placeholder,
							'slug'             => $k
						);
						$temp_scripts                   = new \stdClass();
						$temp_scripts->cliscript_status = $v[ 'status' ];
						$temp_scripts->cliscript_key    = $v[ 'id' ];
						$temp_scripts->category_slug    = $v[ 'category' ];
						$temp_scripts->loadonstart      = 0;
						array_push( $script_list_manual, $temp_scripts );
					}
				}
			}
			if ( !empty( $script_list ) || !empty( $script_list_manual ) ) {

				//add script data
				$this->add_script_data( $thirdPartyScript, $script_list );

				//add placeholder manual scripts
				$this->add_placeholder_manual_scripts( $thirdPartyScript, $script_list_manual );

			} else //unable to load cookie table data - May DB error.
			{
				if ( $this->buffer_type == 2 ) {
					echo $buffer_escaped;
					exit();
				} else {
					return $buffer_escaped;
				}
			}

			//render check script
			$this->renderCheckScript( $thirdPartyScript, $active_rule_id, $cache, $cs_type );

			//Apply blocking
			$buffer_escaped = $this->cs_beforeAutomate( $buffer_escaped );
			$buffer_escaped = $this->apply_blocking(
				$buffer_escaped,
				$thirdPartyScript,
				$active_rule_id,
				$cache,
				$cs_type
			);
			$buffer_escaped = $this->cs_afterAutomate( $buffer_escaped );

			if ( $this->buffer_type == 2 ) {
				echo $buffer_escaped;
				exit();
			} else {
				return $buffer_escaped;
			}
		} catch ( \Exception $e ) {

			$message = $e->getMessage();
			if ( '' !== $message && defined( 'WP_DEBUG' ) && WP_DEBUG ) {
				error_log( 'Error: ' . $message . ' in ' . $e->getFile() . ':' . $e->getLine() );
			}
			if ( $this->buffer_type == 2 ) {
				echo $buffer_escaped;
				exit();
			} else {
				return $buffer_escaped;
			}
		}
		if ( $this->buffer_type == 2 ) {
			echo $buffer_escaped;
			exit();
		} else {
			return $buffer_escaped;
		}
	}

	/**
	 * Render check script
	 * @param $thirdPartyScript
	 * @param $active_rule_id
	 * @param $cache
	 * @param $cs_type
	 * @return void
	 */
	private function renderCheckScript( &$thirdPartyScript, $active_rule_id, $cache, $cs_type ) {

		//  $thirdPartyScript[$key]['check'] = true; - if true  - it will replace the code - it means the code will not render

		$test_prefix = false;
		if ( ConsentMagic()->getOption( 'cs_test_mode' ) ) {
			$test_prefix = '_test';
		}
		$viewed_cookie      = "cs_viewed_cookie_policy" . $test_prefix;
		$cs_track_analytics = get_post_meta( $active_rule_id, '_cs_track_analytics', true );
		$analytics_cat_id   = ConsentMagic()->getOption( 'analytics_cat_id' );

		foreach ( $thirdPartyScript as $k => $v ) {
			if ( isset( $v[ 'category' ] ) ) {
				$category_cookie = "cs_enabled_cookie_term" . $test_prefix . '_' . $v[ 'category' ];

				if ( $cache != 0 ) {
					$thirdPartyScript[ $k ][ 'check' ] = true; //if cache enabled - block scripts
				} else {
					if ( CS_Cookies()->getCookie( $category_cookie ) && CS_Cookies()->getCookie( $viewed_cookie ) ) {
						if ( CS_Cookies()->getCookie( $category_cookie ) == 'yes'
						     && CS_Cookies()->getCookie(
								$viewed_cookie
							) == 'yes' ) {
							$thirdPartyScript[ $k ][ 'check' ] = false;
							//allowed by user then false
						} else {
							$thirdPartyScript[ $k ][ 'check' ] = true; //not allowed by user then true
						}
					} else {
						if ( $cs_type == 'iab' || $cs_type == 'ask_before_tracking' ) {
							if ( (int) $cs_track_analytics === 1 && $analytics_cat_id == $v[ 'category' ] ) {
								if ( isset( $v[ 'additional_consent_ga' ] ) && $v[ 'additional_consent_ga' ] === 1 ) {
									$thirdPartyScript[ $k ][ 'check' ] = false; //default it is false - need answer before track analytic
								} else {
									$thirdPartyScript[ $k ][ 'check' ] = true;
								}
							} else {
								$thirdPartyScript[ $k ][ 'check' ] = true; //default it is true so blocks the code
							}
						} else {
							$thirdPartyScript[ $k ][ 'check' ] = false; //this is rule "inform_and_opiout" OR "just_inform"
						}
					}
				}
			} else {
				$thirdPartyScript[ $k ][ 'check' ] = false; //not configured from admin then false;
			}
		}
	}

	public function checkBlockingEmbeddedVideo( $videoCat, $active_rule_id, $cache ) {

		$test_prefix = false;
		if ( ConsentMagic()->getOption( 'cs_test_mode' ) ) {
			$test_prefix = '_test';
		}
		$viewed_cookie      = "cs_viewed_cookie_policy" . $test_prefix;
		$cs_track_analytics = get_post_meta( $active_rule_id, '_cs_track_analytics', true );
		$analytics_cat_id   = ConsentMagic()->getOption( 'analytics_cat_id' );
		$category_cookie    = "cs_enabled_cookie_term" . $test_prefix . '_' . $videoCat;
		$cs_type            = get_post_meta( $active_rule_id, '_cs_type', true );
		$necessary_cat_id   = ConsentMagic()->getOption( 'necessary_cat_id' );
		$ignore             = (int) get_term_meta( $videoCat, 'cs_ignore_this_category', true );

		if ( $cache != 0 ) {
			$blocking = true; //if cache enabled - block scripts
		} else {
			if ( $necessary_cat_id == $videoCat || $ignore == 1 ) {
				$blocking = false; //if necessary category - do not block
			} else {
				if ( CS_Cookies()->getCookie( $category_cookie ) && CS_Cookies()->getCookie( $viewed_cookie ) ) {
					if ( CS_Cookies()->getCookie( $category_cookie ) == 'yes'
					     && CS_Cookies()->getCookie(
							$viewed_cookie
						) == 'yes' ) {
						$blocking = false;
						//allowed by user then false
					} else {
						$blocking = true; //not allowed by user then true
					}
				} else {
					if ( $cs_type == 'iab' || $cs_type == 'ask_before_tracking' ) {
						if ( (int) $cs_track_analytics === 1 && $analytics_cat_id == $videoCat ) {
							$blocking = false; //default it is false - need answer before track analytic
						} else {
							$blocking = true; //default it is true so blocks the code
						}
					} else {
						$blocking = false; //this is rule "inform_and_opiout" OR "just_inform"
					}
				}
			}
		}

		return $blocking;
	}

	/**
	 * Add placeholder for manually added scripts
	 * @param $thirdPartyScript
	 * @param $script_list_manual
	 * @return void
	 */
	private function add_placeholder_manual_scripts( &$thirdPartyScript, $script_list_manual ) {
		if ( !empty( $script_list_manual ) ) {
			$scripts = array();
			$scripts = apply_filters( 'cs_add_placeholder', $scripts );
			foreach ( $script_list_manual as $k => $v ) {
				$ignore = (int) get_term_meta( $v[ 'cat_id' ], 'cs_ignore_this_category', true );
				if ( $ignore == 0 ) {
					$placeholder_text = esc_html__( 'Accept consent to view this', 'consent-magic' );
					$scriptkey        = $k;
					if ( isset( $thirdPartyScript[ $scriptkey ] ) ) {
						//assign category slug for admin enabled categories
						if ( $v[ 'activation' ] == 1
						     && ConsentMagic()->getOption( 'cs_block_manually_added_cookies' ) == 1 ) {

							$thirdPartyScript[ $scriptkey ][ 'block_script' ]  = 'true';
							$thirdPartyScript[ $scriptkey ][ 'category' ]      = '';
							$thirdPartyScript[ $scriptkey ][ 'category_name' ] = '';
							if ( $v[ 'category_slug' ] != "" && $v[ 'category_slug' ] !== null ) { //a category assigned
								$category = get_term_by(
									'slug',
									$v[ 'category_slug' ],
									'cs-cookies-category'
								);
								if ( $category && !is_wp_error( $category ) ) {
									$thirdPartyScript[ $scriptkey ][ 'category' ]      = $category->term_id;
									$thirdPartyScript[ $scriptkey ][ 'category_name' ] = $category->name;

									$cs_video_general_text = ConsentMagic()->getOption(
										'cs_video_consent_general_text'
									);
									$cs_video_rule_text    = ConsentMagic()->getOption(
										'cs_video_consent_rule_text'
									);
									$placeholder_text      = sprintf(
										"<div><p>%s</p></div><div><a class='cs_manage_current_consent' data-cs-script-type='%s' data-cs-manage='manage_placeholder'>%s %s</a></div>",
										$cs_video_general_text,
										$category->term_id,
										$cs_video_rule_text,
										$category->name
									);
								}
							}
							if ( $scripts && is_array( $scripts ) ) {
								if ( isset( $scripts[ $scriptkey ] ) ) {
									$cs_custom_script = $scripts[ $scriptkey ];
									if ( isset( $cs_custom_script[ 'placeholder' ] ) ) {
										$placeholder_text = $cs_custom_script[ 'placeholder' ];
									}
								}
							}
							$thirdPartyScript[ $scriptkey ][ 'placeholder' ] = $placeholder_text;
						} else //only codes that was enabled by admin. Unset other items
						{
							unset( $thirdPartyScript[ $scriptkey ] );
						}
					}
				}
			}
		}
	}

	/**
	 * Add script data
	 * @param $thirdPartyScript
	 * @param $script_list
	 * @return void
	 */
	private function add_script_data( &$thirdPartyScript, $script_list ) {
		if ( !empty( $script_list ) ) {
			$scripts = array();

			$scripts = apply_filters( 'cs_add_placeholder', $scripts );
			foreach ( $script_list as $k => $v ) {
				$ignore = (int) get_term_meta( $v[ 'cat_id' ], 'cs_ignore_this_category', true );
				if ( $ignore == 0 ) {
					$scriptkey = $k;
					if ( isset( $thirdPartyScript[ $scriptkey ] ) ) {
						//assign category slug for admin enabled categories
						if ( $v[ 'activation' ] == 1
						     && ConsentMagic()->getOption( 'cs_block_pre_defined_scripts' ) == 1 ) {

							$thirdPartyScript[ $scriptkey ][ 'block_script' ]  = 'true';
							$thirdPartyScript[ $scriptkey ][ 'category' ]      = '';
							$thirdPartyScript[ $scriptkey ][ 'category_name' ] = '';
							$thirdPartyScript[ $scriptkey ][ 'slug' ]          = $scriptkey;
							$placeholder_text = '';
							if ( $v[ 'category_slug' ] != "" && $v[ 'category_slug' ] !== null ) { //a category assigned
								$category = get_term_by(
									'slug',
									$v[ 'category_slug' ],
									'cs-cookies-category'
								);
								if ( $category && !is_wp_error( $category ) ) {
									$thirdPartyScript[ $scriptkey ][ 'category' ]      = $category->term_id;
									$thirdPartyScript[ $scriptkey ][ 'category_name' ] = $category->name;
									$placeholder_text                                  = apply_filters(
										'cm_video_placeholder',
										''
									);
								}
							}

							if ( $scripts && is_array( $scripts ) ) {
								if ( isset( $scripts[ $scriptkey ] ) ) {
									$cs_custom_script = $scripts[ $scriptkey ];
									if ( isset( $cs_custom_script[ 'placeholder' ] ) ) {
										$placeholder_text = $cs_custom_script[ 'placeholder' ];
									}
								}
							}
							$thirdPartyScript[ $scriptkey ][ 'placeholder' ] = $placeholder_text;
						} else //only codes that was enabled by admin. Unset other items
						{
							unset( $thirdPartyScript[ $scriptkey ] );
						}
					}
				}
			}
		}
	}

	/**
	 * Apply blocking for scripts
	 * @param $buffer_escaped
	 * @param $thirdPartyScript
	 * @param $active_rule_id
	 * @param $cs_type
	 * @param $cache
	 * @return mixed|string
	 */
	private function apply_blocking( $buffer_escaped, $thirdPartyScript, $active_rule_id, $cache, $cs_type ) {
		$parts = $this->cs_getHeadBodyParts( $buffer_escaped );
		if ( $parts ) {

			//check for advanced Google mode
			if ( CS_Google_Consent_Mode()->enabled_google_consent_mode() ) {
				$parts = $this->check_additional_scripts( $parts );
			}

			if ( ConsentMagic()->getOption( 'cs_script_blocking_enabled' )
			     && ( ( $cs_type
			            && $cs_type != 'just_inform' )
			          || $cache != 0 ) ) {
				$native_scripts_option = get_post_meta( $active_rule_id, '_cs_native_scripts', true );
				$necessary_cat_id      = ConsentMagic()->getOption( 'necessary_cat_id' );

				if ( $cache || ( $cs_type == 'iab' && $native_scripts_option == 1 ) || $cs_type != 'iab' ) {
					foreach ( $thirdPartyScript as $type => $autoData ) {
						if ( isset( $autoData[ 'category' ] ) && $autoData[ 'category' ] != $necessary_cat_id ) {
							if ( !isset( $autoData[ 'callback' ] ) ) {
								$autoData[ 'callback' ] = 'cs_automateDefault';
							}
							$callback = $autoData[ 'callback' ];

							if ( $autoData[ 'internal_cb' ] ) {
								$callback = array(
									$this,
									$callback
								);
							}
							// set parameters for preg_replace_callback() callback
							$parts = call_user_func_array( $callback, array(
								$type,
								$autoData,
								$parts
							) );
						}
					}
				}
			}

			$buffer_escaped = $parts[ 'head' ] . $parts[ 'split' ] . $parts[ 'body' ];
		}

		return $buffer_escaped;
	}

	public function get_cs_blocker_script_list() {

		$data = array(
			'facebook_pixel'  => array(
				'activation'    => ConsentMagic()->getOption( 'cs_block_fb_pixel_scripts' ),
				'category_slug' => ConsentMagic()->getOption( 'cs_block_fb_pixel_scripts_cat' ),
				'cat_id'        => ConsentMagic()->getOption(
					ConsentMagic()->getOption( 'cs_block_fb_pixel_scripts_cat' ) . '_cat_id'
				)
			),
			'googleanalytics' => array(
				'activation'    => ConsentMagic()->getOption( 'cs_block_google_analytics_scripts' ),
				'category_slug' => ConsentMagic()->getOption( 'cs_block_google_analytics_scripts_cat' ),
				'cat_id'        => ConsentMagic()->getOption(
					ConsentMagic()->getOption( 'cs_block_google_analytics_scripts_cat' ) . '_cat_id'
				)
			),
			'youtube_embed'   => array(
				'activation'    => ConsentMagic()->getOption( 'cs_block_yt_embedded_scripts' ),
				'category_slug' => ConsentMagic()->getOption( 'cs_block_yt_embedded_scripts_cat' ),
				'cat_id'        => ConsentMagic()->getOption(
					ConsentMagic()->getOption( 'cs_block_yt_embedded_scripts_cat' ) . '_cat_id'
				)
			),
			'vimeo_embed'     => array(
				'activation'    => ConsentMagic()->getOption( 'cs_block_vimeo_embedded_scripts' ),
				'category_slug' => ConsentMagic()->getOption( 'cs_block_vimeo_embedded_scripts_cat' ),
				'cat_id'        => ConsentMagic()->getOption(
					ConsentMagic()->getOption( 'cs_block_vimeo_embedded_scripts_cat' ) . '_cat_id'
				)
			),
			'twitter_widget'  => array(
				'activation'    => ConsentMagic()->getOption( 'cs_block_tw_tag_scripts' ),
				'category_slug' => ConsentMagic()->getOption( 'cs_block_tw_tag_scripts_cat' ),
				'cat_id'        => ConsentMagic()->getOption(
					ConsentMagic()->getOption( 'cs_block_tw_tag_scripts_cat' ) . '_cat_id'
				)
			),
			'twitter'         => array(
				'activation'    => ConsentMagic()->getOption( 'cs_block_twitter_scripts' ),
				'category_slug' => ConsentMagic()->getOption( 'cs_block_twitter_scripts_cat' ),
				'cat_id'        => ConsentMagic()->getOption(
					ConsentMagic()->getOption( 'cs_block_twitter_scripts_cat' ) . '_cat_id'
				)
			),
			'linkedin_widget' => array(
				'activation'    => ConsentMagic()->getOption( 'cs_block_ln_tag_scripts' ),
				'category_slug' => ConsentMagic()->getOption( 'cs_block_ln_tag_scripts_cat' ),
				'cat_id'        => ConsentMagic()->getOption(
					ConsentMagic()->getOption( 'cs_block_ln_tag_scripts_cat' ) . '_cat_id'
				)
			),
			'pinterest'       => array(
				'activation'    => ConsentMagic()->getOption( 'cs_block_pin_tag_scripts' ),
				'category_slug' => ConsentMagic()->getOption( 'cs_block_pin_tag_scripts_cat' ),
				'cat_id'        => ConsentMagic()->getOption(
					ConsentMagic()->getOption( 'cs_block_pin_tag_scripts_cat' ) . '_cat_id'
				)
			),
			'google_ads_tag'  => array(
				'activation'    => ConsentMagic()->getOption( 'cs_block_google_ads_tag_scripts' ),
				'category_slug' => ConsentMagic()->getOption( 'cs_block_google_ads_tag_scripts_cat' ),
				'cat_id'        => ConsentMagic()->getOption(
					ConsentMagic()->getOption( 'cs_block_google_ads_tag_scripts_cat' ) . '_cat_id'
				)
			),
			'bing_tag'        => array(
				'activation'    => ConsentMagic()->getOption( 'cs_block_big_tag_scripts' ),
				'category_slug' => ConsentMagic()->getOption( 'cs_block_big_tag_scripts_cat' ),
				'cat_id'        => ConsentMagic()->getOption(
					ConsentMagic()->getOption( 'cs_block_big_tag_scripts_cat' ) . '_cat_id'
				)
			),
			'google_adsense'  => array(
				'activation'    => ConsentMagic()->getOption( 'cs_block_google_adsense_scripts' ),
				'category_slug' => ConsentMagic()->getOption( 'cs_block_google_adsense_scripts_cat' ),
				'cat_id'        => ConsentMagic()->getOption(
					ConsentMagic()->getOption( 'cs_block_google_adsense_scripts_cat' ) . '_cat_id'
				)
			),
			'hubspot'         => array(
				'activation'    => ConsentMagic()->getOption( 'cs_block_hubspot_scripts' ),
				'category_slug' => ConsentMagic()->getOption( 'cs_block_hubspot_scripts_cat' ),
				'cat_id'        => ConsentMagic()->getOption(
					ConsentMagic()->getOption( 'cs_block_hubspot_scripts_cat' ) . '_cat_id'
				)
			),
			'matomo'          => array(
				'activation'    => ConsentMagic()->getOption( 'cs_block_matomo_scripts' ),
				'category_slug' => ConsentMagic()->getOption( 'cs_block_matomo_scripts_cat' ),
				'cat_id'        => ConsentMagic()->getOption(
					ConsentMagic()->getOption( 'cs_block_matomo_scripts_cat' ) . '_cat_id'
				)
			),
			'google_maps'     => array(
				'activation'    => ConsentMagic()->getOption( 'cs_block_google_maps_scripts' ),
				'category_slug' => ConsentMagic()->getOption( 'cs_block_google_maps_scripts_cat' ),
				'cat_id'        => ConsentMagic()->getOption(
					ConsentMagic()->getOption( 'cs_block_google_maps_scripts_cat' ) . '_cat_id'
				)
			),
			'googlefonts'     => array(
				'activation'    => ConsentMagic()->getOption( 'cs_block_googlefonts_scripts' ),
				'category_slug' => ConsentMagic()->getOption( 'cs_block_googlefonts_scripts_cat' ),
				'cat_id'        => ConsentMagic()->getOption(
					ConsentMagic()->getOption( 'cs_block_googlefonts_scripts_cat' ) . '_cat_id'
				)
			),
			'google_captcha'  => array(
				'activation'    => ConsentMagic()->getOption( 'cs_block_google_captcha_scripts' ),
				'category_slug' => ConsentMagic()->getOption( 'cs_block_google_captcha_scripts_cat' ),
				'cat_id'        => ConsentMagic()->getOption(
					ConsentMagic()->getOption( 'cs_block_google_captcha_scripts_cat' ) . '_cat_id'
				)
			),
			'addthis'         => array(
				'activation'    => ConsentMagic()->getOption( 'cs_block_addthis_scripts' ),
				'category_slug' => ConsentMagic()->getOption( 'cs_block_addthis_scripts_cat' ),
				'cat_id'        => ConsentMagic()->getOption(
					ConsentMagic()->getOption( 'cs_block_addthis_scripts_cat' ) . '_cat_id'
				)
			),
			'sharethis'       => array(
				'activation'    => ConsentMagic()->getOption( 'cs_block_sharethis_scripts' ),
				'category_slug' => ConsentMagic()->getOption( 'cs_block_sharethis_scripts_cat' ),
				'cat_id'        => ConsentMagic()->getOption(
					ConsentMagic()->getOption( 'cs_block_sharethis_scripts_cat' ) . '_cat_id'
				)
			),
			'soundcloud'      => array(
				'activation'    => ConsentMagic()->getOption( 'cs_block_soundcloud_scripts' ),
				'category_slug' => ConsentMagic()->getOption( 'cs_block_soundcloud_scripts_cat' ),
				'cat_id'        => ConsentMagic()->getOption(
					ConsentMagic()->getOption( 'cs_block_soundcloud_scripts_cat' ) . '_cat_id'
				)
			),
			'slideshare'      => array(
				'activation'    => ConsentMagic()->getOption( 'cs_block_slideshare_scripts' ),
				'category_slug' => ConsentMagic()->getOption( 'cs_block_slideshare_scripts_cat' ),
				'cat_id'        => ConsentMagic()->getOption(
					ConsentMagic()->getOption( 'cs_block_slideshare_scripts_cat' ) . '_cat_id'
				)
			),
			'instagram'       => array(
				'activation'    => ConsentMagic()->getOption( 'cs_block_instagram_scripts' ),
				'category_slug' => ConsentMagic()->getOption( 'cs_block_instagram_scripts_cat' ),
				'cat_id'        => ConsentMagic()->getOption(
					ConsentMagic()->getOption( 'cs_block_instagram_scripts_cat' ) . '_cat_id'
				)
			),
			'hotjar'          => array(
				'activation'    => ConsentMagic()->getOption( 'cs_block_hotjar_scripts' ),
				'category_slug' => ConsentMagic()->getOption( 'cs_block_hotjar_scripts_cat' ),
				'cat_id'        => ConsentMagic()->getOption(
					ConsentMagic()->getOption( 'cs_block_hotjar_scripts_cat' ) . '_cat_id'
				)
			),
			'tiktok_pixel'    => array(
				'activation'    => ConsentMagic()->getOption( 'cs_block_tiktok_scripts' ),
				'category_slug' => ConsentMagic()->getOption( 'cs_block_tiktok_scripts_cat' ),
				'cat_id'        => ConsentMagic()->getOption(
					ConsentMagic()->getOption( 'cs_block_tiktok_scripts_cat' ) . '_cat_id'
				)
			),
			'reddit_pixel'    => array(
				'activation'    => ConsentMagic()->getOption( 'cs_block_reddit_pixel_scripts' ),
				'category_slug' => ConsentMagic()->getOption( 'cs_block_reddit_pixel_scripts_cat' ),
				'cat_id'        => ConsentMagic()->getOption(
					ConsentMagic()->getOption( 'cs_block_reddit_pixel_scripts_cat' ) . '_cat_id'
				)
			),
		);

		$data[ 'google_captcha_cf7' ] = $data[ 'google_captcha' ];

		$s_list = self::$custom_scripts_list;
		foreach ( $s_list as $script ) {
			$data[ 'script_' . $script[ 'script_id' ] ] = array(
				'activation'    => ConsentMagic()->getOption( $script[ 'option_name' ] ),
				'category_slug' => ConsentMagic()->getOption( $script[ 'option_select_name' ] ),
				'cat_id'        => $script[ 'term_id' ]
			);
		}

		return $data;
	}

	public function get_cs_blocker_script_list_manual() {

		$data = array();

		$s_list = self::$custom_scripts_list;
		foreach ( $s_list as $script ) {
			$data[ 'script_' . $script[ 'script_id' ] ] = array(
				'activation'    => ConsentMagic()->getOption( $script[ 'option_name' ] ),
				'category_slug' => ConsentMagic()->getOption( $script[ 'option_select_name' ] ),
				'cat_id'        => $script[ 'term_id' ]
			);
		}

		return $data;
	}

	public function set_custom_scripts_list() {

		if ( self::$custom_scripts_list === null ) {
			global $wpdb;
			$querystr  = $wpdb->prepare(
				"SELECT * FROM $wpdb->posts WHERE $wpdb->posts.post_type = %s ",
				'cs-scripts'
			);
			$pageposts = $wpdb->get_results( $querystr );
			$options   = [];
			if ( $pageposts ) {
				foreach ( $pageposts as $script ) {
					$slug      = ConsentMagic()->getOption( 'cs_' . $script->ID . '_script_cat' );
					$queryterm = $wpdb->prepare( "SELECT * FROM $wpdb->terms WHERE $wpdb->terms.slug = %s", $slug );
					$result    = $wpdb->get_results( $queryterm );
					if ( $result ) {
						$option_name        = 'cs_' . sanitize_title( $script->post_title ) . '_' . $script->ID
						                      . '_script_enable';
						$option_select_name = 'cs_' . $script->ID . '_script_cat';
						$options[]          = array(
							'script_name'        => $script->post_title,
							'script_id'          => $script->ID,
							'term_id'            => $result[ 0 ]->term_id,
							'term_slug'          => $result[ 0 ]->slug,
							'option_name'        => $option_name,
							'option_select_name' => $option_select_name
						);
					}
				}
			}

			self::$custom_scripts_list = $options;
		}
	}

	public function cs_defineRegex() {
		return array(
			'_regexParts'                        => array(
				'-lookbehind_img'        => '(?<!src=")',
				'-lookbehind_link'       => '(?<!href=")',
				'-lookbehind_link_img'   => '(?<!href=")(?<!src=")',
				'-lookbehind_shortcode'  => '(?<!])',
				'-lookbehind_after_body' => '(?<=\<body\>)',
				'-lookahead_body_end'    => '(?=.*\</body\>)',
				'-lookahead_head_end'    => '(?=.*\</head\>)',
				'random_chars'           => '[^\s\["\']+',
				'src_scheme_www'         => '(?:https?://|//)?(?:[www\.]{4})?'
			),
			'_regexPatternScriptBasic'           => '\<script' . '.+?' . '\</script\>',
			'_regexPatternScriptTagOpen'         => '\<script[^\>]*?\>',
			'_regexPatternScriptTagClose'        => '\</script\>',
			'_regexPatternScriptAllAdvanced'     => '\<script' . '[^>]*?' . '\>' . '(' . '(?!\</script\>)' . '.*?' . ')'
			                                        . '?' . '\</script\>',
			'_regexPatternScriptHasNeedle'       => '\<script' . '[^>]*?' . '\>' . '(?!\</script>)' . '[^<]*' . '%s'
			                                        . '[\s\S]*?' . '\</script\>',
			'_regexPatternNoScriptHasNeedle'     => '\<noscript' . '[^>]*?' . '\>' . '(?!\</noscript>)' . '[^<]*'
			                                        . '[^>]*?' . '\<%s' . '[^<]*' . '%s'
			                                        . '([^<>]*/?\>|[^<]*\>.*\</%s\>)' . '[\w|\s]*' . '\</noscript\>',
			'_regexPatternScriptSrc'             => '\<script' . '[^>]+?' . 'src=' . '("|\')' . '(' . '(https?:)?'
			                                        . '//(?:[www\.]{4})?' . '[^\s"\']*?' . '%s' . '%s' . '[^\s"\']*?'
			                                        . ')' . '("|\')' . '[^>]*' . '\>' . '[^<]*' . '\</script\>',
			'_regexPatternIframeBasic'           => '\<iframe' . '.+?' . '\</iframe\>',
			'_regexPatternIframe'                => '\<iframe' . '[^>]+?' . 'src=' . '("|\')' . '(' . '(https?://|//)?'
			                                        . '(?:[www\.]{4})?' . '%s' . '%s' . '[^"\']*?' . ')' . '("|\')'
			                                        . '[^>]*' . '\>' . '(?:' . '(?!\<iframe).*?' . ')' . '\</iframe\>',
			'_regexPatternHtmlElemWithAttr'      => '\<%s' . '[^>]+?' . '%s\s*=\s*' . '(?:"|\')' . '(?:' . '%s' . '%s'
			                                        . '[^"\']*?' . ')' . '(?:"|\')' . '[^>]*' . '(?:' . '\>' . '('
			                                        . '(?!\<%s).*?' . ')' . '\</%s\>' . '|' . '/\>' . ')',
			'_regexPatternHtmlElemWithAttrTypeA' => '\<%s' . '[^>]+?' . '%s=' . '(?:"|\')' . '(?:' . '%s' . '%s'
			                                        . '[^"\']*?' . ')' . '(?:"|\')' . '[^>]*' . '(?:' . '\>' . ')',

		);
	}

	public function cs_beforeAutomate( $content ) {

		$textarr                     = wp_html_split( $content );
		$regex_patterns              = $this->cs_defineRegex();
		$_regexPatternScriptTagOpen  = $regex_patterns[ '_regexPatternScriptTagOpen' ];
		$_regexPatternScriptTagClose = $regex_patterns[ '_regexPatternScriptTagClose' ];
		$changed                     = false;
		$replacePairs                = array(
			"\r\n" => '_RNL_',
			"\n"   => '_NL_',
			'<'    => '_LT_'
		);

		foreach ( $replacePairs as $needle => $replace ) {
			foreach ( $textarr as $i => $html ) {
				if ( preg_match( "#^$_regexPatternScriptTagOpen#", $textarr[ $i ], $m ) ) {
					if ( false !== strpos( $textarr[ $i + 1 ], $needle ) ) {
						$textarr[ $i + 1 ] = str_replace( $needle, $replace, $textarr[ $i + 1 ] );
						$changed           = true;
					}

					if ( '<' === $needle
					     && $needle === $textarr[ $i + 2 ][ 0 ]
					     && '</script>' !== $textarr[ $i + 2 ] ) {
						$textarr[ $i + 2 ] = preg_replace( '#\<(?!/script\>)#', $replace, $textarr[ $i + 2 ] );
					}
				}
			}
		}
		if ( $changed ) {
			$content = implode( $textarr );
		}
		unset( $textarr );

		return $content;
	}

	public function cs_getHeadBodyParts( $buffer ) {
		if ( preg_match( '/<\/head>/i', $buffer, $headMatch, PREG_OFFSET_CAPTURE ) ) {
			$headClosePos = $headMatch[ 0 ][ 1 ];
			$headEndPos   = $headClosePos + strlen( $headMatch[ 0 ][ 0 ] );

			if ( preg_match( '/<body\b/i', $buffer, $bodyMatch, PREG_OFFSET_CAPTURE, $headEndPos ) ) {
				$bodyStart = $bodyMatch[ 0 ][ 1 ];
				$bodyEnd   = $bodyStart + strlen( '<body' );

				$parts = array(
					'head'  => substr( $buffer, 0, $headClosePos ),
					'split' => substr( $buffer, $headClosePos, $bodyEnd - $headClosePos ),
					'body'  => substr( $buffer, $bodyEnd ),
				);

				return $parts;
			}
		}

		return false;
	}

	public function cs_afterAutomate( $content ) {

		return str_replace( array(
			'_RNL_',
			'_NL_',
			'_LT_'
		), array(
			"\r\n",
			"\n",
			"<"
		), $content );
	}

	public function cs_getScriptPatterns() {

		$thirdPartyScript = array();

		$s_list = self::$custom_scripts_list;

		foreach ( $s_list as $script ) {
			$id        = $script[ 'script_id' ];
			$js_needle = array();
			if ( get_post_meta( $id, 'cs_default_script_js_heedle', true ) ) {
				$js_needle = explode( ",", get_post_meta( $id, 'cs_default_script_js_heedle', true ) );
			}
			$thirdPartyScript[ 'script_' . $id ] = array(
				'label'     => $script[ 'script_name' ],
				'js_needle' => $js_needle,
				'js'        => $js_needle,
				'cc'        => 'other',
			);
		}

		$wp_content = str_replace( ABSPATH, '', WP_PLUGIN_DIR );

		$thirdPartyScript[ 'googleanalytics' ] = array(
			'label'          => esc_html__( 'Google Analytics', 'consent-magic' ),
			's'              => array(
				'google-analytics.com',
				'www.googletagmanager.com/ns.html?id=GTM-'
			),
			'js'             => array(
				'www.google-analytics.com/analytics.js',
				$wp_content . '/woocommerce-google-adwords-conversion-tracking-tag/js/public/google_ads.js',
				'googletagmanager.com/gtag/js'
			),
			'js_needle'      => array(
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
			'js_not_contain' => array(
				'kadenceConversionsConfig',
				'pysOptions',
				'"gtag"',
				"'gtag'"
			),
			'cc'             => 'analytical',
			'html_elem'      => array(
				array(
					'name' => 'script',
					'attr' => 'id:gtmkit-edd-js'
				),
			),
		);

		$thirdPartyScript[ 'google_ads_tag' ] = array(
			'label'          => esc_html__( 'Google Publisher Tag', 'consent-magic' ),
			's'              => 'www.googletagmanager.com/ns.html?id=GTM-',
			'js'             => array(
				'www.googletagservices.com/tag/js/gpt.js',
				'www.googleadservices.com/pagead/conversion.js',
				$wp_content . '/woocommerce-google-adwords-conversion-tracking-tag/js/public/google_ads.js',
				'googletagmanager.com/gtag/js',
				$wp_content . '/gtm-kit/assets/integration/woocommerce.js'
			),
			'js_needle'      => array(
				'googletag.pubads',
				'googletag.enableServices',
				'googletag.display',
				'wooptpmDataLayer',
				'www.googletagservices.com/tag/js/gpt.js',
				'www.googleadservices.com/pagead/conversion.js',
				'www.googletagmanager.com/gtm',
				'gtag',
				'gtmkit_dataLayer_content',
			),
			'js_not_contain' => array(
				'kadenceConversionsConfig',
				'pysOptions',
				'"gtag"',
				"'gtag'"
			),
			'noscript_tag'   => array(
				array(
					'tag'   => 'img',
					'value' => 'googletagservices.com/tag/js/gpt'
				),
				array(
					'tag'   => 'img',
					'value' => 'googleadservices.com/pagead/conversion'
				),
				array(
					'tag'   => 'img',
					'value' => 'googletagmanager.com/gtm'
				),
				array(
					'tag'   => 'iframe',
					'value' => 'googletagmanager.com'
				)
			),
			'cc'             => 'analytical',
			'html_elem'      => array(
				array(
					'name' => 'img',
					'attr' => 'src:pubads.g.doubleclick.net/gampad'
				),
				array(
					'name' => 'img',
					'attr' => 'src:googleads.g.doubleclick.net/pagead'
				)
			),
		);

		$thirdPartyScript[ 'google_adsense_new' ] = array(
			'label'     => esc_html__( 'Google Adsense', 'consent-magic' ),
			'js'        => 'pagead2.googlesyndication.com/pagead/js/adsbygoogle.js',
			'js_needle' => array( 'adsbygoogle.js' ),
			'cc'        => 'analytical',
		);

		$thirdPartyScript[ 'google_maps' ] = array(
			'label'     => esc_html__( 'Google maps', 'consent-magic' ),
			'js'        => array(
				'maps.googleapis.com/maps/api',
				'maps.googleapis.com/maps-api',
				'lab_google_map/maps'
			),
			'js_needle' => array(
				'maps.googleapis.com/maps/api',
				'maps.googleapis.com/maps-api',
				'google.map',
				'initMap'
			),
			'cc'        => 'other',
			'html_elem' => array(
				array(
					'name' => 'iframe',
					'attr' => 'src:www.google.com/maps/embed'
				),
				array(
					'name' => 'iframe',
					'attr' => 'src:maps.google.com/maps'
				)
			)
		);

		$thirdPartyScript[ 'googlefonts' ] = array(
			'label'          => esc_html__( 'Google fonts', 'consent-magic' ),
			'js'             => 'fonts.googleapis.com',
			'js_needle'      => array( 'fonts.googleapis.com' ),
			'js_not_contain' => array(
				'stripe-credit-card',
				'window.SR7'
			),
			'html_elem'      => array(
				array(
					'name' => 'link',
					'attr' => 'href:https://fonts.googleapis.com'
				)
			),
			'cc'             => 'other',
		);

		$thirdPartyScript[ 'google_captcha' ] = array(
			'label' => esc_html__( 'Google captcha', 'consent-magic' ),
			'js'    => array(
				'gstatic.com/recaptcha',
				'google.com/recaptcha',
			),
			'cc'    => 'other',
		);

		$thirdPartyScript[ 'google_captcha_cf7' ] = array(
			'label' => esc_html__( 'Google captcha Contact Form 7', 'consent-magic' ),
			'js'    => array(
				$wp_content . '/contact-form-7/modules/recaptcha/index.js',
			),
			'cc'    => 'other',
		);

		$thirdPartyScript[ 'facebook_pixel' ] = array(
			'label'        => esc_html__( 'Facebook Pixel Code', 'consent-magic' ),
			's'            => array(
				'connect.facebook.net/en_US/fbevents.js',
				'connect.facebook.net/signals/config/',
				'connect.facebook.net/signals/plugins/',
				'fjs',
				'facebook-jssdk'
			),
			'js'           => array(
				'connect.facebook.net/en_US/fbevents.js',
				'connect.facebook.net/signals/config/',
				'fjs',
				'facebook-jssdk'
			),
			'js_needle'    => array(
				'connect.facebook.net/en_US/fbevents.js',
				'connect.facebook.net/signals/config/',
				'connect.facebook.net/signals/plugins/',
				'fbq',
				'fjs',
				'facebook-jssdk',
			),
			'noscript_tag' => array(
				array(
					'tag'   => 'img',
					'value' => 'facebook.com/tr'
				),
				array(
					'tag'   => 'img',
					'value' => 'connect.facebook.net/en_US/fbevents'
				),
				array(
					'tag'   => 'img',
					'value' => 'connect.facebook.net/signals/config'
				),
				array(
					'tag'   => 'img',
					'value' => 'connect.facebook.net/signals/plugins'
				)
			),
			'cc'           => 'social-media',
			'html_elem'    => array(
				array(
					'name' => 'img',
					'attr' => 'src:facebook.com/tr'
				)
			)
		);

		$thirdPartyScript[ 'facebook-feed' ] = array(
			'label'     => esc_html__( 'Smash Balloon Custom Facebook Feed', 'consent-magic' ),
			'js'        => array(
				$wp_content . '/custom-facebook-feed/js/cff-scripts.js',
				$wp_content . '/custom-facebook-feed/js/cff-scripts.min.js',
				$wp_content . '/custom-facebook-feed-pro/js/cff-scripts.js',
				$wp_content . '/custom-facebook-feed-pro/js/cff-scripts.min.js',
			),
			'js_needle' => array( 'cff_init' ),
			'cc'        => 'analytical',
		);

		$thirdPartyScript[ 'tiktok_pixel' ] = array(
			'label'        => esc_html__( 'TikTok Pixel', 'consent-magic' ),
			's'            => array(
				'analytics.tiktok.com/i18n/pixel/',
			),
			'js'           => array(
				'analytics.tiktok.com/i18n/pixel/',
			),
			'js_needle'    => array(
				'analytics.tiktok.com/i18n/pixel/',
				'ttq.load',
				'ttq.page',
				'ttq',
				'w.TiktokAnalyticsObject',
				'ttq.methods',
				'ttq.instance',
			),
			'noscript_tag' => array(
				array(
					'tag'   => 'img',
					'value' => 'facebook.com/tr'
				),
				array(
					'tag'   => 'img',
					'value' => 'connect.facebook.net/en_US/fbevents'
				),
				array(
					'tag'   => 'img',
					'value' => 'connect.facebook.net/signals/config'
				),
				array(
					'tag'   => 'img',
					'value' => 'connect.facebook.net/signals/plugins'
				)
			),
			'cc'           => 'social-media',
			'html_elem'    => array(
				array(
					'name' => 'img',
					'attr' => 'src:facebook.com/tr'
				),
			)
		);

		$thirdPartyScript[ 'facebook-for-woocommerce' ] = array(
			'label'     => esc_html__( 'Facebook for Woocommerce', 'consent-magic' ),
			'js_needle' => array(
				'fbq',
				'connect.facebook.net'
			),
			'cc'        => 'analytical',
		);

		$thirdPartyScript[ 'facebook-for-wordpress' ] = array(
			'label'     => esc_html__( 'Facebook for Wordpress', 'consent-magic' ),
			'js'        => 'fbq',
			'js_needle' => array( 'fbq' ),
			'cc'        => 'analytical',
		);

		$thirdPartyScript[ 'twitter' ] = array(
			'label'     => esc_html__( 'Twitter Pixel', 'consent-magic' ),
			'js_needle' => array(
				'static.ads-twitter.com',
			),
			'cc'        => 'analytical',
		);

		$thirdPartyScript[ 'twitter_widget' ] = array(
			'label'     => esc_html__( 'Twitter widget', 'consent-magic' ),
			'js'        => 'platform.twitter.com/widgets.js',
			'js_needle' => array(
				'platform.twitter.com/widgets.js',
				'twitter-wjs',
				'twttr.widgets',
				'twttr.events',
				'twttr.ready',
				'window.twttr',
				'twq'
			),
			'cc'        => 'social-media',
		);

		$thirdPartyScript[ 'twitter-feed' ] = array(
			'label'     => esc_html__( 'Custom Twitter Feeds', 'consent-magic' ),
			'js'        => array(
				$wp_content . '/custom-twitter-feeds/js/ctf-scripts.js',
				$wp_content . '/custom-twitter-feeds/js/ctf-scripts.min.js',
				$wp_content . '/custom-twitter-feeds-pro/js/ctf-scripts.js',
				$wp_content . '/custom-twitter-feeds-pro/js/ctf-scripts.min.js',
			),
			'js_needle' => array( 'ctf_init' ),
			'cc'        => 'social-media',
		);

		$thirdPartyScript[ 'bing_tag' ] = array(
			'label'     => esc_html__( 'Microsoft UET', 'consent-magic' ),
			'js'        => 'bat.bing.com/bat.js',
			'js_needle' => array(
				'bat.bing.com/bat.js',
				'window.uetq=window.uetq',
			),
			'cc'        => 'analytical'
		);

		$thirdPartyScript[ 'linkedin_widget' ] = array(
			'label'        => esc_html__( 'Linkedin widget/Analytics', 'consent-magic' ),
			'js'           => 'platform.linkedin.com/in.js',
			'js_needle'    => array(
				'platform.linkedin.com/in.js',
				'snap.licdn.com/li.lms-analytics/insight.min.js',
				'_linkedin_partner_id'
			),
			'noscript_tag' => array(
				array(
					'tag'   => 'img',
					'value' => 'ads.linkedin.com/collect/'
				)
			),
			'cc'           => 'social-media',
			'html_elem'    => array(
				array(
					'name' => 'img',
					'attr' => 'src:ads.linkedin.com/collect/'
				)
			)
		);

		$thirdPartyScript[ 'pinterest' ] = array(
			'label'        => esc_html__( 'Pinterest widget', 'consent-magic' ),
			'js'           => 'assets.pinterest.com/js/pinit.js',
			'js_needle'    => array(
				'assets.pinterest.com/js/pinit.js',
				'pintrk',
				'window.pintrk',
				's.pinimg.com/ct/core.js'
			),
			'noscript_tag' => array(
				array(
					'tag'   => 'img',
					'value' => 'ct.pinterest.com/v3/'
				)
			),
			'cc'           => 'social-media',
			'html_elem'    => array(
				array(
					'name' => 'img',
					'attr' => 'src:ct.pinterest.com/v3/'
				)
			)
		);

		$thirdPartyScript[ 'youtube_embed' ] = array(
			'label'     => esc_html__( 'Youtube embed', 'consent-magic' ),
			'js'        => 'www.youtube.com/player_api',
			'js_needle' => array(
				'www.youtube.com/player_api',
				'onYouTubePlayerAPIReady',
				'YT.Player',
				'onYouTubeIframeAPIReady',
				'www.youtube.com/iframe_api',
			),
			'cc'        => 'other',
			'html_elem' => array(
				array(
					'name' => 'iframe',
					'attr' => 'src:www.youtube.com/embed'
				),
				array(
					'name' => 'iframe',
					'attr' => 'src:youtu.be'
				),
				array(
					'name' => 'object',
					'attr' => 'data:www.youtube.com/embed'
				),
				array(
					'name' => 'embed',
					'attr' => 'src:www.youtube.com/embed'
				),
				array(
					'name' => 'img',
					'attr' => 'src:www.youtube.com/embed'
				),
				array(
					'name' => 'iframe',
					'attr' => 'src:www.youtube-nocookie.com/embed'
				),
			)
		);

		$thirdPartyScript[ 'youtube-feed' ] = array(
			'label'     => esc_html__( 'Feeds for YouTube', 'consent-magic' ),
			'js'        => array(
				$wp_content . '/feeds-for-youtube/js/sb-youtube.js',
				$wp_content . '/feeds-for-youtube/js/sb-youtube.min.js',
				$wp_content . '/youtube-feed-pro/js/sb-youtube.js',
				$wp_content . '/youtube-feed-pro/js/sb-youtube.min.js',
			),
			'js_needle' => array( 'sby_init' ),
			'cc'        => 'analytical',
		);

		$thirdPartyScript[ 'vimeo_embed' ] = array(
			'label'     => esc_html__( 'Vimeo embed', 'consent-magic' ),
			'js'        => 'player.vimeo.com/api/player.js',
			'js_needle' => array(
				'www.vimeo.com/api/oembed',
				'player.vimeo.com/api/player.js',
				'Vimeo.Player',
				'new Player'
			),
			'cc'        => 'other',
			'html_elem' => array(
				array(
					'name' => 'iframe',
					'attr' => 'src:player.vimeo.com/video'
				),
				array(
					'name' => 'iframe',
					'attr' => 'src:vimeo.com/video'
				)
			)
		);

		$thirdPartyScript[ 'hubspot_analytics' ] = array(
			'label'     => esc_html__( 'Hubspot Analytics', 'consent-magic' ),
			'js'        => 'js.hs-scripts.com',
			'js_needle' => array( 'js.hs-scripts.com' ),
			'cc'        => 'analytical',
		);

		$thirdPartyScript[ 'matomo_analytics' ] = array(
			'label'        => esc_html__( 'Matomo Analytics', 'consent-magic' ),
			'js'           => 'matomo.js',
			'js_needle'    => array(
				'_paq.push',
				'_mtm.push'
			),
			'noscript_tag' => array(
				array(
					'tag'   => 'img',
					'value' => 'matomo.'
				)
			),
			'cc'           => 'analytical',
			'html_elem'    => array(
				array(
					'name' => 'img',
					'attr' => 'src:matomo.'
				)
			)
		);

		$thirdPartyScript[ 'addthis_widget' ] = array(
			'label'     => esc_html__( 'Addthis widget', 'consent-magic' ),
			'js'        => 's7.addthis.com/js',
			'js_needle' => array( 'addthis_widget' ),
			'cc'        => 'social-media',
		);

		$thirdPartyScript[ 'sharethis_widget' ] = array(
			'label'     => esc_html__( 'Sharethis widget', 'consent-magic' ),
			'js'        => 'platform-api.sharethis.com/js/sharethis.js',
			'js_needle' => array( 'sharethis.js' ),
			'cc'        => 'social-media',
		);

		$thirdPartyScript[ 'soundcloud_embed' ] = array(
			'label'     => esc_html__( 'Soundcloud embed', 'consent-magic' ),
			'js'        => 'connect.soundcloud.com',
			'js_needle' => array(
				'SC.initialize',
				'SC.get',
				'SC.connectCallback',
				'SC.connect',
				'SC.put',
				'SC.stream',
				'SC.Recorder',
				'SC.upload',
				'SC.oEmbed',
				'soundcloud.com'
			),
			'cc'        => 'other',
			'html_elem' => array(
				array(
					'name' => 'iframe',
					'attr' => 'src:w.soundcloud.com/player'
				),
				array(
					'name' => 'iframe',
					'attr' => 'src:api.soundcloud.com'
				)
			)
		);

		$thirdPartyScript[ 'slideshare_embed' ] = array(
			'label'     => esc_html__( 'Slideshare embed', 'consent-magic' ),
			'js'        => 'www.slideshare.net/api/oembed',
			'js_needle' => array( 'www.slideshare.net/api/oembed' ),
			'cc'        => 'other',
			'html_elem' => array(
				array(
					'name' => 'iframe',
					'attr' => 'src:www.slideshare.net/slideshow'
				)
			)
		);

		$thirdPartyScript[ 'instagram_embed' ] = array(
			'label'     => esc_html__( 'Instagram embed', 'consent-magic' ),
			'js'        => 'www.instagram.com/embed.js',
			'js_needle' => array(
				'www.instagram.com/embed.js',
				'api.instagram.com/oembed'
			),
			'cc'        => 'social-media',
			'html_elem' => array(
				array(
					'name' => 'iframe',
					'attr' => 'src:www.instagram.com/p'
				)
			)
		);

		$thirdPartyScript[ 'hotjar' ] = array(
			'label'     => esc_html__( 'Hotjar', 'consent-magic' ),
			'js'        => false,
			'js_needle' => array( 'static.hotjar.com/c/hotjar-' ),
			'cc'        => 'analytical'
		);

		$thirdPartyScript[ 'reddit' ] = array(
			'label' => esc_html__( 'Reddit Pixel', 'consent-magic' ),
			'js'    => array(
				'redditstatic.com/ads/pixel.js',
			),
			'cc'    => 'analytical',
		);

		$translatepress_active = is_translatepress_active();

		foreach ( $thirdPartyScript as $key => $data ) {

			if ( !is_string( $key ) ) {
				throw new \Exception(
					sprintf(
						esc_html__(
							"Invalid index found in the thirdparties array. Index should be of type 'string'. Index found: %d.",
							'consent-magic'
						),
						$key
					)
				);
				break;
			}

			$s     = $js = $jsNeedle = $uri = $cb = $htmlElem = $noscriptTag = null;
			$hasJs = $hasJsNeedle = $hasUri = false;

			$defaultCallback      = '_automate' . ucfirst( $key );
			$defaultCallbackExist = function_exists( $defaultCallback );

			$thirdPartyScript[ $key ][ 'has_s' ]            = false;
			$thirdPartyScript[ $key ][ 'has_js' ]           = false;
			$thirdPartyScript[ $key ][ 'has_js_needle' ]    = false;
			$thirdPartyScript[ $key ][ 'has_noscript_tag' ] = false;
			$thirdPartyScript[ $key ][ 'has_uri' ]          = false;
			$thirdPartyScript[ $key ][ 'has_cc' ]           = false;
			$thirdPartyScript[ $key ][ 'has_html_elem' ]    = false;
			$thirdPartyScript[ $key ][ 'internal_cb' ]      = false;

			if ( !isset( $data[ 'label' ] ) ) {
				$label                               = ucfirst( $key );
				$thirdPartyScript[ $key ][ 'label' ] = $label;
			} elseif ( is_string( $data[ 'label' ] ) ) {
				$label                               = sanitize_text_field( $data[ 'label' ] );
				$label                               = $translatepress_active ? $this->translatepress_gettext_tags(
					$label
				) : $label;
				$thirdPartyScript[ $key ][ 'label' ] = $label;
			}

			if ( !isset( $data[ 's' ] ) ) {
				$thirdPartyScript[ $key ][ 's' ] = $s;
			} elseif ( is_string( $data[ 's' ] ) ) {
				$s                                   = sanitize_text_field( $data[ 's' ] );
				$thirdPartyScript[ $key ][ 's' ]     = $s;
				$thirdPartyScript[ $key ][ 'has_s' ] = true;
			} elseif ( is_array( $data[ 's' ] ) ) {
				foreach ( $data[ 's' ] as $k => $v ) {
					if ( is_string( $v ) ) {
						$thirdPartyScript[ $key ][ 's' ][ $k ] = sanitize_text_field( $v );
						$has_s                                 = true;
					} else {
						$thirdPartyScript[ $key ][ 's' ] = $s;
						$has_s                           = false;
						break;
					}
				}
				$thirdPartyScript[ $key ][ 'has_s' ] = $has_s;
			}

			if ( !isset( $data[ 'js' ] ) ) {
				$thirdPartyScript[ $key ][ 'js' ] = $js;
			} elseif ( is_string( $data[ 'js' ] ) ) {
				$js                                   = sanitize_text_field( $data[ 'js' ] );
				$thirdPartyScript[ $key ][ 'js' ]     = $js;
				$thirdPartyScript[ $key ][ 'has_js' ] = true;
			} elseif ( is_array( $data[ 'js' ] ) ) {
				foreach ( $data[ 'js' ] as $k => $v ) {
					if ( is_string( $v ) ) {
						$thirdPartyScript[ $key ][ 'js' ][ $k ] = sanitize_text_field( $v );
						$hasJs                                  = true;
					} else {
						$thirdPartyScript[ $key ][ 'js' ] = $js;
						$hasJs                            = false;
						break;
					}
				}
				$thirdPartyScript[ $key ][ 'has_js' ] = $hasJs;
			}

			if ( !isset( $data[ 'js_needle' ] ) ) {
				$thirdPartyScript[ $key ][ 'js_needle' ] = $jsNeedle;
			} elseif ( is_string( $data[ 'js_needle' ] ) ) {
				$jsNeedle                                    = sanitize_text_field( $data[ 'js_needle' ] );
				$thirdPartyScript[ $key ][ 'js_needle' ]     = $jsNeedle;
				$thirdPartyScript[ $key ][ 'has_js_needle' ] = true;
			} elseif ( is_array( $data[ 'js_needle' ] ) ) {
				foreach ( $data[ 'js_needle' ] as $k => $v ) {
					if ( is_string( $v ) ) {
						$thirdPartyScript[ $key ][ 'js_needle' ][ $k ] = sanitize_text_field( $v );
						$hasJsNeedle                                   = true;
					} else {
						$thirdPartyScript[ $key ][ 'js_needle' ] = $jsNeedle;
						$hasJsNeedle                             = false;
						break;
					}
				}
				$thirdPartyScript[ $key ][ 'has_js_needle' ] = $hasJsNeedle;
			}

			if ( !isset( $data[ 'noscript_tag' ] ) ) {
				$thirdPartyScript[ $key ][ 'noscript_tag' ] = $noscriptTag;
			} elseif ( is_array( $data[ 'noscript_tag' ] ) ) {
				$thirdPartyScript[ $key ][ 'noscript_tag' ] = array();
				foreach ( $data[ 'noscript_tag' ] as $k => $v ) {
					$thirdPartyScript[ $key ][ 'noscript_tag' ][] = array(
						'tag'   => sanitize_text_field( $v[ 'tag' ] ),
						'value' => sanitize_text_field( $v[ 'value' ] ),
					);
					$hasNoscript                                  = true;
				}
				$thirdPartyScript[ $key ][ 'has_noscript_tag' ] = $hasNoscript;
			} else {
				$thirdPartyScript[ $key ][ 'noscript_tag' ] = $noscriptTag;
				$hasNoscript                                = false;
			}

			if ( !isset( $data[ 'uri' ] ) ) {
				$thirdPartyScript[ $key ][ 'uri' ] = $uri;
			} elseif ( is_string( $data[ 'uri' ] ) ) {
				$uri                                   = esc_url_raw( $data[ 'uri' ], array(
					'http',
					'https'
				) );
				$thirdPartyScript[ $key ][ 'uri' ]     = $uri;
				$thirdPartyScript[ $key ][ 'has_uri' ] = true;
			} elseif ( is_array( $data[ 'uri' ] ) ) {
				foreach ( $data[ 'uri' ] as $k => $v ) {
					if ( is_string( $v ) ) {
						$thirdPartyScript[ $key ][ 'uri' ][ $k ] = esc_url_raw( $v, array(
							'http',
							'https'
						) );
						$hasUri                                  = true;
					} else {
						$thirdPartyScript[ $key ][ 'uri' ] = $uri;
						$hasUri                            = false;
						break;
					}
				}
				$thirdPartyScript[ $key ][ 'has_uri' ] = $hasUri;
			}

			if ( isset( $data[ 'callback' ] ) && is_string( $data[ 'callback' ] ) && !empty( $data[ 'callback' ] ) ) {
				$cb = trim( $data[ 'callback' ] );
			} elseif ( isset( $data[ 'callback' ] )
			           && is_array( $data[ 'callback' ] )
			           && 2 === count( $data[ 'callback' ] ) ) {
				$cbMethod                = trim( $data[ 'callback' ][ 1 ] );
				$data[ 'callback' ][ 1 ] = $cbMethod;
				$cb                      = &$data[ 'callback' ];
			} elseif ( !isset( $data[ 'callback' ] ) && $defaultCallbackExist ) {
				$cb = $defaultCallback;
			} else {
				$cb = 'cs_automateDefault';
			}

			if ( !isset( $data[ 'cc' ] ) ) {
				$thirdPartyScript[ $key ][ 'cc' ] = 'other';
			} elseif ( is_string( $data[ 'cc' ] ) ) {
				$thirdPartyScript[ $key ][ 'has_cc' ] = true;
				$cc                                   = sanitize_title( $data[ 'cc' ] );
				if ( !$this->cs_isAllowedCookieCategory( $cc ) ) {
					$thirdPartyScript[ $key ][ 'cc' ] = 'other';
				} else {
					$thirdPartyScript[ $key ][ 'cc' ] = $cc;
				}
			}
			if ( isset( $data[ 'html_elem' ] ) ) {
				//multiple html elements
				if ( is_array( $data[ 'html_elem' ] ) && isset( $data[ 'html_elem' ][ 0 ] ) ) {
					$thirdPartyScript[ $key ][ 'html_elem' ] = array();
					for ( $i = 0; $i < count( $data[ 'html_elem' ] ); $i++ ) {
						$this->processHTMLelm(
							$data[ 'html_elem' ][ $i ],
							$thirdPartyScript[ $key ],
							$i
						); //$data['html_elem'], $thirdPartyScript[$key]
					}
				} else {
					$thirdPartyScript[ $key ][ 'html_elem' ] = array();
					$this->processHTMLelm(
						$data[ 'html_elem' ],
						$thirdPartyScript[ $key ],
						0
					); //$data['html_elem'], $thirdPartyScript[$key]
				}

			} else {
				$thirdPartyScript[ $key ][ 'html_elem' ] = $htmlElem;
			}

			if ( method_exists( $this, $cb ) ) {
				$thirdPartyScript[ $key ][ 'internal_cb' ] = true;
			}
			$thirdPartyScript[ $key ][ 'callback' ] = $cb;
		}

		return $thirdPartyScript;
	}

	public function cs_isAllowedCookieCategory() {

		return array(
			'functional',
			'analytical',
			'social-media',
			'advertising',
			'other'
		);
	}

	public function processHTMLelm( &$data, &$thirdPartyScript, $i ) {
		$thirdPartyScript[ 'html_elem' ][ $i ] = array();
		if ( !isset( $data[ 'name' ] ) ) {
			$thirdPartyScript[ 'html_elem' ][ $i ][ 'name' ] = null;
		} elseif ( isset( $data[ 'name' ] ) && !is_string( $data[ 'name' ] ) ) {
			$thirdPartyScript[ 'html_elem' ][ $i ][ 'name' ] = null;
		} elseif ( !isset( $data[ 'attr' ] ) ) {
			$thirdPartyScript[ 'html_elem' ][ $i ][ 'attr' ] = null;
		} elseif ( isset( $data[ 'attr' ] ) && !is_string( $data[ 'attr' ] ) ) {
			$thirdPartyScript[ 'html_elem' ][ $i ][ 'attr' ] = null;
		} elseif ( isset( $data[ 'attr' ] ) ) {
			$pos = strpos( $data[ 'attr' ], ':' );
			if ( false === $pos || $pos < 1 ) {
				$thirdPartyScript[ 'html_elem' ][ $i ][ 'attr' ] = null;
			}
		}
		if ( null !== $data[ 'name' ] ) {
			$thirdPartyScript[ 'html_elem' ][ $i ][ 'name' ] = sanitize_key( $data[ 'name' ] );
		}
		if ( null !== $data[ 'attr' ] ) {
			$attr                                            = trim( $data[ 'attr' ] );
			$thirdPartyScript[ 'html_elem' ][ $i ][ 'attr' ] = $attr;
			$attrArr                                         = explode( ':', $attr );
			$thirdPartyScript[ 'has_html_elem' ]             = true;
		}
	}

	public function cs_automateDefault( $type = null, $autoData = array(), $parts = array() ) {
		$patterns    = array();
		$hasS        = $autoData[ 'has_s' ];
		$hasJs       = $autoData[ 'has_js' ];
		$hasJsNeedle = $autoData[ 'has_js_needle' ];
		$hasNoscript = $autoData[ 'has_noscript_tag' ];
		$hasUri      = $autoData[ 'has_uri' ];
		$hasHtmlElem = $autoData[ 'has_html_elem' ];

		$regex = $this->cs_defineRegex();

		if ( $hasUri ) {
			$uri = $autoData[ 'uri' ];

			$uriPattTmpl = $regex[ '_regexParts' ][ '-lookbehind_link_img' ] . 'https?://(?:[www\.]{4})?%s';
			foreach ( (array) $uri as $u ) {
				$url        = $this->cs_getUriWithoutSchemaSubdomain( $u );
				$url        = str_replace( '*', $regex[ '_regexParts' ][ 'random_chars' ], $url );
				$escapedUri = $this->cs_escapeRegexChars( $url );
				$patt       = sprintf( $uriPattTmpl, $escapedUri );
				$patterns[] = $patt;
			}
		}

		if ( $hasS ) {
			$s = $autoData[ 's' ];
			foreach ( (array) $s as $term ) {
				$cleanUri   = $this->cs_getCleanUri( $term, true );
				$subdmain   = ( '' !== $cleanUri && '.' === $cleanUri[ 0 ] ) ? '[^.]+?' : '';
				$escapedUri = $this->cs_escapeRegexChars( $cleanUri );
				$patt       = sprintf( $regex[ '_regexPatternIframe' ], $subdmain, $escapedUri );
				$patterns[] = $patt;
			}
		}

		if ( $hasJs ) {
			$js = $autoData[ 'js' ];
			foreach ( (array) $js as $script ) {
				$hasPluginUri     = false;
				$cleanUri         = $this->cs_getCleanUri( $script, true );
				$allowedLocations = array(
					'plugin' => str_replace( ABSPATH, '', WP_PLUGIN_DIR ),
					'theme'  => str_replace( ABSPATH, '', WP_CONTENT_DIR ) . '/themes'
				);

				if ( '' !== $cleanUri
				     && !empty( $allowedLocations )
				     && preg_match( '#^' . join( '|', $allowedLocations ) . '#', $cleanUri ) ) {
					$hasPluginUri = true;
					$uriBegin     = trailingslashit( $this->cs_getCleanUri( home_url( add_query_arg( null, null ) ) ) );

				} elseif ( '' !== $cleanUri && '.' === $cleanUri[ 0 ] ) {
					$uriBegin = '[^.]+?';
				} else {
					$uriBegin = '';
				}

				$escapedUri = $this->cs_escapeRegexChars( $cleanUri );
				if ( $hasPluginUri ) {
					$uriBegin = $this->cs_escapeRegexChars( $uriBegin );
				}

				$patt       = sprintf( $regex[ '_regexPatternScriptSrc' ], $uriBegin, $escapedUri );
				$patterns[] = $patt;
			}
		}

		if ( $hasJsNeedle ) {
			$jsNeedle = $autoData[ 'js_needle' ];
			foreach ( (array) $jsNeedle as $needle ) {
				$escaped    = $this->cs_escapeRegexChars( $needle );
				$patt       = sprintf( $regex[ '_regexPatternScriptHasNeedle' ], $escaped );
				$patterns[] = $patt;
			}
		}

		if ( $hasNoscript ) {
			$noscript = $autoData[ 'noscript_tag' ];
			foreach ( (array) $noscript as $needle ) {
				$escaped    = $this->cs_escapeRegexChars( $needle[ 'value' ] );
				$patt       = sprintf(
					$regex[ '_regexPatternNoScriptHasNeedle' ],
					$needle[ 'tag' ],
					$escaped,
					$needle[ 'tag' ]
				);
				$patterns[] = $patt;
			}
		}

		if ( $hasHtmlElem ) {
			for ( $j = 0; $j < count( $autoData[ 'html_elem' ] ); $j++ ) {
				$htmlElemAttr      = explode( ':', $autoData[ 'html_elem' ][ $j ][ 'attr' ] );
				$htmlElemName      = $this->cs_escapeRegexChars( $autoData[ 'html_elem' ][ $j ][ 'name' ] );
				$htmlElemAttrName  = $this->cs_escapeRegexChars( $htmlElemAttr[ 0 ] );
				$htmlElemAttrValue = $this->cs_escapeRegexChars( $htmlElemAttr[ 1 ] );
				$prefix            = '';
				if ( ( $htmlElemAttrName == 'src' ) || ( $htmlElemAttrName == 'data' && $htmlElemName == 'object' ) ) {
					$prefix = $regex[ '_regexParts' ][ 'src_scheme_www' ];
				}
				if ( ( $htmlElemName == 'img' ) || ( $htmlElemName == 'link' ) || ( $htmlElemName == 'embed' ) ) {
					$patterns[] = sprintf(
						$regex[ '_regexPatternHtmlElemWithAttrTypeA' ],
						$htmlElemName,
						$htmlElemAttrName,
						$prefix,
						$htmlElemAttrValue
					);
				} else {
					$patterns[] = sprintf(
						$regex[ '_regexPatternHtmlElemWithAttr' ],
						$htmlElemName,
						$htmlElemAttrName,
						$prefix,
						$htmlElemAttrValue,
						$htmlElemName,
						$htmlElemName
					);
				}
			}
		}

		return $this->cs_prepare_script( $patterns, '', $type, $parts, $autoData );
	}

	public function cs_getUriWithoutSchemaSubdomain( $uri = '', $subdomain = 'www' ) {
		$uri = preg_replace( "#(https?://|//|$subdomain\.)#", '', $uri );

		return ( null === $uri ) ? '' : $uri;
	}

	public function cs_prepare_script( $patterns = '', $modifiers = '', $type = null, $parts = array(), $autoData = array() ) {

		$prefix         = '(?:\<!--\s+\[cs_skip]\s+--\>_NL_)?';
		$wrapperPattern = '#' . $prefix . '%s#' . $modifiers;
		$pattern        = array();

		foreach ( $patterns as $pttrn ) {
			$pattern[] = sprintf( $wrapperPattern, $pttrn );
		}

		if ( !isset( $parts[ 'head' ] ) || !isset( $parts[ 'body' ] ) ) {
			throw new \InvalidArgumentException(
				sprintf(
					esc_html__( 'Parts array is not valid for %s: head or body entry not found.', 'consent-magic' ),
					$type
				)
			);
		}

		if ( !empty( $pattern ) ) {
			foreach ( $pattern as $patt ) {
				$parts[ 'head' ] = $this->script_replace_callback( $parts[ 'head' ], $patt, $autoData, 'head' );
			}
		}

		if ( null === $parts[ 'head' ] ) {
			throw new \RuntimeException(
				sprintf(
					esc_html__( 'An error occured calling %s context head.', 'consent-magic' ),
					'preg_replace_callback()'
				)
			);
		}

		$prefix         = '((?:\<!--\s+\[cli_skip]\s+--\>_NL_)?';
		$suffix         = ')';
		$wrapperPattern = '#' . $prefix . '%s' . $suffix . '#' . $modifiers;
		$pattern        = array();
		foreach ( $patterns as $pttrn ) {
			$pattern[] = sprintf( $wrapperPattern, $pttrn );
		}

		if ( !empty( $pattern ) ) {
			foreach ( $pattern as $patt ) {
				$parts[ 'body' ] = $this->script_replace_callback( $parts[ 'body' ], $patt, $autoData, 'body' );
			}
		}

		if ( null === $parts[ 'body' ] ) {
			throw new \RuntimeException(
				sprintf(
					esc_html__( 'An error occured calling %s context body.', 'consent-magic' ),
					'preg_replace_callback()'
				)
			);
		}

		return $parts;
	}

	public function script_replace_callback( $html, $pattern, $autoData, $elm_position = 'head' ) {

		$callback = preg_replace_callback( $pattern, function( $matches ) use ( $autoData, $elm_position ) {

			if ( 1 === (int) $autoData[ 'check' ] ) {
				$script_block_on_start = 'true';
				$script_type           = "text/plain";
			} else {
				$script_block_on_start = 'false';
				$script_type           = "text/javascript";
			}

			$script_label     = $autoData[ 'label' ] ?? '';
			$script_slug      = $autoData[ 'slug' ] ?? '';
			$script_cat_slug  = $autoData[ 'category' ] ?? '';
			$placeholder_text = $autoData[ 'placeholder' ] ?? '';
			$match            = $matches[ 0 ];

			$cs_replace = ' data-cs-class="cs-blocker-script" data-cs-slug="' . $script_slug . '" data-cs-label="'
			              . $script_label . '"  data-cs-script-type="' . $script_cat_slug . '" data-cs-block="'
			              . $script_block_on_start . '" data-cs-element-position="' . $elm_position . '"';

			if ( strpos( $match, 'data-cs-class' ) === false ) {
				if ( isset( $autoData[ 'js_not_contain' ] ) ) {
					foreach ( $autoData[ 'js_not_contain' ] as $item ) {
						if ( strpos( $match, $item ) !== false ) {
							return $match;
						}
					}
				}
				if ( preg_match_all( '/<link.*fonts\.googleapis\.com\/css.*?[\/]?>/', $match, $element_match2 ) ) {
					$element_href = $element_match2[ 0 ];

					if ( 1 === (int) $autoData[ 'check' ] ) {
						$element_modded_href = preg_replace(
							'/(href=)/',
							$cs_replace . ' data-cs-placeholder="' . htmlentities( wp_kses_post( $placeholder_text ) )
							. '" data-cs-href=',
							$element_href,
							1
						);
					} else {
						if ( preg_match( '/<link.*(href\s*=\s*(?:"|\')(.*)(?:"|\')).*>/i', $match, $element_match2 ) ) {
							$element_href        = $element_match2[ 1 ];
							$element_modded_href = preg_replace(
								'/(href=)/',
								$element_href . $cs_replace . ' data-cs-placeholder="' . htmlentities(
									wp_kses_post( $placeholder_text )
								) . '" data-cs-href=',
								$element_href,
								1
							);
						} else {
							$element_modded_href = $element_href;
						}
					}

					$match = str_replace( $element_href, $element_modded_href, $match );

				} else if ( preg_match(
					'/(<noscript[^\>]*)\>\s*(\<[^>]*(?:\/?>|\>[\w|\s]*\<\/\w*\>))[\w|\s]*<\/noscript>/i',
					$match,
					$noscript_match
				) ) {
					$replace = $noscript_match[ 1 ] . ' ' . $cs_replace . ' data-cs-noscript="true" data-cs-content="'
					           . htmlentities( $noscript_match[ 2 ] ) . '"';
					$match   = str_replace( $noscript_match[ 1 ], $replace, $match );

					if ( 1 === (int) $autoData[ 'check' ] ) {
						$match = str_replace( $noscript_match[ 2 ], '', $match );
					}

				} else if ( ( preg_match(
						'/<iframe.*(src\s*=\s*(?:"|\')(.*)(?:"|\')).*>.*<\/iframe>/i',
						$match,
						$element_match
					) )
				            || ( preg_match( '/<object.*(src=\"(.*)\").*>.*<\/object >/', $match, $element_match ) )
				            || ( preg_match( '/<embed.*(src=\"(.*)\").*>/', $match, $element_match ) )
				            || ( preg_match( '/<img[^\<\>]*(src=\"(.*)\")[^\<\>]*>/', $match, $element_match ) ) ) {

					//Show the video but block personal data tracking when required by the rule and user consent
					if ( (int) $this->cs_block_video_personal_data === 1 ) {

						$element_src = $element_match[ 1 ];

						//add privacy part when block embedded videos
						if ( preg_match( '/www\.(youtube\.com)\/embed/i', $element_src, $src_match )
						     || preg_match( '/www\.(youtube\-nocookie\.com)\/embed/i', $element_src, $src_match ) ) {

							//youtube
							$src             = $src_match[ 1 ];
							$element_src_new = str_replace( $src, 'youtube-nocookie.com', $element_src );

							if ( 1 === (int) $autoData[ 'check' ] ) {

								$element_modded_src = preg_replace(
									'/(src=)/',
									' data-cs-class="cs-blocker-script" data-cs-slug="' . $script_slug
									. '" data-cs-label="' . $script_label . '"  data-cs-script-type="'
									. $script_cat_slug . '" data-cs-block="' . $script_block_on_start
									. '" data-cs-element-position="' . $elm_position
									. '" data-additional-cookie=true" data-additional-video="youtube" src=',
									$element_src_new,
									1
								);
							} else {
								$element_modded_src = preg_replace(
									'/(src=)/',
									' data-cs-class="cs-blocker-script" data-cs-slug="' . $script_slug
									. '" data-cs-label="' . $script_label . '"  data-cs-script-type="'
									. $script_cat_slug . '" data-cs-block="' . $script_block_on_start
									. '" data-cs-element-position="' . $elm_position
									. '" data-additional-cookie=false" data-additional-video="youtube" src=',
									$element_src,
									1
								);
							}

						} elseif ( preg_match( '/.*(?:player\.)?vimeo\.com.*/', $element_src ) ) {

							//vimeo
							if ( preg_match(
								'/.*(?:player\.)?vimeo\.com.*(((?:\&\#038\;|\&amp\;|\&|\?)).*(dnt\=(?:0|1|true|false))((?:\&\#038\;|\&amp\;|\&)?)(.*?)(?:\"|\'))/i',
								$element_src,
								$src_match
							) ) {

								if ( 1 === (int) $autoData[ 'check' ] ) {
									$new_dnt = str_replace( $src_match[ 3 ], 'dnt=1', $element_src );
								} else {
									if ( !empty( $src_match[ 4 ] ) ) {
										$new_dnt = str_replace( $src_match[ 3 ] . $src_match[ 4 ], '', $element_src );
									} else {
										$new_dnt = str_replace( $src_match[ 3 ], '', $element_src );
									}
								}

							} elseif ( preg_match(
								'/.*(?:player\.)?vimeo\.com.*?((?:\&\#038\;|\&amp\;|\&)?)((?:\"|\'))/i',
								$element_src,
								$src_match
							) ) {

								if ( 1 === (int) $autoData[ 'check' ] ) {

									if ( empty( $src_match[ 1 ] ) ) {
										$new_dnt = substr_replace( $src_match[ 0 ], '&dnt=1', -1, 0 );

									} else {
										$new_dnt = substr_replace( $src_match[ 0 ], 'dnt=1', -1, 0 );
									}

									$new_dnt = str_replace( $src_match[ 0 ], $new_dnt, $element_src );
								} else {
									$new_dnt = $element_src;
								}
							} else {
								$new_dnt = $element_src;
							}

							$element_modded_src = preg_replace(
								'/(src=)/',
								' data-cs-class="cs-blocker-script" data-cs-slug="' . $script_slug . '" data-cs-label="'
								. $script_label . '"  data-cs-script-type="' . $script_cat_slug . '" data-cs-block="'
								. $script_block_on_start . '" data-cs-element-position="' . $elm_position
								. '" data-additional-cookie=false" data-additional-video="vimeo" src=',
								$new_dnt,
								1
							);

						} else {
							if ( 1 === (int) $autoData[ 'check' ] ) {
								$element_modded_src = preg_replace(
									'/(src=)/',
									$cs_replace . ' data-cs-placeholder="' . htmlentities(
										wp_kses_post( $placeholder_text )
									) . '" data-cs-src=',
									$element_src,
									1
								);
							} else {
								$element_modded_src = preg_replace(
									'/(src=)/',
									$element_src . $cs_replace . ' data-cs-placeholder="' . htmlentities(
										wp_kses_post( $placeholder_text )
									) . '" data-cs-src=',
									$element_src,
									1
								);
							}
						}

						$match = str_replace( $element_src, $element_modded_src, $match );

					} else {
						$element_src = $element_match[ 1 ];

						if ( 1 === (int) $autoData[ 'check' ] ) {
							$element_modded_src = preg_replace(
								'/(src=)/',
								$cs_replace . ' data-cs-placeholder="' . htmlentities(
									wp_kses_post( $placeholder_text )
								) . '" data-cs-src=',
								$element_src,
								1
							);
						} else {
							$element_modded_src = preg_replace(
								'/(src=)/',
								$element_src . $cs_replace . ' data-cs-placeholder="' . htmlentities(
									wp_kses_post( $placeholder_text )
								) . '" data-cs-src=',
								$element_src,
								1
							);
						}

						$match = str_replace( $element_src, $element_modded_src, $match );
					}
				} else {

					//add advanced blocking for Google consent mode v2
					if ( CS_Google_Consent_Mode()->enabled_google_consent_mode()
					     && ( preg_match( '/<script.*>(?:.|\n)*function\s*gtag\(\s*\)(?:.|\n)*<\/script>/', $match )
					          || preg_match( '/<script.*googletagmanager.com\/gtag\/js\?id=.*<\/script>/', $match )
					          || preg_match(
						          '/<script.*?>.*?googletagmanager\.com\/gtm\.js\?id=.*?<\/script>/',
						          $match
					          )
					          || preg_match( '/<script(.*?)>(.*?gtmkit_dataLayer_content.*?)<\/script>/', $match ) ) ) {
						$cs_replace = ' data-cs-class="cs-blocker-script-advanced" data-cs-slug="' . $script_slug
						              . '" data-cs-label="' . $script_label . '"' . ' data-cs-script-type="'
						              . $script_cat_slug . '" data-cs-element-position="' . $elm_position . '"';
						$match      = str_replace(
							'<script',
							'<script type="text/javascript"' . ' ' . $cs_replace,
							$match
						);

					} else {
						if ( preg_match( '/<script[^\>](type=(?:"|\')(.*?)(?:"|\')).*?>/', $match )
						     && preg_match(
							     '/<script[^\>](type=(?:"|\')text\/javascript(.*?)(?:"|\')).*?>/',
							     $match
						     ) ) {

							preg_match(
								'/<script[^\>](type=(?:"|\')text\/javascript(.*?)(?:"|\')).*?>/',
								$match,
								$output_array
							);

							$re = preg_quote( $output_array[ 1 ], '/' );

							if ( !empty( $output_array ) ) {
								$match = preg_replace(
									'/' . $re . '/',
									'type="' . $script_type . '"' . ' ' . $cs_replace,
									$match,
									1
								);
							}
						} else {
							$match = str_replace(
								'<script',
								'<script type="' . $script_type . '"' . ' ' . $cs_replace,
								$match
							);
						}
					}
				}
			}

			return $match;
		}, $html );

		return $callback;
	}

	public function cs_escapeRegexChars( $str = '' ) {

		$chars = array(
			'^',
			'$',
			'(',
			')',
			'<',
			'>',
			'.',
			'*',
			'+',
			'?',
			'[',
			'{',
			'\\',
			'|'
		);

		foreach ( $chars as $k => $char ) {
			$chars[ $k ] = '\\' . $char;
		}

		$replaced = preg_replace( '#(' . join( '|', $chars ) . ')#', '\\\${1}', $str );

		return ( null !== $replaced ) ? $replaced : $str;
	}

	public function cs_getCleanUri( $uri = '', $stripSubDomain = false, $subdomain = 'www' ) {

		if ( !is_string( $uri ) ) {
			return '';
		}

		$regexSubdomain = '';
		if ( $stripSubDomain && is_string( $subdomain ) ) {
			$subdomain = trim( $subdomain );
			if ( '' !== $subdomain ) {
				$regexSubdomain = $this->cs_escapeRegexChars( "$subdomain." );
			}
		}

		$regex = '^' . 'https?://' . $regexSubdomain . '([^/?]+)' . '(.*)' . '$';

		$uri = preg_replace( "#$regex#", '${1}', $uri );

		return ( null === $uri ) ? '' : $uri;
	}

	//add advanced blocking for Google analytics and Google ads
	public function check_additional_scripts( $html ) {

		$wp_content = preg_quote( str_replace( ABSPATH, '', WP_PLUGIN_DIR ), '/' );

		$gtm_src_patterns = array(
			'/googletagmanager\.com\/gtag\/js\?id\=/',
			"/$wp_content\/gtm-kit\/assets\/integration\/.*?\.js/",
			'/google-site-kit/'
		);

		$gtm_patterns = array(
			'/googletagmanager\.com\/gtag\/js\?id\=/',
			"/$wp_content\/gtm-kit\/assets\/integration\/.*?\.js/",
			'/gtmkit_dataLayer_content/',
			'/googletagmanager\.com\/gtm\.js\?id=/',
			'/function\s*gtag/',
			'/__gtagTracker/',
			'/MonsterInsightsDualTracker/',
			'/googletagmanager\.com\/gtm\.js\?id=/',
		);

		$consent_mode_patterns = array(
			'/gtag\s*\(\s*[\'"]consent[\'"]\s*,\s*[\'"][^\'"]*[\'"]\s*,\s*\{.*?\}\s*\)/is',
			'/gtag\s*\(\s*[\'"]consent[\'"]\s*,\s*[\'"][^\'"]*[\'"]\s*,\s*\{[^}]*?mode[^}]*?\}\s*\)/is',
			'/__gtagTracker\s*\(\s*[\'"]consent[\'"]\s*,\s*[\'"][^\'"]*[\'"]\s*,\s*\{.*?\}\s*\)/is'
		);

		foreach ( $html as $k => &$part ) {

			$doc_cmp = new \DOMDocument();
			libxml_use_internal_errors( true );
			$doc_cmp->loadHTML( $part, LIBXML_HTML_NOIMPLIED | LIBXML_HTML_NODEFDTD );
			libxml_clear_errors();

			$xpath   = new \DOMXPath( $doc_cmp );
			$scripts = $xpath->query( '//script' );

			if ( !empty( $scripts ) ) {
				foreach ( $scripts as $script ) {
					$original_script_html = $doc_cmp->saveHTML( $script );       // <script>...</script>
					$script_content       = $script->nodeValue;

					$modified_script_content = preg_replace( $consent_mode_patterns, '', $script_content );

					if ( $script_content !== $modified_script_content ) {
						$script->nodeValue    = $modified_script_content;
						$modified_script_html = $doc_cmp->saveHTML( $script );
						$part                 = str_replace( $original_script_html, $modified_script_html, $part );
					}
				}
			}

			$doc = new \DOMDocument();
			libxml_use_internal_errors( true );
			$doc->loadHTML( $part, LIBXML_HTML_NOIMPLIED | LIBXML_HTML_NODEFDTD );
			libxml_clear_errors();

			$xpath   = new \DOMXPath( $doc );
			$scripts = $xpath->query( '//script' );

			if ( !empty( $scripts ) ) {
				foreach ( $scripts as $script ) {
					$original_script = $doc->saveHTML( $script );
					$script_content  = $script->nodeValue;

					if ( !$script->hasAttribute( 'data-cs-class' ) ) {

						preg_replace_callback( $gtm_patterns, function( $matches ) use ( $k, $script ) {

							$match = $matches[ 0 ];
							$script->setAttribute( 'type', 'text/plain' );
							$script->setAttribute( 'data-cs-class', 'cs-blocker-script-advanced' );
							$script->setAttribute( 'class', 'cs-blocker-script-advanced' );
							$script->setAttribute( 'data-cs-element-position', $k );


							return $match;
						}, $script_content );

						$src = $script->getAttribute( 'src' );
						preg_replace_callback( $gtm_src_patterns, function( $matches ) use ( $k, $script ) {

							$match = $matches[ 0 ];
							$script->setAttribute( 'type', 'text/plain' );
							$script->setAttribute( 'data-cs-class', 'cs-blocker-script-advanced' );
							$script->setAttribute( 'class', 'cs-blocker-script-advanced' );
							$script->setAttribute( 'data-cs-element-position', $k );

							return $match;
						}, $src );
					}

					$modified_script = $doc->saveHTML( $script );
					$part            = str_replace( $original_script, $modified_script, $part );
				}
			}

			if ( preg_match_all(
				'/<script([^>]*?)>[^<]*?src\=.*?googletagmanager.com\/gtm\.js\?id\=.*?\(\s*window\s*\,\s*document\s*\,\s*(?:"|\')\s*script\s*(?:"|\')\s*\,\s*(?:"|\')\s*(\w*)\s*(?:"|\')\s*\,.*?\).*?<\/script>/',
				$part,
				$element_match_scripts,
				PREG_SET_ORDER
			) ) {
				foreach ( $element_match_scripts as $element_match_script ) {
					if ( !in_array( $element_match_script[ 2 ], $this->front_scripts[ 'data_layers' ] ) ) {
						$this->front_scripts[ 'data_layers' ][] = $element_match_script[ 2 ];
					}
				}
			}
		}

		if ( isPYSActivated() && class_exists( '\PixelYourSite\GATags' ) ) {
			$dataLayerName = 'dataLayerPYS';

			switch ( \PixelYourSite\GATags()->getOption( 'gtag_datalayer_type' ) ) {
				case 'disable':
					$dataLayerName = 'dataLayer';
					break;
				case 'custom':
					$dataLayerName = \PixelYourSite\GATags()->getOption( 'gtag_datalayer_name' );
					break;
				case 'default':
					$dataLayerName = 'dataLayerPYS';
					break;
			}
			$this->front_scripts[ 'data_layers' ][] = $dataLayerName;
		}
		$this->front_scripts[ 'data_layers' ][] = 'dataLayer';

		$this->front_scripts[ 'data_layers' ] = array_unique( $this->front_scripts[ 'data_layers' ] );
		unset( $part );

		$last_body_close_tag = strrpos( $html[ 'body' ], '</body>' );

		if ( $last_body_close_tag !== false ) {
			$html[ 'body' ] = substr_replace(
				$html[ 'body' ],
				'<input type="hidden" class="cs-blocker-script-advanced-data" value="' . htmlspecialchars(
					json_encode( $this->front_scripts ),
					ENT_QUOTES,
					'UTF-8'
				) . '">' . '</body>',
				$last_body_close_tag,
				strlen( '</body>' )
			);
		}

		return $html;
	}

	private function translatepress_gettext_tags( $string ) {
		if ( is_string( $string ) && strpos( $string, 'data-trpgettextoriginal=' ) !== false ) {
			$string = preg_replace( '/ data-trpgettextoriginal=\d+#!trpen#/i', '', $string );
			$string = preg_replace(
				'/data-trpgettextoriginal=\d+#!trpen#/i',
				'',
				$string
			);//sometimes it can be without space
			$string = str_ireplace( '#!trpst#trp-gettext', '', $string );
			$string = str_ireplace( '#!trpst#/trp-gettext', '', $string );
			$string = str_ireplace( '#!trpst#\/trp-gettext', '', $string );
			$string = str_ireplace( '#!trpen#', '', $string );
		}

		return $string;
	}

	public function render_video_placeholder() {
		$cs_video_text = array(
			'cs_video_consent_general_text' => '',
			'cs_video_consent_rule_text'    => '',
		);

		$embedded_video_id        = ConsentMagic()->getOption( 'embedded_video_cat_id' );
		$embedded_video_id        = ( $embedded_video_id !== false ) ? $embedded_video_id : update_cat_id_options(
			'embedded_video_cat_id'
		);
		$current_lang             = get_locale();
		$cs_language_availability = ConsentMagic()->getOption( 'cs_language_availability' );
		$cs_user_default_language = ConsentMagic()->getOption( 'cs_user_default_language' );
		if ( isset( $cs_language_availability[ $current_lang ] ) && $cs_language_availability[ $current_lang ] == 0 ) {
			$current_lang = $cs_user_default_language;
		}

		foreach ( $cs_video_text as $key => $text ) {

			$attr_value_old = ConsentMagic()->getOption( $key );

			if ( !empty( $attr_value_old ) ) {
				ConsentMagic()->updateLangOptions( $key, $attr_value_old, CMPRO_DEFAULT_LANGUAGE );
				ConsentMagic()->deleteOption( $key );

				$cs_video_text[ $key ]         = $attr_value_old;
				$embedded_video_name           = array();
				$embedded_video_description    = ConsentMagic()->getLangOptionTerm(
					'embedded_video',
					CMPRO_DEFAULT_LANGUAGE
				);
				$embedded_video_name[ 'name' ] = ( is_array(
					$embedded_video_description
				) ) ? $embedded_video_description[ 'name' ] : 'Embedded video';
			} else {
				if ( ConsentMagic()->getOption( 'cs_enable_translations' ) == 1 ) {
					$cs_video_text[ $key ] = ConsentMagic()->getLangOption( $key, $current_lang );
					$embedded_video_name   = ConsentMagic()->getLangOptionTerm( 'embedded_video', $current_lang );

					if ( empty( $cs_video_text[ $key ] ) ) {
						$cs_video_text[ $key ] = ConsentMagic()->getLangOption( $key, $cs_user_default_language );
						$embedded_video_name   = ConsentMagic()->getLangOptionTerm(
							'embedded_video',
							$cs_user_default_language
						);
						if ( empty( $cs_video_text[ $key ] ) ) {
							$cs_video_text[ $key ] = ConsentMagic()->getLangOption( $key, CMPRO_DEFAULT_LANGUAGE );
							$embedded_video_name   = ConsentMagic()->getLangOptionTerm(
								'embedded_video',
								CMPRO_DEFAULT_LANGUAGE
							);
						}
					}
				} else {
					$cs_video_text[ $key ] = ConsentMagic()->getLangOption( $key, CMPRO_DEFAULT_LANGUAGE );
					$embedded_video_name   = ConsentMagic()->getLangOptionTerm(
						'embedded_video',
						CMPRO_DEFAULT_LANGUAGE
					);
				}
			}
		}
		$embedded_video_name_text = ( $embedded_video_name
		                              && isset( $embedded_video_name[ 'category_name' ] ) ) ? $embedded_video_name[ 'category_name' ] : 'Embedded Videos';
		$placeholder_text         = sprintf(
			"<div><p>%s</p></div><div><a class='cs_manage_current_consent' data-cs-script-type='%s' data-cs-manage='manage_placeholder'>%s %s</a></div>",
			$cs_video_text[ 'cs_video_consent_general_text' ],
			$embedded_video_id,
			$cs_video_text[ 'cs_video_consent_rule_text' ],
			$embedded_video_name_text
		);

		add_filter( 'cm_video_placeholder', fn () => $placeholder_text );
	}
}

/**
 * @return CS_Script_Blocker
 */
function CS_Script_Blocker() {
	return CS_Script_Blocker::instance();
}

CS_Script_Blocker();