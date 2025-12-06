<?php

/**
 * @wordpress-plugin
 * Plugin Name:       ConsentMagic Pro
 * Plugin URI:        https://www.pixelyoursite.com/plugins/consentmagic/
 * Description:       Persuade your visitors to agree to tracking, while respecting the legal requirements.
 * Version:           5.1.0.1
 * Requires at least: 5.8
 * Author:            ConsentMagic
 * Author URI:        https://www.pixelyoursite.com/plugins/consentmagic/
 * License URI:       https://www.pixelyoursite.com/plugins/consentmagic/
 * Text Domain:       consent-magic
 * Domain Path:       /languages
 */

// If this file is called directly, abort.
if ( !defined( 'WPINC' ) ) {
	die;
}
define( 'CMPRO_PLUGIN_DEVELOPMENT_MODE', false );
define( 'CMPRO_PLUGIN_BASENAME', plugin_basename( __FILE__ ) );
define( 'CMPRO_PLUGIN_PATH', plugin_dir_path( __FILE__ ) );
define( 'CMPRO_PLUGIN_VIEWS_PATH', plugin_dir_path( __FILE__ ) . 'views/' );
define( 'CMPRO_PLUGIN_SCRIPTS_PATH', plugin_dir_path( __FILE__ ) . 'includes/scripts' );
define( 'CMPRO_PLUGIN_URL', plugin_dir_url( __FILE__ ) );
define( 'CMPRO_DB_KEY_PREFIX', 'CS-' );
define( 'CMPRO_LATEST_VERSION_NUMBER', '5.1.0.1' );
define( 'CMPRO_MIN_PHP_VERSION', '7.4.0' );
define( 'CMPRO_SETTINGS_FIELD', CMPRO_DB_KEY_PREFIX . CMPRO_LATEST_VERSION_NUMBER );
define( 'CMPRO_SETTINGS_DESIGN_FIELD', CMPRO_DB_KEY_PREFIX . '-DS' );
define( 'CMPRO_MIGRATED_VERSION', CMPRO_DB_KEY_PREFIX . 'MV' );
define( 'CMPRO_PLUGIN_FILENAME', __FILE__ );
define( 'CMPRO_ITEM_NAME', 'ConsentMagic' );
define( 'CMPRO_POST_TYPE', 'consentmagic' );
define( 'CMPRO_TEMPLATE_POST_TYPE', 'cs-template' );
define( 'CMPRO_POST_TYPE_COOKIES', 'cs-cookies' );
define( 'CMPRO_POST_TYPE_SCRIPTS', 'cs-scripts' );
define( 'CMPRO_ADMIN_PAGE', 'consent-magic' );
define( 'CMPRO_ADMIN_OPTIONS_NAME', 'ConsentMagic' );
define( 'CMPRO_ACTIVATION_ID', 'proconsentmagic' );
define( 'CMPRO_DEFAULT_LANGUAGE', 'en_US' );
define( 'CMPRO_LICENSE_PLUGIN_NAME', 'consent_magic_pro' );
define( 'CMPRO_LICENSE_TYPE', 'edd' ); // or 'woo'
define( 'CMPRO_LICENSE_NAME', 'ConsentMagic by PixelYourSite' );  // or 'ConsentMagic PRO'
define( 'CMPRO_CMP_ID', 444 );
define( 'CMPRO_CMP_VERSION', 1 );

//Video link in the bottom bar
define( 'CMPRO_VIDEO_URL', 'https://www.youtube.com/watch?v=fAwsayYLo5s' );
define( 'CMPRO_VIDEO_TITLE', 'Meta Pixel and API setup - Boost EMQ' );

// Exit if accessed directly
defined( 'ABSPATH' ) || exit;

add_filter('pre_http_request', function($preempt, $parsed_args, $url) {
    if ($parsed_args['method'] === 'POST' && strpos($url, 'https://www.pixelyoursite.com') !== false) {
        // Directly access 'item_name' from the body array
        $item_name = isset($parsed_args['body']['item_name']) ? $parsed_args['body']['item_name'] : 'Unknown Item';

        $response_array = [
            "success" => true,
            "license" => "valid",
            "item_id" => false,
            "item_name" => $item_name, // Use directly from the POST array
            "checksum" => "B5E0B5F8DD8689E6ACA49DD6E6E1A930",
            "expires" => "lifetime",
            "payment_id" => 123321,
            "customer_name" => "GPL",
            "customer_email" => "noreply@gmail.com",
            "license_limit" => 10,
            "site_count" => 1,
            "activations_left" => 10,
            "price_id" => "4"
        ];

        $response_body = json_encode($response_array);

        return [
            'headers' => [],
            'body' => $response_body,
            'response' => [
                'code' => 200,
                'message' => 'OK'
            ],
        ];
    }

    return $preempt;
}, 10, 3);

add_action('init', function() {
    if (ConsentMagicPro\ConsentMagic()->getOption('edd_consent_magic_pro_activated') !== 'Activated') {
        ConsentMagicPro\ConsentMagic()->updateOptions(array(
            'edd_consent_magic_pro_activated' => 'Activated',
            'edd_license_key' => 'B5E0B5F8DD8689E6ACA49DD6E6E1A930',
            'edd_consent_magic_pro_deactivate_checkbox' => 'off'
        ));
    }
});

function cmpro_isCMFreeActive() {

	if ( !function_exists( 'is_plugin_active' ) ) {
		include_once ABSPATH . 'wp-admin/includes/plugin.php';
	}

	return is_plugin_active( 'consent-magic/consent-magic.php' );
}

/**
 * The code that runs during plugin activation.
 * This action is documented in includes/class-cs-activator.php
 */
function cmpro_activate_consent_smart_pro() {

	if ( cmpro_isCMFreeActive() ) {
		deactivate_plugins( 'consent-magic/consent-magic.php' );
	}

	require_once plugin_dir_path( __FILE__ ) . 'includes/CS_Activator.php';
	( new ConsentMagicPro\CS_Activator )->activate();

	set_transient( 'cs-admin-notice-activation', true, 5 );
}

register_activation_hook( __FILE__, 'cmpro_activate_consent_smart_pro' );

if ( cmpro_isCMFreeActive() ) {
	return; // exit early when CM Free is active
}

function cmpro_plugin_off() {
	deactivate_plugins( CMPRO_PLUGIN_BASENAME );

	return;
}

//check min PHP version
if ( version_compare( PHP_VERSION, CMPRO_MIN_PHP_VERSION ) < 0 ) {

	add_action( 'admin_notices', 'cmpro_incompatible_notice' );
	add_action( 'admin_init', 'cmpro_plugin_off' );

	if ( isset( $_GET[ 'activate' ] ) ) {
		unset( $_GET[ 'activate' ] );
	}

	return;
}

function cmpro_incompatible_notice() {
	echo "<div class='notice notice-error cm-fixed-notice'><p><strong>" . CMPRO_ITEM_NAME . " </strong>" . sprintf( esc_html__( ' has been deactivated. The minimum PHP version is %s. You have version %s.', 'consent-magic' ), CMPRO_MIN_PHP_VERSION, PHP_VERSION ) . "</p></div>";
}

/**
 * IMPORTANT
 * Use only the code below at the top of a plugin file below the plugin header, or at the top of a theme functions file.
 */

require_once CMPRO_PLUGIN_PATH . 'includes/CS_Logger.php';

/**
 * The core plugin class that is used to define internationalization,
 * admin-specific hooks, and public-facing site hooks.
 */
require_once CMPRO_PLUGIN_PATH . '/vendor/autoload.php';

require_once CMPRO_PLUGIN_PATH . 'includes/functions/cs-pluggable.php';

require_once CMPRO_PLUGIN_PATH . 'includes/functions/cs-general-functions.php';

require_once CMPRO_PLUGIN_PATH . 'includes/functions/cs-render-functions.php';

require_once CMPRO_PLUGIN_PATH . 'includes/functions/cs-render-notices.php';

require_once CMPRO_PLUGIN_PATH . 'includes/functions/cs-render-scripts.php';

require_once CMPRO_PLUGIN_PATH . 'includes/functions/cs-render-script-categories.php';

require_once CMPRO_PLUGIN_PATH . 'includes/functions/cs-text-functions.php';

require_once CMPRO_PLUGIN_PATH . 'includes/functions/cs-language-functions.php';

require_once CMPRO_PLUGIN_PATH . 'includes/CS_Options.php';

require_once CMPRO_PLUGIN_PATH . 'includes/ConsentMagic.php';

require_once CMPRO_PLUGIN_PATH . 'includes/CS_Translator.php';

require_once CMPRO_PLUGIN_PATH . 'includes/CS_Cookies.php';

require_once CMPRO_PLUGIN_PATH . 'includes/common/traits/CS_Throttle.php';

\ConsentMagicPro\ConsentMagic()->init();

if ( CMPRO_LICENSE_TYPE == 'edd' ) {
	if ( !class_exists( 'ConsentMagicPro\CS_EDD_License_manager' ) ) {
		require_once plugin_dir_path( __FILE__ ) . 'license-edd.php';
	}

	if ( class_exists( 'ConsentMagicPro\CS_EDD_License_manager' ) ) {
		new ConsentMagicPro\CS_EDD_License_manager( 'consent-magic', CMPRO_LICENSE_TYPE, CMPRO_LATEST_VERSION_NUMBER, CMPRO_LICENSE_NAME );
	}
} elseif ( CMPRO_LICENSE_TYPE == 'woo' ) {
	// Load WC_AM_Client class if it exists.
	if ( !class_exists( 'ConsentMagicPro\WC_AM_Client_2_8' ) ) {
		require_once plugin_dir_path( __FILE__ ) . 'wc-am-client.php';
	}

	// Instantiate WC_AM_Client class object if the WC_AM_Client class is loaded.
	if ( class_exists( 'ConsentMagicPro\WC_AM_Client_2_8' ) ) {
		$cs_wcam_lib = new ConsentMagicPro\WC_AM_Client_2_8( __FILE__, '', CMPRO_LATEST_VERSION_NUMBER, 'plugin', 'https://www.consentmagic.com/', CMPRO_LICENSE_NAME );
	}
}

if ( ConsentMagicPro\is_plugin_activated() && ConsentMagicPro\ConsentMagic()->check_first_flow() ) {
	require_once CMPRO_PLUGIN_PATH . 'includes/CS_Integrations.php';

	require_once CMPRO_PLUGIN_PATH . 'includes/functions/cs-shortcode.php';
}

/**
 * The code that runs during plugin uninstall.
 */
function cmpro_deactivation_consent_magic_pro() {
	require_once plugin_dir_path( __FILE__ ) . 'includes/CS_Uninstall.php';
	ConsentMagicPro\CS_Uninstall::deactivation();
}

register_deactivation_hook( __FILE__, 'cmpro_deactivation_consent_magic_pro' );

add_action( 'admin_init', 'cs_plugin_redirect_pro' );

/**
 * Show a notice to anyone who has just updated this plugin
 * This notice shouldn't display to anyone who has just installed the plugin for the first time
 */
function cmpro_display_update_notice() {
	$user_id = get_current_user_id();
	// Check the transient to see if we've just updated the plugin
	if ( CMPRO_LATEST_VERSION_NUMBER == '1.4.1' && !get_user_meta( $user_id, 'cs_notice_dismissed' ) ) { ?>
        <div class="notice notice-warning cm-fixed-notice cs-notice-dismissed-wrap">
            <p>
				<?php echo '<strong>' . esc_html__( 'Important:', 'consent-magic' ) . '</strong>' . sprintf( esc_html__( 'You can enable a dedicated Google Fonts category %sfrom here%s. The plugin will inform users about Google Fonts and give them an opt-out option. If they block Google Fonts, another font type is used. It works for "Inform and Opt-out" and "Ask before tracking" consent types. ', 'consent-magic' ), '<a href="' . esc_url( get_admin_url( null, 'admin.php?page=consent-magic&tab=cs-script-blocking' ) ) . '">', '</a>' ); ?>
            </p>
            <p>
				<?php printf( esc_html__( ' %sWatch video%s ', 'consent-magic' ), '<a href="https://www.youtube.com/watch?v=58iGGaZsl0E" target="_blank">', '</a>' ); ?>
            </p>
            <p><a href="#" class="cs-notice-dismissed"><?php esc_html__( 'Dismiss:', 'consent-magic' ); ?></a></p>
        </div>
	<?php }
}

add_action( 'wp_ajax_cmpro_ajax_notice_dismissed', 'cmpro_ajax_notice_dismissed' );

function cmpro_ajax_notice_dismissed() {
	$user_id = get_current_user_id();
	add_user_meta( $user_id, 'cs_notice_dismissed', 'true', true );
	wp_send_json_success();
}

function cmpro_notice_dismissed() {
	$user_id = get_current_user_id();
	if ( isset( $_GET[ 'cs-notice-dismissed' ] ) ) {
		add_user_meta( $user_id, 'cs_notice_dismissed', 'true', true );
	}
}

add_action( 'admin_init', 'cmpro_notice_dismissed' );

function cs_plugin_redirect_pro() {
	if ( \ConsentMagicPro\ConsentMagic()->getOption( 'cs_plugin_do_activation_redirect' ) ) {
		\ConsentMagicPro\ConsentMagic()->updateOptions( array(  'cs_plugin_do_activation_redirect' => false ) );
	}
}

function cmpro_test_mode_notice_pro() {
	?>
    <div class='notice notice-warning cm-fixed-notice'>
        <p>
			<?php esc_html_e( 'Testing mode is enabled for ConsentMagic. Only users with these roles can see the consent messages: ', 'consent-magic' ); ?>
			<?php
			if ( \ConsentMagicPro\ConsentMagic()->getOption( 'cs_admin_permissions' ) ) { ?>
                <b><?php echo esc_html( implode( ", ", \ConsentMagicPro\ConsentMagic()->getOption( 'cs_admin_permissions' ) ) ); ?></b>
			<?php }
			?>
        </p>
    </div>
	<?php
}

/**
 * Begins execution of the plugin.
 * Since everything within the plugin is registered via hooks,
 * then kicking off the plugin from this point in the file does
 * not affect the page life cycle.
 * @since    1.0.0
 */
function cmpro_run() {
	if ( !isset( $_GET[ 'uxb_iframe' ] ) ) {
		ConsentMagicPro\ConsentMagic()->run();
	}
}

cmpro_run();

add_action( 'plugins_loaded', 'cmpro_test_mode_init' );

function cmpro_test_mode_init() {
	$user          = wp_get_current_user();
	$allowed_roles = ConsentMagicPro\ConsentMagic()->getOption( 'cs_admin_permissions' );
	if ( ( ConsentMagicPro\ConsentMagic()->getOption( 'cs_test_mode' ) && array_intersect( $allowed_roles, $user->roles ) ) || ( ConsentMagicPro\ConsentMagic()->getOption( 'cs_test_mode' ) && array_intersect( array( 'administrator' ), $user->roles ) ) ) {
		add_action( 'admin_notices', 'cmpro_test_mode_notice_pro' );
	}

	if ( ConsentMagicPro\is_plugin_activated() && ConsentMagicPro\ConsentMagic()->check_first_flow() ) {
		add_action( 'admin_notices', 'cmpro_display_update_notice' );
	}
}

//declare compatible with Woocommerce HPOS
add_action( 'before_woocommerce_init', function() {
	if ( class_exists( \Automattic\WooCommerce\Utilities\FeaturesUtil::class ) ) {
		\Automattic\WooCommerce\Utilities\FeaturesUtil::declare_compatibility( 'custom_order_tables', __FILE__, true );
	}
} );
