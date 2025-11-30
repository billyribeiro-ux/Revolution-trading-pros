<?php

namespace ConsentMagicPro;

// If this file is called directly, abort.
if ( !defined( 'WPINC' ) ) {
	die;
}

$vendor_list   = CS_IAB_Integration()->get_vendor_list();
$purposes_list = !empty( $vendor_list->purposes ) ? $vendor_list->purposes : array();
$iab_settings  = CS_IAB_Integration()->get_settings();
$purposes      = array(
	array(
		'option_value' => 0,
		'option_name'  => __( 'None', 'consent-magic' )
	)
);
if ( !empty( $purposes_list ) ) {
	foreach ( $purposes_list as $purpose ) {
		if ( $iab_settings->purposes->{$purpose->id} ) {
			$purposes[] = array(
				'option_value' => $purpose->id,
				'option_name'  => $purpose->name
			);
		}
	}
}

$consent_api_cats         = CS_WP_Consent_Api()->getWPConsentAPICategories();
$consent_api_default_cats = CS_WP_Consent_Api()->getWPConsentAPIDefaultCategories();

$unassigned    = get_term_by( 'slug', 'unassigned', 'cs-cookies-category' );
$terms_primary = get_cookies_terms_objects( 'cs_primary_term', true, $unassigned->term_id );
$terms         = get_cookies_terms_objects( 'cs_primary_term', false, $unassigned->term_id );

?>

<div class="cards-wrapper cards-wrapper-style2 gap-22">
    <h3 class="primary-heading">
		<?php esc_html_e( 'Cookie/Script Types', 'consent-magic' ); ?>:
    </h3>

    <div class="script-categories-wrap">
		<?php
		if ( $terms_primary ) {
			render_script_categories( $terms_primary );
		}
		if ( $terms ) {
			render_script_categories( $terms, false );
		}
		?>

        <div>
            <button type="button" class="btn btn-primary btn-primary-type2 with-icon cs_new_cat">
                <i class="icon-plus"></i>
				<?php esc_html_e( 'Add new type', 'consent-magic' ); ?>
            </button>

            <div class="cs_new_cat_block">
                <div class="language_item language_item_textarea gap-22">
                    <div>
                        <h4 class="font-semibold-type2 mb-4">
							<?php esc_html_e( 'Title', 'consent-magic' ); ?>:
                        </h4>

                        <input type="text"
                               name="cs[<?php echo esc_attr( ConsentMagic()->plugin_name ); ?>][new_category][title]"
                               placeholder="<?php esc_attr_e( 'Title', 'consent-magic' ); ?>"
                               class="form-control input-full"
                               value=""/>
                    </div>

                    <div>
                        <h4 class="font-semibold-type2 mb-4">
							<?php esc_html_e( 'IAB Purpose (recommended)', 'consent-magic' ); ?>:
                        </h4>

                        <div class="cs_data_render">
                            <div class="select-wrap select-full-wrap">
								<?php
								$default_categories = CS_IAB_Integration()->get_default_script_categories();
								$iab_cat            = $default_categories[ 'default' ];
								?>
                                <select
                                        name="cs[<?php echo esc_attr( ConsentMagic()->plugin_name ); ?>][new_category][iab_cat]"
                                        autocomplete="off"
                                        class="form-control-cm"
                                        style="width: 100%;">
									<?php foreach ( $purposes as $option_value ) : ?>
                                        <option value="<?php echo esc_attr( $option_value[ 'option_value' ] ); ?>" <?php selected( $iab_cat, $option_value[ 'option_value' ] ) ?>><?php echo esc_attr( $option_value[ 'option_name' ] ); ?></option>
									<?php endforeach; ?>
                                </select>
                            </div>
                        </div>
                    </div>

                    <?php if ( CS_WP_Consent_Api()->isEnableWPConsent() ) : ?>
                        <div>
                            <h4 class="font-semibold mb-4">
                                <?php esc_html_e( 'WP Consent API', 'consent-magic' ); ?>:
                            </h4>

                            <div class="cs_data_render">
                                <select
                                        name="cs[<?php echo esc_attr( ConsentMagic()->plugin_name ); ?>][new_category][cs_wp_consent_api_cat]"
                                        autocomplete="off"
                                        class="form-control-cm"
                                        style="width: 100%;">
                                    <?php foreach ( $consent_api_cats as $key => $cat ) : ?>
                                        <option value="<?php echo esc_attr( $key ); ?>" <?php selected( $consent_api_default_cats[ 'default' ], $key ) ?>><?php echo esc_attr( $cat ); ?></option>
                                    <?php endforeach; ?>
                                </select>
                            </div>
                        </div>
                    <?php endif; ?>

                    <div class="input_row_default">
                        <h4 class="font-semibold-type2 mb-4">
							<?php esc_html_e( 'Description', 'consent-magic' ); ?>:
                        </h4>

                        <div class="cs_data_render_wrap">
							<?php renderWpEditorByTerm( '', 'description', '', 'cs_cookie_cat_description form-control-cm js-control-element' ); ?>
                        </div>
                    </div>
                </div>
            </div>
        </div>
    </div>
</div>

<div class="cm-modal cs_renew_consent_wrap_new_cat">
    <div class="cm-modal-content-wrap">
        <div class="cm-modal-content">
            <p class="mb-24 fw-500"><?php esc_html_e( 'Existing consent will be invalidated, and your visitors will be asked for consent again.', 'consent-magic' ); ?></p>
            <div class="modal-buttons">
                <button type="button" class="btn btn-red btn-primary-type2 cs_renew_consent_cancel_new_cat"><i
                            class="icon-restore"></i> <?php esc_html_e( 'Cancel', 'consent-magic' ); ?>
                </button>
                <button type="button"
                        class="btn btn-primary btn-primary-type2 cs_renew_consent_category_new_cat"><i
                            class="icon-renew"></i> <?php esc_html_e( 'OK', 'consent-magic' ); ?>
                </button>
            </div>
        </div>
    </div>
</div>
