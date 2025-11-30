<?php

/**
 * Fired during plugin activation.
 * This class defines all code necessary to run during the plugin's activation.
 * @package    ConsentMagic
 * @subpackage ConsentMagic/includes
 */

namespace ConsentMagicPro;

class CS_Activator {

	public $main_tb            = 'cs_proof_consent';
	public $statistics_tb      = 'cs_stats_consent';
	public $unblock_ip_list_tb = 'cs_unblock_ip';

	/**
	 * @since    1.0.0
	 */
	public function activate() {
		global $wpdb;
		require_once ABSPATH . 'wp-admin/includes/upgrade.php';
		if ( is_multisite() ) {
			// Get all blogs in the network and activate plugin on each one
			$blog_ids = $wpdb->get_col( $wpdb->prepare( 'SELECT blog_id FROM %1$s', $wpdb->blogs ) );
			foreach ( $blog_ids as $blog_id ) {
				switch_to_blog( $blog_id );
				$this->install_tables();
				restore_current_blog();
				ConsentMagic()->updateOptions( array( 'cs_plugin_do_activation_redirect' => true ) );
			}
		} else {
			$this->install_tables();
			ConsentMagic()->updateOptions( array( 'cs_plugin_do_activation_redirect' => true ) );
		}
		if ( !ConsentMagic()->getOption( 'cs_autodetect_email' ) ) {
			ConsentMagic()->updateOptions(  array( 'cs_autodetect_email' => get_bloginfo( 'admin_email' ) ) );
		}
		if ( !ConsentMagic()->getOption( 'cs_send_important_emails' ) ) {
			ConsentMagic()->updateOptions( array( 'cs_send_important_emails' => array( get_bloginfo( 'admin_email' ) ) ) );
		}
	}

	/**
	 * Install necessary tables
	 */
	public function install_tables() {
		global $wpdb;
		require_once ABSPATH . 'wp-admin/includes/upgrade.php';

		//creating main table ========================
		$table_name   = $wpdb->prefix . $this->main_tb;
		$like         = '%' . $wpdb->esc_like( $table_name ) . '%';
		$search_query = $wpdb->prepare( "SHOW TABLES LIKE %s", $like );
		if ( !$wpdb->get_results( $search_query, ARRAY_N ) ) {
			$create_table_sql = $wpdb->prepare( 'CREATE TABLE `%1$s`(
			    `uuid` INT NOT NULL AUTO_INCREMENT,
			    `created_at` DATETIME NOT NULL DEFAULT CURRENT_TIMESTAMP,
			    `updated_at` DATETIME NOT NULL DEFAULT CURRENT_TIMESTAMP,
			    `url` TEXT NOT NULL,
			    `ip` TEXT NOT NULL,
			    `email` TEXT NOT NULL,
			    `profile` TEXT NOT NULL,
			    `rule` TEXT NOT NULL,
			    `consent_type` VARCHAR(50) NOT NULL,
			    `category` LONGTEXT NOT NULL,
			    `current_action` VARCHAR(50) NOT NULL,
			    PRIMARY KEY(`uuid`)
			);', $table_name );
			dbDelta( $create_table_sql );
		}

		//creating statistics table ========================
		$table_name_statistics = $wpdb->prefix . $this->statistics_tb;
		$like                  = '%' . $wpdb->esc_like( $table_name_statistics ) . '%';
		$search_query          = $wpdb->prepare( "SHOW TABLES LIKE %s", $like );
		if ( !$wpdb->get_results( $search_query, ARRAY_N ) ) {
			$create_table_sql = $wpdb->prepare( 'CREATE TABLE `%1$s`(
			    `id` INT NOT NULL AUTO_INCREMENT,
			    `uuid` INT NOT NULL,
			    `current_action` VARCHAR(50) NOT NULL,
			    `rule` TEXT NOT NULL,
			    `rule_id` TEXT NOT NULL,
			    `consent_type` VARCHAR(50) NOT NULL,
			    `created_at` DATETIME NOT NULL DEFAULT CURRENT_TIMESTAMP,
			    PRIMARY KEY(`id`)
			);', $table_name_statistics );
			dbDelta( $create_table_sql );
		}

		//creating unblock users ip list table ========================
		$table_name_ip = $wpdb->prefix . $this->unblock_ip_list_tb;
		$like          = '%' . $wpdb->esc_like( $table_name_ip ) . '%';
		$search_query  = $wpdb->prepare( "SHOW TABLES LIKE %s", $like );
		if ( !$wpdb->get_results( $search_query, ARRAY_N ) ) {
			$create_table_sql = $wpdb->prepare( 'CREATE TABLE `%1$s`(
			    `id` INT NOT NULL AUTO_INCREMENT,
			    `ip` VARCHAR(250) NOT NULL,
			    `created_at` DATETIME NOT NULL DEFAULT CURRENT_TIMESTAMP,
			    PRIMARY KEY(`id`)
			);', $table_name_ip );
			dbDelta( $create_table_sql );
		}

		$charset_collate = $wpdb->get_charset_collate();

		$table_name   = $wpdb->prefix . 'cs_scan';
		$like         = '%' . $wpdb->esc_like( $table_name ) . '%';
		$search_query = $wpdb->prepare( "SHOW TABLES LIKE %s", $like );

		if ( !$wpdb->get_results( $search_query, ARRAY_N ) ) {
			$create_table_sql = "CREATE TABLE `$table_name`(
			    `id_cs_scan` INT NOT NULL AUTO_INCREMENT,
			    `status` INT NOT NULL DEFAULT '0',
			    `created_at` DATETIME NOT NULL DEFAULT CURRENT_TIMESTAMP,
			    `total_url` INT NOT NULL DEFAULT '0',
			    `total_cookies` INT NOT NULL DEFAULT '0',
			    `total_scripts` INT NOT NULL DEFAULT '0',
			    `current_action` VARCHAR(50) NOT NULL,
			    `current_offset` INT NOT NULL DEFAULT '0',
			    PRIMARY KEY(`id_cs_scan`)
            ) $charset_collate;";

			dbDelta( $create_table_sql );
		}

		// Creates a table to store all the scanned cookies.
		$table_name   = $wpdb->prefix . 'cs_scan_cookies';
		$like         = '%' . $wpdb->esc_like( $table_name ) . '%';
		$search_query = $wpdb->prepare( "SHOW TABLES LIKE %s", $like );

		if ( !$wpdb->get_results( $search_query, ARRAY_N ) ) {
			$create_table_sql = "CREATE TABLE `$table_name`(
			    `id_cs_scan_cookies` INT NOT NULL AUTO_INCREMENT,
			    `id_cs_scan` INT NOT NULL DEFAULT '0',
			    `cookie_enabled` INT NOT NULL DEFAULT '0',
			    `cookie_name` VARCHAR(255) NOT NULL,
			    `cookie_parent_cat_key` VARCHAR(85) NOT NULL,
			    `expiry` VARCHAR(255) NOT NULL,
			    `type` VARCHAR(255) NOT NULL,
			    `category` VARCHAR(255) NOT NULL,
                `category_id` INT NOT NULL,
                `description` TEXT NULL DEFAULT '',
                `created_at` DATETIME NOT NULL DEFAULT CURRENT_TIMESTAMP,
			    PRIMARY KEY(`id_cs_scan_cookies`),
			    UNIQUE `cookie` (`cookie_name`)
            )";

			dbDelta( $create_table_sql . ' ' . $charset_collate );
		}

		// Creates a table to store all the scanned scripts.
		$table_name   = $wpdb->prefix . 'cs_scan_scripts';
		$like         = '%' . $wpdb->esc_like( $table_name ) . '%';
		$search_query = $wpdb->prepare( "SHOW TABLES LIKE %s", $like );

		if ( !$wpdb->get_results( $search_query, ARRAY_N ) ) {
			$create_table_sql = "CREATE TABLE `$table_name`(
			    `id_cs_scan_scripts` INT NOT NULL AUTO_INCREMENT,
			    `id_cs_scan` INT NOT NULL DEFAULT '0',
			    `script_name` VARCHAR(85) NOT NULL,
			    `script_slug` VARCHAR(85) NOT NULL,
			    `script_parent_cat_key` VARCHAR(85) NOT NULL,
                `category_id` INT NOT NULL,
                `script_body` VARCHAR(255) NOT NULL,
                `description` TEXT NULL DEFAULT '',
                `script_enabled` TINYINT NOT NULL,
                `created_at` DATETIME NOT NULL DEFAULT CURRENT_TIMESTAMP,
			    PRIMARY KEY(`id_cs_scan_scripts`),
			    UNIQUE `scripts` (`script_name`)
            )";

			dbDelta( $create_table_sql . ' ' . $charset_collate );
		}

		// Creates a table to store all the options.
		$table_name   = $wpdb->prefix . 'cs_options';
		$like         = '%' . $wpdb->esc_like( $table_name ) . '%';
		$search_query = $wpdb->prepare( "SHOW TABLES LIKE %s", $like );

		if ( !$wpdb->get_results( $search_query, ARRAY_N ) ) {
			$create_table_sql = "CREATE TABLE `$table_name`(
			    `id` INT NOT NULL AUTO_INCREMENT,
			    `name` VARCHAR(100) NOT NULL,
			    `value` MEDIUMTEXT NULL DEFAULT '',
			    `design` TINYINT(1) NOT NULL DEFAULT 0, 
                `updated_at` DATETIME NOT NULL DEFAULT CURRENT_TIMESTAMP,
			     PRIMARY KEY(`id`),
                 UNIQUE `name` (`name`)
            )";

			dbDelta( $create_table_sql . ' ' . $charset_collate );
		}

		// Creates a table to store translations.
		$table_name   = $wpdb->prefix . 'cs_translations';
		$like         = '%' . $wpdb->esc_like( $table_name ) . '%';
		$search_query = $wpdb->prepare( "SHOW TABLES LIKE %s", $like );

		if ( !$wpdb->get_results( $search_query, ARRAY_N ) ) {
			$create_table_sql = "CREATE TABLE `$table_name`(
			    `id` INT NOT NULL AUTO_INCREMENT,
			    `option` VARCHAR(100) NOT NULL,
			    `language` VARCHAR(10) NOT NULL,
			    `value` TEXT NULL DEFAULT '',
			    `category` TINYINT(1) NOT NULL DEFAULT 0, 
			    `category_name` VARCHAR(100) DEFAULT '', 
			    `meta` TINYINT(1) NOT NULL DEFAULT 0, 
			    `meta_id` INT NOT NULL DEFAULT '0',
                `updated_at` DATETIME NOT NULL DEFAULT CURRENT_TIMESTAMP,
			     PRIMARY KEY(`id`),
			     UNIQUE KEY `option_language` (`option`, `language`, `meta_id`)
            )";

			dbDelta( $create_table_sql . ' ' . $charset_collate );
			ConsentMagic()->loadDefaultTranslations();
		}
	}
}
