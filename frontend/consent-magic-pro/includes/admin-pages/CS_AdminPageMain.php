<?php

namespace ConsentMagicPro;

if ( !defined( 'ABSPATH' ) ) {
	exit; // Exit if accessed directly.
}

class CS_AdminPageMain extends CS_AdminPage implements CS_Page {

	/**
	 * Admin page main
	 * @return void
	 */
	public function renderPage() {

		$this->dbCronHandle();

		$this->manageCSPermissions();

		// Get options:
		$design_options = ConsentMagic()->getCSOptionsDesign();

		if ( isset( $_POST[ '_wpnonce' ] ) && wp_verify_nonce( $_POST[ '_wpnonce' ], 'cs-update-' . CMPRO_SETTINGS_FIELD ) ) {

			// Check if form has been set:
			if ( isset( $_POST[ 'cs_update_action' ] ) && $_POST[ 'cs_update_action' ] == 'update_admin_settings_form' ) {

				$this->updateCronHandle();

				if ( isset( $_POST[ 'cs' ][ $this->plugin_name ][ 'cs_check_flow' ] ) ) {
					if ( isset( $_POST[ 'cs' ][ $this->plugin_name ][ 'cs_active_rule_id_first' ] ) ) {
						$this->change_rule_order_admin( (int) sanitize_text_field( $_POST[ 'cs' ][ $this->plugin_name ][ 'cs_active_rule_id_first' ] ) );
					}
					$user_id = get_current_user_id();
					add_user_meta( $user_id, 'cs_notice_dismissed', 'true', true );

					if ( isset( $_POST[ 'cs' ][ $this->plugin_name ][ 'cs_block_googlefonts_enabled' ] ) ) {
						$term = get_term_by( 'slug', 'googlefonts', 'cs-cookies-category' );
						if ( $term ) {
							$val = !(bool) sanitize_text_field( $_POST[ 'cs' ][ $this->plugin_name ][ 'cs_block_googlefonts_enabled' ] );
							update_term_meta( $term->term_id, 'cs_ignore_this_category', $val );
						}
					}

					if ( isset( $_POST[ 'cs' ][ $this->plugin_name ][ 'cs_policy_existing_page' ] ) ) {
						ConsentMagic()->updateOptions( array( 'cs_policy_existing_page' => array( CMPRO_DEFAULT_LANGUAGE => sanitize_text_field( $_POST[ 'cs' ][ $this->plugin_name ][ 'cs_policy_existing_page' ] ) ) ) );
					}
				}

				//update CM options
				$this->updateOptionsHandle();

				foreach ( $design_options as $key => $value ) {
					if ( isset( $_POST[ 'cs_d' ][ $this->plugin_name ][ $key ] ) ) {
						ConsentMagic()->updateOptionsDesign( array( $key => sanitize_text_field( $_POST[ 'cs_d' ][ $this->plugin_name ][ $key ] ) ) );
					}
				}

				if ( isset( $_POST[ 'cs_post_name' ] ) && isset( $_POST[ 'cs_rule_id' ] ) ) {
					$rule_post                 = array();
					$rule_post[ 'ID' ]         = isset( $_POST[ 'cs_rule_id' ] ) ? sanitize_text_field( $_POST[ 'cs_rule_id' ] ) : null;
					$rule_post[ 'post_title' ] = isset( $_POST[ 'cs_post_name' ] ) ? sanitize_text_field( $_POST[ 'cs_post_name' ] ) : null;
					wp_update_post( wp_slash( $rule_post ) );
				}

				if ( isset( $_POST[ 'cs' ][ $this->plugin_name ][ 'cs_gdpr_rule' ] ) ) {
					$id = get_post_id_by_slug( 'cs_gdpr_rule' );
					update_post_meta( $id, '_cs_enable_rule', sanitize_text_field( $_POST[ 'cs' ][ $this->plugin_name ][ 'cs_gdpr_rule' ] ) );
				}
				if ( isset( $_POST[ 'cs' ][ $this->plugin_name ][ 'cs_ldu_rule' ] ) ) {
					$id = get_post_id_by_slug( 'cs_ldu_rule' );
					update_post_meta( $id, '_cs_enable_rule', sanitize_text_field( $_POST[ 'cs' ][ $this->plugin_name ][ 'cs_ldu_rule' ] ) );
				}
				if ( isset( $_POST[ 'cs' ][ $this->plugin_name ][ 'cs_rest_of_world_rule' ] ) ) {
					$id = get_post_id_by_slug( 'cs_rest_of_world_rule' );
					update_post_meta( $id, '_cs_enable_rule', sanitize_text_field( $_POST[ 'cs' ][ $this->plugin_name ][ 'cs_rest_of_world_rule' ] ) );
				}
				if ( isset( $_POST[ 'cs' ][ $this->plugin_name ][ 'cs_iab_rule' ] ) ) {
					$id = get_post_id_by_slug( 'cs_iab_rule' );
					update_post_meta( $id, '_cs_enable_rule', sanitize_text_field( $_POST[ 'cs' ][ $this->plugin_name ][ 'cs_iab_rule' ] ) );
				}

				if ( isset( $_POST[ 'cs' ][ 'custom_enable' ] ) && !empty( $_POST[ 'cs' ][ 'custom_enable' ] ) ) {
					foreach ( $_POST[ 'cs' ][ 'custom_enable' ] as $key => $val ) {
						$data_arr = array(
							'cookie_enabled' => sanitize_text_field( $val ),
						);
						update_scan_cookie_data( sanitize_text_field( $key ), $data_arr );
					}
				}

				if ( isset( $_POST[ 'cs' ][ 'custom_cookie_cat' ] ) && !empty( $_POST[ 'cs' ][ 'custom_cookie_cat' ] ) ) {
					foreach ( $_POST[ 'cs' ][ 'custom_cookie_cat' ] as $key => $val ) {
						$category_id = get_term_by( 'slug', sanitize_text_field( $val ), 'cs-cookies-category' );
						$data_arr    = array(
							'category'    => sanitize_text_field( $val ),
							'category_id' => $category_id->term_id
						);
						update_scan_cookie_data( sanitize_text_field( $key ), $data_arr );
					}
				}

				$unassigned = get_term_by( 'slug', 'unassigned', 'cs-cookies-category' );

				if ( isset( $_POST[ 'cs' ][ $this->plugin_name ] ) ) {
					foreach ( $_POST[ 'cs' ][ $this->plugin_name ] as $item_name => $val ) {
						$item_name = sanitize_text_field( $item_name );
						$val       = sanitize_text_field( $val );

						if ( stripos( $item_name, '_cookie_cat' ) ) {
							$cookie_id = preg_replace( "/[^0-9]/", '', $item_name );
							wp_set_object_terms( $cookie_id, $val, 'cs-cookies-category' );
						}

						if ( stripos( $item_name, '_script_cat' ) ) {
							$cookie_id = preg_replace( "/[^0-9]/", '', $item_name );
							wp_set_object_terms( $cookie_id, $val, 'cs-cookies-category' );
						}

						if ( stripos( $item_name, 'category_' . $unassigned->term_id ) ) {
							$cookie_id = preg_replace( "/[^0-9]/", '', $item_name );
							if ( $cookie_id == $unassigned->term_id ) {
								update_term_meta( $cookie_id, 'cs_ignore_this_category', $val );
							}
						}

						if ( stripos( $item_name, '_script_enable' ) ) {
							ConsentMagic()->updateOptions( array(  $item_name => $val ) );
						}
						if ( stripos( $item_name, '_cookie_enable' ) ) {
							ConsentMagic()->updateOptions( array( $item_name => $val ) );
						}
					}
				}

				$this->updateRuleIdHandle();

				$this->renderSuccessMessage();

				$this->addNewScriptCategoryHandle();
				$this->updateScriptsCategoryHandle();
			}

			$this->addDesignTemplateHandle();
			$this->updateDesignTemplateHandle();
			$this->deleteDesignTemplateHandle();

			$this->deleteCookieCategoryHandle();

			$this->addRuleHandle();

			$this->addScriptHandle();
			$this->updateScriptHandle();

			$this->addCookieHandle();
			$this->updateCookieHandle();

			if ( isset( $_POST[ 'cm-restore-defaults-design' ] ) && $_POST[ 'cm-restore-defaults-design' ] == 1 ) {
				ConsentMagic()->resetToDefaultsDesign();
			}

			if ( isset( $_POST[ 'cs_settings_ajax_update' ] ) && $_POST[ 'cs_settings_ajax_update' ] == 'delete_custom_post' ) {

				if ( isset( $_POST[ 'post_id' ] ) ) {
					$post_id  = sanitize_text_field( $_POST[ 'post_id' ] );
					$data_cat = isset( $_POST[ 'data_cat' ] ) ? sanitize_text_field( $_POST[ 'data_cat' ] ) : null;
					wp_delete_post( $post_id, true );
					ConsentMagic()->deleteOption( 'cs_' . $data_cat . '_' . $post_id . '_cookie_enable' );
					ConsentMagic()->deleteOption( 'cs_' . $post_id . '_cookie_cat' );
					ConsentMagic()->deleteOption( 'cs_' . $data_cat . '_' . $post_id . '_script_enable' );
					ConsentMagic()->deleteOption( 'cs_' . $post_id . '_script_cat' );
				}
			}
		}

		$this->renderHTML();
	}

	public function renderTitle() {
		add_action( 'init', function() {
			if ( isset( $_GET[ 'primary_rule_id' ] ) ) {
				$id    = sanitize_text_field( $_GET[ 'primary_rule_id' ] );
				$title = __( 'Edit', 'consent-magic' ) . ' ' . get_the_title( $id );
			} elseif ( isset( $_GET[ 'new_rule' ] ) ) {
				$title = __( 'New Rule', 'consent-magic' );
			} elseif ( isset( $_GET[ 'tab' ] ) ) {
				if ( $_GET[ 'tab' ] == 'cs-script-blocking' ) {
					$title = __( 'Scripts and Cookies', 'consent-magic' );
				} elseif ( $_GET[ 'tab' ] == 'cs-policy-gen' ) {
					$title = __( 'Privacy Policy', 'consent-magic' );
				} elseif ( $_GET[ 'tab' ] == 'cs-text' ) {
					$title = __( 'Text', 'consent-magic' );
				} elseif ( $_GET[ 'tab' ] == 'cs-multi-step-design' ) {
					$title = __( 'Multi-step design', 'consent-magic' );
				} elseif ( $_GET[ 'tab' ] == 'cs-single-step-design' ) {
					$title = __( 'Single-step design', 'consent-magic' );
				} else {
					$title = __( 'ConsentMagic PRO', 'consent-magic' );
				}
			} elseif ( isset( $_GET[ 'new_script' ] ) || isset( $_GET[ 'script_id' ] ) ) {
				if ( isset( $_GET[ 'script_id' ] ) ) {
					$id    = sanitize_text_field( $_GET[ 'script_id' ] );
					$title = __( 'Edit', 'consent-magic' ) . ' ' . get_the_title( $id );
				} else {
					$title = __( 'New Script', 'consent-magic' );
				}
			} elseif ( isset( $_GET[ 'design_template' ] ) ) {
				if ( $_GET[ 'design_template' ] == 'add' ) {
					$title = __( 'New design template', 'consent-magic' );
				} else {
					$id    = isset( $_GET[ 'template_id' ] ) ? sanitize_text_field( $_GET[ 'template_id' ] ) : '';
					$title = __( 'Edit design template', 'consent-magic' ) . ' ' . get_the_title( $id );
				}
			} else {
				$title = __( 'ConsentMagic PRO', 'consent-magic' );
			}

			$this->page_title = $title;
		} );
	}

	/**
	 * @return void
	 */
	private function dbCronHandle() {
		if ( ConsentMagic()->getOption( 'cs_geolocation' ) && ConsentMagic()->getOption( 'cs_geo_activated' ) && ConsentMagic()->getOption( 'cs_often_update' ) != 'never' ) {
			db_cron_update();
		} else {
			db_cron_deactivate();
		}
	}

	public function change_rule_order_admin( $rule_id ) {
		$args_posts = array(
			'post_type'      => CMPRO_POST_TYPE,
			'post_status'    => 'publish',
			'include'        => array( $rule_id ),
			'posts_per_page' => 1,
		);
		$rule_first = get_posts( $args_posts );

		$args_posts = array(
			'post_type'      => CMPRO_POST_TYPE,
			'post_status'    => 'publish',
			'posts_per_page' => -1,
			'order'          => 'ASC',
			'exclude'        => $rule_id

		);
		$rule_other = get_posts( $args_posts );

		if ( $rule_first != false && !empty( $rule_first ) ) {
			$counter = 0;
			update_post_meta( $rule_first[ 0 ]->ID, '_cs_order', $counter );

			$counter++;
			if ( $rule_other != false && !empty( $rule_other ) ) {
				foreach ( $rule_other as $rule ) {
					update_post_meta( $rule->ID, '_cs_order', $counter );
					$counter++;
				}
			}
		}
	}

	private function updateCronHandle() {
		$cs_auto_scan_interval = ConsentMagic()->getOption( 'cs_auto_scan_interval' );
		if ( isset( $_POST[ 'cs' ][ $this->plugin_name ][ 'cs_auto_scan_interval' ] ) && ( $_POST[ 'cs' ][ $this->plugin_name ][ 'cs_auto_scan_interval' ] !== $cs_auto_scan_interval ) ) {
			if ( wp_next_scheduled( 'cs_cron_bulck_scan_hook' ) ) {
				wp_clear_scheduled_hook( 'cs_cron_bulck_scan_hook' );
				if ( $_POST[ 'cs' ][ $this->plugin_name ][ 'cs_auto_scan_interval' ] == "once_a_week" ) {
					wp_schedule_event( time(), 'weekly', 'cs_cron_bulck_scan_hook' );
				} else if ( $_POST[ 'cs' ][ $this->plugin_name ][ 'cs_auto_scan_interval' ] == "once_a_month" ) {
					wp_schedule_event( time(), 'monthly', 'cs_cron_bulck_scan_hook' );
				} else {
					if ( wp_next_scheduled( 'cs_cron_bulck_scan_hook' ) ) {
						wp_clear_scheduled_hook( 'cs_cron_bulck_scan_hook' );
					}
				}
			}
		}
	}

	private function updateOptionsHandle() {
		if ( isset( $_POST[ 'cs' ][ $this->plugin_name ] ) && is_array( $_POST[ 'cs' ][ $this->plugin_name ] ) ) {
			$the_options = ConsentMagic()->getCSOptions();

			$post_data = $_POST[ 'cs' ][ $this->plugin_name ];
			array_walk( $post_data, array(
				ConsentMagic(),
				'cs_sanitize_array'
			) );

			if ( isset( $post_data[ 'text' ] ) ) {
				foreach (  $post_data[ 'text' ] as $option => $languages ) {
					if ( isset( $post_data[ "{$option}_language_control" ] ) ) {
						if ($post_data[ "{$option}_language_control" ] == 1  ) {
							ConsentMagic()->clearLangOptions( $option );
							$update_langs = array();
							foreach ( $languages as $key_lang => $language ) {
								$description = wp_kses_post( wp_unslash( $_POST[ 'cs' ][ $this->plugin_name ][ 'text' ][ $option ][ $key_lang ] ) );
								if ( !empty( $description ) ) {
									ConsentMagic()->updateLangOptions( $option, $description, $key_lang );
									$update_langs[] = $key_lang;
								}
							}
							ConsentMagic()->update_language_availability( $update_langs );
						} else {
							foreach ( $languages as $key_lang => $language ) {
								$description = wp_kses_post( wp_unslash( $_POST[ 'cs' ][ $this->plugin_name ][ 'text' ][ $option ][ $key_lang ] ) );
								if ( !empty( $description ) ) {
									ConsentMagic()->updateLangOptions( $option, $description, $key_lang );
								}
							}
						}
					}
				}
			}

			foreach ( $the_options as $key => $value ) {
				if ( isset( $post_data[ $key ] ) ) {
					switch ( $key ) {
						case 'cs_auto_scan_email':
							$email = sanitize_email( $post_data[ $key ] );
							if ( !empty( $email ) ) {
								ConsentMagic()->updateOptions( array( $key => $email ) );
							}
							break;
						case 'cs_policy_existing_page':
							$pages = array();
							if ( is_array( $post_data[ $key ] ) ) {
								foreach ( $post_data[ $key ] as $lang => $page_id ) {
									if ( $lang != 'new' ) {
										$pages[ $lang ] = $page_id;
									}
								}
							}
							ConsentMagic()->updateOptions( array( $key => $pages ) );
							break;
						default:
							ConsentMagic()->updateOptions( array( $key =>  $post_data[ $key ] ) );
							break;
					}

					//Save pys cookie category to scan result table
					if ( $key == 'cs_block_pys_scripts_cat' ) {
						$scanner = new CS_Scanner_Module();
						$scanner->update_scan_pys_category(  $post_data[ $key ] );
					}
				}
			}

			if ( isset( $_POST[ 'cs' ][ $this->plugin_name ][ '_cs_top_push' ] ) ) {
				ConsentMagic()->updateOptions( array( 'cs_top_push' => sanitize_text_field( $_POST[ 'cs' ][ $this->plugin_name ][ '_cs_top_push' ] ) ) );
			}
		}
	}

	private function updateRuleIdHandle() {
		if ( isset( $_POST[ 'cs_rule_id' ] ) && isset( $_POST[ 'cs' ][ $this->plugin_name ] ) ) {

			$rule_id   = sanitize_text_field( $_POST[ 'cs_rule_id' ] );

			$post_data = $_POST[ 'cs' ][ $this->plugin_name ];
			array_walk( $post_data, array(
				ConsentMagic(),
				'cs_sanitize_array'
			) );

			//update consent version, if cs_type doesn't match
			if ( isset( $post_data[ '_cs_type' ] ) ) {
				$old_type = get_post_meta( $rule_id, '_cs_type', true );
				$new_type = $post_data[ '_cs_type' ];

				if ( $old_type != $new_type ) {
					$this->admin->renew_consent_run();
				}
			}

			if ( isset( $post_data[ 'text' ] ) ) {
				foreach (  $post_data[ 'text' ] as $option => $languages ) {
					ConsentMagic()->clearLangOptions( $option, 'meta', (int)$rule_id );
					$update_langs = array();
					foreach ( $languages as $key_lang => $language ) {
						$description = wp_kses_post( wp_unslash( $_POST[ 'cs' ][ $this->plugin_name ][ 'text' ][ $option ][ $key_lang ] ) );
						if ( !empty( $description ) ) {
							ConsentMagic()->updateLangOptions( $option, $description, $key_lang, 'meta', (int)$rule_id );
							$update_langs[] = $key_lang;
						}
					}

					ConsentMagic()->update_language_availability( $update_langs );
				}
			}

			foreach ( $post_data as $item_name => $val ) {
				if ( gettype( $val ) === 'string' ) {
					update_post_meta( $rule_id,  $item_name,  $val );
				}
			}
		}
	}

	private function updateDesignTemplateHandle() {
		if ( isset( $_POST[ 'cs_update_action' ] ) && $_POST[ 'cs_update_action' ] == 'update_design_template_form' ) {

			$post_data = array();
			array_walk( $_POST[ 'cs' ][ $this->plugin_name ], function( &$value, $key ) use ( &$post_data ) {
				$post_data[ sanitize_text_field( $key ) ] = sanitize_text_field( $value );
			} );

			$post_id = $post_data[ 'cs_template_id' ];

			$template_data = array(
				'ID'         => $post_id,
				'post_title' => $post_data[ 'cs_post_name' ],
				'meta_input' => array(
					'cs_backend_color'                    => $post_data[ 'cs_backend_color' ],
					'cs_border_style'                     => $post_data[ 'cs_border_style' ],
					'cs_border_weight'                    => $post_data[ 'cs_border_weight' ],
					'cs_border_color'                     => $post_data[ 'cs_border_color' ],
					'cs_text_block_bg'                    => $post_data[ 'cs_text_block_bg' ],
					'cs_text_color'                       => $post_data[ 'cs_text_color' ],
					'cs_links_color'                      => $post_data[ 'cs_links_color' ],
					'cs_titles_text_color'                => $post_data[ 'cs_titles_text_color' ],
					'cs_subtitles_text_color'             => $post_data[ 'cs_subtitles_text_color' ],
					'cs_accept_all_buttons_bg'            => $post_data[ 'cs_accept_all_buttons_bg' ],
					'cs_accept_all_buttons_text_color'    => $post_data[ 'cs_accept_all_buttons_text_color' ],
					'cs_custom_button_buttons_bg'         => $post_data[ 'cs_custom_button_buttons_bg' ],
					'cs_custom_button_buttons_text_color' => $post_data[ 'cs_custom_button_buttons_text_color' ],
					'cs_deny_all_buttons_bg'              => $post_data[ 'cs_deny_all_buttons_bg' ],
					'cs_deny_all_buttons_text_color'      => $post_data[ 'cs_deny_all_buttons_text_color' ],
					'cs_options_buttons_text_color'       => $post_data[ 'cs_options_buttons_text_color' ],
					'cs_options_buttons_bg'               => $post_data[ 'cs_options_buttons_bg' ],
					'cs_confirm_buttons_bg'               => $post_data[ 'cs_confirm_buttons_bg' ],
					'cs_confirm_buttons_text_color'       => $post_data[ 'cs_confirm_buttons_text_color' ],
					'cs_sticky_bg'                        => $post_data[ 'cs_sticky_bg' ],
					'cs_sticky_link_color'                => $post_data[ 'cs_sticky_link_color' ],
					'cs_logo'                             => $post_data[ 'cs_logo' ],
					'cs_position_vertical_list'           => $post_data[ 'cs_position_vertical_list' ],
					'cs_position_horizontal_list'         => $post_data[ 'cs_position_horizontal_list' ],
					'cs_logo_size'                        => $post_data[ 'cs_logo_size' ],
					'cs_cat_color'                        => $post_data[ 'cs_cat_color' ],
					'cs_active_toggle_color'              => $post_data[ 'cs_active_toggle_color' ],
					'cs_active_toggle_text_color'         => $post_data[ 'cs_active_toggle_text_color' ],
					'cs_main_template'                    => '',
					'cs_shortcodes_text_color'            => $post_data[ 'cs_shortcodes_text_color' ],
					'cs_tab_buttons_bg'                   => $post_data[ 'cs_tab_buttons_bg' ],
					'cs_tab_buttons_text_color'           => $post_data[ 'cs_tab_buttons_text_color' ],
				)
			);

			wp_update_post( $template_data );

			$this->renderSuccessMessage();
		}
	}

	private function addDesignTemplateHandle() {
		if ( isset( $_POST[ 'cs_update_action' ] ) && $_POST[ 'cs_update_action' ] == 'add_design_template_form' ) {

			$post_data = array();
			array_walk( $_POST[ 'cs' ][ $this->plugin_name ], function( &$value, $key ) use ( &$post_data ) {
				$post_data[ sanitize_text_field( $key ) ] = sanitize_text_field( $value );
			} );

			$template_data = array(
				'post_type'   => CMPRO_TEMPLATE_POST_TYPE,
				'post_title'  => $post_data[ 'cs_default_template_name' ],
				'post_status' => 'publish',
				'ping_status' => 'closed',
				'post_author' => 1,
				'meta_input'  => array(
					'cs_backend_color'                    => $post_data[ 'cs_backend_color' ],
					'cs_border_style'                     => $post_data[ 'cs_border_style' ],
					'cs_border_weight'                    => $post_data[ 'cs_border_weight' ],
					'cs_border_color'                     => $post_data[ 'cs_border_color' ],
					'cs_text_block_bg'                    => $post_data[ 'cs_text_block_bg' ],
					'cs_text_color'                       => $post_data[ 'cs_text_color' ],
					'cs_links_color'                      => $post_data[ 'cs_links_color' ],
					'cs_titles_text_color'                => $post_data[ 'cs_titles_text_color' ],
					'cs_subtitles_text_color'             => $post_data[ 'cs_subtitles_text_color' ],
					'cs_accept_all_buttons_bg'            => $post_data[ 'cs_accept_all_buttons_bg' ],
					'cs_accept_all_buttons_text_color'    => $post_data[ 'cs_accept_all_buttons_text_color' ],
					'cs_custom_button_buttons_bg'         => $post_data[ 'cs_custom_button_buttons_bg' ],
					'cs_custom_button_buttons_text_color' => $post_data[ 'cs_custom_button_buttons_text_color' ],
					'cs_deny_all_buttons_bg'              => $post_data[ 'cs_deny_all_buttons_bg' ],
					'cs_deny_all_buttons_text_color'      => $post_data[ 'cs_deny_all_buttons_text_color' ],
					'cs_options_buttons_text_color'       => $post_data[ 'cs_options_buttons_text_color' ],
					'cs_options_buttons_bg'               => $post_data[ 'cs_options_buttons_bg' ],
					'cs_confirm_buttons_bg'               => $post_data[ 'cs_confirm_buttons_bg' ],
					'cs_confirm_buttons_text_color'       => $post_data[ 'cs_confirm_buttons_text_color' ],
					'cs_sticky_bg'                        => $post_data[ 'cs_sticky_bg' ],
					'cs_sticky_link_color'                => $post_data[ 'cs_sticky_link_color' ],
					'cs_logo'                             => $post_data[ 'cs_logo' ],
					'cs_position_vertical_list'           => $post_data[ 'cs_position_vertical_list' ],
					'cs_position_horizontal_list'         => $post_data[ 'cs_position_horizontal_list' ],
					'cs_logo_size'                        => $post_data[ 'cs_logo_size' ],
					'cs_cat_color'                        => $post_data[ 'cs_cat_color' ],
					'cs_active_toggle_color'              => $post_data[ 'cs_active_toggle_color' ],
					'cs_active_toggle_text_color'         => $post_data[ 'cs_active_toggle_text_color' ],
					'cs_main_template'                    => '',
					'cs_shortcodes_text_color'            => $post_data[ 'cs_shortcodes_text_color' ],
					'cs_tab_buttons_bg'                   => $post_data[ 'cs_tab_buttons_bg' ],
					'cs_tab_buttons_text_color'           => $post_data[ 'cs_tab_buttons_text_color' ],
				)
			);

			wp_insert_post( $template_data );

			$this->renderSuccessMessage();
		}
	}

	private function addNewScriptCategoryHandle() {
		if ( isset( $_POST[ 'cs' ][ $this->plugin_name ][ 'new_category' ] ) && is_array( $_POST[ 'cs' ][ $this->plugin_name ][ 'new_category' ] ) ) {

			if ( !empty( $_POST[ 'cs' ][ $this->plugin_name ][ 'new_category' ][ 'title' ] ) && !empty( $_POST[ 'cs' ][ $this->plugin_name ][ 'new_category' ][ 'description' ] ) ) {
				$new_slug    = sanitize_title( $_POST[ 'cs' ][ $this->plugin_name ][ 'new_category' ][ 'title' ] );
				$description = wp_kses_post( wp_unslash( $_POST[ 'cs' ][ $this->plugin_name ][ 'new_category' ][ 'description' ] ) );
				$term        = get_term_by( 'slug', $new_slug, 'cs-cookies-category' );

				if ( !$term ) {
					global $sitepress;
					$wpml_default_lang = 'en';
					$wpml_current_lang = 'en';
					if ( function_exists( 'icl_object_id' ) && $sitepress ) {
						$wpml_default_lang = $sitepress->get_default_language();
						$wpml_current_lang = apply_filters( 'wpml_current_language', NULL );
					}
					//check only in default language
					if ( $wpml_default_lang == $wpml_current_lang ) {
						$cid = wp_insert_term( $new_slug, 'cs-cookies-category', array(
							'slug' => $new_slug
						) );
						if ( !is_wp_error( $cid ) ) {
							$description = array(
								'name'  => $new_slug,
								'descr' => $description,
							);
							// Get term_id, set default as 0 if not set
							$cat_id = isset( $cid[ 'term_id' ] ) ? $cid[ 'term_id' ] : 0;
							add_term_meta( $cat_id, 'cs_primary_term', 'custom', true );
							add_term_meta( $cat_id, 'cs_necessary_term', 'custom', true );
							add_term_meta( $cat_id, 'cs_ignore_this_category', '0', true );
							add_term_meta( $cat_id, '_cs_iab_cat', isset( $_POST[ 'cs' ][ $this->plugin_name ][ 'new_category' ][ 'iab_cat' ] ) ? sanitize_text_field( $_POST[ 'cs' ][ $this->plugin_name ][ 'new_category' ][ 'iab_cat' ] ) : null, true );
							add_term_meta( $cat_id, '_cs_wp_consent_api_cat', isset( $_POST[ 'cs' ][ $this->plugin_name ][ 'new_category' ][ 'cs_wp_consent_api_cat' ] ) ? sanitize_text_field( $_POST[ 'cs' ][ $this->plugin_name ][ 'new_category' ][ 'cs_wp_consent_api_cat' ] ) : null, true );
							ConsentMagic()->updateOptions( array( $new_slug . '_cat_id' =>  $cat_id ) );
							ConsentMagic()->updateLangOptions( $new_slug,  $description, CMPRO_DEFAULT_LANGUAGE, 'term' );
						}
					}
				}
			}
		}
	}

	private function updateScriptsCategoryHandle() {
		if ( isset( $_POST[ 'cs' ][ $this->plugin_name ][ 'categories' ] ) && is_array( $_POST[ 'cs' ][ $this->plugin_name ][ 'categories' ] ) ) {

			$post_data = $_POST[ 'cs' ][ $this->plugin_name ][ 'categories' ];
			if ( is_array( $post_data ) ) {
				array_walk( $post_data, array(
					ConsentMagic(),
					'cs_sanitize_array'
				) );
			} else {
				$post_data = sanitize_text_field( $post_data );
			}

			if ( isset( $_POST[ 'cs' ][ $this->plugin_name ][ 'cs_not_block_unassigned_cookies' ] ) ) {
				ConsentMagic()->updateOptions( array( 'cs_not_block_unassigned_cookies' => sanitize_text_field( $_POST[ 'cs' ][ $this->plugin_name ][ 'cs_not_block_unassigned_cookies' ] ) ) );
			}
			if ( isset( $_POST[ 'cs' ][ $this->plugin_name ][ 'default_font' ] ) ) {
				ConsentMagic()->updateOptions( array( 'default_font' => sanitize_text_field( $_POST[ 'cs' ][ $this->plugin_name ][ 'default_font' ] ) ) );
			}

			foreach ( $post_data as $cat_id => $category_langs ) {
				if ( isset( $_POST[ 'cs' ][ $this->plugin_name ][ "cs_language_control_{$cat_id}" ] ) ) {
					$langs = array();

					$term = get_term_by( 'ID', $cat_id, 'cs-cookies-category' );
					if ( !$term ) {
						continue;
					}

					if ( $_POST[ 'cs' ][ $this->plugin_name ][ "cs_language_control_{$cat_id}" ] == 1 ) {
						ConsentMagic()->clearLangOptions( $term->slug, 'term' );
					}

					foreach ( $category_langs as $lang_key => $category ) {
						if ( !empty( $category[ 'title' ] ) && !empty( $category[ 'description' ] ) ) {
							$langs[] = $lang_key;

							$description = array(
								'name'  => wp_unslash($category[ 'title' ] ),
								'descr' => wp_kses_post( wp_unslash( $_POST[ 'cs' ][ $this->plugin_name ][ 'categories' ][ $cat_id ][ $lang_key ][ 'description' ] ) ),
							);

							ConsentMagic()->updateLangOptions( $term->slug,  $description, $lang_key, 'term' );

							if ( isset( $category[ 'ignore_this_category' ] ) ) {
								update_term_meta( $cat_id, 'cs_ignore_this_category', !$category[ 'ignore_this_category' ] );
							}

							if ( isset( $category[ 'iab_cat' ] ) ) {
								update_term_meta( $cat_id, '_cs_iab_cat', $category[ 'iab_cat' ] );
							}

							if ( isset( $category[ 'cs_wp_consent_api_cat' ] ) ) {
								update_term_meta( $cat_id, '_cs_wp_consent_api_cat', $category[ 'cs_wp_consent_api_cat' ] );
							}
						}
					}

					if ( $_POST[ 'cs' ][ $this->plugin_name ][ "cs_language_control_{$cat_id}" ] == 1 ) {
						ConsentMagic()->update_language_availability( $langs );
					}
				}
			}
		}
	}

	private function deleteCookieCategoryHandle() {
		if ( isset( $_POST[ 'cs_settings_ajax_update' ] ) && $_POST[ 'cs_settings_ajax_update' ] == 'delete_cookie_cat' && isset( $_POST[ 'cs_cookie_cat_id' ] ) ) {

			$id   = sanitize_text_field( $_POST[ 'cs_cookie_cat_id' ] );
			$term = get_term_by( 'ID', $id, 'cs-cookies-category' );

			$scripts = get_posts( array(
				'post_type'   => CMPRO_POST_TYPE_SCRIPTS,
				'numberposts' => -1,
				'post_status' => 'any',
				'tax_query'   => array(
					array(
						'taxonomy'         => 'cs-cookies-category',
						'field'            => 'term_id',
						'terms'            => $id,
						'include_children' => false
					)
				)
			) );

			$update_category = 'unassigned';
			$unassigned      = get_term_by( 'slug', $update_category, 'cs-cookies-category' );

			if ( $scripts ) {
				foreach ( $scripts as $script ) {
					ConsentMagic()->updateOptions( array(  'cs_' . $script->ID . '_cookie_cat' => '' ) );
					ConsentMagic()->updateOptions( array( 'cs_' . $script->ID . '_script_cat' => $update_category ) );
					wp_set_object_terms( $script->ID, $unassigned->term_id, 'cs-cookies-category' );
				}
			}

			$script_list = get_predefined_script_pattern();
			foreach ( $script_list as $script ) {
				$category = ConsentMagic()->getOption( $script[ 'cat_key' ] );
				if ( $category == $term->slug ) {
					ConsentMagic()->updateOptions( array( $script[ 'cat_key' ] => $update_category ) );
				}
			}

			if ( $term ) {
				wp_delete_term( $id, 'cs-cookies-category', array(
					'default'       => $unassigned->term_id,
					'force_default' => true
				) );
				ConsentMagic()->deleteOption( $term->slug . '_cat_id' );
			}
		}
	}

	private function deleteDesignTemplateHandle() {
		if ( isset( $_POST[ 'cs_settings_ajax_update' ] ) && $_POST[ 'cs_settings_ajax_update' ] == 'delete_template' ) {

			if ( isset( $_POST[ 'template_id' ] ) ) {
				$id = sanitize_text_field( $_POST[ 'template_id' ] );
				$scripts = get_posts( array(
					'post_type'   => CMPRO_POST_TYPE,
					'numberposts' => -1,
					'post_status' => 'any',
				) );

				if ( $scripts ) {
					foreach ( $scripts as $script ) {
						$script_theme = get_post_meta( $script->ID, '_cs_theme', true );
						if ( $script_theme == $id ) {
							update_post_meta( $script->ID, '_cs_theme', get_page_id_by_path( 'cs_light_theme', '', CMPRO_TEMPLATE_POST_TYPE ) );
						}
					}
				}

				wp_delete_post( $id, true );
			}
		}
	}

	private function addRuleHandle() {
		if ( isset( $_POST[ 'cs_update_action' ] ) && $_POST[ 'cs_update_action' ] == 'add_rule_form' && isset( $_POST[ 'cs' ][ $this->plugin_name ] ) ) {

			$unassigned         = get_term_by( 'slug', 'unassigned', 'cs-cookies-category' );
			$unassigned_id      = $unassigned->term_id;
			$categories         = get_cookies_terms_objects( null, false, $unassigned_id );
			$categories_options = array();

			if ( !empty( $categories ) ) {
				foreach ( $categories as $category ) {
					if ( isset( $_POST[ 'cs' ][ $this->plugin_name ][ '_cs_smart_sticky_' . $category->term_id ] ) ) {
						$categories_options[ '_cs_smart_sticky_' . $category->term_id ] = sanitize_text_field( $_POST[ 'cs' ][ $this->plugin_name ][ '_cs_smart_sticky_' . $category->term_id ] );
					}

					if ( isset( $_POST[ 'cs' ][ $this->plugin_name ][ '_cs_smart_sticky_mobile_' . $category->term_id ] ) ) {
						$categories_options[ '_cs_smart_sticky_mobile_' . $category->term_id ] = sanitize_text_field( $_POST[ 'cs' ][ $this->plugin_name ][ '_cs_smart_sticky_mobile_' . $category->term_id ] );
					}

					if ( isset( $_POST[ 'cs' ][ $this->plugin_name ][ '_cs_custom_button_' . $category->term_id ] ) ) {
						$categories_options[ '_cs_custom_button_' . $category->term_id ] = sanitize_text_field( $_POST[ 'cs' ][ $this->plugin_name ][ '_cs_custom_button_' . $category->term_id ] );
					}
				}
			}

			$post_data = $_POST[ 'cs' ][ $this->plugin_name ];
			array_walk( $post_data, array(
				ConsentMagic(),
				'cs_sanitize_array'
			) );

			if ( !$this->admin->cs_post_exists_by_slug( $post_data[ 'cs_default_rule_name' ], CMPRO_POST_TYPE ) ) {

				$category = get_term_by( 'slug', 'rules', 'cs-category' );
				if ( $category && is_object( $category ) ) {
					$posts_in_category           = $category->count;
					$order                       = $posts_in_category;
					$category_id                 = $category->term_id;
					$cookie_data                 = array(
						'post_type'     => CMPRO_POST_TYPE,
						'post_title'    => $post_data[ 'cs_default_rule_name' ],
						'post_category' => array( $category_id ),
						'post_status'   => 'publish',
						'ping_status'   => 'closed',
						'post_author'   => 1,
						'meta_input'    => array(
							'_cs_type'                               => $post_data[ '_cs_type' ] ?? null,
							'_cs_sticky'                             => $post_data[ '_cs_sticky' ] ?? null,
							'_cs_smart_sticky'                       => $post_data[ '_cs_smart_sticky' ] ?? null,
							'_cs_smart_sticky_mobile'                => $post_data[ '_cs_smart_sticky_mobile' ] ?? null,
							'_cs_mobile_side_sticky'                 => $post_data[ '_cs_mobile_side_sticky' ] ?? null,
							'_cs_custom_button'                      => $post_data[ '_cs_custom_button' ] ?? null,
							'_cs_bars_position'                      => $post_data[ '_cs_bars_position' ] ?? null,
							'_cs_bars_type'                          => $post_data[ '_cs_bars_type' ] ?? null,
							'_cs_top_push'                           => $post_data[ '_cs_top_push' ] ?? null,
							'_cs_privacy_link'                       => $post_data[ '_cs_privacy_link' ] ?? null,
							'_cs_close_on_scroll'                    => $post_data[ '_cs_close_on_scroll' ] ?? null,
							'_cs_theme'                              => $post_data[ '_cs_theme' ] ?? null,
							'_cs_design_type'                        => $post_data[ '_cs_design_type' ] ?? null,
							'_cs_deny_all_btn'                       => $post_data[ '_cs_deny_all_btn' ] ?? null,
							'_cs_hide_close_btn'                     => $post_data[ '_cs_hide_close_btn' ] ?? null,
							'_cs_deny_consent_for_close'             => $post_data[ '_cs_deny_consent_for_close' ] ?? null,
							'_cs_predefined_rule'                    => 0,
							'_cs_target'                             => $post_data[ '_cs_target' ] ?? null,
							'_cs_us_states_target'                   => $post_data[ '_cs_us_states_target' ] ?? null,
							'_cs_custom_text'                        => $post_data[ '_cs_custom_text' ] ?? null,
							'_cs_order'                              => $order,
							'_cs_track_analytics'                    => $post_data[ '_cs_track_analytics' ] ?? null,
							'_cs_block_content'                      => $post_data[ '_cs_block_content' ] ?? null,
							'_cs_refresh_after_consent'              => $post_data[ '_cs_refresh_after_consent' ] ?? 0,
							'_cs_enable_rule'                        => $post_data[ '_cs_enable_rule' ] ?? null,
							'_cs_no_ip_rule'                         => $post_data[ '_cs_no_ip_rule' ] ?? null,
							'_excluded_from_consent_storing'         => $post_data[ '_excluded_from_consent_storing' ] ?? null,
							'_cs_showing_rule_until_express_consent' => $post_data[ '_cs_showing_rule_until_express_consent' ] ?? null,
							'_cs_native_scripts'                     => $post_data[ '_cs_native_scripts' ] ?? null,
							'_cs_use_meta_ldu'                       => $post_data[ '_cs_use_meta_ldu' ] ?? null,
							'_cs_google_consent_mode'                => $post_data[ '_cs_google_consent_mode' ] ?? null,
							'_cs_bing_consent_mode'                  => $post_data[ '_cs_bing_consent_mode' ] ?? null,
							'_cs_reddit_ldu'                         => $post_data[ '_cs_reddit_ldu' ] ?? null,
						)
					);
					$cookie_data[ 'meta_input' ] = array_merge( $cookie_data[ 'meta_input' ], $categories_options );

					$post_id = wp_insert_post( $cookie_data );
					wp_set_object_terms( $post_id, $cookie_data[ 'post_category' ], 'cs-category' );

					if ( isset( $post_data[ 'text' ] ) ) {
						foreach (  $post_data[ 'text' ] as $option => $languages ) {
							$update_langs = array();
							foreach ( $languages as $key_lang => $language ) {
								$description = wp_kses_post( wp_unslash( $_POST[ 'cs' ][ $this->plugin_name ][ 'text' ][ $option ][ $key_lang ] ) );
								if ( !empty( $description ) ) {
									ConsentMagic()->updateLangOptions( $option, $description, $key_lang, 'meta', $post_id );
									$update_langs[] = $key_lang;
								}
							}

							ConsentMagic()->update_language_availability( $update_langs );
						}
					}
				}
			}

			$this->renderSuccessMessage();
		}
	}

	private function addScriptHandle() {
		if ( isset( $_POST[ 'cs_update_action' ] ) && $_POST[ 'cs_update_action' ] == 'add_script_form' ) {

			$post_data = $_POST[ 'cs' ][ $this->plugin_name ];

			array_walk( $post_data, array(
				ConsentMagic(),
				'cs_sanitize_array'
			) );

			if ( !$this->admin->cs_post_exists_by_slug( $post_data[ 'cs_default_script_name' ], CMPRO_POST_TYPE_SCRIPTS ) ) {
				if ( !empty( $post_data[ 'cs_default_script_name' ] ) && !empty( $post_data[ 'cs_default_script_js_heedle' ] ) ) {
					$category = get_term_by( 'slug', $post_data[ 'add_new_script_select' ], 'cs-cookies-category' );
					if ( $category && is_object( $category ) ) {
						$needle = array();
						foreach ( $post_data[ 'cs_default_script_js_heedle' ] as $value ) {
							if ( !empty( $value ) ) {
								$needle[] = cs_sanitize_url( $value );
							}
						}

						$category_id = $category->term_id;
						$cookie_data = array(
							'post_type'     => CMPRO_POST_TYPE_SCRIPTS,
							'post_title'    => $post_data[ 'cs_default_script_name' ],
							'post_category' => array( $category_id ),
							'post_status'   => 'publish',
							'ping_status'   => 'closed',
							'post_author'   => 1,
							'meta_input'    => array(
								'cs_default_script_desc'      => isset( $_POST[ 'cs' ][ $this->plugin_name ][ 'cs_default_script_desc' ] ) ? wp_kses_post( wp_unslash( $_POST[ 'cs' ][ $this->plugin_name ][ 'cs_default_script_desc' ] ) ) : null,
								'cs_default_script_js_heedle' => implode( ',', $needle )
							)
						);
						$post_id     = wp_insert_post( $cookie_data );
						wp_set_object_terms( $post_id, $cookie_data[ 'post_category' ], 'cs-cookies-category' );
						ConsentMagic()->updateOptions( array(  'cs_' . sanitize_title( $post_data[ 'cs_default_script_name' ] ) . '_' . $post_id . '_script_enable' =>  '1' ) );
						ConsentMagic()->updateOptions( array( 'cs_' . $post_id . '_script_cat' => $post_data[ 'add_new_script_select' ] ) );
					}
				}
			}

			$this->renderSuccessMessage();
		}
	}

	private function updateScriptHandle() {
		if ( isset( $_POST[ 'cs_update_action' ] ) && $_POST[ 'cs_update_action' ] == 'update_script_form' ) {

			$post_data = $_POST[ 'cs' ][ $this->plugin_name ];

			array_walk( $post_data, array(
				ConsentMagic(),
				'cs_sanitize_array'
			) );

			if ( !empty( $post_data[ 'cs_default_script_js_heedle' ] ) && is_array( $post_data[ 'cs_default_script_js_heedle' ] ) ) {
				$post_id    = $post_data[ 'cs_script_id' ];
				$script_cat = $post_data[ 'cs_' . $post_id . '_script_cat' ] ?? null;
				$category   = get_term_by( 'slug', $script_cat, 'cs-cookies-category' );
				if ( $category && is_object( $category ) ) {
					$category_id = $category->term_id;
					wp_set_object_terms( $post_id, $category_id, 'cs-cookies-category' );
					ConsentMagic()->updateOptions( array( 'cs_' . $script_cat . '_' . $post_id . '_script_enable' => true ) );
					ConsentMagic()->updateOptions( array( 'cs_' . $post_id . '_script_cat' => $script_cat ) );
				}

				$needle = array();
				foreach ( $post_data[ 'cs_default_script_js_heedle' ] as $value ) {
					if ( !empty( $value ) ) {
						$needle[] = cs_sanitize_url( $value );
					}
				}
				$template_data = array(
					'ID'         => $post_id,
					'meta_input' => array(
						'cs_default_script_desc'      => isset( $_POST[ 'cs' ][ $this->plugin_name ][ 'cs_default_script_desc' ] ) ? wp_kses_post( wp_unslash( $_POST[ 'cs' ][ $this->plugin_name ][ 'cs_default_script_desc' ] ) ) : null,
						'cs_default_script_js_heedle' => implode( ',', $needle )
					)
				);

				wp_update_post( $template_data );
				$this->renderSuccessMessage();
			}
		}
	}

	private function addCookieHandle() {
		if ( isset( $_POST[ 'cs_update_action' ] ) && $_POST[ 'cs_update_action' ] == 'add_cookie_form' ) {

			$post_data = array();
			array_walk( $_POST[ 'cs' ][ $this->plugin_name ], function( &$value, $key ) use ( &$post_data ) {
				$post_data[ sanitize_text_field( $key ) ] = sanitize_text_field( $value );
			} );

			if ( !$this->admin->cs_post_exists_by_slug( $post_data[ 'cs_default_cookie_name' ], CMPRO_POST_TYPE_COOKIES ) ) {

				$category = get_term_by( 'slug', $post_data[ 'add_new_cookie_select' ], 'cs-cookies-category' );
				if ( $category && is_object( $category ) ) {
					$category_id = $category->term_id;
					$cookie_data = array(
						'post_type'     => CMPRO_POST_TYPE_COOKIES,
						'post_title'    => $post_data[ 'cs_default_cookie_name' ],
						'post_category' => array( $category_id ),
						'post_status'   => 'publish',
						'ping_status'   => 'closed',
						'post_author'   => 1,
						'meta_input'    => array(
							'cs_default_cookie_desc' => $post_data[ 'cs_default_cookie_desc' ],
						)
					);
					$post_id     = wp_insert_post( $cookie_data );
					wp_set_object_terms( $post_id, $cookie_data[ 'post_category' ], 'cs-cookies-category' );
					ConsentMagic()->updateOptions( array(  'cs_' . sanitize_title( $post_data[ 'cs_default_cookie_name' ] ) . '_' . $post_id . '_cookie_enable' => '1' ) );
					ConsentMagic()->updateOptions( array(  'cs_' . $post_id . '_cookie_cat' => $post_data[ 'add_new_cookie_select' ] ) );
				}
			}

			$this->renderSuccessMessage();
		}
	}

	private function updateCookieHandle() {
		if ( isset( $_POST[ 'cs_update_action' ] ) && $_POST[ 'cs_update_action' ] == 'update_cookie_form' ) {

			$post_data = array();
			array_walk( $_POST[ 'cs' ][ $this->plugin_name ], function( &$value, $key ) use ( &$post_data ) {
				$post_data[ sanitize_text_field( $key ) ] = sanitize_text_field( $value );
			} );

			$post_id    = $post_data[ 'cs_cookie_id' ];
			$script_cat = isset( $_POST[ 'cs_' . $post_id . '_cookie_cat' ] ) ? sanitize_text_field( $_POST[ 'cs_' . $post_id . '_cookie_cat' ] ) : null;
			$category   = get_term_by( 'slug', $script_cat, 'cs-cookies-category' );
			if ( $category && is_object( $category ) ) {
				$category_id = $category->term_id;
				wp_set_object_terms( $post_id, $category_id, 'cs-cookies-category' );
				ConsentMagic()->updateOptions(  array( 'cs_' . $script_cat . '_' . $post_id . '_cookie_enable' => true ) );
				ConsentMagic()->updateOptions( array( 'cs_' . $post_id . '_cookie_cat' => $script_cat ) );
			}
			$template_data = array(
				'ID'         => $post_id,
				'meta_input' => array(
					'cs_default_cookie_desc' => isset( $_POST[ 'cs_default_cookie_desc' ] ) ? sanitize_text_field( $_POST[ 'cs_default_cookie_desc' ] ) : null,
				)
			);

			wp_update_post( $template_data );
			$this->renderSuccessMessage();
		}
	}
}

