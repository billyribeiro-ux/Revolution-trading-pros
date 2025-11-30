<?php

namespace ConsentMagicPro;

// If this file is called directly, abort.
if ( !defined( 'WPINC' ) ) {
	die;
}

$p_id             = $id;
$metas            = get_post_meta( $id );
$cs_bars_position = $metas[ '_cs_bars_position' ][ 0 ];
$cs_sticky        = ( $metas[ '_cs_sticky' ][ 0 ] == 1 ) ? 'sticky' : 'not-sticky';
$cs_bar_type      = $metas[ '_cs_bars_type' ][ 0 ];

$enabled = get_post_meta( $p_id, '_cs_enable_rule', true ) == '1'
?>

<div class="rules-wrap <?php echo $enabled ? '' : 'disabled';
echo ' ' . ( $geo_enabled ? '' : 'one-rule' ); ?>" id="<?php echo esc_attr( $p_id ); ?>"
     data-id="<?php echo esc_attr( $p_id ); ?>">
    <div class="card card-style2">
        <div class="card-header card-header-style1 disable-card-wrap sortable-header">
            <div class="cm-sortable-header rule-item">
                <div>
                    <i class="icon-drag-drop"></i>
                </div>

                <div class="rule-actions-wrap">
                    <div class="enable_rule">
						<?php render_several_switcher_input( '_cs_enable_rule', false, true, $p_id ); ?>
                        <h4 class="secondary-heading switcher-label"><?php esc_attr_e( get_the_title( $p_id ) ); ?></h4>
                    </div>

                    <div class="rule-actions">
                        <div class="rule-actions-buttons">
                            <div class="rule-enabled-actions action-buttons">
								<?php
								if ( !$geo_enabled ) : ?>
                                    <a href="<?php echo admin_url( 'admin.php?page=cs-geolocation' ); ?>"
                                       class="rule-location-message link text-extra-small fw-500"><?php esc_html_e( 'Configure geolocation to use multiple rules', 'consent-magic' ); ?></a>
								<?php endif;
								?>

                                <a href="<?php echo esc_url( get_admin_url( null, 'admin.php?page=consent-magic' ) . '&primary_rule_id=' . $p_id ); ?>"
                                   class="btn btn-gray-type2 with-small-icon btn-primary-type2"><i
                                            class="icon-edit"></i>
                                    <span>
	                                    <?php esc_html_e( 'Edit', 'consent-magic' ); ?>
                                    </span>
                                </a>
                                <div class="preview_list_action">
                                    <button type="button"
                                            class="btn btn-gray-type2 btn-primary-type2 with-small-icon preview_list_action_btn"
                                            data-id="<?php echo esc_attr( $p_id ); ?>"
                                            data-template="<?php echo esc_attr( $cs_bar_type ); ?>"><i
                                                class="icon-view"></i>
                                        <span>
                                            <?php esc_html_e( 'Preview', 'consent-magic' ); ?>
                                        </span>
                                    </button>
									<?php include CMPRO_PLUGIN_VIEWS_PATH . "admin/components/spinner.php"; ?>
                                </div>

								<?php if ( get_post_meta( $p_id, '_cs_predefined_rule', true ) == 0 ) { ?>
                                    <button type="button" data-id="<?php echo esc_attr( $p_id ); ?>"
                                            class="btn btn-red btn-primary-type2 with-small-icon custom_rule_delete_btn">
                                        <i class="icon-delete"></i>
                                        <span>
										    <?php esc_html_e( 'Delete', 'consent-magic' ); ?>
                                        </span>
                                    </button>
								<?php } ?>
                            </div>

                            <div class="rule-disabled-actions">
                                <p class="text-gray text-extra-small fw-500">
									<?php esc_html_e( 'Disabled', 'consent-magic' ); ?>
                                </p>
                            </div>
                        </div>
                    </div>
                </div>

				<?php cardCollapseSettings(); ?>
            </div>
        </div>

        <div class="card-body">
            <div class="rule-item-info gap-24">
                <div class="description-block">
                    <h4 class="font-semibold mr-6"><?php esc_html_e( 'Target:', 'consent-magic' ); ?></h4>
                    <p>
						<?php
						if ( get_meta_value_by_key( $p_id, '_cs_target' ) ) {
							$all_locations      = renderCSTypeLocations();
							$all_locations_keys = array_keys( $all_locations );
							$gdpr_countries     = array();
							$non_gdpr_countries = array();

							foreach ( $all_locations as $key => $location ) {
								if ( $location[ 1 ] === 'GDPR' ) {
									$gdpr_countries[] = $key;
								} else if ( $location[ 1 ] === 'NONGDPR' ) {
									$non_gdpr_countries[] = $key;
								}
							}

							$locations       = explode( ',', get_meta_value_by_key( $p_id, '_cs_target' ) );
							$ldu_states_keys = array_keys( renderCSTypeUSStates() );
							$states          = explode( ',', get_meta_value_by_key( $p_id, '_cs_us_states_target' ) );
							$ldu_countries   = array( 'US' );

							$key = array_search( 'GDPR', $locations );
							if ( $key !== false ) {
								unset( $locations[ $key ] );
							}
							$key = array_search( 'NONGDPR', $locations );
							if ( $key !== false ) {
								unset( $locations[ $key ] );
							}

							if ( empty( array_diff( $states, $ldu_states_keys ) ) && empty( array_diff( $ldu_states_keys, $states ) ) && empty( array_diff( $locations, $ldu_countries ) ) && empty( array_diff( $ldu_countries, $locations ) ) ) {
								$result = 'Limited Data Use (LDU)';
							} elseif ( empty( array_diff( $locations, $gdpr_countries ) ) && empty( array_diff( $gdpr_countries, $locations ) ) ) {
								$result = 'GDPR countries';
							} elseif ( empty( array_diff( $locations, $non_gdpr_countries ) ) && empty( array_diff( $non_gdpr_countries, $locations ) ) ) {
								$result = 'Non-GDPR countries';
							} elseif ( empty( array_diff( $locations, $all_locations_keys ) ) && empty( array_diff( $all_locations_keys, $locations ) ) ) {
								$result = 'All countries';
							} else {
								$result = 'Custom';
							}

							echo esc_html__( $result, 'consent-magic' );
						} else {
							esc_html_e( 'No Group was selected.', 'consent-magic' );
						}
						?></p>
                </div>

                <div class="d-flex align-items-center">
					<?php if ( get_the_title( $p_id ) == 'GDPR Rule' && !metadata_exists( 'post', $p_id, '_cs_no_ip_rule' ) ) {
						update_post_meta( $p_id, '_cs_no_ip_rule', 1 );
					}

					render_switcher_input( '_cs_no_ip_rule', false, true, $p_id ); ?>
                    <h4 class="secondary-heading switcher-label"><?php esc_html_e( "Show this rule when we can't retrieve the IP", 'consent-magic' ); ?></h4>
                </div>

                <div class="description-block">
                    <h4 class="font-semibold mr-6"><?php esc_html_e( 'Consent type:', 'consent-magic' ); ?></h4>
                    <p><?php esc_html_e( get_meta_value_by_key( $p_id, '_cs_type' ) ); ?></p>
                </div>

                <div>
                    <h4 class="font-semibold mb-8"><?php esc_html_e( 'Design:', 'consent-magic' ); ?></h4>
                    <div class="design-types">
						<?php echo wp_kses_post( get_meta_value_by_key( $p_id, '_cs_bars_type' ) );
						$bars_type = get_post_meta( $p_id, '_cs_bars_type', true );
						if ( $bars_type == 'bar_small' || $bars_type == 'bar_large' ) : ?>
                            <p class="purple-label"><?php echo esc_attr( $cs_bars_position ); ?></p>
						<?php endif; ?>
                        <p class="purple-label"><?php echo esc_attr( $cs_sticky ); ?></p>
                    </div>
                </div>
            </div>
        </div>
    </div>
</div>