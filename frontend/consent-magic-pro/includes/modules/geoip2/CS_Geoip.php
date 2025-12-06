<?php

namespace ConsentMagicPro;

if ( !defined( 'ABSPATH' ) ) {
	exit; // Exit if accessed directly.
}

use MaxMind\Db\Reader;
use MaxMind\Db\Reader\InvalidDatabaseException;

class CS_Geoip {

	public $version;

	public $download_url;

	public $databases = array();

	public $database_extension;

	public function __construct() {

		include_once ABSPATH . 'wp-admin/includes/plugin.php';

		$this->databases          = array(
			'city'    => 'GeoLite2-City',
			'country' => 'GeoLite2-Country'
		);
		$this->download_url       = 'https://download.maxmind.com/app/geoip_download';
		$this->database_extension = '.mmdb';

	}

	public function cs_check_country() {

		$ipAddress           = get_client_ip();
		$databaseCountryFile = CMPRO_PLUGIN_PATH . 'includes/modules/geoip2/GeoLite2-Country.mmdb';
		$Country             = false;

		if ( file_exists( $databaseCountryFile ) && !empty( $ipAddress ) ) {

			try {
				$reader       = new Reader( $databaseCountryFile );
				$Country_data = $reader->get( $ipAddress );
				if ( isset( $Country_data[ 'country' ][ 'iso_code' ] ) ) {
					$Country = $Country_data[ 'country' ][ 'iso_code' ];
				} elseif ( isset( $Country_data[ 'registered_country' ][ 'iso_code' ] ) ) {
					$Country = $Country_data[ 'registered_country' ][ 'iso_code' ];
				}
				$reader->close();

			} catch ( InvalidDatabaseException $e ) {
				error_log( $e->getMessage() );
				$Country = false;
				db_cron_update_one_time();
			} catch ( \BadMethodCallException|\Exception $e ) {
				error_log( $e->getMessage() );
				$Country = false;
			}
		}

		return $Country;
	}

	public function get_state() {

		$ipAddress        = get_client_ip();
		$databaseCityFile = CMPRO_PLUGIN_PATH . 'includes/modules/geoip2/GeoLite2-City.mmdb';
		$State_code       = false;
		if ( file_exists( $databaseCityFile ) && !empty( $ipAddress ) ) {

			try {
				$reader     = new Reader( $databaseCityFile );
				$State_data = $reader->get( $ipAddress );
				$State_code = $State_data[ 'subdivisions' ][ 0 ][ 'iso_code' ] ?? false;
				$reader->close();

			} catch ( InvalidDatabaseException $e ) {
				error_log( $e->getMessage() );
				$State_code = false;
				db_cron_update_one_time();
			} catch ( \BadMethodCallException|\Exception $e ) {
				error_log( $e->getMessage() );
				$State_code = false;
			}
		}

		return $State_code;
	}

	public function cs_validate_geo_license_key( $key, $account_id ) {

		$key        = trim( stripslashes( $key ) );
		$account_id = trim( stripslashes( $account_id ) );

		// if empty license keys return WP_Error.
		if ( empty( $key ) ) {
			return new \WP_Error( 'geo_error', esc_html__( 'License key or Account ID is empty.', 'consent-magic' ) );
		}

		// Check the license key by attempting to download the Geolocation database.
		$tmp_database_path = $this->cs_download_database( $key, $account_id, $this->databases[ 'country' ] );
		if ( is_wp_error( $tmp_database_path ) ) {
			return $tmp_database_path;
		}

		// Downloading DB
		$tmp_database_path = $this->update_database( $this->databases[ 'country' ], $tmp_database_path );

		return $tmp_database_path;
	}

	//download maxmindDB when settings saved
	public function cs_download_database( $key, $account_id, $database ) {

		$tmp_database_path = null;

		if ( is_plugin_activated() && ConsentMagic()->check_first_flow() ) {

			if ( strlen( $key ) > 17 ) {
				if ( is_callable( 'curl_init' ) ) {
					require_once ABSPATH . 'wp-admin/includes/file.php';
					$tmp_archive_path = wp_tempnam();
					$error            = false;
					try {
						$ch = curl_init();
						$fp = fopen( $tmp_archive_path, 'w+' );

						curl_setopt( $ch, CURLOPT_URL, "https://download.maxmind.com/geoip/databases/$database/download?suffix=tar.gz" );
						curl_setopt( $ch, CURLOPT_RETURNTRANSFER, 1 );
						curl_setopt( $ch, CURLOPT_CUSTOMREQUEST, 'GET' );
						curl_setopt( $ch, CURLOPT_FILE, $fp );
						curl_setopt( $ch, CURLOPT_FOLLOWLOCATION, true );
						curl_setopt( $ch, CURLOPT_USERPWD, sanitize_text_field( $account_id ) . ':' . sanitize_text_field( $key ) );

						$result   = curl_exec( $ch );
						$httpCode = curl_getinfo( $ch, CURLINFO_HTTP_CODE );
						if ( curl_errno( $ch ) || $httpCode === 404 ) {
							$error = __( 'Failed to download the MaxMind database.', 'consent-magic' );
						} elseif ( $httpCode === 401 ) {
							$error = __( 'The MaxMind license key or Account ID is invalid.', 'consent-magic' );
						} else {
							$archive = new \Archive_Tar( $tmp_archive_path );
							if ( $archive->extract( dirname( $tmp_archive_path ) ) ) {

								// List the contents of the archive
								$files = $archive->listContent();
								if ( !empty( $files ) ) {
									// Get the filename of the first file in the archive
									$filename = $files[ 0 ][ 'filename' ]; // Get the filename from the first entry

									// Construct the tmp_database_path
									$tmp_database_path = trailingslashit( dirname( $tmp_archive_path ) ) . trailingslashit( $filename ) . $database . $this->database_extension;;
								}
							}
						}
						curl_close( $ch );

					} catch ( \Exception $exception ) {
						error_log( $exception->getMessage() );
					} finally {
						// Remove the archive since we only care about a single file in it.
						unlink( $tmp_archive_path );
					}

					if ( $error !== false ) {
						return new \WP_Error( 'geo_error', esc_html__( $error, 'consent-magic' ) );
					}

				}
			} else {
				$download_uri = add_query_arg( array(
					'edition_id'  => $database,
					'license_key' => urlencode( sanitize_text_field( $key ) ),
					'suffix'      => 'tar.gz',
				), $this->download_url );

				// Needed for the download_url call right below.
				require_once ABSPATH . 'wp-admin/includes/file.php';

				$tmp_archive_path = download_url( esc_url_raw( $download_uri ) );
				if ( is_wp_error( $tmp_archive_path ) ) {
					// Transform the error into something more informative.
					$error_data = $tmp_archive_path->get_error_data();
					if ( isset( $error_data[ 'code' ] ) ) {
						switch ( $error_data[ 'code' ] ) {
							case 401:
								return new \WP_Error( 'geo_error', esc_html__( 'The MaxMind license key is invalid.', 'consent-magic' ) );
							default:
								return new \WP_Error( 'geo_error', esc_html__( 'Failed to download the MaxMind database.', 'consent-magic' ) );
						}
					}

					return new \WP_Error( 'geo_error', esc_html__( 'Failed to download the MaxMind database.', 'consent-magic' ) );
				}

				// Extract the database from the archive.
				try {
					$archive = new \Archive_Tar( $tmp_archive_path );
					if ( $archive->extract( dirname( $tmp_archive_path ) ) ) {

						// List the contents of the archive
						$files = $archive->listContent();
						if ( !empty( $files ) ) {
							// Get the filename of the first file in the archive
							$filename = $files[ 0 ][ 'filename' ]; // Get the filename from the first entry

							// Construct the tmp_database_path
							$tmp_database_path = trailingslashit( dirname( $tmp_archive_path ) ) . trailingslashit( $filename ) . $database . $this->database_extension;;
						}
					}
				} catch ( \Exception $exception ) {
					error_log( $exception->getMessage() );
				} finally {
					// Remove the archive since we only care about a single file in it.
					unlink( $tmp_archive_path );
				}
			}

		} else {
			return new \WP_Error( 'geo_error', esc_html__( 'Consent Magic license is invalid.', 'consent-magic' ) );
		}

		if ( $tmp_database_path === null ) {
			return new \WP_Error( 'geo_error', esc_html__( 'Error while downloading the MaxMind database.', 'consent-magic' ) );
		}

		return $tmp_database_path;
	}

	public function update_database( $database, $new_database_path = null ) {
		// Allow us to easily interact with the filesystem.
		require_once ABSPATH . 'wp-admin/includes/file.php';
		if ( !WP_Filesystem() ) {
			WP_Filesystem();
		}
		global $wp_filesystem;

		$target_database_path = trailingslashit( CMPRO_PLUGIN_PATH ) . trailingslashit( ConsentMagic()->getOption( 'cs_geo_db_file_path' ) ) . $database . $this->database_extension;

		// If there's no database path, we can't store the database.
		if ( empty( $target_database_path ) ) {
			return new \WP_Error( 'geo_error', esc_html__( 'Database path is empty.', 'consent-magic' ) );
		}

		if ( $wp_filesystem->exists( $target_database_path ) ) {
			$wp_filesystem->delete( $target_database_path );
		}

		if ( isset( $new_database_path ) ) {
			$tmp_database_path = $new_database_path;
		} else {
			// We can't download a database if there's no license key configured.
			$license_key = ConsentMagic()->getOption( 'cs_geo_licence_key' );
			$account_id  = ConsentMagic()->getOption( 'cs_geo_licence_account_id' );
			if ( empty( $license_key ) ) {
				return new \WP_Error( 'geo_error', esc_html__( 'License key or Account ID is empty.', 'consent-magic' ) );
			}

			$tmp_database_path = $this->cs_download_database( $license_key, $account_id, $database );
			if ( is_wp_error( $tmp_database_path ) ) {
				return $tmp_database_path;
			}
		}

		// Move the new database into position.
		$wp_filesystem->move( $tmp_database_path, $target_database_path, true );
		$wp_filesystem->delete( dirname( $tmp_database_path ), true );

		return true;
	}

	public function check_databases() {
		if ( get_transient( 'cm_geo_check_databases' ) ) {
			return;
		}

		if ( $this->check_database_paths() ) {
			db_cron_update_one_time();
			set_transient( 'cm_geo_check_databases', true, 3600 );
		}

	}

	public function check_database_paths() {
		foreach ( $this->databases as $database ) {
			$database_path = CMPRO_PLUGIN_PATH . "includes/modules/geoip2/$database{$this->database_extension}";
			if ( !file_exists( $database_path ) ) {
				return true;
			}
		}

		return false;
	}

}

new CS_Geoip();