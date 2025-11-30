<?php

if ( is_multisite() && !defined( 'COOKIEHASH' ) ) {
	wp_cookie_constants();
}

// Define constants for supported wp_template_part_area taxonomy.
if ( !defined( 'WP_TEMPLATE_PART_AREA_HEADER' ) ) {
	define( 'WP_TEMPLATE_PART_AREA_HEADER', 'header' );
}

if ( !function_exists( 'wp_get_current_user' ) ) :
	/**
	 * Retrieve the current user object.
	 *
	 * Will set the current user, if the current user is not set. The current user
	 * will be set to the logged-in person. If no user is logged-in, then it will
	 * set the current user to 0, which is invalid and won't have any permissions.
	 *
	 * @return WP_User Current WP_User instance.
	 * @see _wp_get_current_user()
	 * @global WP_User $current_user Checks if the current user is set.
	 *
	 * @since 2.0.3
	 *
	 */
	function wp_get_current_user() {
		include_once( ABSPATH . 'wp-includes/user.php' );
		return _wp_get_current_user();
	}
endif;

if ( !function_exists( 'wp_set_current_user' ) ) :
	/**
	 * Changes the current user by ID or name.
	 *
	 * Set $id to null and specify a name if you do not know a user's ID.
	 *
	 * Some WordPress functionality is based on the current user and not based on
	 * the signed in user. Therefore, it opens the ability to edit and perform
	 * actions on users who aren't signed in.
	 *
	 * @param int|null $id User ID.
	 * @param string $name User's username.
	 * @return WP_User Current user User object.
	 * @global WP_User $current_user The current user object which holds the user data.
	 *
	 * @since 2.0.3
	 *
	 */
	function wp_set_current_user( $id, $name = '' ) {
		global $current_user;

		// If `$id` matches the current user, there is nothing to do.
		if ( isset( $current_user ) && ( $current_user instanceof WP_User ) && ( $id == $current_user->ID ) && ( null !== $id ) ) {
			return $current_user;
		}

		$current_user = new WP_User( $id, $name );

		setup_userdata( $current_user->ID );

		/**
		 * Fires after the current user is set.
		 *
		 * @since 2.0.1
		 */
		do_action( 'set_current_user' );

		return $current_user;
	}
endif;

if ( ! function_exists( 'wp_validate_auth_cookie' ) ) :
	global $wp_version;
	if ( version_compare( $wp_version, '6.7.99' ) >= 0 ) {
		/**
		 * Validates authentication cookie.
		 *
		 * The checks include making sure that the authentication cookie is set and
		 * pulling in the contents (if $cookie is not used).
		 *
		 * Makes sure the cookie is not expired. Verifies the hash in cookie is what is
		 * should be and compares the two.
		 *
		 * @since 2.5.0
		 *
		 * @global int $login_grace_period
		 *
		 * @param string $cookie Optional. If used, will validate contents instead of cookie's.
		 * @param string $scheme Optional. The cookie scheme to use: 'auth', 'secure_auth', or 'logged_in'.
		 *                       Note: This does *not* default to 'auth' like other cookie functions.
		 * @return int|false User ID if valid cookie, false if invalid.
		 */
		function wp_validate_auth_cookie( $cookie = '', $scheme = '' ) {
			$cookie_elements = wp_parse_auth_cookie( $cookie, $scheme );
			if ( ! $cookie_elements ) {
				/**
				 * Fires if an authentication cookie is malformed.
				 *
				 * @since 2.7.0
				 *
				 * @param string $cookie Malformed auth cookie.
				 * @param string $scheme Authentication scheme. Values include 'auth', 'secure_auth',
				 *                       or 'logged_in'.
				 */
				do_action( 'auth_cookie_malformed', $cookie, $scheme );
				return false;
			}

			$scheme     = $cookie_elements['scheme'];
			$username   = $cookie_elements['username'];
			$hmac       = $cookie_elements['hmac'];
			$token      = $cookie_elements['token'];
			$expiration = $cookie_elements['expiration'];

			$expired = (int) $expiration;

			// Allow a grace period for POST and Ajax requests.
			if ( wp_doing_ajax() || 'POST' === $_SERVER['REQUEST_METHOD'] ) {
				$expired += HOUR_IN_SECONDS;
			}

			// Quick check to see if an honest cookie has expired.
			if ( $expired < time() ) {
				/**
				 * Fires once an authentication cookie has expired.
				 *
				 * @since 2.7.0
				 *
				 * @param string[] $cookie_elements {
				 *     Authentication cookie components. None of the components should be assumed
				 *     to be valid as they come directly from a client-provided cookie value.
				 *
				 *     @type string $username   User's username.
				 *     @type string $expiration The time the cookie expires as a UNIX timestamp.
				 *     @type string $token      User's session token used.
				 *     @type string $hmac       The security hash for the cookie.
				 *     @type string $scheme     The cookie scheme to use.
				 * }
				 */
				do_action( 'auth_cookie_expired', $cookie_elements );
				return false;
			}

			$user = get_user_by( 'login', $username );
			if ( ! $user ) {
				/**
				 * Fires if a bad username is entered in the user authentication process.
				 *
				 * @since 2.7.0
				 *
				 * @param string[] $cookie_elements {
				 *     Authentication cookie components. None of the components should be assumed
				 *     to be valid as they come directly from a client-provided cookie value.
				 *
				 *     @type string $username   User's username.
				 *     @type string $expiration The time the cookie expires as a UNIX timestamp.
				 *     @type string $token      User's session token used.
				 *     @type string $hmac       The security hash for the cookie.
				 *     @type string $scheme     The cookie scheme to use.
				 * }
				 */
				do_action( 'auth_cookie_bad_username', $cookie_elements );
				return false;
			}

			if ( str_starts_with( $user->user_pass, '$P$' ) || str_starts_with( $user->user_pass, '$2y$' ) ) {
				// Retain previous behaviour of phpass or vanilla bcrypt hashed passwords.
				$pass_frag = substr( $user->user_pass, 8, 4 );
			} else {
				// Otherwise, use a substring from the end of the hash to avoid dealing with potentially long hash prefixes.
				$pass_frag = substr( $user->user_pass, -4 );
			}

			$key = wp_hash( $username . '|' . $pass_frag . '|' . $expiration . '|' . $token, $scheme );

			$hash = hash_hmac( 'sha256', $username . '|' . $expiration . '|' . $token, $key );

			if ( ! hash_equals( $hash, $hmac ) ) {
				/**
				 * Fires if a bad authentication cookie hash is encountered.
				 *
				 * @since 2.7.0
				 *
				 * @param string[] $cookie_elements {
				 *     Authentication cookie components. None of the components should be assumed
				 *     to be valid as they come directly from a client-provided cookie value.
				 *
				 *     @type string $username   User's username.
				 *     @type string $expiration The time the cookie expires as a UNIX timestamp.
				 *     @type string $token      User's session token used.
				 *     @type string $hmac       The security hash for the cookie.
				 *     @type string $scheme     The cookie scheme to use.
				 * }
				 */
				do_action( 'auth_cookie_bad_hash', $cookie_elements );
				return false;
			}

			$manager = WP_Session_Tokens::get_instance( $user->ID );
			if ( ! $manager->verify( $token ) ) {
				/**
				 * Fires if a bad session token is encountered.
				 *
				 * @since 4.0.0
				 *
				 * @param string[] $cookie_elements {
				 *     Authentication cookie components. None of the components should be assumed
				 *     to be valid as they come directly from a client-provided cookie value.
				 *
				 *     @type string $username   User's username.
				 *     @type string $expiration The time the cookie expires as a UNIX timestamp.
				 *     @type string $token      User's session token used.
				 *     @type string $hmac       The security hash for the cookie.
				 *     @type string $scheme     The cookie scheme to use.
				 * }
				 */
				do_action( 'auth_cookie_bad_session_token', $cookie_elements );
				return false;
			}

			// Ajax/POST grace period set above.
			if ( $expiration < time() ) {
				$GLOBALS['login_grace_period'] = 1;
			}

			/**
			 * Fires once an authentication cookie has been validated.
			 *
			 * @since 2.7.0
			 *
			 * @param string[] $cookie_elements {
			 *     Authentication cookie components.
			 *
			 *     @type string $username   User's username.
			 *     @type string $expiration The time the cookie expires as a UNIX timestamp.
			 *     @type string $token      User's session token used.
			 *     @type string $hmac       The security hash for the cookie.
			 *     @type string $scheme     The cookie scheme to use.
			 * }
			 * @param WP_User  $user            User object.
			 */
			do_action( 'auth_cookie_valid', $cookie_elements, $user );

			return $user->ID;
		}
	} else {
		/**
		 * Validates authentication cookie.
		 *
		 * The checks include making sure that the authentication cookie is set and
		 * pulling in the contents (if $cookie is not used).
		 *
		 * Makes sure the cookie is not expired. Verifies the hash in cookie is what is
		 * should be and compares the two.
		 *
		 * @param string $cookie Optional. If used, will validate contents instead of cookie's.
		 * @param string $scheme Optional. The cookie scheme to use: 'auth', 'secure_auth', or 'logged_in'.
		 * @return int|false User ID if valid cookie, false if invalid.
		 * @global int $login_grace_period
		 *
		 * @since 2.5.0
		 *
		 */
			function wp_validate_auth_cookie( $cookie = '', $scheme = '' ) {
				$cookie_elements = wp_parse_auth_cookie( $cookie, $scheme );
				if ( !$cookie_elements ) {
					/**
					 * Fires if an authentication cookie is malformed.
					 *
					 * @param string $cookie Malformed auth cookie.
					 * @param string $scheme Authentication scheme. Values include 'auth', 'secure_auth',
					 *                       or 'logged_in'.
					 * @since 2.7.0
					 *
					 */
					do_action( 'auth_cookie_malformed', $cookie, $scheme );
					return false;
				}

				$scheme = $cookie_elements[ 'scheme' ];
				$username = $cookie_elements[ 'username' ];
				$hmac = $cookie_elements[ 'hmac' ];
				$token = $cookie_elements[ 'token' ];
				$expired = $cookie_elements[ 'expiration' ];
				$expiration = $cookie_elements[ 'expiration' ];

				// Allow a grace period for POST and Ajax requests.
				if ( wp_doing_ajax() || 'POST' === $_SERVER[ 'REQUEST_METHOD' ] ) {
					$expired += HOUR_IN_SECONDS;
				}

				// Quick check to see if an honest cookie has expired.
				if ( $expired < time() ) {
					/**
					 * Fires once an authentication cookie has expired.
					 *
					 * @param string[] $cookie_elements {
					 *     Authentication cookie components. None of the components should be assumed
					 *     to be valid as they come directly from a client-provided cookie value.
					 *
					 * @type string $username User's username.
					 * @type string $expiration The time the cookie expires as a UNIX timestamp.
					 * @type string $token User's session token used.
					 * @type string $hmac The security hash for the cookie.
					 * @type string $scheme The cookie scheme to use.
					 * }
					 * @since 2.7.0
					 *
					 */
					do_action( 'auth_cookie_expired', $cookie_elements );
					return false;
				}

				$user = get_user_by( 'login', $username );
				if ( !$user ) {
					/**
					 * Fires if a bad username is entered in the user authentication process.
					 *
					 * @param string[] $cookie_elements {
					 *     Authentication cookie components. None of the components should be assumed
					 *     to be valid as they come directly from a client-provided cookie value.
					 *
					 * @type string $username User's username.
					 * @type string $expiration The time the cookie expires as a UNIX timestamp.
					 * @type string $token User's session token used.
					 * @type string $hmac The security hash for the cookie.
					 * @type string $scheme The cookie scheme to use.
					 * }
					 * @since 2.7.0
					 *
					 */
					do_action( 'auth_cookie_bad_username', $cookie_elements );
					return false;
				}

				$pass_frag = substr( $user->user_pass, 8, 4 );

				$key = wp_hash( $username . '|' . $pass_frag . '|' . $expiration . '|' . $token, $scheme );

				// If ext/hash is not present, compat.php's hash_hmac() does not support sha256.
				$algo = function_exists( 'hash' ) ? 'sha256' : 'sha1';
				$hash = hash_hmac( $algo, $username . '|' . $expiration . '|' . $token, $key );

				if ( !hash_equals( $hash, $hmac ) ) {
					/**
					 * Fires if a bad authentication cookie hash is encountered.
					 *
					 * @param string[] $cookie_elements {
					 *     Authentication cookie components. None of the components should be assumed
					 *     to be valid as they come directly from a client-provided cookie value.
					 *
					 * @type string $username User's username.
					 * @type string $expiration The time the cookie expires as a UNIX timestamp.
					 * @type string $token User's session token used.
					 * @type string $hmac The security hash for the cookie.
					 * @type string $scheme The cookie scheme to use.
					 * }
					 * @since 2.7.0
					 *
					 */
					do_action( 'auth_cookie_bad_hash', $cookie_elements );
					return false;
				}

				$manager = WP_Session_Tokens::get_instance( $user->ID );
				if ( !$manager->verify( $token ) ) {
					/**
					 * Fires if a bad session token is encountered.
					 *
					 * @param string[] $cookie_elements {
					 *     Authentication cookie components. None of the components should be assumed
					 *     to be valid as they come directly from a client-provided cookie value.
					 *
					 * @type string $username User's username.
					 * @type string $expiration The time the cookie expires as a UNIX timestamp.
					 * @type string $token User's session token used.
					 * @type string $hmac The security hash for the cookie.
					 * @type string $scheme The cookie scheme to use.
					 * }
					 * @since 4.0.0
					 *
					 */
					do_action( 'auth_cookie_bad_session_token', $cookie_elements );
					return false;
				}

				// Ajax/POST grace period set above.
				if ( $expiration < time() ) {
					$GLOBALS[ 'login_grace_period' ] = 1;
				}

				/**
				 * Fires once an authentication cookie has been validated.
				 *
				 * @param string[] $cookie_elements {
				 *     Authentication cookie components.
				 *
				 * @type string $username User's username.
				 * @type string $expiration The time the cookie expires as a UNIX timestamp.
				 * @type string $token User's session token used.
				 * @type string $hmac The security hash for the cookie.
				 * @type string $scheme The cookie scheme to use.
				 * }
				 * @param WP_User $user User object.
				 * @since 2.7.0
				 *
				 */
				do_action( 'auth_cookie_valid', $cookie_elements, $user );

				return $user->ID;
			}
	}

endif;

if ( !function_exists( 'wp_parse_auth_cookie' ) ) :
	/**
	 * Parses a cookie into its components.
	 *
	 * @param string $cookie Authentication cookie.
	 * @param string $scheme Optional. The cookie scheme to use: 'auth', 'secure_auth', or 'logged_in'.
	 * @return string[]|false {
	 *     Authentication cookie components. None of the components should be assumed
	 *     to be valid as they come directly from a client-provided cookie value. If
	 *     the cookie value is malformed, false is returned.
	 *
	 * @type string $username User's username.
	 * @type string $expiration The time the cookie expires as a UNIX timestamp.
	 * @type string $token User's session token used.
	 * @type string $hmac The security hash for the cookie.
	 * @type string $scheme The cookie scheme to use.
	 * }
	 * @since 4.0.0 The `$token` element was added to the return value.
	 *
	 * @since 2.7.0
	 */
	function wp_parse_auth_cookie( $cookie = '', $scheme = '' ) {
		if ( empty( $cookie ) ) {
			switch ( $scheme ) {
				case 'auth':
					$cookie_name = AUTH_COOKIE;
					break;
				case 'secure_auth':
					$cookie_name = SECURE_AUTH_COOKIE;
					break;
				case 'logged_in':
					$cookie_name = LOGGED_IN_COOKIE;
					break;
				default:
					if ( is_ssl() ) {
						$cookie_name = SECURE_AUTH_COOKIE;
						$scheme = 'secure_auth';
					} else {
						$cookie_name = AUTH_COOKIE;
						$scheme = 'auth';
					}
			}

			if ( empty( $_COOKIE[ $cookie_name ] ) ) {
				return false;
			}
			$cookie = $_COOKIE[ $cookie_name ];
		}

		$cookie_elements = explode( '|', $cookie );
		if ( count( $cookie_elements ) !== 4 ) {
			return false;
		}

		list( $username, $expiration, $token, $hmac ) = $cookie_elements;

		return compact( 'username', 'expiration', 'token', 'hmac', 'scheme' );
	}
endif;

if ( !function_exists( 'get_userdata' ) ) :
	/**
	 * Retrieve user info by user ID.
	 *
	 * @param int $user_id User ID
	 * @return WP_User|false WP_User object on success, false on failure.
	 * @since 0.71
	 *
	 */
	function get_userdata( $user_id ) {
		return get_user_by( 'id', $user_id );
	}
endif;

if ( !function_exists( 'get_user_by' ) ) :
	/**
	 * Retrieves user info by a given field.
	 *
	 * @param string $field The field to retrieve the user with. id | ID | slug | email | login.
	 * @param int|string $value A value for $field. A user ID, slug, email address, or login name.
	 * @return WP_User|false WP_User object on success, false on failure.
	 * @since 2.8.0
	 * @since 4.4.0 Added 'ID' as an alias of 'id' for the `$field` parameter.
	 *
	 * @global WP_User $current_user The current user object which holds the user data.
	 *
	 */
	function get_user_by( $field, $value ) {
		$userdata = WP_User::get_data_by( $field, $value );

		if ( !$userdata ) {
			return false;
		}

		$user = new WP_User();
		$user->init( $userdata );

		return $user;
	}
endif;

if ( !function_exists( 'wp_hash' ) ) :
	/**
	 * Get hash of given string.
	 *
	 * @param string $data Plain text to hash
	 * @param string $scheme Authentication scheme (auth, secure_auth, logged_in, nonce)
	 * @return string Hash of $data
	 * @since 2.0.3
	 *
	 */
	function wp_hash( $data, $scheme = 'auth' ) {
		$salt = wp_salt( $scheme );

		return hash_hmac( 'md5', $data, $salt );
	}
endif;

if ( !function_exists( 'wp_salt' ) ) :
	/**
	 * Returns a salt to add to hashes.
	 *
	 * Salts are created using secret keys. Secret keys are located in two places:
	 * in the database and in the wp-config.php file. The secret key in the database
	 * is randomly generated and will be appended to the secret keys in wp-config.php.
	 *
	 * The secret keys in wp-config.php should be updated to strong, random keys to maximize
	 * security. Below is an example of how the secret key constants are defined.
	 * Do not paste this example directly into wp-config.php. Instead, have a
	 * {@link https://api.wordpress.org/secret-key/1.1/salt/ secret key created} just
	 * for you.
	 *
	 *     define('AUTH_KEY',         ' Xakm<o xQy rw4EMsLKM-?!T+,PFF})H4lzcW57AF0U@N@< >M%G4Yt>f`z]MON');
	 *     define('SECURE_AUTH_KEY',  'LzJ}op]mr|6+![P}Ak:uNdJCJZd>(Hx.-Mh#Tz)pCIU#uGEnfFz|f ;;eU%/U^O~');
	 *     define('LOGGED_IN_KEY',    '|i|Ux`9<p-h$aFf(qnT:sDO:D1P^wZ$$/Ra@miTJi9G;ddp_<q}6H1)o|a +&JCM');
	 *     define('NONCE_KEY',        '%:R{[P|,s.KuMltH5}cI;/k<Gx~j!f0I)m_sIyu+&NJZ)-iO>z7X>QYR0Z_XnZ@|');
	 *     define('AUTH_SALT',        'eZyT)-Naw]F8CwA*VaW#q*|.)g@o}||wf~@C-YSt}(dh_r6EbI#A,y|nU2{B#JBW');
	 *     define('SECURE_AUTH_SALT', '!=oLUTXh,QW=H `}`L|9/^4-3 STz},T(w}W<I`.JjPi)<Bmf1v,HpGe}T1:Xt7n');
	 *     define('LOGGED_IN_SALT',   '+XSqHc;@Q*K_b|Z?NC[3H!!EONbh.n<+=uKR:>*c(u`g~EJBf#8u#R{mUEZrozmm');
	 *     define('NONCE_SALT',       'h`GXHhD>SLWVfg1(1(N{;.V!MoE(SfbA_ksP@&`+AycHcAV$+?@3q+rxV{%^VyKT');
	 *
	 * Salting passwords helps against tools which has stored hashed values of
	 * common dictionary strings. The added values makes it harder to crack.
	 *
	 * @param string $scheme Authentication scheme (auth, secure_auth, logged_in, nonce).
	 * @return string Salt value
	 * @since 2.5.0
	 *
	 * @link https://api.wordpress.org/secret-key/1.1/salt/ Create secrets for wp-config.php
	 *
	 */
	function wp_salt( $scheme = 'auth' ) {
		static $cached_salts = array();
		if ( isset( $cached_salts[ $scheme ] ) ) {
			/**
			 * Filters the WordPress salt.
			 *
			 * @param string $cached_salt Cached salt for the given scheme.
			 * @param string $scheme Authentication scheme. Values include 'auth',
			 *                            'secure_auth', 'logged_in', and 'nonce'.
			 * @since 2.5.0
			 *
			 */
			return apply_filters( 'salt', $cached_salts[ $scheme ], $scheme );
		}

		static $duplicated_keys;
		if ( null === $duplicated_keys ) {
			$duplicated_keys = array(
				'put your unique phrase here' => true,
			);

			/*
			 * translators: This string should only be translated if wp-config-sample.php is localized.
			 * You can check the localized release package or
			 * https://i18n.svn.wordpress.org/<locale code>/branches/<wp version>/dist/wp-config-sample.php
			 */
			$duplicated_keys[ __( 'put your unique phrase here' ) ] = true;

			foreach ( array(
						  'AUTH',
						  'SECURE_AUTH',
						  'LOGGED_IN',
						  'NONCE',
						  'SECRET'
					  ) as $first ) {
				foreach ( array(
							  'KEY',
							  'SALT'
						  ) as $second ) {
					if ( !defined( "{$first}_{$second}" ) ) {
						continue;
					}
					$value = constant( "{$first}_{$second}" );
					$duplicated_keys[ $value ] = isset( $duplicated_keys[ $value ] );
				}
			}
		}

		$values = array(
			'key'  => '',
			'salt' => '',
		);
		if ( defined( 'SECRET_KEY' ) && SECRET_KEY && empty( $duplicated_keys[ SECRET_KEY ] ) ) {
			$values[ 'key' ] = SECRET_KEY;
		}
		if ( 'auth' === $scheme && defined( 'SECRET_SALT' ) && SECRET_SALT && empty( $duplicated_keys[ SECRET_SALT ] ) ) {
			$values[ 'salt' ] = SECRET_SALT;
		}

		if ( in_array( $scheme, array(
			'auth',
			'secure_auth',
			'logged_in',
			'nonce'
		), true ) ) {
			foreach ( array(
						  'key',
						  'salt'
					  ) as $type ) {
				$const = strtoupper( "{$scheme}_{$type}" );
				if ( defined( $const ) && constant( $const ) && empty( $duplicated_keys[ constant( $const ) ] ) ) {
					$values[ $type ] = constant( $const );
				} elseif ( !$values[ $type ] ) {
					$values[ $type ] = get_site_option( "{$scheme}_{$type}" );
					if ( !$values[ $type ] ) {
						$values[ $type ] = wp_generate_password( 64, true, true );
						update_site_option( "{$scheme}_{$type}", $values[ $type ] );
					}
				}
			}
		} else {
			if ( !$values[ 'key' ] ) {
				$values[ 'key' ] = get_site_option( 'secret_key' );
				if ( !$values[ 'key' ] ) {
					$values[ 'key' ] = wp_generate_password( 64, true, true );
					update_site_option( 'secret_key', $values[ 'key' ] );
				}
			}
			$values[ 'salt' ] = hash_hmac( 'md5', $scheme, $values[ 'key' ] );
		}

		$cached_salts[ $scheme ] = $values[ 'key' ] . $values[ 'salt' ];

		/** This filter is documented in wp-includes/pluggable.php */
		return apply_filters( 'salt', $cached_salts[ $scheme ], $scheme );
	}
endif;

if ( !function_exists( 'get_avatar' ) ) :
	global $wp_version;
	if ( version_compare( $wp_version, '6.2.99' ) >= 0 ) {

		/**
		 * Retrieves the avatar `<img>` tag for a user, email address, MD5 hash, comment, or post.
		 *
		 * @param mixed $id_or_email The avatar to retrieve. Accepts a user ID, Gravatar MD5 hash,
		 *                              user email, WP_User object, WP_Post object, or WP_Comment object.
		 * @param int $size Optional. Height and width of the avatar in pixels. Default 96.
		 * @param string $default_value URL for the default image or a default type. Accepts:
		 *                              - '404' (return a 404 instead of a default image)
		 *                              - 'retro' (a 8-bit arcade-style pixelated face)
		 *                              - 'robohash' (a robot)
		 *                              - 'monsterid' (a monster)
		 *                              - 'wavatar' (a cartoon face)
		 *                              - 'identicon' (the "quilt", a geometric pattern)
		 *                              - 'mystery', 'mm', or 'mysteryman' (The Oyster Man)
		 *                              - 'blank' (transparent GIF)
		 *                              - 'gravatar_default' (the Gravatar logo)
		 *                              Default is the value of the 'avatar_default' option,
		 *                              with a fallback of 'mystery'.
		 * @param string $alt Optional. Alternative text to use in the avatar image tag.
		 *                              Default empty.
		 * @param array $args {
		 *     Optional. Extra arguments to retrieve the avatar.
		 *
		 * @type int $height Display height of the avatar in pixels. Defaults to $size.
		 * @type int $width Display width of the avatar in pixels. Defaults to $size.
		 * @type bool $force_default Whether to always show the default image, never the Gravatar.
		 *                                       Default false.
		 * @type string $rating What rating to display avatars up to. Accepts:
		 *                                       - 'G' (suitable for all audiences)
		 *                                       - 'PG' (possibly offensive, usually for audiences 13 and above)
		 *                                       - 'R' (intended for adult audiences above 17)
		 *                                       - 'X' (even more mature than above)
		 *                                       Default is the value of the 'avatar_rating' option.
		 * @type string $scheme URL scheme to use. See set_url_scheme() for accepted values.
		 *                                       Default null.
		 * @type array|string $class Array or string of additional classes to add to the img element.
		 *                                       Default null.
		 * @type bool $force_display Whether to always show the avatar - ignores the show_avatars option.
		 *                                       Default false.
		 * @type string $loading Value for the `loading` attribute.
		 *                                       Default null.
		 * @type string $fetchpriority Value for the `fetchpriority` attribute.
		 *                                       Default null.
		 * @type string $decoding Value for the `decoding` attribute.
		 *                                       Default null.
		 * @type string $extra_attr HTML attributes to insert in the IMG element. Is not sanitized.
		 *                                       Default empty.
		 * }
		 * @return string|false `<img>` tag for the user's avatar. False on failure.
		 * @since 2.5.0
		 * @since 4.2.0 Added the optional `$args` parameter.
		 * @since 5.5.0 Added the `loading` argument.
		 * @since 6.1.0 Added the `decoding` argument.
		 * @since 6.3.0 Added the `fetchpriority` argument.
		 *
		 */
		function get_avatar( $id_or_email, $size = 96, $default_value = '', $alt = '', $args = null ) {
			$defaults = array(
				// get_avatar_data() args.
				'size'          => 96,
				'height'        => null,
				'width'         => null,
				'default'       => get_option( 'avatar_default', 'mystery' ),
				'force_default' => false,
				'rating'        => get_option( 'avatar_rating' ),
				'scheme'        => null,
				'alt'           => '',
				'class'         => null,
				'force_display' => false,
				'loading'       => null,
				'fetchpriority' => null,
				'decoding'      => null,
				'extra_attr'    => '',
			);

			if ( empty( $args ) ) {
				$args = array();
			}

			$args[ 'size' ] = (int) $size;
			$args[ 'default' ] = $default_value;
			$args[ 'alt' ] = $alt;

			$args = wp_parse_args( $args, $defaults );

			if ( empty( $args[ 'height' ] ) ) {
				$args[ 'height' ] = $args[ 'size' ];
			}
			if ( empty( $args[ 'width' ] ) ) {
				$args[ 'width' ] = $args[ 'size' ];
			}

			// Update args with loading optimized attributes.
			$loading_optimization_attr = wp_get_loading_optimization_attributes( 'img', $args, 'get_avatar' );

			$args = array_merge( $args, $loading_optimization_attr );

			if ( is_object( $id_or_email ) && isset( $id_or_email->comment_ID ) ) {
				$id_or_email = get_comment( $id_or_email );
			}

			/**
			 * Allows the HTML for a user's avatar to be returned early.
			 *
			 * Returning a non-null value will effectively short-circuit get_avatar(), passing
			 * the value through the {@see 'get_avatar'} filter and returning early.
			 *
			 * @param string|null $avatar HTML for the user's avatar. Default null.
			 * @param mixed $id_or_email The avatar to retrieve. Accepts a user ID, Gravatar MD5 hash,
			 *                                 user email, WP_User object, WP_Post object, or WP_Comment object.
			 * @param array $args Arguments passed to get_avatar_url(), after processing.
			 * @since 4.2.0
			 *
			 */
			$avatar = apply_filters( 'pre_get_avatar', null, $id_or_email, $args );

			if ( !is_null( $avatar ) ) {
				/** This filter is documented in wp-includes/pluggable.php */
				return apply_filters( 'get_avatar', $avatar, $id_or_email, $args[ 'size' ], $args[ 'default' ], $args[ 'alt' ], $args );
			}

			if ( !$args[ 'force_display' ] && !get_option( 'show_avatars' ) ) {
				return false;
			}

			$url2x = get_avatar_url( $id_or_email, array_merge( $args, array( 'size' => $args[ 'size' ] * 2 ) ) );

			$args = get_avatar_data( $id_or_email, $args );

			$url = $args[ 'url' ];

			if ( !$url || is_wp_error( $url ) ) {
				return false;
			}

			$class = array(
				'avatar',
				'avatar-' . (int) $args[ 'size' ],
				'photo'
			);

			if ( !$args[ 'found_avatar' ] || $args[ 'force_default' ] ) {
				$class[] = 'avatar-default';
			}

			if ( $args[ 'class' ] ) {
				if ( is_array( $args[ 'class' ] ) ) {
					$class = array_merge( $class, $args[ 'class' ] );
				} else {
					$class[] = $args[ 'class' ];
				}
			}

			// Add `loading`, `fetchpriority`, and `decoding` attributes.
			$extra_attr = $args[ 'extra_attr' ];

			if ( in_array( $args[ 'loading' ], array(
					'lazy',
					'eager'
				), true ) && !preg_match( '/\bloading\s*=/', $extra_attr ) ) {
				if ( !empty( $extra_attr ) ) {
					$extra_attr .= ' ';
				}

				$extra_attr .= "loading='{$args['loading']}'";
			}

			if ( in_array( $args[ 'fetchpriority' ], array(
					'high',
					'low',
					'auto'
				), true ) && !preg_match( '/\bfetchpriority\s*=/', $extra_attr ) ) {
				if ( !empty( $extra_attr ) ) {
					$extra_attr .= ' ';
				}

				$extra_attr .= "fetchpriority='{$args['fetchpriority']}'";
			}

			if ( in_array( $args[ 'decoding' ], array(
					'async',
					'sync',
					'auto'
				), true ) && !preg_match( '/\bdecoding\s*=/', $extra_attr ) ) {
				if ( !empty( $extra_attr ) ) {
					$extra_attr .= ' ';
				}

				$extra_attr .= "decoding='{$args['decoding']}'";
			}

			$avatar = sprintf( "<img alt='%s' src='%s' srcset='%s' class='%s' height='%d' width='%d' %s/>", esc_attr( $args[ 'alt' ] ), esc_url( $url ), esc_url( $url2x ) . ' 2x', esc_attr( implode( ' ', $class ) ), (int) $args[ 'height' ], (int) $args[ 'width' ], $extra_attr );

			/**
			 * Filters the HTML for a user's avatar.
			 *
			 * @param string $avatar HTML for the user's avatar.
			 * @param mixed $id_or_email The avatar to retrieve. Accepts a user ID, Gravatar MD5 hash,
			 *                              user email, WP_User object, WP_Post object, or WP_Comment object.
			 * @param int $size Height and width of the avatar in pixels.
			 * @param string $default_value URL for the default image or a default type. Accepts:
			 *                              - '404' (return a 404 instead of a default image)
			 *                              - 'retro' (a 8-bit arcade-style pixelated face)
			 *                              - 'robohash' (a robot)
			 *                              - 'monsterid' (a monster)
			 *                              - 'wavatar' (a cartoon face)
			 *                              - 'identicon' (the "quilt", a geometric pattern)
			 *                              - 'mystery', 'mm', or 'mysteryman' (The Oyster Man)
			 *                              - 'blank' (transparent GIF)
			 *                              - 'gravatar_default' (the Gravatar logo)
			 * @param string $alt Alternative text to use in the avatar image tag.
			 * @param array $args Arguments passed to get_avatar_data(), after processing.
			 * @since 2.5.0
			 * @since 4.2.0 Added the `$args` parameter.
			 *
			 */
			return apply_filters( 'get_avatar', $avatar, $id_or_email, $args[ 'size' ], $args[ 'default' ], $args[ 'alt' ], $args );
		}
	} else {
		function get_avatar( $id_or_email, $size = 96, $default = '', $alt = '', $args = null ) {
			$defaults = array(
				// get_avatar_data() args.
				'size'          => 96,
				'height'        => null,
				'width'         => null,
				'default'       => get_option( 'avatar_default', 'mystery' ),
				'force_default' => false,
				'rating'        => get_option( 'avatar_rating' ),
				'scheme'        => null,
				'alt'           => '',
				'class'         => null,
				'force_display' => false,
				'loading'       => null,
				'extra_attr'    => '',
			);

			if ( wp_lazy_loading_enabled( 'img', 'get_avatar' ) ) {
				$defaults[ 'loading' ] = wp_get_loading_attr_default( 'get_avatar' );
			}

			if ( empty( $args ) ) {
				$args = array();
			}

			$args[ 'size' ] = (int) $size;
			$args[ 'default' ] = $default;
			$args[ 'alt' ] = $alt;

			$args = wp_parse_args( $args, $defaults );

			if ( empty( $args[ 'height' ] ) ) {
				$args[ 'height' ] = $args[ 'size' ];
			}
			if ( empty( $args[ 'width' ] ) ) {
				$args[ 'width' ] = $args[ 'size' ];
			}

			if ( is_object( $id_or_email ) && isset( $id_or_email->comment_ID ) ) {
				$id_or_email = get_comment( $id_or_email );
			}

			/**
			 * Allows the HTML for a user's avatar to be returned early.
			 *
			 * Returning a non-null value will effectively short-circuit get_avatar(), passing
			 * the value through the {@see 'get_avatar'} filter and returning early.
			 *
			 * @param string|null $avatar HTML for the user's avatar. Default null.
			 * @param mixed $id_or_email The avatar to retrieve. Accepts a user_id, Gravatar MD5 hash,
			 *                                 user email, WP_User object, WP_Post object, or WP_Comment object.
			 * @param array $args Arguments passed to get_avatar_url(), after processing.
			 * @since 4.2.0
			 *
			 */
			$avatar = apply_filters( 'pre_get_avatar', null, $id_or_email, $args );

			if ( !is_null( $avatar ) ) {
				/** This filter is documented in wp-includes/pluggable.php */
				return apply_filters( 'get_avatar', $avatar, $id_or_email, $args[ 'size' ], $args[ 'default' ], $args[ 'alt' ], $args );
			}

			if ( !$args[ 'force_display' ] && !get_option( 'show_avatars' ) ) {
				return false;
			}

			$url2x = get_avatar_url( $id_or_email, array_merge( $args, array( 'size' => $args[ 'size' ] * 2 ) ) );

			$args = get_avatar_data( $id_or_email, $args );

			$url = $args[ 'url' ];

			if ( !$url || is_wp_error( $url ) ) {
				return false;
			}

			$class = array(
				'avatar',
				'avatar-' . (int) $args[ 'size' ],
				'photo'
			);

			if ( !$args[ 'found_avatar' ] || $args[ 'force_default' ] ) {
				$class[] = 'avatar-default';
			}

			if ( $args[ 'class' ] ) {
				if ( is_array( $args[ 'class' ] ) ) {
					$class = array_merge( $class, $args[ 'class' ] );
				} else {
					$class[] = $args[ 'class' ];
				}
			}

			// Add `loading` attribute.
			$extra_attr = $args[ 'extra_attr' ];
			$loading = $args[ 'loading' ];

			if ( in_array( $loading, array(
					'lazy',
					'eager'
				), true ) && !preg_match( '/\bloading\s*=/', $extra_attr ) ) {
				if ( !empty( $extra_attr ) ) {
					$extra_attr .= ' ';
				}

				$extra_attr .= "loading='{$loading}'";
			}

			$avatar = sprintf( "<img alt='%s' src='%s' srcset='%s' class='%s' height='%d' width='%d' %s/>", esc_attr( $args[ 'alt' ] ), esc_url( $url ), esc_url( $url2x ) . ' 2x', esc_attr( implode( ' ', $class ) ), (int) $args[ 'height' ], (int) $args[ 'width' ], $extra_attr );

			/**
			 * Filters the HTML for a user's avatar.
			 *
			 * @param string $avatar HTML for the user's avatar.
			 * @param mixed $id_or_email The avatar to retrieve. Accepts a user_id, Gravatar MD5 hash,
			 *                            user email, WP_User object, WP_Post object, or WP_Comment object.
			 * @param int $size Square avatar width and height in pixels to retrieve.
			 * @param string $default URL for the default image or a default type. Accepts '404', 'retro', 'monsterid',
			 *                            'wavatar', 'indenticon', 'mystery', 'mm', 'mysteryman', 'blank', or 'gravatar_default'.
			 * @param string $alt Alternative text to use in the avatar image tag.
			 * @param array $args Arguments passed to get_avatar_data(), after processing.
			 * @since 2.5.0
			 * @since 4.2.0 The `$args` parameter was added.
			 *
			 */
			return apply_filters( 'get_avatar', $avatar, $id_or_email, $args[ 'size' ], $args[ 'default' ], $args[ 'alt' ], $args );
		}
	}
endif;


if ( !function_exists( 'wp_clear_auth_cookie' ) ) :
	/**
	 * Removes all of the cookies associated with authentication.
	 *
	 * @since 2.5.0
	 */
	function wp_clear_auth_cookie() {
		/**
		 * Fires just before the authentication cookies are cleared.
		 *
		 * @since 2.7.0
		 */
		do_action( 'clear_auth_cookie' );

		/** This filter is documented in wp-includes/pluggable.php */
		if ( !apply_filters( 'send_auth_cookies', true, 0, 0, 0, '', '' ) ) {
			return;
		}

		// Auth cookies.
		setcookie( AUTH_COOKIE, ' ', time() - YEAR_IN_SECONDS, ADMIN_COOKIE_PATH, COOKIE_DOMAIN, true, true );
		setcookie( SECURE_AUTH_COOKIE, ' ', time() - YEAR_IN_SECONDS, ADMIN_COOKIE_PATH, COOKIE_DOMAIN, true, true );
		setcookie( AUTH_COOKIE, ' ', time() - YEAR_IN_SECONDS, PLUGINS_COOKIE_PATH, COOKIE_DOMAIN, true, true );
		setcookie( SECURE_AUTH_COOKIE, ' ', time() - YEAR_IN_SECONDS, PLUGINS_COOKIE_PATH, COOKIE_DOMAIN, true, true );
		setcookie( LOGGED_IN_COOKIE, ' ', time() - YEAR_IN_SECONDS, COOKIEPATH, COOKIE_DOMAIN, true, true );
		setcookie( LOGGED_IN_COOKIE, ' ', time() - YEAR_IN_SECONDS, SITECOOKIEPATH, COOKIE_DOMAIN, true, true );

		// Settings cookies.
		setcookie( 'wp-settings-' . get_current_user_id(), ' ', time() - YEAR_IN_SECONDS, SITECOOKIEPATH, "", true, true );
		setcookie( 'wp-settings-time-' . get_current_user_id(), ' ', time() - YEAR_IN_SECONDS, SITECOOKIEPATH, "", true, true );

		// Old cookies.
		setcookie( AUTH_COOKIE, ' ', time() - YEAR_IN_SECONDS, COOKIEPATH, COOKIE_DOMAIN, true, true );
		setcookie( AUTH_COOKIE, ' ', time() - YEAR_IN_SECONDS, SITECOOKIEPATH, COOKIE_DOMAIN, true, true );
		setcookie( SECURE_AUTH_COOKIE, ' ', time() - YEAR_IN_SECONDS, COOKIEPATH, COOKIE_DOMAIN, true, true );
		setcookie( SECURE_AUTH_COOKIE, ' ', time() - YEAR_IN_SECONDS, SITECOOKIEPATH, COOKIE_DOMAIN, true, true );

		// Even older cookies.
		setcookie( USER_COOKIE, ' ', time() - YEAR_IN_SECONDS, COOKIEPATH, COOKIE_DOMAIN, true, true );
		setcookie( PASS_COOKIE, ' ', time() - YEAR_IN_SECONDS, COOKIEPATH, COOKIE_DOMAIN, true, true );
		setcookie( USER_COOKIE, ' ', time() - YEAR_IN_SECONDS, SITECOOKIEPATH, COOKIE_DOMAIN, true, true );
		setcookie( PASS_COOKIE, ' ', time() - YEAR_IN_SECONDS, SITECOOKIEPATH, COOKIE_DOMAIN, true, true );

		// Post password cookie.
		setcookie( 'wp-postpass_' . COOKIEHASH, ' ', time() - YEAR_IN_SECONDS, COOKIEPATH, COOKIE_DOMAIN, true, true );
	}
endif;

if ( !function_exists( 'wp_get_loading_attr_default' ) ) :
	function wp_get_loading_attr_default( $context ) {
		_deprecated_function( __FUNCTION__, '6.3.0', 'wp_get_loading_optimization_attributes()' );
		global $wp_query;

		// Skip lazy-loading for the overall block template, as it is handled more granularly.
		if ( 'template' === $context ) {
			return false;
		}

		/*
		 * Do not lazy-load images in the header block template part, as they are likely above the fold.
		 * For classic themes, this is handled in the condition below using the 'get_header' action.
		 */
		$header_area = WP_TEMPLATE_PART_AREA_HEADER;
		if ( "template_part_{$header_area}" === $context ) {
			return false;
		}

		// Special handling for programmatically created image tags.
		if ( 'the_post_thumbnail' === $context || 'wp_get_attachment_image' === $context ) {
			/*
			 * Skip programmatically created images within post content as they need to be handled together with the other
			 * images within the post content.
			 * Without this clause, they would already be counted below which skews the number and can result in the first
			 * post content image being lazy-loaded only because there are images elsewhere in the post content.
			 */
			if ( doing_filter( 'the_content' ) ) {
				return false;
			}

			// Conditionally skip lazy-loading on images before the loop.
			if ( // Only apply for main query but before the loop.
				$wp_query->before_loop && $wp_query->is_main_query() /*
				 * Any image before the loop, but after the header has started should not be lazy-loaded,
				 * except when the footer has already started which can happen when the current template
				 * does not include any loop.
				 */ && did_action( 'get_header' ) && !did_action( 'get_footer' ) ) {
				return false;
			}
		}

		/*
		 * The first elements in 'the_content' or 'the_post_thumbnail' should not be lazy-loaded,
		 * as they are likely above the fold.
		 */
		if ( 'the_content' === $context || 'the_post_thumbnail' === $context ) {
			// Only elements within the main query loop have special handling.
			if ( is_admin() || !in_the_loop() || !is_main_query() ) {
				return 'lazy';
			}

			// Increase the counter since this is a main query content element.
			$content_media_count = wp_increase_content_media_count();

			// If the count so far is below the threshold, return `false` so that the `loading` attribute is omitted.
			if ( $content_media_count <= wp_omit_loading_attr_threshold() ) {
				return false;
			}

			// For elements after the threshold, lazy-load them as usual.
			return 'lazy';
		}

		// Lazy-load by default for any unknown context.
		return 'lazy';
	}
endif;

if ( !function_exists( 'wp_increase_content_media_count' ) ) :
	/**
	 * Increases an internal content media count variable.
	 *
	 * @param int $amount Optional. Amount to increase by. Default 1.
	 * @return int The latest content media count, after the increase.
	 * @since 5.9.0
	 * @access private
	 *
	 */
	function wp_increase_content_media_count( $amount = 1 ) {
		static $content_media_count = 0;

		$content_media_count += $amount;

		return $content_media_count;
	}
endif;

if ( !function_exists( 'wp_omit_loading_attr_threshold' ) ) :
	/**
	 * Gets the threshold for how many of the first content media elements to not lazy-load.
	 *
	 * This function runs the {@see 'wp_omit_loading_attr_threshold'} filter, which uses a default threshold value of 3.
	 * The filter is only run once per page load, unless the `$force` parameter is used.
	 *
	 * @param bool $force Optional. If set to true, the filter will be (re-)applied even if it already has been before.
	 *                    Default false.
	 * @return int The number of content media elements to not lazy-load.
	 * @since 5.9.0
	 *
	 */
	function wp_omit_loading_attr_threshold( $force = false ) {
		static $omit_threshold;

		// This function may be called multiple times. Run the filter only once per page load.
		if ( !isset( $omit_threshold ) || $force ) {
			/**
			 * Filters the threshold for how many of the first content media elements to not lazy-load.
			 *
			 * For these first content media elements, the `loading` attribute will be omitted. By default, this is the case
			 * for only the very first content media element.
			 *
			 * @param int $omit_threshold The number of media elements where the `loading` attribute will not be added. Default 3.
			 * @since 6.3.0 The default threshold was changed from 1 to 3.
			 *
			 * @since 5.9.0
			 */
			$omit_threshold = apply_filters( 'wp_omit_loading_attr_threshold', 3 );
		}

		return $omit_threshold;
	}
endif;

if ( ! function_exists( 'is_user_logged_in' ) ) :
	/**
	 * Determines whether the current visitor is a logged in user.
	 *
	 * For more information on this and similar theme functions, check out
	 * the {@link https://developer.wordpress.org/themes/basics/conditional-tags/
	 * Conditional Tags} article in the Theme Developer Handbook.
	 *
	 * @since 2.0.0
	 *
	 * @return bool True if user is logged in, false if not logged in.
	 */
	function is_user_logged_in() {
		$user = wp_get_current_user();

		return $user->exists();
	}
endif;