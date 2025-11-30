<?php
/**
 * Blocking Elementor widgets
 * @link  https://www.pixelyoursite.com/plugins/consentmagic/
 * @since 1.9.0
 */

namespace ConsentMagicPro;

if ( !defined( 'ABSPATH' ) ) {
	exit; // Exit if accessed directly.
}

class CS_Elementor {

	private static $_instance = null;

	public $settings = [];

	public $youtube_url = '';

	/**
	 * $_instance CS_Elementor
	 * @return CS_Elementor|null
	 */
	public static function instance() {
		if ( is_null( self::$_instance ) ) {
			self::$_instance = new self();
		}

		return self::$_instance;
	}

	public function check_elementor_widgets( $video_object ) {

		$active_rule_id        = ConsentMagic()->get_active_rule_id();
		$native_scripts_option = get_post_meta( $active_rule_id, '_cs_native_scripts', true );
		$cs_type               = get_post_meta( $active_rule_id, '_cs_type', true );
		$native_scripts        = ( $cs_type == 'iab' && $native_scripts_option == 1 ) || $cs_type != 'iab';

		if ( ( ConsentMagic()->getOption( 'cs_plugin_activation' ) == 1 && ConsentMagic()->getOption( 'cs_script_blocking_enabled' ) == 1 && $native_scripts ) || ConsentMagic()->getOption( 'cs_enable_site_cache' ) == 1 ) {
			if ( $video_object->get_type() === 'widget' ) {
				$video_settings = $video_object->get_frontend_settings();

				if ( !empty( $video_settings ) && !empty( $video_settings[ 'video_type' ] ) && $video_settings[ 'video_type' ] === 'youtube' ) {
					if ( isset( $video_settings[ 'lightbox' ] ) && $video_settings[ 'lightbox' ] === 'yes' ) {
						return;
					}

					$this->youtube_url = $video_settings[ 'youtube_url' ];
					if ( strpos( $this->youtube_url, 'youtube.com' ) !== false || strpos( $this->youtube_url, 'youtu.be' ) !== false || strpos( $this->youtube_url, 'vimeo.com' ) !== false ) {

						$this->settings = $video_settings;
						if ( empty( $this->settings[ 'show_image_overlay' ] ) || $this->settings[ 'show_image_overlay' ] !== 'yes' ) {
							$video_object->remove_render_attribute( '_wrapper', 'data-settings', null );
						}

						ob_start();
						add_action( 'elementor/frontend/widget/after_render', array( $this, 'modify_video_widget' ) );
					}
				}
			}
		}
	}

	public function modify_video_widget() {

		$content_escaped = ob_get_contents();
		ob_end_clean();

		if ( isset( $this->settings[ 'show_image_overlay' ] ) && $this->settings[ 'show_image_overlay' ] === 'yes' ) {
			$placeholder_text = apply_filters( 'cm_video_placeholder', '' );
			$placeholder      = "<div class='cs_manage_elementor_placeholder disabled'>" . $placeholder_text . "</div>";
			$content_escaped  = str_replace( '<div class="elementor-custom-embed-image-overlay', $placeholder . '<div class="elementor-custom-embed-image-overlay', $content_escaped );
			echo $content_escaped;
		} else {
			$atts                  = [];
			$atts[ 'rel' ]         = 0;
			$atts[ 'enablejsapi' ] = 1;
			$atts[ 'origin' ]      = get_site_url();

			if ( isset( $this->settings[ 'controls' ] ) ) {
				$atts[ 'controls' ] = $this->settings[ 'controls' ] === 'yes' ? 1 : 0;
			}

			if ( isset( $this->settings[ 'play_on_mobile' ] ) ) {
				$atts[ 'playsinline' ] = $this->settings[ 'play_on_mobile' ] === 'yes' ? 1 : 0;
			}

			if ( isset( $this->settings[ 'modestbranding' ] ) ) {
				$atts[ 'modestbranding' ] = $this->settings[ 'modestbranding' ] === 'yes' ? 1 : 0;
			}

			if ( isset( $this->settings[ 'autoplay' ] ) ) {
				$atts[ 'autoplay' ] = $this->settings[ 'autoplay' ] === 'yes' ? 1 : 0;
			}

			if ( isset( $this->settings[ 'mute' ] ) ) {
				$atts[ 'mute' ] = $this->settings[ 'mute' ] === 'yes' ? 1 : 0;
			}

			if ( isset( $this->settings[ 'loop' ] ) ) {
				$atts[ 'loop' ] = $this->settings[ 'loop' ] === 'yes' ? 1 : 0;
			}

			if ( isset( $this->settings[ 'start' ] ) ) {
				$atts[ 'start' ] = (int) ( $this->settings[ 'start' ] );
			}

			if ( isset( $this->settings[ 'end' ] ) ) {
				$atts[ 'end' ] = (int) ( $this->settings[ 'end' ] );
			}

			$video_parameters = '&';
			if ( !empty( $atts ) ) {
				foreach ( $atts as $key => $value ) {
					$video_parameters .= $key . '=' . $value . '&';
				}
			}

			if ( strpos( $this->youtube_url, 'embed' ) !== false ) {
				$this->youtube_url = preg_replace( '#https://www\.youtube\.com/embed/([^?&]+).*#', 'https://www.youtube.com/watch?v=$1', $this->youtube_url );
			}

			$video_iframe = wp_oembed_get( $this->youtube_url );

			if ( ( preg_match( '/<iframe.*(src\s*=\s*(?:"|\')(.*?))(?:"|\').*>.*<\/iframe>/i', $video_iframe, $element_match ) ) ) {
				$video_iframe = str_replace( $element_match[ 1 ], $element_match[ 1 ] . $video_parameters, $video_iframe );
			}

			echo str_replace( '<div class="elementor-video"></div>', '<div class="elementor-video" style="height: 100%">' . $video_iframe . '</div>', $content_escaped );
		}

		$this->settings = array();
		remove_action( 'elementor/frontend/widget/after_render', array(
			$this,
			'modify_video_widget'
		) );
	}
}

function CS_Elementor() {
	return CS_Elementor::instance();
}

CS_Elementor();