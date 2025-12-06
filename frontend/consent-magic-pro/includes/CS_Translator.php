<?php

namespace ConsentMagicPro;

if ( !defined( 'ABSPATH' ) ) {
	exit; // Exit if accessed directly.
}

class CS_Translator {

	private static ?CS_Translator $_instance = null;

	private ?string $current_language = null;


	/**
	 * $_instance CS_Translator
	 * @param null $language
	 * @return CS_Translator|null
	 */
	public static function instance( $language = null ): ?CS_Translator {
		if ( is_null( self::$_instance ) ) {
			self::$_instance = new self( $language );
		}

		return self::$_instance;
	}

	public function __construct( $language = null ) {
		if ( $language !== null ) {
			$this->current_language = $language;
		} elseif ( $this->current_language === null ) {
			$this->current_language = get_locale();
		}
	}

	/**
	 * Set the current language
	 * @param string $language
	 * @return void
	 */
	public function setCurrentLanguage( string $language ): void {
		$this->current_language = $language;
	}

	/**
	 * Render the language options for the terms
	 * @param array $terms
	 * @return array
	 */
	public function renderLangOptionsTerm( array $terms ): array {

		if ( ConsentMagic()->getOption( 'cs_enable_translations' ) == 1 ) {
			$current_lang            = $this->current_language;
			$cs_user_default_language = ConsentMagic()->getOption( 'cs_user_default_language' );
			$cs_language_availability = ConsentMagic()->getOption( 'cs_language_availability' );
			if ( isset( $cs_language_availability[ $current_lang ] ) && $cs_language_availability[ $current_lang ] == 0 ) {
				$current_lang = $cs_user_default_language;
			}
		} else {
			$current_lang = $cs_user_default_language = CMPRO_DEFAULT_LANGUAGE;
		}

		if ( !empty( $terms ) ) {
			foreach ( $terms as $term ) {
				if ( !empty( $term->description ) ) {
					$description = array(
						'name'  => $term->name,
						'descr' => $term->description,
					);
					ConsentMagic()->updateLangOptions( $term->slug, $description, CMPRO_DEFAULT_LANGUAGE, 'term' );
					wp_update_term( $term->term_id, 'cs-cookies-category', array( 'description' => '' ) );
					$display_lang = CMPRO_DEFAULT_LANGUAGE;
				} else {
					$description = ConsentMagic()->getLangOptionTerm( $term->slug, $current_lang );
					$display_lang = $current_lang;
					if ( empty( $description ) || ( empty( $description[ 'value' ] ) && empty( $description[ 'category_name' ] ) ) ) {
						$description = ConsentMagic()->getLangOptionTerm( $term->slug, $cs_user_default_language );
						$display_lang = $cs_user_default_language;
						if ( empty( $description ) || ( empty( $description[ 'value' ] ) && empty( $description[ 'category_name' ] ) ) ) {
							$description = ConsentMagic()->getLangOptionTerm( $term->slug, CMPRO_DEFAULT_LANGUAGE );
							$display_lang = CMPRO_DEFAULT_LANGUAGE;
						}
					}
				}

				if ( !empty( $description ) && is_array( $description ) ) {

					if ( ( !isset( $description[ 'category_name' ] ) || empty( $description[ 'category_name' ] ) ) && ( !isset( $description[ 'value' ] ) || empty( $description[ 'value' ] ) ) ) {
						$defaults =             ConsentMagic()->getDefaultTranslation( $term->slug, CMPRO_DEFAULT_LANGUAGE, true );
						$term->name_l        = $defaults[ 'name' ] ?? '';
						$term->description_l = $defaults[ 'descr' ] ?? '';
						$term->display_lang  = CMPRO_DEFAULT_LANGUAGE;
					} else {
						$term->name_l        = $description[ 'category_name' ];
						$term->description_l = $description[ 'value' ];
						$term->display_lang  = $display_lang;
					}

				} else {
					$term->name_l        = $term->name;
					$term->description_l = '';
					$term->display_lang  = CMPRO_DEFAULT_LANGUAGE;
				}
			}
		}

		return $terms;
	}
}

/**
 * CS_Translator function
 * @param null $language
 * @return ?CS_Translator
 */
function CS_Translator( $language = null ): ?CS_Translator {
	return CS_Translator::instance( $language );
}

CS_Translator();