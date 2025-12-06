<?php

namespace ConsentMagicPro;

// If this file is called directly, abort.
if ( !defined( 'WPINC' ) ) {
	die;
}

function render_script_categories( $terms, $primary = true ) {
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

	$fonts = array(
		array(
			'option_value' => 'arial',
			'option_name'  => esc_html__( 'Arial', 'consent-magic' )
		),
		array(
			'option_value' => 'verdana',
			'option_name'  => esc_html__( 'Verdana', 'consent-magic' )
		),
		array(
			'option_value' => 'times',
			'option_name'  => esc_html__( 'Times New Roman', 'consent-magic' )
		)
	);

	$consent_api_cats = CS_WP_Consent_Api()->getWPConsentAPICategories();
	$consent_api_default_cats = CS_WP_Consent_Api()->getWPConsentAPIDefaultCategories();

	foreach ( $terms as $term ) :
		?>
        <div class="card card-style7 disable-control">
            <div class="card-header card-header-style6">
                <div class="switcher disable-control-switcher d-flex align-items-center">
					<?php if ( $term->slug !== 'unassigned' ) { ?>
						<?php
						$names = array(
							'categories',
							$term->term_id,
							$term->display_lang,
							'ignore_this_category',
						);
						$value = !get_term_meta( $term->term_id, 'cs_ignore_this_category', true );
						render_static_switcher_input( $names, $value ); ?>
					<?php } ?>
                    <h4 class="secondary-heading switcher-label"><?php echo esc_html( $term->name_l ); ?></h4>
                </div>

                <div class="script-cat-actions">
					<?php if ( !$primary ) : ?>
                        <button type="button" data-id="<?php echo esc_attr( $term->term_id ); ?>"
                                class="btn btn-red btn-primary-type2 with-small-icon delete_cookie_cat_button"><i
                                    class="icon-delete"></i> <?php esc_html_e( 'Delete', 'consent-magic' ); ?>
                        </button>
					<?php endif; ?>

					<?php cardCollapseSettingsWithText(); ?>
                </div>
            </div>

            <div class="card-body">
                <div class="gap-24">
                    <div>
                        <h4 class="font-semibold mb-4">
							<?php esc_html_e( 'Title', 'consent-magic' ); ?>:
                        </h4>

						<?php renderTextInputTerm( 'title', $term, false, true, '', 'js-control-element', 'required' ); ?>
                    </div>

					<?php if ( $term->slug != 'necessary' ) : ?>
                        <div>
                            <h4 class="font-semibold mb-4">
								<?php esc_html_e( 'IAB Purpose (recommended)', 'consent-magic' ); ?>:
                            </h4>

                            <div class="cs_data_render">
								<?php $iab_cat = get_term_meta( $term->term_id, '_cs_iab_cat', true );
								if ( empty( $iab_cat ) && $iab_cat != 0 ) {
									$default_categories = CS_IAB_Integration()->get_default_script_categories();
									$iab_cat            = $default_categories[ $term->slug ];
								}
								?>

								<?php renderSelectInputStaticTerm( 'iab_cat', $term, $iab_cat, $purposes, true, 'cs_cookie_iab_cat', 'full' ); ?>

								<?php if ( !in_array( $iab_cat, array_column( $purposes, 'option_value' ) ) ) : ?>
                                    <div class="mt-8">
										<?php render_warning_message( esc_html__( 'Warning: Deactivated IAB Purpose present in category mapping: ', 'consent-magic' ) . '<a href="' . esc_url( get_admin_url() . 'admin.php?page=cs-iab' ) . '" class="link link-small">' . $vendor_list->purposes->{$iab_cat}->name . '</a>' ); ?>
                                    </div>
								<?php endif; ?>
                            </div>
                        </div>
					<?php endif; ?>

		            <?php if ( CS_WP_Consent_Api()->isEnableWPConsent() ) : ?>
                        <div>
                            <h4 class="font-semibold mb-4">
                                <?php esc_html_e( 'WP Consent API Category', 'consent-magic' ); ?>:
                            </h4>

                            <div class="cs_data_render">
                                <?php $wp_consent_cat = get_term_meta( $term->term_id, '_cs_wp_consent_api_cat', true );
                                if ( empty( $wp_consent_cat ) ) {
                                    $wp_consent_cat = $consent_api_default_cats[ $term->slug ] ?? $consent_api_default_cats[ 'default' ];
                                }
                                ?>

                                <?php renderSelectInputStaticTerm( 'cs_wp_consent_api_cat', $term, $wp_consent_cat, $consent_api_cats, true, '', 'full' ); ?>
                            </div>
                        </div>
		            <?php endif; ?>

                    <div class="input_row_default">
                        <h4 class="font-semibold mb-4">
							<?php esc_html_e( 'Description', 'consent-magic' ); ?>:
                        </h4>

                        <div class="cs_data_render_wrap">
							<?php renderWpEditorByTerm( $term, 'description', $term->description_l, 'cs_cookie_cat_description form-control js-control-element', $term->term_id ); ?>

                            <div class="form-control-lang-new"
                                 data-lang="<?php echo esc_attr( $term->display_lang ) ?>">
                            </div>
                        </div>
                    </div>

					<?php if ( $term->slug === 'googlefonts' ) { ?>
                        <div>
                            <h4 class="font-semibold mb-4">
								<?php esc_html_e( 'Select the fallback font', 'consent-magic' ); ?>:
                            </h4>

							<?php renderSelectInput( 'default_font', $fonts, true ); ?>
                        </div>
					<?php } ?>

					<?php if ( $term->slug === 'unassigned' ) { ?>
                        <div class="d-flex align-items-center">
							<?php render_switcher_input( 'cs_not_block_unassigned_cookies' ); ?>
                            <h4 class="switcher-label"><?php esc_html_e( "Don't block Unassigned before consent is expressed for 'Ask before tracking' consent type.", 'consent-magic' ); ?></h4>
                        </div>
					<?php } ?>

                    <div class="language_list_block">
                        <div class="card card-style4">
                            <div class="card-header card-header-style4">
								<?php
								render_additional_lang_button( $term->name_l ); ?>

                                <div class="cs_load_langs"
                                     data-option-type="1"
                                     data-current-lang="<?php echo esc_attr( $term->display_lang ); ?>"
                                     data-key="<?php echo esc_attr( $term->term_id ); ?>"
                                     data-render="renderWpEditorByTerm">

									<?php cardCollapseSettingsWithText( 'Show' ); ?>
                                </div>
                            </div>

                            <div class="card-body">
                                <div class="language_list_container language_list_container_ajax">
									<?php render_hidden_checkbox( sprintf( 'cs_language_control_%s', $term->term_id ), 0, 'cs_input_language_control' ); ?>
									<?php render_lang_loader(); ?>

                                    <div class="loading_body">
                                        <div class="language_list"></div>
                                        <div class="add_new_lang_wrap">
											<?php render_add_lang_button( 'cs_edit_cat_description', 'renderWpEditorByTerm' ); ?>
                                            <div style="display: none" class="add_new_lang">
                                                <div class="language_item language_item_textarea"
                                                     style="display: none;">
                                                    <div>
                                                        <h4 class="font-semibold mb-4">
															<?php esc_html_e( 'Title', 'consent-magic' ); ?>:
                                                        </h4>

                                                        <div class="input_row">
                                                            <input type="text"
                                                                   class="cs_cookie_cat_title form-control js-control-element input-full"/>
                                                        </div>
                                                    </div>

                                                    <div class="input_row_default">
                                                        <h4 class="font-semibold mb-4">
															<?php esc_html_e( 'Description', 'consent-magic' ); ?>:
                                                        </h4>

                                                        <div class="cs_data_render_wrap cs_data_render_wrap_editor">
                                                            <div class="input_row cs_data_render">
                                                            </div>
                                                        </div>
                                                    </div>

                                                    <div class="cs_data_render_buttons">
                                                        <h4 class="font-semibold mb-4"><?php esc_html_e( 'Language', 'consent-magic' ); ?></h4>
														<?php renderSelectLanguagesByTermNew( 'cs_edit_cat_description', $term->term_id, '', 'renderWpEditorByTerm' ); ?>
                                                    </div>

                                                    <div>
														<?php include CMPRO_PLUGIN_VIEWS_PATH . 'admin/buttons/admin-lang-delete.php'; ?>
                                                    </div>
                                                </div>
                                            </div>
                                        </div>
                                    </div>
                                </div>
                            </div>
                        </div>
                    </div>
                </div>
            </div>
        </div>
	<?php
	endforeach;
}