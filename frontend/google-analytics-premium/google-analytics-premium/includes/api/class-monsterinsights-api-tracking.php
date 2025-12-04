<?php
/**
 * Tracking API Client class for MonsterInsights.
 *
 * @since 8.0.0
 *
 * @package MonsterInsights
 */

class MonsterInsights_API_Tracking extends MonsterInsights_API_Client {
	/**
	 * Constructor.
	 */
	public function __construct() {
		$this->base_url = apply_filters( 'monsterinsights_api_url_tracking', 'https://api.monsterinsights.com/v2/tracking' );
		parent::__construct();
	}
} 