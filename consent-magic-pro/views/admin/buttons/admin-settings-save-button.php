<?php

namespace ConsentMagicPro;

// If this file is called directly, abort.
if ( !defined( 'WPINC' ) ) {
	die;
}
?>
<div class="save-settings">
	<?php if ( isset( $_GET[ 'primary_rule_id' ] ) ) : ?>
        <div class="preview_list_action">
            <button type="button" class="btn btn_preview_rule_edit">
				<?php esc_html_e( 'Preview', 'consent-magic' ); ?>
            </button>
			<?php include CMPRO_PLUGIN_VIEWS_PATH . "admin/components/spinner.php"; ?>
        </div>
	<?php elseif ( isset( $_GET[ 'design_template' ] ) ) : ?>
		<?php include CMPRO_PLUGIN_VIEWS_PATH . "admin/components/design-preview-buttons.php"; ?>
	<?php endif; ?>

    <div class="video-link">
		<?php if ( !empty( CMPRO_VIDEO_URL ) && !empty( CMPRO_VIDEO_TITLE ) && !isset( $_GET[ 'design_template' ] ) ) : ?>
            <span class="font-semibold"><?php esc_html_e( 'Recommended', 'consent-magic' ); ?>: </span>
            <a href="<?php echo esc_url( CMPRO_VIDEO_URL ); ?>" target="_blank" class="link link-underline">
				<?php echo esc_html( CMPRO_VIDEO_TITLE ); ?>
            </a>
		<?php endif; ?>
    </div>

    <div class="empty-block"></div>

    <div class="save-settings-actions">
		<?php
		if ( isset( $_GET[ 'new_rule' ] ) || isset( $_GET[ 'primary_rule_id' ] ) ) : ?>
            <a href="<?php echo esc_url( admin_url( 'admin.php?page=consent-magic' ) ); ?>"
               class="back-button"><?php esc_html_e( 'Back', 'consent-magic' ); ?></a>
            <button id="cm-save-settings"><?php esc_html_e( 'Save', 'consent-magic' ); ?></button>
		<?php elseif ( isset( $_GET[ 'new_script' ] ) || isset( $_GET[ 'script_id' ] ) ): ?>
            <a href="<?php echo esc_url( admin_url( 'admin.php?page=consent-magic&tab=cs-script-blocking' ) ); ?>"
               class="back-button"><?php esc_html_e( 'Back', 'consent-magic' ); ?></a>
            <button id="cm-save-settings"><?php esc_html_e( 'Save', 'consent-magic' ); ?></button>
		<?php elseif ( isset( $_GET[ 'design_template' ] ) ): ?>
			<?php $design_type = isset( $_GET[ 'design' ] ) ? sanitize_text_field( $_GET[ 'design' ] ) : ''; ?>
            <a href="<?php echo esc_url( admin_url( "admin.php?page=consent-magic&tab=$design_type" ) ); ?>"
               class="back-button"><?php esc_html_e( 'Back', 'consent-magic' ); ?></a>
            <button id="cm-save-settings"><?php esc_html_e( 'Save', 'consent-magic' ); ?></button>
		<?php elseif ( isset( $_GET[ 'page' ] ) && $_GET[ 'page' ] == 'cs-proof-consent' && isset( $_GET[ 'tab' ] ) && $_GET[ 'tab' ] == 'cs-records' ): ?>
            <button id="cm-delete-consent"><?php esc_html_e( 'Delete consent', 'consent-magic' ); ?></button>
            <div class="export-consent-button">
                <button id="cm-export-consent"><?php esc_html_e( 'Export consent', 'consent-magic' ); ?></button>
				<?php include CMPRO_PLUGIN_VIEWS_PATH . "admin/components/spinner.php"; ?>
            </div>
		<?php else : ?>
			<?php if ( isset( $_GET[ 'tab' ] ) && ( $_GET[ 'tab' ] == 'cs-multi-step-design' || $_GET[ 'tab' ] == 'cs-single-step-design' ) ): ?>
                <button id="cm-restore-defaults-design-btn"><?php esc_html_e( 'Restore defaults', 'consent-magic' ); ?></button>
			<?php endif; ?>
            <button id="cm-save-settings"><?php esc_html_e( 'Save Changes', 'consent-magic' ); ?></button>
		<?php endif; ?>
    </div>
</div>
