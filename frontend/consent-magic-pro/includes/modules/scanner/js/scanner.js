( function ( $ ) {
    'use strict';

    const CS_scanner = {
        continue_scan: 1,
        abortingScan: 0,
        onPrg: 0,
        loader: null,
        progressBar: null,
        Set: function () {
            jQuery( document ).on( 'click', '.cs_cookies_scan', function () {
                CS_scanner.loader = $( this ).closest( '.run-scan-button-wrap' );
                CS_scanner.progressBar = $( this ).closest( '.run-scan-wrap' ).find( '.cm-scan-bar' );
                CS_scanner.loader.addClass( 'loading' );
                CS_scanner.progressBar.html( 'Scanned 0 pages' ).slideDown( 400 );
                CS_scanner.continue_scan = 1;
                CS_scanner.scanNow();
            } );
        },
        scanNow: function () {
            this.takePages();
        },
        takePages: function () {
            let data = {
                action: 'cs_scanner',
                security: cs_scanner.nonces.cs_scanner,
                cs_scanner_action: 'cs_get_pages',
            };
            $.ajax( {
                url: cs_scanner.ajax_url,
                data: data,
                dataType: 'json',
                type: 'POST',
                success: function ( data ) {
                    CS_scanner.scan_id = typeof data.scan_id != 'undefined' ? data.scan_id : 0;
                    if ( true === data.status ) {
                        CS_scanner.bulkScan( data.scan_id, data.total );
                        CS_scanner.progressBar.html( 'Scanned 0 of ' + data.total + ' pages' )
                    } else {
                        CS_scanner.loader.removeClass( 'loading' );
                    }
                },
                error: function () {
                    console.log( 'error ' );
                    CS_scanner.progressBar.html( 'Something went wrong' );
                }
            } );
        },
        bulkScan: function ( scan_id, total, page_offset = 0 ) {

            let data = {
                action: 'cs_scanner',
                security: cs_scanner.nonces.cs_scanner,
                cs_scanner_action: 'cs_bulk_scan',
                scan_id: scan_id,
                total: total,
                page_offset: page_offset
            };

            $.ajax( {
                url: cs_scanner.ajax_url,
                data: data,
                dataType: 'json',
                type: 'POST',
                success: function ( response ) {
                    console.log( 'response.data ', response.data );
                    if ( response.success === true ) {
                        if ( response.data.page_offset !== 'false' ) {
                            CS_scanner.bulkScan( response.data.scan_id, response.data.total, response.data.page_offset );
                            CS_scanner.progressBar.html( 'Scanned ' + response.data.page_offset + ' of ' + response.data.total + ' pages' )
                        } else {
                            location.reload();
                        }
                    }
                },
                error: function () {
                    console.log( 'error ' );
                    CS_scanner.progressBar.html( 'Something went wrong' );
                }
            } );
        }
    };

    jQuery( document ).ready( function () {
        CS_scanner.Set();

        $( '#cs_auto_scan_type' ).on( 'change', function () {
            let val = $( this ).val(),
                data = {
                    action: 'cs_update_scan_page',
                    security: cs_scanner.nonces.cs_scanner,
                    cs_scan_existing_page: val,
                };
            $.ajax( {
                url: cs_scanner.ajax_url,
                data: data,
                dataType: 'json',
                type: 'POST',
                success: function ( data ) {

                },
                error: function () {
                    console.log( 'error ' );
                }
            } );
            console.log( 'val ', val );
        } );
    } );
} )( jQuery );