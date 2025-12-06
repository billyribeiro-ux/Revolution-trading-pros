<?php

namespace ConsentMagicPro;

// If this file is called directly, abort.
if ( !defined( 'WPINC' ) ) {
	die;
}

function renderUpgradeBlock( $url = null, $text = null, $label = null ) {

	if ( !$url ) {
		$url = 'https://www.pixelyoursite.com/plugins/consentmagic';
	}

	if ( !$text ) {
		$text = __( 'With the PRO version you can have multiple rules with Geo-Targeting, including pre-defined rules', 'consent-magic' );;
	}

	if ( !$label ) {
		$label = __( 'Upgrade', 'consent-magic' );
	}

	$url = untrailingslashit( $url ) . '/?utm_source=pys-free-plugin&utm_medium=pro-badge&utm_campaign=pro-feature';

	$output = '';

	$output .= '<div class="upgrade-block-wrap">';
	$output .= '<div class="row align-items-center">';
	$output .= '<div>';
	$output .= '<p>' . esc_html__( $text ) . '</p>';
	$output .= '</div>';
	$output .= '<div>';
	$output .= '<a href="' . esc_url( $url ) . '" target="_blank" rel="nofollow" class="btn btn-pro"><i class="icon-upgrade"></i> ' . esc_html__( $label ) . '</a>';
	$output .= '</div>';
	$output .= '</div>';
	$output .= '</div>';

	return $output;
}

/**
 * Output number input
 * @param      $key
 * @param null $placeholder
 * @param bool $disabled
 * @param bool $design
 * @param bool $meta
 * @param null $id
 */
function render_number_input( $key, $placeholder = null, $disabled = false, $design = false, $meta = false, $id = null ) {
	$unic = uniqid();
	if ( $design ) {
		$attr_name  = "cs_d[" . ConsentMagic()->plugin_name . "][$key]";
		$attr_id    = 'cs_d_' . ConsentMagic()->get_plugin_name() . '_' . $key . '_' . $unic;
		$attr_value = $meta ? get_post_meta( $id, $key, true ) : ConsentMagic()->getOption( $key );
	} else {
		$attr_name  = "cs[" . ConsentMagic()->plugin_name . "][$key]";
		$attr_id    = 'cs_' . ConsentMagic()->get_plugin_name() . '_' . $key . '_' . $unic;
		$attr_value = $meta ? get_post_meta( $id, $key, true ) : ConsentMagic()->getOption( $key );
	}
	?>

    <div class="input-number-wrapper">
        <button class="decrease"><i class="icon-minus"></i></button>
        <input <?php disabled( $disabled ); ?>
                type="number"
                name="<?php esc_attr_e( $attr_name ); ?>"
                id="<?php esc_attr_e( $attr_id ); ?>"
                value="<?php echo esc_attr( $attr_value ); ?>"
                placeholder="<?php esc_attr_e( $placeholder ); ?>"
                min="0"
        >
        <button class="increase"><i class="icon-plus"></i></button>
    </div>

	<?php
}

/**
 * Output number input like text input
 * @param        $key
 * @param null   $placeholder
 * @param bool   $disabled
 * @param bool   $design
 * @param bool   $meta
 * @param null   $id
 * @param string $type
 * @return void
 */
function render_number_text_input( $key, $placeholder = null, $disabled = false, $design = false, $meta = false, $id = null, $type = 'short' ) {
	$unic = uniqid();
	if ( $design ) {
		$attr_name = "cs_d[" . ConsentMagic()->plugin_name . "][$key]";
		$attr_id   = 'cs_d_' . ConsentMagic()->get_plugin_name() . '_' . $key . '_' . $unic;
	} else {
		$attr_name = "cs[" . ConsentMagic()->plugin_name . "][$key]";
		$attr_id   = 'cs_' . ConsentMagic()->get_plugin_name() . '_' . $key . '_' . $unic;
	}
	$attr_value = $meta ? get_post_meta( $id, $key, true ) : ConsentMagic()->getOption( $key );
	?>

    <input <?php disabled( $disabled ); ?>
            type="number"
            name="<?php esc_attr_e( $attr_name ); ?>"
            id="<?php esc_attr_e( $attr_id ); ?>"
            value="<?php echo esc_attr( $attr_value ); ?>"
            placeholder="<?php esc_attr_e( $placeholder ); ?>"
            class="form-control input-<?php echo esc_attr( $type ); ?>"
            min="0"
    >
	<?php
}

/**
 * Output checkbox input stylized as switcher
 * @param      $key
 * @param bool $json
 * @param bool $meta
 * @param null $id
 * @param bool $collapse
 * @param bool $disabled
 * @param bool $label
 * @param bool $default
 * @param bool $custom_classes
 */
function render_switcher_input( $key, $json = true, $meta = false, $id = null, $collapse = false, $disabled = false, $label = false, $default = false, $custom_classes = false ) {
	$unic = uniqid();
	if ( $json ) {
		$attr_id    = 'cs_' . ConsentMagic()->get_plugin_name() . '_' . $unic;
		$attr_value = ConsentMagic()->getOption( $key );
	} else {
		$attr_id    = $key . '_' . $unic;
		$attr_value = $meta ? get_post_meta( $id, $key, true ) : ConsentMagic()->getOption( $key );
	}

	$attr_name = "cs[" . ConsentMagic()->plugin_name . "][$key]";

	$classes = array( 'primary-switch' );

	if ( $collapse ) {
		$classes[] = 'collapse-control';
	}

	if ( $custom_classes ) {
		$classes[] = $custom_classes;
	}

	$classes = implode( ' ', $classes );

	if ( $default === false ) {
		$default = $attr_value;
	}

	?>
    <div class="<?php esc_attr_e( $classes ); ?>">
		<?php if ( !$disabled ) : ?>
            <input type="hidden" name="<?php echo esc_attr( $attr_name ); ?>" value="0"
                   class="<?php echo esc_attr( $key ); ?>">
		<?php endif; ?>

        <input type="checkbox"
               name="<?php echo esc_attr( $attr_name ); ?>"
               value="1"
			<?php disabled( $disabled ); ?>
			<?php checked( $default ); ?>
               id="<?php echo esc_attr( $attr_id ); ?>"
               class="primary-switch-input <?php echo esc_attr( $key ); ?>"
			<?php if ( $collapse ) : ?>
                data-target="cs_<?php echo esc_attr( ConsentMagic()->get_plugin_name() ); ?>_<?php echo esc_attr( $key ); ?>_panel"
			<?php endif; ?>
        >

        <label class="primary-switch-btn" for="<?php echo esc_attr( $attr_id ); ?>"></label>
		<?php if ( $label ) : ?>
            <h4 class="switcher-label secondary-heading"><?php echo esc_html( $label ); ?></h4>
		<?php endif; ?>
    </div>

	<?php
}

/**
 * Render static swither input
 * @param        $key
 * @param        $value
 * @param bool   $collapse
 * @param bool   $disabled
 * @param bool   $label
 * @param string $additional_classes
 * @param array  $dataset
 * @param bool   $this_form
 * @return string
 */
function render_static_switcher_input( $keys, $value, $collapse = false, $disabled = false, $label = false, $additional_classes = '', $dataset = array(), $this_form = true ) {
	$unic = uniqid();

	$names     = '[' . implode( '][', $keys ) . ']';
	$key_names = implode( '_', $keys );
	$attr_name = "cs[" . ConsentMagic()->plugin_name . "]$names";
	$attr_id   = 'cs_' . $key_names . '_' . $unic;

	$classes = array( 'primary-switch' );

	if ( $collapse ) {
		$classes[] = 'collapse-control';
	}

	$additional_classes_input = array();
	if ( $additional_classes ) {
		$additional_classes_input[] = $additional_classes;
	}

	$form = '';
	if ( !$this_form ) {
		$form = 'form=form' . $unic;
	}

	$data = '';
	if ( !empty( $dataset ) ) {
		foreach ( $dataset as $key => $item ) {
			$data .= ' data-' . $key . '="' . htmlspecialchars( $item, ENT_QUOTES, 'UTF-8' ) . '"';
		}
	}

	$classes                  = implode( ' ', $classes );
	$additional_classes_input = implode( ' ', $additional_classes_input );

	?>
    <div class="<?php echo esc_attr( $classes ); ?>">
		<?php if ( !$disabled ) : ?>
            <input <?php echo esc_html( $form ); ?>
                    type="hidden"
                    name="<?php echo esc_attr( $attr_name ); ?>"
                    value="0"
                    class="<?php echo esc_attr( $key_names ); ?>">
		<?php endif; ?>
		<?php if ( $collapse ) : ?>
            <input <?php echo esc_html( $form ); ?>
                    type="checkbox"
                    name="<?php echo esc_attr( $attr_name ); ?>"
                    value="1"
				<?php disabled( $disabled ); ?>
				<?php checked( $value ); ?>
                    id="<?php echo esc_attr( $attr_id ); ?>"
                    class="primary-switch-input <?php echo esc_attr( $key_names ); ?> <?php echo esc_attr( $additional_classes_input ); ?>"
                    data-target="cs_<?php echo esc_attr( ConsentMagic()->get_plugin_name() ); ?>_<?php echo esc_attr( $key_names ); ?>_panel"
				<?php echo esc_attr( $data ); ?>>
		<?php else : ?>
            <input <?php echo esc_html( $form ); ?>
                    type="checkbox"
                    name="<?php echo esc_attr( $attr_name ); ?>"
                    value="1"
				<?php disabled( $disabled ); ?>
				<?php checked( $value ); ?>
                    id="<?php echo esc_attr( $attr_id ); ?>"
                    class="primary-switch-input <?php echo esc_attr( $key_names ); ?> <?php echo esc_attr( $additional_classes_input ); ?>"
				<?php echo wp_kses_post( $data ); ?>>
		<?php endif; ?>
        <label class="primary-switch-btn" for="<?php echo esc_attr( $attr_id ); ?>"></label>
		<?php if ( $label ) : ?>
            <label class="switcher-label secondary-heading"><?php echo esc_html( $label ); ?></label>
		<?php endif; ?>

    </div>
	<?php
}


/**
 * Output checkbox input stylized as switcher
 * @param      $key
 * @param bool $json
 * @param bool $meta
 * @param null $id
 * @param bool $collapse
 * @param bool $disabled
 * @param bool $label
 * @param bool $default
 * @param bool $custom_classes
 */
function render_checkbox_input( $key, $json = true, $meta = false, $id = null, $collapse = false, $disabled = false, $label = false, $default = false, $custom_classes = false ) {
	$unic = uniqid();
	if ( $json ) {
		$attr_id    = 'cs_' . ConsentMagic()->get_plugin_name() . '_' . $unic;
		$attr_value = ConsentMagic()->getOption( $key );
	} else {
		$attr_id    = $key . '_' . $unic;
		$attr_value = $meta ? get_post_meta( $id, $key, true ) : ConsentMagic()->getOption( $key );
	}

	$attr_name = "cs[" . ConsentMagic()->plugin_name . "][$key]";

	$classes = array( 'small-checkbox' );

	if ( $collapse ) {
		$classes[] = 'collapse-control';
	}

	if ( $custom_classes ) {
		$classes[] = $custom_classes;
	}

	$classes = implode( ' ', $classes );

	if ( $default === false ) {
		$default = $attr_value;
	}

	?>
    <div class="<?php echo esc_attr( $classes ); ?>">
		<?php if ( !$disabled ) : ?>
            <input type="hidden" name="<?php echo esc_attr( $attr_name ); ?>" value="0"
                   class="<?php echo esc_attr( $key ); ?>">
		<?php endif; ?>
		<?php if ( $collapse ) : ?>
            <input type="checkbox"
                   name="<?php echo esc_attr( $attr_name ); ?>"
                   value="1"
				<?php disabled( $disabled ); ?>
				<?php checked( $default ); ?>
                   id="<?php echo esc_attr( $attr_id ); ?>"
                   class="small-control-input <?php echo esc_attr( $key ); ?>"
                   data-target="cs_<?php echo esc_attr( ConsentMagic()->get_plugin_name() ); ?>_<?php echo esc_attr( $key ); ?>'_panel">
		<?php else : ?>
            <input type="checkbox"
                   name="<?php echo esc_attr( $attr_name ); ?>"
                   value="1"
				<?php disabled( $disabled ); ?>
				<?php checked( $default ); ?>
                   id="<?php echo esc_attr( $attr_id ); ?>"
                   class="small-control-input <?php echo esc_attr( $key ); ?>">
		<?php endif; ?>

        <label class="small-control small-checkbox-label" for="<?php echo esc_attr( $attr_id ); ?>">
            <span class="small-control-indicator"><i class="icon-check"></i></span>
			<?php if ( $label ) : ?>
                <span class="small-control-description"><?php echo wp_kses_post( $label ); ?></span>
			<?php endif; ?>
        </label>
    </div>
	<?php
}

function render_hidden_checkbox( $key, $value = null, $class = null ) {
	$key = "cs[" . ConsentMagic()->plugin_name . "][$key]";
	?>
    <input type="hidden" name="<?php echo esc_attr( $key ); ?>" value="<?php echo esc_attr( $value ); ?>"
           class="<?php echo esc_attr( $class ); ?>">
	<?php
}

/**
 * Output checkbox input stylized as switcher
 * @param        $key
 * @param bool   $json
 * @param bool   $meta
 * @param null   $id
 * @param bool   $collapse
 * @param bool   $disabled
 * @param bool   $label
 * @param bool   $default
 * @param null   $additional_styles
 * @param string $additional_classes
 * @param bool   $input_actions
 * @param array  $dataset
 * @return string
 */
function render_switcher_input_front( $key, $json = true, $meta = false, $id = null, $collapse = false, $disabled = false, $label = false, $default = false, $additional_styles = null, $additional_classes = '', $input_actions = true, $dataset = array() ) {
	if ( $json ) {
		$attr_name  = "cs[" . ConsentMagic()->plugin_name . "][$key]";
		$attr_id    = 'cs_' . ConsentMagic()->get_plugin_name();
		$attr_value = ConsentMagic()->getOption( $key );
	} else {
		$attr_name  = $key;
		$attr_id    = $key;
		$attr_value = $meta ? get_post_meta( (int) $id, $key, true ) : ConsentMagic()->getOption( $key );
	}

	$classes = array( 'primary-switch' );

	if ( $collapse ) {
		$classes[] = 'collapse-control';
	}

	$classes = implode( ' ', $classes );

	if ( !$default ) {
		$default = $attr_value;
	}

	if ( $input_actions ) {
		$input_class = 'cs_script_action';
	} else {
		$input_class = '';
	}

	$data = '';
	if ( !empty( $dataset ) ) {
		foreach ( $dataset as $key => $item ) {
			$data .= ' data-' . $key . '="' . htmlspecialchars( $item, ENT_QUOTES, 'UTF-8' ) . '"';
		}
	}

	?>

    <div class="<?php esc_attr_e( $classes ); ?>">
		<?php if ( !$disabled ) : ?>
            <input type="hidden"
                   name="<?php echo esc_attr( $attr_name ); ?>"
                   value="0"
                   class="<?php echo esc_attr( $key ); ?>">
		<?php endif; ?>
		<?php if ( $collapse ) : ?>
            <input type="checkbox"
                   name="<?php echo esc_attr( $attr_name ); ?>"
                   value="1"
				<?php disabled( $disabled ); ?>
				<?php checked( $default ); ?>
                   id="<?php echo esc_attr( $attr_id ); ?>"
                   class="primary-switch-input <?php echo esc_attr( $input_class ); ?> <?php echo esc_attr( $key ) ?> <?php echo esc_attr( $additional_classes ); ?>"
                   data-target="cs_<?php echo esc_attr( ConsentMagic()->get_plugin_name() ); ?>_<?php echo esc_attr( $key ); ?>_panel" <?php echo esc_attr( $data ); ?>>
		<?php else: ?>
            <input type="checkbox"
                   name="<?php echo esc_attr( $attr_name ); ?>"
                   value="1"
				<?php disabled( $disabled ); ?>
				<?php checked( $default ); ?>
                   id="<?php echo esc_attr( $attr_id ); ?>"
                   class="primary-switch-input <?php echo esc_attr( $input_class ); ?> <?php echo esc_attr( $key ); ?> <?php echo esc_attr( $additional_classes ); ?>" <?php echo wp_kses_post( $data ); ?>>
		<?php endif; ?>
        <label class="primary-switch-btn" for="<?php echo esc_attr( $attr_id ); ?>"
               style="<?php echo esc_attr( $additional_styles ); ?>"></label>
		<?php if ( $label ) : ?>
            <h4 class="switcher-label secondary-heading"><?php echo esc_html( $label ); ?></h4>
		<?php endif; ?>
    </div>
	<?php
}


/**
 * Output checkbox input stylized as switcher
 * @param      $key
 * @param bool $json
 * @param bool $meta
 * @param null $id
 * @param bool $collapse
 * @param bool $disabled
 * @param bool $label
 */
function render_several_switcher_input( $key, $json = true, $meta = false, $id = null, $collapse = false, $disabled = false, $label = false ) {
	if ( $json ) {
		$attr_name  = "cs[" . ConsentMagic()->plugin_name . "][$key]";
		$attr_id    = 'cs_' . ConsentMagic()->get_plugin_name() . '_' . $key;
		$attr_value = ConsentMagic()->getOption( $key );
	} else {
		$attr_name  = $key . '_' . $id;
		$attr_id    = $key . '_' . $id;
		$attr_value = $meta ? get_post_meta( $id, $key, true ) : ConsentMagic()->getOption( $key );
	}

	$classes = array( 'primary-switch' );

	if ( $collapse ) {
		$classes[] = 'collapse-control';
	}

	$classes = implode( ' ', $classes );

	?>

    <div class="<?php esc_attr_e( $classes ); ?>">
		<?php if ( !$disabled ) : ?>
            <input type="hidden" name="<?php echo esc_attr( $attr_name ); ?>" value="0"
                   class="<?php echo esc_attr( $key ); ?>">
		<?php endif; ?>

        <input type="checkbox"
               name="<?php echo esc_attr( $attr_name ); ?>"
               value="1"
			<?php disabled( $disabled ); ?>
			<?php checked( $attr_value ); ?>
               id="<?php echo esc_attr( $attr_id ); ?>"
               class="primary-switch-input <?php echo esc_attr( $key ); ?>"
			<?php if ( $collapse ) : ?>
                data-target="cs_<?php echo esc_attr( ConsentMagic()->get_plugin_name() ); ?>_<?php echo esc_attr( $key ); ?>_panel"
			<?php endif; ?>
        >

        <label class="primary-switch-btn" for="<?php echo esc_attr( $attr_id ); ?>"></label>
		<?php if ( $label ) : ?>
            <h4 class="switcher-label secondary-heading"><?php echo esc_html( $label ); ?></h4>
		<?php endif; ?>
    </div>
	<?php
}

/**
 * Output radio input
 * @param      $key
 * @param      $value
 * @param      $label
 * @param bool $disabled
 */
function render_radio_input( $key, $value, $label, $disabled = false ) {

	$attr_value = ConsentMagic()->getOption( $key );
	$attr_name = "cs[" . ConsentMagic()->plugin_name . "][$key]";
	$id        = $key . "_" . rand( 1, 1000000 );
	?>

    <div class="radio-standard">
        <input type="radio" name="<?php esc_attr_e( $attr_name ); ?>"
			<?php disabled( $disabled, true ); ?>
			<?php echo ( $attr_value == $value ) ? 'checked="checked"' : ''; ?>
               class="radio-control-input"
               id="<?php echo esc_attr( $id ); ?>"
               value="<?php echo esc_attr( $value ); ?>">
        <label class="radio-checkbox-label" for="<?php echo esc_attr( $id ); ?>">
            <span class="radio-control-indicator"></span>
            <span class="radio-control-description"><?php echo esc_html( $label ); ?></span>
        </label>
    </div>
	<?php
}

/**
 * @param string $key
 * @param string $placeholder
 * @param string $class
 */
function renderTextarea( $key, $placeholder = '', $class = '', $full_width = false, $meta = false, $id = null ) {

	$attr_name  = "cs[" . ConsentMagic()->plugin_name . "][$key]";
	$attr_id    = 'cs_' . $key;
	$attr_value = $meta ? get_post_meta( $id, $key, true ) : ConsentMagic()->getOption( $key );
	$attr_width = $full_width ? 'width: 100%;' : '';
	?>

    <textarea name="<?php esc_attr_e( $attr_name ); ?>"
              id="<?php esc_attr_e( $attr_id ); ?>"
              placeholder="<?php esc_attr_e( $placeholder ); ?>"
              style="<?php esc_attr_e( $attr_width ); ?>"
              class="form-control textarea-standard <?php echo esc_attr( $class ); ?>"><?php echo esc_attr( $attr_value ); ?></textarea>

	<?php
}

/**
 * @param string $key
 * @param        $lang
 * @param string $class
 * @param bool   $meta
 * @param null   $id
 */
function renderWpEditor( $key, $value, $lang, $class = '' ) {

	if ( !class_exists( '_WP_Editors', false ) ) {
		wp_enqueue_editor();
	}

	if ( $lang === false ) {
		$attr_id   = 'cs_' . $key;
		$attr_name = "cs[" . ConsentMagic()->plugin_name . "][$key]";
	} else {
		$attr_name = "cs[" . ConsentMagic()->plugin_name . "][text][$key][$lang]";
		$attr_id   = $key . '_' . uniqid();
	}

	wp_editor( wp_kses_post( $value ), esc_attr( $attr_id ), array(
		'wpautop'          => 1,
		'media_buttons'    => 0,
		'textarea_name'    => esc_attr( $attr_name ),
		'textarea_rows'    => 5,
		'tabindex'         => null,
		'editor_css'       => '',
		'editor_class'     => esc_attr( $class ),
		'teeny'            => 1,
		'dfw'              => 0,
		'quicktags'        => 1,
		'drag_drop_upload' => false,
		'tinymce'          => array(
			'content_css' => CMPRO_PLUGIN_URL . '/assets/css/style-wp-editor.min.css?ver=' . CMPRO_LATEST_VERSION_NUMBER
		)
	) );
}

/**
 * It uses for nolang editors
 * @param string $key
 * @param string $class
 * @param bool   $meta
 * @param null   $id
 */
function renderSimpleWpEditor( $key, $class = '', $meta = false, $id = null ) {

	if ( !class_exists( '_WP_Editors', false ) ) {
		wp_enqueue_editor();
	}

	$attr_name = "cs[" . ConsentMagic()->plugin_name . "][$key]";
	$attr_id   = 'cs_' . $key;

	$attr_value = $meta ? get_post_meta( $id, $key, true ) : ConsentMagic()->getOption( $key );

	wp_editor( wp_kses_post( $attr_value ), esc_attr( $attr_id ), array(
		'wpautop'          => 1,
		'media_buttons'    => 0,
		'textarea_name'    => esc_attr( $attr_name ),
		'textarea_rows'    => 10,
		'tabindex'         => null,
		'editor_css'       => '',
		'editor_class'     => esc_attr( $class ),
		'teeny'            => 1,
		'dfw'              => 0,
		'quicktags'        => 1,
		'drag_drop_upload' => false,
		'tinymce'          => array(
			'content_css' => CMPRO_PLUGIN_URL . '/assets/css/style-wp-editor.min.css?ver=' . CMPRO_LATEST_VERSION_NUMBER
		)
	) );
}

/**
 * @param        $term
 * @param string $key
 * @param        $value
 * @param string $class
 */
function renderWpEditorByTerm( $term, $key, $value, $class = '' ) {

	if ( !class_exists( '_WP_Editors', false ) ) {
		wp_enqueue_editor();
	}

	if ( $term ) {
		wp_editor( wp_kses_post( $value ), 'cs_edit_cat_description_' . esc_attr( $term->term_id ) . uniqid(), array(
			'wpautop'          => 1,
			'media_buttons'    => 0,
			'textarea_rows'    => 5,
			'textarea_name'    => "cs[" . ConsentMagic()->plugin_name . "][categories][" . $term->term_id . "][" . $term->display_lang . "][" . $key . "]",
			'tabindex'         => null,
			'editor_css'       => '',
			'editor_class'     => esc_attr( $class ),
			'teeny'            => 1,
			'dfw'              => 0,
			'quicktags'        => 1,
			'drag_drop_upload' => false,
			'tinymce'          => array(
				'content_css' => CMPRO_PLUGIN_URL . '/assets/css/style-wp-editor.min.css?ver=' . CMPRO_LATEST_VERSION_NUMBER
			)
		) );
	} else {
		wp_editor( '', 'cs_new_cat_description' . uniqid(), array(
			'wpautop'          => 1,
			'media_buttons'    => 0,
			'textarea_rows'    => 5,
			'textarea_name'    => "cs[" . ConsentMagic()->plugin_name . "][new_category][" . $key . "]",
			'tabindex'         => null,
			'editor_css'       => '',
			'editor_class'     => esc_attr( $class ),
			'teeny'            => 1,
			'dfw'              => 0,
			'quicktags'        => 1,
			'drag_drop_upload' => false,
			'tinymce'          => array(
				'content_css' => CMPRO_PLUGIN_URL . '/assets/css/style-wp-editor.min.css?ver=' . CMPRO_LATEST_VERSION_NUMBER
			)
		) );
	}
}

/**
 * @param $key
 * @param $value
 * @param $id
 * @param $lang
 * @param $meta
 * @param $placeholder
 * @param $class
 * @param $required
 * @param $disabled
 * @return void
 */
function renderTextInput( $key, $value = null, $id = null, $lang = CMPRO_DEFAULT_LANGUAGE, $meta = false, $placeholder = '', $class = '', $required = false, $disabled = false ): void {

	if ( $lang === false ) {
		$attr_id   = 'cs_' . $key;
		$attr_name = "cs[" . ConsentMagic()->plugin_name . "][$key]";
        if ( $value === null ) {
            if ( $meta ) {
                $value = get_post_meta( $id, $key, true );
            } else {
                $value = ConsentMagic()->getOption( $key );
            }
        }
	} else {
		$attr_name = "cs[" . ConsentMagic()->plugin_name . "][text][$key][$lang]";
		$attr_id   = $key . '_' . uniqid();
	}

	?>
    <input type="text"
           name="<?php echo esc_attr( $attr_name ); ?>"
           id="<?php echo esc_attr( $attr_id ); ?>"
           placeholder="<?php esc_attr_e( $placeholder ); ?>"
           class="form-control input-full <?php echo esc_attr( $class ); ?>"
           value="<?php echo esc_attr( $value ); ?>" <?php esc_attr_e( $required ); ?> <?php disabled( $disabled ); ?> />
	<?php
}

function renderTextInputTerm( $key, $term, $placeholder = '', $class = '', $required = false, $disabled = false ) {

	$attr_name = "cs[" . ConsentMagic()->plugin_name . "][categories][" . $term->term_id . "][" . $term->display_lang . "][" . $key . "]";
	$attr_id   = 'cs_' . $key . '_' . $term->term_id;

	?>
    <input type="text"
           name="<?php echo esc_attr( $attr_name ); ?>"
           id="<?php echo esc_attr( $attr_id ); ?>"
           placeholder="<?php esc_attr_e( $placeholder ); ?>"
           class="form-control input-full <?php echo esc_attr( $class ); ?>"
           value="<?php echo esc_attr( $term->name_l ); ?>"
		<?php esc_attr_e( $required ); ?>
		<?php disabled( $disabled ); ?> />
	<?php
}

/**
 * @param string $key
 * @param string $placeholder
 * @param string $class
 */
function renderProofDeletingEmail( $key = 'cs_send_proof_deleting_email', $full_width = false, $type = 'middle' ) {
	$attr_id   = 'cs_' . $key;
	$attr_name = "cs[" . ConsentMagic()->plugin_name . "][$key]";
	if ( ConsentMagic()->getOption( $key ) ) {
		$attr_value = ConsentMagic()->getOption( $key );
	} else {
		$attr_value = ConsentMagic()->getOption( 'cs_autodetect_email' );
	}

	$attr_width = $full_width ? 'width: 100%;' : '';

	?>

    <input type="text" name="<?php esc_attr_e( $attr_name ); ?>"
           id="<?php esc_attr_e( $attr_id ); ?>"
           class="form-control input-<?php echo esc_attr( $type ); ?>" value="<?php esc_attr_e( $attr_value ); ?>"
           style="<?php esc_attr_e( $attr_width ); ?>"/>
	<?php
}

/**
 * @param string $key
 * @param string $placeholder
 * @param string $class
 */
function renderColorPickerInput( $key, $id = null, $placeholder = '', $class = '' ) {
	$attr_name  = "cs[" . ConsentMagic()->plugin_name . "][$key]";
	$attr_id    = 'cs_' . $key;
	$attr_value = $id ? get_post_meta( $id, $key, true ) : ConsentMagic()->getOption( $key );

	?>

    <input name="<?php esc_attr_e( $attr_name ); ?>"
           id="<?php esc_attr_e( $attr_id ); ?>"
           placeholder="<?php esc_attr_e( $placeholder ); ?>"
           class="form-control color-field input-no-style <?php echo esc_attr( $class ); ?>"
           value="<?php echo esc_attr( $attr_value ); ?>"/>

	<?php
}

/**
 * @param string $key
 * @param array  $options
 * @param string $full_width
 */
function renderSelectInput( $key, $options = array(), $full_width = false, $design = false, $type = 'standard' ) {
	if ( $design ) {
		$attr_name  = "cs_d[" . ConsentMagic()->plugin_name . "][$key]";
		$attr_id    = 'cs_d_' . ConsentMagic()->get_plugin_name() . '_' . $key;
		$attr_value = ConsentMagic()->getOption( $key );
		$attr_width = $full_width ? 'width: 100%;' : '';
	} else {
		$attr_name  = "cs[" . ConsentMagic()->plugin_name . "][$key]";
		$attr_id    = 'cs_' . $key;
		$attr_value = ConsentMagic()->getOption( $key );
		$attr_width = $full_width ? 'width: 100%;' : '';
	}

	?>
    <div class="select-wrap select-<?php echo esc_attr( $type ); ?>-wrap">
        <select id="<?php esc_attr_e( $attr_id ); ?>"
                name="<?php esc_attr_e( $attr_name ); ?>" autocomplete="off"
                class="form-control-cm"
                style="<?php esc_attr_e( $attr_width ); ?>">
			<?php foreach ( $options as $option_key => $option_value ) : ?>
				<?php if ( is_array( $option_value ) ) {
					?>
                    <option value="<?php echo esc_attr( $option_value[ 'option_value' ] ); ?>" <?php selected( $option_value[ 'option_value' ], esc_attr( $attr_value ) ); ?>><?php echo esc_attr( $option_value[ 'option_name' ] ); ?></option>
				<?php } else { ?>
                    <option value="<?php echo esc_attr( $option_key ); ?>" <?php selected( $option_key, is_array( $attr_value ) ? esc_attr( $attr_value[ mb_strtolower( $option_value ) ] ) : esc_attr( $attr_value ) ); ?>><?php echo esc_attr( $option_value ); ?></option>
				<?php } endforeach; ?>
        </select>
    </div>
	<?php
}

/**
 * Render select input with static values
 * @param        $key
 * @param        $value
 * @param array  $options
 * @param bool   $full_width
 * @param string $classes
 * @param string $type
 * @return void
 */
function renderSelectInputStatic( $keys, $value, $options = array(), $full_width = false, $classes = '', $type = 'standard' ) {

	$unic = uniqid();
	$names     = '[' . implode( '][', $keys ) . ']';
	$key_names = implode( '_', $keys );
	$attr_name = "cs[" . ConsentMagic()->plugin_name . "]$names";
	$attr_id   = 'cs_' . $key_names . '_' . $unic;

	$attr_width = $full_width ? 'width: 100%;' : '';

	?>
    <div class="select-wrap select-<?php echo esc_attr( $type ); ?>-wrap">
        <select class="form-control-cm <?php esc_attr_e( $classes ); ?>" id="<?php esc_attr_e( $attr_id ); ?>"
                name="<?php esc_attr_e( $attr_name ); ?>" autocomplete="off"
                style="<?php esc_attr_e( $attr_width ); ?>">
			<?php foreach ( $options as $option_key => $option_value ) : ?>
				<?php if ( is_array( $option_value ) ) {
					?>
                    <option value="<?php echo esc_attr( $option_value[ 'option_value' ] ); ?>" <?php selected( $option_value[ 'option_value' ], esc_attr( $value ) ); ?>><?php echo esc_attr( $option_value[ 'option_name' ] ); ?></option>
				<?php } else { ?>
                    <option value="<?php echo esc_attr( $option_key ); ?>" <?php selected( $option_key, is_array( $value ) ? esc_attr( $value[ mb_strtolower( $option_value ) ] ) : esc_attr( $value ) ); ?>><?php echo esc_attr( $option_value ); ?></option>
				<?php } endforeach; ?>
        </select>
    </div>

	<?php
}

function renderSelectInputStaticTerm( $key, $term, $value, $options = array(), $full_width = false, $classes = '', $type = 'standard' ) {

	$attr_name  = "cs[" . ConsentMagic()->plugin_name . "][categories][" . $term->term_id . "][" . $term->display_lang . "][" . $key . "]";
	$attr_id    = 'cs_' . $key . '_' . $term->term_id;
	$attr_width = $full_width ? 'width: 100%;' : '';

	?>
    <div class="select-wrap select-<?php echo esc_attr( $type ); ?>-wrap">
        <select class="form-control-cm <?php esc_attr_e( $classes ); ?>" id="<?php esc_attr_e( $attr_id ); ?>"
                name="<?php esc_attr_e( $attr_name ); ?>" autocomplete="off"
                style="<?php esc_attr_e( $attr_width ); ?>">
			<?php foreach ( $options as $option_key => $option_value ) : ?>
				<?php if ( is_array( $option_value ) ) {
					?>
                    <option value="<?php echo esc_attr( $option_value[ 'option_value' ] ); ?>" <?php selected( $option_value[ 'option_value' ], esc_attr( $value ) ); ?>><?php echo esc_attr( $option_value[ 'option_name' ] ); ?></option>
				<?php } else { ?>
                    <option value="<?php echo esc_attr( $option_key ); ?>" <?php selected( $option_key, is_array( $value ) ? esc_attr( $value[ mb_strtolower( $option_value ) ] ) : esc_attr( $value ) ); ?>><?php echo esc_attr( $option_value ); ?></option>
				<?php } endforeach; ?>
        </select>
    </div>

	<?php
}

/**
 * @param string $key
 * @param array  $options
 * @param string $full_width
 */
function get_renderSelectInput( $key, $options = array(), $full_width = false, $design = false ) {
	if ( $design ) {
		$attr_name  = "cs_d[" . ConsentMagic()->plugin_name . "][$key]";
		$attr_id    = 'cs_d_' . ConsentMagic()->get_plugin_name() . '_' . $key;
		$attr_value = ConsentMagic()->getOption( $key );
		$attr_width = $full_width ? 'width: 100%;' : '';
	} else {
		$attr_name  = "cs[" . ConsentMagic()->plugin_name . "][$key]";
		$attr_id    = 'cs_' . $key;
		$attr_value = ConsentMagic()->getOption( $key );
		$attr_width = $full_width ? 'width: 100%;' : '';
	}

	$output = '';

	$output .= '<select class="form-control-cm" id="' . esc_attr( $attr_id ) . '"
	        name="' . esc_attr( $attr_name ) . '" autocomplete="off" style="' . esc_attr( $attr_width ) . '">'; ?>
	<?php foreach ( $options as $option_key => $option_value ) : ?>
		<?php if ( is_array( $option_value ) ) {
			$output .= '<option value="' . esc_attr( $option_value[ 'option_value' ] ) . '" ' . selected( $option_value[ 'option_value' ], esc_attr( $attr_value ) ) . '>' . esc_attr( $option_value[ 'option_name' ] ) . '</option>'; ?>
		<?php } else {
			$output .= '<option value="' . esc_attr( $option_key ) . '" ' . selected( $option_key, is_array( $attr_value ) ? esc_attr( $attr_value[ mb_strtolower( $option_value ) ] ) : esc_attr( $attr_value ) ) . '>' . esc_attr( $option_value ) . '</option>';
		} endforeach; ?>

	<?php
	$output .= '</select>';

	return $output;
}

/**
 * @param string $key
 * @param        $id
 * @param array  $options
 * @param bool   $full_width
 * @param bool   $design
 * @param bool   $disabled
 */
function renderSelectInputMeta( $key, $id, $options, $full_width = false, $design = false, $disabled = false ) {
	if ( $design ) {
		$attr_id    = 'cs_' . $key;
		$attr_value = get_post_meta( $id, $key, true ) ? get_post_meta( $id, $key, true ) : ConsentMagic()->getOption( $key );
	} else {
		$attr_id    = $key;
		$attr_value = get_post_meta( $id, $key, true );

	}

	$attr_name  = "cs[" . ConsentMagic()->plugin_name . "][$key]";
	$attr_width = $full_width ? 'width: 100%;' : '';

	?>

    <div class="select-wrap select-standard-wrap <?php echo $disabled ? 'disabled' : ''; ?>">
        <select class="form-control-cm <?php echo esc_attr( $key ); ?>
                    <?php echo $disabled ? 'disabled' : ''; ?>"
                id="<?php esc_attr_e( $attr_id ); ?>"
                name="<?php esc_attr_e( $attr_name ); ?>" autocomplete="off"
                style="<?php esc_attr_e( $attr_width ); ?>">
			<?php foreach ( $options as $option_key => $option_value ) : ?>
				<?php if ( is_array( $option_value ) ) {
					?>
                    <option value="<?php echo esc_attr( $option_value[ 'option_value' ] ); ?>" <?php selected( $option_value[ 'option_value' ], esc_attr( $attr_value ) ); ?>><?php echo esc_attr( $option_value[ 'option_name' ] ); ?></option>
				<?php } else { ?>
                    <option value="<?php echo esc_attr( $option_key ); ?>" <?php selected( $option_key, is_array( $attr_value ) ? esc_attr( $attr_value[ mb_strtolower( $option_value ) ] ) : esc_attr( $attr_value ) ); ?>><?php echo esc_attr( $option_value ); ?></option>
				<?php } endforeach; ?>
        </select>
    </div>
	<?php
}

/**
 * @param string $key
 * @param bool   $full_width
 */
function renderSelectLocation( $key, $id, $full_width = false ) {

	$attr_id    = $key;
	$attr_width = $full_width ? 'width: 100%;' : '';
	$attr_name  = "cs[" . ConsentMagic()->plugin_name . "][$key]";

	$selected_countries = explode( ",", get_meta_value_by_key( $id, '_cs_target' ) );
	$selected_states    = explode( ",", get_meta_value_by_key( $id, '_cs_us_states_target' ) );
	$countries          = renderCSTypeLocations();
	$states             = renderCSTypeUSStates();

	?>

    <div class="select-wrap select-full-wrap">
        <select class="select-full" id="<?php esc_attr_e( $attr_id ); ?>" multiple data-placeholder="Select Location"
                name="<?php esc_attr_e( $attr_name ); ?>" autocomplete="off"
                style="<?php esc_attr_e( $attr_width ); ?>">
            <option data-value="GDPR"
                    class="country"
                    value="GDPR" <?php selected( in_array( 'GDPR', $selected_countries ) ); ?>><?php esc_html_e( 'Select GDPR countries', 'consent-magic' ); ?></option>
            <option data-value="NONGDPR"
                    class="country"
                    value="NONGDPR" <?php selected( in_array( 'NONGDPR', $selected_countries ) ); ?>><?php esc_html_e( 'Select non - GDPR countries', 'consent-magic' ); ?></option>
			<?php foreach ( $countries as $country_key => $country_value ) : ?>
                <option data-pid="<?php echo esc_attr( $country_value[ 1 ] ); ?>"
                        data-country="<?php echo esc_attr( $country_key ); ?>"
                        class="country"
                        value="<?php echo esc_attr( $country_key ); ?>" <?php selected( in_array( $country_key, $selected_countries ) ); ?>><?php echo esc_attr( $country_value[ 0 ] ); ?></option>
				<?php if ( $country_key == 'US' ) : ?>
                    <optgroup label="" id="cs_target_states_group">
						<?php foreach ( $states as $state_key => $state_value ) : ?>
                            <option data-value="<?php echo esc_attr( $state_key ); ?>"
                                    class="state"
                                    value="<?php echo esc_attr( $state_key ); ?>"<?php selected( in_array( $state_key, $selected_states ) ); ?>>
								<?php echo esc_attr( $state_value[ 'name' ] ); ?>
                            </option>
						<?php endforeach; ?>
                    </optgroup>
				<?php endif; ?>
			<?php endforeach; ?>
        </select>

        <input type="hidden"
               name="<?php echo esc_attr( "cs[" . ConsentMagic()->plugin_name . "][_cs_us_states_target]" ) ?>"
               id="cs_states_target"
               value="<?php echo esc_attr( implode( ",", $selected_states ) ); ?>">
        <input type="hidden" name="<?php echo esc_attr( "cs[" . ConsentMagic()->plugin_name . "][_cs_target]" ) ?>"
               id="cs_countries_target"
               value="<?php echo esc_attr( implode( ",", $selected_countries ) ); ?>">
    </div>

	<?php
}

/**
 * Output multi select input
 * @param        $key
 * @param        $values
 * @param bool   $disabled
 * @param string $class
 * @param null   $id
 * @param string $placeholder
 * @param bool   $admin_check
 */
function render_multi_select_input( $key, $values, $disabled = false, $class = '', $id = null, $placeholder = '', $admin_check = false ) {
	$attr_name = "cs[" . ConsentMagic()->plugin_name . "][$key][]";
	$attr_id   = 'cs_' . $key;

	if ( $id ) {
		$selected = explode( ",", get_post_meta( $id, $key, true ) );
	} else {
		$selected = ConsentMagic()->getOption( $key );
	}

	if ( !$selected ) {
		$selected = array();
	}
	?>

    <div class="select-wrap select-full-wrap">
        <input type="hidden" name="<?php esc_attr_e( $attr_name ); ?>" value="">
        <select class="form-control-cm select-full <?php echo esc_attr( $class ); ?>"
                name="<?php esc_attr_e( $attr_name ); ?>"
                id="<?php esc_attr_e( $attr_id ); ?>"
			<?php disabled( $disabled ); ?>
                style="width: 100%;"
                placeholder="<?php esc_attr_e( $placeholder ); ?>"
                multiple="multiple">
			<?php if ( $values ) { ?>
				<?php foreach ( $values as $option_key => $option_value ) : ?>
					<?php if ( $admin_check ) { ?>
                        <option value="<?php

						if ( $class == 'cs_multi_select_tags' ) {
							echo esc_attr( $option_value );
						} else {
							echo esc_attr( $option_key );
						}
						?>"
							<?php

							if ( $class == 'cs_multi_select_tags' ) {
								if ( $option_value != 'null' ) {
									selected( in_array( $option_value, $selected ) );
								}
							} else {
								if ( $option_key != 'null' ) {
									selected( in_array( $option_key, $selected ) );
								}
							}

							?>
							<?php disabled( $option_key, 'disabled' ); ?>
                        >
							<?php echo esc_attr( $option_value ); ?>
                        </option>
					<?php } else if ( $option_value !== 'Administrator' ) { ?>
                        <option value="<?php

						if ( $class == 'cs_multi_select_tags' ) {
							echo esc_attr( $option_value );
						} else {
							echo esc_attr( $option_key );
						}

						?>"
							<?php

							if ( $class == 'cs_multi_select_tags' ) {
								if ( $option_value != 'null' ) {
									selected( in_array( $option_value, $selected ) );
								}
							} else {
								if ( $option_key != 'null' ) {
									selected( in_array( $option_key, $selected ) );
								}
							}

							?>
							<?php disabled( $option_key, 'disabled' ); ?>
                        >
							<?php echo esc_attr( $option_value ); ?>
                        </option>
					<?php } ?>
				<?php endforeach; ?>
			<?php } ?>
			<?php if ( $admin_check ) { ?>
                <option value="visitor"><?php echo esc_html__( 'Visitor', 'consent-magic' ); ?></option>
			<?php } ?>
        </select>
    </div>

	<?php
}

function renderCSTypeOptions() {

	$options = array(
		'just_inform'         => esc_html__( 'Just inform', 'consent-magic' ),
		'ask_before_tracking' => esc_html__( 'Ask before tracking', 'consent-magic' ),
		'inform_and_opiout'   => esc_html__( 'Inform and Opt-out', 'consent-magic' ),
		'iab'                 => esc_html__( 'IAB', 'consent-magic' ),
	);

	return $options;
}

function renderCSTypeLocations() {

	$options = array(
		'AD' => array(
			__( 'Andorra', 'consent-magic' ),
			'NONGDPR'
		),
		'AF' => array(
			__( 'Afghanistan', 'consent-magic' ),
			'NONGDPR'
		),
		'AG' => array(
			__( 'Antigua and Barbuda', 'consent-magic' ),
			'NONGDPR'
		),
		'AI' => array(
			__( 'Anguilla', 'consent-magic' ),
			'NONGDPR'
		),
		'AL' => array(
			__( 'Albania', 'consent-magic' ),
			'NONGDPR'
		),
		'DZ' => array(
			__( 'Algeria', 'consent-magic' ),
			'NONGDPR'
		),
		'AM' => array(
			__( 'Armenia', 'consent-magic' ),
			'NONGDPR'
		),
		'AO' => array(
			__( 'Angola', 'consent-magic' ),
			'NONGDPR'
		),
		'AQ' => array(
			__( 'Antarctica', 'consent-magic' ),
			'NONGDPR'
		),
		'AR' => array(
			__( 'Argentina', 'consent-magic' ),
			'NONGDPR'
		),
		'AS' => array(
			__( 'American Samoa', 'consent-magic' ),
			'NONGDPR'
		),
		'AT' => array(
			__( 'Austria', 'consent-magic' ),
			'GDPR'
		),
		'AU' => array(
			__( 'Australia', 'consent-magic' ),
			'NONGDPR'
		),
		'AW' => array(
			__( 'Aruba', 'consent-magic' ),
			'NONGDPR'
		),
		'AX' => array(
			__( 'Åland Islands', 'consent-magic' ),
			'NONGDPR'
		),
		'AZ' => array(
			__( 'Azerbaijan', 'consent-magic' ),
			'NONGDPR'
		),
		'BA' => array(
			__( 'Bosnia and Herzegovina', 'consent-magic' ),
			'NONGDPR'
		),
		'BB' => array(
			__( 'Barbados', 'consent-magic' ),
			'NONGDPR'
		),
		'BD' => array(
			__( 'Bangladesh', 'consent-magic' ),
			'NONGDPR'
		),
		'BE' => array(
			__( 'Belgium', 'consent-magic' ),
			'GDPR'
		),
		'BF' => array(
			__( 'Burkina Faso', 'consent-magic' ),
			'NONGDPR'
		),
		'BG' => array(
			__( 'Bulgaria', 'consent-magic' ),
			'GDPR'
		),
		'BH' => array(
			__( 'Bahrain', 'consent-magic' ),
			'NONGDPR'
		),
		'BI' => array(
			__( 'Burundi', 'consent-magic' ),
			'NONGDPR'
		),
		'BJ' => array(
			__( 'Benin', 'consent-magic' ),
			'NONGDPR'
		),
		'BL' => array(
			__( 'Saint Barthélemy', 'consent-magic' ),
			'NONGDPR'
		),
		'BM' => array(
			__( 'Bermuda', 'consent-magic' ),
			'NONGDPR'
		),
		'BN' => array(
			__( 'Brunei Darussalam', 'consent-magic' ),
			'NONGDPR'
		),
		'BO' => array(
			__( 'Bolivia (Plurinational State of)', 'consent-magic' ),
			'NONGDPR'
		),
		'BQ' => array(
			__( 'Bonaire, Sint Eustatius and Saba', 'consent-magic' ),
			'NONGDPR'
		),
		'BR' => array(
			__( 'Brazil', 'consent-magic' ),
			'NONGDPR'
		),
		'BS' => array(
			__( 'Bahamas', 'consent-magic' ),
			'NONGDPR'
		),
		'BT' => array(
			__( 'Bhutan', 'consent-magic' ),
			'NONGDPR'
		),
		'BV' => array(
			__( 'Bouvet Island', 'consent-magic' ),
			'NONGDPR'
		),
		'BW' => array(
			__( 'Botswana', 'consent-magic' ),
			'NONGDPR'
		),
		'BY' => array(
			__( 'Belarus', 'consent-magic' ),
			'NONGDPR'
		),
		'BZ' => array(
			__( 'Belize', 'consent-magic' ),
			'NONGDPR'
		),
		'CA' => array(
			__( 'Canada', 'consent-magic' ),
			'NONGDPR'
		),
		'CC' => array(
			__( 'Cocos (Keeling) Islands', 'consent-magic' ),
			'NONGDPR'
		),
		'CD' => array(
			__( 'Congo, Democratic Republic of the', 'consent-magic' ),
			'NONGDPR'
		),
		'CF' => array(
			__( 'Central African Republic', 'consent-magic' ),
			'NONGDPR'
		),
		'CG' => array(
			__( 'Congo', 'consent-magic' ),
			'NONGDPR'
		),
		'CI' => array(
			__( "Côte d'Ivoire", 'consent-magic' ),
			'NONGDPR'
		),
		'CK' => array(
			__( 'Cook Islands', 'consent-magic' ),
			'NONGDPR'
		),
		'CL' => array(
			__( 'Chile', 'consent-magic' ),
			'NONGDPR'
		),
		'CM' => array(
			__( 'Cameroon', 'consent-magic' ),
			'NONGDPR'
		),
		'CN' => array(
			__( 'China', 'consent-magic' ),
			'NONGDPR'
		),
		'CO' => array(
			__( 'Colombia', 'consent-magic' ),
			'NONGDPR'
		),
		'CR' => array(
			__( 'Costa Rica', 'consent-magic' ),
			'NONGDPR'
		),
		'CU' => array(
			__( 'Cuba', 'consent-magic' ),
			'NONGDPR'
		),
		'CV' => array(
			__( 'Cabo Verde', 'consent-magic' ),
			'NONGDPR'
		),
		'CW' => array(
			__( 'Curaçao', 'consent-magic' ),
			'NONGDPR'
		),
		'CX' => array(
			__( 'Christmas Island', 'consent-magic' ),
			'NONGDPR'
		),
		'CY' => array(
			__( 'Cyprus', 'consent-magic' ),
			'GDPR'
		),
		'HR' => array(
			__( 'Croatia', 'consent-magic' ),
			'GDPR'
		),
		'KH' => array(
			__( 'Cambodia', 'consent-magic' ),
			'NONGDPR'
		),
		'KM' => array(
			__( 'Comoros', 'consent-magic' ),
			'NONGDPR'
		),
		'TD' => array(
			__( 'Chad', 'consent-magic' ),
			'NONGDPR'
		),
		'CZ' => array(
			__( 'Czechia', 'consent-magic' ),
			'GDPR'
		),
		'KY' => array(
			__( 'Cayman Islands', 'consent-magic' ),
			'NONGDPR'
		),
		'DJ' => array(
			__( 'Djibouti', 'consent-magic' ),
			'NONGDPR'
		),
		'DK' => array(
			__( 'Denmark', 'consent-magic' ),
			'GDPR'
		),
		'DM' => array(
			__( 'Dominica', 'consent-magic' ),
			'NONGDPR'
		),
		'DO' => array(
			__( 'Dominican Republic', 'consent-magic' ),
			'NONGDPR'
		),
		'EZ' => array(
			__( 'Ecuador', 'consent-magic' ),
			'NONGDPR'
		),
		'EE' => array(
			__( 'Estonia', 'consent-magic' ),
			'GDPR'
		),
		'SV' => array(
			__( 'El Salvador', 'consent-magic' ),
			'NONGDPR'
		),
		'SZ' => array(
			__( 'Eswatini', 'consent-magic' ),
			'NONGDPR'
		),
		'EG' => array(
			__( 'Egypt', 'consent-magic' ),
			'NONGDPR'
		),
		'ER' => array(
			__( 'Eritrea', 'consent-magic' ),
			'NONGDPR'
		),
		'GQ' => array(
			__( 'Equatorial Guinea', 'consent-magic' ),
			'NONGDPR'
		),
		'ET' => array(
			__( 'Ethiopia', 'consent-magic' ),
			'NONGDPR'
		),
		'FI' => array(
			__( 'Finland', 'consent-magic' ),
			'GDPR'
		),
		'FG' => array(
			__( 'Fiji', 'consent-magic' ),
			'NONGDPR'
		),
		'FK' => array(
			__( 'Falkland Islands (Malvinas)', 'consent-magic' ),
			'NONGDPR'
		),
		'FO' => array(
			__( 'Faroe Islands', 'consent-magic' ),
			'NONGDPR'
		),
		'FR' => array(
			__( 'France', 'consent-magic' ),
			'GDPR'
		),
		'GF' => array(
			__( 'French Guiana', 'consent-magic' ),
			'NONGDPR'
		),
		'PF' => array(
			__( 'French Polynesia', 'consent-magic' ),
			'NONGDPR'
		),
		'TF' => array(
			__( 'French Southern Territories', 'consent-magic' ),
			'NONGDPR'
		),
		'DE' => array(
			__( 'Germany', 'consent-magic' ),
			'GDPR'
		),
		'GA' => array(
			__( 'Gabon', 'consent-magic' ),
			'NONGDPR'
		),
		'GD' => array(
			__( 'Grenada', 'consent-magic' ),
			'NONGDPR'
		),
		'GE' => array(
			__( 'Georgia', 'consent-magic' ),
			'NONGDPR'
		),
		'GG' => array(
			__( 'Guernsey', 'consent-magic' ),
			'NONGDPR'
		),
		'GH' => array(
			__( 'Ghana', 'consent-magic' ),
			'NONGDPR'
		),
		'GI' => array(
			__( 'Gibraltar', 'consent-magic' ),
			'NONGDPR'
		),
		'GL' => array(
			__( 'Greenland', 'consent-magic' ),
			'NONGDPR'
		),
		'GM' => array(
			__( 'Gambia', 'consent-magic' ),
			'NONGDPR'
		),
		'GN' => array(
			__( 'Guinea', 'consent-magic' ),
			'NONGDPR'
		),
		'GP' => array(
			__( 'Guadeloupe', 'consent-magic' ),
			'NONGDPR'
		),
		'GR' => array(
			__( 'Greece', 'consent-magic' ),
			'GDPR'
		),
		'GT' => array(
			__( 'Guatemala', 'consent-magic' ),
			'NONGDPR'
		),
		'GU' => array(
			__( 'Guam', 'consent-magic' ),
			'NONGDPR'
		),
		'GW' => array(
			__( 'Guinea-Bissau', 'consent-magic' ),
			'NONGDPR'
		),
		'GY' => array(
			__( 'Guyana', 'consent-magic' ),
			'NONGDPR'
		),
		'HK' => array(
			__( 'Hong Kong', 'consent-magic' ),
			'NONGDPR'
		),
		'VA' => array(
			__( 'Holy See', 'consent-magic' ),
			'NONGDPR'
		),
		'HM' => array(
			__( 'Heard Island and McDonald Islands', 'consent-magic' ),
			'NONGDPR'
		),
		'HN' => array(
			__( 'Honduras', 'consent-magic' ),
			'NONGDPR'
		),
		'HT' => array(
			__( 'Haiti', 'consent-magic' ),
			'NONGDPR'
		),
		'HU' => array(
			__( 'Hungary', 'consent-magic' ),
			'GDPR'
		),
		'ID' => array(
			__( 'Indonesia', 'consent-magic' ),
			'NONGDPR'
		),
		'IE' => array(
			__( 'Ireland', 'consent-magic' ),
			'GDPR'
		),
		'IL' => array(
			__( 'Israel', 'consent-magic' ),
			'NONGDPR'
		),
		'IM' => array(
			__( 'Isle of Man', 'consent-magic' ),
			'NONGDPR'
		),
		'IN' => array(
			__( 'India', 'consent-magic' ),
			'NONGDPR'
		),
		'IO' => array(
			__( 'British Indian Ocean Territory', 'consent-magic' ),
			'NONGDPR'
		),
		'IQ' => array(
			__( 'Iraq', 'consent-magic' ),
			'NONGDPR'
		),
		'IR' => array(
			__( 'Iran (Islamic Republic of)', 'consent-magic' ),
			'NONGDPR'
		),
		'IS' => array(
			__( 'Iceland', 'consent-magic' ),
			'GDPR'
		),
		'IT' => array(
			__( 'Italy', 'consent-magic' ),
			'GDPR'
		),
		'JE' => array(
			__( 'Jersey', 'consent-magic' ),
			'NONGDPR'
		),
		'JM' => array(
			__( 'Jamaica', 'consent-magic' ),
			'NONGDPR'
		),
		'JO' => array(
			__( 'Jordan', 'consent-magic' ),
			'NONGDPR'
		),
		'JP' => array(
			__( 'Japan', 'consent-magic' ),
			'NONGDPR'
		),
		'KE' => array(
			__( 'Kenya', 'consent-magic' ),
			'NONGDPR'
		),
		'KG' => array(
			__( 'Kyrgyzstan', 'consent-magic' ),
			'NONGDPR'
		),
		'KI' => array(
			__( 'Kiribati', 'consent-magic' ),
			'NONGDPR'
		),
		'KN' => array(
			__( 'Saint Kitts and Nevis', 'consent-magic' ),
			'NONGDPR'
		),
		'KP' => array(
			__( "Korea (Democratic People's Republic of)", 'consent-magic' ),
			'NONGDPR'
		),
		'KR' => array(
			__( 'Korea, Republic of', 'consent-magic' ),
			'NONGDPR'
		),
		'KW' => array(
			__( 'Kuwait', 'consent-magic' ),
			'NONGDPR'
		),
		'KZ' => array(
			__( 'Kazakhstan', 'consent-magic' ),
			'NONGDPR'
		),
		'LA' => array(
			__( "Lao People's Democratic Republic", 'consent-magic' ),
			'NONGDPR'
		),
		'LB' => array(
			__( 'Lebanon', 'consent-magic' ),
			'NONGDPR'
		),
		'LI' => array(
			__( 'Liechtenstein', 'consent-magic' ),
			'GDPR'
		),
		'LR' => array(
			__( 'Liberia', 'consent-magic' ),
			'NONGDPR'
		),
		'LS' => array(
			__( 'Lesotho', 'consent-magic' ),
			'NONGDPR'
		),
		'LT' => array(
			__( 'Lithuania', 'consent-magic' ),
			'GDPR'
		),
		'LU' => array(
			__( 'Luxembourg', 'consent-magic' ),
			'GDPR'
		),
		'LV' => array(
			__( 'Latvia', 'consent-magic' ),
			'GDPR'
		),
		'LY' => array(
			__( 'Libya', 'consent-magic' ),
			'NONGDPR'
		),
		'MA' => array(
			__( 'Morocco', 'consent-magic' ),
			'NONGDPR'
		),
		'MC' => array(
			__( 'Monaco', 'consent-magic' ),
			'NONGDPR'
		),
		'MD' => array(
			__( 'Moldova, Republic of', 'consent-magic' ),
			'NONGDPR'
		),
		'ME' => array(
			__( 'Montenegro', 'consent-magic' ),
			'NONGDPR'
		),
		'MG' => array(
			__( 'Madagascar', 'consent-magic' ),
			'NONGDPR'
		),
		'MH' => array(
			__( 'Marshall Islands', 'consent-magic' ),
			'NONGDPR'
		),
		'ML' => array(
			__( 'Mali', 'consent-magic' ),
			'NONGDPR'
		),
		'MM' => array(
			__( 'Myanmar', 'consent-magic' ),
			'NONGDPR'
		),
		'MN' => array(
			__( 'Mongolia', 'consent-magic' ),
			'NONGDPR'
		),
		'MO' => array(
			__( 'Macao', 'consent-magic' ),
			'NONGDPR'
		),
		'MQ' => array(
			__( 'Martinique', 'consent-magic' ),
			'NONGDPR'
		),
		'MR' => array(
			__( 'Mauritania', 'consent-magic' ),
			'NONGDPR'
		),
		'MS' => array(
			__( 'Montserrat', 'consent-magic' ),
			'NONGDPR'
		),
		'MT' => array(
			__( 'Malta', 'consent-magic' ),
			'GDPR'
		),
		'MU' => array(
			__( 'Mauritius', 'consent-magic' ),
			'NONGDPR'
		),
		'MV' => array(
			__( 'Maldives', 'consent-magic' ),
			'NONGDPR'
		),
		'MW' => array(
			__( 'Malawi', 'consent-magic' ),
			'NONGDPR'
		),
		'MX' => array(
			__( 'Mexico', 'consent-magic' ),
			'NONGDPR'
		),
		'MY' => array(
			__( 'Malaysia', 'consent-magic' ),
			'NONGDPR'
		),
		'FM' => array(
			__( 'Micronesia (Federated States of)', 'consent-magic' ),
			'NONGDPR'
		),
		'YT' => array(
			__( 'Mayotte', 'consent-magic' ),
			'NONGDPR'
		),
		'MZ' => array(
			__( 'Mozambique', 'consent-magic' ),
			'NONGDPR'
		),
		'NM' => array(
			__( 'Namibia', 'consent-magic' ),
			'NONGDPR'
		),
		'NC' => array(
			__( 'New Caledonia', 'consent-magic' ),
			'NONGDPR'
		),
		'NE' => array(
			__( 'Niger', 'consent-magic' ),
			'NONGDPR'
		),
		'NF' => array(
			__( 'Norfolk Island', 'consent-magic' ),
			'NONGDPR'
		),
		'NG' => array(
			__( 'Nigeria', 'consent-magic' ),
			'NONGDPR'
		),
		'NI' => array(
			__( 'Nicaragua', 'consent-magic' ),
			'NONGDPR'
		),
		'NL' => array(
			__( 'Netherlands', 'consent-magic' ),
			'GDPR'
		),
		'NO' => array(
			__( 'Norway', 'consent-magic' ),
			'GDPR'
		),
		'NP' => array(
			__( 'Nepal', 'consent-magic' ),
			'NONGDPR'
		),
		'NR' => array(
			__( 'Nauru', 'consent-magic' ),
			'NONGDPR'
		),
		'NU' => array(
			__( 'Niue', 'consent-magic' ),
			'NONGDPR'
		),
		'NZ' => array(
			__( 'New Zealand', 'consent-magic' ),
			'NONGDPR'
		),
		'MK' => array(
			__( 'North Macedonia', 'consent-magic' ),
			'NONGDPR'
		),
		'MP' => array(
			__( 'Northern Mariana Islands', 'consent-magic' ),
			'NONGDPR'
		),
		'OM' => array(
			__( 'Oman', 'consent-magic' ),
			'NONGDPR'
		),
		'PA' => array(
			__( 'Panama', 'consent-magic' ),
			'NONGDPR'
		),
		'PE' => array(
			__( 'Peru', 'consent-magic' ),
			'NONGDPR'
		),
		'PG' => array(
			__( 'Papua New Guinea', 'consent-magic' ),
			'NONGDPR'
		),
		'PH' => array(
			__( 'Philippines', 'consent-magic' ),
			'NONGDPR'
		),
		'PK' => array(
			__( 'Pakistan', 'consent-magic' ),
			'NONGDPR'
		),
		'PL' => array(
			__( 'Poland', 'consent-magic' ),
			'GDPR'
		),
		'PN' => array(
			__( 'Pitcairn', 'consent-magic' ),
			'NONGDPR'
		),
		'PR' => array(
			__( 'Puerto Rico', 'consent-magic' ),
			'NONGDPR'
		),
		'PS' => array(
			__( 'Palestine, State of', 'consent-magic' ),
			'NONGDPR'
		),
		'PT' => array(
			__( 'Portugal', 'consent-magic' ),
			'GDPR'
		),
		'PW' => array(
			__( 'Palau', 'consent-magic' ),
			'NONGDPR'
		),
		'PY' => array(
			__( 'Paraguay', 'consent-magic' ),
			'NONGDPR'
		),
		'QA' => array(
			__( 'Qatar', 'consent-magic' ),
			'NONGDPR'
		),
		'RE' => array(
			__( "Réunion", 'consent-magic' ),
			'NONGDPR'
		),
		'RO' => array(
			__( 'Romania', 'consent-magic' ),
			'GDPR'
		),
		'RU' => array(
			__( 'Russian Federation', 'consent-magic' ),
			'NONGDPR'
		),
		'RW' => array(
			__( 'Rwanda', 'consent-magic' ),
			'NONGDPR'
		),
		'PM' => array(
			__( 'Saint Pierre and Miquelon', 'consent-magic' ),
			'NONGDPR'
		),
		'RS' => array(
			__( 'Serbia', 'consent-magic' ),
			'NONGDPR'
		),
		'SA' => array(
			__( 'Saudi Arabia', 'consent-magic' ),
			'NONGDPR'
		),
		'SB' => array(
			__( 'Solomon Islands', 'consent-magic' ),
			'NONGDPR'
		),
		'SC' => array(
			__( 'Seychelles', 'consent-magic' ),
			'NONGDPR'
		),
		'SD' => array(
			__( 'Sudan', 'consent-magic' ),
			'NONGDPR'
		),
		'SE' => array(
			__( 'Sweden', 'consent-magic' ),
			'GDPR'
		),
		'SG' => array(
			__( 'Singapore', 'consent-magic' ),
			'NONGDPR'
		),
		'SH' => array(
			__( 'Saint Helena, Ascension and Tristan da Cunha', 'consent-magic' ),
			'NONGDPR'
		),
		'SI' => array(
			__( 'Slovenia', 'consent-magic' ),
			'GDPR'
		),
		'SJ' => array(
			__( 'Svalbard and Jan Mayen', 'consent-magic' ),
			'NONGDPR'
		),
		'SK' => array(
			__( 'Slovakia', 'consent-magic' ),
			'GDPR'
		),
		'SL' => array(
			__( 'Sierra Leone', 'consent-magic' ),
			'NONGDPR'
		),
		'SM' => array(
			__( 'San Marino', 'consent-magic' ),
			'NONGDPR'
		),
		'SN' => array(
			__( 'Senegal', 'consent-magic' ),
			'NONGDPR'
		),
		'SO' => array(
			__( 'Somalia', 'consent-magic' ),
			'NONGDPR'
		),
		'SR' => array(
			__( 'Suriname', 'consent-magic' ),
			'NONGDPR'
		),
		'SS' => array(
			__( 'South Sudan', 'consent-magic' ),
			'NONGDPR'
		),
		'ST' => array(
			__( 'Sao Tome and Principe', 'consent-magic' ),
			'NONGDPR'
		),
		'GS' => array(
			__( 'South Georgia and the South Sandwich Islands', 'consent-magic' ),
			'NONGDPR'
		),
		'MF' => array(
			__( 'Saint Martin (French part)', 'consent-magic' ),
			'NONGDPR'
		),
		'ES' => array(
			__( 'Spain', 'consent-magic' ),
			'GDPR'
		),
		'CH' => array(
			__( 'Switzerland', 'consent-magic' ),
			'GDPR'
		),
		'LC' => array(
			__( 'Saint Lucia', 'consent-magic' ),
			'NONGDPR'
		),
		'LK' => array(
			__( 'Sri Lanka', 'consent-magic' ),
			'NONGDPR'
		),
		'SX' => array(
			__( 'Sint Maarten (Dutch part)', 'consent-magic' ),
			'NONGDPR'
		),
		'SY' => array(
			__( 'Syrian Arab Republic', 'consent-magic' ),
			'NONGDPR'
		),
		'VC' => array(
			__( 'Saint Vincent and the Grenadines', 'consent-magic' ),
			'NONGDPR'
		),
		'WS' => array(
			__( 'Samoa', 'consent-magic' ),
			'NONGDPR'
		),
		'ZA' => array(
			__( 'South Africa', 'consent-magic' ),
			'NONGDPR'
		),
		'TC' => array(
			__( 'Turks and Caicos Islands', 'consent-magic' ),
			'NONGDPR'
		),
		'TG' => array(
			__( 'Togo', 'consent-magic' ),
			'NONGDPR'
		),
		'TH' => array(
			__( 'Thailand', 'consent-magic' ),
			'NONGDPR'
		),
		'TJ' => array(
			__( 'Tajikistan', 'consent-magic' ),
			'NONGDPR'
		),
		'TK' => array(
			__( 'Tokelau', 'consent-magic' ),
			'NONGDPR'
		),
		'TL' => array(
			__( 'Timor-Leste', 'consent-magic' ),
			'NONGDPR'
		),
		'TM' => array(
			__( 'Turkmenistan', 'consent-magic' ),
			'NONGDPR'
		),
		'TN' => array(
			__( 'Tunisia', 'consent-magic' ),
			'NONGDPR'
		),
		'TO' => array(
			__( 'Tonga', 'consent-magic' ),
			'NONGDPR'
		),
		'TR' => array(
			__( 'Turkey', 'consent-magic' ),
			'NONGDPR'
		),
		'TT' => array(
			__( 'Trinidad and Tobago', 'consent-magic' ),
			'NONGDPR'
		),
		'TV' => array(
			__( 'Tuvalu', 'consent-magic' ),
			'NONGDPR'
		),
		'TW' => array(
			__( 'Taiwan, Province of China', 'consent-magic' ),
			'NONGDPR'
		),
		'TZ' => array(
			__( 'Tanzania, United Republic of', 'consent-magic' ),
			'NONGDPR'
		),
		'UA' => array(
			__( 'Ukraine', 'consent-magic' ),
			'NONGDPR'
		),
		'UG' => array(
			__( 'Uganda', 'consent-magic' ),
			'NONGDPR'
		),
		'AE' => array(
			__( 'United Arab Emirates', 'consent-magic' ),
			'NONGDPR'
		),
		'UM' => array(
			__( 'United States Minor Outlying Islands', 'consent-magic' ),
			'NONGDPR'
		),
		'US' => array(
			__( 'United States of America', 'consent-magic' ),
			'NONGDPR'
		),
		'UY' => array(
			__( 'Uruguay', 'consent-magic' ),
			'NONGDPR'
		),
		'UZ' => array(
			__( 'Uzbekistan', 'consent-magic' ),
			'NONGDPR'
		),
		'GB' => array(
			__( 'United Kingdom of Great Britain and Northern Ireland', 'consent-magic' ),
			'GDPR'
		),
		'VE' => array(
			__( 'Venezuela (Bolivarian Republic of)', 'consent-magic' ),
			'NONGDPR'
		),
		'VG' => array(
			__( 'Virgin Islands (British)', 'consent-magic' ),
			'NONGDPR'
		),
		'VI' => array(
			__( 'Virgin Islands (U.S.)', 'consent-magic' ),
			'NONGDPR'
		),
		'VN' => array(
			__( 'Viet Nam', 'consent-magic' ),
			'NONGDPR'
		),
		'VU' => array(
			__( 'Vanuatu', 'consent-magic' ),
			'NONGDPR'
		),
		'EH' => array(
			__( 'Western Sahara', 'consent-magic' ),
			'NONGDPR'
		),
		'WF' => array(
			__( 'Wallis and Futuna', 'consent-magic' ),
			'NONGDPR'
		),
		'YE' => array(
			__( 'Yemen', 'consent-magic' ),
			'NONGDPR'
		),
		'ZM' => array(
			__( 'Zambia', 'consent-magic' ),
			'NONGDPR'
		),
		'ZW' => array(
			__( 'Zimbabwe', 'consent-magic' ),
			'NONGDPR'
		),
	);

	return $options;
}

function renderCSTypeUSStates() {

	$options = array(
		'US_AL' => array(
			'name' => __( 'Alabama', 'consent-magic' ),
			'code' => 0
		),
		'US_АК' => array(
			'name' => __( 'Alaska', 'consent-magic' ),
			'code' => 0
		),
		'US_AZ' => array(
			'name' => __( 'Arizona', 'consent-magic' ),
			'code' => 0
		),
		'US_AR' => array(
			'name' => __( 'Arkansas', 'consent-magic' ),
			'code' => 0
		),
		'US_CA' => array(
			'name' => __( 'California', 'consent-magic' ),
			'code' => 1000
		),
		'US_CO' => array(
			'name' => __( 'Colorado', 'consent-magic' ),
			'code' => 1001
		),
		'US_CT' => array(
			'name' => __( 'Connecticut', 'consent-magic' ),
			'code' => 1002
		),
		'US_DE' => array(
			'name' => __( 'Delaware', 'consent-magic' ),
			'code' => 1007
		),
		'US_DC' => array(
			'name' => __( 'District of Columbia', 'consent-magic' ),
			'code' => 0
		),
		'US_FL' => array(
			'name' => __( 'Florida', 'consent-magic' ),
			'code' => 1003
		),
		'US_GA' => array(
			'name' => __( 'Georgia', 'consent-magic' ),
			'code' => 0
		),
		'US_HI' => array(
			'name' => __( 'Hawaii', 'consent-magic' ),
			'code' => 0
		),
		'US_ID' => array(
			'name' => __( 'Idaho', 'consent-magic' ),
			'code' => 0
		),
		'US_IL' => array(
			'name' => __( 'Illinois', 'consent-magic' ),
			'code' => 0
		),
		'US_IN' => array(
			'name' => __( 'Indiana', 'consent-magic' ),
			'code' => 0
		),
		'US_IA' => array(
			'name' => __( 'Iowa', 'consent-magic' ),
			'code' => 0
		),
		'US_KS' => array(
			'name' => __( 'Kansas', 'consent-magic' ),
			'code' => 0
		),
		'US_KY' => array(
			'name' => __( 'Kentucky', 'consent-magic' ),
			'code' => 0
		),
		'US_LA' => array(
			'name' => __( 'Louisiana', 'consent-magic' ),
			'code' => 0
		),
		'US_ME' => array(
			'name' => __( 'Maine', 'consent-magic' ),
			'code' => 0
		),
		'US_MD' => array(
			'name' => __( 'Maryland', 'consent-magic' ),
			'code' => 0
		),
		'US_МА' => array(
			'name' => __( 'Massachusetts', 'consent-magic' ),
			'code' => 0
		),
		'US_MI' => array(
			'name' => __( 'Michigan', 'consent-magic' ),
			'code' => 0
		),
		'US_MN' => array(
			'name' => __( 'Minnesota', 'consent-magic' ),
			'code' => 0
		),
		'US_MS' => array(
			'name' => __( 'Mississippi', 'consent-magic' ),
			'code' => 0
		),
		'US_МО' => array(
			'name' => __( 'Missouri', 'consent-magic' ),
			'code' => 0
		),
		'US_MT' => array(
			'name' => __( 'Montana', 'consent-magic' ),
			'code' => 1006
		),
		'US_NE' => array(
			'name' => __( 'Nebraska', 'consent-magic' ),
			'code' => 1008
		),
		'US_NV' => array(
			'name' => __( 'Nevada', 'consent-magic' ),
			'code' => 0
		),
		'US_NH' => array(
			'name' => __( 'New Hampshire', 'consent-magic' ),
			'code' => 1009
		),
		'US_NJ' => array(
			'name' => __( 'New Jersey', 'consent-magic' ),
			'code' => 1010
		),
		'US_NM' => array(
			'name' => __( 'New Mexico', 'consent-magic' ),
			'code' => 0
		),
		'US_NY' => array(
			'name' => __( 'New York', 'consent-magic' ),
			'code' => 0
		),
		'US_NC' => array(
			'name' => __( 'North Carolina', 'consent-magic' ),
			'code' => 0
		),
		'US_ND' => array(
			'name' => __( 'North Dakota', 'consent-magic' ),
			'code' => 0
		),
		'US_ОН' => array(
			'name' => __( 'Ohio', 'consent-magic' ),
			'code' => 0
		),
		'US_ОК' => array(
			'name' => __( 'Oklahoma', 'consent-magic' ),
			'code' => 0
		),
		'US_OR' => array(
			'name' => __( 'Oregon', 'consent-magic' ),
			'code' => 1004
		),
		'US_РА' => array(
			'name' => __( 'Pennsylvania', 'consent-magic' ),
			'code' => 0
		),
		'US_RI' => array(
			'name' => __( 'Rhode Island', 'consent-magic' ),
			'code' => 0
		),
		'US_SC' => array(
			'name' => __( 'South Carolina', 'consent-magic' ),
			'code' => 0
		),
		'US_SD' => array(
			'name' => __( 'South Dakota', 'consent-magic' ),
			'code' => 0
		),
		'US_TN' => array(
			'name' => __( 'Tennessee', 'consent-magic' ),
			'code' => 0
		),
		'US_ТХ' => array(
			'name' => __( 'Texas', 'consent-magic' ),
			'code' => 1005
		),
		'US_UT' => array(
			'name' => __( 'Utah', 'consent-magic' ),
			'code' => 0
		),
		'US_VT' => array(
			'name' => __( 'Vermont', 'consent-magic' ),
			'code' => 0
		),
		'US_VA' => array(
			'name' => __( 'Virginia', 'consent-magic' ),
			'code' => 0
		),
		'US_WA' => array(
			'name' => __( 'Washington', 'consent-magic' ),
			'code' => 0
		),
		'US_WV' => array(
			'name' => __( 'West Virginia', 'consent-magic' ),
			'code' => 0
		),
		'US_WI' => array(
			'name' => __( 'Wisconsin', 'consent-magic' ),
			'code' => 0
		),
		'US_WY' => array(
			'name' => __( 'Wyoming', 'consent-magic' ),
			'code' => 0
		),
	);

	return $options;
}


function renderCSBarTypeOptions() {

	$options = array(
		'bar_small'   => 'Small bar',
		'bar_large'   => 'Large bar',
		'popup_small' => 'Small popup',
		'popup_large' => 'Large popup'
	);

	return $options;
}

function renderCSBarPositionOptions() {

	$options = array(
		'bottom' => 'Bottom',
		'top'    => 'Top'
	);

	return $options;
}

function renderCSDesignTypeOptions() {

	$options = array(
		'multi'  => 'Multi-step design',
		'single' => 'Single-step design'
	);

	return $options;
}

function renderUploadBtn( $id, $key ) {
	$attr_name = "cs[" . ConsentMagic()->plugin_name . "][$key]";
	$attr_id   = 'cs_' . $key;
	$img_id    = get_post_meta( $id, 'cs_logo', true );
	$image     = wp_get_attachment_image_src( $img_id, array(
		443,
		216
	) );
	if ( $img_id ) { ?>

        <div class="logo-wrap">
            <div>
                <a href="#"
                   class="cs_img_upload btn btn-primary btn-primary-type2 d-inline-block"><?php esc_html_e( 'Select logo', 'consent-magic' ); ?></a>
            </div>

            <div>
                <div class="cs_logo_img_wrap">
                    <p class="img_recommender text-gray mt-24 mb-24"><?php esc_html_e( '500 kb max | JPEG, PNG', 'consent-magic' ); ?></p>
                    <img src="<?php echo esc_url( $image[ 0 ] ); ?>" alt="img"/>
                </div>

                <input type="hidden"
                       id="<?php echo esc_attr( $attr_id ); ?>"
                       name="<?php echo esc_attr( $attr_name ); ?>"
                       value="<?php echo esc_attr( $img_id ); ?>">
            </div>

            <a href="#" class="btn btn-red btn-primary-type2 d-block mt-24 cs_img_upload_rmv"
               style="display:block;"><?php esc_html_e( 'Remove image', 'consent-magic' ); ?></a>
        </div>

	<?php } else { ?>

        <div class="logo-wrap">
            <div>
                <a href="#"
                   class="cs_img_upload btn btn-primary btn-primary-type2 d-inline-block"><?php esc_html_e( 'Select logo', 'consent-magic' ); ?></a>
            </div>

            <div>
                <div class="cs_logo_img_wrap">
                    <p class="img_recommender text-gray mt-24 mb-24"
                       style="display:none;"><?php esc_html_e( '500 kb max | JPEG, PNG', 'consent-magic' ); ?></p>
                </div>

                <input type="hidden"
                       id="<?php echo esc_attr( $attr_id ); ?>"
                       name="<?php echo esc_attr( $attr_name ); ?>"
                       value="">
            </div>

            <a href="#" class="btn btn-red btn-primary-type2 d-block mt-24 cs_img_upload_rmv"
               style="display:none;"><?php esc_html_e( 'Remove image', 'consent-magic' ); ?></a>

        </div>

	<?php }
}

function renderSelectLanguages( $key = false, $key_field = false, $lang = CMPRO_DEFAULT_LANGUAGE, $lang_list = false, $id_rule = false, $data_render = false, $meta = false, $select_type = 'standard' ) {

	if ( $key ) {
		$attr_name = "cs[" . ConsentMagic()->plugin_name . "][$key]";
	} elseif ( ( $key_field && $id_rule ) || ( $id_rule === null && $meta ) ) {
		$attr_name = null;
	}

	if ( is_array( $lang_list ) ) {
		$langs = $lang_list;
	} else {
		$langs = get_language_list();
	}

	?>
    <div class="select-wrap select-<?php echo esc_attr( $select_type ); ?>-wrap">
        <select class="form-control-cm form-control-lang"
                id="id-<?php echo uniqid() ?>"
                name="<?php echo esc_attr( $attr_name ?? null ); ?>"
                data-placeholder="<?php echo esc_html__( 'Select language', 'consent-magic' ) ?>"
                autocomplete="off"
                style="width: 100%;"
                data-field="<?php echo esc_attr( $key_field ) ?>" data-rule="<?php echo esc_attr( $id_rule ) ?>"
                data-render="<?php echo esc_attr( $data_render ) ?>"
                data-option-type="<?php echo $meta ? 1 : 0 ?>"
                data-lang="<?php echo esc_attr( $lang ); ?>"
                data-plugin-name="<?php echo esc_attr( ConsentMagic()->get_plugin_name() ); ?>"">
		<?php foreach ( $langs as $lang_key => $lang_value ) : ?>
            <option data-pid="<?php echo esc_attr( $lang_key ); ?>"

                    value="<?php echo esc_attr( $lang_key ); ?>" <?php selected( $lang, $lang_key ); ?>><?php echo esc_attr( $lang_value[ 'label' ] ); ?></option>
		<?php endforeach; ?>
        </select>
    </div>
	<?php
}


function renderSelectLanguagesNew( $key = false, $key_field = false, $id_rule = false, $data_render = false, $meta = false, $select_type = 'standard' ) {

	$langs = get_language_list();

	?>
    <div class="select-wrap select-<?php echo esc_attr( $select_type ); ?>-wrap">
        <select class="form-control-cm form-control-lang-add"
                data-placeholder="<?php echo esc_html__( 'Select language', 'consent-magic' ) ?>"
                autocomplete="off"
                style="width: 100%;"
                data-field="<?php echo esc_attr( $key_field ) ?>"
                data-rule="<?php echo esc_attr( $id_rule ) ?>"
                data-render="<?php echo esc_attr( $data_render ) ?>"
                data-option-type="<?php echo $meta ? 1 : 0 ?>"
                data-plugin-name="<?php echo esc_attr( ConsentMagic()->get_plugin_name() ); ?>">
			<?php foreach ( $langs as $lang_key => $lang_value ) : ?>
                <option></option>
                <option data-pid="<?php echo esc_attr( $lang_key ); ?>"
                        value="<?php echo esc_attr( $lang_key ); ?>"><?php echo esc_attr( $lang_value[ 'label' ] ); ?></option>
			<?php endforeach; ?>
        </select>
    </div>
	<?php
}

function renderSelectLanguagesByTermNew( $attr_name = null, $id = null, $class = '', $data_render = '', $select_type = 'standard' ) {

	$langs = get_language_list();

	?>
    <div class="select-wrap select-<?php echo esc_attr( $select_type ); ?>-wrap">
        <select class="form-control-cm form-control-lang-new form-control-lang-add-term <?php echo esc_attr( $class ); ?>"
                data-placeholder="<?php echo esc_html__( 'Select language', 'consent-magic' ) ?>"
                name="<?php echo esc_attr( $attr_name ) . '_' . esc_attr( $id ); ?>"
                autocomplete="off"
                style="width: 100%;"
                data-rule="<?php echo esc_attr( $id ) ?>"
                data-render="<?php echo esc_attr( $data_render ) ?>"
                data-option-type="1"
                data-field="<?php echo esc_attr( $attr_name ) ?>"
                data-plugin-name="<?php echo esc_attr( ConsentMagic()->get_plugin_name() ); ?>">
			<?php foreach ( $langs as $lang_key => $lang_value ) : ?>
                <option></option>
                <option data-pid="<?php echo esc_attr( $lang_key ); ?>"
                        value="<?php echo esc_attr( $lang_key ); ?>"><?php echo esc_attr( $lang_value[ 'label' ] ); ?></option>
			<?php endforeach; ?>
        </select>
    </div>
	<?php
}

//update options in WP DB if options not isset
function update_cat_id_options( $option = 'all' ) {

	if ( $option != 'all' ) {
		$term = get_term_by( 'slug', str_replace( '_cat_id', '', $option ), 'cs-cookies-category' );

		if ( !empty( $term ) ) {
			ConsentMagic()->updateOptions( array(  $option => $term->term_id ) );

			return $term->term_id;
		}
	} else {
		$cat_id_options = array(
			'embedded_video_cat_id' => '',
			'unassigned_cat_id'     => '',
			'googlefonts_cat_id'    => '',
			'marketing_cat_id'      => '',
			'analytics_cat_id'      => '',
			'necessary_cat_id'      => '',
		);

		foreach ( $cat_id_options as $key => $cat_id_option ) {
			$term = get_term_by( 'slug', str_replace( '_cat_id', '', $cat_id_option ), 'cs-cookies-category' );
			if ( !empty( $term ) ) {
				ConsentMagic()->updateOptions( array( $option => $term->term_id ) );
				$cat_id_options[ $key ] = $term->term_id;
			}
		}

		return $cat_id_options;
	}
}

// get all default pre-defined categories
function get_default_categories() {
	return array(
		'necessary',
		'analytics',
		'marketing',
		'googlefonts',
		'unassigned',
		'embedded_video'
	);
}

function get_predefined_script_pattern() {
	return array(
		'Hotjar'                    => array(
			'option_key' => 'hotjar',
			'cat_key'    => 'cs_block_hotjar_scripts_cat',
			'script'     => 'hotjar'
		),
		'Instagram'                 => array(
			'option_key' => 'instagram',
			'cat_key'    => 'cs_block_instagram_scripts_cat',
			'script'     => 'instagram'
		),
		'Facebook Pixel'            => array(
			'option_key' => 'fb_pixel',
			'cat_key'    => 'cs_block_fb_pixel_scripts_cat',
			'script'     => 'fb_pixel'
		),
		'Google Analytics'          => array(
			'option_key' => 'google_analytics',
			'cat_key'    => 'cs_block_google_analytics_scripts_cat',
			'script'     => 'google_analytics'
		),
		'Google Publisher Tag'      => array(
			'option_key' => 'google_ads_tag',
			'cat_key'    => 'cs_block_google_tag_manager_scripts_cat',
			'script'     => 'google_ads_tag'
		),
		'Google Adsense'            => array(
			'option_key' => 'google_adsense',
			'cat_key'    => 'cs_block_google_adsense_scripts_cat',
			'script'     => 'google_adsense'
		),
		'Google maps'               => array(
			'option_key' => 'google_maps',
			'cat_key'    => 'cs_block_google_maps_scripts_cat',
			'script'     => 'google_maps'
		),
		'Google fonts'              => array(
			'option_key' => 'googlefonts',
			'cat_key'    => 'cs_block_googlefonts_scripts_cat',
			'script'     => 'googlefonts'
		),
		'Google captcha'            => array(
			'option_key' => 'google_captcha',
			'cat_key'    => 'cs_block_google_captcha_scripts_cat',
			'script'     => 'google_captcha'
		),
		'Twitter widget'            => array(
			'option_key' => 'tw_tag',
			'cat_key'    => 'cs_block_tw_tag_scripts_cat',
			'script'     => 'tw_tag'
		),
		'Twitter Pixel'             => array(
			'option_key' => 'twitter',
			'cat_key'    => 'cs_block_twitter_scripts_cat',
			'script'     => 'twitter'
		),
		'Bing Tag'                  => array(
			'option_key' => 'big_tag',
			'cat_key'    => 'cs_block_big_tag_scripts_cat',
			'script'     => 'big_tag'
		),
		'Linkedin widget/Analytics' => array(
			'option_key' => 'ln_tag',
			'cat_key'    => 'cs_block_ln_tag_scripts_cat',
			'script'     => 'ln_tag'
		),
		'Pinterest widget'          => array(
			'option_key' => 'pin_tag',
			'cat_key'    => 'cs_block_pin_tag_scripts_cat',
			'script'     => 'pin_tag'
		),
		'Youtube embed'             => array(
			'option_key' => 'yt_embedded',
			'cat_key'    => 'cs_block_yt_embedded_scripts_cat',
			'script'     => 'yt_embedded'
		),
		'Vimeo embed'               => array(
			'option_key' => 'vimeo_embedded',
			'cat_key'    => 'cs_block_vimeo_embedded_scripts_cat',
			'script'     => 'vimeo_embedded'
		),
		'Hubspot Analytics'         => array(
			'option_key' => 'hubspot',
			'cat_key'    => 'cs_block_hubspot_scripts_cat',
			'script'     => 'hubspot'
		),
		'Matomo Analytics'          => array(
			'option_key' => 'matomo',
			'cat_key'    => 'cs_block_matomo_scripts_cat',
			'script'     => 'matomo'
		),
		'Addthis widget'            => array(
			'option_key' => 'addthis',
			'cat_key'    => 'cs_block_addthis_scripts_cat',
			'script'     => 'addthis'
		),
		'Sharethis widget'          => array(
			'option_key' => 'sharethis',
			'cat_key'    => 'cs_block_sharethis_scripts_cat',
			'script'     => 'sharethis'
		),
		'Soundcloud embed'          => array(
			'option_key' => 'soundcloud',
			'cat_key'    => 'cs_block_soundcloud_scripts_cat',
			'script'     => 'soundcloud'
		),
		'Slideshare embed'          => array(
			'option_key' => 'slideshare',
			'cat_key'    => 'cs_block_slideshare_scripts_cat',
			'script'     => 'slideshare'
		),
		'Tik-tok pixel'             => array(
			'option_key' => 'tiktok',
			'cat_key'    => 'cs_block_tiktok_scripts_cat',
			'script'     => 'tiktok'
		),
		'Reddit pixel'             => array(
			'option_key' => 'reddit',
			'cat_key'    => 'cs_block_reddit_pixel_scripts_cat',
			'script'     => 'reddit'
		)
	);
}

function render_stepper( $step, $step_count = 5 ) {
	$step_width = 100 / $step_count;
	?>
    <div class="stepper">
        <div class="progress" style="width:<?php echo $step_width * $step - ( $step_width / 2 ); ?>%;"></div>
		<?php for ( $i = 1; $i <= $step_count; $i++ ): ?>
            <div class="step-number <?php echo ( $i <= $step ? 'active' : '' ) . ' ' . ( $i == $step ? 'current' : '' ); ?>"><?php echo $i < $step ? '<i class="icon-check"></i>' : $i; ?></div>
		<?php endfor; ?>
    </div>
	<?php
}

function cardCollapseSettings( $card_toggle = true ) {
	?>
    <span class="card-collapse <?php echo esc_attr( $card_toggle ? 'card-toggle' : '' ); ?>">
        <?php include CMPRO_PLUGIN_VIEWS_PATH . '/admin/buttons/admin-properties-button.php'; ?>
    </span>
	<?php
}

function cardCollapseSettingsWithText( $edit = 'Edit', $collapse = 'Collapse', $card_toggle = true, $return = false ) {
	$html = '
    <div class="card-collapse card-collapse-text ' . ( $card_toggle ? 'card-toggle' : '' ) . '">
        <div class="first">' . esc_html__( $edit, 'consent-magic' ) . '
        </div>

        <div class="second translate-right">' . esc_html__( $collapse, 'consent-magic' ) . '
        </div>
    </div>';

	if ( $return ) {
		return $html;
	} else {
		echo $html;
	}
}

function renderShortcodeBlock( $shortcodes ) {
	if ( !empty( $shortcodes ) ) :
		foreach ( $shortcodes as $shortcode ) :
			$codes = is_array( $shortcode[ 'code' ] ) ? $shortcode[ 'code' ] : array(
				array(
					'text' => $shortcode[ 'code' ],
					'type' => 'code'
				)
			);
			$copy_code = implode( '', array_column( $codes, 'text' ) );
			?>

            <div class="shortcode-wrap">
                <div class="shortcode">
                    <div class="shortcode-inner">
						<?php foreach ( $codes as $code ) : ?>
                            <span class="shortcode-code">
                            <?php if ( $code[ 'type' ] == 'code' ) : ?>
                                <span class="shortcode-title"><?php echo esc_html( $code[ 'text' ] ); ?></span>
                            <?php else : ?>
                                <span class="fw-500"><?php echo esc_html( $code[ 'text' ] ); ?></span>
                            <?php endif; ?>
                        </span>
						<?php endforeach; ?>
                    </div>

                    <div class="shortcode-copy"
                         onclick="copyShortcode( '<?php echo esc_html( $copy_code ); ?>', this)">
						<?php renderTooltip( 'icon-content-copy' ); ?>
                    </div>
                </div>
                <div class="line"></div>
                <div class="shortcode-content">
                    <p>
						<?php echo esc_html( $shortcode[ 'description' ] ); ?>
                    </p>
                </div>
            </div>
		<?php
		endforeach;
	endif;
}

function renderTooltip( $icon, $text_tooltip = 'Copied' ) {
	?>
    <div class="tooltip">
        <span class="copy-btn"><i class="<?php echo esc_attr( $icon ); ?>"></i></span>
        <span class="tooltip-text"><?php echo esc_attr__( $text_tooltip, 'consent-magic' ); ?></span>
    </div>
	<?php
}