<?php

namespace ConsentMagicPro;

// If this file is called directly, abort.
if ( !defined( 'WPINC' ) ) {
	die;
}

?>

<div class="cards-wrapper cards-wrapper-style2 statistics gap-8">
    <div class="card card-static card-style9">
        <div class="card-body">
            <div class="gap-24">
                <h3 class="primary-heading-type2"><?php esc_html_e( 'Global stats', 'consent-magic' ); ?></h3>
                <div class="line"></div>

				<?php
				$messages = ConsentMagic()->getOption( 'cs_proof_show_count' );
				if ( !$messages ) {
					$messages = 0;
				}
				?>

                <p class="primary-heading-type2"><?php esc_html_e( 'Total messages shown:', 'consent-magic' ); ?>
					<?php echo esc_attr( $messages ); ?></p>

                <div>
                    <h4 class="font-semibold-type2 mb-8"><?php esc_html_e( 'Date range filters', "consent-magic" ); ?></h4>

                    <div class="data-range-search">
                        <div class="select-wrap select-daterange-wrap">
                            <div class="select-daterange datepicker_global">
                                <span></span>
                            </div>
                        </div>

                        <button type="button" id="btn_search_global" class="btn btn-primary btn-primary-type2">
							<?php esc_html_e( 'Search', "consent-magic" ); ?>
                        </button>
                    </div>
                </div>

                <div class="cm-table-wrap">
                    <table id="proof_global_stats_data" class="display cm-table">
                        <thead>
                        <tr>
                            <th><?php esc_html_e( 'Action', "consent-magic" ); ?></th>
                            <th><?php esc_html_e( 'Count', "consent-magic" ); ?></th>
                        </tr>
                        </thead>
                        <tfoot>
                        <tr>
                            <th><?php esc_html_e( 'Action', "consent-magic" ); ?></th>
                            <th><?php esc_html_e( 'Count', "consent-magic" ); ?></th>
                        </tr>
                        </tfoot>
                    </table>
                </div>
            </div>
        </div>
    </div>

    <div class="card card-static card-style9">
        <div class="card-body">
            <div class="gap-24">
                <h3 class="primary-heading-type2"><?php esc_html_e( 'Stats by Consent Type', 'consent-magic' ); ?></h3>
                <div class="line"></div>

                <div>
                    <h4 class="font-semibold-type2 mb-8"><?php esc_html_e( 'Date range filters', "consent-magic" ); ?></h4>

                    <div class="data-range-search">
                        <div class="select-wrap select-daterange-wrap">
                            <div class="select-daterange datepicker_consent_type">
                                <span></span>
                            </div>
                        </div>

                        <button type="button" id="btn_search_consent_type" class="btn btn-primary btn-primary-type2">
							<?php esc_html_e( 'Search', "consent-magic" ); ?>
                        </button>
                    </div>
                </div>

                <div class="cm-table-wrap">
                    <table id="proof_cs_type_stats_data" class="display cm-table">
                        <thead>
                        <tr>
                            <th><?php esc_html_e( 'Consent type', "consent-magic" ); ?></th>
                            <th><?php esc_html_e( 'Allow All', "consent-magic" ); ?></th>
                            <th><?php esc_html_e( 'Disallow All', "consent-magic" ); ?></th>
                            <th><?php esc_html_e( 'Close on scroll', "consent-magic" ); ?></th>
                            <th><?php esc_html_e( 'Confirm choices', "consent-magic" ); ?></th>
                            <th><?php esc_html_e( 'Close consent', "consent-magic" ); ?></th>
                            <th><?php esc_html_e( 'Close Options Popup', "consent-magic" ); ?></th>
                        </tr>
                        </thead>
                        <tfoot>
                        <tr>
                            <th><?php esc_html_e( 'Consent type', "consent-magic" ); ?></th>
                            <th><?php esc_html_e( 'Allow All', "consent-magic" ); ?></th>
                            <th><?php esc_html_e( 'Disallow All', "consent-magic" ); ?></th>
                            <th><?php esc_html_e( 'Close on scroll', "consent-magic" ); ?></th>
                            <th><?php esc_html_e( 'Confirm choices', "consent-magic" ); ?></th>
                            <th><?php esc_html_e( 'Close consent', "consent-magic" ); ?></th>
                            <th><?php esc_html_e( 'Close Options Popup', "consent-magic" ); ?></th>
                        </tr>
                        </tfoot>
                    </table>
                </div>
            </div>
        </div>
    </div>

    <div class="card card-static card-style9">
        <div class="card-body">
            <div class="gap-24">
                <h3 class="primary-heading-type2"><?php esc_html_e( 'Stats by rule', 'consent-magic' ); ?></h3>
                <div class="line"></div>

                <div>
                    <h4 class="font-semibold-type2 mb-8"><?php esc_html_e( 'Date range filters', "consent-magic" ); ?></h4>

                    <div class="data-range-search">
                        <div class="select-wrap select-daterange-wrap">
                            <div class="select-daterange datepicker_rule">
                                <span></span>
                            </div>
                        </div>

                        <button type="button" id="btn_search_rule" class="btn btn-primary btn-primary-type2">
							<?php esc_html_e( 'Search', "consent-magic" ); ?>
                        </button>
                    </div>
                </div>

                <div class="cm-table-wrap">
                    <table id="proof_rules_stats_data" class="display cm-table">
                        <thead>
                        <tr>
                            <th><?php esc_html_e( 'Rule name', "consent-magic" ); ?></th>
                            <th><?php esc_html_e( 'Allow All', "consent-magic" ); ?></th>
                            <th><?php esc_html_e( 'Disallow All', "consent-magic" ); ?></th>
                            <th><?php esc_html_e( 'Close on scroll', "consent-magic" ); ?></th>
                            <th><?php esc_html_e( 'Confirm choices', "consent-magic" ); ?></th>
                            <th><?php esc_html_e( 'Close consent', "consent-magic" ); ?></th>
                            <th><?php esc_html_e( 'Close Options Popup', "consent-magic" ); ?></th>
                        </tr>
                        </thead>
                        <tfoot>
                        <tr>
                            <th><?php esc_html_e( 'Rule name', "consent-magic" ); ?></th>
                            <th><?php esc_html_e( 'Allow All', "consent-magic" ); ?></th>
                            <th><?php esc_html_e( 'Disallow All', "consent-magic" ); ?></th>
                            <th><?php esc_html_e( 'Close on scroll', "consent-magic" ); ?></th>
                            <th><?php esc_html_e( 'Confirm choices', "consent-magic" ); ?></th>
                            <th><?php esc_html_e( 'Close consent', "consent-magic" ); ?></th>
                            <th><?php esc_html_e( 'Close Options Popup', "consent-magic" ); ?></th>
                        </tr>
                        </tfoot>
                    </table>
                </div>
            </div>
        </div>
    </div>
</div>