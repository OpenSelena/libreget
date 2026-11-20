<script lang="ts">
  import { onMount } from 'svelte';
  import {
    loadProfiles,
    saveProfile,
    deleteProfile,
    pullCreator,
    pullAllCreators,
    type Profile,
  } from '$lib/api/nami';
  import { showToast } from '$lib/stores/toast-store.svelte';
  import { t } from '$lib/i18n';

  interface Props {
    onSelectProfileForDownload?: (url: string) => void;
  }

  let { onSelectProfileForDownload }: Props = $props();

  const platforms = [
    { key: 'instagram', label: 'Instagram' },
    { key: 'tiktok', label: 'TikTok' },
    { key: 'facebook', label: 'Facebook' },
    { key: 'x', label: 'X' },
  ];

  let activePlatform = $state('instagram');
  let profiles = $state<Profile[]>([]);
  let newUrl = $state('');
  let loading = $state(false);
  let error = $state('');
  let syncingProfileUrl = $state<string | null>(null);
  let syncingAll = $state(false);
  let syncFeedback = $state<string | null>(null);

  let requestId = 0;

  async function loadPlatformProfiles(targetPlatform = activePlatform) {
    const thisRequest = ++requestId;
    loading = true;
    error = '';
    try {
      const result = await loadProfiles(targetPlatform);
      if (thisRequest !== requestId) return;
      profiles = result;
    } catch (err) {
      if (thisRequest !== requestId) return;
      error = $t('nami.failed_to_load_profiles') as string;
      console.error(err);
    } finally {
      if (thisRequest === requestId) {
        loading = false;
      }
    }
  }

  function errorMessage(err: unknown, fallback: string): string {
    if (typeof err === 'string') return err;
    if (err instanceof Error) return err.message;
    return fallback;
  }

  async function addProfile() {
    const trimmedUrl = newUrl.trim();
    if (!trimmedUrl) {
      error = $t('nami.enter_url_or_username') as string;
      return;
    }
    error = '';
    try {
      await saveProfile(activePlatform, trimmedUrl);
      newUrl = '';
      showToast('success', 'Profile added successfully');
      await loadPlatformProfiles();
    } catch (err) {
      error = errorMessage(err, $t('nami.failed_to_add_profile') as string);
      showToast('error', error);
      console.error(err);
    }
  }

  async function removeProfile(url: string) {
    try {
      await deleteProfile(activePlatform, url);
      showToast('info', 'Profile deleted');
      await loadPlatformProfiles();
    } catch (err) {
      error = errorMessage(err, $t('nami.failed_to_delete_profile') as string);
      showToast('error', error);
      console.error('Failed to delete profile:', err);
    }
  }

  function formatSynced(timestamp?: number | null): string {
    if (!timestamp) return 'Never synced';
    const d = new Date(timestamp * 1000);
    return `Synced ${d.toLocaleDateString(undefined, { month: 'short', day: 'numeric' })}`;
  }

  async function handlePull(profile: Profile) {
    syncingProfileUrl = profile.url;
    syncFeedback = null;
    error = '';
    try {
      const res = await pullCreator(profile.url, activePlatform);
      if (res.success) {
        syncFeedback = `Synced ${res.files_downloaded} new files from ${profile.username || profile.url}`;
        showToast('success', syncFeedback);
        await loadPlatformProfiles();
      } else {
        error = res.message || 'Sync failed';
        showToast('error', error);
      }
    } catch (err) {
      error = errorMessage(err, 'Failed to sync profile');
      showToast('error', error);
    } finally {
      syncingProfileUrl = null;
    }
  }

  async function handlePullAll() {
    if (profiles.length === 0 || syncingAll) return;
    syncingAll = true;
    syncFeedback = null;
    error = '';
    try {
      const summary = await pullAllCreators(activePlatform);
      syncFeedback = `Batch sync complete: ${summary.total_downloaded} files downloaded across ${summary.succeeded} creators`;
      showToast('success', syncFeedback);
      await loadPlatformProfiles();
    } catch (err) {
      error = errorMessage(err, 'Failed to sync all creators');
      showToast('error', error);
    } finally {
      syncingAll = false;
    }
  }

  function switchPlatform(p: string) {
    activePlatform = p;
    loadPlatformProfiles(p);
  }

  onMount(() => {
    loadPlatformProfiles(activePlatform);
  });
</script>

<div class="pm-page">
  <!-- Platform Selector -->
  <div class="segmented segmented-lg" role="tablist">
    {#each platforms as platform (platform.key)}
      <button
        type="button"
        role="tab"
        aria-selected={activePlatform === platform.key}
        class="segmented-btn"
        class:active={activePlatform === platform.key}
        onclick={() => switchPlatform(platform.key)}
      >
        {platform.label}
      </button>
    {/each}
  </div>

  <!-- Add Profile Row -->
  <div class="surface-card pm-add-card">
    <input
      class="input input-lg"
      type="text"
      bind:value={newUrl}
      placeholder="Add creator profile URL or @username..."
      onkeydown={(e) => e.key === 'Enter' && addProfile()}
      autocomplete="off"
      spellcheck="false"
    />
    <button
      type="button"
      class="btn btn-lg btn-primary"
      onclick={addProfile}
      disabled={!newUrl.trim() || loading}
    >
      Add Profile
    </button>
  </div>

  <!-- Sync Feedback or Alert -->
  {#if error}
    <div class="notice notice-danger" role="alert">
      <span>{error}</span>
    </div>
  {/if}

  {#if syncFeedback}
    <div class="notice notice-success" role="status">
      <span>{syncFeedback}</span>
      <button type="button" class="btn btn-sm btn-ghost" onclick={() => (syncFeedback = null)}>×</button>
    </div>
  {/if}

  <!-- Header Bar -->
  <div class="pm-section-head">
    <div class="pm-count-badge">
      <span class="section-header-title">{activePlatform.toUpperCase()} PROFILES</span>
      <span class="badge badge-neutral">{profiles.length}</span>
    </div>

    <button
      type="button"
      class="btn btn-sm btn-secondary"
      onclick={handlePullAll}
      disabled={profiles.length === 0 || syncingAll}
    >
      <svg
        viewBox="0 0 16 16"
        width="13"
        height="13"
        fill="none"
        stroke="currentColor"
        stroke-width="1.5"
        stroke-linecap="round"
        stroke-linejoin="round"
        class:spin={syncingAll}
      >
        <path d="M1.5 8a6.5 6.5 0 0 1 11.1-4.6L14 5" />
        <path d="M14 2v3h-3" />
        <path d="M14.5 8a6.5 6.5 0 0 1-11.1 4.6L2 11" />
        <path d="M2 14v-3h3" />
      </svg>
      <span>{syncingAll ? 'Syncing...' : 'Sync All'}</span>
    </button>
  </div>

  <!-- Profiles List -->
  {#if loading}
    <div class="empty-state">
      <div class="spinner"></div>
      <p class="empty-state-hint">Loading profiles...</p>
    </div>
  {:else if profiles.length === 0}
    <div class="empty-state surface-card">
      <p class="empty-state-title">No profiles added for {activePlatform}</p>
      <p class="empty-state-hint">Paste a username or profile URL above to track and sync creator updates.</p>
    </div>
  {:else}
    <div class="surface-card">
      {#each profiles as profile (profile.url)}
        <div
          class="list-row interactive"
          role="button"
          tabindex="0"
          onclick={() => onSelectProfileForDownload?.(profile.url)}
          onkeydown={(e) => {
            if (e.key === 'Enter') onSelectProfileForDownload?.(profile.url);
          }}
        >
          <div class="pm-initials-badge" data-platform={activePlatform}>
            <span>{(profile.username || profile.platform || '?').slice(0, 2).toUpperCase()}</span>
          </div>

          <span class="list-row-content">
            <span class="list-row-title">{profile.username || profile.url}</span>
            <span class="list-row-sub">{profile.url} • {formatSynced(profile.last_pulled_at)}</span>
          </span>

          <span class="list-row-trailing">
            <button
              type="button"
              class="btn btn-sm btn-secondary"
              onclick={(e) => {
                e.stopPropagation();
                handlePull(profile);
              }}
              disabled={syncingProfileUrl === profile.url}
              title="Sync latest creator updates"
            >
              <svg
                viewBox="0 0 16 16"
                width="12"
                height="12"
                fill="none"
                stroke="currentColor"
                stroke-width="1.5"
                class:spin={syncingProfileUrl === profile.url}
              >
                <path d="M1.5 8a6.5 6.5 0 0 1 11.1-4.6L14 5" />
                <path d="M14 2v3h-3" />
                <path d="M14.5 8a6.5 6.5 0 0 1-11.1 4.6L2 11" />
                <path d="M2 14v-3h3" />
              </svg>
              <span>{syncingProfileUrl === profile.url ? 'Syncing...' : 'Sync'}</span>
            </button>

            {#if onSelectProfileForDownload}
              <button
                type="button"
                class="btn btn-sm btn-primary"
                onclick={(e) => {
                  e.stopPropagation();
                  onSelectProfileForDownload?.(profile.url);
                }}
              >
                Download
              </button>
            {/if}

            <button
              type="button"
              class="btn btn-sm btn-ghost"
              title="Remove Profile"
              onclick={(e) => {
                e.stopPropagation();
                removeProfile(profile.url);
              }}
            >
              <svg viewBox="0 0 24 24" width="14" height="14" fill="none" stroke="currentColor" stroke-width="2">
                <polyline points="3 6 5 6 21 6" />
                <path d="M19 6v14a2 2 0 0 1-2 2H7a2 2 0 0 1-2-2V6m3 0V4a2 2 0 0 1 2-2h4a2 2 0 0 1 2 2v2" />
              </svg>
            </button>
          </span>
        </div>
      {/each}
    </div>
  {/if}
</div>

<style>
  .pm-page {
    display: flex;
    flex-direction: column;
    gap: var(--space-4);
    width: 100%;
  }

  .pm-add-card {
    display: flex;
    gap: var(--space-2);
    padding: var(--space-2);
  }

  .pm-add-card .input {
    flex: 1;
  }

  .pm-section-head {
    display: flex;
    align-items: center;
    justify-content: space-between;
    padding: 0 var(--space-1);
  }

  .pm-count-badge {
    display: flex;
    align-items: center;
    gap: var(--space-2);
  }

  .pm-initials-badge {
    width: 34px;
    height: 34px;
    border-radius: var(--radius-sm);
    display: flex;
    align-items: center;
    justify-content: center;
    font-size: 12px;
    font-weight: 700;
    color: var(--on-accent);
    background: var(--accent);
    flex-shrink: 0;
  }

  .pm-initials-badge[data-platform="instagram"] {
    background: linear-gradient(135deg, #833ab4, #fd1d1d, #fcb045);
    color: #ffffff;
  }

  .pm-initials-badge[data-platform="tiktok"] {
    background: #010101;
    color: #00f2fe;
  }

  .pm-initials-badge[data-platform="x"] {
    background: #000000;
    color: #ffffff;
  }

  .pm-initials-badge[data-platform="facebook"] {
    background: #1877f2;
    color: #ffffff;
  }

  .spin {
    animation: spin 1s linear infinite;
  }

  @keyframes spin {
    to { transform: rotate(360deg); }
  }
</style>
