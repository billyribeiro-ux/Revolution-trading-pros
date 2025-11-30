<?php

namespace ConsentMagicPro;

// If this file is called directly, abort.
if ( !defined( 'WPINC' ) ) {
	die;
}

?>

<div id="consent-magic" class="wrap <?php echo is_rtl() ? 'rtl' : ''; ?>">
    <div class="cm-general-menu">
        <div class="cm-logo">
            <img src="<?php echo esc_url( CMPRO_PLUGIN_URL ); ?>/assets/images/cm-logo.svg" alt="cm-logo">
        </div>

		<?php
		$tab_head_arr = array(
			'cs-general'            => esc_html__( 'Main', 'consent-magic' ),
			//no need to add translation here
			'cs-script-blocking'    => esc_html__( 'Scripts and Cookies', 'consent-magic' ),
			'cs-policy-gen'         => esc_html__( 'Privacy Policy', 'consent-magic' ),
			'cs-text'               => esc_html__( 'Text', 'consent-magic' ),
			'cs-multi-step-design'  => esc_html__( 'Multi-step design', 'consent-magic' ),
			'cs-single-step-design' => esc_html__( 'Single-step design', 'consent-magic' ),
		);

		if ( $this->page_slug == 'main' && count( $_GET ) === 1 ) {
			generate_settings_tabhead( $tab_head_arr );
		} else {
			generate_settings_tabhead( $tab_head_arr, 'consent-magic', $this->page_slug );
		}
		?>
    </div>

    <div class="cm-general-menu-mobile">
		<?php
		if ( $this->page_slug == 'main' && count( $_GET ) === 1 ) {
			generate_settings_tabhead_mobile( $tab_head_arr );
		} else {
			generate_settings_tabhead_mobile( $tab_head_arr, 'consent-magic', $this->page_slug );
		}
		?>
    </div>

	<?php if ( ConsentMagic()->getOption( 'cs_check_flow' ) ) : ?>
        <h1 class="cm-general-title"><?php echo esc_html( $this->page_title ); ?></h1>
	<?php else: ?>
        <h1 class="cm-general-title-empty"></h1>
	<?php endif; ?>

    <div class="container-wrap">
		<?php if ( !ConsentMagic()->getOption( 'cs_check_flow' ) ) {
			if ( CMPRO_LICENSE_TYPE == 'edd' ) {
				require_once CMPRO_PLUGIN_VIEWS_PATH . 'admin/first-flow/html-first-flow-edd.php';
			} elseif ( CMPRO_LICENSE_TYPE == 'woo' ) {
				require_once CMPRO_PLUGIN_VIEWS_PATH . 'admin/first-flow/html-first-flow-woo.php';
			}
		} elseif ( isset( $this->page_slug ) ) {
			$settings_view = CMPRO_PLUGIN_VIEWS_PATH . "admin/{$this->page_slug}/html-{$this->page_slug}.php";
			if ( file_exists( $settings_view ) ) {
				require_once $settings_view;
			}
		} ?>
    </div>

	<?php if ( ConsentMagic()->getOption( 'cs_check_flow' ) && $this->page_slug !== 'statistics' && $this->page_slug !== 'license' ) {
		include_once CMPRO_PLUGIN_VIEWS_PATH . "admin/buttons/admin-settings-save-button.php";
	}
	?>

</div>
