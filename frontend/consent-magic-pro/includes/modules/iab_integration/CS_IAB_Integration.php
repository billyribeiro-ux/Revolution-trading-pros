<?php

namespace ConsentMagicPro;

if ( !defined( 'ABSPATH' ) ) {
	exit; // Exit if accessed directly.
}

class CS_IAB_Integration {

	private static $_instance = null;
	private        $vendor_list_url;
	private        $vendor_list_option;
	private        $vendor_list_option_cache;
	private        $additional_vendor_list_cache;
	private        $additional_vendor_list_url;

	private $legitimate_purposes;
	private $purpose_list_url;
	private $purpose_list_option;
	private $current_lang;
	private $current_iab_lang;
	private $settings_key;
	private $enable              = null;
	private $force_download_json = false;
	private $script_categories;
	private $json_path;

	/**
	 * $_instance CS_IAB_Integration
	 * @return CS_IAB_Integration|null
	 */
	public static function instance() {
		if ( is_null( self::$_instance ) ) {
			self::$_instance = new self();
		}

		return self::$_instance;
	}

	public function __construct() {

		include_once ABSPATH . 'wp-admin/includes/plugin.php';

		$this->purpose_list_url    = 'https://vendor-list.consensu.org/v3/purposes-%s.json';
		$this->purpose_list_option = 'cmpro_purpose_list_option';

		$this->vendor_list_url              = 'https://vendorlist.consensu.org/v3/vendor-list.json';
		$this->vendor_list_option           = 'cmpro_vendor_list_option';
		$this->vendor_list_option_cache     = 'cmpro_vendor_list_option_cache';
		$this->additional_vendor_list_cache = 'cmpro_additional_vendor_list_option_cache';
		$this->additional_vendor_list_url   = 'https://storage.googleapis.com/tcfac/additional-consent-providers.csv';
		$this->json_path                    = CMPRO_PLUGIN_PATH . 'includes/modules/iab_integration/json';
		$this->check_json_path();

		$this->settings_key = 'cmpro_iab_settings';

		$this->legitimate_purposes = array(
			2,
			7,
			8,
			9,
			10,
			11
		);

		$this->script_categories = array(
			'analytics'      => 7,
			'marketing'      => 4,
			'googlefonts'    => 1,
			'embedded_video' => 4,
			'default'        => 1
		);

		if ( ConsentMagic()->getOption( 'cs_enable_translations' ) == 1 ) {
			$this->current_lang       = get_locale();
			$cs_user_default_language = ConsentMagic()->getOption( 'cs_user_default_language' );
			$cs_language_availability = ConsentMagic()->getOption( 'cs_language_availability' );
			if ( isset( $cs_language_availability[ $this->current_lang ] )
			     && $cs_language_availability[ $this->current_lang ] == 0 ) {
				$this->current_lang = $cs_user_default_language;
			}
			$this->current_lang = preg_replace( '/_.*$/', '', strtolower( $this->current_lang ) );
		} else {
			$this->current_lang = 'en';
		}

		$this->current_iab_lang = $this->get_language_code();
		if ( !file_exists( $this->json_path . '/vendor_list_' . $this->current_iab_lang . '.json' ) ) {
			$this->force_download_json = true;
		}
	}

	public function enabled() {

		if ( $this->enable === null ) {

			$allowed_actions = array(
				'preview_list_show',
				'cs_preview_show'
			);
			$enable          = null;

			if ( isset( $_POST[ 'action' ] )
			     && in_array( sanitize_text_field( $_POST[ 'action' ] ), $allowed_actions )
			     && isset( $_POST[ 'cs_rule_id' ] ) ) {

				if ( isset( $_POST[ 'cs_type' ] ) && $_POST[ 'cs_type' ] == 'iab' ) {
					$enable = 1;
				}
				$active_rule_id = sanitize_text_field( $_POST[ 'cs_rule_id' ] );
			} else {
				$active_rule_id = ConsentMagic()->get_active_rule_id();
			}

			if ( $enable === null ) {
				$cs_type = get_post_meta( $active_rule_id, '_cs_type', true );
				if ( $cs_type == 'iab' ) {
					$enable = 1;
				}
			}

		} else {
			$enable = $this->enable;
		}

		return $enable;
	}

	public function get_vendor_list() {
		static $cmpro_vendor_list_cache = null;
		if ( $cmpro_vendor_list_cache !== null ) {
			return $cmpro_vendor_list_cache;
		}

		$now  = strtotime( 'now' );
		$file = $this->json_path . '/vendor_list_en.json';
		$list = null;

		if ( file_exists( $file ) && is_readable( $file ) ) {
			$max_size = 10 * 1024 * 1024; // 10 MB
			$size     = @filesize( $file );
			if ( $size !== false && $size > $max_size ) {
				@unlink( $file );
			} else {
				$contents = file_get_contents( $file );
				$decoded  = json_decode( $contents );
				if ( json_last_error() === JSON_ERROR_NONE && !empty( $decoded ) ) {
					$list = $decoded;
				} else {
					$list = null;
				}
			}
		}

		$cached = ConsentMagic()->getOption( $this->vendor_list_option_cache );
		if ( !$list || !$cached || $now > $cached || $this->force_download_json ) {
			$request = $this->execute_query( $this->vendor_list_url );
			if ( !empty( $request ) ) {
				$list = json_decode( $request );

				$vendors = (array) $list->vendors;
				usort( $vendors, function( $vendor1, $vendor2 ) {
					if ( $vendor1->name == $vendor2->name ) {
						return 0;
					}

					return ( $vendor1->name > $vendor2->name ) ? 1 : -1;
				} );

				$list->vendors = new \stdClass();
				foreach ( $vendors as $vendor ) {
					$list->vendors->{$vendor->id} = $vendor;
				}

				ConsentMagic()->updateOptions( array( $this->vendor_list_option_cache => strtotime( '+7 days ' ) ) );
				file_put_contents( $this->json_path . '/vendor_list_en.json', json_encode( $list ), LOCK_EX );
			} else {
				ConsentMagic()->updateOptions( array( $this->vendor_list_option_cache => strtotime( '+1 hour ' ) ) );
			}
		}

		if ( !empty( $list ) ) {
			if ( !isset( $list->additionalVendorListVersion ) ) {
				$list->additionalVendorListVersion = 1;
			}

			//additional vendors
			$cached = ConsentMagic()->getOption( $this->additional_vendor_list_cache );
			if ( empty( $list->additional_vendors )
			     || !$cached
			     || $now > $cached
			     || $this->force_download_json ) {

				$ch = curl_init();
				curl_setopt( $ch, CURLOPT_URL, $this->additional_vendor_list_url );
				curl_setopt( $ch, CURLOPT_RETURNTRANSFER, 1 );
				$fileContents = curl_exec( $ch );
				curl_close( $ch );

				$handle = fopen( 'php://temp', 'r+' );
				fwrite( $handle, $fileContents );
				rewind( $handle );

				if ( $handle !== false ) {
					$row                = 0;
					$additional_vendors = array();
					while ( ( $data = fgetcsv( $handle, 10000, ",", '"', '' ) ) !== false ) {
						if ( $row !== 0 ) {
							$additional_vendor                 = new \stdClass;
							$additional_vendor->id             = $data[ 0 ] ?? '';
							$additional_vendor->name           = $data[ 1 ] ?? '';
							$additional_vendor->privacy_policy = $data[ 2 ] ?? '';
							$additional_vendors[]              = $additional_vendor;
						}
						$row++;
					}

					if ( !empty( $additional_vendors ) ) {
						usort( $additional_vendors, function( $vendor1, $vendor2 ) {
							if ( $vendor1->name == $vendor2->name ) {
								return 0;
							}

							return ( $vendor1->name > $vendor2->name ) ? 1 : -1;
						} );
						$list->additional_vendors = new \stdClass();
						foreach ( $additional_vendors as $vendor ) {
							$list->additional_vendors->{$vendor->id} = $vendor;
						}

						$list->additionalVendorListVersion++;

						ConsentMagic()->updateOptions( array( $this->additional_vendor_list_cache => strtotime( '+7 days ' ) ) );
						file_put_contents( $this->json_path . '/vendor_list_en.json', json_encode( $list ), LOCK_EX );
					} else {
						ConsentMagic()->updateOptions( array( $this->additional_vendor_list_cache => strtotime( '+1 hour ' ) ) );
					}
				} else {
					ConsentMagic()->updateOptions( array( $this->additional_vendor_list_cache => strtotime( '+1 hour ' ) ) );
				}
			}

			if ( $this->current_lang != 'en' && $this->current_iab_lang ) {

				$purposes_by_lang = '';
				$download_langs   = ConsentMagic()->getOption( 'cs_iab_download_langs' );
				$option           = $this->purpose_list_option . '_' . $this->current_iab_lang;
				$cached           = ConsentMagic()->getOption( $option . '_cache' );
				$update_json      = false;

				if ( !in_array( $this->current_iab_lang, $download_langs )
				     || !$cached
				     || $now > $cached
				     || $this->force_download_json ) {

					$request = $this->execute_query( sprintf( $this->purpose_list_url, $this->current_iab_lang ) );
					if ( !empty( $request ) ) {
						$request = json_decode( $request );
						ConsentMagic()->updateOptions( array( $option . '_cache' => strtotime( '+7 days ' ) ) );

						if ( !in_array( $this->current_iab_lang, $download_langs ) ) {
							ConsentMagic()->updateOptions( array( 'cs_iab_download_langs' => array_merge( $download_langs, array( $this->current_iab_lang ) ) ) );
						}
						$purposes_by_lang = $request;
						$update_json      = true;
					} else {
						ConsentMagic()->updateOptions( array( $option . '_cache' => strtotime( '+1 hour ' ) ) );
					}
				} else {
					$file = $this->json_path . '/vendor_list_' . $this->current_iab_lang . '.json';
					if ( file_exists( $file ) && is_readable( $file ) ) {
						$purposes_by_lang = file_get_contents( $file );
						$purposes_by_lang = json_decode( $purposes_by_lang );
					} else {
						$purposes_by_lang = null;
					}
				}

				if ( !empty( $purposes_by_lang ) ) {
					$list->purposes        = $purposes_by_lang->purposes;
					$list->specialPurposes = $purposes_by_lang->specialPurposes;
					$list->features        = $purposes_by_lang->features;
					$list->specialFeatures = $purposes_by_lang->specialFeatures;
					$list->stacks          = $purposes_by_lang->stacks;
					$list->dataCategories  = $purposes_by_lang->dataCategories;

					if ( $update_json ) {
						file_put_contents( $this->json_path . '/vendor_list_' . $this->current_iab_lang
						                   . '.json', json_encode( $list ), LOCK_EX );
					}
				}
			}
		}

		if ( !empty( $list ) && is_object( $list ) ) {
			$list->cache_label = time();
		}

		$cmpro_vendor_list_cache = ( !empty( $list ) ) ? $list : array();

		return $cmpro_vendor_list_cache;
	}

	private function execute_query( $url ) {

		$request = wp_safe_remote_get( $url );
		if ( is_wp_error( $request ) || wp_remote_retrieve_response_code( $request ) != 200 ) {
			return '';
		}

		return wp_remote_retrieve_body( $request );
	}

	public function get_legitimate_purposes() {
		return $this->legitimate_purposes;
	}

	public function get_language_code() {

		$available_langs = array(
			'ar'    => 'ar',
			'bg'    => 'bg',
			'bs'    => 'bs',
			'ca'    => 'ca',
			'cs'    => 'cs',
			'da'    => 'da',
			'de'    => 'de',
			'el'    => 'el',
			'en'    => 'en',
			'es'    => 'es',
			'et'    => 'et',
			'eu'    => 'eu',
			'fi'    => 'fi',
			'fr'    => 'fr',
			'gl'    => 'gl',
			'he'    => 'he',
			'hr'    => 'hr',
			'hu'    => 'hu',
			'it'    => 'it',
			'ja'    => 'ja',
			'lt'    => 'lt',
			'lv'    => 'lv',
			'mt'    => 'mt',
			'nl'    => 'nl',
			'no'    => 'no',
			'pl'    => 'pl',
			'pt-br' => 'pt-br',
			'pt-pt' => 'pt-pt',
			'ro'    => 'ro',
			'ru'    => 'ru',
			'sk'    => 'sk',
			'sl'    => 'sl',
			'sr'    => 'sr-cyrl',
			'sv'    => 'sv',
			'tr'    => 'tr',
			'uk'    => 'uk',
			'zh'    => 'zh',
		);

		if ( array_key_exists( $this->current_lang, $available_langs ) ) {
			return $available_langs[ $this->current_lang ];
		}

		$lang = str_replace( '_', '-', $this->current_lang );
		if ( array_key_exists( $lang, $available_langs ) ) {
			return $available_langs[ $lang ];
		}

		$lang = preg_replace( '/_.*$/', '', $this->current_lang );
		if ( array_key_exists( $lang, $available_langs ) ) {
			return $available_langs[ $lang ];
		}

		return 'en';
	}

	public function get_current_iab_lang() {
		return $this->current_iab_lang;
	}

	/**
	 * Get IAB Settings
	 * @return array
	 */
	public function get_settings() {
		$settings = ConsentMagic()->getOption( $this->settings_key );

		if ( empty( $settings ) || $settings === 'null' ) {
			$settings = $this->load_default_settings();
		} else {
			$vendor_list = $this->get_vendor_list();

			if ( ( isset( $vendor_list->vendorListVersion )
			       && $vendor_list->vendorListVersion != $settings->vendorListVersion )
			     || !isset( $vendor_list->additionalVendorListVersion )
			     || $vendor_list->additionalVendorListVersion != $settings->additionalVendorListVersion ) {
				$settings = $this->update_vendor_list( $vendor_list, $settings );
			}
		}

		return $settings;
	}

	/**
	 * Update IAB Settings
	 * @param $settings
	 * @return void
	 */
	public function update_settings( $settings ) {
		if ( is_object( $settings ) ) {
			$settings = $this->update_count( $settings );
			ConsentMagic()->updateOptions( array( $this->settings_key => $settings ) );
		}
	}

	private function load_default_settings() {

		$vendor_list                = $this->get_vendor_list();
		$purposes_options           = new \stdClass();
		$special_purposes_options   = new \stdClass();
		$features_options           = new \stdClass();
		$vendors_options            = new \stdClass();
		$additional_vendors_options = new \stdClass();
		$special_features_options   = new \stdClass();
		$settings                   = new \stdClass();

		if ( !empty( $vendor_list ) ) {
			if ( !empty( $vendor_list->purposes ) ) {
				foreach ( $vendor_list->purposes as $purpose ) {
					$purposes_options->{$purpose->id} = $this->get_default_value( 'purposes' );
				}
			}

			if ( !empty( $vendor_list->specialPurposes ) ) {
				foreach ( $vendor_list->specialPurposes as $purpose ) {
					$special_purposes_options->{$purpose->id} = $this->get_default_value( 'specialPurposes' );
				}
			}

			if ( !empty( $vendor_list->features ) ) {
				foreach ( $vendor_list->features as $feature ) {
					$features_options->{$feature->id} = $this->get_default_value( 'features' );
				}
			}

			if ( !empty( $vendor_list->specialFeatures ) ) {
				foreach ( $vendor_list->specialFeatures as $feature ) {
					$special_features_options->{$feature->id} = $this->get_default_value( 'specialFeatures' );
				}
			}

			if ( !empty( $vendor_list->vendors ) ) {
				foreach ( $vendor_list->vendors as $vendor ) {
					$vendors_options->{$vendor->id} = $this->get_default_value( 'vendors' );
				}
			}

			if ( !empty( $vendor_list->additional_vendors ) ) {
				foreach ( $vendor_list->additional_vendors as $vendor ) {
					$additional_vendors_options->{$vendor->id} = $this->get_default_value( 'vendors' );
				}
			}
		}

		$settings->vendorListVersion           = $vendor_list->vendorListVersion;
		$settings->additionalVendorListVersion = 0;
		$settings->purposes                    = $purposes_options;
		$settings->specialPurposes             = $special_purposes_options;
		$settings->features                    = $features_options;
		$settings->specialFeatures             = $special_features_options;
		$settings->vendors                     = $vendors_options;
		$settings->additional_vendors          = $additional_vendors_options;

		$settings->purposes_count            = 0;
		$settings->special_purposes_count    = 0;
		$settings->features_count            = 0;
		$settings->special_features_count    = 0;
		$settings->vendors_count             = 0;
		$settings->additional_vendors_count  = 0;
		$settings->quantity                  = new \stdClass;
		$settings->quantity->purposes        = new \stdClass;
		$settings->quantity->specialFeatures = new \stdClass;

		$this->update_settings( $settings );

		return $settings;
	}

	public function set_default_option( $type, $value ) {

		$settings = $this->get_settings();

		switch ( $type ) {
			case 'purposes':
				$settings->purposes->{$value} = $this->get_default_value( $type );
				break;
			case 'specialFeatures' :
				$settings->specialFeatures->{$value} = $this->get_default_value( $type );
				break;
			case 'vendors' :
				$settings->vendors->{$value} = $this->get_default_value( $type );
				break;
			case 'specialPurposes' :
				$settings->specialPurposes->{$value} = $this->get_default_value( $type );
				break;
			case 'features' :
				$settings->features->{$value} = $this->get_default_value( $type );
				break;
			default:
				break;
		}

		$this->update_settings( $settings );

		return $settings;
	}

	public function get_default_value( $type ) {

		$default_value = '';

		switch ( $type ) {
			case 'purposes':
			case 'specialFeatures' :
			case 'specialPurposes' :
			case 'features' :
				$default_value = 1;
				break;

			case 'vendors' :
				$default_value = 0;
				break;

			default:
				break;
		}

		return $default_value;
	}

	private function update_count( $settings ) {
		$vendor_list = $this->get_vendor_list();
		if ( !empty( $vendor_list ) && !empty( $settings ) ) {

			if ( !empty( $vendor_list->specialPurposes ) && !empty( $settings->specialPurposes ) ) {
				$count = 0;
				foreach ( $settings->specialPurposes as $key => $purpose ) {
					if ( isset( $vendor_list->specialPurposes->{$key} ) && $purpose ) {
						$count++;
					}
				}
				$settings->special_purposes_count = $count;
			}

			if ( !empty( $vendor_list->features ) && !empty( $settings->features ) ) {
				$count = 0;
				foreach ( $settings->features as $key => $feature ) {
					if ( isset( $vendor_list->features->{$key} ) && $feature ) {
						$count++;
					}
				}
				$settings->features_count = $count;
			}

			if ( !empty( $vendor_list->specialFeatures ) && !empty( $settings->specialFeatures ) ) {
				$count = 0;
				foreach ( $settings->specialFeatures as $key => $feature ) {
					if ( isset( $vendor_list->specialFeatures->{$key} ) && $feature ) {
						$count++;
					}
				}
				$settings->special_features_count = $count;
			}

			if ( !empty( $vendor_list->vendors ) && !empty( $settings->vendors ) ) {
				$count             = 0;
				$show_only_vendors = ConsentMagic()->getOption( 'cs_iab_show_only_vendors' );
				foreach ( $settings->vendors as $key => $vendor ) {
					if ( isset( $vendor_list->vendors->{$key} ) && ( !$show_only_vendors || $vendor ) ) {

						foreach ( $vendor_list->purposes as $k => &$purpose ) {
							if ( !isset( $settings->quantity->purposes->{$k} ) || $count === 0 ) {
								$settings->quantity->purposes->{$k} = 0;
							}
							if ( in_array( $purpose->id, $vendor_list->vendors->{$key}->purposes )
							     || in_array( $purpose->id, $vendor_list->vendors->{$key}->flexiblePurposes )
							     || in_array( $purpose->id, $vendor_list->vendors->{$key}->legIntPurposes ) ) {
								$settings->quantity->purposes->{$k}++;
							}
						}
						unset( $purpose );

						foreach ( $vendor_list->specialFeatures as $k => &$feature ) {
							if ( !isset( $settings->quantity->specialFeatures->{$k} ) || $count === 0 ) {
								$settings->quantity->specialFeatures->{$k} = 0;
							}
							if ( in_array( $feature->id, $vendor_list->vendors->{$key}->specialFeatures ) ) {
								$settings->quantity->specialFeatures->{$k}++;
							}
						}
						unset( $feature );

						$count++;
					}
				}
				$settings->vendors_count = $count;
			}

			if ( !empty( $vendor_list->additional_vendors ) && !empty( $settings->additional_vendors ) ) {
				$count             = 0;
				$show_only_vendors = ConsentMagic()->getOption( 'cs_iab_show_only_additional_vendors' );
				foreach ( $settings->additional_vendors as $key => $vendor ) {
					if ( isset( $vendor_list->additional_vendors->{$key} ) && ( !$show_only_vendors || $vendor ) ) {
						$count++;
					}
				}
				$settings->additional_vendors_count = $count;
			}

			if ( !empty( $vendor_list->purposes ) && !empty( $settings->purposes ) ) {
				$count = 0;

				foreach ( $settings->purposes as $key => $purpose ) {
					if ( isset( $vendor_list->purposes->{$key} ) && $purpose ) {
						$count++;
					}
				}
				$settings->purposes_count = $count;
			}
		}

		return $settings;
	}

	private function update_vendor_list( $vendor_list, $settings ) {
		if ( !empty( $vendor_list ) ) {
			if ( !empty( $vendor_list->purposes ) ) {
				$new_purposes = new \stdClass();
				foreach ( $vendor_list->purposes as $purpose ) {
					$new_purposes->{$purpose->id} = $settings->purposes->{$purpose->id} ??
					                                $this->get_default_value( 'purposes' );
				}
				$settings->purposes = $new_purposes;
			}

			if ( !empty( $vendor_list->specialPurposes ) ) {
				$new_special_purposes = new \stdClass();
				foreach ( $vendor_list->specialPurposes as $purpose ) {
					$new_special_purposes->{$purpose->id} = $settings->specialPurposes->{$purpose->id} ??
					                                        $this->get_default_value( 'specialPurposes' );
				}
				$settings->specialPurposes = $new_special_purposes;
			}

			if ( !empty( $vendor_list->features ) ) {
				$new_features = new \stdClass();
				foreach ( $vendor_list->features as $feature ) {
					$new_features->{$feature->id} = $settings->features->{$feature->id} ??
					                                $this->get_default_value( 'specialPurposes' );
				}
				$settings->features = $new_features;
			}

			if ( !empty( $vendor_list->specialFeatures ) ) {
				$new_special_features = new \stdClass();
				foreach ( $vendor_list->specialFeatures as $feature ) {
					$new_special_features->{$feature->id} = $settings->specialFeatures->{$feature->id} ??
					                                        $this->get_default_value( 'specialFeatures' );
				}
				$settings->specialFeatures = $new_special_features;
			}

			if ( !empty( $vendor_list->vendors ) ) {
				$new_vendors = new \stdClass();
				foreach ( $vendor_list->vendors as $vendor ) {
					$new_vendors->{$vendor->id} = $settings->vendors->{$vendor->id} ??
					                              $this->get_default_value( 'vendors' );
				}
				$settings->vendors = $new_vendors;
			}

			if ( !empty( $vendor_list->additional_vendors ) ) {
				$new_vendors = new \stdClass();
				foreach ( $vendor_list->additional_vendors as $vendor ) {
					$new_vendors->{$vendor->id} = $settings->additional_vendors->{$vendor->id} ??
					                              $this->get_default_value( 'vendors' );
				}
				$settings->additional_vendors = $new_vendors;
			}

			$settings->vendorListVersion           = $vendor_list->vendorListVersion;
			$settings->additionalVendorListVersion = $vendor_list->additionalVendorListVersion;
			$this->update_settings( $settings );
		}

		return $settings;
	}

	/**
	 * Get active vendors
	 * @return array
	 */
	public function get_active_vendors() {
		$settings = $this->get_settings();
		$vendors  = array();
		if ( !empty( $settings->vendors ) ) {
			if ( ConsentMagic()->getOption( 'cs_iab_show_only_vendors' ) ) {
				$vendors = array_filter( (array) $settings->vendors, function( $item ) {
					return ! !$item;
				} );
			} else {
				$vendors = (array) $settings->vendors;
			}

			$vendors = array_keys( $vendors );
		}

		return $vendors;
	}

	/**
	 * Get active additional vendors
	 * @return array|int[]|string[]
	 */
	public function get_active_additional_vendors() {
		$settings = $this->get_settings();
		$vendors  = array();
		if ( !empty( $settings->additional_vendors ) ) {
			if ( ConsentMagic()->getOption( 'cs_iab_show_only_additional_vendors' ) ) {
				$vendors = array_filter( (array) $settings->additional_vendors, function( $item ) {
					return ! !$item;
				} );
			} else {
				$vendors = (array) $settings->additional_vendors;
			}

			$vendors = array_keys( $vendors );
		}

		return $vendors;
	}

	/**
	 * Get vendor list version
	 * @return string
	 */
	public function get_vendor_list_version() {
		$vendor_list = $this->get_vendor_list();

		return $vendor_list->vendorListVersion;
	}

	/**
	 * Get default script categories
	 * @return array
	 */
	public function get_default_script_categories() {
		return $this->script_categories;
	}

	/**
	 * Check if purpose is enabled by category
	 * @param $category
	 * @return bool
	 */
	public function check_enabled_purpose_by_category( $category ) {
		$enabled      = false;
		$iab_settings = $this->get_settings();

		$iab_cat = get_term_meta( $category, '_cs_iab_cat', true );
		if ( !empty( $iab_cat ) && $iab_cat != 0 ) {
			if ( isset( $iab_settings->purposes->{$iab_cat} ) && $iab_settings->purposes->{$iab_cat} != 0 ) {
				$enabled = true;
			}
		}

		return $enabled;
	}

	private function check_json_path() {
		if ( !is_dir( $this->json_path ) ) {
			mkdir( $this->json_path, 0755, true );
		}
	}
}

/**
 * @return CS_IAB_Integration|null
 */
function CS_IAB_Integration(): ?CS_IAB_Integration {
	return CS_IAB_Integration::instance();
}