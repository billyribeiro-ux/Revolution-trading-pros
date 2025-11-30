<?php

/**
 * Fired during plugin deactivation.
 *
 * This class defines all code necessary to run during the plugin's deactivation.
 *
 * @package    ConsentMagic
 * @subpackage ConsentMagic/includes
 */

namespace ConsentMagicPro;

class CS_Uninstall {

	public static function deactivation( $license = true ): void {
		global $wpdb;
		if ( ConsentMagic()->getOption( 'cs_deactivation_db_clear' ) == 1 ) {
			// Remove settings:

			$table = $wpdb->prefix . 'cs_options';
			if ( !$license ) {
				$column = 'name';
				$options = array(
					'edd_consent_magic_pro',
					'edd_consent_magic_pro_activated',
					'edd_license_key',
					'edd_consent_magic_pro_deactivate_checkbox',
					'wc_am_product_id_consent_magic_pro',
					'wc_am_client_consent_magic_pro',
					'wc_am_client_consent_magic_pro_activated',
					'wc_am_client_consent_magic_pro_deactivate_checkbox',
					'wc_am_client_consent_magic_pro_instance',
				);

				$placeholders = implode( ',', array_fill( 0, count( $options ), '%s' ) );

				$sql = $wpdb->prepare(
					"DELETE FROM $table WHERE $column NOT IN ($placeholders)",
					$options
				);
			} else {
				$sql = $wpdb->prepare( 'DROP TABLE IF EXISTS %1$s;', $table );
			}
			$wpdb->query( $sql );

			$args = array();
			$args[ 'posts_per_page' ] = -1;
			$args[ 'numberposts' ] = -1;

			// Remove custom meta:
			$args [ 'post_type' ] = CMPRO_POST_TYPE;
			$posts = get_posts( $args );
			if ( $posts ) {
				foreach ( $posts as $post ) {
					wp_delete_post( $post->ID, true );
				}
			}

			// Remove custom meta:
			$args [ 'post_type' ] = CMPRO_TEMPLATE_POST_TYPE;
			$posts = get_posts( $args );
			if ( $posts ) {
				foreach ( $posts as $post ) {
					wp_delete_post( $post->ID, true );
				}
			}

			// Remove custom meta:
			$args [ 'post_type' ] = CMPRO_POST_TYPE_COOKIES;
			$posts = get_posts( $args );
			if ( $posts ) {
				foreach ( $posts as $post ) {
					wp_delete_post( $post->ID, true );
				}
			}

			// Remove custom meta:
			$args [ 'post_type' ] = CMPRO_POST_TYPE_SCRIPTS;
			$posts = get_posts( $args );
			if ( $posts ) {
				foreach ( $posts as $post ) {
					wp_delete_post( $post->ID, true );
				}
			}

			// Remove script categories
			$args = array(
				'taxonomy'   => 'cs-cookies-category',
				'hide_empty' => false,
			);
			$categories = get_terms( $args );
			if ( $categories ) {
				foreach ( $categories as $category ) {
					wp_delete_term( $category->term_id, 'cs-cookies-category' );
				}
			}

			$tables = array(
				$wpdb->prefix . 'cs_statistics_consent',
				$wpdb->prefix . 'cs_stats_consent',
				$wpdb->prefix . 'cs_proof_consent',
				$wpdb->prefix . 'cs_scan_cookies',
				$wpdb->prefix . 'cs_scan',
				$wpdb->prefix . 'cs_scan_scripts',
				$wpdb->prefix . 'cs_unblock_ip',
				$wpdb->prefix . 'cs_translations',
			);

			foreach ( $tables as $table ) {
				$sql = $wpdb->prepare( 'DROP TABLE IF EXISTS %1$s;', $table );
				$wpdb->query( $sql );
			}
		}

		db_cron_deactivate();
	}
}
