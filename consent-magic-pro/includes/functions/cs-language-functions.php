<?php

namespace ConsentMagicPro;

// If this file is called directly, abort.
if ( !defined( 'WPINC' ) ) {
	die;
}

/**
 * Generate front text
 * @param $front_options
 * @param $custom_text
 * @param $active_rule_id
 * @param $wp_current_lang
 * @param $cs_user_default_language
 * @return mixed
 */
function generate_front_text( $front_options, $custom_text, $active_rule_id, $wp_current_lang, $cs_user_default_language ) {
	$enable_translations = ConsentMagic()->getOption( 'cs_enable_translations' );

	foreach ( $front_options as $key => $option ) {
		$option_field   = empty( $option ) ? substr( $key, 1 ) : $key;
		$attr_value_old = $custom_text && empty( $option ) ? get_post_meta(
			$active_rule_id,
			$key,
			true
		) : ConsentMagic()->getOption( $option_field );

		if ( !empty( $attr_value_old ) ) {
			// Update default language values
			if ( $custom_text && empty( $option ) ) {
				ConsentMagic()->updateLangOptions(
					$key,
					$attr_value_old,
					CMPRO_DEFAULT_LANGUAGE,
					'meta',
					$active_rule_id
				);
				delete_post_meta( $active_rule_id, $key );
			} else {
				ConsentMagic()->updateLangOptions( $key, $attr_value_old, CMPRO_DEFAULT_LANGUAGE );
				ConsentMagic()->deleteOption( $key );
			}
			$front_options[ $key ] = $attr_value_old;
		} else {
			// Fetch translated option
			$front_options[ $key ] = fetch_translated_option(
				$key,
				$option_field,
				$enable_translations,
				$wp_current_lang,
				$cs_user_default_language,
				$custom_text,
				$active_rule_id
			);
		}
	}

	return $front_options;
}

/**
 * Fetch translated option
 * @param $key
 * @param $option_field
 * @param $enable_translations
 * @param $current_lang
 * @param $default_lang
 * @param $custom_text
 * @param $rule_id
 * @return mixed|string
 */
function fetch_translated_option( $key, $option_field, $enable_translations, $current_lang, $default_lang, $custom_text, $rule_id ) {
	$languages = $enable_translations == 1 ? array(
		$current_lang,
		$default_lang,
		CMPRO_DEFAULT_LANGUAGE
	) : array( CMPRO_DEFAULT_LANGUAGE );

	foreach ( $languages as $lang ) {
		foreach ( [ $key, $option_field ] as $field ) {
			if ( $custom_text ) {
				$option = ConsentMagic()->getLangOption( $field, $lang, 'meta', $rule_id ) ?: ConsentMagic(
				)->getLangOption( $field, $lang );
			} else {
				$option = ConsentMagic()->getLangOption( $field, $lang );
			}

			if ( !empty( $option ) ) {
				return $option;
			}
		}
	}

	return '';
}

function get_language_list() {
	return array(
		'af'             => array(
			'name'  => 'Afrikaans',
			'label' => 'Afrikaans',
		),
		'am'             => array(
			'name'  => 'Amharic',
			'label' => 'አማርኛ',
		),
		'ar'             => array(
			'name'  => 'Arabic',
			'label' => 'العربية',
		),
		'ary'            => array(
			'name'  => 'Moroccan Arabic',
			'label' => 'العربية المغربية',
		),
		'as'             => array(
			'name'  => 'Assamese',
			'label' => 'অসমীয়া',
		),
		'azb'            => array(
			'name'  => 'South Azerbaijani',
			'label' => 'گؤنئی آذربایجان',
		),
		'az'             => array(
			'name'  => 'Azerbaijani',
			'label' => 'Azərbaycan dili',
		),
		'bel'            => array(
			'name'  => 'Belarusian',
			'label' => 'Беларуская мова',
		),
		'bg_BG'          => array(
			'name'  => 'Bulgarian',
			'label' => 'Български',
		),
		'bn_BD'          => array(
			'name'  => 'Bengali (Bangladesh)',
			'label' => 'বাংলা',
		),
		'bo'             => array(
			'name'  => 'Tibetan',
			'label' => 'བོད་ཡིག',
		),
		'bs_BA'          => array(
			'name'  => 'Bosnian',
			'label' => 'Bosanski',
		),
		'ca'             => array(
			'name'  => 'Catalan',
			'label' => 'Català',
		),
		'ceb'            => array(
			'name'  => 'Cebuano',
			'label' => 'Cebuano',
		),
		'cs_CZ'          => array(
			'name'  => 'Czech',
			'label' => 'Čeština',
		),
		'cy'             => array(
			'name'  => 'Welsh',
			'label' => 'Cymraeg',
		),
		'da_DK'          => array(
			'name'  => 'Danish',
			'label' => 'Dansk',
		),
		'de_CH_informal' => array(
			'name'  => 'German (Switzerland, Informal)',
			'label' => 'Deutsch (Schweiz, Du)',
		),
		'de_AT'          => array(
			'name'  => 'German (Austria)',
			'label' => 'Deutsch (Österreich)',
		),
		'de_DE_formal'   => array(
			'name'  => 'German (Formal)',
			'label' => 'Deutsch (Sie)',
		),
		'de_DE'          => array(
			'name'  => 'German',
			'label' => 'Deutsch',
		),
		'de_CH'          => array(
			'name'  => 'German (Switzerland)',
			'label' => 'Deutsch (Schweiz)',
		),
		'dsb'            => array(
			'name'  => 'Lower Sorbian',
			'label' => 'Dolnoserbšćina',
		),
		'dzo'            => array(
			'name'  => 'Dzongkha',
			'label' => 'རྫོང་ཁ',
		),
		'el'             => array(
			'name'  => 'Greek',
			'label' => 'Ελληνικά',
		),
		'en_ZA'          => array(
			'name'  => 'English (South Africa)',
			'label' => 'English (South Africa)',
		),
		'en_CA'          => array(
			'name'  => 'English (Canada)',
			'label' => 'English (Canada)',
		),
		'en_NZ'          => array(
			'name'  => 'English (New Zealand)',
			'label' => 'English (New Zealand)',
		),
		'en_GB'          => array(
			'name'  => 'English (UK)',
			'label' => 'English (UK)',
		),
		'en_AU'          => array(
			'name'  => 'English (Australia)',
			'label' => 'English (Australia)',
		),
		'en_US'          => array(
			'name'  => 'English (United States)',
			'label' => 'English (United States)',
		),
		'eo'             => array(
			'name'  => 'Esperanto',
			'label' => 'Esperanto',
		),
		'es_EC'          => array(
			'name'  => 'Spanish (Ecuador)',
			'label' => 'Español de Ecuador',
		),
		'es_AR'          => array(
			'name'  => 'Spanish (Argentina)',
			'label' => 'Español de Argentina',
		),
		'es_CO'          => array(
			'name'  => 'Spanish (Colombia)',
			'label' => 'Español de Colombia',
		),
		'es_MX'          => array(
			'name'  => 'Spanish (Mexico)',
			'label' => 'Español de México',
		),
		'es_DO'          => array(
			'name'  => 'Spanish (Dominican Republic)',
			'label' => 'Español de República Dominicana',
		),
		'es_PE'          => array(
			'name'  => 'Spanish (Peru)',
			'label' => 'Español de Perú',
		),
		'es_UY'          => array(
			'name'  => 'Spanish (Uruguay)',
			'label' => 'Español de Uruguay',
		),
		'es_CL'          => array(
			'name'  => 'Spanish (Chile)',
			'label' => 'Español de Chile',
		),
		'es_PR'          => array(
			'name'  => 'Spanish (Puerto Rico)',
			'label' => 'Español de Puerto Rico',
		),
		'es_VE'          => array(
			'name'  => 'Spanish (Venezuela)',
			'label' => 'Español de Venezuela',
		),
		'es_GT'          => array(
			'name'  => 'Spanish (Guatemala)',
			'label' => 'Español de Guatemala',
		),
		'es_ES'          => array(
			'name'  => 'Spanish (Spain)',
			'label' => 'Español',
		),
		'es_CR'          => array(
			'name'  => 'Spanish (Costa Rica)',
			'label' => 'Español de Costa Rica',
		),
		'et'             => array(
			'name'  => 'Estonian',
			'label' => 'Eesti',
		),
		'eu'             => array(
			'name'  => 'Basque',
			'label' => 'Euskara',
		),
		'fa_IR'          => array(
			'name'  => 'Persian',
			'label' => 'فارسی',
		),
		'fa_AF'          => array(
			'name'  => 'Persian (Afghanistan)',
			'label' => '(فارسی (افغانستان',
		),
		'fi'             => array(
			'name'  => 'Finnish',
			'label' => 'Finnish',
		),
		'fr_CA'          => array(
			'name'  => 'French (Canada)',
			'label' => 'Français du Canada',
		),
		'fr_FR'          => array(
			'name'  => 'French (France)',
			'label' => 'Français',
		),
		'fr_BE'          => array(
			'name'  => 'French (Belgium)',
			'label' => 'Français de Belgique',
		),
		'fr_CH'          => array(
			'name'  => 'French (Switzerland)',
			'label' => 'Français de Switzerland',
		),
		'fur'            => array(
			'name'  => 'Friulian',
			'label' => 'Friulian',
		),
		'ga_IE'          => array(
			'name'  => 'Irish',
			'label' => 'Irish',
		),
		'gd'             => array(
			'name'  => 'Scottish Gaelic',
			'label' => 'Gàidhlig',
		),
		'gl_ES'          => array(
			'name'  => 'Galician',
			'label' => 'Galego',
		),
		'gu'             => array(
			'name'  => 'Gujarati',
			'label' => 'ગુજરાતી',
		),
		'haz'            => array(
			'name'  => 'Hazaragi',
			'label' => 'هزاره گی',
		),
		'he_IL'          => array(
			'name'  => 'Hebrew',
			'label' => 'עִבְרִית',
		),
		'hi_IN'          => array(
			'name'  => 'Hindi',
			'label' => 'हिन्दी',
		),
		'hr'             => array(
			'name'  => 'Croatian',
			'label' => 'Hrvatski',
		),
		'hsb'            => array(
			'name'  => 'Upper Sorbian',
			'label' => 'Hornjoserbšćina',
		),
		'hu_HU'          => array(
			'name'  => 'Hungarian',
			'label' => 'Magyar',
		),
		'hy'             => array(
			'name'  => 'Armenian',
			'label' => 'Հայերեն',
		),
		'id_ID'          => array(
			'name'  => 'Indonesian',
			'label' => 'Bahasa Indonesia',
		),
		'is_IS'          => array(
			'name'  => 'Icelandic',
			'label' => 'Íslenska',
		),
		'it_IT'          => array(
			'name'  => 'Italian',
			'label' => 'Italiano',
		),
		'ja'             => array(
			'name'  => 'Japanese',
			'label' => '日本語',
		),
		'jv_ID'          => array(
			'name'  => 'Javanese',
			'label' => 'Basa Jawa',
		),
		'ka_GE'          => array(
			'name'  => 'Georgian',
			'label' => 'ქართული',
		),
		'kab'            => array(
			'name'  => 'Kabyle',
			'label' => 'Taqbaylit',
		),
		'kk'             => array(
			'name'  => 'Kazakh',
			'label' => 'Қазақ тілі',
		),
		'km'             => array(
			'name'  => 'Khmer',
			'label' => 'ភាសាខ្មែរ',
		),
		'kn'             => array(
			'name'  => 'Kannada',
			'label' => 'ಕನ್ನಡ',
		),
		'ko_KR'          => array(
			'name'  => 'Korean',
			'label' => '한국어',
		),
		'ckb'            => array(
			'name'  => 'Kurdish (Sorani)',
			'label' => 'كوردی‎',
		),
		'lo'             => array(
			'name'  => 'Lao',
			'label' => 'ພາສາລາວ',
		),
		'lt_LT'          => array(
			'name'  => 'Lithuanian',
			'label' => 'Lietuvių kalba',
		),
		'lv'             => array(
			'name'  => 'Latvian',
			'label' => 'Latviešu valoda',
		),
		'mk_MK'          => array(
			'name'  => 'Macedonian',
			'label' => 'Македонски јазик',
		),
		'ml_IN'          => array(
			'name'  => 'Malayalam',
			'label' => 'മലയാളം',
		),
		'mn'             => array(
			'name'  => 'Mongolian',
			'label' => 'Монгол',
		),
		'mr'             => array(
			'name'  => 'Marathi',
			'label' => 'मराठी',
		),
		'mt'             => array(
			'name'  => 'Maltese',
			'label' => 'Maltese',
		),
		'ms_MY'          => array(
			'name'  => 'Malay',
			'label' => 'Bahasa Melayu',
		),
		'my_MM'          => array(
			'name'  => 'Myanmar (Burmese)',
			'label' => 'ဗမာစာ',
		),
		'nb_NO'          => array(
			'name'  => 'Norwegian (Bokmål)',
			'label' => 'Norsk bokmål',
		),
		'ne_NP'          => array(
			'name'  => 'Nepali',
			'label' => 'नेपाली',
		),
		'nl_NL_formal'   => array(
			'name'  => 'Dutch (Formal)',
			'label' => 'Nederlands (Formeel)',
		),
		'nl_BE'          => array(
			'name'  => 'Dutch (Belgium)',
			'label' => 'Nederlands (België)',
		),
		'nl_NL'          => array(
			'name'  => 'Dutch',
			'label' => 'Nederlands',
		),
		'nn_NO'          => array(
			'name'  => 'Norwegian (Nynorsk)',
			'label' => 'Norsk nynorsk',
		),
		'oci'            => array(
			'name'  => 'Occitan',
			'label' => 'Occitan',
		),
		'pa_IN'          => array(
			'name'  => 'Punjabi',
			'label' => 'ਪੰਜਾਬੀ',
		),
		'pl_PL'          => array(
			'name'  => 'Polish',
			'label' => 'Polski',
		),
		'ps'             => array(
			'name'  => 'Pashto',
			'label' => 'پښتو',
		),
		'pt_PT'          => array(
			'name'  => 'Portuguese (Portugal)',
			'label' => 'Português',
		),
		'pt_AO'          => array(
			'name'  => 'Portuguese (Angola)',
			'label' => 'Português de Angola',
		),
		'pt_PT_ao90'     => array(
			'name'  => 'Portuguese (Portugal, AO90)',
			'label' => 'Português (AO90)',
		),
		'pt_BR'          => array(
			'name'  => 'Portuguese (Brazil)',
			'label' => 'Português do Brasil',
		),
		'rhg'            => array(
			'name'  => 'Rohingya',
			'label' => 'Ruáinga',
		),
		'ro_RO'          => array(
			'name'  => 'Romanian',
			'label' => 'Română',
		),
		'ru_RU'          => array(
			'name'  => 'Russian',
			'label' => 'Русский',
		),
		'sah'            => array(
			'name'  => 'Sakha',
			'label' => 'Сахалыы',
		),
		'snd'            => array(
			'name'  => 'Sindhi',
			'label' => 'سنڌي',
		),
		'si_LK'          => array(
			'name'  => 'Sinhala',
			'label' => 'සිංහල',
		),
		'sk_SK'          => array(
			'name'  => 'Slovak',
			'label' => 'Slovenčina',
		),
		'skr'            => array(
			'name'  => 'Saraiki',
			'label' => 'سرائیکی',
		),
		'sl_SI'          => array(
			'name'  => 'Slovenian',
			'label' => 'Slovenščina',
		),
		'sq'             => array(
			'name'  => 'Albanian',
			'label' => 'Shqip',
		),
		'sr_RS'          => array(
			'name'  => 'Serbian',
			'label' => 'Српски језик',
		),
		'sv_SE'          => array(
			'name'  => 'Swedish',
			'label' => 'Svenska',
		),
		'sw'             => array(
			'name'  => 'Swahili',
			'label' => 'Kiswahili',
		),
		'szl'            => array(
			'name'  => 'Silesian',
			'label' => 'Ślōnskŏ gŏdka',
		),
		'ta_IN'          => array(
			'name'  => 'Tamil',
			'label' => 'தமிழ்',
		),
		'ta_LK'          => array(
			'name'  => 'Tamil (Sri Lanka)',
			'label' => 'தமிழ்',
		),
		'te'             => array(
			'name'  => 'Telugu',
			'label' => 'తెలుగు',
		),
		'th'             => array(
			'name'  => 'Thai',
			'label' => 'ไทย',
		),
		'tl'             => array(
			'name'  => 'Tagalog',
			'label' => 'Tagalog',
		),
		'tr_TR'          => array(
			'name'  => 'Turkish',
			'label' => 'Türkçe',
		),
		'tt_RU'          => array(
			'name'  => 'Tatar',
			'label' => 'Татар теле',
		),
		'tah'            => array(
			'name'  => 'Tahitian',
			'label' => 'Reo Tahiti',
		),
		'ug_CN'          => array(
			'name'  => 'Uighur',
			'label' => 'ئۇيغۇرچە',
		),
		'uk'             => array(
			'name'  => 'Ukrainian',
			'label' => 'Українська',
		),
		'ur'             => array(
			'name'  => 'Urdu',
			'label' => 'اردو',
		),
		'uz_UZ'          => array(
			'name'  => 'Uzbek',
			'label' => 'O‘zbekcha',
		),
		'vi'             => array(
			'name'  => 'Vietnamese',
			'label' => 'Tiếng Việt',
		),
		'zh_HK'          => array(
			'name'  => 'Chinese (Hong Kong)',
			'label' => '香港中文版	',
		),
		'zh_TW'          => array(
			'name'  => 'Chinese (Taiwan)',
			'label' => '繁體中文',
		),
		'zh_CN'          => array(
			'name'  => 'Chinese (China)',
			'label' => '简体中文',
		)
	);
}

function get_lang_meta_list() {
	return array(
		'_cs_text_in_small_bar_popup',
		'_cs_text_in_large_bar_popup',
		'_cs_btn_text_allow_all',
		'_cs_btn_text_disable_all',
		'_cs_btn_text_customize',
		'_cs_btn_text_privacy_cookie',
		'_cs_btn_text_sticky_cookie',
		'_cs_btn_text_custom_button',
		'_cs_text_in_options_popup',
		'_cs_title_in_options_popup',
		'_cs_subtitle_in_options_popup',
		'_cs_btn_text_allow_all_in_options_popup',
		'_cs_btn_text_disable_all_in_options_popup',
		'_cs_btn_text_confirm_in_options_popup',
		'_cs_always_on_in_options_popup',
		'_cs_text_consent',
		'_cs_text_in_single_design',
		'_cs_title_in_single_design',
		'_cs_subtitle_in_single_design',
		'_cs_btn_text_allow_all_in_single_design',
		'_cs_btn_text_disable_all_in_single_design',
		'_cs_btn_text_confirm_in_single_design',
		'_cs_always_on_in_single_design',
		'_cs_btn_text_custom_button_in_single_design',
	);
}

function get_general_language( $current_lang ) {

	$short_current_lang = preg_replace( '/_.*$/', '', $current_lang );

	switch ( $short_current_lang ) {
		case 'en':
			$general_lang = 'en_US';
			break;
		case 'bg':
			$general_lang = 'bg_BG';
			break;
		case 'cs':
			$general_lang = 'cs_CZ';
			break;
		case 'da':
			$general_lang = 'da_DK';
			break;
		case 'de':
			$general_lang = 'de_DE';
			break;
		case 'el':
			$general_lang = 'el';
			break;
		case 'es':
			$general_lang = 'es_ES';
			break;
		case 'et':
			$general_lang = 'et';
			break;
		case 'fi':
			$general_lang = 'fi';
			break;
		case 'fr':
			$general_lang = 'fr_FR';
			break;
		case 'ga':
			$general_lang = 'ga_IE';
			break;
		case 'he':
			$general_lang = 'he_IL';
			break;
		case 'hr':
			$general_lang = 'hr';
			break;
		case 'hu':
			$general_lang = 'hu_HU';
			break;
		case 'it':
			$general_lang = 'it_IT';
			break;
		case 'lt':
			$general_lang = 'lt_LT';
			break;
		case 'lv':
			$general_lang = 'lv';
			break;
		case 'mt':
			$general_lang = 'mt';
			break;
		case 'nl':
			$general_lang = 'nl_NL';
			break;
		case 'pl':
			$general_lang = 'pl_PL';
			break;
		case 'pt':
			$general_lang = 'pt_PT';
			break;
		case 'ro':
			$general_lang = 'ro_RO';
			break;
		case 'sk':
			$general_lang = 'sk_SK';
			break;
		case 'sl':
			$general_lang = 'sl_SI';
			break;
		case 'sv':
			$general_lang = 'sv_SE';
			break;
		case 'uk':
			$general_lang = 'uk';
			break;
		default:
			$general_lang = $current_lang;
	}

	return $general_lang;
}

/**
 * Get languages supported by Consent Magic
 * @return array
 */
function get_cm_available_langs() {
	$langs = array(
		'en_US',
		'bg_BG',
		'cs_CZ',
		'da_DK',
		'de_DE',
		'el',
		'es_ES',
		'et',
		'fi',
		'fr_FR',
		'ga_IE',
		'he_IL',
		'hr',
		'hu_HU',
		'it_IT',
		'lt_LT',
		'lv',
		'mt',
		'nl_NL',
		'pl_PL',
		'pt_PT',
		'ro_RO',
		'sk_SK',
		'sl_SI',
		'sv_SE',
		'uk',
	);

	return $langs;
}

function render_add_lang_button( $key, $render ) {
	?>
    <button class="btn btn-primary btn-primary-type2 with-icon add_new_lang_action"
            data-render="<?php echo esc_attr( $render ); ?>"
            data-key="<?php echo esc_attr( $key ); ?>"
            data-plugin-name="<?php echo esc_attr( ConsentMagic()->get_plugin_name() ); ?>"><i
                class="icon-plus"></i> <?php esc_html_e( 'Add New Language', 'consent-magic' ); ?>
    </button>
	<?php
}

function render_lang_loader() {
	?>
	<?php include CMPRO_PLUGIN_VIEWS_PATH . "admin/components/spinner.php"; ?>
	<?php
}

function render_additional_lang_button( $target ) {
	?>
    <h4 class="lh-134"><?php esc_html_e(
			sprintf( __( 'Additional languages for %s:', 'consent-magic' ), $target )
		); ?></h4>
	<?php
}

function renderPrivacyPolicyPage( $language ) {

    $cs_policy_existing_pages = ConsentMagic()->getOption( 'cs_policy_existing_page' );
    if ( isset( $cs_policy_existing_pages[ $language ] ) ) {
        $policy_link = $cs_policy_existing_pages[ $language ];
    } elseif ( isset( $cs_policy_existing_pages[ CMPRO_DEFAULT_LANGUAGE ] ) ) {
        $policy_link = $cs_policy_existing_pages[ CMPRO_DEFAULT_LANGUAGE ];
    } else {
        $policy_link = null;
    }

    return $policy_link;

}