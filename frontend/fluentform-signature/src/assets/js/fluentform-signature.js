// jQuery('document.body').on('fluentform_init', function (e, $theForm, form) {
//     console.log($theForm)
// });

jQuery(document).ready(function ($) {
    window.setTimeout(function () {
        var form = $('body').find('form');
        // The canvas and signature pad store. We'll use this to maintain the state.
        let store = {
            canvases: [],
            signaturePads: [],
            signatures: []
        };

        window.store = store;

        // Initialize the signature pad, store & register events.

        function initSignature(canvas, iteration) {
            let canvasWidth = $(canvas).closest('fieldset').innerWidth();
            const isPreviewForm = $(canvas).closest('.ff_form_preview').length;
            let screenType = $('.ff_device_control.active').data('type');

            if (isPreviewForm) {
                screenType = screenType || window.localStorage.getItem('ff_window_type');
                if (screenType === 'monitor') {
                    canvasWidth = 955;
                } else if (screenType === 'mobile') {
                    canvasWidth = 335;
                } else if (screenType === 'tablet') {
                    canvasWidth = 728;
                }
                if (screenType === 'monitor' && jQuery('.ff_preview_body').hasClass('ff_preview_only')) {
                    canvasWidth = window.innerWidth - 100;
                }
            }

            const cellCount = parseInt($(canvas).closest('.ff-t-container').children('.ff-t-cell').length);
            if (cellCount > 0) {
                const containerCell = $(canvas).closest('.ff-t-cell');
                let flexBasis = parseInt(containerCell.css('flex-basis'), 10) / 100;
                if (
                    (isPreviewForm && screenType === 'mobile') ||
                    (!isPreviewForm && window.innerWidth <= 425)
                ) {
                    isAddFlexWrap(canvas);
                } else if (
                    (isPreviewForm && (screenType === 'monitor' || screenType === 'tablet')) ||
                    (!isPreviewForm && window.innerWidth >= 768)
                ) {
                    canvasWidth = setContainerCellCanvasWidth(canvas, canvasWidth, flexBasis);
                    canvasWidth -= cellCount*7.5;
                }
            }

            $(canvas).attr('width', canvasWidth);

            let signaturePad = new SignaturePad(canvas, {
                // The stroke end event to populate the
                // data URI and pushing it to the input.
                onEnd: function() {
                    setDataURI(this._canvas.id, this.toDataURL());
                },
                penColor: canvas.dataset.penColor,
                minWidth: Math.abs(canvas.dataset.penSize) / 10
            });

            store.canvases.push(canvas);

            store.signaturePads.push(signaturePad);

            $signaturePadActions = $("#" + canvas.id).siblings(":last");

            // Register clear signature event.
            $signaturePadActions
                .find(".fluentform-signature-clear")
                .on("click", function() {
                    store.signatures[iteration] = signaturePad.toData();

                    signaturePad.clear();

                    setDataURI(canvas.id, "");
                });

            // Register undo signature event.
            $signaturePadActions
                .find(".fluentform-signature-undo")
                .on("click", function() {
                    let data = signaturePad.toData();

                    if (data && data.length) {
                        let pop = data.pop(); // remove the last dot or line

                        // Store the popped out data to allow the user to redo the signature.
                        // We should use the current iteration to maintain the store's data.
                        if (!store.signatures[iteration]) {
                            store.signatures[iteration] = [];
                        }

                        store.signatures[iteration].push(pop);

                        signaturePad.fromData(data);
                    }
                });

            // Register redo signature event.
            $signaturePadActions
                .find(".fluentform-signature-redo")
                .on("click", function () {
                    let redos = store.signatures[iteration];

                    if (redos && redos.length) {
                        let data = signaturePad.toData();

                        data.push(redos.pop());

                        signaturePad.fromData(data);
                    }
                });
        }

        function isAddFlexWrap(canvas) {
            $(canvas).parents('.ff-t-container').css('flex-wrap', 'wrap');
        }

        function setContainerCellCanvasWidth(canvas, canvasWidth, flexBasis) {
            $(canvas).parents('.ff-t-container').css('flex-wrap', 'nowrap');
            return parseInt(canvasWidth * flexBasis, 10);
        }

        // Loop through each of the signature fields and initialize the signature pad.
        $.each($(".fluentform-signature-pad"), function (index, canvas) {
            initSignature(canvas, index);
            form.on('screen-change', function (e, width) {
                initSignature(canvas, index);
            })
            
            form.on('fluentform-preview-mode-change', function(e, status) {
                initSignature(canvas, index);
            })

            jQuery(window).on("orientationchange resize", function(e) {
                initSignature(canvas, index);
            });

            // Clear signature pads when fluentform asks to reset.
            $("#fluentform_" + canvas.dataset.formId).on("reset", function () {
                store.signaturePads[index].clear();

                setDataURI(canvas.id, "");
            });
        });

        /**
         * To resize the canvas when needed.
         */
        function resizeCanvas() {
            $.each(store.canvases, function (i, canvas) {
                let $parent = $(this).parent();

                store.signaturePads[i].clear();

                setDataURI(canvas.id, "");
            });
        }

        /**
         * Set the data URI.
         * @param String canvasId
         * @param String value
         */
        function setDataURI(canvasId, value) {
            $("#" + canvasId).parent().siblings().first().val(value);
        }

        resizeCanvas();
    }, 10);
});
