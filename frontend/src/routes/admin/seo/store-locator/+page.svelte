<script lang="ts">
	import { browser } from '$app/environment';
	import {
		IconMapPin,
		IconPlus,
		IconEdit,
		IconTrash,
		IconDownload,
		IconExternalLink,
		IconCheck,
		IconClock,
		IconPhone,
		IconMail,
		IconWorld
	} from '$lib/icons';
	import {
		locations,
		generateLocationSchema,
		generateKml,
		isLocationOpen,
		getNextOpenTime,
		formatAddress,
		getDirectionsUrl,
		daysOfWeek,
		type Location,
		type BusinessType
	} from '$lib/seo';
	import ConfirmationModal from '$lib/components/admin/ConfirmationModal.svelte';

	// State using Svelte 5 runes
	let locationList = $state<Location[]>([]);
	let loading = $state(false);
	let showAddModal = $state(false);
	let editingLocation = $state<Location | null>(null);

	// Delete confirmation modal state
	let showDeleteModal = $state(false);
	let pendingDeleteId = $state<string | null>(null);

	// Form state
	let formData = $state({
		name: '',
		businessType: 'LocalBusiness' as BusinessType,
		streetAddress: '',
		city: '',
		state: '',
		postalCode: '',
		country: 'US',
		phone: '',
		email: '',
		website: ''
	});

	// Stats
	let stats = $derived({
		total: locationList.length,
		active: locationList.filter((l) => l.isActive).length,
		withSchema: locationList.length // All have schema support
	});

	$effect(() => {
		if (browser) {
			const unsubscribe = locations.subscribe((locs) => {
				locationList = locs;
			});

			// Add sample locations if empty
			if (locationList.length === 0) {
				addSampleLocations();
			}

			return () => unsubscribe();
		}
	});

	function addSampleLocations() {
		// Sample location for demonstration
		locations.add({
			name: 'Revolution Trading Pros HQ',
			businessType: 'FinancialService',
			address: {
				streetAddress: '350 Fifth Avenue',
				addressLocality: 'New York',
				addressRegion: 'NY',
				postalCode: '10118',
				addressCountry: 'US',
				coordinates: { latitude: 40.7484, longitude: -73.9857 }
			},
			contacts: [
				{ type: 'phone', value: '+1 (555) 123-4567', isPrimary: true },
				{ type: 'email', value: 'info@revolutiontradingpros.com', isPrimary: true }
			],
			website: 'https://revolution-trading-pros.pages.dev',
			hours: [
				{ dayOfWeek: 'Monday', opens: '09:00', closes: '17:00' },
				{ dayOfWeek: 'Tuesday', opens: '09:00', closes: '17:00' },
				{ dayOfWeek: 'Wednesday', opens: '09:00', closes: '17:00' },
				{ dayOfWeek: 'Thursday', opens: '09:00', closes: '17:00' },
				{ dayOfWeek: 'Friday', opens: '09:00', closes: '17:00' },
				{ dayOfWeek: 'Saturday', opens: '00:00', closes: '00:00', isClosed: true },
				{ dayOfWeek: 'Sunday', opens: '00:00', closes: '00:00', isClosed: true }
			],
			isActive: true,
			isPrimary: true
		});
	}

	function openAddModal() {
		editingLocation = null;
		formData = {
			name: '',
			businessType: 'LocalBusiness',
			streetAddress: '',
			city: '',
			state: '',
			postalCode: '',
			country: 'US',
			phone: '',
			email: '',
			website: ''
		};
		showAddModal = true;
	}

	function openEditModal(location: Location) {
		editingLocation = location;
		formData = {
			name: location.name,
			businessType: location.businessType,
			streetAddress: location.address.streetAddress,
			city: location.address.addressLocality,
			state: location.address.addressRegion,
			postalCode: location.address.postalCode,
			country: location.address.addressCountry,
			phone: location.contacts.find((c) => c.type === 'phone')?.value || '',
			email: location.contacts.find((c) => c.type === 'email')?.value || '',
			website: location.website || ''
		};
		showAddModal = true;
	}

	function saveLocation() {
		const locationData = {
			name: formData.name,
			businessType: formData.businessType,
			address: {
				streetAddress: formData.streetAddress,
				addressLocality: formData.city,
				addressRegion: formData.state,
				postalCode: formData.postalCode,
				addressCountry: formData.country
			},
			contacts: [
				...(formData.phone
					? [{ type: 'phone' as const, value: formData.phone, isPrimary: true }]
					: []),
				...(formData.email
					? [{ type: 'email' as const, value: formData.email, isPrimary: true }]
					: [])
			],
			website: formData.website || '',
			hours: daysOfWeek.map((day) => ({
				dayOfWeek: day,
				opens: '09:00',
				closes: '17:00',
				isClosed: day === 'Saturday' || day === 'Sunday'
			})),
			isActive: true
		};

		if (editingLocation) {
			locations.update(editingLocation.id, locationData);
		} else {
			locations.add(locationData);
		}

		showAddModal = false;
	}

	function deleteLocation(id: string) {
		pendingDeleteId = id;
		showDeleteModal = true;
	}

	function confirmDeleteLocation() {
		if (!pendingDeleteId) return;
		showDeleteModal = false;
		const id = pendingDeleteId;
		pendingDeleteId = null;

		locations.remove(id);
	}

	function toggleLocation(id: string) {
		locations.toggle(id);
	}

	function downloadKml() {
		const kml = generateKml(locationList, 'Revolution Trading Pros Locations');
		const blob = new Blob([kml], { type: 'application/vnd.google-earth.kml+xml' });
		const url = URL.createObjectURL(blob);
		const a = document.createElement('a');
		a.href = url;
		a.download = 'locations.kml';
		a.click();
		URL.revokeObjectURL(url);
	}

	function viewSchema(location: Location) {
		const schema = generateLocationSchema(location);
		alert(JSON.stringify(schema, null, 2));
	}

	const businessTypes: { value: BusinessType; label: string }[] = [
		{ value: 'LocalBusiness', label: 'Local Business' },
		{ value: 'FinancialService', label: 'Financial Service' },
		{ value: 'ProfessionalService', label: 'Professional Service' },
		{ value: 'EducationalOrganization', label: 'Educational Organization' },
		{ value: 'Store', label: 'Store' },
		{ value: 'Restaurant', label: 'Restaurant' }
	];
</script>

<svelte:head>
	<title>Store Locator | SEO</title>
</svelte:head>

<div class="store-locator-page">
	<header class="page-header">
		<div>
			<h1>
				<IconMapPin size={28} />
				Store Locator & Local SEO
			</h1>
			<p>Manage multiple locations with LocalBusiness schema and KML export</p>
		</div>
		<div class="header-actions">
			<button class="btn-secondary" onclick={downloadKml}>
				<IconDownload size={18} />
				Export KML
			</button>
			<button class="btn-primary" onclick={openAddModal}>
				<IconPlus size={18} />
				Add Location
			</button>
		</div>
	</header>

	<div class="stats-grid">
		<div class="stat-card">
			<div class="stat-value">{stats.total}</div>
			<div class="stat-label">Total Locations</div>
		</div>
		<div class="stat-card">
			<div class="stat-value">{stats.active}</div>
			<div class="stat-label">Active</div>
		</div>
		<div class="stat-card">
			<div class="stat-value">{stats.withSchema}</div>
			<div class="stat-label">With Schema</div>
		</div>
	</div>

	<div class="locations-section">
		{#if loading}
			<div class="loading">Loading locations...</div>
		{:else if locationList.length === 0}
			<div class="empty-state">
				<IconMapPin size={48} />
				<p>No locations yet. Add your first location to get started.</p>
				<button class="btn-primary" onclick={openAddModal}>
					<IconPlus size={18} />
					Add Location
				</button>
			</div>
		{:else}
			<div class="locations-list">
				{#each locationList as location}
					<div class="location-card" class:inactive={!location.isActive}>
						{#if location.isPrimary}
							<span class="primary-badge">Primary</span>
						{/if}
						<div class="location-header">
							<h3>{location.name}</h3>
							<div class="location-status">
								{#if isLocationOpen(location)}
									<span class="open"><IconCheck size={14} /> Open</span>
								{:else}
									<span class="closed"><IconClock size={14} /> {getNextOpenTime(location)}</span>
								{/if}
							</div>
						</div>

						<div class="location-details">
							<div class="detail">
								<IconMapPin size={16} />
								{formatAddress(location.address)}
							</div>
							{#if location.contacts.find((c) => c.type === 'phone')}
								<div class="detail">
									<IconPhone size={16} />
									{location.contacts.find((c) => c.type === 'phone')?.value}
								</div>
							{/if}
							{#if location.contacts.find((c) => c.type === 'email')}
								<div class="detail">
									<IconMail size={16} />
									{location.contacts.find((c) => c.type === 'email')?.value}
								</div>
							{/if}
							{#if location.website}
								<div class="detail">
									<IconWorld size={16} />
									<a href={location.website} target="_blank" rel="noopener">{location.website}</a>
								</div>
							{/if}
						</div>

						<div class="location-actions">
							<a
								href={getDirectionsUrl(location)}
								target="_blank"
								rel="noopener"
								class="action-btn"
								title="Get Directions"
							>
								<IconExternalLink size={16} />
								Directions
							</a>
							<button class="action-btn" onclick={() => viewSchema(location)} title="View Schema">
								View Schema
							</button>
							<button class="action-btn" onclick={() => openEditModal(location)} title="Edit">
								<IconEdit size={16} />
							</button>
							<button
								class="action-btn"
								onclick={() => toggleLocation(location.id)}
								title={location.isActive ? 'Deactivate' : 'Activate'}
							>
								{location.isActive ? 'Deactivate' : 'Activate'}
							</button>
							<button
								class="action-btn danger"
								onclick={() => deleteLocation(location.id)}
								title="Delete"
							>
								<IconTrash size={16} />
							</button>
						</div>
					</div>
				{/each}
			</div>
		{/if}
	</div>
</div>

{#if showAddModal}
	<div
		class="modal-overlay"
		onclick={() => (showAddModal = false)}
		onkeydown={(e: KeyboardEvent) => e.key === 'Escape' && (showAddModal = false)}
		role="button"
		tabindex="-1"
		aria-label="Close modal"
	>
		<div
			class="modal"
			role="dialog"
			aria-modal="true"
			tabindex="-1"
			onclick={(e: MouseEvent) => e.stopPropagation()}
			onkeydown={(e: KeyboardEvent) => e.stopPropagation()}
		>
			<h3>{editingLocation ? 'Edit Location' : 'Add New Location'}</h3>

			<div class="form-grid">
				<div class="form-group full-width">
					<label for="name">Location Name *</label>
					<input type="text" id="name" name="name" bind:value={formData.name} required />
				</div>

				<div class="form-group">
					<label for="businessType">Business Type</label>
					<select id="businessType" bind:value={formData.businessType}>
						{#each businessTypes as type}
							<option value={type.value}>{type.label}</option>
						{/each}
					</select>
				</div>

				<div class="form-group">
					<label for="phone">Phone</label>
					<input type="tel" id="phone" name="phone" autocomplete="tel" bind:value={formData.phone} />
				</div>

				<div class="form-group full-width">
					<label for="streetAddress">Street Address *</label>
					<input type="text" id="streetAddress" name="streetAddress" bind:value={formData.streetAddress} required />
				</div>

				<div class="form-group">
					<label for="city">City *</label>
					<input type="text" id="city" name="city" bind:value={formData.city} required />
				</div>

				<div class="form-group">
					<label for="state">State/Province *</label>
					<input type="text" id="state" name="state" bind:value={formData.state} required />
				</div>

				<div class="form-group">
					<label for="postalCode">Postal Code *</label>
					<input type="text" id="postalCode" name="postalCode" bind:value={formData.postalCode} required />
				</div>

				<div class="form-group">
					<label for="country">Country</label>
					<input type="text" id="country" name="country" bind:value={formData.country} />
				</div>

				<div class="form-group full-width">
					<label for="email">Email</label>
					<input type="email" id="email" name="email" autocomplete="email" bind:value={formData.email} />
				</div>

				<div class="form-group full-width">
					<label for="website">Website</label>
					<input type="url" id="website" name="website" bind:value={formData.website} placeholder="https://" />
				</div>
			</div>

			<div class="modal-actions">
				<button class="btn-secondary" onclick={() => (showAddModal = false)}>Cancel</button>
				<button class="btn-primary" onclick={saveLocation}>
					{editingLocation ? 'Save Changes' : 'Add Location'}
				</button>
			</div>
		</div>
	</div>
{/if}

<style>
	.store-locator-page {
		padding: 2rem;
		max-width: 1200px;
		margin: 0 auto;
	}

	.page-header {
		display: flex;
		justify-content: space-between;
		align-items: flex-start;
		margin-bottom: 2rem;
	}

	.page-header h1 {
		display: flex;
		align-items: center;
		gap: 0.75rem;
		font-size: 1.75rem;
		font-weight: 700;
		color: #1a1a1a;
		margin-bottom: 0.5rem;
	}

	.page-header p {
		color: #666;
		font-size: 0.95rem;
	}

	.header-actions {
		display: flex;
		gap: 1rem;
	}

	.btn-primary,
	.btn-secondary {
		display: inline-flex;
		align-items: center;
		gap: 0.5rem;
		padding: 0.75rem 1.5rem;
		border-radius: 6px;
		font-weight: 500;
		cursor: pointer;
		transition: all 0.2s;
		text-decoration: none;
		border: none;
	}

	.btn-primary {
		background: #3b82f6;
		color: white;
	}

	.btn-primary:hover {
		background: #2563eb;
	}

	.btn-secondary {
		background: white;
		color: #374151;
		border: 1px solid #e5e5e5;
	}

	.btn-secondary:hover {
		background: #f9fafb;
	}

	.stats-grid {
		display: grid;
		grid-template-columns: repeat(auto-fit, minmax(150px, 1fr));
		gap: 1rem;
		margin-bottom: 2rem;
	}

	.stat-card {
		background: white;
		border: 1px solid #e5e5e5;
		border-radius: 8px;
		padding: 1.5rem;
		text-align: center;
	}

	.stat-value {
		font-size: 1.75rem;
		font-weight: 700;
		color: #1a1a1a;
	}

	.stat-label {
		color: #666;
		font-size: 0.85rem;
	}

	.locations-section {
		background: white;
		border: 1px solid #e5e5e5;
		border-radius: 12px;
		padding: 1.5rem;
	}

	.loading,
	.empty-state {
		text-align: center;
		padding: 3rem;
		color: #666;
	}

	.empty-state p {
		margin: 1rem 0;
	}

	.locations-list {
		display: flex;
		flex-direction: column;
		gap: 1rem;
	}

	.location-card {
		position: relative;
		padding: 1.5rem;
		background: #f9fafb;
		border-radius: 8px;
		border: 1px solid #e5e5e5;
	}

	.location-card.inactive {
		opacity: 0.6;
	}

	.primary-badge {
		position: absolute;
		top: 1rem;
		right: 1rem;
		background: #10b981;
		color: white;
		font-size: 0.7rem;
		font-weight: 600;
		padding: 0.25rem 0.5rem;
		border-radius: 4px;
		text-transform: uppercase;
	}

	.location-header {
		display: flex;
		justify-content: space-between;
		align-items: center;
		margin-bottom: 1rem;
	}

	.location-header h3 {
		font-size: 1.1rem;
		font-weight: 600;
		color: #1a1a1a;
		margin: 0;
	}

	.location-status {
		font-size: 0.85rem;
	}

	.location-status .open {
		color: #10b981;
		display: flex;
		align-items: center;
		gap: 0.25rem;
	}

	.location-status .closed {
		color: #6b7280;
		display: flex;
		align-items: center;
		gap: 0.25rem;
	}

	.location-details {
		display: flex;
		flex-direction: column;
		gap: 0.5rem;
		margin-bottom: 1rem;
	}

	.detail {
		display: flex;
		align-items: center;
		gap: 0.5rem;
		font-size: 0.9rem;
		color: #374151;
	}

	.detail a {
		color: #3b82f6;
		text-decoration: none;
	}

	.detail a:hover {
		text-decoration: underline;
	}

	.location-actions {
		display: flex;
		gap: 0.5rem;
		flex-wrap: wrap;
	}

	.action-btn {
		display: inline-flex;
		align-items: center;
		gap: 0.25rem;
		padding: 0.5rem 0.75rem;
		background: white;
		border: 1px solid #e5e5e5;
		border-radius: 4px;
		font-size: 0.8rem;
		cursor: pointer;
		text-decoration: none;
		color: #374151;
		transition: all 0.2s;
	}

	.action-btn:hover {
		background: #f3f4f6;
	}

	.action-btn.danger {
		color: #ef4444;
	}

	.action-btn.danger:hover {
		background: #fee2e2;
	}

	.modal-overlay {
		position: fixed;
		inset: 0;
		background: rgba(0, 0, 0, 0.5);
		display: flex;
		align-items: center;
		justify-content: center;
		z-index: 1000;
		padding: 1rem;
	}

	.modal {
		background: white;
		border-radius: 12px;
		padding: 2rem;
		max-width: 600px;
		width: 100%;
		max-height: 90vh;
		overflow-y: auto;
	}

	.modal h3 {
		font-size: 1.25rem;
		font-weight: 600;
		margin-bottom: 1.5rem;
	}

	.form-grid {
		display: grid;
		grid-template-columns: 1fr 1fr;
		gap: 1rem;
	}

	.form-group {
		display: flex;
		flex-direction: column;
		gap: 0.5rem;
	}

	.form-group.full-width {
		grid-column: 1 / -1;
	}

	.form-group label {
		font-weight: 500;
		font-size: 0.9rem;
		color: #374151;
	}

	.form-group input,
	.form-group select {
		padding: 0.75rem;
		border: 1px solid #e5e5e5;
		border-radius: 6px;
		font-size: 0.95rem;
	}

	.form-group input:focus,
	.form-group select:focus {
		outline: none;
		border-color: #3b82f6;
		box-shadow: 0 0 0 3px rgba(59, 130, 246, 0.1);
	}

	.modal-actions {
		display: flex;
		gap: 1rem;
		justify-content: flex-end;
		margin-top: 2rem;
	}

	@media (max-width: 768px) {
		.store-locator-page {
			padding: 1rem;
		}

		.page-header {
			flex-direction: column;
			gap: 1rem;
		}

		.header-actions {
			width: 100%;
			flex-direction: column;
		}

		.form-grid {
			grid-template-columns: 1fr;
		}

		.form-group.full-width {
			grid-column: 1;
		}
	}
</style>

<ConfirmationModal
	isOpen={showDeleteModal}
	title="Delete Location"
	message="Delete this location?"
	confirmText="Delete"
	variant="danger"
	onConfirm={confirmDeleteLocation}
	onCancel={() => { showDeleteModal = false; pendingDeleteId = null; }}
/>
