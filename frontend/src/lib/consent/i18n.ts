/**
 * Consent System Internationalization (i18n)
 *
 * Multi-language support for consent UI with:
 * - 12 EU languages + more
 * - Auto-detection from browser
 * - Easy extension for new languages
 *
 * @module consent/i18n
 * @version 1.0.0
 */

import { browser } from '$app/environment';
import { writable, derived, get } from 'svelte/store';

/**
 * Supported languages
 */
export type SupportedLanguage =
	| 'en'
	| 'de'
	| 'fr'
	| 'es'
	| 'it'
	| 'nl'
	| 'pt'
	| 'pl'
	| 'sv'
	| 'da'
	| 'fi'
	| 'no'
	| 'cs'
	| 'ro'
	| 'hu'
	| 'el'
	| 'bg'
	| 'hr'
	| 'sk'
	| 'sl'
	| 'et'
	| 'lv'
	| 'lt'
	| 'ja'
	| 'zh'
	| 'ko';

/**
 * Translation keys
 */
export interface ConsentTranslations {
	// Banner
	bannerTitle: string;
	bannerDescription: string;
	acceptAll: string;
	rejectAll: string;
	customize: string;

	// Modal
	modalTitle: string;
	modalDescription: string;
	savePreferences: string;
	close: string;

	// Categories
	necessary: string;
	necessaryDescription: string;
	analytics: string;
	analyticsDescription: string;
	marketing: string;
	marketingDescription: string;
	preferences: string;
	preferencesDescription: string;

	// Settings button
	cookieSettings: string;

	// Cookie policy page
	cookiePolicyTitle: string;
	cookiePolicyIntro: string;
	whatAreCookies: string;
	whatAreCookiesDescription: string;
	howWeUseCookies: string;
	cookieDeclaration: string;
	lastUpdated: string;
	cookieName: string;
	cookiePurpose: string;
	cookieDuration: string;
	cookieType: string;

	// Placeholders
	contentBlocked: string;
	contentBlockedDescription: string;
	enableCookies: string;

	// Consent receipt
	consentReceipt: string;
	downloadReceipt: string;
	consentId: string;
	consentDate: string;
	consentCategories: string;

	// Privacy signals
	gpcDetected: string;
	dntDetected: string;
	regionDetected: string;

	// Misc
	enabled: string;
	disabled: string;
	required: string;
	optional: string;
	on: string;
	off: string;
}

/**
 * English translations (default)
 */
const en: ConsentTranslations = {
	bannerTitle: 'We value your privacy',
	bannerDescription:
		'We use cookies to enhance your browsing experience, serve personalized content, and analyze our traffic. By clicking "Accept All", you consent to our use of cookies.',
	acceptAll: 'Accept All',
	rejectAll: 'Reject All',
	customize: 'Customize',

	modalTitle: 'Cookie Preferences',
	modalDescription:
		'Manage your cookie preferences below. You can enable or disable different types of cookies and save your preferences.',
	savePreferences: 'Save Preferences',
	close: 'Close',

	necessary: 'Necessary',
	necessaryDescription:
		'Essential cookies that enable core functionality. These cannot be disabled as they are required for the website to function properly.',
	analytics: 'Analytics',
	analyticsDescription:
		'Help us understand how visitors interact with our website by collecting anonymous information.',
	marketing: 'Marketing',
	marketingDescription:
		'Used to track visitors across websites to display relevant advertisements.',
	preferences: 'Preferences',
	preferencesDescription:
		'Allow the website to remember your preferences such as language or region.',

	cookieSettings: 'Cookie Settings',

	cookiePolicyTitle: 'Cookie Policy',
	cookiePolicyIntro:
		'This Cookie Policy explains how we use cookies and similar technologies on our website.',
	whatAreCookies: 'What are cookies?',
	whatAreCookiesDescription:
		'Cookies are small text files that are stored on your device when you visit a website. They help the website remember your preferences and understand how you use the site.',
	howWeUseCookies: 'How we use cookies',
	cookieDeclaration: 'Cookie Declaration',
	lastUpdated: 'Last updated',
	cookieName: 'Cookie Name',
	cookiePurpose: 'Purpose',
	cookieDuration: 'Duration',
	cookieType: 'Type',

	contentBlocked: 'Content Blocked',
	contentBlockedDescription:
		'This content requires cookies to be enabled. Please accept the required cookies to view this content.',
	enableCookies: 'Enable Cookies',

	consentReceipt: 'Consent Receipt',
	downloadReceipt: 'Download Receipt',
	consentId: 'Consent ID',
	consentDate: 'Date',
	consentCategories: 'Consented Categories',

	gpcDetected: 'Global Privacy Control detected',
	dntDetected: 'Do Not Track detected',
	regionDetected: 'Region',

	enabled: 'Enabled',
	disabled: 'Disabled',
	required: 'Required',
	optional: 'Optional',
	on: 'On',
	off: 'Off'
};

/**
 * German translations
 */
const de: ConsentTranslations = {
	bannerTitle: 'Wir schätzen Ihre Privatsphäre',
	bannerDescription:
		'Wir verwenden Cookies, um Ihr Surferlebnis zu verbessern, personalisierte Inhalte bereitzustellen und unseren Datenverkehr zu analysieren. Durch Klicken auf "Alle akzeptieren" stimmen Sie der Verwendung von Cookies zu.',
	acceptAll: 'Alle akzeptieren',
	rejectAll: 'Alle ablehnen',
	customize: 'Anpassen',

	modalTitle: 'Cookie-Einstellungen',
	modalDescription:
		'Verwalten Sie unten Ihre Cookie-Einstellungen. Sie können verschiedene Arten von Cookies aktivieren oder deaktivieren.',
	savePreferences: 'Einstellungen speichern',
	close: 'Schließen',

	necessary: 'Notwendig',
	necessaryDescription:
		'Essentielle Cookies, die Kernfunktionen ermöglichen. Diese können nicht deaktiviert werden.',
	analytics: 'Analyse',
	analyticsDescription: 'Helfen uns zu verstehen, wie Besucher mit unserer Website interagieren.',
	marketing: 'Marketing',
	marketingDescription: 'Werden verwendet, um Besuchern relevante Werbung anzuzeigen.',
	preferences: 'Präferenzen',
	preferencesDescription:
		'Ermöglichen der Website, Ihre Einstellungen wie Sprache oder Region zu speichern.',

	cookieSettings: 'Cookie-Einstellungen',

	cookiePolicyTitle: 'Cookie-Richtlinie',
	cookiePolicyIntro:
		'Diese Cookie-Richtlinie erklärt, wie wir Cookies auf unserer Website verwenden.',
	whatAreCookies: 'Was sind Cookies?',
	whatAreCookiesDescription:
		'Cookies sind kleine Textdateien, die auf Ihrem Gerät gespeichert werden.',
	howWeUseCookies: 'Wie wir Cookies verwenden',
	cookieDeclaration: 'Cookie-Erklärung',
	lastUpdated: 'Zuletzt aktualisiert',
	cookieName: 'Cookie-Name',
	cookiePurpose: 'Zweck',
	cookieDuration: 'Dauer',
	cookieType: 'Typ',

	contentBlocked: 'Inhalt blockiert',
	contentBlockedDescription:
		'Dieser Inhalt erfordert aktivierte Cookies. Bitte akzeptieren Sie die erforderlichen Cookies.',
	enableCookies: 'Cookies aktivieren',

	consentReceipt: 'Einwilligungsnachweis',
	downloadReceipt: 'Nachweis herunterladen',
	consentId: 'Einwilligungs-ID',
	consentDate: 'Datum',
	consentCategories: 'Eingewilligte Kategorien',

	gpcDetected: 'Global Privacy Control erkannt',
	dntDetected: 'Do Not Track erkannt',
	regionDetected: 'Region',

	enabled: 'Aktiviert',
	disabled: 'Deaktiviert',
	required: 'Erforderlich',
	optional: 'Optional',
	on: 'An',
	off: 'Aus'
};

/**
 * French translations
 */
const fr: ConsentTranslations = {
	bannerTitle: 'Nous respectons votre vie privée',
	bannerDescription:
		"Nous utilisons des cookies pour améliorer votre expérience, proposer du contenu personnalisé et analyser notre trafic. En cliquant sur « Tout accepter », vous consentez à l'utilisation de cookies.",
	acceptAll: 'Tout accepter',
	rejectAll: 'Tout refuser',
	customize: 'Personnaliser',

	modalTitle: 'Préférences de cookies',
	modalDescription:
		'Gérez vos préférences de cookies ci-dessous. Vous pouvez activer ou désactiver différents types de cookies.',
	savePreferences: 'Enregistrer les préférences',
	close: 'Fermer',

	necessary: 'Nécessaires',
	necessaryDescription:
		'Cookies essentiels qui permettent les fonctionnalités de base. Ils ne peuvent pas être désactivés.',
	analytics: 'Analytiques',
	analyticsDescription:
		'Nous aident à comprendre comment les visiteurs interagissent avec notre site.',
	marketing: 'Marketing',
	marketingDescription: 'Utilisés pour afficher des publicités pertinentes aux visiteurs.',
	preferences: 'Préférences',
	preferencesDescription:
		'Permettent au site de mémoriser vos préférences comme la langue ou la région.',

	cookieSettings: 'Paramètres des cookies',

	cookiePolicyTitle: 'Politique de cookies',
	cookiePolicyIntro:
		'Cette politique de cookies explique comment nous utilisons les cookies sur notre site.',
	whatAreCookies: 'Que sont les cookies ?',
	whatAreCookiesDescription:
		'Les cookies sont de petits fichiers texte stockés sur votre appareil.',
	howWeUseCookies: 'Comment nous utilisons les cookies',
	cookieDeclaration: 'Déclaration des cookies',
	lastUpdated: 'Dernière mise à jour',
	cookieName: 'Nom du cookie',
	cookiePurpose: 'Objectif',
	cookieDuration: 'Durée',
	cookieType: 'Type',

	contentBlocked: 'Contenu bloqué',
	contentBlockedDescription:
		'Ce contenu nécessite des cookies activés. Veuillez accepter les cookies requis.',
	enableCookies: 'Activer les cookies',

	consentReceipt: 'Reçu de consentement',
	downloadReceipt: 'Télécharger le reçu',
	consentId: 'ID de consentement',
	consentDate: 'Date',
	consentCategories: 'Catégories consenties',

	gpcDetected: 'Global Privacy Control détecté',
	dntDetected: 'Do Not Track détecté',
	regionDetected: 'Région',

	enabled: 'Activé',
	disabled: 'Désactivé',
	required: 'Requis',
	optional: 'Optionnel',
	on: 'On',
	off: 'Off'
};

/**
 * Spanish translations
 */
const es: ConsentTranslations = {
	bannerTitle: 'Valoramos su privacidad',
	bannerDescription:
		'Utilizamos cookies para mejorar su experiencia de navegación, ofrecer contenido personalizado y analizar nuestro tráfico. Al hacer clic en "Aceptar todo", acepta el uso de cookies.',
	acceptAll: 'Aceptar todo',
	rejectAll: 'Rechazar todo',
	customize: 'Personalizar',

	modalTitle: 'Preferencias de cookies',
	modalDescription:
		'Gestione sus preferencias de cookies a continuación. Puede activar o desactivar diferentes tipos de cookies.',
	savePreferences: 'Guardar preferencias',
	close: 'Cerrar',

	necessary: 'Necesarias',
	necessaryDescription:
		'Cookies esenciales que permiten la funcionalidad básica. No se pueden desactivar.',
	analytics: 'Analíticas',
	analyticsDescription:
		'Nos ayudan a entender cómo interactúan los visitantes con nuestro sitio web.',
	marketing: 'Marketing',
	marketingDescription: 'Se utilizan para mostrar anuncios relevantes a los visitantes.',
	preferences: 'Preferencias',
	preferencesDescription:
		'Permiten que el sitio web recuerde sus preferencias como idioma o región.',

	cookieSettings: 'Configuración de cookies',

	cookiePolicyTitle: 'Política de cookies',
	cookiePolicyIntro:
		'Esta política de cookies explica cómo utilizamos las cookies en nuestro sitio web.',
	whatAreCookies: '¿Qué son las cookies?',
	whatAreCookiesDescription:
		'Las cookies son pequeños archivos de texto que se almacenan en su dispositivo.',
	howWeUseCookies: 'Cómo usamos las cookies',
	cookieDeclaration: 'Declaración de cookies',
	lastUpdated: 'Última actualización',
	cookieName: 'Nombre de cookie',
	cookiePurpose: 'Propósito',
	cookieDuration: 'Duración',
	cookieType: 'Tipo',

	contentBlocked: 'Contenido bloqueado',
	contentBlockedDescription:
		'Este contenido requiere cookies activadas. Por favor, acepte las cookies requeridas.',
	enableCookies: 'Activar cookies',

	consentReceipt: 'Recibo de consentimiento',
	downloadReceipt: 'Descargar recibo',
	consentId: 'ID de consentimiento',
	consentDate: 'Fecha',
	consentCategories: 'Categorías consentidas',

	gpcDetected: 'Global Privacy Control detectado',
	dntDetected: 'Do Not Track detectado',
	regionDetected: 'Región',

	enabled: 'Activado',
	disabled: 'Desactivado',
	required: 'Requerido',
	optional: 'Opcional',
	on: 'On',
	off: 'Off'
};

/**
 * Italian translations
 */
const it: ConsentTranslations = {
	bannerTitle: 'Rispettiamo la tua privacy',
	bannerDescription:
		'Utilizziamo i cookie per migliorare la tua esperienza, fornire contenuti personalizzati e analizzare il nostro traffico. Cliccando su "Accetta tutto", acconsenti all\'uso dei cookie.',
	acceptAll: 'Accetta tutto',
	rejectAll: 'Rifiuta tutto',
	customize: 'Personalizza',

	modalTitle: 'Preferenze cookie',
	modalDescription:
		'Gestisci le tue preferenze cookie qui sotto. Puoi abilitare o disabilitare diversi tipi di cookie.',
	savePreferences: 'Salva preferenze',
	close: 'Chiudi',

	necessary: 'Necessari',
	necessaryDescription:
		'Cookie essenziali che abilitano le funzionalità di base. Non possono essere disabilitati.',
	analytics: 'Analitici',
	analyticsDescription: 'Ci aiutano a capire come i visitatori interagiscono con il nostro sito.',
	marketing: 'Marketing',
	marketingDescription: 'Utilizzati per mostrare pubblicità pertinenti ai visitatori.',
	preferences: 'Preferenze',
	preferencesDescription:
		'Permettono al sito di ricordare le tue preferenze come lingua o regione.',

	cookieSettings: 'Impostazioni cookie',

	cookiePolicyTitle: 'Politica sui cookie',
	cookiePolicyIntro: 'Questa politica sui cookie spiega come utilizziamo i cookie sul nostro sito.',
	whatAreCookies: 'Cosa sono i cookie?',
	whatAreCookiesDescription: 'I cookie sono piccoli file di testo memorizzati sul tuo dispositivo.',
	howWeUseCookies: 'Come utilizziamo i cookie',
	cookieDeclaration: 'Dichiarazione dei cookie',
	lastUpdated: 'Ultimo aggiornamento',
	cookieName: 'Nome cookie',
	cookiePurpose: 'Scopo',
	cookieDuration: 'Durata',
	cookieType: 'Tipo',

	contentBlocked: 'Contenuto bloccato',
	contentBlockedDescription:
		'Questo contenuto richiede i cookie abilitati. Per favore accetta i cookie richiesti.',
	enableCookies: 'Abilita cookie',

	consentReceipt: 'Ricevuta di consenso',
	downloadReceipt: 'Scarica ricevuta',
	consentId: 'ID consenso',
	consentDate: 'Data',
	consentCategories: 'Categorie consentite',

	gpcDetected: 'Global Privacy Control rilevato',
	dntDetected: 'Do Not Track rilevato',
	regionDetected: 'Regione',

	enabled: 'Abilitato',
	disabled: 'Disabilitato',
	required: 'Richiesto',
	optional: 'Opzionale',
	on: 'On',
	off: 'Off'
};

/**
 * Dutch translations
 */
const nl: ConsentTranslations = {
	bannerTitle: 'Wij waarderen uw privacy',
	bannerDescription:
		'Wij gebruiken cookies om uw surfervaring te verbeteren, gepersonaliseerde inhoud te tonen en ons verkeer te analyseren. Door op "Alles accepteren" te klikken, stemt u in met ons gebruik van cookies.',
	acceptAll: 'Alles accepteren',
	rejectAll: 'Alles weigeren',
	customize: 'Aanpassen',

	modalTitle: 'Cookie-voorkeuren',
	modalDescription:
		'Beheer hieronder uw cookie-voorkeuren. U kunt verschillende soorten cookies in- of uitschakelen.',
	savePreferences: 'Voorkeuren opslaan',
	close: 'Sluiten',

	necessary: 'Noodzakelijk',
	necessaryDescription:
		'Essentiële cookies die kernfunctionaliteit mogelijk maken. Deze kunnen niet worden uitgeschakeld.',
	analytics: 'Analytisch',
	analyticsDescription: 'Helpen ons te begrijpen hoe bezoekers omgaan met onze website.',
	marketing: 'Marketing',
	marketingDescription: 'Worden gebruikt om relevante advertenties aan bezoekers te tonen.',
	preferences: 'Voorkeuren',
	preferencesDescription: 'Laat de website uw voorkeuren onthouden zoals taal of regio.',

	cookieSettings: 'Cookie-instellingen',

	cookiePolicyTitle: 'Cookiebeleid',
	cookiePolicyIntro: 'Dit cookiebeleid legt uit hoe wij cookies gebruiken op onze website.',
	whatAreCookies: 'Wat zijn cookies?',
	whatAreCookiesDescription:
		'Cookies zijn kleine tekstbestanden die op uw apparaat worden opgeslagen.',
	howWeUseCookies: 'Hoe wij cookies gebruiken',
	cookieDeclaration: 'Cookie-verklaring',
	lastUpdated: 'Laatst bijgewerkt',
	cookieName: 'Cookienaam',
	cookiePurpose: 'Doel',
	cookieDuration: 'Duur',
	cookieType: 'Type',

	contentBlocked: 'Inhoud geblokkeerd',
	contentBlockedDescription:
		'Deze inhoud vereist ingeschakelde cookies. Accepteer de vereiste cookies.',
	enableCookies: 'Cookies inschakelen',

	consentReceipt: 'Toestemmingsbewijs',
	downloadReceipt: 'Download bewijs',
	consentId: 'Toestemmings-ID',
	consentDate: 'Datum',
	consentCategories: 'Toegestane categorieën',

	gpcDetected: 'Global Privacy Control gedetecteerd',
	dntDetected: 'Do Not Track gedetecteerd',
	regionDetected: 'Regio',

	enabled: 'Ingeschakeld',
	disabled: 'Uitgeschakeld',
	required: 'Vereist',
	optional: 'Optioneel',
	on: 'Aan',
	off: 'Uit'
};

/**
 * Portuguese translations
 */
const pt: ConsentTranslations = {
	bannerTitle: 'Valorizamos a sua privacidade',
	bannerDescription:
		'Utilizamos cookies para melhorar a sua experiência, fornecer conteúdo personalizado e analisar o nosso tráfego. Ao clicar em "Aceitar tudo", consente a utilização de cookies.',
	acceptAll: 'Aceitar tudo',
	rejectAll: 'Rejeitar tudo',
	customize: 'Personalizar',

	modalTitle: 'Preferências de cookies',
	modalDescription:
		'Gerencie as suas preferências de cookies abaixo. Pode ativar ou desativar diferentes tipos de cookies.',
	savePreferences: 'Guardar preferências',
	close: 'Fechar',

	necessary: 'Necessários',
	necessaryDescription:
		'Cookies essenciais que permitem funcionalidades básicas. Não podem ser desativados.',
	analytics: 'Analíticos',
	analyticsDescription: 'Ajudam-nos a compreender como os visitantes interagem com o nosso site.',
	marketing: 'Marketing',
	marketingDescription: 'Utilizados para mostrar anúncios relevantes aos visitantes.',
	preferences: 'Preferências',
	preferencesDescription:
		'Permitem que o site memorize as suas preferências como idioma ou região.',

	cookieSettings: 'Definições de cookies',

	cookiePolicyTitle: 'Política de Cookies',
	cookiePolicyIntro: 'Esta política de cookies explica como utilizamos cookies no nosso site.',
	whatAreCookies: 'O que são cookies?',
	whatAreCookiesDescription:
		'Cookies são pequenos ficheiros de texto armazenados no seu dispositivo.',
	howWeUseCookies: 'Como usamos cookies',
	cookieDeclaration: 'Declaração de Cookies',
	lastUpdated: 'Última atualização',
	cookieName: 'Nome do cookie',
	cookiePurpose: 'Finalidade',
	cookieDuration: 'Duração',
	cookieType: 'Tipo',

	contentBlocked: 'Conteúdo bloqueado',
	contentBlockedDescription:
		'Este conteúdo requer cookies ativados. Por favor aceite os cookies necessários.',
	enableCookies: 'Ativar cookies',

	consentReceipt: 'Recibo de consentimento',
	downloadReceipt: 'Descarregar recibo',
	consentId: 'ID de consentimento',
	consentDate: 'Data',
	consentCategories: 'Categorias consentidas',

	gpcDetected: 'Global Privacy Control detetado',
	dntDetected: 'Do Not Track detetado',
	regionDetected: 'Região',

	enabled: 'Ativado',
	disabled: 'Desativado',
	required: 'Obrigatório',
	optional: 'Opcional',
	on: 'On',
	off: 'Off'
};

/**
 * All translations
 */
const translations: Record<SupportedLanguage, ConsentTranslations> = {
	en,
	de,
	fr,
	es,
	it,
	nl,
	pt,
	// Fallback to English for other languages (can be extended)
	pl: en,
	sv: en,
	da: en,
	fi: en,
	no: en,
	cs: en,
	ro: en,
	hu: en,
	el: en,
	bg: en,
	hr: en,
	sk: en,
	sl: en,
	et: en,
	lv: en,
	lt: en,
	ja: en,
	zh: en,
	ko: en
};

/**
 * Current language store
 */
export const currentLanguage = writable<SupportedLanguage>('en');

/**
 * Translations store (derived from current language)
 */
export const t = derived(currentLanguage, ($lang) => translations[$lang] || translations.en);

/**
 * Detect browser language
 */
export function detectBrowserLanguage(): SupportedLanguage {
	if (!browser) return 'en';

	try {
		const browserLang = navigator.language || (navigator as any).userLanguage;
		if (browserLang) {
			const lang = browserLang.split('-')[0].toLowerCase() as SupportedLanguage;
			if (translations[lang]) {
				return lang;
			}
		}
	} catch (e) {
		console.debug('[i18n] Failed to detect browser language:', e);
	}

	return 'en';
}

/**
 * Initialize i18n with browser detection
 */
export function initializeI18n(): void {
	if (!browser) return;

	const detectedLang = detectBrowserLanguage();
	currentLanguage.set(detectedLang);

	console.debug('[i18n] Initialized with language:', detectedLang);
}

/**
 * Set language manually
 */
export function setLanguage(lang: SupportedLanguage): void {
	if (translations[lang]) {
		currentLanguage.set(lang);
		console.debug('[i18n] Language set to:', lang);
	} else {
		console.warn('[i18n] Language not supported:', lang);
	}
}

/**
 * Get current language
 */
export function getLanguage(): SupportedLanguage {
	return get(currentLanguage);
}

/**
 * Get all supported languages
 */
export function getSupportedLanguages(): SupportedLanguage[] {
	return Object.keys(translations) as SupportedLanguage[];
}

/**
 * Get translation for a specific key
 */
export function getTranslation(key: keyof ConsentTranslations): string {
	const lang = get(currentLanguage);
	return translations[lang]?.[key] || translations.en[key] || key;
}

/**
 * Add custom translations for a language
 */
export function addTranslations(
	lang: SupportedLanguage,
	customTranslations: Partial<ConsentTranslations>
): void {
	translations[lang] = {
		...translations[lang],
		...customTranslations
	};
}
