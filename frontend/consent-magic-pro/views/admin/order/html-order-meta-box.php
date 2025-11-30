<?php

namespace ConsentMagicPro;

if ( !defined( 'ABSPATH' ) ) {
	exit; // Exit if accessed directly.
}

$data = array(
	"cs_type"                => $order->get_meta( "_cs_type" ),
	"cs_expressed"           => $order->get_meta( "_cs_expressed" ),
	"cs_category_yes"        => $order->get_meta( "_cs_category_yes" ),
	"cs_category_no"         => $order->get_meta( "_cs_category_no" ),
	"cs_title"               => $order->get_meta( "_cs_title" ),
	"cs_google_consent_mode" => array(
		'analytics_storage'  => $order->get_meta( "_cm_analytics_storage" ),
		'ad_storage'         => $order->get_meta( "_cm_ad_storage" ),
		'ad_user_data'       => $order->get_meta( "_cm_ad_user_data" ),
		'ad_personalization' => $order->get_meta( "_cm_ad_personalization" ),
	),
);

$terms_list = get_cookies_terms_list( false, true );

$order_options            = array(
	'cs_customer_email_consent_type'      => 'option',
	'cs_customer_email_rule_name'         => 'option',
	'cs_customer_email_consent_expressed' => 'option',
);
$wp_current_lang          = get_locale();
$cs_user_default_language = ConsentMagic()->getOption( 'cs_user_default_language' );
$cs_language_availability = ConsentMagic()->getOption( 'cs_language_availability' );
if ( isset( $cs_language_availability[ $wp_current_lang ] ) && $cs_language_availability[ $wp_current_lang ] == 0 ) {
	$wp_current_lang = $cs_user_default_language;
}

$order_options = generate_front_text( $order_options, false, false, $wp_current_lang, $cs_user_default_language );

if ( $data && is_array( $data ) ) :
	?>
    <style>
        table.cs_order_meta {
            width: 100%;
            text-align: left
        }

        table.cs_order_meta td.cs_border span {
            border-top: 1px solid #f1f1f1;
            display: block;
        }

        table.cs_order_meta th,
        table.cs_order_meta td {
            padding: 10px
        }

        table.cs_order_meta th {
            width: 70%;
        }
    </style>

    <table class="cs_order_meta">
        <tr>
            <th><?php echo esc_attr( $order_options[ 'cs_customer_email_consent_type' ] ); ?>:</th>
            <td><?php echo !empty( $data[ 'cs_type' ] ) ? esc_attr( $data[ 'cs_type' ] ) : "" ?></td>
        </tr>

        <tr>
            <th><?php echo esc_attr( $order_options[ 'cs_customer_email_rule_name' ] ); ?>:</th>
            <td><?php echo !empty( $data[ 'cs_title' ] ) ? esc_attr( $data[ 'cs_title' ] ) : "" ?></td>
        </tr>

        <tr>
            <th><?php echo esc_attr( $order_options[ 'cs_customer_email_consent_expressed' ] ); ?>:</th>
            <td><?php echo !empty( $data[ 'cs_expressed' ] ) ? esc_attr( $data[ 'cs_expressed' ] ) : "" ?></td>
        </tr>

		<?php if ( in_array( 'granted', $data[ 'cs_google_consent_mode' ] ) || in_array( 'denied', $data[ 'cs_google_consent_mode' ] ) ) : ?>
            <tr>
                <td colspan="2" class="cs_border"><span></span></td>
            </tr>

            <tr>
                <th colspan="2"><?php esc_html_e( 'Google consent mode expressed:', "consent-magic" ); ?></th>
            </tr>

			<?php
			foreach ( $data[ 'cs_google_consent_mode' ] as $key => $mode ) :
				?>
                <tr>
                    <th><?php echo esc_html( $key ) . ': '; ?></th>
                    <td><?php echo $mode ? esc_html( $mode ) : esc_html__( 'No data', "consent-magic" ); ?></td>
                </tr>
			<?php endforeach; ?>
		<?php endif; ?>

		<?php if ( ( ( !empty( $data[ 'cs_category_yes' ] ) || !empty( $data[ 'cs_category_no' ] ) ) || $data[ 'cs_type' ] == 'inform_and_opiout' ) ) {
			?>
            <tr>
                <td colspan="2" class="cs_border"><span></span></td>
            </tr>

            <tr>
                <th colspan="2"> <?php echo esc_html__( 'Consent by category', 'consent-magic' ); ?>:</th>
            </tr>

			<?php if ( $data[ 'cs_type' ] == 'Inform and Opt-out' && $data[ 'cs_expressed' ] == 'No' ) { ?>
				<?php if ( !empty( $terms_list ) ) {
					foreach ( $terms_list as $cat ) {
						if ( $cat !== 'Unassigned' ) {
							?>
                            <tr>
                                <th><?php echo esc_html( $cat ) . ': '; ?></th>
                                <td><?php esc_html_e( 'Yes', 'consent-magic' ); ?></td>
                            </tr>
						<?php }
					} ?>
				<?php } ?>
			<?php } else { ?>
				<?php if ( !empty( $data[ 'cs_category_yes' ] ) ) {
					foreach ( $data[ 'cs_category_yes' ] as $cat ) {
						?>
                        <tr>
                            <th><?php echo esc_html( $cat ) . ': '; ?></th>
                            <td><?php esc_html_e( 'Yes', 'consent-magic' ); ?></td>
                        </tr>
					<?php } ?>
				<?php } ?>
				<?php if ( !empty( $data[ 'cs_category_no' ] ) ) {
					foreach ( $data[ 'cs_category_no' ] as $cat ) {
						if ( $cat !== 'Unassigned' && $cat !== 'Necessary' ) {
							?>
                            <tr>
                                <th><?php echo esc_html( $cat ) . ': '; ?></th>
                                <td><?php esc_html_e( 'No', 'consent-magic' ); ?></td>
                            </tr>
						<?php } ?>
					<?php } ?>
				<?php } ?>
			<?php } ?>
		<?php } ?>
    </table>
<?php else: ?>
    <h2><?php esc_html_e( 'No data', "consent-magic" ); ?></h2>
<?php endif; ?>