<?php

namespace ConsentMagicPro;

abstract class CS_Options {

	public string $plugin_name;

	protected string $version;

	protected array $values = array();

	protected array $values_design = array();

	protected string $option_key = '';

	protected string $option_key_design = '';

	protected string $defaults_json_path;

	protected string $defaults_json_path_design;

	protected array $options = array();

	protected array $options_design = array();

	protected array $defaults = array();

	protected array $defaults_design = array();

	protected string $translations_path = '';

	public array $cs_options = array();

	private string $options_table_name = 'cs_options';

	private string $translations_table_name = 'cs_translations';

	private array $cache_lang_option = array();
	private array $cache_lang_term   = array();

	/**
	 * Define the core functionality of the plugin.
	 */
	public function __construct() {

		if ( defined( 'CMPRO_LATEST_VERSION_NUMBER' ) ) {
			$this->version = CMPRO_LATEST_VERSION_NUMBER;
		} else {
			$this->version = '1.0.0';
		}
		$this->plugin_name       = 'consent-magic';
		$this->option_key        = 'cs' . $this->plugin_name;
		$this->option_key_design = CMPRO_SETTINGS_DESIGN_FIELD;

		$this->loadOptions();

		$this->maybeMigrate();

		$this->maybeLoad();
		$this->maybeLoadDesign();
	}

	/**
	 * Load options schemas and defaults, initialize paths
	 * @return void
	 */
	private function loadOptions(): void {
		// initialize options
		$this->locateOptions(
			CMPRO_PLUGIN_PATH . '/includes/options_fields.json',
			CMPRO_PLUGIN_PATH . '/includes/options_defaults.json'
		);
		// initialize options
		$this->locateOptionsDesign(
			CMPRO_PLUGIN_PATH . '/includes/design_options_fields.json',
			CMPRO_PLUGIN_PATH . '/includes/design_options_default.json'
		);

		$this->translations_path = CMPRO_PLUGIN_PATH . '/includes/translations.json';
	}

	/**
	 * The name of the plugin used to uniquely identify it within the context of
	 * WordPress and to define internationalization functionality.
	 * @return string
	 */
	public function get_plugin_name(): string {
		return $this->plugin_name;
	}

	/**
	 * Retrieve the version number of the plugin.
	 * @return string
	 */
	public function get_version(): string {
		return $this->version;
	}

	/**
	 * Get option value (with design fallback)
	 * @param string $key
	 * @return mixed
	 */
	public function getOption( $key ) {

		// get option default if unset
		if ( !isset( $this->values[ $key ] ) ) {
			$this->values[ $key ] = $this->defaults[ $key ] ?? null;
		}

		if ( isset( $this->values[ $key ] ) ) {
			return $this->values[ $key ];
		} else {
			if ( !isset( $this->values_design[ $key ] ) ) {
				$this->values_design[ $key ] = $this->defaults_design[ $key ] ?? null;
			}

			return $this->values_design[ $key ];
		}
	}

	/**
	 * Serialize values for storage when needed
	 * @param mixed $value
	 * @return string
	 */
	private function serializeValue( $value ): string {
		if ( is_bool( $value ) || is_null( $value ) || is_array( $value ) || is_object( $value ) ) {
			return serialize( $value );
		}

		return $value;
	}

	/**
	 * Check if a string looks like a PHP serialized value
	 * @param string $value
	 * @return bool
	 */
	private function isSerialized( string $value ): bool {
		$value = trim( $value );
		if ( $value === '' ) {
			return false;
		}
		if ( $value === 'N;' ) {
			return true;
		} // null

		$tokens = [ 'b', 'i', 'd', 's', 'a', 'O' ];
		if ( !in_array( $value[ 0 ], $tokens ) ) {
			return false;
		}

		$lastChar = substr( $value, -1 );
		if ( $lastChar !== ';' && $lastChar !== '}' ) {
			return false;
		}

		return true;
	}

	/**
	 * Unserialize or JSON-decode stored value
	 * @param string $value
	 * @param bool   $array
	 * @return mixed
	 */
	private function unserializeValue( $value, $array = true ) {
		// Try to unserialize first (for new format)
		if ( $this->isSerialized( $value ) ) {
			return unserialize( $value );
		}

		// Fallback to JSON decode for backward compatibility
		$decoded = json_decode( $value, $array );

		// If JSON decode was successful and it's not a string that looks like JSON
		if ( json_last_error() === JSON_ERROR_NONE && $value !== $decoded ) {
			return $decoded;
		}

		return $value;
	}

	/**
	 * Get translation value for an option/meta by language
	 * @param string $key
	 * @param string $lang
	 * @param string $type option|term|meta
	 * @param int    $meta_id
	 * @return string|null
	 */
	public function getLangOption( $key, $lang, $type = 'option', $meta_id = 0 ): ?string {
		// Build cache key and return if present (use array_key_exists to allow null/empty caching)
		$cache_key = $type . '|' . $key . '|' . $lang . '|' . (int) $meta_id;
		if ( array_key_exists( $cache_key, $this->cache_lang_option ) ) {
			return $this->cache_lang_option[ $cache_key ];
		}

		global $wpdb;
		$table_name = $wpdb->prefix . $this->translations_table_name;

		if ( $type === 'meta' ) {
			$where = $wpdb->prepare( " AND `meta` = %d AND `meta_id` = %d", 1, $meta_id );
		} else {
			$where = " AND `meta` = 0 AND `meta_id` = 0";
		}

		$sql  = $wpdb->prepare(
			"
					SELECT `value`
						FROM $table_name
						WHERE `option` = %s
							AND `language` = %s
							$where
				",
			$key,
			$lang
		);
		$data = $wpdb->get_var( $sql );

		if ( empty( $data ) ) {
			$lang_gen = get_general_language( $lang );
			if ( $lang_gen !== $lang ) {
				$sql  = $wpdb->prepare(
					"
					SELECT `value`
						FROM $table_name
						WHERE `option` = %s
							AND `language` = %s
							$where
				",
					$key,
					$lang_gen
				);
				$data = $wpdb->get_var( $sql );
			}
		}

		$this->cache_lang_option[ $cache_key ] = $data;

		return $data;
	}

	/**
	 * Get category translation (value and category_name) for language
	 * @param string $key
	 * @param string $lang
	 * @return array|null
	 */
	public function getLangOptionTerm( $key, $lang ): ?array {
		$cache_key = $key . '|' . $lang;
		if ( array_key_exists( $cache_key, $this->cache_lang_term ) ) {
			return $this->cache_lang_term[ $cache_key ];
		}
		global $wpdb;
		$table_name = $wpdb->prefix . $this->translations_table_name;

		$sql = $wpdb->prepare(
			"
					SELECT `value`, `category_name`
						FROM $table_name
						WHERE `option` = %s
							AND `language` = %s
							AND `category` = 1
						  	",
			$key,
			$lang
		);

		$data = $wpdb->get_row( $sql, ARRAY_A );

		if ( empty( $data ) ) {
			$lang_gen = get_general_language( $lang );
			if ( $lang_gen !== $lang ) {
				$sql  = $wpdb->prepare(
					"
					SELECT `value`, `category_name`
						FROM $table_name
						WHERE `option` = %s
							AND `language` = %s
							AND `category` = 1
						  	",
					$key,
					$lang_gen
				);
				$data = $wpdb->get_row( $sql, ARRAY_A );
			}
		}

		$this->cache_lang_term[ $cache_key ] = $data;

		return $data;
	}

	/**
	 * Load non-design options from DB into memory
	 * @param bool $force
	 * @return void
	 */
	private function maybeLoad( $force = false ): void {
		if ( $force || empty( $this->values ) ) {
			global $wpdb;
			$table_name = $wpdb->prefix . $this->options_table_name;
			$sql        = "
					SELECT `name`, `value`
						FROM $table_name
						WHERE `design` = 0";
			$values     = $wpdb->get_results( $sql, ARRAY_A );

			if ( !empty( $values ) ) {
				foreach ( $values as $value ) {
					$this->values[ $value[ 'name' ] ] = $this->unserializeValue( $value[ 'value' ] );
				}
			}
		}

		// if there are no settings defined, use default values
		if ( !is_array( $this->values ) ) {
			$this->values = $this->defaults;
		}
	}

	/**
	 * Load options field schema and defaults from JSON
	 * @param string $fields
	 * @param string $defaults
	 * @return void
	 */
	private function locateOptions( $fields, $defaults ): void {

		$this->loadJSON( $fields, false );
		$this->loadJSON( $defaults, true );

		$this->defaults_json_path = $defaults;
	}

	/**
	 * Reset non-design options to defaults from JSON
	 * @return void
	 */
	public function resetToDefaults(): void {

		if ( !file_exists( $this->defaults_json_path ) ) {
			return;
		}

		$content = file_get_contents( $this->defaults_json_path );
		$values  = json_decode( $content, true );

		global $wpdb;
		$table_name = $wpdb->prefix . $this->options_table_name;
		$sql        = "
			DELETE FROM $table_name
			       WHERE `design`=0";
		$wpdb->query( $sql );

		$this->updateOptions( $values );
	}

	/**
	 * Load options fields or defaults JSON
	 * @param string $file
	 * @param bool   $is_defaults
	 * @return void|null
	 */
	private function loadJSON( $file, $is_defaults ) {

		if ( !file_exists( $file ) ) {
			return;
		}

		$content = file_get_contents( $file );
		$values  = json_decode( $content, true );

		if ( null === $values ) {
			return;
		}

		if ( $is_defaults ) {
			$this->defaults = $values;
		} else {
			$this->options = $values;
		}
	}

	/**
	 * Persist non-design options into DB
	 * @param array $options
	 * @return void
	 */
	public function updateOptions( $options = array() ): void {

		global $wpdb;
		$table_name = $wpdb->prefix . $this->options_table_name;

		foreach ( $options as $key => $value ) {
			$this->values [ $key ] = $value;
			$sql                   = $wpdb->prepare(
				"
					INSERT INTO $table_name (name, value, updated_at)
					    VALUES (%s, %s, NOW())
					    ON DUPLICATE KEY UPDATE value = VALUES(value), updated_at = NOW()",
				$key,
				$this->serializeValue( $value )
			);

			$wpdb->query( $sql );
		}
	}

	/**
	 * Upsert a translation value for given language
	 * @param string $option_name
	 * @param mixed  $value
	 * @param string $language
	 * @param string $type option|term|meta
	 * @param int    $meta_id
	 * @return void
	 */
	public function updateLangOptions( $option_name, $value, $language, $type = 'option', $meta_id = 0 ): void {

		global $wpdb;
		$table_name = $wpdb->prefix . $this->translations_table_name;

		if ( $type === 'option' ) {
			$sql = $wpdb->prepare(
				"
					INSERT INTO $table_name (`option`, `language`, `value`, `updated_at`)
					     VALUES (%s, %s, %s, NOW())
					     ON DUPLICATE KEY UPDATE
					         `value` = VALUES(`value`),
					         `updated_at` = VALUES(`updated_at`)",
				$option_name,
				$language,
				$value
			);
			$wpdb->query( $sql );
		} elseif ( $type === 'term' ) {
			$sql = $wpdb->prepare(
				"
					INSERT INTO $table_name (`option`, `language`, `value`, `category`, `category_name`, `meta_id`, `updated_at`)
					     VALUES (%s, %s, %s, 1, %s, 0, NOW())
					     ON DUPLICATE KEY UPDATE
					         `value` = VALUES(`value`),
					         `category_name` = VALUES(`category_name`),
					         `updated_at` = VALUES(`updated_at`)",
				$option_name,
				$language,
				$value[ 'descr' ],
				$value[ 'name' ]
			);
			$wpdb->query( $sql );
		} elseif ( $type === 'meta' ) {
			$sql = $wpdb->prepare(
				"
					INSERT INTO $table_name (`option`, `language`, `value`, `meta`, `meta_id`, `updated_at`)
					     VALUES (%s, %s, %s, 1, %d, NOW())
					     ON DUPLICATE KEY UPDATE
					         `value` = VALUES(`value`),
					         `updated_at` = VALUES(`updated_at`)",
				$option_name,
				$language,
				$value,
				$meta_id
			);
			$wpdb->query( $sql );
		}

		// Invalidate translation caches after update
		$this->cache_lang_option = array();
		$this->cache_lang_term   = array();
	}

	/**
	 * Clear translation values for an option
	 * @param string $option_name
	 * @param string $type option|term|meta
	 * @param int    $meta_id
	 * @return void
	 */
	public function clearLangOptions( $option_name, $type = 'option', $meta_id = 0 ): void {
		global $wpdb;
		$table_name = $wpdb->prefix . $this->translations_table_name;

		if ( $type === 'meta' ) {
			$sql = $wpdb->prepare(
				"
				DELETE FROM $table_name
				WHERE `option` = %s
				AND `meta` = 1
				AND `meta_id` = %d",
				$option_name,
				$meta_id
			);
			$wpdb->query( $sql );
		} elseif ( $type === 'term' ) {
			$sql = $wpdb->prepare(
				"
				DELETE FROM $table_name
				WHERE `option` = %s
				AND `category` = 1",
				$option_name
			);
			$wpdb->query( $sql );
		} elseif ( $type === 'option' ) {
			$sql = $wpdb->prepare(
				"
				DELETE FROM $table_name
				WHERE `option` = %s
					AND `category` = 0
				    AND `meta` = 0
					AND `meta_id` = 0
					AND `category` = 0",
				$option_name
			);
			$wpdb->query( $sql );
		}

		// Invalidate translation caches after delete
		$this->cache_lang_option = array();
		$this->cache_lang_term   = array();
	}

	/**
	 * Delete an option from DB storage
	 * @param string $key_input
	 * @return void
	 */
	public function deleteOption( $key_input ): void {
		global $wpdb;
		$table_name = $wpdb->prefix . $this->options_table_name;

		if ( isset( $this->options[ $key_input ] ) ) {
			unset( $this->values[ $key_input ] );
			$sql = $wpdb->prepare(
				"
				DELETE FROM $table_name
				WHERE `name` = %s",
				$key_input
			);
			$wpdb->query( $sql );
		}
	}

	/**
	 * Read default non-design options from JSON
	 * @return array|null
	 */
	public function getCSOptions(): ?array {

		if ( !file_exists( $this->defaults_json_path ) ) {
			return null;
		}

		$content = file_get_contents( $this->defaults_json_path );
		$values  = json_decode( $content, true );

		return $values;
	}

	/**
	 * Read default design options from JSON
	 * @return array|null
	 */
	public function getCSOptionsDesign(): ?array {

		if ( !file_exists( $this->defaults_json_path_design ) ) {
			return null;
		}

		$content = file_get_contents( $this->defaults_json_path_design );
		$values  = json_decode( $content, true );

		return $values;
	}

	/**
	 * Load design options from DB into memory
	 * @param bool $force
	 * @return void
	 */
	private function maybeLoadDesign( $force = false ): void {
		if ( $force || empty( $this->values_design ) ) {
			global $wpdb;
			$table_name = $wpdb->prefix . $this->options_table_name;
			$sql        = "
					SELECT `name`, `value`
					FROM $table_name
					WHERE `design` = 1";
			$values     = $wpdb->get_results( $sql, ARRAY_A );
			if ( !empty( $values ) ) {
				$this->values_design = array_column( $values, 'value', 'name' );
			}
		}

		// if there are no settings defined, use default values
		if ( !is_array( $this->values_design ) ) {
			$this->values_design = $this->defaults_design;
		}
	}

	/**
	 * Load design field schema and defaults from JSON
	 * @param string $fields
	 * @param string $defaults
	 * @return void
	 */
	public function locateOptionsDesign( $fields, $defaults ): void {

		$this->loadJSONDesign( $fields, false );
		$this->loadJSONDesign( $defaults, true );

		$this->defaults_json_path_design = $defaults;
	}

	/**
	 * Reset design options to defaults from JSON
	 * @return void
	 */
	public function resetToDefaultsDesign(): void {

		if ( !file_exists( $this->defaults_json_path_design ) ) {
			return;
		}

		$content = file_get_contents( $this->defaults_json_path_design );
		$values  = json_decode( $content, true );

		global $wpdb;
		$table_name = $wpdb->prefix . $this->options_table_name;
		$sql        = "
			DELETE FROM $table_name
			       WHERE `design`=1";
		$wpdb->query( $sql );

		$this->updateOptionsDesign( $values );
	}

	/**
	 * Load design JSON (fields or defaults)
	 * @param string $file
	 * @param bool   $is_defaults
	 * @return void|null
	 */
	private function loadJSONDesign( $file, $is_defaults ) {

		if ( !file_exists( $file ) ) {
			return;
		}

		$content = file_get_contents( $file );
		$values  = json_decode( $content, true );

		if ( null === $values ) {
			return;
		}

		if ( $is_defaults ) {
			$this->defaults_design = $values;
		} else {
			$this->options_design = $values;
		}
	}

	/**
	 * Persist design options into DB
	 * @param array $options
	 * @return void
	 */
	public function updateOptionsDesign( $options = array() ): void {

		global $wpdb;
		$table_name = $wpdb->prefix . $this->options_table_name;

		foreach ( $options as $key => $value ) {
			if ( isset( $this->options_design[ $key ] ) ) {
				$this->values_design[ $this->sanitize_text_field( $key ) ] = $this->sanitize_form_field_design(
					$key,
					$value
				);
				$sql                                                       = $wpdb->prepare(
					"
						INSERT INTO $table_name (name, value, design, updated_at)
						    VALUES (%s, %s, 1, NOW())
						    ON DUPLICATE KEY UPDATE value = VALUES(value), updated_at = NOW()",
					$key,
					$this->serializeValue( $value )
				);

				$wpdb->query( $sql );
			}
		}
	}

	/**
	 * Sanitize form field
	 * @param string $key   Field key
	 * @param array  $value Field value
	 * @return mixed
	 */
	private function sanitize_form_field( $key, $value ) {

		$type = $this->options[ $key ];

		// look for very specific sanitization filter
		$filter_name = "{$this->option_key}_settings_sanitize_{$key}_field";

		if ( has_filter( $filter_name ) ) {
			return apply_filters( $filter_name, $value );
		}

		// look for a sanitize_FIELDTYPE_field method
		if ( is_callable( array(
			$this,
			'sanitize_' . $type . '_field'
		) ) ) {
			return $this->{'sanitize_' . $type . '_field'}( $value );
		}

		// fallback to text
		return $this->sanitize_text_field( $value );
	}

	/**
	 * Sanitize design form field
	 * @param string $key
	 * @param mixed  $value
	 * @return mixed
	 */
	private function sanitize_form_field_design( $key, $value ) {

		$type = $this->options_design[ $key ];

		// look for very specific sanitization filter
		$filter_name = "{$this->option_key_design}_settings_sanitize_{$key}_field";
		if ( has_filter( $filter_name ) ) {
			return apply_filters( $filter_name, $value );
		}

		// look for a sanitize_FIELDTYPE_field method
		if ( is_callable( array(
			$this,
			'sanitize_' . $type . '_field'
		) ) ) {
			return $this->{'sanitize_' . $type . '_field'}( $value );
		}

		// fallback to text
		return $this->sanitize_text_field( $value );
	}

	/**
	 * Sanitize text or array field value
	 * @param mixed $value
	 * @return string|array
	 */
	public function sanitize_text_field( $value ) {

		$value = is_null( $value ) ? '' : $value;

		if ( is_array( $value ) ) {
			foreach ( $value as $key => $val ) {
				$value[ $key ] = wp_kses_post( trim( stripslashes( $val ) ) );
			}

			return $value;
		} else {
			return wp_kses_post( trim( stripslashes( $value ) ) );
		}
	}

	/**
	 * Sanitize array field value
	 * @param mixed $values
	 * @return array
	 */
	public function sanitize_array_field( $values ): array {

		$values    = is_array( $values ) ? $values : array();
		$sanitized = array();

		foreach ( $values as $key => $value ) {

			$new_value = $this->sanitize_text_field( $value );

			if ( isset( $new_value ) && $new_value !== '' ) {
				$sanitized[ $key ] = $new_value;
			}

		}

		return $sanitized;
	}

	/**
	 * Reload default translations from bundled JSON into DB
	 * @return void
	 */
	public function loadDefaultTranslations(): void {

		$translations = $this->getDefaultTranslations();

		if ( !empty( $translations ) ) {

			global $wpdb;
			$table_name = $wpdb->prefix . $this->translations_table_name;
			$sql        = "
				    DELETE FROM $table_name
				           WHERE `meta`=0";
			$wpdb->query( $sql );

			// Get existing language availability or initialize empty array
			$language_availability = $this->getOption( 'cs_language_availability' );
			if ( !is_array( $language_availability ) ) {
				$language_availability = array();
			}

			foreach ( $translations[ 'text' ] as $key => $option ) {
				foreach ( $option as $lang => $text ) {
					$wpdb->insert( $table_name, array(
						'option'     => $key,
						'language'   => $lang,
						'value'      => $text,
						'updated_at' => date( 'Y-m-d H:i:s' )
					) );
					$language_availability[ $lang ] = 1;
				}
			}

			foreach ( $translations[ 'categories' ] as $key => $category ) {
				foreach ( $category as $lang => $text ) {
					$wpdb->insert( $table_name, array(
						'option'        => $key,
						'language'      => $lang,
						'value'         => $text[ 'descr' ],
						'category'      => 1,
						'category_name' => $text[ 'name' ],
						'updated_at'    => date( 'Y-m-d H:i:s' )
					) );
					$language_availability[ $lang ] = 1;
				}
			}

			$this->updateOptions( array( 'cs_language_availability' => $language_availability ) );
		}
	}

	/**
	 * Get single default translation text
	 * @param string $option
	 * @param string $language
	 * @param bool   $category
	 * @return string|false
	 */
	public function getDefaultTranslation( $option, $language, $category = false ) {
		if ( !file_exists( $this->translations_path ) ) {
			return false;
		}

		$content      = file_get_contents( $this->translations_path );
		$translations = json_decode( $content, true );
		$text         = '';

		if ( $translations !== null ) {
			if ( $category ) {
				$text = $translations[ 'categories' ][ $option ][ $language ] ?? '';
			} else {
				$text = $translations[ 'text' ][ $option ][ $language ] ?? '';
			}
		}

		return $text;
	}

	/**
	 * Read all default translations from JSON
	 * @return array|false
	 */
	public function getDefaultTranslations() {
		if ( !file_exists( $this->translations_path ) ) {
			return false;
		}

		$content      = file_get_contents( $this->translations_path );
		$translations = json_decode( $content, true );
		$text         = '';

		if ( $translations !== null ) {
			$text = $translations;
		}

		return $text;
	}

	/**
	 * Insert default translations for keys not yet present in DB
	 * @return void
	 */
	public function loadTranslations(): void {

		$current_version = $this->getOption( 'cs_check_translations' );
		if ( $this->getOption( 'cs_check_translations' ) != CMPRO_LATEST_VERSION_NUMBER ) {

			$translations = $this->getDefaultTranslations();
			$update       = array();

			if ( version_compare( $current_version, '5.0.2.1', '<=' ) ) {
				$update = array(
					'*' => array(
						'he_IL' => '',
					)
				);

				$language_availability            = $this->getOption( 'cs_language_availability' );
				$language_availability[ 'he_IL' ] = 1;
				$this->updateOptions( array( 'cs_language_availability' => $language_availability ) );
			}

			if ( !empty( $update ) ) {
				foreach ( $update as $key => $locales ) {
					if ( $key === '*' ) {
						foreach ( $translations[ 'text' ] as $option => $translation ) {
							foreach ( $locales as $locale => $value ) {
								$current_value = $value !== '' ? $value : ( $translation[ $locale ] ?? '' );
								if ( !empty( $current_value ) ) {
									$this->updateLangOptions( $option, $current_value, $locale );
								}
							}
						}
						foreach ( $translations[ 'categories' ] as $option => $translation ) {
							foreach ( $locales as $locale => $value ) {
								$current_value = $value
								                 !== '' ? $value : ( ( isset( $translation[ $locale ][ 'descr' ] )
								                                       && $translation[ $locale ][ 'descr' ]
								                                          !== '' ) ? $translation[ $locale ] : '' );
								if ( !empty( $current_value ) ) {
									$this->updateLangOptions( $option, $current_value, $locale, 'term' );
								}
							}
						}
					} else {
						if ( isset( $translations[ 'text' ][ $key ] ) ) {
							foreach ( $locales as $locale => $value ) {
								$current_value = $value !== '' ? $value : ( $translations[ 'text' ][ $key ][ $locale ]
								                                            ?? '' );
								if ( !empty( $current_value ) ) {
									$this->updateLangOptions( $key, $current_value, $locale );
								}
							}
						}
						if ( isset( $translations[ 'categories' ][ $key ] ) ) {
							foreach ( $locales as $locale => $value ) {
								$current_value = $value
								                 !== '' ? $value : ( ( isset( $translations[ 'categories' ][ $key ][ $locale ][ 'descr' ] )
								                                       && $translations[ 'categories' ][ $key ][ $locale ][ 'descr' ]
								                                          !== '' ) ? $translations[ 'categories' ][ $key ][ $locale ] : '' );
								if ( !empty( $current_value ) ) {
									$this->updateLangOptions( $key, $current_value, $locale, 'term' );
								}
							}
						}
					}
				}
			}

			$this->updateOptions( array( 'cs_check_translations' => CMPRO_LATEST_VERSION_NUMBER ) );
		}
	}

	/**
	 * Get translations for an option/term/meta
	 * @param string $option
	 * @param string $type option|term|meta
	 * @param int    $meta_id
	 * @return array
	 */
	public function getTranslations( $option, $type = 'option', $meta_id = 0 ): array {

		global $wpdb;
		$table_name = $wpdb->prefix . $this->translations_table_name;

		if ( $type == 'term' ) {
			$sql = $wpdb->prepare(
				"
					SELECT *
						FROM $table_name
						WHERE `option` = %s
						  AND `category` = 1",
				$option
			);
		} elseif ( $type == 'meta' ) {
			$sql = $wpdb->prepare(
				"
					SELECT *
						FROM $table_name
						WHERE `option` = %s
						  AND `meta` = 1
						  AND `meta_id` = %d",
				$option,
				$meta_id
			);
		} else {
			$sql = $wpdb->prepare(
				"
					SELECT *
						FROM $table_name
						WHERE `option` = %s
						  AND `category` = 0
						  AND `meta` = 0",
				$option
			);
		}

		$translations = $wpdb->get_results( $sql, ARRAY_A );

		if ( !empty( $translations ) ) {
			if ( $type != 'term' ) {
				$translations = array_column( $translations, 'value', 'language' );
			}
		} else {
			$translations = array();
		}

		return $translations;
	}

	/**
	 * Check settings and perform migrations/defaults if needed
	 * @param object $plugin_admin
	 * @return void
	 */
	protected function cs_check_settings( $plugin_admin ): void {

		$current_version = $this->getOption( 'cs_check_settings' );
		if ( $current_version != CMPRO_LATEST_VERSION_NUMBER ) {

			//migrate to the only one setting of Google consent mode v2
			if ( version_compare( $this->version, '2.0.6.2', '<=' ) ) {
				if ( $this->getOption( 'cs_google_analytics_consent_mode' )
				     || $this->getOption( 'cs_google_ads_consent_mode' ) ) {

					$this->updateOptions( array( 'cs_google_consent_mode' => 1 ) );
					$this->deleteOption( 'cs_google_analytics_consent_mode' );
					$this->deleteOption( 'cs_google_ads_consent_mode' );
				}
			}

			//set Google consent mode v2 to true by default
			if ( version_compare( $current_version, '3.0.2', '<=' ) ) {
				$this->updateOptions( array( 'cs_google_consent_mode' => true ) );
			}

			//Force set admin permissions
			$admin = $this->getOption( 'cs_admin_permissions' );
			if ( empty( $admin ) || ( isset( $admin[ 0 ] ) && empty( $admin[ 0 ] ) ) ) {
				$this->updateOptions( array( 'cs_admin_permissions' => array( 'administrator' ) ) );
			}

			//Update geolocation
			if ( ( (int) $this->getOption( 'cs_geolocation' ) === 1
			       || (int) $this->getOption( 'cs_geo_activated' ) === 1 ) ) {
				if ( !defined( 'DISABLE_WP_CRON' ) || !DISABLE_WP_CRON ) {
					db_cron_update_one_time();
				} else {
					ConsentMagic()->loader->add_action( 'admin_init', $plugin_admin, 'cs_force_update_runner' );
				}
			}

			//Update scripts and cookies tables
			global $wpdb;
			$script_table = $wpdb->prefix . 'cs_scan_scripts';
			$like         = '%' . $wpdb->esc_like( $script_table ) . '%';
			$search_query = $wpdb->prepare( "SHOW TABLES LIKE %s", $like );

			if ( !$wpdb->get_results( $search_query, ARRAY_N ) ) {
				require_once CMPRO_PLUGIN_PATH . 'includes/CS_Activator.php';
				( new CS_Activator )->activate();
			} else {
				$column = $wpdb->query(
					$wpdb->prepare(
						'SELECT column_name
									FROM information_schema.columns
									WHERE table_schema = \'%1$s\'
									AND table_name = \'%2$s\'
									AND column_name = \'script_enabled\' ',
						DB_NAME,
						$script_table
					)
				);

				if ( !$column ) {
					$wpdb->query(
						"ALTER TABLE $script_table ADD COLUMN script_enabled TINYINT NOT NULL DEFAULT 1 AFTER description"
					);
					$wpdb->query(
						"ALTER TABLE $script_table ADD COLUMN script_slug VARCHAR(85) DEFAULT 1 NOT NULL AFTER script_name"
					);

					$cookie_table = $wpdb->prefix . 'cs_scan_cookies';
					$update       = $wpdb->prepare( 'UPDATE `%1$s` SET `cookie_enabled` = 1', $cookie_table );
					$wpdb->query( $update );
				}
			}

			//Update default language
			$default_language = $this->getOption( 'cs_user_default_language' );
			if ( empty( $default_language ) ) {
				$this->updateOptions( array(
					'cs_user_default_language' => CMPRO_DEFAULT_LANGUAGE,
				) );
			}

			//Update language availability
			$language_availability = $this->getOption( 'cs_language_availability' );

			// If language availability is empty or corrupted, rebuild it
			if ( !is_array( $language_availability ) || empty( $language_availability ) || count( $language_availability ) < 2 ) {
				$translations_table = $wpdb->prefix . $this->translations_table_name;
				$available_langs    = $wpdb->get_results(
					"
					SELECT DISTINCT `language`
						FROM $translations_table
				",
					ARRAY_A
				);

				$update_langs = array();
				if ( !empty( $available_langs ) ) {
					foreach ( $available_langs as $available_lang ) {
						$update_langs[ $available_lang[ 'language' ] ] = 1;
					}
				}

				$translations = $this->getDefaultTranslations();

				if ( !empty( $translations[ 'text' ] ) ) {
					foreach ( $translations[ 'text' ] as $translation ) {
						foreach ( $translation as $locale => $value ) {
							if ( !isset( $update_langs[ $locale ] ) ) {
								$update_langs[ $locale ] = 1;
							}
						}
					}
				}

				$this->updateOptions( array(
					'cs_language_availability' => $update_langs
				) );
			}

			//Change Bing tag description
			if ( version_compare( $current_version, '5.0.0.2', '<=' ) ) {
				$this->updateOptions( array(
					'cs_block_big_tag_scripts_descr' => 'Microsoft UET (Universal Event Tracking) collects data about user interactions on your website to help measure performance and optimize advertising campaigns. It enables tracking for services like Microsoft Advertising (Bing Ads), Microsoft Clarity (session replay and heatmaps), and other Microsoft marketing and analytics tools. Data collected may include pages visited, actions taken (such as purchases or form submissions), and behavioral metrics. This helps improve ad targeting, conversion tracking, and user experience analysis.'
				) );
			}

			$cs_policy_existing_page = $this->getOption( 'cs_policy_existing_page' );
			if ( !is_array( $cs_policy_existing_page ) ) {
				$this->updateOptions( array( 'cs_policy_existing_page' => array( CMPRO_DEFAULT_LANGUAGE => $cs_policy_existing_page ) ) );
			}

			$this->updateOptions( array( 'cs_check_settings' => CMPRO_LATEST_VERSION_NUMBER ) );
		}
	}

	/**
	 * Recursively sanitize array values in-place (by reference)
	 * @param mixed      $value
	 * @param string|int $key
	 * @return void
	 */
	public function cs_sanitize_array( &$value, $key ): void {
		if ( is_array( $value ) ) {
			array_walk( $value, array(
				$this,
				'cs_sanitize_array'
			) );
		} else {
			$value = sanitize_text_field( $value );
		}
	}

	/**
	 * Initial migration: create tables and copy legacy options/translations
	 * @return void
	 */
	public function maybeMigrate(): void {
		global $wpdb;
		$options_table_name = $wpdb->prefix . $this->options_table_name;
		$like               = '%' . $wpdb->esc_like( $options_table_name ) . '%';
		$search_query       = $wpdb->prepare( "SHOW TABLES LIKE %s", $like );

		if ( !$wpdb->get_results( $search_query, ARRAY_N ) ) {
			require_once CMPRO_PLUGIN_PATH . 'includes/CS_Activator.php';
			( new CS_Activator )->install_tables();

			$check_settings = array(
				'cs_check_settings'            => CMPRO_LATEST_VERSION_NUMBER,
				'cs_check_translations'        => CMPRO_LATEST_VERSION_NUMBER,
				'cs_check_script_cat_settings' => CMPRO_LATEST_VERSION_NUMBER,
				'cs_check_rule_settings'       => CMPRO_LATEST_VERSION_NUMBER,
				'cs_check_design'              => CMPRO_LATEST_VERSION_NUMBER,
			);

			$this->updateOptions( $check_settings );
			$translations_table_name = $wpdb->prefix . $this->translations_table_name;
			require_once CMPRO_PLUGIN_PATH . '/includes/functions/cs-settings-keys.php';
			$language_keys = get_language_keys();

			$general_options = get_option( 'csconsent-magic' );
			if ( !empty( $general_options ) ) {
				if ( !is_array( $general_options ) ) {
					$general_options = json_decode( $general_options, true );
				}

				if ( !empty( $general_options ) ) {
					$cm_options_language_keys = $language_keys[ 'cm_options' ];
					$iab_keys                 = get_iab_translation_keys();
					$cm_options_language_keys = array_merge( $cm_options_language_keys, $iab_keys );
					$language_list            = get_language_list();

					foreach ( $general_options as $key => $value ) {
						$lang_option = false;
						foreach ( $cm_options_language_keys as $lang_key => $l ) {

							if ( strpos( $key, $lang_key ) === 0 ) {
								$language = substr( $key, strlen( $lang_key ) + 1 );

								if ( $language === 'lang' ) {
									$lang_option = true;
									continue;
								}

								if ( !array_key_exists( $language, $language_list ) ) {
									continue;
								}

								if ( empty( $value ) ) {
									$value = $this->getDefaultTranslation( $lang_key, $language );
								}

								$this->updateLangOptions( $lang_key, $value, $language );
								$lang_option = true;
								break;
							}
						}
						if ( !$lang_option ) {
							if ( !array_key_exists( $key, $check_settings ) ) {
								$this->updateOptions( array( $key => $value ) );
							}
						}
					}
				}
			}

			//Scripts
			$taxonomy = 'cs-cookies-category';
			$scripts  = $wpdb->get_results(
				$wpdb->prepare(
					"
						SELECT t.*, tt.term_taxonomy_id, tt.taxonomy, tt.description, tt.parent, tt.count
					         FROM {$wpdb->terms} AS t
					         INNER JOIN {$wpdb->term_taxonomy} AS tt ON t.term_id = tt.term_id
					         WHERE tt.taxonomy = %s",
					$taxonomy
				)
			);

			if ( !empty( $scripts ) ) {
				foreach ( $scripts as $script ) {
					$langs = get_term_meta( $script->term_id, 'cs_edit_cat_description_lang', true );
					if ( !empty( $langs ) ) {
						foreach ( $langs as $lang ) {
							$current_translation = get_term_meta(
								$script->term_id,
								'cs_edit_cat_description_' . $lang,
								true
							);

							if ( is_array( $current_translation ) ) {
								$value = $current_translation[ 'descr' ] ?? '';
								if ( empty( $value ) ) {
									$value = $this->getDefaultTranslation( $script->slug, $lang, true );
								}
								$current_translation[ 'descr' ] = $value;

								$this->updateLangOptions( $script->slug, $current_translation, $lang, 'term' );
							}
						}
					}
				}
			}

			//Rules
			$rules = $wpdb->get_results(
				$wpdb->prepare(
					"SELECT *
								FROM {$wpdb->posts}
								WHERE post_type = %s
								  AND post_status = %s",
					CMPRO_POST_TYPE,
					'publish'
				)
			);

			if ( !empty( $rules ) ) {
				foreach ( $rules as $rule ) {
					foreach ( $language_keys[ 'rules' ] as $option => $l ) {
						$langs = get_post_meta( $rule->ID, $option . '_lang', true );
						if ( !empty( $langs ) ) {
							foreach ( $langs as $lang ) {
								$current_translation = get_post_meta( $rule->ID, $option . '_' . $lang, true );

								if ( empty( $current_translation ) ) {
									continue;
								}

								$this->updateLangOptions( $option, $current_translation, $lang, 'meta', $rule->ID );
							}
						}
					}
				}
			}

			//Design
			$design_options = get_option( 'CS--DS' );
			if ( !empty( $design_options ) ) {
				if ( !is_array( $design_options ) ) {
					$design_options = json_decode( $design_options, true );
				}

				if ( !empty( $design_options ) ) {
					$this->updateOptionsDesign( $design_options );
				}
			}

			//WP options
			$migrate_keys = $this->getMigrateKeys();
			foreach ( $migrate_keys as $key => $option ) {
				if ( $option === 0 ) {
					continue;
				}

				$this->insertMigratedValue( $key );
			}

			//IAB Languages
			$download_iab_langs = $this->getOption( 'cs_iab_download_langs' );
			if ( is_array( $download_iab_langs ) ) {
				foreach ( $download_iab_langs as $lang ) {
					$key = 'cmpro_purpose_list_option_' . $lang;
					$this->insertMigratedValue( $key );
				}
			}

			//TODO enable it
			//$this->deleteOldOptions();
			$this->updateOptions( array( 'migrated_options' => 0 ) );

			foreach ( $check_settings as $option => $v ) {
				$this->deleteOption( $option );
			}

			$pages = array(
				'consent-magic',
				'cs-settings',
				'cs-iab',
				'cs-geolocation',
				'cs-proof-consent',
				'cs-proof-statistics',
				'cs-additionals',
				'cs-license'
			);

			if ( isset( $_GET[ 'page' ] ) && in_array( $_GET[ 'page' ], $pages ) ) {
				if ( !function_exists( 'wp_safe_redirect' ) ) {
					require_once ABSPATH . 'wp-includes/pluggable.php';
				}
				wp_safe_redirect( admin_url( 'admin.php?page=consent-magic' ) );
				exit;
			}
		}
	}

	/**
	 * Insert a migrated WP option into plugin storage if present
	 * @param string $key
	 * @return void
	 */
	private function insertMigratedValue( $key ): void {
		$value = get_option( $key, null );
		if ( !is_null( $value ) ) {

			//maybe json_decode
			if ( is_string( $value ) ) {
				$value = $this->unserializeValue( $value, false );
			}

			if ( strpos( $key, 'wt_' ) === 0 ) {
				$key = substr( $key, strlen( 'wt_' ) );
			}

			$this->updateOptions( array( $key => $value ) );
		}
	}

	/**
	 * Optionally delete old WP options after migration (kept for reference)
	 * @return void
	 */
	private function deleteOldOptions(): void {
		global $wpdb;

		$old_options   = $this->getMigrateKeys();
		$language_keys = get_language_keys();

		foreach ( $old_options as $option ) {
			if ( !is_multisite() ) {
				delete_option( $option );
			} else {
				delete_site_option( $option );
			}
		}

		//TODO Check and delete old options

		//Scripts
		$taxonomy = 'cs-cookies-category';
		$scripts  = $wpdb->get_results(
			$wpdb->prepare(
				"
						SELECT t.*, tt.term_taxonomy_id, tt.taxonomy, tt.description, tt.parent, tt.count
					         FROM {$wpdb->terms} AS t
					         INNER JOIN {$wpdb->term_taxonomy} AS tt ON t.term_id = tt.term_id
					         WHERE tt.taxonomy = %s",
				$taxonomy
			)
		);

		if ( !empty( $scripts ) ) {
			foreach ( $scripts as $script ) {
				$langs = get_term_meta( $script->term_id, 'cs_edit_cat_description_lang', true );
				if ( !empty( $langs ) ) {
					foreach ( $langs as $lang ) {
						delete_term_meta( $script->term_id, 'cs_edit_cat_description_' . $lang );
					}
				}
				delete_term_meta( $script->term_id, 'cs_edit_cat_description_lang' );
			}
		}

		//Rules
		$rules = $wpdb->get_results(
			$wpdb->prepare(
				"SELECT *
								FROM {$wpdb->posts}
								WHERE post_type = %s
								  AND post_status = %s",
				CMPRO_POST_TYPE,
				'publish'
			)
		);

		if ( !empty( $rules ) ) {
			foreach ( $rules as $rule ) {
				foreach ( $language_keys[ 'rules' ] as $option => $l ) {
					$langs = get_post_meta( $rule->ID, $option . '_lang', true );
					if ( !empty( $langs ) ) {
						foreach ( $langs as $lang ) {
							delete_post_meta( $rule->ID, $option . '_' . $lang );
						}
					}
					delete_post_meta( $rule->ID, $option . '_lang' );
				}
			}
		}
	}

	/**
	 * Get list of legacy WP option keys to migrate
	 * @return array
	 */
	private function getMigrateKeys(): array {
		$keys = array(
			'csconsent_magic'                                    => 0,
			'CS--DS'                                             => 0,
			'cs_plugin_do_activation_redirect'                   => 1,
			'cs_admin_permissions'                               => 1,
			'cs_check_unblock_ip_tables'                         => 1,
			'cs_policy_existing_page'                            => 1,
			'wt_cs_consent_version'                              => 1,
			'wt_cs_consent_version_test'                         => 1,
			'cs_autodetect_email'                                => 1,
			'cs_send_important_emails'                           => 1,
			'cs_privacy_overview_content_settings'               => 0, //Don't include this option
			'cs_admin_ignore_users'                              => 1,
			'csmart_admin_modules'                               => 1,
			'cs_check_proof_tables'                              => 1,
			'cs_proof_show_count'                                => 1,
			'edd_consent_magic_pro'                              => 1,
			'edd_consent_magic_pro_activated'                    => 1,
			'edd_license_key'                                    => 1,
			'edd_consent_magic_pro_deactivate_checkbox'          => 1,
			'wc_am_product_id_consent_magic_pro'                 => 1,
			'wc_am_client_consent_magic_pro'                     => 1,
			'wc_am_client_consent_magic_pro_activated'           => 1,
			'wc_am_client_consent_magic_pro_deactivate_checkbox' => 1,
			'wc_am_client_consent_magic_pro_instance'            => 1,
			'cmpro_vendor_list_option'                           => 0, //Don't include this option
			'cmpro_vendor_list_option_cache'                     => 1,
			'cmpro_additional_vendor_list_option_cache'          => 1,
			'cmpro_iab_settings'                                 => 1,
			'necessary_cat_id'                                   => 1,
			'analytics_cat_id'                                   => 1,
			'marketing_cat_id'                                   => 1,
			'embedded_video_cat_id'                              => 1,
			'googlefonts_cat_id'                                 => 1,
			'unassigned_cat_id'                                  => 1,
			'cs_auto_scan_type'                                  => 1,
			'cs_check_scanner_tables'                            => 1,
			'cs_sb_buffer_type'                                  => 1,
			'cs_sb_buffer_option'                                => 1,
			'cs_admin_ignore_users_ip'                           => 1,
			'cs_active_rule_id'                                  => 0, //Don't include this option
			'cs_active_rule_id_first'                            => 0, //Don't include this option
			'cs_scan_status'                                     => 0, //Don't include this option
		);

		global $wpdb;

		//Scripts
		$taxonomy = 'cs-cookies-category';
		$scripts  = $wpdb->get_results(
			$wpdb->prepare(
				"
						SELECT t.*, tt.term_taxonomy_id, tt.taxonomy, tt.description, tt.parent, tt.count
					         FROM {$wpdb->terms} AS t
					         INNER JOIN {$wpdb->term_taxonomy} AS tt ON t.term_id = tt.term_id
					         WHERE tt.taxonomy = %s",
				$taxonomy
			)
		);
		if ( $scripts ) {
			foreach ( $scripts as $script ) {
				$keys[ 'cs_' . $script->name . '_' . $script->term_id . '_script_enable' ] = 1;
				$keys[ 'cs_' . $script->term_id . '_script_cat' ]                          = 1;
				$keys[ 'cs_' . $script->term_id . '_cookie_cat' ]                          = 0;
				$keys[ $script->term_id . '_cat_id' ]                                      = 0;
			}
		}

		$download_iab_langs = ConsentMagic()->getOption( 'cs_iab_download_langs' );
		if ( is_array( $download_iab_langs ) ) {
			foreach ( $download_iab_langs as $lang ) {
				$option                     = 'cmpro_purpose_list_option_' . $lang;
				$keys[ $option ]            = 0;
				$keys[ $option . '_cache' ] = 0;
			}
		}

		return $keys;
	}
}
