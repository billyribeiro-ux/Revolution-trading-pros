<?php

namespace ConsentMagicPro;

// If this file is called directly, abort.
if ( !defined( 'WPINC' ) ) {
	die;
}

function is_plugin_activated() {
	if ( ConsentMagic()->getOption( 'wc_am_client_consent_magic_pro_activated' ) == 'Activated' || ConsentMagic()->getOption( 'wc_am_client_consent_magic_pro_activated' ) == 'Expired' || ConsentMagic()->getOption( 'edd_consent_magic_pro_activated' ) == 'Activated' || ConsentMagic()->getOption( 'edd_consent_magic_pro_activated' ) == 'Expired' ) {
		return true;
	} else {
		return false;
	}
}

function getCurrentAdminTab( $default = 'cs-general' ) {
	if ( !empty( $_GET[ 'tab' ] ) ) {
		return sanitize_text_field( $_GET[ 'tab' ] );
	}

	return $default;
}

function isPYSActivated() {

	if ( !function_exists( 'is_plugin_active' ) ) {
		include_once ABSPATH . 'wp-admin/includes/plugin.php';
	}

	if ( is_plugin_active( 'pixelyoursite-pro/pixelyoursite-pro.php' ) || is_plugin_active( 'pixelyoursite/facebook-pixel-master.php' ) ) {
		return true;
	}

	return false;
}

function isPYSProActivated() {

	if ( !function_exists( 'is_plugin_active' ) ) {
		include_once ABSPATH . 'wp-admin/includes/plugin.php';
	}

	if ( is_plugin_active( 'pixelyoursite-pro/pixelyoursite-pro.php' ) ) {
		return true;
	}

	return false;
}

function isPYSFreeActivated() {

	if ( !function_exists( 'is_plugin_active' ) ) {
		include_once ABSPATH . 'wp-admin/includes/plugin.php';
	}

	if ( is_plugin_active( 'pixelyoursite/facebook-pixel-master.php' ) ) {
		return true;
	}

	return false;
}


function isPinterestActive() {

	if ( !function_exists( 'is_plugin_active' ) ) {
		include_once ABSPATH . 'wp-admin/includes/plugin.php';
	}

	if ( is_plugin_active( 'pixelyoursite-pinterest/pixelyoursite-pinterest.php' ) ) {
		return true;
	}

	return false;
}

function isBingActive() {

	if ( !function_exists( 'is_plugin_active' ) ) {
		include_once ABSPATH . 'wp-admin/includes/plugin.php';
	}

	if ( is_plugin_active( 'pixelyoursite-bing/pixelyoursite-bing.php' ) ) {
		return true;
	}

	return false;
}

/**
 * Check if Reddit plugin is active.
 * @return bool
 */
function isRedditActive(): bool {

	if ( !function_exists( 'is_plugin_active' ) ) {
		include_once ABSPATH . 'wp-admin/includes/plugin.php';
	}

	if ( is_plugin_active( 'pixelyoursite-reddit/pixelyoursite-reddit.php' ) ) {
		return true;
	}

	return false;
}

function isCookiebotActivated() {

	if ( !function_exists( 'is_plugin_active' ) ) {
		include_once ABSPATH . 'wp-admin/includes/plugin.php';
	}

	if ( is_plugin_active( 'cookiebot/cookiebot.php' ) ) {
		return true;
	}

	return false;

}

function isFbPixelActivated() {

	if ( !function_exists( 'is_plugin_active' ) ) {
		include_once ABSPATH . 'wp-admin/includes/plugin.php';
	}

	if ( is_plugin_active( 'official-facebook-pixel/facebook-for-wordpress.php' ) ) {
		return true;
	}

	return false;
}

function isFbWooActivated() {

	if ( !function_exists( 'is_plugin_active' ) ) {
		include_once ABSPATH . 'wp-admin/includes/plugin.php';
	}

	if ( is_plugin_active( 'facebook-for-woocommerce/facebook-for-woocommerce.php' ) ) {
		return true;
	}

	return false;
}

/**
 * Check if WooCommerce plugin installed and activated.
 * @return bool
 */
function isWooCommerceActive() {
	return class_exists( 'woocommerce' );
}

/**
 * Check if Easy Digital Downloads plugin installed and activated.
 * @return bool
 */
function isEddActive() {
	return function_exists( 'EDD' );
}

function getAvailableUserRoles() {

	$wp_roles   = new \WP_Roles();
	$user_roles = array();

	foreach ( $wp_roles->get_names() as $slug => $name ) {
		$user_roles[ $slug ] = $name;
	}

	return $user_roles;
}

function isConversionExporterActivated() {

	if ( !function_exists( 'is_plugin_active' ) ) {
		include_once ABSPATH . 'wp-admin/includes/plugin.php';
	}

	if ( is_plugin_active( 'conversion-exporter/conversion-exporter.php' ) ) {
		return true;
	}

	return false;
}

function manageAdminPermissions() {
	global $wp_roles;

	$roles = ConsentMagic()->getOption( 'cs_admin_permissions' );

	foreach ( $wp_roles->roles as $role => $options ) {

		if ( in_array( $role, $roles ) ) {
			$wp_roles->add_cap( $role, 'manage_cs' );
		} else {
			$wp_roles->remove_cap( $role, 'manage_cs' );
		}
	}
}

/**
 * Clean variables using sanitize_text_field. Arrays are cleaned recursively.
 * Non-scalar values are ignored.
 * @param string|array $var
 * @return string|array
 */
function deepSanitizeTextField( $var ) {

	if ( is_array( $var ) ) {
		return array_map( 'deepSanitizeTextField', $var );
	} else {
		return is_scalar( $var ) ? sanitize_text_field( $var ) : $var;
	}
}

/**
 * Generate tab head for General page.
 * method will translate the string to current language
 */
function generate_settings_tabhead( $title_arr, $page = 'consent-magic', $default = 'cs-general' ) {
	$out_arr = array();
	foreach ( $title_arr as $k => $v ) {
		$out_arr[ $k ] = $v;
		if ( $k == 'cs-buttons' ) {
			//tab head for modules
			$out_arr = apply_filters( 'cs_module_settings_tabhead', $out_arr );
		}
	} ?>

    <nav class="nav cm-nav-tabs">
		<?php
		foreach ( $out_arr as $k => $v ) {
			if ( is_array( $v ) ) {
				$v = ( isset( $v[ 2 ] ) ? $v[ 2 ] : '' ) . $v[ 0 ] . ' ' . ( isset( $v[ 1 ] ) ? $v[ 1 ] : '' );
			}
			if ( $page == 'consent-magic' && $k == getCurrentAdminTab( $default ) ) {
				$class = 'active';
			} else {
				$class = '';
			}
			$admin_url = is_plugin_activated() ? buildAdminUrl( $page, $k ) : admin_url( 'admin.php?page=cs-license' );
			?>
            <a class="cm-nav-tab cm-nav-link <?php echo esc_attr( $class ); ?>"
               href="<?php echo esc_url( $admin_url ); ?>"><?php echo esc_attr( $v ); ?></a>
			<?php
		}
		?>
    </nav>
	<?php
}

/**
 * Generate tab head for General page mobile.
 * method will translate the string to current language
 */
function generate_settings_tabhead_mobile( $title_arr, $page = 'consent-magic', $default = 'cs-general' ) {
	$out_arr = array();
	foreach ( $title_arr as $k => $v ) {
		$out_arr[ $k ] = $v;
		if ( $k == 'cs-buttons' ) {
			//tab head for modules
			$out_arr = apply_filters( 'cs_module_settings_tabhead', $out_arr );
		}
	}

	$current_tab = getCurrentAdminTab( $default );
	$active      = isset( $out_arr[ $current_tab ] );
	?>

    <button class="cm-dropdown-toggle">
        <span class="cm-current-menu <?php echo esc_attr( $active ? 'active' : '' ); ?>"><?php echo esc_html( $out_arr[ $current_tab ] ?? $out_arr[ 'cs-general' ] ); ?></span>
        <i class="icon-chevron-down"></i>
    </button>

    <ul class="cm-dropdown-menu">
		<?php
		foreach ( $out_arr as $k => $v ) {
			if ( is_array( $v ) ) {
				$v = ( isset( $v[ 2 ] ) ? $v[ 2 ] : '' ) . $v[ 0 ] . ' ' . ( isset( $v[ 1 ] ) ? $v[ 1 ] : '' );
			}
			if ( $page == 'consent-magic' && $k == $current_tab ) {
				$class = 'active';
			} else {
				$class = '';
			}
			$admin_url = is_plugin_activated() ? buildAdminUrl( $page, $k ) : admin_url( 'admin.php?page=cs-license' );
			?>
            <li class="<?php echo esc_attr( $class ); ?>"><a
                        href="<?php echo esc_url( $admin_url ); ?>"><?php echo esc_attr( $v ); ?></a><?php echo wp_kses_post( $class == 'active' ? '<i class="icon-check"></i>' : '' ); ?>
            </li>
			<?php
		}
		?>
    </ul>
	<?php
}

function buildAdminUrl( $page, $tab = '', $action = '', $extra = array() ) {

	$args = array( 'page' => $page );

	if ( $tab ) {
		$args[ 'tab' ] = $tab;
	}

	if ( $action ) {
		$args[ 'action' ] = $action;
	}

	$args = array_merge( $args, $extra );

	return add_query_arg( $args, admin_url( 'admin.php' ) );
}

function get_custom_theme_list() {
	$args = array(
		'post_type'      => CMPRO_TEMPLATE_POST_TYPE,
		'posts_per_page' => -1,
		'orderby'        => 'meta_value',
		'meta_key'       => 'cs_main_template'
	);

	$options = array();

	$query = new \WP_Query( $args );

	while ( $query->have_posts() ) {
		$query->the_post();
		$options[] = array(
			'option_value' => get_the_ID(),
			'option_name'  => esc_html( get_the_title() )
		);
	}

	return $options;
}

function get_custom_scripts_list( $searchByFromdate_sql = null, $searchByTodate_sql = null ) {

	$args = get_list_query_args( CMPRO_POST_TYPE_SCRIPTS, $searchByFromdate_sql, $searchByTodate_sql );

	$options = array();

	$query = new \WP_Query( $args );

	while ( $query->have_posts() ) {
		$query->the_post();
		$term_list          = wp_get_object_terms( get_the_ID(), 'cs-cookies-category' );
		$option_name        = 'cs_' . sanitize_title( get_the_title() ) . '_' . get_the_ID() . '_script_enable';
		$option_select_name = 'cs_' . get_the_ID() . '_script_cat';
		$buttons            = '<div class="custom-scripts-actions table_right_aligned"><a href="' . esc_url( get_admin_url( null, "admin.php?page=consent-magic" ) . "&script_id=" . get_the_ID() ) . '" class="button-link link-edit">' . __( 'Edit', 'consent-magic' ) . '</a>
					<a href="#" class="button-link link-delete delete_custom_post" data-cat="' . $option_select_name . '" data-id="' . get_the_ID() . '">' . __( 'Delete', 'consent-magic' ) . '</a></div>
				';
		$unassigned         = get_term_by( 'slug', 'unassigned', 'cs-cookies-category' );
		$unassigned_id      = $unassigned->term_id;
		$options_terms      = get_cookies_terms_list( $unassigned_id );
		$options[]          = array(
			'script_name'        => get_the_title(),
			'script_id'          => get_the_ID(),
			'term_id'            => $term_list[ 0 ]->term_id,
			'term_slug'          => $term_list[ 0 ]->slug,
			'option_name'        => $option_name,
			'option_select_name' => $option_select_name,
			'buttons'            => $buttons,
			'cat_select'         => ConsentMagic()->getOption( $option_select_name ),
			'options_terms'      => $options_terms,
			'checked'            => checked( ConsentMagic()->getOption( $option_name ), true, false )
		);
	}

	return $options;
}

function get_scan_cookies_list() {

	$cs_scanner    = new CS_Scanner;
	$scan_cookies  = $cs_scanner->get_scan_cookies( 0, 0 );
	$options_terms = get_cookies_terms_list();
	$options       = array();
	if ( $scan_cookies ) {
		foreach ( $scan_cookies as $cookie ) {
			$option_select_name = 'cs[custom_cookie_cat][' . $cookie[ 'id_cs_scan_cookies' ] . ']';
			$option_name        = 'cs[custom_enable][' . $cookie[ 'id_cs_scan_cookies' ] . ']';
			$disabled           = ( ( strpos( $cookie[ 'description' ], 'Default wordpress cookie' ) !== false && $cookie[ 'category' ] === 'necessary' ) || $cookie[ 'type' ] === 'pys' ) ? 'disabled' : '';
			$options[]          = array(
				'cookie_name'        => $cookie[ 'cookie_name' ],
				'cookie_id'          => $cookie[ 'id_cs_scan_cookies' ],
				'term_id'            => $cookie[ 'category_id' ],
				'term_slug'          => $cookie[ 'category' ],
				'term'               => ucfirst( $cookie[ 'category' ] ),
				'option_name'        => $option_name,
				'option_select_name' => $option_select_name,
				'cat_select'         => get_scan_cookie_data( $cookie[ 'id_cs_scan_cookies' ], 'category' ),
				'options_terms'      => $options_terms,
				'checked'            => checked( get_scan_cookie_data( $cookie[ 'id_cs_scan_cookies' ], 'cookie_enabled' ), true, false ),
				'description'        => $cookie[ 'description' ],
				'disabled'           => $disabled,
				'details_button'     => cardCollapseSettingsWithText( 'Details', 'Collapse', false, true )
			);
		}
	}

	return $options;
}

function get_scan_scripts_list() {
	$options = array();

	if ( !class_exists( 'CS_Scanner' ) ) {
		require_once CMPRO_PLUGIN_PATH . '/includes/modules/scanner/CS_Scanner_Module.php';
	}
	$cs_scanner   = new CS_Scanner;
	$scan_scripts = $cs_scanner->get_scan_scripts( 0, 0 );

	if ( $scan_scripts ) {
		foreach ( $scan_scripts as $script ) {
			if ( $script[ 'script_body' ] !== 'custom' ) {
				$options[] = array(
					'script_name'   => $script[ 'script_name' ],
					'script_id'     => $script[ 'id_cs_scan_scripts' ],
					'term'          => get_term( $script[ 'category_id' ] )->name,
					'script_body'   => $script[ 'script_body' ],
					'description'   => $script[ 'description' ],
					'script_button' => '<a href="#" data-target="pre-defined-scripts-bar" class="open_target_tab link table_link">' . esc_html__( 'Pre-Defined Script', 'consent-magic' ) . '</a>'
				);
			} else {
				$script_obj = get_post_by_title( $script[ 'script_name' ], 'cs-scripts' );
				if ( $script_obj ) {
					$script_id = $script_obj->ID;
				} else {
					$script_id = get_term_by( 'slug', 'unassigned', 'cs-cookies-category' )->term_id;
				}
				$options[] = array(
					'script_name'   => $script[ 'script_name' ],
					'script_id'     => $script[ 'id_cs_scan_scripts' ],
					'term'          => get_term( $script[ 'category_id' ] )->name,
					'script_body'   => $script[ 'script_body' ],
					'description'   => $script[ 'description' ],
					'script_button' => '<a href="' . get_admin_url( null, 'admin.php?page=consent-magic' ) . '&script_id=' . esc_attr( $script_id ) . '" class="link table_link">' . esc_html__( 'Custom Script', 'consent-magic' ) . '</a>'
				);
			}
		}
	}

	return $options;
}

function get_post_by_title( $page_title, $post_type = 'post', $output = OBJECT ) {
	global $wpdb;
	$post = $wpdb->get_var( $wpdb->prepare( "SELECT ID FROM $wpdb->posts WHERE post_title = %s AND post_type=%s", $page_title, $post_type ) );
	if ( $post ) {
		return get_post( $post, $output );
	}

	return null;
}

function get_scan_cookie_data( $id, $key ) {
	global $wpdb;

	$cookies_table = $wpdb->prefix . 'cs_scan_cookies';
	$sql           = $wpdb->prepare( 'SELECT `%1$s` FROM %2$s WHERE `id_cs_scan_cookies` = %3$s ORDER BY id_cs_scan_cookies DESC LIMIT 1', $key, $cookies_table, $id );
	$enabled       = $wpdb->get_results( $sql, ARRAY_A );

	if ( !empty( $enabled ) ) {
		return $enabled[ 0 ][ $key ];
	} else {
		return false;
	}
}

function update_scan_cookie_data( $id, $data_arr ) {
	global $wpdb;

	$cookies_table = $wpdb->prefix . 'cs_scan_cookies';

	$sql = $wpdb->prepare( 'SELECT * FROM %1$s WHERE id_cs_scan_cookies=%2$d', $cookies_table, $id );

	$cookie = $wpdb->get_row( $sql, ARRAY_A );
	if ( !empty( $cookie ) && $cookie[ 'type' ] == 'pys' ) {
		return false;
	}

	if ( $wpdb->update( $cookies_table, $data_arr, array( 'id_cs_scan_cookies' => $id ) ) ) { // phpcs:ignore WordPress.DB.DirectDatabaseQuery,WordPress.DB.PreparedSQL.NotPrepared
		return true;
	} else {
		return false;
	}
}

function update_scan_script_data( $id, $data_arr ) {
	global $wpdb;

	$scripts_table = $wpdb->prefix . 'cs_scan_scripts';

	if ( $wpdb->update( $scripts_table, $data_arr, array( 'id_cs_scan_scripts' => $id ) ) ) { // phpcs:ignore WordPress.DB.DirectDatabaseQuery,WordPress.DB.PreparedSQL.NotPrepared
		return true;
	} else {
		return false;
	}
}

function get_list_query_args( $postType, $searchByFromdate_sql = null, $searchByTodate_sql = null ) {
	$args = array(
		'tax_query'      => [
			[
				'taxonomy' => 'cs-cookies-category',
				'operator' => 'EXISTS',
			],
		],
		'post_type'      => $postType,
		'posts_per_page' => -1
	);

	if ( $searchByFromdate_sql && $searchByTodate_sql ) {
		$searchBy       = explode( "-", $searchByFromdate_sql );
		$searchByTodate = explode( "-", $searchByTodate_sql );
		$args           = array(
			'tax_query'      => [
				[
					'taxonomy' => 'cs-cookies-category',
					'operator' => 'EXISTS',
				],
			],
			'date_query'     => array(
				array(
					'after'     => array(
						'year'  => $searchBy[ 0 ],
						'month' => $searchBy[ 1 ],
						'day'   => $searchBy[ 2 ],
					),
					'before'    => array(
						'year'  => $searchByTodate[ 0 ],
						'month' => $searchByTodate[ 1 ],
						'day'   => $searchByTodate[ 2 ],
					),
					'inclusive' => true,
				)
			),
			'post_type'      => $postType,
			'posts_per_page' => -1
		);
	}

	return $args;
}

/**
 * Render existing pages array
 */
function get_existing_pages() {

	$args = array(
		'post_status'  => array(
			'publish',
			'pending',
			'draft',
			'auto-draft',
			'future',
			'private',
			'inherit'
		),
		'sort_order'   => 'ASC',
		'sort_column'  => 'post_title',
		'hierarchical' => 0,
		'offset'       => 0,
		'post_type'    => 'page'
	);

	$pages = get_pages( $args );

	$options = array();

	foreach ( $pages as $page ) {
		$options[] = array(
			'option_value' => $page->ID,
			'option_name'  => esc_html( $page->post_title ) . ' (ID: ' . $page->ID . ')'
		);
	}

	return $options;
}

/**
 * Render existing pages and posts array
 */
function get_existing_scan_pages() {

	$options = array(
		array(
			'option_value' => 'all',
			'option_name'  => 'All'
		)
	);

	$args = array(
		'post_status' => 'publish',
		'offset'      => 0,
		'nopaging'    => true,
		'post_type'   => 'any'
	);

	$pages = get_posts( $args );

	foreach ( $pages as $page ) {
		array_push( $options, array(
			'option_value' => $page->ID,
			'option_name'  => esc_html( $page->post_title )
		) );
	}

	return $options;
}

function get_meta_value_by_key( $id, $key ) {
	if ( $key == '_cs_type' ) {
		switch ( get_post_meta( $id, $key, true ) ) {
			case 'ask_before_tracking':
				return "Ask before tracking";
				break;
			case 'just_inform':
				return "Just inform";
				break;
			case 'inform_and_opiout':
				return "Inform and Opt-out";
				break;
			case 'iab':
				return "IAB";
				break;
			default:
				return '';
		}
	} else if ( $key == '_cs_bars_type' ) {
		switch ( get_post_meta( $id, $key, true ) ) {
			case 'bar_small':
				return '<p class="purple-label">' . __( 'bar', 'consent-magic' ) . '</p><p class="purple-label">' . __( 'small', 'consent-magic' ) . '</p>';
				break;
			case 'bar_large':
				return '<p class="purple-label">' . __( 'bar', 'consent-magic' ) . '</p><p class="purple-label">' . __( 'large', 'consent-magic' ) . '</p>';
				break;
			case 'popup_small':
				return '<p class="purple-label">' . __( 'popup', 'consent-magic' ) . '</p><p class="purple-label">' . __( 'small', 'consent-magic' ) . '</p>';
				break;
			case 'popup_large':
				return '<p class="purple-label">' . __( 'popup', 'consent-magic' ) . '</p><p class="purple-label">' . esc_html__( 'large', 'consent-magic' ) . '</p>';
				break;
			default:
				return '';
		}
	} else {
		return get_post_meta( $id, $key, true );
	}
}

function get_page_id_by_path( $path, $output, $post_type ) {

	$id = get_page_by_path( $path, $output, $post_type );

	if ( isset( $id->ID ) ) {
		return $id->ID;
	}

	return false;
}

function get_rules_by_meta_key( $meta_key ) {

	$args_posts = array(
		'post_type'      => CMPRO_POST_TYPE,
		'post_status'    => 'publish',
		'meta_query'     => [
			[
				'key'     => $meta_key,
				'compare' => 'EXISTS'
			],
		],
		'posts_per_page' => -1,
		'order'          => 'ASC',
		'meta_key'       => '_cs_order',
		'orderby'        => 'meta_value_num',
		'meta_type'      => 'NUMERIC',
	);
	$loop_posts = new \WP_Query( $args_posts );
	if ( !$loop_posts->have_posts() ) {
		return false;
	} else {
		if ( (int) ConsentMagic()->getOption( 'cs_geolocation' ) === 0 || (int) ConsentMagic()->getOption( 'cs_geo_activated' ) === 0 ) {
			$geo_enabled = false;
		} else {
			$geo_enabled = true;
		}

		while ( $loop_posts->have_posts() ) {
			$loop_posts->the_post();
			$id      = get_the_ID();
			$post_id = get_the_ID();
			include CMPRO_PLUGIN_VIEWS_PATH . 'admin/main/admin-settings-rules-list-wrap.php';
		}
	}
}

function get_post_id_by_slug( $post_slug, $slug_post_type = CMPRO_POST_TYPE ) {

	$post = get_page_by_path( $post_slug, OBJECT, $slug_post_type );

	if ( $post ) {
		return $post->ID;
	} else {
		return null;
	}
}

function get_post_id_by_slug_from_db( $post_slug, $slug_post_type = CMPRO_POST_TYPE ) {

	global $wpdb;
	$id = $wpdb->get_var( $wpdb->prepare( "SELECT ID FROM $wpdb->posts WHERE post_name = %s AND post_type= %s AND post_status = 'publish'", $post_slug, $slug_post_type ) );

	return $id;
}

/**
 * Return cs-cookies-category list
 */
function get_cookies_terms_list( $ignore = false, $array = false ) {

	$terms = get_terms( [
		'taxonomy'   => 'cs-cookies-category',
		'hide_empty' => false,
		'exclude'    => array( $ignore ),
	] );

	if ( empty( $terms ) || !is_array( $terms ) ) {
		$terms = array();
	}

	$terms = CS_Translator()->renderLangOptionsTerm( $terms );

	$options = array();

	foreach ( $terms as $option ) {
		if ( isset( $option->name_l ) && !empty( $option->name_l ) ) {
			$option->name = $option->name_l;
		}

		if ( !$array ) {
			$options[ $option->slug ] = $option->name_l;
		} else {
			$options[] = $option->name_l;
		}
	}

	return $options;
}

/**
 * Return cs-cookies-category objects list
 */
function get_cookies_terms_objects( $sort_by_key = null, $primary = false, $ignore_cat_id = false ) {

	$args = array(
		'taxonomy'   => 'cs-cookies-category',
		'hide_empty' => false,
		'orderby'    => 'name',
		'order'      => 'ASC',
		'meta_key'   => $sort_by_key,
	);

	if ( $ignore_cat_id ) {
		$args[ 'exclude' ] = $ignore_cat_id;
	}
	if ( $sort_by_key && $primary ) {
		$args[ 'meta_value' ] = 'primary';
	} else {
		$args[ 'meta_value' ] = 'custom';
	}

	$terms = get_terms( $args );
	if ( empty( $terms ) ) {
		$terms = get_cookies_terms_objects_query( $args );
	}

	if ( empty( $terms ) || !is_array( $terms ) ) {
		$terms = array();
	}

	return CS_Translator()->renderLangOptionsTerm( $terms );
}

function get_cookies_terms_objects_all( $ignore_cat_id = false ) {

	$args = array(
		'taxonomy'   => 'cs-cookies-category',
		'hide_empty' => false,
		'orderby'    => 'name',
		'order'      => 'ASC',
		'exclude'    => $ignore_cat_id ? array( $ignore_cat_id ) : array(),
	);

	$terms = get_terms( $args );
	if ( empty( $terms ) ) {
		$terms = get_cookies_terms_objects_query( $args );
	}

	if ( empty( $terms ) || !is_array( $terms ) ) {
		$terms = array();
	}

	return CS_Translator()->renderLangOptionsTerm( $terms );
}

/**
 * Get cookies terms using WP_Term_Query if get_terms returns an empty array
 * @param $args
 * @return array
 */
function get_cookies_terms_objects_query( $args ) {
	return ( new \WP_Term_Query( $args ) )->terms;
}

function getCurrentAdminPage() {
	if ( !empty( $_GET[ 'page' ] ) ) {
		return sanitize_text_field( $_GET[ 'page' ] );
	}

	return '';
}

//change cron recurrence for update geolocation
function db_cron_change_recurrence( $reshedule = false ) {
	if ( ConsentMagic()->getOption( 'cs_geolocation' ) && ConsentMagic()->getOption( 'cs_geo_activated' ) && ConsentMagic()->getOption( 'cs_often_update' ) != 'never' ) {
		$update = ConsentMagic()->getOption( 'cs_often_update' );
		if ( $update && $update != 'never' ) {
			db_cron_deactivate();
			if ( $reshedule ) {
				wp_reschedule_event( time(), $update, 'cs_db_cron_update_hook' );
			} else {
				wp_schedule_event( time(), $update, 'cs_db_cron_update_hook' );
			}
		}
	}
}

// unschedule event upon plugin deactivation
function db_cron_deactivate() {
	if ( wp_next_scheduled( 'cs_db_cron_update_hook' ) ) {
		wp_clear_scheduled_hook( 'cs_db_cron_update_hook' );
	}
}

function db_cron_check_status() {
	if ( !wp_next_scheduled( 'cs_db_cron_update_hook' ) ) {
		$update = ConsentMagic()->getOption( 'cs_often_update' );
		if ( $update && $update != 'never' ) {
			wp_schedule_event( time(), $update, 'cs_db_cron_update_hook' );
		}
	}

	return false;
}

// update DB if error reading DB and first time
function db_cron_update_one_time( $first_time = false ) {
	if ( ( !wp_next_scheduled( 'cs_db_cron_update_hook_one_time' ) && wp_next_scheduled( 'cs_db_cron_update_hook' ) > wp_next_scheduled( 'cs_db_cron_update_hook_one_time' ) ) || $first_time ) {
		wp_schedule_single_event( time() + 60, 'cs_db_cron_update_hook_one_time', array( $first_time ) );
	}
}

function create_email_before_delete_consent() {
	$email = sanitize_email( ConsentMagic()->getOption( 'cs_send_proof_deleting_email' ) );
	if ( !empty( $email ) ) {
		global $wpdb;
		$table                  = $wpdb->prefix . 'cs_proof_consent';
		$cs_proof_consent_count = $wpdb->get_var( $wpdb->prepare( 'SELECT COUNT(*) FROM %1$s', $table ) );
		$subject                = 'Consent - ' . current_time( 'mysql' );
		$headers                = array( 'Content-Type: text/html; charset=UTF-8' );
		$message                = '<p>' . sprintf( esc_html__( 'ConsentMagic is configured to delete stored consent when it reaches %s entries. This email confirms consent was deleted on %s', 'consent-magic' ), $cs_proof_consent_count, current_time( 'mysql' ) ) . '</p>' . '<p>' . esc_html__( 'Consent entries are attached to this email.', 'consent-magic' ) . '</p>' . '<p>' . sprintf( esc_html__( 'You can change these settings %shere%s', 'consent-magic' ), "<a href='" . admin_url( 'admin.php?page=cs-proof-consent&tab=cs-settings' ) . "' target='_blank'>", '</a>' ) . '</p>';
		$attachment             = create_csv();
		$sent                   = wp_mail( $email, $subject, $message, $headers, $attachment );
		if ( $sent ) {
			$wpdb->query( $wpdb->prepare( 'DELETE FROM %1$s', $table ) );
			renew_consent_run();
		} else {
			echo 'Message not sent!';
		}
	} else {
		echo 'Message not sent! The e-mail address is incorrect!';
	}
}

function create_csv() {
	global $wpdb;
	$table = $wpdb->prefix . 'cs_proof_consent';

	$step_data = array();

	$export_keys = array(
		'UUID'         => 'uuid',
		'Time'         => 'created_at',
		'URL'          => 'url',
		'IP'           => 'ip',
		'User'         => 'email',
		'User profile' => 'profile',
		'Rule'         => 'rule',
		'Consent type' => 'consent_type',
		'Consents'     => 'category',
		'Action'       => 'current_action',
	);

	clear_export_proof_consent( $export_keys, 'export_proof_consent.csv' );

	$consent_array = $wpdb->get_results( $wpdb->prepare( 'SELECT * FROM %1$s WHERE 1=1 LIMIT 5000', $table ) );

	$array = [];
	foreach ( $consent_array as $item ) {
		$array[ 'UUID' ]         = $item->uuid;
		$array[ 'Time' ]         = $item->created_at;
		$array[ 'URL' ]          = $item->url;
		$array[ 'IP' ]           = $item->ip;
		$array[ 'User' ]         = $item->email;
		$array[ 'User profile' ] = wp_strip_all_tags( $item->profile );
		$array[ 'Rule' ]         = $item->rule;
		$array[ 'Consent type' ] = $item->consent_type;
		$array[ 'Consents' ]     = strip_tags( $item->category );
		$array[ 'Action' ]       = $item->current_action;
		array_push( $step_data, $array );
	}

	return export_proof_consent( $step_data, $export_keys, 'export_proof_consent.csv' );
}

function renew_consent_run() {
	$consent_version = ConsentMagic()->cs_get_consent_version();
	if ( !empty( $consent_version ) ) {
		$consent_version = $consent_version + 1;
		ConsentMagic()->updateOptions( array( 'cs_consent_version' => $consent_version ) );
	}
}

function clear_export_proof_consent( $export_keys, $file_name ) {

	$file     = CMPRO_PLUGIN_PATH . $file_name;
	$file_url = CMPRO_PLUGIN_URL . $file_name;
	# Generate CSV data from array
	$fh = fopen( $file, 'w' );
	# to use memory instead
	# write out the headers
	fputcsv( $fh, $export_keys, ";",'"', '' );
	fclose( $fh );

	return $file_url;
}

function export_proof_consent( $items, $export_keys, $file_name ) {

	$file     = CMPRO_PLUGIN_PATH . $file_name;
	$file_url = CMPRO_PLUGIN_URL . $file_name;
	# Generate CSV data from array
	$fh = fopen( $file, 'a+' );
	# to use memory instead

	# write out the headers
	foreach ( $items as $item ) {
		unset( $csv_line );
		foreach ( $export_keys as $key => $value ) {
			if ( isset( $item[ $key ] ) ) {
				$csv_line[] = $item[ $key ];
			}
		}
		if ( isset( $csv_line ) ) {
			fputcsv( $fh, $csv_line, ";",'"', '' );
		}
	}
	fclose( $fh );

	return $file;
}

function getDataByKey( $inputArray, $inputKey ) {
	$resultArray = [];

	array_walk( $inputArray, function( $item, $key ) use ( &$resultArray, $inputKey ) {
		$resultArray[ $item[ $inputKey ] ][] = $item;
	} );

	return $resultArray;
}

/**
 * Get client IP address
 * @return string
 */
function get_client_ip() {

	$ip = '';
	if ( isset( $_SERVER[ 'HTTP_CF_CONNECTING_IP' ] ) ) {
		$ip = sanitize_text_field( wp_unslash( $_SERVER[ 'HTTP_CF_CONNECTING_IP' ] ) );
	} elseif ( !empty( $_SERVER[ 'HTTP_CLIENT_IP' ] ) ) {
		//ip from share internet
		$ip = sanitize_text_field( $_SERVER[ 'HTTP_CLIENT_IP' ] );
	} elseif ( !empty( $_SERVER[ 'HTTP_X_FORWARDED_FOR' ] ) ) {
		//ip pass from proxy
		$ip = sanitize_text_field( wp_unslash( $_SERVER[ 'HTTP_X_FORWARDED_FOR' ] ) );
	} elseif ( !empty( $_SERVER[ 'REMOTE_ADDR' ] ) ) {
		$ip = sanitize_text_field( wp_unslash( $_SERVER[ 'REMOTE_ADDR' ] ) );
	}

	return explode( ",", $ip )[ 0 ];
}

function get_cs_type_name( $cs_type ) {
	switch ( $cs_type ) {
		case 'just_inform':
			$cs_type_db = 'Just inform';
			break;
		case 'inform_and_opiout':
			$cs_type_db = 'Inform and Opt-out';
			break;
		case 'ask_before_tracking':
			$cs_type_db = 'Ask before tracking';
			break;
		case 'iab':
			$cs_type_db = 'IAB';
			break;
		default:
			$cs_type_db = '';
			break;
	}

	return $cs_type_db;
}

function get_current_action( $current_action ) {
	switch ( $current_action ) {
		case 'allow_all':
			$current_action_db = 'Allow all';
			break;
		case 'disable_all':
			$current_action_db = 'Disable all';
			break;
		case 'cs_confirm':
			$current_action_db = 'Confirm my choices';
			break;
		case 'scroll':
			$current_action_db = 'Close on Scroll';
			break;
		case 'cs_close_consent':
			$current_action_db = 'Close consent';
			break;
		case 'cs_close_opt_popup':
			$current_action_db = 'Close Options Popup';
			break;
		default:
			$current_action_db = '';
			break;
	}

	return $current_action_db;
}

/**
 * Check is Woo Supporting High-Performance Order Storage
 * @return bool
 */
function isWooUseHPStorage() {
	if ( class_exists( \Automattic\WooCommerce\Utilities\OrderUtil::class ) ) {
		return \Automattic\WooCommerce\Utilities\OrderUtil::custom_orders_table_usage_is_enabled();
	}

	return false;

}

function db_cron_update() {
	if ( ConsentMagic()->getOption( 'cs_geolocation' ) && ConsentMagic()->getOption( 'cs_geo_activated' ) && ConsentMagic()->getOption( 'cs_often_update' ) != 'never' ) {
		if ( is_plugin_activated() && ConsentMagic()->check_first_flow() ) {
			$update = ConsentMagic()->getOption( 'cs_often_update' );
			if ( $update && $update != 'never' ) {
				if ( !wp_next_scheduled( 'cs_db_cron_update_hook' ) ) {
					wp_schedule_event( time(), $update, 'cs_db_cron_update_hook' );
				}
			}
		}
	} else {
		db_cron_deactivate();
	}

	return false;
} // end db_cron_update()

function generate_name( $length = 36 ) {
	$characters        = 'abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ';
	$characters_length = strlen( $characters );
	$random_string     = '';
	for ( $i = 0; $i < $length; $i++ ) {
		$random_string .= $characters[ random_int( 0, $characters_length - 1 ) ];
	}

	return $random_string;
}

/**
 * Is supreme modules pro for divi active
 * @return bool
 */
function is_active_supreme_modules_pro_for_divi() {

	if ( !function_exists( 'is_plugin_active' ) ) {
		include_once ABSPATH . 'wp-admin/includes/plugin.php';
	}

	if ( is_plugin_active( 'supreme-modules-pro-for-divi/supreme-modules-pro-for-divi.php' ) ) {
		return true;
	}

	return false;
}

/**
 * Get text editor fields
 * @return string[]
 */
function get_text_editor_fields() {
	return array(
		'cs_text_in_small_bar_popup',
		'cs_text_in_large_bar_popup',
		'cs_text_in_options_popup',
		'cs_video_consent_general_text',
		'cs_video_consent_rule_text',
		'cs_advanced_matching_description',
		'cs_server_side_consent_description',
		'cs_default_script_desc',
		'_cs_text_in_small_bar_popup',
		'_cs_text_in_large_bar_popup',
		'_cs_text_in_options_popup',
	);
}

function replace_text_for_partners( $text, $lang, $count, $classes = '' ) {

	if ( $count === 0 ) {
		return $text;
	}

	$search = array(
		'en_US' => array(
			'search'  => 'our partners',
			'replace' => 'our %s partners',
		),
		'bg_BG' => array(
			'search'  => 'нашите партньори',
			'replace' => 'нашите %s партньори',
		),
		'cs_CZ' => array(
			'search'  => 'naši partneři',
			'replace' => 'naši %s partneři',
		),
		'da_DK' => array(
			'search'  => 'vores partnere',
			'replace' => 'vores %s partnere',
		),
		'de_DE' => array(
			'search'  => 'unsere Partner',
			'replace' => 'unsere %s Partner',
		),
		'el'    => array(
			'search'  => 'οι συνεργάτες μας',
			'replace' => '%s οι συνεργάτες μας',
		),
		'es_ES' => array(
			'search'  => 'nuestros socios',
			'replace' => 'nuestros %s socios',
		),
		'et'    => array(
			'search'  => 'meie partnerid',
			'replace' => 'meie %s partnerid',
		),
		'fi'    => array(
			'search'  => 'kumppanimme käytämme',
			'replace' => 'kumppanimme %s käytämme',
		),
		'fr_FR' => array(
			'search'  => 'nos partenaires',
			'replace' => 'nos %s partenaires',
		),
		'hr'    => array(
			'search'  => 'naši partneri',
			'replace' => 'naši %s partneri',
		),
		'hu_HU' => array(
			'search'  => 'partnereink információkat',
			'replace' => '%s partnereink információkat',
		),
		'it_IT' => array(
			'search'  => 'nostri partner',
			'replace' => 'nostri %s partner',
		),
		'lt_LT' => array(
			'search'  => 'mūsų partneriai',
			'replace' => 'mūsų %s partneriai',
		),
		'lv'    => array(
			'search'  => 'mūsu partneri',
			'replace' => 'mūsu %s partneri',
		),
		'mt'    => array(
			'search'  => 'partijiet konnessi',
			'replace' => '%s partijiet konnessi',
		),
		'nl_NL' => array(
			'search'  => 'onze partners',
			'replace' => 'onze %s partners',
		),
		'pl_PL' => array(
			'search'  => 'nasi partnerzy',
			'replace' => 'nasi %s partnerzy',
		),
		'pt_PT' => array(
			'search'  => 'nossos parceiros',
			'replace' => 'nossos %s parceiros',
		),
		'ro_RO' => array(
			'search'  => 'partenerii noștri',
			'replace' => '%s partenerii noștri',
		),
		'sk_SK' => array(
			'search'  => 'naši partneri',
			'replace' => 'naši %s partneri',
		),
		'sl_SI' => array(
			'search'  => 'naši partnerji',
			'replace' => 'naši %s partnerji',
		),
		'sv_SE' => array(
			'search'  => 'våra partners',
			'replace' => 'våra %s partners',
		),
		'uk'    => array(
			'search'  => 'наші партнери',
			'replace' => 'наші %s партнери',
		),
	);

	if ( isset( $search[ $lang ] ) ) {
		$replace = sprintf( $search[ $lang ][ 'replace' ], $count );
		$text    = str_replace( $search[ $lang ][ 'search' ], "<a class='cs_open_vendors $classes'>$replace</a>", $text );
	}

	return $text;
}

/**
 * Save the image on the server.
 * @param $base64_img
 * @param $title
 * @return int|\WP_Error
 */
function save_image( $base64_img, $title ) {

	$upload_dir  = wp_upload_dir();
	$upload_path = str_replace( '/', DIRECTORY_SEPARATOR, $upload_dir[ 'path' ] ) . DIRECTORY_SEPARATOR;


	$mime            = substr( $base64_img, 11, 4 );
	$mime            = str_replace( ';', '', $mime );
	$img             = str_replace( 'data:image/' . $mime . ';base64,', '', $base64_img );
	$img             = str_replace( ' ', '+', $img );
	$decoded         = base64_decode( $img );
	$filename        = $title . '.' . $mime;
	$file_type       = 'image/' . $mime;
	$hashed_filename = hash( 'sha256', $filename . microtime() ) . '_' . $filename;

	// Save the image in the uploads' directory.
	$upload_file = file_put_contents( $upload_path . $hashed_filename, $decoded );

	$attachment = array(
		'post_mime_type' => $file_type,
		'post_title'     => preg_replace( '/\.[^.]+$/', '', basename( $hashed_filename ) ),
		'post_content'   => '',
		'post_status'    => 'inherit',
		'guid'           => $upload_dir[ 'url' ] . '/' . basename( $hashed_filename )
	);

	$attach_id = wp_insert_attachment( $attachment, $upload_dir[ 'path' ] . '/' . $hashed_filename );
	wp_update_attachment_metadata( $attach_id, wp_generate_attachment_metadata( $attach_id, $upload_dir[ 'path' ] . '/' . $hashed_filename ) );

	return $attach_id;
}

/**
 * Sanitize url for saving
 * @param $needle
 * @return array|mixed|string|string[]|null
 */
function cs_sanitize_url( $needle ) {
	if ( $needle !== null ) {
		$patterns = array(
			'/^(?:https?)?:?[\\|\/]*(?:www.)?/',
			'/[\\|\/]*$/'
		);
		$needle   = preg_replace( $patterns, '', $needle );
	}

	return $needle;
}

function is_translatepress_active() {
	if ( !function_exists( 'is_plugin_active' ) ) {
		include_once ABSPATH . 'wp-admin/includes/plugin.php';
	}

	if ( is_plugin_active( 'translatepress-multilingual/index.php' ) ) {
		return true;
	}

	return false;
}

/**
 * Get all server IPs
 * @return string
 */
function getAllServerIps(): string {
	$ips = [];

	// 1. Get external/public IP using an external service
	$external_ip = @file_get_contents( 'https://api64.ipify.org?format=json' );
	if ( $external_ip ) {
		$decoded = json_decode( $external_ip, true );
		if ( !empty( $decoded[ 'ip' ] ) ) {
			$ips[] = $decoded[ 'ip' ];
		}
	}

	// 2. Get IP from the server's hostname
	$hostname = gethostname();
	if ( $hostname ) {
		$hostname_ip = gethostbyname( $hostname );
		if ( filter_var( $hostname_ip, FILTER_VALIDATE_IP ) ) {
			$ips[] = $hostname_ip;
		}

		// 3. Get all IPs associated with the hostname
		$multiple_ips = gethostbynamel( $hostname );
		if ( is_array( $multiple_ips ) ) {
			foreach ( $multiple_ips as $ip ) {
				if ( filter_var( $ip, FILTER_VALIDATE_IP ) ) {
					$ips[] = $ip;
				}
			}
		}
	}

	// 4. Get IP resolved from the server name (e.g., domain)
	if ( !empty( $_SERVER[ 'SERVER_NAME' ] ) ) {
		$server_name_ip = gethostbyname( $_SERVER[ 'SERVER_NAME' ] );
		if ( filter_var( $server_name_ip, FILTER_VALIDATE_IP ) ) {
			$ips[] = $server_name_ip;
		}
	}

	// 5. Get the IP address of the interface the server is running on
	if ( !empty( $_SERVER[ 'SERVER_ADDR' ] ) && filter_var( $_SERVER[ 'SERVER_ADDR' ], FILTER_VALIDATE_IP ) ) {
		$ips[] = $_SERVER[ 'SERVER_ADDR' ];
	}

	// Remove duplicates and return as a comma-separated string
	$unique_ips = array_unique( $ips );

	return implode( ', ', $unique_ips );
}