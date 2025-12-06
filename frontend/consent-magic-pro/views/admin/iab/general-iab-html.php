<?php

namespace ConsentMagicPro;

// If this file is called directly, abort.
if ( !defined( 'WPINC' ) ) {
	die;
}

$vendor_list  = CS_IAB_Integration()->get_vendor_list();
$iab_settings = CS_IAB_Integration()->get_settings();

?>
<div class="cards-wrapper cards-wrapper-style2 cs_iab_settings" id="cs_iab_settings">
    <div class="card card-style8">
        <div class="card-header card-header-style3">
            <h3 class="primary-heading-type2"><?php esc_html_e( 'Purposes', 'consent-magic' ); ?></h3>
			<?php cardCollapseSettingsWithText( 'Details' ); ?>
        </div>

        <div class="card-body">
			<?php if ( !empty( $vendor_list ) && !empty( $vendor_list->purposes ) ) : ?>
                <div class="gap-24">
					<?php foreach ( $vendor_list->purposes as $key => $purpose ) : ?>
						<?php if ( !isset( $iab_settings->purposes->{$purpose->id} ) ) {
							$iab_settings = CS_IAB_Integration()->set_default_option( 'purposes', $purpose->id );
						} ?>

                        <div class="card card-style9 cs_iab_item">
                            <div class="card-header d-flex align-items-center justify-content-between">
                                <div class="iab-card-header-inner">
                                    <div class="iab-card-header">
										<?php
										$names = array(
											'cs_iab_show_purpose_' . $purpose->id,
										);
										render_static_switcher_input( $names, $iab_settings->purposes->{$purpose->id}, false, false, false, 'cs-purposes-input' ); ?>

                                        <h4 class="secondary-heading switcher-label"><?php echo esc_html( $purpose->name ); ?></h4>
                                    </div>

                                    <div class="iab-card-header-details">
										<?php cardCollapseSettingsWithText( 'Details' ); ?>
                                    </div>

                                    <div class="line line-showing"></div>
                                </div>
                            </div>

                            <div class="card-body no-border">
                                <p class="iab-description">
									<?php echo esc_html( $purpose->description ); ?>
                                </p>

                                <div class="line line-showing mt-24"></div>
                            </div>
                        </div>
					<?php endforeach; ?>
                </div>
			<?php endif; ?>
        </div>
    </div>

    <div class="card card-style8">
        <div class="card-header card-header-style3">
            <h3 class="primary-heading-type2"><?php esc_html_e( 'Special Purposes', 'consent-magic' ); ?></h3>
			<?php cardCollapseSettingsWithText( 'Details' ); ?>
        </div>

        <div class="card-body">
			<?php if ( !empty( $vendor_list ) && !empty( $vendor_list->specialPurposes ) ) : ?>
                <div class="gap-24">
					<?php foreach ( $vendor_list->specialPurposes as $key => $purpose ) : ?>
						<?php if ( !isset( $iab_settings->specialPurposes->{$purpose->id} ) ) {
							$iab_settings = CS_IAB_Integration()->set_default_option( 'specialPurposes', $purpose->id );
						} ?>

                        <div class="card card-style9 cs_iab_item">
                            <div class="card-header d-flex align-items-center justify-content-between">
                                <div class="iab-card-header-inner">
                                    <div class="iab-card-header">
										<?php
										$names = array(
											'cs_iab_show_special_purpose_' . $purpose->id,
										);
										render_static_switcher_input( $names, $iab_settings->specialPurposes->{$purpose->id}, false, false, false, 'cs-special-purposes-input' ); ?>

                                        <h4 class="secondary-heading switcher-label"><?php echo esc_html( $purpose->name ); ?></h4>
                                    </div>

                                    <div class="iab-card-header-details">
										<?php cardCollapseSettingsWithText( 'Details' ); ?>
                                    </div>

                                    <div class="line line-showing"></div>
                                </div>
                            </div>

                            <div class="card-body no-border">
                                <p class="iab-description">
									<?php echo esc_html( $purpose->description ); ?>
                                </p>

                                <div class="line line-showing mt-24"></div>
                            </div>
                        </div>
					<?php endforeach; ?>
                </div>
			<?php endif; ?>
        </div>
    </div>

    <div class="card card-style8">
        <div class="card-header card-header-style3">
            <h3 class="primary-heading-type2"><?php esc_html_e( 'Features', 'consent-magic' ); ?></h3>
			<?php cardCollapseSettingsWithText( 'Details' ); ?>
        </div>

        <div class="card-body">
			<?php if ( !empty( $vendor_list ) && !empty( $vendor_list->features ) ) : ?>
                <div class="gap-24">
					<?php foreach ( $vendor_list->features as $key => $feature ) : ?>
						<?php if ( !isset( $iab_settings->features->{$feature->id} ) ) {
							$iab_settings = CS_IAB_Integration()->set_default_option( 'features', $feature->id );
						} ?>

                        <div class="card card-style9 cs_iab_item">
                            <div class="card-header d-flex align-items-center justify-content-between">
                                <div class="iab-card-header-inner">
                                    <div class="iab-card-header">
										<?php
										$names = array(
											'cs_iab_show_feature_' . $feature->id,
										);
										render_static_switcher_input( $names, $iab_settings->features->{$feature->id}, false, false, false, 'cs-features-input' ); ?>

                                        <h4 class="secondary-heading switcher-label"><?php echo esc_html( $feature->name ); ?></h4>
                                    </div>

                                    <div class="iab-card-header-details">
										<?php cardCollapseSettingsWithText( 'Details' ); ?>
                                    </div>

                                    <div class="line line-showing"></div>
                                </div>
                            </div>

                            <div class="card-body no-border">
                                <p class="iab-description">
									<?php echo esc_html( $feature->description ); ?>
                                </p>

                                <div class="line line-showing mt-24"></div>
                            </div>
                        </div>
					<?php endforeach; ?>
                </div>
			<?php endif; ?>
        </div>
    </div>

    <div class="card card-style8">
        <div class="card-header card-header-style3">
            <h3 class="primary-heading-type2"><?php esc_html_e( 'Special Features', 'consent-magic' ); ?></h3>
			<?php cardCollapseSettingsWithText( 'Details' ); ?>
        </div>

        <div class="card-body">
			<?php if ( !empty( $vendor_list ) && !empty( $vendor_list->specialFeatures ) ) : ?>
                <div class="gap-24">
					<?php foreach ( $vendor_list->specialFeatures as $key => $feature ) : ?>
						<?php if ( !isset( $iab_settings->specialFeatures->{$feature->id} ) ) {
							$iab_settings = CS_IAB_Integration()->set_default_option( 'specialFeatures', $feature->id );
						} ?>

                        <div class="card card-style9 cs_iab_item">
                            <div class="card-header d-flex align-items-center justify-content-between">
                                <div class="iab-card-header-inner">
                                    <div class="iab-card-header">
										<?php
										$names = array(
											'cs_iab_show_special_feature_' . $feature->id,
										);
										render_static_switcher_input( $names, $iab_settings->specialFeatures->{$feature->id}, false, false, false, 'cs-special-features-input' ); ?>

                                        <h4 class="secondary-heading switcher-label"><?php echo esc_html( $feature->name ); ?></h4>
                                    </div>

                                    <div class="iab-card-header-details">
										<?php cardCollapseSettingsWithText( 'Details' ); ?>
                                    </div>

                                    <div class="line line-showing"></div>
                                </div>
                            </div>

                            <div class="card-body no-border">
                                <p class="iab-description">
									<?php echo esc_html( $feature->description ); ?>
                                </p>

                                <div class="line line-showing mt-24"></div>
                            </div>
                        </div>
					<?php endforeach; ?>
                </div>
			<?php endif; ?>
        </div>
    </div>

    <div class="card card-style8">
        <div class="card-header card-header-style3">
            <h3 class="primary-heading-type2"><?php esc_html_e( 'Vendors', 'consent-magic' ); ?></h3>
			<?php cardCollapseSettingsWithText( 'Details' ); ?>
        </div>

        <div class="card-body">
            <p class="text-gray lh-134 mb-24"><?php esc_html_e( sprintf( 'IAB framework has support for %d vendors. You can chose to show only the relevant ones for your website by using the "Show just the following vendors" option.', count( (array) $iab_settings->vendors ) ), 'consent-magic' ) ?></p>

            <div class="d-flex align-items-center">
				<?php render_switcher_input( 'cs_iab_show_only_vendors' ); ?>
                <h4 class="secondary-heading switcher-label cs_iab_show_only_vendors"><?php esc_html_e( 'Show just the following vendors', 'consent-magic' ); ?></h4>
            </div>

            <div class="cs_iab_show_only_vendors_details"
				<?php echo ( ConsentMagic()->getOption( 'cs_iab_show_only_vendors' ) ) ? '' : 'style="display: none;" '; ?>
            >
                <div class="line mt-24 mb-24"></div>

                <input type="hidden"
                       id="cs_iab_vendor_settings"
                       value="<?php echo esc_html( json_encode( $iab_settings->vendors, JSON_FORCE_OBJECT ) ); ?>"
                       name="cs[<?php echo esc_attr( ConsentMagic()->plugin_name ); ?>][cs_iab_vendor_settings]"
                >

                <div class="cs_iab_vendor_list">
                    <input type="text"
                           id="cs_search_vendor"
                           class="input-full mb-24"
                           placeholder="<?php esc_html_e( 'Search vendor', 'consent-magic' ) ?>">

                    <div id="cs_vendor_list">
						<?php if ( !empty( $vendor_list ) && !empty( $vendor_list->vendors ) ) : ?>
							<?php foreach ( $vendor_list->vendors as $key => $vendor ) : ?>
								<?php if ( !isset( $iab_settings->vendors->{$vendor->id} ) ) {
									$iab_settings = CS_IAB_Integration()->set_default_option( 'vendors', $vendor->id );
								}
								$dataset = array(
									'vendor_id' => $vendor->id,
								);
								?>

                                <div class="cs_iab_vendor_container">
                                    <div class="card card-style9 cs_iab_item">
                                        <div class="card-header d-flex align-items-center justify-content-between">
                                            <div class="d-flex align-items-center">
												<?php
												$names = array(
													'cs_iab_show_vendor_' . $vendor->id,
												);
												render_static_switcher_input( $names, $iab_settings->vendors->{$vendor->id}, false, false, false, 'cs-vendor-input', $dataset, false ); ?>
                                                <h4 class="secondary-heading switcher-label cs_vendor_name"><?php echo esc_html( $vendor->name ); ?></h4>
                                            </div>

                                            <div class="cs_iab_vendor_details">
												<?php cardCollapseSettingsWithText( 'Details' ); ?>
                                            </div>

                                        </div>

                                        <div class="card-body no-border">
                                            <div class="list-item-description">
                                            </div>
                                        </div>
                                    </div>
                                </div>
							<?php endforeach; ?>
						<?php endif; ?>
                    </div>
                </div>
            </div>
        </div>
    </div>

    <div class="card card-style8">
        <div class="card-header card-header-style3">
            <h3 class="primary-heading-type2"><?php esc_html_e( 'Google Ad Tech Vendors', 'consent-magic' ); ?></h3>
			<?php cardCollapseSettingsWithText( 'Details' ); ?>
        </div>

        <div class="card-body">
            <p class="text-gray lh-134 mb-24"><?php esc_html_e( sprintf( 'Google Ad Tech Vendors includes %d additional vendors. You can chose to show only the relevant ones for your website by using the "Show just the following additional vendors" option.', count( (array) $vendor_list->additional_vendors ) ), 'consent-magic' ) ?></p>

            <div class="d-flex align-items-center">
				<?php render_switcher_input( 'cs_iab_show_only_additional_vendors' ); ?>
                <h4 class="secondary-heading switcher-label cs_iab_show_only_vendors"><?php esc_html_e( 'Show just the following additional vendors', 'consent-magic' ); ?></h4>
            </div>

            <div class="cs_iab_show_only_additional_vendors_details"
				<?php echo ( ConsentMagic()->getOption( 'cs_iab_show_only_additional_vendors' ) ) ? '' : 'style="display: none;" '; ?>
            >
                <div class="line mt-24 mb-24"></div>

                <input type="hidden"
                       id="cs_iab_additional_vendor_settings"
                       value="<?php echo esc_html( json_encode( $iab_settings->additional_vendors ) ); ?>"
                       name="cs[<?php echo esc_attr( ConsentMagic()->plugin_name ); ?>][cs_iab_additional_vendor_settings]"
                >

                <div class="cs_iab_vendor_list">
                    <input type="text"
                           id="cs_search_additional_vendor"
                           class="input-full mb-24"
                           placeholder="<?php esc_html_e( 'Search additional vendor', 'consent-magic' ) ?>">

                    <div id="cs_additional_vendor_list">
						<?php if ( !empty( $vendor_list ) && !empty( $vendor_list->additional_vendors ) ) : ?>
							<?php foreach ( $vendor_list->additional_vendors as $key => $vendor ) : ?>
								<?php if ( !isset( $iab_settings->additional_vendors->{$vendor->id} ) ) {
									$iab_settings = CS_IAB_Integration()->set_default_option( 'vendors', $vendor->id );
								}
								$dataset = array(
									'vendor_id' => $vendor->id,
								);
								?>

                                <div class="cs_iab_vendor_container">
                                    <div class="card card-style9 cs_iab_item">
                                        <div class="card-header d-flex align-items-center justify-content-between">
                                            <div class="d-flex align-items-center">
												<?php
												$names = array(
													'cs_iab_show_additional_vendor_' . $vendor->id,
												);
												render_static_switcher_input( $names, $iab_settings->additional_vendors->{$vendor->id}, false, false, false, 'cs-additional-vendor-input', $dataset, false ); ?>
                                                <h4 class="secondary-heading switcher-label cs_vendor_name"><?php echo esc_html( $vendor->name ); ?></h4>
                                            </div>

                                            <div class="cs_iab_additional_vendor_details">
												<?php cardCollapseSettingsWithText( 'Details' ); ?>
                                            </div>

                                        </div>

                                        <div class="card-body no-border">
                                            <div class="list-item-description">
                                            </div>
                                        </div>
                                    </div>
                                </div>
							<?php endforeach; ?>
						<?php endif; ?>
                    </div>
                </div>
            </div>
        </div>
    </div>
</div>
