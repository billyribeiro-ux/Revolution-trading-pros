<?php
/**
 * Admin View: User Agent Meta Box
 *
 * @var AV8_Edit_Interface $this
 * @var string $ip
 */

if ( ! defined( 'ABSPATH' ) ) {
	exit; // Exit if accessed directly
}

$converted = $this->receipt->status() === 'Converted';
$width_p   = $converted ? '17%' : '20%';


?>

<div class="woocommerce_cart_reports_clientdata_wrapper">
	<table cellpading="0" width="100%" cellspacing="0" class="woocommerce_cart_reports_clientdata_items">
		<thead>
		<tr>
			<th class="hist1" style="width: <?php echo esc_attr( $width_p ); ?>; text-align: left">
				<?php
				esc_html_e( 'Cart Last Updated', 'woocommerce_cart_reports' );
				echo wp_kses_post( av8_tooltip(
					__(
						'<i>Cart Last Updated</i> indicates the last time the customer performed a cart-related action on your site. These actions include viewing the cart, adding new items to the cart, updating quantities, or removing products from the cart.',
						'woocommerce_cart_reports'
					)
				) );
				?>
			</th>
			<?php if ( $converted ) : ?>
				<th class="ip" style="width: <?php echo esc_attr( $width_p ); ?>; text-align: left">
					<?php
					esc_html_e( 'Time To Conversion', 'woocommerce_cart_reports' );
					echo wp_kses_post( av8_tooltip(
						__(
							'<i>Time to Conversion</i> indicates total time elapsed from when the cart was first created until the actual conversion. (purchase)',
							'woocommerce_cart_reports'
						)
					) );
					?>
				</th>
			<?php else: ?>
				<th class="ip" style="width: <?php echo esc_attr( $width_p ); ?>; text-align: left">
					<?php
					esc_html_e( 'Cart Age', 'woocommerce_cart_reports' );
					echo wp_kses_post( av8_tooltip(
						__(
							'<i>Cart Age</i> indicates the total time elapsed since this non-converted cart has been created.',
							'woocommerce_cart_reports'
						)
					) ); ?>
				</th>
			<?php endif; ?>

			<th class="ip" style="width: <?php echo esc_attr( $width_p ); ?>; text-align: left">
				<?php esc_html_e( 'Customer IP Address', 'woocommerce_cart_reports' ); ?>
			</th>
		</tr>
		</thead>

		<tbody>
		<tr>
			<td class="lastUpdated">
				<p>
					<?php echo esc_html( get_the_modified_time( 'F j, Y \a\t g:i a', $this->receipt->post_id ) ); ?>
				</p>
			</td>
			<?php if ( $converted ): ?>
				<td>
					<p><?php echo esc_html( $this->receipt->get_age_text() ); ?></p>
				</td>
			<?php else: ?>
			<?php
			$created               = $this->receipt->created();
			$gmt_offset            = $this->receipt->get_timezone_offset();
			$hour_in_seconds       = 1 * 60 * 60;
			$gmt_offset_seconds    = abs( $gmt_offset * $hour_in_seconds );
			$created_with_timezone = abs( $created + $gmt_offset_seconds );
			?>

				<td>
					<div id="counter">
						<span style="color:lightgray;">
							<?php esc_html_e(
								'Not Available',
								'woocommerce_cart_reports'
							); ?>
						</span>
					</div>
				</td>

				<script type='text/javascript'>

                  function DaysHMSCounter(initDate, id) {
                    this.counterDate = moment(initDate, 'X')
                    this.container = document.getElementById(id)
                    this.update()
                  }

                  DaysHMSCounter.prototype.calculate = function () {
                    var now = moment()
                    this.duration = moment.duration(now.diff(this.counterDate))
                  }

                  DaysHMSCounter.prototype.update = function () {
                    this.calculate()
                    this.container.innerHTML = '<p>' + this.duration.format(
                      '[<strong>]d[</strong>] __ [<strong>]h[</strong>] _ [<strong>]m[</strong>] _ [<strong>]s[</strong>] _') + '</p>'
                    var self = this
                    setTimeout(function () {
                        self.update()
                      },
                      (
                        1000
                      )
                    )
                  }

                  window.onload = function () {
                    new DaysHMSCounter('<?php echo esc_js( $created_with_timezone ); ?>', 'counter')
                  }
				</script>
			<?php endif; ?>

			<td>
				<p>
					<?php if ( $ip ): ?>
						<?php echo esc_html( $ip ); ?>
					<?php else: ?>
						<span style="color:lightgray;">
							<?php
							esc_html_e( 'Not Available', 'woocommerce_cart_reports' );
							echo wp_kses_post( av8_tooltip(
								__(
									'You have probably unchecked "Log IP Address" in the WooCommerce Cart Reports settings panel.',
									'woocommerce_cart_reports'
								)
							) ); ?>
						</span>
					<?php endif; ?>
				</p>
			</td>
		</tr>
		</tbody>
	</table>
</div>

