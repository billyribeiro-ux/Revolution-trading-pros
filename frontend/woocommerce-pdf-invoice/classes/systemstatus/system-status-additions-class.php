<?php

    if ( ! defined( 'ABSPATH' ) ) {
        exit; // Exit if accessed directly
    }
    
	/**
	 * Admin Notices for SagePay Form
	 */
	class PDF_Invoices_System_Status_Additions {
		
		public function __construct() {

			/**
			 * Add some notices to WooCommerce System Status
			 */
			add_action( 'woocommerce_system_status_report', array( $this, 'action_woocommerce_system_status_report' ), 50, 0 );

		}

		function action_woocommerce_system_status_report() {

			$settings 		= get_option( 'woocommerce_pdf_invoice_settings' );

			$pdf_generator 	= isset( $settings['pdf_generator'] ) ? $settings['pdf_generator'] : NULL;
			$create_invoice = isset( $settings['create_invoice'] ) ? $settings['create_invoice'] : NULL;
			$setchroot 		= isset( $settings['setchroot'] ) ? $settings['setchroot'] : NULL;
			$enable_remote 	= isset( $settings['enable_remote'] ) ? $settings['enable_remote'] : NULL;
			$pdf_date 		= isset( $settings['pdf_date'] ) ? $settings['pdf_date'] : NULL;
			$pdf_creation 	= isset( $settings['pdf_creation'] ) ? $settings['pdf_creation'] : NULL;

			$mailer          = WC()->mailer();
			$email_templates = $mailer->get_emails();

			$debug_data   = array();

			$debug_data['php_version_test'] = array(
				'name'    => __( 'PHP Version', 'woocommerce-pdf-invoice' ),
				'tip'	  => __( 'Is the version of PHP installed compatible?', 'woocommerce-pdf-invoice' ),
				/* translators: PHP version */
				'note'    => version_compare( phpversion(), "7.1" ) ? sprintf( __( 'Version %s is installed.', 'woocommerce-pdf-invoice' ), phpversion() ) :  sprintf( __( 'Please contact your host to upgrade your PHP version to at least version 7.1, this server is using version %s.', 'woocommerce-pdf-invoice' ), phpversion() ),
				'success' => version_compare( phpversion(), "7.1" ) ? 1 : 0,
			);

			$debug_data['extension_loaded_domdocument'] = array(
				'name'    => __( 'DOMDocument', 'woocommerce-pdf-invoice' ),
				'tip'	  => __( 'Is DOMDocument installed on this hosting?', 'woocommerce-pdf-invoice' ),
				/* translators: DOMDocument version */
				'note'    => class_exists("DOMDocument") ? sprintf( __( 'Version %s is installed.', 'woocommerce-pdf-invoice' ), phpversion("DOM") ) : __( 'DOMDocument is required, please contact your host to have it installed.', 'woocommerce-pdf-invoice' ),
				'success' => class_exists("DOMDocument") ? 1 : 0,
			);

			$debug_data['extension_loaded_pcre'] = array(
				'name'    => __( 'PCRE', 'woocommerce-pdf-invoice' ),
				'tip'	  => __( 'Is PCRE installed on this hosting?', 'woocommerce-pdf-invoice' ),
				/* translators: PCRE version */
				'note'    => function_exists("preg_match") && @preg_match("/./u", "a") ? sprintf( __( 'Version %s is installed.', 'woocommerce-pdf-invoice' ), phpversion("pcre") ) : __( 'PCRE is required with Unicode support (the \"u\" modifier).', 'woocommerce-pdf-invoice' ),
				'success' => function_exists("preg_match") && @preg_match("/./u", "a") ? 1 : 0,
			);

			$debug_data['extension_loaded_zlib'] = array(
				'name'    => __( 'Zlib', 'woocommerce-pdf-invoice' ),
				'tip'	  => __( 'Is Zlib installed on this hosting?', 'woocommerce-pdf-invoice' ),
				/* translators: ZLib version */
				'note'    => function_exists("gzcompress") ? sprintf( __( 'Version %s is installed.', 'woocommerce-pdf-invoice' ), phpversion("zlib") ) : __( 'Recommended to compress PDF documents.', 'woocommerce-pdf-invoice' ),
				'success' => function_exists("gzcompress") ? 1 : 0,
			);

			$debug_data['extension_loaded_ziparchive'] = array(
				'name'    => __( 'ZipArchive', 'woocommerce-pdf-invoice' ),
				'tip'	  => __( 'Is ZipArchive installed on this hosting?', 'woocommerce-pdf-invoice' ),
				/* translators: ZipArchive version */
				'note'    => class_exists("ZipArchive") ? sprintf( __( 'Version %s is installed.', 'woocommerce-pdf-invoice' ), phpversion("zip") ) : __( 'Required for bulk exporting invoices', 'woocommerce-pdf-invoice' ),
				'success' => class_exists("ZipArchive") ? 1 : 0,
			);

			$debug_data['extension_loaded_iconv'] = array(
				'name'    => __( 'ICONV', 'woocommerce-pdf-invoice' ),
				'tip'	  => __( 'Is ICONV installed on this hosting?', 'woocommerce-pdf-invoice' ),
				/* translators: ICONV version */
				'note'    => extension_loaded( 'iconv' ) ? sprintf( __( 'Version %s is installed.', 'woocommerce-pdf-invoice' ), ICONV_VERSION ) :  __( 'ICONV is required, please contact your host to have it installed.', 'woocommerce-pdf-invoice' ),
				'success' => extension_loaded( 'iconv' ) ? 1 : 0,
			);

			$debug_data['extension_loaded_mbstring'] = array(
				'name'    => __( 'MBString', 'woocommerce-pdf-invoice' ),
				'tip'	  => __( 'Is MBString installed on this hosting?', 'woocommerce-pdf-invoice' ),
				/* translators: mbstring version */
				'note'    => extension_loaded( 'mbstring' ) ? __( 'Yes', 'woocommerce-pdf-invoice' ) :  __( 'Multibyte Support is required, please contact your host to have it installed.', 'woocommerce-pdf-invoice' ),
				'success' => extension_loaded( 'mbstring' ) ? 1 : 0,
			);

			$debug_data['extension_loaded_gd'] = array(
				'name'    => __( 'GD', 'woocommerce-pdf-invoice' ),
				'tip'	  => __( 'Is GD installed on this hosting?', 'woocommerce-pdf-invoice' ),
				/* translators: GD version */
				'note'    => extension_loaded( 'gd' ) ? sprintf( __( 'Version %s is installed.', 'woocommerce-pdf-invoice' ), phpversion("gd") ) :  __( 'Required if you have images in your PDFs', 'woocommerce-pdf-invoice' ),
				'success' => extension_loaded( 'gd' ) ? 1 : 0,
			);

			$debug_data['pdf_generator'] = array(
				'name'    => __( 'PDF Generator', 'woocommerce-pdf-invoice' ),
				'tip'	  => __( 'Which PDF Generator is being used to create the PDF?', 'woocommerce-pdf-invoice' ),
				'note'    => isset( $pdf_generator ) ? ucwords( $pdf_generator ) :  __( 'None', 'woocommerce-pdf-invoice' ),
				'success' => isset( $pdf_generator ) ? 1 : 0,
			);

			$debug_data['create_invoice'] = array(
				'name'    => __( 'Order status to create invoice', 'woocommerce-pdf-invoice' ),
				'tip'     => __( 'When is the invoice being created?', 'woocommerce-pdf-invoice' ),
				'note'    => isset( $create_invoice ) ? ucwords( $create_invoice ) :  __( 'No status selected', 'woocommerce-pdf-invoice' ),
				'success' => isset( $create_invoice ) ? 1 : 0,
			);

			$debug_data['setchroot'] = array(
				'name'    => __( 'Set Resources Folder', 'woocommerce-pdf-invoice' ),
				'tip'     => __( 'Is the ESet Resources Folder set to Yes or No?', 'woocommerce-pdf-invoice' ),
				'note'    => ucwords( $setchroot ),
				'success' => $setchroot ? 1 : 0,
			);

			$debug_data['enable_remote'] = array(
				'name'    => __( 'Remote Logo', 'woocommerce-pdf-invoice' ),
				'tip'     => __( 'Is the Enable Remote Logo set to Yes or No?', 'woocommerce-pdf-invoice' ),
				'note'    => ucwords( $enable_remote ),
				'success' => $enable_remote ? 1 : 0,
			);

			$debug_data['pdf_date'] = array(
				'name'    => __( 'Which date does the invoice use?', 'woocommerce-pdf-invoice' ),
				'tip'     => __( 'Order date or Completed date?', 'woocommerce-pdf-invoice' ),
				'note'    => ucwords( $pdf_date ),
				'success' => $pdf_date ? 1 : 0,
			);

			$debug_data['pdf_creation'] = array(
				'name'    => __( 'PDF Creation Method', 'woocommerce-pdf-invoice' ),
				'tip'     => __( 'How is the PDF created? On the fly or as a file?', 'woocommerce-pdf-invoice' ),
				'note'    => ucwords( $pdf_creation ),
				'success' => $pdf_creation ? 1 : 0,
			);

			// Generate notes for each email
			foreach ( $email_templates as $email_template ) {

				$settings = $email_template->settings;
				
				$template  = isset( $settings['pdf_invoice_attach_pdf_invoice'] ) && $settings['pdf_invoice_attach_pdf_invoice'] === 'yes' ? 
				/* translators: template file name */
				sprintf( __( 'Yes, using template file %s.', 'woocommerce-pdf-invoice' ), ucwords( $settings['pdf_invoice_template_pdf_invoice'] ) ) : __( 'No.', 'woocommerce-pdf-invoice' );

				/* translators: recipient of email */
				$recipiant = isset( $email_template->recipient ) ? sprintf( __( ' Recipient : %s', 'woocommerce-pdf-invoice' ), $email_template->recipient ) : __( ' Recipient : customer', 'woocommerce-pdf-invoice' ); 

				$debug_data[$email_template->id] = array(
					'name'    => $email_template->title . __( ' email ', 'woocommerce-pdf-invoice' ),
					'tip'     => __( 'If an invoice is available, is it attached to this email?', 'woocommerce-pdf-invoice' ),
					'note'    => $template . $recipiant,
					'success' => isset( $settings['pdf_invoice_attach_pdf_invoice'] )  && $settings['pdf_invoice_attach_pdf_invoice'] === 'yes' ? 1 : 0,
				);

			}

			include( PDFPLUGINPATH . 'assets/templates/systemstatus.php' );

		}

	} // End class
	
	$PDF_Invoices_System_Status_Additions = new PDF_Invoices_System_Status_Additions;
