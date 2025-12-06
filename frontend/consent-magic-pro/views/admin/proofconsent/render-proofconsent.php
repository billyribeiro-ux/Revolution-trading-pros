<?php

namespace ConsentMagicPro;

// If this file is called directly, abort.
if ( !defined( 'WPINC' ) ) {
	die;
}

function render_proof_global_stats_data_head() {
	?>
    }
    <tr>
        <th><?php esc_html_e( 'Action', "consent-magic" ); ?></th>
        <th><?php esc_html_e( 'Count', "consent-magic" ); ?></th>
    </tr>
	<?php
}

function render_proof_cs_type_stats_data_head() {
	?>
    }
    <tr>
        <th><?php esc_html_e( 'Consent type', "consent-magic" ); ?></th>
        <th><?php esc_html_e( 'Allow All', "consent-magic" ); ?></th>
        <th><?php esc_html_e( 'Disallow All', "consent-magic" ); ?></th>
        <th><?php esc_html_e( 'Close on scroll', "consent-magic" ); ?></th>
        <th><?php esc_html_e( 'Confirm choices', "consent-magic" ); ?></th>
    </tr>
	<?php
}

function render_proof_rules_stats_data_head() {
	?>
    }
    <tr>
        <th><?php esc_html_e( 'Rule name', "consent-magic" ); ?></th>
        <th><?php esc_html_e( 'Allow All', "consent-magic" ); ?></th>
        <th><?php esc_html_e( 'Disallow All', "consent-magic" ); ?></th>
        <th><?php esc_html_e( 'Close on scroll', "consent-magic" ); ?></th>
        <th><?php esc_html_e( 'Confirm choices', "consent-magic" ); ?></th>
    </tr>
	<?php
}