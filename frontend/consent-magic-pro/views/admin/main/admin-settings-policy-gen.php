<?php

namespace ConsentMagicPro;

// If this file is called directly, abort.
if ( !defined( 'WPINC' ) ) {
	die;
}

$options                 = get_existing_pages();
$cs_policy_existing_page = ConsentMagic()->getOption( 'cs_policy_existing_page' );
if ( !is_array( $cs_policy_existing_page ) ) {
	$cs_policy_existing_page = array( CMPRO_DEFAULT_LANGUAGE => $cs_policy_existing_page );
}

$pages_count = count( $cs_policy_existing_page );

?>
<div class="cs-tab-content" data-id="<?php echo esc_attr( $target_id ); ?>">
    <div class="cards-wrapper cards-wrapper-style2">
        <div class="card card-static card-style3">
            <div class="card-header card-header-style3">
                <h4 class="font-semibold-type2"><?php esc_html_e(
			            'Select an existing pages',
			            'consent-magic'
		            ); ?></h4>
            </div>

            <div class="card-body">
                <div class="settings-policy-pages">

                    <div class="language_list">
						<?php foreach ( $cs_policy_existing_page as $key => $value ) : ?>
                            <div class="language_item">
                                <div class="cs_data_render_buttons">
									<?php renderSelectLanguages(
										false,
										'cs_policy_existing_page',
										$key,
										false,
										false,
										'renderSelectStandardInput',
										false,
										'full'
									); ?>
                                </div>

                                <div class="cs_data_render_wrap cs_data_render_wrap_select">
                                    <div class="select_row cs_data_render">
										<?php
										$names = array(
											'cs_policy_existing_page',
											$key,
										);

										renderSelectInputStatic(
											$names,
											$value,
											$options,
											false,
											'js-control-element',
											'full'
										);
										?>
                                    </div>
                                </div>

                                <div class="remove-page">
									<?php include CMPRO_PLUGIN_VIEWS_PATH . 'admin/buttons/admin-page-delete.php'; ?>
                                </div>
                            </div>
						<?php endforeach; ?>
                    </div>

                    <div class="add_new_lang_wrap mt-16">
						<?php render_add_lang_button( 'cs_policy_existing_page', 'renderSelectStandardInput' ); ?>
                        <div style="display: none" class="add_new_lang">
                            <div class="language_item language_item_select" style="display: none">
                                <div class="cs_data_render_buttons">
									<?php renderSelectLanguagesNew(
										false,
										'cs_policy_existing_page',
										false,
										'renderSelectStandardInput',
										false,
										'full'
									); ?>
                                </div>

                                <div class="select_row_default">
                                    <div class="cs_data_render_wrap">
                                        <div class="select_row cs_data_render select-wrap select-full-wrap">
                                        </div>
                                    </div>
                                </div>

                                <div>
									<?php include CMPRO_PLUGIN_VIEWS_PATH . 'admin/buttons/admin-page-delete.php'; ?>
                                </div>
                            </div>
                        </div>

                        <div class="select_default_data" style="display: none">
							<?php
							$names = array(
								'cs_policy_existing_page_default_data',
							);

							renderSelectInputStatic(
								$names,
								'',
								$options,
								false,
								'js-control-element',
							);
							?>
                        </div>
                    </div>
                </div>
            </div>
        </div>
    </div>
</div>