<?php

namespace ConsentMagicPro;

/**
 * The CS_Logger class appears to be a logging utility class, responsible for handling log data for the ConsentMagic plugin.
 */
class CS_Logger {

	private static $_instance = null;
	private        $log_path  = '';

	/**
	 * $_instance CS_Logger
	 * @return CS_Logger|null
	 */
	public static function instance() {
		if ( is_null( self::$_instance ) ) {
			self::$_instance = new self();
		}

		return self::$_instance;
	}

	public function __construct() {
		$this->log_path = CMPRO_PLUGIN_PATH . 'logs/cm-debug.log';

		$this->maybe_download_log();
		$this->maybe_delete_log();
	}

	public function log( ...$data ) {
		if ( !file_exists( $this->log_path ) ) {
			$dir = dirname( $this->log_path );

			if ( !is_dir( $dir ) ) {
				mkdir( $dir, 0777, true );
			}

			touch( $this->log_path );
			chmod( $this->log_path, 0666 );
		}

		$timestamp     = date( "Y-m-d H:i:s" );
		$formattedData = array_map( fn ( $arg ) => print_r( $arg, true ), $data );
		$logEntry      = "[$timestamp] " . implode( " - ", $formattedData ) . "\n";
		file_put_contents( $this->log_path, $logEntry, FILE_APPEND | LOCK_EX );
	}

	public function maybe_download_log() {
		if ( isset( $_GET[ 'cm-download-log' ] ) && file_exists( $this->log_path ) ) {
			// Prevent caching
			header( 'Cache-Control: no-store, no-cache, must-revalidate, max-age=0' );
			header( 'Cache-Control: post-check=0, pre-check=0', false );
			header( 'Pragma: no-cache' );
			header( 'Expires: 0' );

			// Set content headers
			header( 'Content-Type: text/plain; charset=utf-8' );
			header( 'Content-Disposition: attachment; filename="debug-' . date('Y-m-d-H-i-s') . '.log"' );
			header( 'Content-Length: ' . filesize( $this->log_path ) );

			// Add ETag based on file modification time to ensure fresh content
			header( 'ETag: "' . md5( filemtime( $this->log_path ) . filesize( $this->log_path ) ) . '"' );

			// Output file content
			readfile( $this->log_path );
			exit;
		}
	}

	public function maybe_delete_log() {
		if ( isset( $_GET[ 'cm-delete-log' ] ) && file_exists( $this->log_path ) ) {
			unlink( $this->log_path );
		}
	}
}


function CS_Logger() {
	return CS_Logger::instance();
}

CS_Logger();