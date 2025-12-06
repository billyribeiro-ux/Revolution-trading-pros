<?php

namespace ConsentMagicPro;

// If this file is called directly, abort.
if ( !defined( 'WPINC' ) ) {
	die;
}

function render_language_block( $id, $inputs, $title, $subtitle, $card_style = 1, $header_style = 3 ) {
	?>
    <div class="card card-style<?php echo esc_attr( $card_style ); ?>">
        <div class="card-header card-header-style<?php echo esc_attr( $header_style ); ?>">
            <div class="gap-8">
                <h3 class="primary-heading"><?php echo esc_html( $title ); ?></h3>
                <p class="text-gray"><?php echo esc_html( $subtitle ); ?></p>
            </div>

			<?php cardCollapseSettingsWithText(); ?>
        </div>

        <div class="card-body language_list_block_wrap">
			<?php
			foreach ( $inputs as $input ) {
				if ( $input[ 'type' ] === 'textarea' ) {
					render_textarea_text( $id, $input[ 'key' ], $input[ 'meta' ], $title, $input[ 'input_title' ] );
				} elseif ( $input[ 'type' ] === 'input' ) {
					render_input_text( $id, $input[ 'key' ], $input[ 'meta' ], $title, $input[ 'input_title' ] );
				}
			}
			?>
        </div>
    </div>
	<?php
}

function render_language_block_ajax( $inputs, $title, $subtitle, $card_style = 1, $header_style = 3 ) {
	?>
    <div class="card card-style<?php echo esc_attr( $card_style ); ?>">
        <div class="card-header card-header-style<?php echo esc_attr( $header_style ); ?>">
            <div class="gap-8">
                <h3 class="primary-heading"><?php echo esc_html( $title ); ?></h3>
                <p class="text-gray"><?php echo esc_html( $subtitle ); ?></p>
            </div>

			<?php cardCollapseSettingsWithText(); ?>
        </div>

        <div class="card-body language_list_block_wrap">
			<?php
			foreach ( $inputs as $input ) {
				if ( $input[ 'type' ] === 'textarea' ) {
					render_language_block_textarea_ajax( $input[ 'key' ], $input[ 'input_title' ], $input[ 'input_subtitle' ] );
				} elseif ( $input[ 'type' ] === 'input' ) {
					render_language_block_input_ajax( $input[ 'key' ], $input[ 'input_title' ], $input[ 'input_subtitle' ] );
				}
			}
			?>
        </div>
    </div>
	<?php
}

function render_language_block_new_template( $inputs, $title, $subtitle, $card_style = 1, $header_style = 3 ) {
	?>
    <div class="card card-style<?php echo esc_attr( $card_style ); ?>">
        <div class="card-header card-header-style<?php echo esc_attr( $header_style ); ?>">
            <div class="gap-8">
                <h3 class="primary-heading"><?php echo esc_html( $title ); ?></h3>
                <p class="text-gray"><?php echo esc_html( $subtitle ); ?></p>
            </div>

			<?php cardCollapseSettingsWithText(); ?>
        </div>

        <div class="card-body language_list_block_wrap">
			<?php
			foreach ( $inputs as $input ) {
				if ( $input[ 'type' ] === 'textarea' ) {
					render_textarea_text_new_template( $input[ 'key' ], $input[ 'meta' ], $title, $input[ 'input_title' ] );
				} elseif ( $input[ 'type' ] === 'input' ) {
					render_input_text_new_template( $input[ 'key' ], $input[ 'meta' ], $title, $input[ 'input_title' ] );
				}
			}
			?>
        </div>
    </div>
	<?php
}

function render_textarea_text( $id, $key, $meta, $title, $input_title ) {
	$current_lang             = get_locale();
	$cs_user_default_language = ConsentMagic()->getOption( 'cs_user_default_language' );
	$cs_language_availability = ConsentMagic()->getOption( 'cs_language_availability' );
	if ( isset( $cs_language_availability[ $current_lang ] ) && $cs_language_availability[ $current_lang ] == 0 ) {
		$current_lang = $cs_user_default_language;
	}

	if ( $meta ) {
		$translations = ConsentMagic()->getTranslations( $key, 'meta', $id );
	} else {
		$translations = ConsentMagic()->getTranslations( $key );
	}

	if ( isset( $translations[ $current_lang ] ) ) {
		$display_translation = $translations[ $current_lang ];
		$display_lang        = $current_lang;
	} elseif ( isset( $translations[ CMPRO_DEFAULT_LANGUAGE ] ) ) {
		$display_translation = $translations[ CMPRO_DEFAULT_LANGUAGE ];
		$display_lang        = CMPRO_DEFAULT_LANGUAGE;
	} else {
		$display_translation = '';
		$display_lang        = CMPRO_DEFAULT_LANGUAGE;
	}

	?>

    <div class="language_list_block">
        <div class="cart_line_item input_row_default mb-8">
            <div class="cs_data_render_wrap cs_data_render_wrap_editor">
                <div class="input_row cs_data_render">
					<?php if ( !empty( $input_title ) ): ?>
                        <h4 class="font-semibold mb-4"><?php echo esc_html( $input_title ); ?></h4>
					<?php endif; ?>
					<?php renderWpEditor( $key, $display_translation, $display_lang, 'js-control-element' ); ?>
                </div>
            </div>
        </div>

        <div class="card card-style4">
            <div class="card-header card-header-style4">
				<?php
				$text = !empty( $input_title ) ? $input_title : $title;
				render_additional_lang_button( $text ); ?>
				<?php cardCollapseSettingsWithText( 'Show' ); ?>
            </div>

            <div class="card-body">
                <div class="language_list">
					<?php
					if ( !empty( $translations ) ) :
						foreach ( $translations as $lang => $value ) :
							if ( $lang != $display_lang ) :?>
                                <div class="language_item language_item_textarea"
                                     style="<?php echo isset( $cs_language_availability[ $lang ] ) &&
								                       $cs_language_availability[ $lang ] ==
								                       0 ? 'display:none;' : '' ?>">
                                    <div class="cs_data_render_wrap cs_data_render_wrap_editor">
                                        <div class="input_row cs_data_render">
                                            <div class="cs_data_render_wrap">
												<?php renderWpEditor( $key, $value, $lang, 'js-control-element' ); ?>
                                            </div>
                                        </div>
                                    </div>

                                    <div class="cs_data_render_buttons">
                                        <h4 class="font-semibold-type2 mb-4"><?php esc_html_e( 'Language', 'consent-magic' ); ?></h4>
										<?php renderSelectLanguages( false, $key, $lang, false, $id, 'renderWpEditor', $meta ); ?>
                                    </div>

                                    <div>
										<?php include CMPRO_PLUGIN_VIEWS_PATH .
										              'admin/buttons/admin-lang-delete.php'; ?>
                                    </div>
                                </div>
							<?php endif;
						endforeach;
					endif;
					?>
                </div>

                <div class="add_new_lang_wrap">
					<?php render_add_lang_button( $key, 'renderWpEditor' ); ?>
                    <div style="display: none" class="add_new_lang">
                        <div class="language_item language_item_textarea" style="display: none">
                            <div class="cs_data_render_wrap">
                                <div class="input_row cs_data_render">
                                </div>
                            </div>

                            <div class="cs_data_render_buttons">
                                <h4 class="font-semibold-type2 mb-4"><?php esc_html_e( 'Language', 'consent-magic' ); ?></h4>
								<?php renderSelectLanguagesNew( false, $key, $id, 'renderWpEditor', $meta ); ?>
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

	<?php
}

function render_textarea_text_new_template( $key, $meta, $title, $input_title ) {
	$current_lang = get_locale();
	?>
    <div class="language_list_block">
        <div class="cart_line_item input_row_default mb-8">
            <div class="cs_data_render_wrap cs_data_render_wrap_editor">
                <div class="input_row cs_data_render">
					<?php if ( !empty( $input_title ) ): ?>
                        <h4 class="font-semibold mb-4"><?php echo esc_html( $input_title ); ?></h4>
					<?php endif; ?>
					<?php renderWpEditor( $key, '', $current_lang, 'js-control-element' ); ?>
                </div>
            </div>
        </div>

        <div class="card card-style4">
            <div class="card-header card-header-style4">
				<?php
				$text = !empty( $input_title ) ? $input_title : $title;
				render_additional_lang_button( $text ); ?>
				<?php cardCollapseSettingsWithText( 'Show' ); ?>
            </div>

            <div class="card-body">
                <div class="language_list">
                </div>

                <div class="add_new_lang_wrap">
					<?php render_add_lang_button( $key, 'renderWpEditor' ); ?>
                    <div style="display: none" class="add_new_lang">
                        <div class="language_item language_item_textarea" style="display: none">
                            <div class="cs_data_render_wrap">
                                <div class="input_row cs_data_render">
                                </div>
                            </div>

                            <div class="cs_data_render_buttons">
                                <h4 class="font-semibold-type2 mb-4"><?php esc_html_e( 'Language', 'consent-magic' ); ?></h4>
								<?php renderSelectLanguagesNew( false, $key, false, 'renderWpEditor', $meta ); ?>
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

	<?php
}

function render_input_text( $id, $key, $meta, $title, $input_title ) {
	$current_lang             = get_locale();
	$cs_user_default_language = ConsentMagic()->getOption( 'cs_user_default_language' );
	$cs_language_availability = ConsentMagic()->getOption( 'cs_language_availability' );

	if ( isset( $cs_language_availability[ $current_lang ] ) && $cs_language_availability[ $current_lang ] == 0 ) {
		$current_lang = $cs_user_default_language;
	}

	if ( $meta ) {
		$translations = ConsentMagic()->getTranslations( $key, 'meta', $id );
	} else {
		$translations = ConsentMagic()->getTranslations( $key );
	}

	if ( isset( $translations[ $current_lang ] ) ) {
		$display_translation = $translations[ $current_lang ];
		$display_lang        = $current_lang;
	} elseif ( isset( $translations[ CMPRO_DEFAULT_LANGUAGE ] ) ) {
		$display_translation = $translations[ CMPRO_DEFAULT_LANGUAGE ];
		$display_lang        = CMPRO_DEFAULT_LANGUAGE;
	} else {
		$display_translation = '';
		$display_lang        = CMPRO_DEFAULT_LANGUAGE;
	}

	?>

    <div class="language_list_block">
        <div class="cart_line_item input_row_default mb-8">
            <div class="cs_data_render_wrap cs_data_render_wrap_input">
                <div class="input_row cs_data_render">
					<?php if ( !empty( $input_title ) ): ?>
                        <h4 class="font-semibold mb-4"><?php echo esc_html( $input_title ); ?></h4>
					<?php endif; ?>

					<?php renderTextInput( $key, $display_translation, $id, $display_lang, $meta, '', 'js-control-element' ); ?>
                </div>
            </div>
        </div>

        <div class="card card-style4">
            <div class="card-header card-header-style4">
				<?php
				$text = !empty( $input_title ) ? $input_title : $title;
				render_additional_lang_button( $text ); ?>
				<?php cardCollapseSettingsWithText( 'Show' ); ?>
            </div>

            <div class="card-body">
                <div class="language_list">
					<?php
					if ( !empty( $translations ) ) :
						foreach ( $translations as $lang => $value ) :
							if ( $lang != $display_lang ) :?>
                                <div class="language_item language_item_input"
                                     style="<?php echo isset( $cs_language_availability[ $lang ] ) &&
								                       $cs_language_availability[ $lang ] ==
								                       0 ? 'display:none;' : '' ?>">

                                    <div class="cs_data_render_buttons">
										<?php renderSelectLanguages( false, $key, $lang, false, $id, 'renderTextInput', $meta, 'middle' ); ?>
                                    </div>

                                    <div class="cs_data_render_wrap cs_data_render_wrap_input">
                                        <div class="input_row cs_data_render">
											<?php renderTextInput( $key, $value, $id, $lang, $meta, '', 'js-control-element' ); ?>
                                        </div>
                                    </div>

                                    <div>
										<?php include CMPRO_PLUGIN_VIEWS_PATH .
										              'admin/buttons/admin-lang-delete.php'; ?>
                                    </div>
                                </div>
							<?php endif;
						endforeach;
					endif;
					?>
                </div>

                <div class="add_new_lang_wrap">
					<?php render_add_lang_button( $key, 'renderTextInput' ); ?>
                    <div style="display: none" class="add_new_lang">
                        <div class="language_item language_item_input" style="display: none">
                            <div class="cs_data_render_buttons">
								<?php renderSelectLanguagesNew( false, $key, $id, 'renderTextInput', $meta, 'middle' ); ?>
                            </div>

                            <div class="cs_data_render_wrap">
                                <div class="input_row cs_data_render">
                                </div>
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
	<?php
}

function render_input_text_new_template( $key, $meta, $title, $input_title ) {
	$current_lang = get_locale();
	?>

    <div class="language_list_block">
        <div class="cart_line_item input_row_default mb-8">
            <div class="cs_data_render_wrap cs_data_render_wrap_input">
                <div class="input_row cs_data_render">
					<?php if ( !empty( $input_title ) ): ?>
                        <h4 class="font-semibold mb-4"><?php echo esc_html( $input_title ); ?></h4>
					<?php endif; ?>
					<?php renderTextInput( $key, '', false, $current_lang, $meta, '', 'js-control-element' ); ?>
                </div>
            </div>
        </div>

        <div class="card card-style4">
            <div class="card-header card-header-style4">
				<?php
				$text = !empty( $input_title ) ? $input_title : $title;
				render_additional_lang_button( $text ); ?>
				<?php cardCollapseSettingsWithText( 'Show' ); ?>
            </div>

            <div class="card-body">
                <div class="language_list">
                </div>

                <div class="add_new_lang_wrap">
					<?php render_add_lang_button( $key, 'renderTextInput' ); ?>
                    <div style="display: none" class="add_new_lang">
                        <div class="language_item language_item_input" style="display: none">
                            <div class="cs_data_render_buttons">
								<?php renderSelectLanguagesNew( false, $key, false, 'renderTextInput', $meta, 'middle' ); ?>
                            </div>

                            <div class="cs_data_render_wrap">
                                <div class="input_row cs_data_render">
                                </div>
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
	<?php
}

function render_language_block_textarea_ajax( $key, $input_title, $input_subtitle ) {
	$current_lang = get_locale();

	$translation  = ConsentMagic()->getLangOption( $key, $current_lang );
	$display_lang = $current_lang;
	if ( empty( $translation ) ) {
		$translation  = ConsentMagic()->getLangOption( $key, CMPRO_DEFAULT_LANGUAGE );
		$display_lang = CMPRO_DEFAULT_LANGUAGE;
	}

	?>
    <div class="language_list_block gap-22">
        <div class="input_row_default">
			<?php if ( !empty( $input_subtitle ) ): ?>
                <h4 class="font-semibold mb-8">
					<?php echo esc_html( $input_subtitle ); ?>:
                </h4>
			<?php endif; ?>

            <div class="cs_data_render_wrap">
				<?php renderWpEditor( $key, $translation, $display_lang, 'js-control-element' ); ?>
            </div>
        </div>

        <div class="card card-style4">
            <div class="card-header card-header-style4">
				<?php
				render_additional_lang_button( $input_title ); ?>

                <div class="cs_load_langs"
                     data-option-type="0"
                     data-current-lang="<?php echo esc_attr( $display_lang ); ?>"
                     data-key="<?php echo esc_attr( $key ); ?>"
                     data-render="renderWpEditor">

					<?php cardCollapseSettingsWithText( 'Show' ); ?>
                </div>
            </div>

            <div class="card-body">
                <div class="language_list_container language_list_container_ajax">
					<?php render_hidden_checkbox( sprintf( '%s_language_control', $key ), 0, 'cs_input_language_control' ); ?>
					<?php render_lang_loader(); ?>

                    <div class="loading_body">
                        <div class="language_list"></div>
                        <div class="add_new_lang_wrap">
							<?php render_add_lang_button( $key, 'renderWpEditor' ); ?>
                            <div style="display: none" class="add_new_lang">
                                <div class="language_item language_item_textarea" style="display: none;">

                                    <div class="cs_data_render_wrap cs_data_render_wrap_editor">
                                        <div class="input_row cs_data_render">
                                        </div>
                                    </div>

                                    <div class="cs_data_render_buttons">
                                        <h4 class="font-semibold-type2 mb-4"><?php esc_html_e( 'Language', 'consent-magic' ); ?></h4>
										<?php renderSelectLanguagesNew( false, $key, false, 'renderWpEditor' ); ?>
                                    </div>

                                    <div>
										<?php include CMPRO_PLUGIN_VIEWS_PATH .
										              'admin/buttons/admin-lang-delete.php'; ?>
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
}

function render_language_block_input_ajax( $key, $input_title, $input_subtitle ) {
	$current_lang             = get_locale();
	$cs_user_default_language = ConsentMagic()->getOption( 'cs_user_default_language' );
	$cs_language_availability = ConsentMagic()->getOption( 'cs_language_availability' );

	if ( isset( $cs_language_availability[ $current_lang ] ) && $cs_language_availability[ $current_lang ] == 0 ) {
		$current_lang = $cs_user_default_language;
	}

	$translation  = ConsentMagic()->getLangOption( $key, $current_lang );
	$display_lang = $current_lang;
	if ( empty( $translation ) ) {
		$translation  = ConsentMagic()->getLangOption( $key, CMPRO_DEFAULT_LANGUAGE );
		$display_lang = CMPRO_DEFAULT_LANGUAGE;
	}

	?>

    <div class="language_list_block">
        <div class="cart_line_item input_row_default mb-8">
			<?php if ( !empty( $input_subtitle ) ): ?>
                <h4 class="font-semibold mb-4">
					<?php echo esc_html( $input_subtitle ); ?>:
                </h4>
			<?php endif; ?>
            <div class="cs_data_render_wrap cs_data_render_wrap_input">
                <div class="input_row cs_data_render">
					<?php renderTextInput( $key, $translation, null, $display_lang, false, '', 'js-control-element' ); ?>
                </div>
            </div>
        </div>

        <div class="card card-style4">
            <div class="card-header card-header-style4">
				<?php

				render_additional_lang_button( $input_title ); ?>

                <div class="cs_load_langs"
                     data-option-type="0"
                     data-current-lang="<?php echo esc_attr( $display_lang ); ?>"
                     data-key="<?php echo esc_attr( $key ); ?>"
                     data-render="renderTextInput">

					<?php cardCollapseSettingsWithText( 'Show' ); ?>
                </div>
            </div>

            <div class="card-body">
                <div class="language_list_container language_list_container_ajax">
					<?php render_hidden_checkbox( sprintf( '%s_language_control', $key ), 0, 'cs_input_language_control' ); ?>
					<?php render_lang_loader(); ?>

                    <div class="loading_body">
                        <div class="language_list"></div>

                        <div class="add_new_lang_wrap">
							<?php render_add_lang_button( $key, 'renderTextInput' ); ?>
                            <div style="display: none" class="add_new_lang">
                                <div class="language_item language_item_input" style="display: none">
									<?php renderSelectLanguagesNew( false, $key, false, 'renderTextInput', false, 'middle' ); ?>

                                    <div class="cs_data_render_wrap cs_data_render_wrap_text_input">
                                        <div class="input_row cs_data_render">
                                        </div>
                                    </div>

                                    <div>
										<?php include CMPRO_PLUGIN_VIEWS_PATH .
										              'admin/buttons/admin-lang-delete.php'; ?>
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
}