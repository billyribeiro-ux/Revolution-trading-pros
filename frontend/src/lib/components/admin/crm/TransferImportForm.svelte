<script lang="ts">
    import { onMount } from 'svelte';
    import { goto } from '$app/navigation';
    import { crmAPI } from '#lib/api/crm.js';
    import { contactTransferFields, type TransferOption } from '#lib/api/crm-transfer-contract.js';
    import { apiErrorMessage } from '#lib/utils/api-error-message.js';
    import type { ImportJob, ImportType } from '#lib/crm/types.js';
    import type { JsonValue } from '#lib/api/_types.js';
    let {kind='contacts',jobId}: {kind?:ImportType;jobId?:string}=$props();
    let job=$state<ImportJob|null>(null), preview=$state<{headers:string[];rows:Record<string,JsonValue>[];total_rows:number}|null>(null);
    let files=$state<FileList>(), mapping=$state<Record<string,string>>({});
    let mode=$state<'skip'|'update'|'fail'>('skip'), status=$state('');
    type RefKind='lists'|'tags';
    const referenceKinds:RefKind[]=['lists','tags'];
    let selections=$state<Record<RefKind,string[]>>({lists:[],tags:[]});
    let referenceSearch=$state({lists:'',tags:''});
    let referenceErrors=$state({lists:'',tags:''});
    type Catalog={options:TransferOption[];missing:string[];page:number;pages:number;total:number|null;loading:boolean};
    const emptyCatalog=():Catalog=>({options:[],missing:[],page:1,pages:1,total:null,loading:false});
    let catalogs=$state<Record<RefKind,Catalog>>({lists:emptyCatalog(),tags:emptyCatalog()});
    const referenceGeneration={lists:0,tags:0};
    let busy=$state(false), loading=$state(true), error=$state(''), notice=$state('');
    let disposed=false;
    const pending=$derived(job?.status==='pending');
    function apply(saved:ImportJob) {
        job=saved;mapping={...saved.field_mapping};
        mode=saved.settings?.update_existing ? 'update' : saved.settings?.skip_duplicates===false ? 'fail' : 'skip';
        status=saved.settings?.contact_status ?? '';selections.lists=saved.settings?.add_to_lists ?? [];selections.tags=saved.settings?.apply_tags ?? [];
    }
    async function load() {
        if (busy) return; loading=true;error='';
        try {
            if (jobId) {
                const saved=await crmAPI.getImportJob(jobId);
                if (disposed) return;apply(saved);
                const source=await crmAPI.getImportPreview(saved.id);
                if (disposed) return;preview=source;
                if (saved.status==='pending') mapping=Object.fromEntries(source.headers.map(header=>[header,saved.field_mapping?.[header] ?? (contactTransferFields.includes(header as typeof contactTransferFields[number]) ? header : '')]));
                await Promise.all(referenceKinds.map(kind=>loadReferences(kind,1)));
            }
        } catch (cause) {if (!disposed) error=apiErrorMessage(cause,'Unable to load the saved import');}
        finally {if (!disposed) loading=false;}
    }
    async function upload(event:SubmitEvent) {
        event.preventDefault();const file=files?.[0];if (busy || !file) return;
        busy=true;error='';notice='';
        try {
            if (file.size>2*1024*1024) throw new Error('Choose a CSV or JSON file up to2MiB and1000rows');
            const data=new FormData();data.set('type',kind);data.set('file',file);
            const saved=await crmAPI.createImportJob(data);
            if (disposed) return;apply(saved);
            await goto(`/admin/crm/import-export/new?job=${encodeURIComponent(saved.id)}`,{replace:true,reset:false});
        } catch (cause) {if (!disposed) error=apiErrorMessage(cause,'The upload was not confirmed');}
        finally {if (!disposed) busy=false;}
    }
    async function loadReferences(kind:RefKind,page:number) {
        const current=++referenceGeneration[kind];catalogs[kind].loading=true;referenceErrors[kind]='';
        try {
            const result=await crmAPI.getTransferOptions(kind,{page,per_page:20,search:referenceSearch[kind] || undefined,selected:selections[kind]});
            if (disposed || current!==referenceGeneration[kind]) return;
            const options=[...new Map([...result.data,...result.selected].map(value=>[value.id,value])).values()];
            catalogs[kind]={options,missing:result.missing,page,pages:result.meta.total_pages ?? 1,total:result.meta.total,loading:false};
        } catch (cause) {if (!disposed && current===referenceGeneration[kind]) referenceErrors[kind]=apiErrorMessage(cause,`Unable to load saved ${kind}`);}
        finally {if (!disposed && current===referenceGeneration[kind]) catalogs[kind].loading=false;}
    }
    async function start(event:SubmitEvent) {
        event.preventDefault();if (busy || !job || !pending) return;
        busy=true;error='';notice='';
        try {
            const saved=await crmAPI.startImportJob(job.id,Object.fromEntries(Object.entries(mapping).filter(([,value])=>value)),{update_existing:mode==='update',skip_duplicates:mode==='skip',add_to_lists:selections.lists,apply_tags:selections.tags,...(status ? {contact_status:status} : {})});
            if (disposed) return;apply(saved);
            notice=saved.status==='completed' ? `Import completed: ${saved.created_count} created, ${saved.updated_count} updated, ${saved.skipped_count} skipped.` : 'Validation failed. No contacts were changed; upload a corrected file.';
        } catch (cause) {if (!disposed) error=apiErrorMessage(cause,'Import execution was not confirmed. Refresh its saved status before retrying.');}
        finally {if (!disposed) busy=false;}
    }
    async function cancel() {
        if (busy || !job || !pending) return;busy=true;error='';
        try {const saved=await crmAPI.cancelImportJob(job.id);if (!disposed) {apply(saved);notice='Pending import cancelled.';}}
        catch (cause) {if (!disposed) error=apiErrorMessage(cause,'Cancellation was not confirmed');}
        finally {if (!disposed) busy=false;}
    }
    onMount(()=>{void load();return()=>{disposed=true;};});
</script>

<div class="transfer-form">
    <a href="/admin/crm/import-export">← Import / Export</a>
    <h1>{job ? 'Saved import' : `Import ${kind}`}</h1>
    <p>Upload CSV or a JSON array, map fields, then apply the saved file. Up to1000contacts and2MiB per upload.</p>
    {#if kind!=='contacts'}<p role="status">The {kind} transfer workflow remains unfinished. Contact transfers are currently implemented.</p>{/if}
    {#if error}<div role="alert">{error}<button type="button" onclick={load} disabled={busy}>Refresh saved status</button></div>{/if}
    {#if notice}<p role="status">{notice}</p>{/if}
    {#if loading}<p>Loading saved import…</p>
    {:else if !job}
        <form onsubmit={upload}>
            <label>CSV or JSON file<input name="file" type="file" accept=".csv,.json,text/csv,application/json" bind:files disabled={busy} required /></label>
            <p>Recognized fields: {contactTransferFields.join(', ')}. Tags use a JSON array and custom_fields uses a JSON object.</p>
            <button type="submit" disabled={busy || !files?.length}>{busy ? 'Uploading…' : 'Upload and preview'}</button>
        </form>
    {:else}
        <section aria-label="Saved import summary"><h2>{job.file_name}</h2><p>Status: <strong>{job.status}</strong> · {job.processed_rows} / {job.total_rows} rows processed</p><p>{job.created_count} created · {job.updated_count} updated · {job.skipped_count} skipped · {job.error_count} errors</p><button type="button" onclick={load} disabled={busy}>Refresh saved status</button></section>
        {#if preview}
            <form onsubmit={start}>
                <h2>Field mapping</h2>
                {#each preview.headers as header (header)}<label>{header}<select aria-label={`Map ${header}`} bind:value={mapping[header]} disabled={busy || !pending}><option value="">Skip this field</option>{#each contactTransferFields as field (field)}<option value={field}>{field.replaceAll('_',' ')}</option>{/each}</select></label>{/each}
                <h2>Import settings</h2>
                <label>Existing or repeated email<select bind:value={mode} disabled={busy || !pending}><option value="skip">Skip duplicate emails</option><option value="update">Update one existing contact; reject repeated file emails</option><option value="fail">Reject duplicate emails</option></select></label>
                <label>Contact status override<select bind:value={status} disabled={busy || !pending}><option value="">Keep file or saved status</option>{#each ['lead','prospect','customer','churned','unqualified','new','contacted','qualified','proposal','negotiation','won','lost'] as value (value)}<option value={value}>{value}</option>{/each}</select></label>
                {#each referenceKinds as refKind (refKind)}
                    <fieldset disabled={busy || !pending}><legend>Add contacts to saved {refKind}</legend>
                        <label>Search saved {refKind}<input bind:value={referenceSearch[refKind]} maxlength="500" /></label>
                        <button type="button" onclick={()=>loadReferences(refKind,1)} disabled={catalogs[refKind].loading}>Search {refKind}</button>
                        {#if referenceErrors[refKind]}<p role="alert">{referenceErrors[refKind]}</p><button type="button" onclick={()=>loadReferences(refKind,catalogs[refKind].page)}>Retry {refKind}</button>{/if}
                        <label>Selected {refKind}<select multiple bind:value={selections[refKind]} disabled={catalogs[refKind].loading}>{#each catalogs[refKind].options as option (option.id)}<option value={option.id}>{option.title}</option>{/each}{#each catalogs[refKind].missing as id (id)}<option value={id}>Unavailable saved selection ({id})</option>{/each}</select></label>
                        {#if catalogs[refKind].missing.length}<p>Some saved references no longer exist. Remove unavailable selections before applying.</p><button type="button" onclick={()=>selections[refKind]=selections[refKind].filter(id=>!catalogs[refKind].missing.includes(id))}>Remove unavailable {refKind}</button>{/if}
                        <div class="actions"><button type="button" onclick={()=>loadReferences(refKind,catalogs[refKind].page-1)} disabled={catalogs[refKind].loading || catalogs[refKind].page<=1}>Previous {refKind}</button><span>Page{catalogs[refKind].page} of{catalogs[refKind].pages} · {catalogs[refKind].total ?? '—'} matches</span><button type="button" onclick={()=>loadReferences(refKind,catalogs[refKind].page+1)} disabled={catalogs[refKind].loading || catalogs[refKind].page>=catalogs[refKind].pages}>Next {refKind}</button></div>
                    </fieldset>
                {/each}
                <p>All rows are validated before changes are committed. Contact imports do not confirm newsletter subscriptions.</p>
                {#if pending}<div class="actions"><button type="submit" disabled={busy}>{busy ? 'Applying…' : 'Apply import'}</button><button type="button" onclick={cancel} disabled={busy}>Cancel pending import</button></div>{/if}
            </form>
            <details><summary>Preview ({Math.min(20,preview.total_rows)} of {preview.total_rows} rows)</summary><div class="preview-table"><table><thead><tr>{#each preview.headers as header (header)}<th>{header}</th>{/each}</tr></thead><tbody>{#each preview.rows as row (row)}<tr>{#each preview.headers as header (header)}<td>{typeof row[header]==='string' ? row[header] : JSON.stringify(row[header] ?? null)}</td>{/each}</tr>{/each}</tbody></table></div></details>
        {/if}
        {#if job.errors?.length}<h2>Validation errors</h2><ul>{#each job.errors as item (item.row)}<li>Row{item.row}{item.field ? ` (${item.field})` : ''}: {item.message}</li>{/each}</ul>{/if}
        {#if !pending}<a href="/admin/crm/import-export/new">Upload another file</a>{/if}
    {/if}
</div>
<style>
    .transfer-form{max-width:1000px;color:var(--text-primary,#f0f6fc)}a{color:#e6b800}h1{font-size:1.8rem;margin:1rem 0}h2{font-size:1.2rem}p{color:var(--text-secondary,#b1bac4);line-height:1.6}form,section,details{background:var(--bg-elevated,#161b22);padding:1.4rem;border:1px solid #30363d;border-radius:12px;margin:1rem 0}label{display:grid;gap:.4rem;margin:1rem 0}input,select,button{font:inherit;border:1px solid #30363d;border-radius:7px;padding:.65rem;background:#0d1117;color:#f0f6fc}button{cursor:pointer}button:disabled{opacity:.6;cursor:wait}.actions{display:flex;gap:1rem;flex-wrap:wrap}[role='alert']{border:1px solid #f85149;padding:1rem;border-radius:8px;display:flex;justify-content:space-between;gap:1rem}[role='status']{color:#e6b800}.preview-table{overflow:auto}table{width:100%;border-collapse:collapse}td,th{text-align:left;padding:.6rem;border-bottom:1px solid #30363d;white-space:pre-wrap;min-width:140px}
</style>
