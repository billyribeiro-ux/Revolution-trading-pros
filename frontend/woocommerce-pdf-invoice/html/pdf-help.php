<?php

	if ( ! defined( 'ABSPATH' ) ) {
		exit; // Exit if accessed directly
	}

	// Multisite
	$blog_id = NULL;
	if( function_exists( 'get_current_blog_id' ) ) {
		$blog_id = get_current_blog_id();
	}

	$admin_url = get_admin_url( $blog_id );

	$theme = wp_get_theme();

	$plugin_template    = PDFPLUGINPATH . 'templates/template.php';
	$theme_template     = get_stylesheet_directory() . '/pdf_templates/template.php';

	$current_user 		= wp_get_current_user();

	$directory_error 	= '';
	$file_error 		= '';

	$create_url 		= sanitize_text_field( wp_unslash( $_SERVER['REQUEST_URI'] ) );
	$create_url 		= add_query_arg( 'make_template', MD5( $current_user->user_email ), $create_url );

	$item_loop 			= 0;

	// Allow admins to copy template.php to the theme folder
	if( isset( $_GET['template'] ) && in_array( 'administrator', $current_user->roles ) && isset( $_GET['make_template'] ) && $_GET['make_template'] === MD5( $current_user->user_email ) ) {

		$plugin   = WC_pdf_invoice_helper_functions::get_template_folder( 'plugin' ) . wc_clean( $_GET['template'] ) . '.php';
		$template = WC_pdf_invoice_helper_functions::get_template_folder( 'theme' ) . wc_clean( $_GET['template'] ) . '.php';

		// Create templates directory in theme folder
		if( wp_mkdir_p( dirname( $template ), 0777, true ) ) {
			$directory_error = "failed to create directory...<br />";
		}

		// Create template file in theme templates folder
		if (!copy( $plugin, $template ) ) {
		    $file_error = "failed to copy $theme_template...<br />";
		}

	}

?>
<h3>Setting up PDF Invoices and customising the invoice template.</h3>

<?php
// Theme folder is not writable, can't magic the templates to the theme folder
if( !wp_is_writable( get_template_directory() ) ) {
	echo '<div class="notice notice-error"><h4>' .  esc_html__('Your theme folder is not writable.', 'woocommerce-pdf-invoice' ) . '</h4></div>';
}

if( ( defined('DISALLOW_FILE_EDIT') && DISALLOW_FILE_EDIT ) || !current_user_can( 'edit_themes' ) ) {
	echo '<div class="notice notice-error">';
	echo '<h4>' .  esc_html__('The theme editor is disabled on this site. You will not be able to edit your PDF templates from within WordPress.', 'woocommerce-pdf-invoice' ) . '</h4>';
	if( !current_user_can( 'edit_themes' ) ) {
		echo '<p>' . esc_html__('Your user account settings may mean the template editor is disabled. Contact a site administrator.', 'woocommerce-pdf-invoice' ) . '</p>';
	}
	echo '</div>';
}

// Theme folder is writable, let the magic happen
if( wp_is_writable( get_template_directory() ) ) {

	$all_templates = WC_pdf_invoice_helper_functions::get_template_files( 'plugin' ) + WC_pdf_invoice_helper_functions::get_template_files( 'theme' );
?>
	<table width="100%" class="widefat pdf-invoice-template-table">
		<thead>
			<tr>
			<th align="left"><?php esc_html_e('Template Name', 'woocommerce-pdf-invoice' ); ?></th>
			<th align="left"><?php esc_html_e('In plugin folder?', 'woocommerce-pdf-invoice' ); ?></th>
			<th align="left"><?php esc_html_e('In theme folder?', 'woocommerce-pdf-invoice' ); ?></th>
			<th align="left"><?php esc_html_e('Action', 'woocommerce-pdf-invoice' ); ?></th>
			<th align="left"><?php esc_html_e('Duplicate?', 'woocommerce-pdf-invoice' ); ?></th>
			</tr>
		</thead>
<?php
	foreach ( $all_templates as $k => $v ) {

		$item_loop++;

		if( $item_loop % 2 == 0 ){ 
	        $row_class 		= ' template_odd';  
	    } else { 
	        $row_class 		= ' template_even';
	    }

		echo '<tbody class="' . esc_html( $row_class ). '"><tr class="' .esc_html( $row_class ) . '"><td>' . esc_html( $v ) . '</td>' ;
		if( WC_pdf_invoice_helper_functions::is_template_there( $k, 'plugin' ) ) {
			echo '<td>Y</td>';
		} else {
			echo '<td>N</td>';
		}

		if( WC_pdf_invoice_helper_functions::is_template_there( $k, 'theme' ) ) {

			$edit_url = admin_url( 'theme-editor.php?file=pdf_templates%2F' . $k . '.php&theme=' . $theme->get( 'TextDomain' ), 'https' );
			echo '<td>Y</td>';
			echo '<td>' .
			'<a href="' . wp_kses_post( $edit_url ) . '" title"' .  esc_html__('Customise your template file', 'woocommerce-pdf-invoice' ) . '">' .  esc_html__('Customise', 'woocommerce-pdf-invoice' ) . '</a>' .
			wc_help_tip( esc_html__('Customise your template file using the WordPress theme editor', 'woocommerce-pdf-invoice' ) ) // phpcs:ignore WordPress.Security.EscapeOutput.OutputNotEscaped
			. '</td>';

		} else {

			$create_url = add_query_arg( 'template', $k , $create_url );
			echo '<td>N</td>';
			echo '<td>' .
			'<a href="' . wp_kses_post( $create_url ) . '" title"' .  esc_html__('Copy the template file into your theme folder ready for customising.', 'woocommerce-pdf-invoice' ) . '">' .  esc_html__('Copy', 'woocommerce-pdf-invoice' ) . '</a>' .
			wc_help_tip( esc_html__('Copy the template file into your theme folder ready for customising.', 'woocommerce-pdf-invoice' ) ) // phpcs:ignore WordPress.Security.EscapeOutput.OutputNotEscaped
			. '</td>';

		}

		echo '<td>' . WC_pdf_invoice_helper_functions::get_duplicate_form( $k, $current_user->user_email ) . '</td>'; // phpcs:ignore WordPress.Security.EscapeOutput.OutputNotEscaped

		echo '</tr></tbody>';
	}

	echo '</table>';

?>

<?php
}
?>

<ol class="documentation-toc">
	<li><a href="https://docs.woocommerce.com/document/woocommerce-pdf-invoice-setup-and-customization/#section-1" target="_blank"><?php esc_html_e("Installation and Updating", 'woocommerce-pdf-invoice' ); ?></a></li>
	<li><a href="https://docs.woocommerce.com/document/woocommerce-pdf-invoice-setup-and-customization/#section-2" target="_blank"><?php esc_html_e("Setup and Configuration", 'woocommerce-pdf-invoice' ); ?></a>
		<ol>
			<li><a href="https://docs.woocommerce.com/document/woocommerce-pdf-invoice-setup-and-customization/#section-3" target="_blank"><?php esc_html_e("Date Format", 'woocommerce-pdf-invoice' ); ?></a></li>
			<li><a href="https://docs.woocommerce.com/document/woocommerce-pdf-invoice-setup-and-customization/#section-4" target="_blank"><?php esc_html_e("Testing Invoice Layouts", 'woocommerce-pdf-invoice' ); ?></a></li>
			<li><a href="https://docs.woocommerce.com/document/woocommerce-pdf-invoice-setup-and-customization/#section-5" target="_blank"><?php esc_html_e("PDF File Name Parameters", 'woocommerce-pdf-invoice' ); ?></a></li>
			<li><a href="https://docs.woocommerce.com/document/woocommerce-pdf-invoice-setup-and-customization/#section-6" target="_blank"><?php esc_html_e("PDF Invoice Number Suffix", 'woocommerce-pdf-invoice' ); ?></a></li>
		</ol>
	</li>
	<li><a href="https://docs.woocommerce.com/document/woocommerce-pdf-invoice-setup-and-customization/#section-7" target="_blank"><?php esc_html_e("Create Invoices for Existing Orders", 'woocommerce-pdf-invoice' ); ?></a></li>
	<li><a href="https://docs.woocommerce.com/document/woocommerce-pdf-invoice-setup-and-customization/#section-8" target="_blank"><?php esc_html_e("Order Screen", 'woocommerce-pdf-invoice' ); ?></a></li>
	<li><a href="https://docs.woocommerce.com/document/woocommerce-pdf-invoice-setup-and-customization/#section-9" target="_blank"><?php esc_html_e("Usage", 'woocommerce-pdf-invoice' ); ?></a></li>
	<li><a href="https://docs.woocommerce.com/document/woocommerce-pdf-invoice-setup-and-customization/#section-10" target="_blank"><?php esc_html_e("Customization", 'woocommerce-pdf-invoice' ); ?></a>
		<ol>
			<li><a href="https://docs.woocommerce.com/document/woocommerce-pdf-invoice-setup-and-customization/#section-11" target="_blank"><?php esc_html_e("Copy the template to your theme", 'woocommerce-pdf-invoice' ); ?></a></li>
			<li><a href="https://docs.woocommerce.com/document/woocommerce-pdf-invoice-setup-and-customization/#section-12" target="_blank"><?php esc_html_e("Add new placeholders", 'woocommerce-pdf-invoice' ); ?></a></li>
			<li><a href="https://docs.woocommerce.com/document/woocommerce-pdf-invoice-setup-and-customization/#section-13" target="_blank"><?php esc_html_e("Using the included fonts", 'woocommerce-pdf-invoice' ); ?></a></li>
			<li><a href="https://docs.woocommerce.com/document/woocommerce-pdf-invoice-setup-and-customization/#section-14" target="_blank"><?php esc_html_e("Using Google Fonts", 'woocommerce-pdf-invoice' ); ?></a></li>
			<li><a href="https://docs.woocommerce.com/document/woocommerce-pdf-invoice-setup-and-customization/#section-15" target="_blank"><?php esc_html_e("Using a True Type font", 'woocommerce-pdf-invoice' ); ?></a></li>
		</ol>
	</li>
	<li><a href="https://docs.woocommerce.com/document/woocommerce-pdf-invoice-setup-and-customization/#section-16" target="_blank"><?php esc_html_e("Using Hooks and Filters", 'woocommerce-pdf-invoice' ); ?></a></li>
	<li><a href="https://docs.woocommerce.com/document/woocommerce-pdf-invoice-setup-and-customization/#section-17" target="_blank"><?php esc_html_e("Translating PDF Invoice", 'woocommerce-pdf-invoice' ); ?></a></li>
	<li><a href="https://docs.woocommerce.com/document/woocommerce-pdf-invoice-setup-and-customization/#section-18" target="_blank"><?php esc_html_e("FAQ", 'woocommerce-pdf-invoice' ); ?></a>
		<ol>
			<li><a href="https://docs.woocommerce.com/document/woocommerce-pdf-invoice-setup-and-customization/#section-19" target="_blank"><?php esc_html_e("Are PDFs stored in a secure way?", 'woocommerce-pdf-invoice' ); ?></a></li>
		</ol>
	</li>
	<li><a href="https://docs.woocommerce.com/document/woocommerce-pdf-invoice-setup-and-customization/#section-20" target="_blank"><?php esc_html_e("Bulk Exporting PDFs", 'woocommerce-pdf-invoice' ); ?></a></li>
	<li><a href="https://docs.woocommerce.com/document/woocommerce-pdf-invoice-setup-and-customization/#section-21" target="_blank"><?php esc_html_e("Feedback and feature requests", 'woocommerce-pdf-invoice' ); ?></a></li>
	<li><a href="https://docs.woocommerce.com/document/woocommerce-pdf-invoice-setup-and-customization/#section-22" target="_blank"><?php esc_html_e("Questions &amp; Support", 'woocommerce-pdf-invoice' ); ?></a></li>
</ol>

