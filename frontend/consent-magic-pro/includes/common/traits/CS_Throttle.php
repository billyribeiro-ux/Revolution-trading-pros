<?php

namespace ConsentMagicPro;

if ( !defined( 'ABSPATH' ) ) {
	exit; // Exit if accessed directly.
}

trait CS_Throttle {

	private int $limit    = 3;
	private int $interval = 5;

	/**
	 * Set throttle limit
	 */
	public function setThrottleLimit( int $limit ): void {
		$this->limit = $limit;
	}

	/**
	 * Set throttle interval
	 */
	public function setThrottleInterval( int $interval ): void {
		$this->interval = $interval;
	}

	/**
	 * Check limit. Return true if can continue, false if exceeded.
	 */
	public function checkThrottle( string $action_key ): bool {
		$user_key    = is_user_logged_in() ? 'user_' . get_current_user_id() : 'ip_' . get_client_ip();
		$storage_key = 'cm_throttle_' . md5( $action_key . '_' . $user_key );

		$count = get_transient( $storage_key );

		if ( $count === false ) {
			set_transient( $storage_key, 1, $this->interval );

			return true;
		}

		if ( $count < $this->limit ) {
			set_transient( $storage_key, $count + 1, $this->interval );

			return true;
		}

		return false;
	}
}
