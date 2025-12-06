<?php

namespace ConsentMagicPro;

if ( !defined( 'ABSPATH' ) ) {
	exit; // Exit if accessed directly.
}

class CS_AdminPageIab extends CS_AdminPage implements CS_Page {

	public function renderPage() {
		$this->manageCSPermissions();
		$this->updateAdminSettingsHandle();
		$this->renderHTML();
	}

	public function renderTitle() {
		add_action( 'init', function() {
			$this->page_title = __( 'IAB Settings', 'consent-magic' );
		} );
	}

	private function updateAdminSettingsHandle() {
		if ( isset( $_POST[ 'cs_update_action' ] ) && $_POST[ 'cs_update_action' ] == 'update_admin_settings_form' ) {

			// Check nonce:
			check_admin_referer( 'cs-update-' . CMPRO_SETTINGS_FIELD );

			$the_options = ConsentMagic()->getCSOptions();
			foreach ( $the_options as $key => $value ) {
				if ( isset( $_POST[ 'cs' ][ $this->plugin_name ][ $key ] ) ) {
					ConsentMagic()->updateOptions( array( $key => sanitize_text_field( $_POST[ 'cs' ][ $this->plugin_name ][ $key ] ) ) );
				}
			}

			$vendor_list  = CS_IAB_Integration()->get_vendor_list();
			$iab_settings = CS_IAB_Integration()->get_settings();
			if ( !empty( $vendor_list ) ) {
				if ( !empty( $vendor_list->purposes ) ) {
					foreach ( $vendor_list->purposes as $purpose ) {
						if ( isset( $_POST[ 'cs' ][ $this->plugin_name ][ 'cs_iab_show_purpose_' . $purpose->id ] ) ) {
							$iab_settings->purposes->{$purpose->id} = sanitize_text_field( $_POST[ 'cs' ][ $this->plugin_name ][ 'cs_iab_show_purpose_' . $purpose->id ] );
						}
					}
				}

				if ( !empty( $vendor_list->specialPurposes ) ) {
					foreach ( $vendor_list->specialPurposes as $purpose ) {
						if ( isset( $_POST[ 'cs' ][ $this->plugin_name ][ 'cs_iab_show_special_purpose_' . $purpose->id ] ) ) {
							$iab_settings->specialPurposes->{$purpose->id} = sanitize_text_field( $_POST[ 'cs' ][ $this->plugin_name ][ 'cs_iab_show_special_purpose_' . $purpose->id ] );
						}
					}
				}

				if ( !empty( $vendor_list->features ) ) {
					foreach ( $vendor_list->features as $feature ) {
						if ( isset( $_POST[ 'cs' ][ $this->plugin_name ][ 'cs_iab_show_feature_' . $feature->id ] ) ) {
							$iab_settings->features->{$feature->id} = sanitize_text_field( $_POST[ 'cs' ][ $this->plugin_name ][ 'cs_iab_show_feature_' . $feature->id ] );
						}
					}
				}

				if ( !empty( $vendor_list->specialFeatures ) ) {
					foreach ( $vendor_list->specialFeatures as $feature ) {
						if ( isset( $_POST[ 'cs' ][ $this->plugin_name ][ 'cs_iab_show_special_feature_' . $feature->id ] ) ) {
							$iab_settings->specialFeatures->{$feature->id} = sanitize_text_field( $_POST[ 'cs' ][ $this->plugin_name ][ 'cs_iab_show_special_feature_' . $feature->id ] );
						}
					}
				}
			}

			if ( isset( $_POST[ 'cs' ][ $this->plugin_name ][ 'cs_iab_vendor_settings' ] ) ) {
				$vendors = json_decode( stripslashes( sanitize_text_field( $_POST[ 'cs' ][ $this->plugin_name ][ 'cs_iab_vendor_settings' ] ) ) );
				if ( !empty( $vendors ) ) {
					foreach ( $vendors as $key => $vendor ) {
						if ( isset( $iab_settings->vendors->{$key} ) ) {
							$iab_settings->vendors->{$key} = $vendor;
						}
					}
				}
			}

			if ( isset( $_POST[ 'cs' ][ $this->plugin_name ][ 'cs_iab_additional_vendor_settings' ] ) ) {
				$vendors = json_decode( stripslashes( sanitize_text_field( $_POST[ 'cs' ][ $this->plugin_name ][ 'cs_iab_additional_vendor_settings' ] ) ) );
				if ( !empty( $vendors ) ) {
					foreach ( $vendors as $key => $vendor ) {
						if ( isset( $iab_settings->additional_vendors->{$key} ) ) {
							$iab_settings->additional_vendors->{$key} = $vendor;
						}
					}
				}
			}

			CS_IAB_Integration()->update_settings( $iab_settings );

			$this->renderSuccessMessage();
		}
	}
}