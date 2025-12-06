<?php

namespace FluentFormSignature;

class Component
{
    /**
     * Push signature editor component to the fluentform components array.
     *
     * @param  array $components
     * @return array $components
     */
    public static function push($components)
    {
        $signature = [
            'index'      => 8,
            'element'    => 'signature',
            'attributes' => [
                'name' => 'signature',
                'class' => '',
            ],
            'settings' => [
                'container_class'  => '',
                'label'            => __('Signature', 'fluentform-signature'),
                'label_placement'  => '',
                'help_message'     => '',
	            'sign_background_color' => '#ffffff',
	            'sign_border_color' => '#FFEB3B',
	            'sign_pen_color' => '#333',
	            'sign_pen_size' => 2,
	            'sign_pad_height' => 200,
	            'sign_instruction' => __('Sign Here', 'fluentform-signature'),
	            'admin_field_label' => __('Signature', 'fluentform-signature'),
                'validation_rules' => [
                    'required' => [
                        'value'   => false,
                        'message' => __('This field is required', 'fluentform-signature'),
                    ],
                ],
                'conditional_logics' => []
            ],
            'editor_options' => [
                'title'      => 'Signature',
                'icon_class' => 'ff-icon-ink-pen',
                'template'   => 'imagePlaceholder',
                'imageUrl' => FLUENTFORM_SIGNATURE_URL . 'src/assets/images/signature.png'
            ],
        ];

        $signatureComponent = apply_filters_deprecated(
            'fluentform-signature-component',
            [
                $signature
            ],
            FLUENTFORM_FRAMEWORK_UPGRADE,
            'fluentform/signature_component',
            'Use fluentform/signature_component instead of fluentform-signature-component.'
        );
        $signatureComponent = apply_filters('fluentform/signature_component', $signature);

        $components['advanced'][] = $signatureComponent;

        return $components;
    }
    
    public static function  placementSettings( $placement_settings ) {
	    $placement_settings['signature'] = array(
		    'general'  => array(
			    'label',
			    'label_placement',
			    'sign_instruction',
			    'admin_field_label',
			    'validation_rules',
		    ),
		    'advanced' => array(
				'name',
			    'sign_background_color',
			    'sign_border_color',
			    'sign_pen_color',
			    'sign_pen_size',
			    'sign_pad_height',
			    'help_message',
			    'container_class',
			    'class',
			    'conditional_logics',
		    ),
	    );
	    return $placement_settings;
    }
    
    public static function addTags($tags) {
	    $tags['signature'] = array('signature', 'stamp', 'mark', 'autograph');
	    return $tags;
    }
    
    public static function customizationSettings($customization_settings) {
    	
	    $customization_settings['sign_background_color'] = array(
		    'template'  => 'inputColor',
		    'label'     => __('Pad Background Color', 'fluentform-signature'),
		    'help_text' => __('The Background color of the signature pad', 'fluentform-signature')
	    );
	    $customization_settings['sign_border_color'] = array(
		    'template'  => 'inputColor',
		    'label'     => __('Border Color', 'fluentform-signature'),
		    'help_text' => __('The Border color of the signature pad', 'fluentform-signature')
	    );
	    $customization_settings['sign_pen_color'] = array(
		    'template'  => 'inputColor',
		    'label'     => __('Pen Color', 'fluentform-signature'),
		    'help_text' => __('The Pen color of the signature pad', 'fluentform-signature')
	    );
	    $customization_settings['sign_pen_size'] = array(
			'type'      => 'number',
			'template'  => 'inputText',
		    'label'     => __('Pen Size', 'fluentform-signature'),
		    'help_text' => __('Size of your Pen', 'fluentform-signature')
	    );
	    $customization_settings['sign_instruction'] = array(
			'template'  => 'inputText',
		    'label'     => __('Sign Instruction', 'fluentform-signature'),
		    'help_text' => __('Write Label that will show under the signature pad', 'fluentform-signature')
	    );
	    $customization_settings['sign_pad_height'] = array(
			'type'      => 'number',
			'template'  => 'inputText',
		    'label'     => __('Pad Height', 'fluentform-signature'),
		    'help_text' => __('Height of the signature pad', 'fluentform-signature'),
	    );

	    return $customization_settings;
    }
}
