<script lang="ts">
  import {
    loadProfiles,
    saveProfile,
    deleteProfile,
    httpFetch,
    type Profile,
  } from '$lib/api/nami';
  import {
    isActive,
    isCancelling,
    getFilesDownloaded,
    getStage,
    getStageIndex,
    getStageTotal,
    getLastMessage,
    getStatus,
    getStatusType,
    getStatusDetail,
    clearStatus as clearStoreStatus,
    startDownload as startStoreDownload,
    stopDownload as stopStoreDownload,
    getDraft,
    setDraft,
  } from '$lib/stores/nami_download_store.svelte';
  import {
    getOutputDir,
    getCookiesFile,
    getBrowserCookies,
    loadSettings,
  } from '$lib/stores/nami_settings_store.svelte';
  import {
    ensureDependenciesChecked,
    getDependencyStatus,
    isCheckingDependencies,
  } from '$lib/stores/nami_deps_store.svelte';
  import { showToast } from '$lib/stores/toast-store.svelte';
  import { parseProfileUrl, type ParsedProfile } from '$lib/utils/nami_parser';
  import { namiStageLabel, namiStatusMessage, type NamiTranslate } from '$lib/nami/i18n';
  import { resolveCreatorMetadata, getCachedCreator } from '$lib/nami/metadata';
  import { onMount, onDestroy } from 'svelte';
  import { goto } from '$app/navigation';
  import { t } from '$lib/i18n';

  interface Props {
    onSwitchToProfiles?: () => void;
    onSwitchToSettings?: () => void;
  }

  let { onSwitchToProfiles, onSwitchToSettings }: Props = $props();

  const tr: NamiTranslate = (key, values) => $t(key, values) as string;

  const initialDraft = getDraft();
  let url = $state(initialDraft.url);
  let contentType = $state(initialDraft.contentType || 'all');
  let selectedProfileUrl = $state<string | null>(initialDraft.selectedProfileUrl);

  onDestroy(() => {
    setDraft({ url, contentType, selectedProfileUrl });
  });

  interface CreatorInfo {
    platform: string;
    platformName: string;
    name: string;
    handle: string;
    meta: string;
    avatar: string;
    url: string;
  }

  let savedProfiles = $state<Array<Profile & { _platformLabel?: string }>>([]);
  let currentCreator = $state<CreatorInfo | null>(null);
  let avatarError = $state(false);
  let isResolvingMeta = $state(false);
  let profileAvatars = $state<Record<string, string>>({});
  let urlInputEl = $state<HTMLInputElement | null>(null);

  let downloading = $derived(isActive());
  let cancelling = $derived(isCancelling());
  let filesDownloaded = $derived(getFilesDownloaded());
  let stage = $derived(getStage());
  let stageIndex = $derived(getStageIndex());
  let stageTotal = $derived(getStageTotal());
  let lastMessage = $derived(getLastMessage());
  let status = $derived(getStatus());
  let statusType = $derived(getStatusType());
  let statusDetail = $derived(getStatusDetail());

  let outputDir = $derived(getOutputDir());
  let cookiesFile = $derived(getCookiesFile());
  let browserCookies = $derived(getBrowserCookies());
  let hasCookies = $derived(Boolean(cookiesFile.trim() || (browserCookies && browserCookies !== 'none')));
  let missingOutputDir = $derived(!outputDir.trim());
  let needsCookiesWarning = $derived(
    (contentType === 'stories' || contentType === 'highlights') && !hasCookies
  );

  let parsed = $derived(parseProfileUrl(url.trim()));
  let isSupported = $derived(parsed.platform !== 'unknown');
  let isInstagram = $derived(parsed.platform === 'instagram');
  let platformMismatch = $derived(
    (contentType === 'stories' || contentType === 'highlights') && !isInstagram
  );

  let depStatus = $derived(getDependencyStatus());
  let depChecking = $derived(isCheckingDependencies());

  let isSaved = $derived.by(() => {
    if (!currentCreator) return false;
    const clean = currentCreator.url.toLowerCase().trim();
    return savedProfiles.some((p) => p.url.toLowerCase().trim() === clean);
  });

  onMount(async () => {
    await loadSettings();
    await loadSavedProfiles();
    await ensureDependenciesChecked();
    if (url.trim()) {
      handleUrlChange(url.trim());
    }
  });

  async function loadSavedProfiles() {
    try {
      const platformLabels: Record<string, string> = {
        instagram: 'Instagram',
        tiktok: 'TikTok',
        facebook: 'Facebook',
        x: 'X',
      };
      const results = await Promise.all(
        Object.keys(platformLabels).map(async (platform) => {
          const profiles = await loadProfiles(platform);
          return profiles.map((p) => ({ ...p, _platformLabel: platformLabels[platform] }));
        })
      );
      savedProfiles = results.flat();
      savedProfiles.forEach(async (profile) => {
        const target = profile.username || profile.url;
        const cached = getCachedCreator(profile.platform, target);
        if (cached?.avatar) {
          profileAvatars[profile.url] = cached.avatar;
        } else {
          const parsed = parseProfileUrl(profile.url);
          if (parsed.platform !== 'unknown' && parsed.cleanUrl) {
            try {
              const meta = await resolveCreatorMetadata(parsed);
              if (meta?.avatar) {
                profileAvatars[profile.url] = meta.avatar;
              }
            } catch {}
          }
        }
      });
    } catch (error) {
      console.error('Failed to load profiles:', error);
    }
  }

  let addedNoticeActive = $state(false);
  let addedNoticeTimer: ReturnType<typeof setTimeout> | null = null;

  onDestroy(() => {
    if (addedNoticeTimer) {
      clearTimeout(addedNoticeTimer);
    }
  });

  function handleUrlChange(val: string) {
    url = val;
    avatarError = false;
    addedNoticeActive = false;
    const trimmed = val.trim();

    if (!trimmed) {
      currentCreator = null;
      setDraft({ url: '', contentType, selectedProfileUrl: null });
      return;
    }

    const p = parseProfileUrl(trimmed);
    if (p.platform === 'unknown' || !p.cleanUrl) {
      currentCreator = null;
      return;
    }

    const platformLabels: Record<string, string> = {
      instagram: 'Instagram',
      tiktok: 'TikTok',
      facebook: 'Facebook',
      x: 'X',
    };
    const platName = platformLabels[p.platform] || p.platform;
    const handle = p.username ? `@${p.username}` : p.cleanUrl;
    let name = p.username ? p.username : platName;
    let meta = `${handle} • ${platName}`;
    let avatar = '';

    const target = p.username || p.cleanUrl;
    const cached = getCachedCreator(p.platform, target);
    if (cached) {
      if (cached.name) name = cached.name;
      if (cached.meta) meta = cached.meta;
      if (cached.avatar) avatar = cached.avatar;
    }

    currentCreator = {
      platform: p.platform,
      platformName: platName,
      name,
      handle,
      meta,
      avatar,
      url: p.cleanUrl,
    };

    setDraft({ url: p.cleanUrl, contentType, selectedProfileUrl: p.cleanUrl });

    // Asynchronously enrich metadata & real avatar without blocking
    enrichCreatorInfo(p);
  }

  async function enrichCreatorInfo(p: ParsedProfile) {
    if (!currentCreator?.avatar) {
      isResolvingMeta = true;
    }
    try {
      const meta = await resolveCreatorMetadata(p);
      if (meta && currentCreator && currentCreator.url === p.cleanUrl) {
        currentCreator = {
          ...currentCreator,
          name: meta.name || currentCreator.name,
          meta: meta.meta || currentCreator.meta,
          avatar: meta.avatar || currentCreator.avatar,
        };
      }
    } catch {
      // Fallback remains active
    } finally {
      isResolvingMeta = false;
    }
  }

  async function handlePaste() {
    try {
      const text = await navigator.clipboard.readText();
      if (text && text.trim()) {
        handleUrlChange(text.trim());
      }
    } catch {
      showToast('error', 'Clipboard permission was not granted');
    }
  }

  function clearUrl() {
    url = '';
    handleUrlChange('');
    urlInputEl?.focus();
  }

  async function toggleSaveProfile() {
    if (!currentCreator) return;
    const targetUrl = currentCreator.url;
    const platform = currentCreator.platform;

    try {
      if (isSaved) {
        await deleteProfile(platform, targetUrl);
        showToast('info', `Profile removed: ${currentCreator.name}`);
      } else {
        await saveProfile(platform, targetUrl);
        showToast('success', `Profile saved: ${currentCreator.name}`);
      }
      await loadSavedProfiles();
    } catch (err: unknown) {
      const msg = err instanceof Error ? err.message : String(err);
      showToast('error', `Failed to update profile: ${msg}`);
    }
  }

  async function handleDeleteSavedProfile(p: Profile) {
    try {
      await deleteProfile(p.platform, p.url);
      showToast('info', `Removed ${p.username || p.url}`);
      await loadSavedProfiles();
    } catch (err: unknown) {
      const msg = err instanceof Error ? err.message : String(err);
      showToast('error', `Failed to delete profile: ${msg}`);
    }
  }

  function loadProfileDirectly(profileUrl: string) {
    handleUrlChange(profileUrl);
  }

  async function startDownload() {
    const downloadUrl = currentCreator?.url || url.trim();
    if (!downloadUrl || missingOutputDir || needsCookiesWarning || platformMismatch) {
      return;
    }

    const creatorSnapshot = currentCreator ? { ...currentCreator } : null;

    try {
      void startStoreDownload(downloadUrl, outputDir, cookiesFile, contentType, creatorSnapshot);
      url = '';
      currentCreator = null;
      setDraft({ url: '', contentType, selectedProfileUrl: null });
      addedNoticeActive = true;
      if (addedNoticeTimer) clearTimeout(addedNoticeTimer);
      addedNoticeTimer = setTimeout(() => {
        addedNoticeActive = false;
      }, 5000);
    } catch (error) {
      console.error('Failed to start download:', error);
    }
  }

  async function handleStop() {
    await stopStoreDownload();
  }

  function clearStatus() {
    clearStoreStatus();
  }

  const CONTENT_TYPES: Array<{ key: string; label: string }> = [
    { key: 'all', label: 'All Posts' },
    { key: 'photos', label: 'Photos' },
    { key: 'videos', label: 'Videos' },
    { key: 'stories', label: 'Stories' },
    { key: 'highlights', label: 'Highlights' },
  ];
</script>

<div class="nami-home-stage">
  <!-- Centered Mascot Logo with warm light halo -->
  <div class="nami-mascot-wrapper">
    <div class="nami-mascot-glow"></div>
    <div class="nami-mascot-icon">
      <img src="/icons/nav/nami.svg" alt="Nami" width="84" height="84" class="mascot-img" />
    </div>
  </div>

  <!-- Canonical LibreGet HomeUrlBar -->
  <div class="url-bar">
    <input
      id="nami-url-input"
      class="url-input"
      type="text"
      aria-label="Creator profile or media URL"
      bind:this={urlInputEl}
      bind:value={url}
      oninput={(e) => handleUrlChange((e.target as HTMLInputElement).value)}
      onkeydown={(e) => {
        if (e.key === 'Enter') startDownload();
      }}
      placeholder="Paste creator profile or media URL..."
      autocomplete="off"
      spellcheck="false"
    />

    {#if url.trim()}
      <button
        type="button"
        class="clear-btn"
        onclick={clearUrl}
        title="Clear URL"
        aria-label="Clear URL"
      >
        <svg viewBox="0 0 16 16" width="14" height="14" fill="currentColor">
          <path d="M8 1.4a6.6 6.6 0 1 0 0 13.2A6.6 6.6 0 0 0 8 1.4zm2.35 3.5a.75.75 0 0 1 1.06 1.06L9.06 8l2.35 2.35a.75.75 0 1 1-1.06 1.06L8 9.06 5.65 11.4a.75.75 0 1 1-1.06-1.06L6.94 8 4.6 5.65A.75.75 0 0 1 5.65 4.6L8 6.94l2.35-2.04z" />
        </svg>
      </button>

      <button
        type="button"
        class="go-btn"
        onclick={startDownload}
        title="Download"
        disabled={missingOutputDir || needsCookiesWarning || platformMismatch}
        aria-label="Start download"
      >
        <svg viewBox="0 0 24 24" width="18" height="18" fill="none" stroke="currentColor" stroke-width="2.5" stroke-linecap="round" stroke-linejoin="round">
          <path d="M12 5v14M6 13l6 6 6-6" />
        </svg>
      </button>
    {:else}
      <button
        type="button"
        class="paste-btn"
        onclick={handlePaste}
      >
        <span>Paste</span>
        <kbd>Ctrl+V</kbd>
      </button>
    {/if}
  </div>

  <!-- Detected Platform Feedback -->
  {#if currentCreator}
    <div class="feedback feedback-enter" data-supported="true">
      <span class="feedback-text">{currentCreator.platformName} detected</span>
    </div>
  {/if}

  <!-- Added to Downloads Feedback Notice (Exact LibreGet / Demo match) -->
  {#if addedNoticeActive}
    <div class="feedback feedback-enter added-notice">
      <strong class="added-notice-title">{$t('librebox.added_to_downloads')}</strong>
      <button type="button" class="added-notice-link" onclick={() => goto("/downloads")}>
        {$t('librebox.open_downloads')} &rarr;
      </button>
    </div>
  {/if}

  <!-- Alerts / Notices -->
  {#if depStatus && !depStatus.ok}
    <div class="notice notice-danger" role="alert">
      <span>{namiStatusMessage(depStatus.message, tr)}</span>
    </div>
  {/if}

  {#if status && !status.includes('Starting') && !status.includes('Switching to')}
    <div
      class="notice"
      class:notice-success={statusType === 'success'}
      class:notice-danger={statusType === 'error'}
      class:notice-info={statusType === 'info'}
      role={statusType === 'error' ? 'alert' : 'status'}
    >
      <div class="notice-body">
        <span class="notice-title">{namiStatusMessage(status, tr)}</span>
        {#if statusDetail}
          <details style="margin-top: 4px;">
            <summary style="cursor: pointer; font-size: 11px;">View Log</summary>
            <pre style="margin-top: 4px; font-size: 11px; max-height: 120px; overflow-y: auto;">{statusDetail}</pre>
          </details>
        {/if}
      </div>
      <button type="button" class="btn btn-sm btn-ghost" onclick={clearStatus}>×</button>
    </div>
  {/if}

  {#if missingOutputDir}
    <div class="notice notice-warning" role="alert">
      <span>Output directory is not set.</span>
      <button type="button" class="btn btn-sm btn-secondary" onclick={() => onSwitchToSettings?.()}>
        Configure Settings
      </button>
    </div>
  {/if}

  {#if needsCookiesWarning}
    <div class="notice notice-warning" role="alert">
      <span>Stories & Highlights require active cookies. Please connect a browser bridge in Settings.</span>
      <button type="button" class="btn btn-sm btn-secondary" onclick={() => onSwitchToSettings?.()}>
        Settings
      </button>
    </div>
  {/if}

  <!-- Creator Inspector Panel (When URL is entered) -->
  {#if currentCreator}
    <section class="home-panel" aria-label="Creator preview">
      <div class="home-panel-body">
        <!-- MediaPreview Row -->
        <div class="preview-container">
          <div class="preview-thumbnail-wrapper">
            {#if !avatarError && currentCreator.avatar}
              <img
                class="preview-thumbnail"
                src={currentCreator.avatar}
                alt={currentCreator.name}
                onerror={() => (avatarError = true)}
              />
            {:else if isResolvingMeta}
              <div class="preview-shimmer-badge" data-platform={currentCreator.platform}>
                <div class="shimmer-pulse"></div>
              </div>
            {:else}
              <div class="preview-initials-badge" data-platform={currentCreator.platform}>
                <span>{currentCreator.name.slice(0, 2).toUpperCase()}</span>
              </div>
            {/if}
          </div>

          <div class="preview-info">
            <p class="preview-title">{currentCreator.name}</p>
            <p class="preview-author">{currentCreator.meta}</p>
          </div>

          <!-- Save / Saved Toggle Button -->
          <button
            type="button"
            class="btn btn-sm"
            class:btn-primary={isSaved}
            class:btn-secondary={!isSaved}
            onclick={toggleSaveProfile}
            title={isSaved ? 'Remove from Saved' : 'Save Profile'}
          >
            <svg viewBox="0 0 24 24" width="13" height="13" fill={isSaved ? 'currentColor' : 'none'} stroke="currentColor" stroke-width="2">
              <path d="m19 21-7-4-7 4V5a2 2 0 0 1 2-2h10a2 2 0 0 1 2 2v16z" />
            </svg>
            <span>{isSaved ? 'Saved' : 'Save'}</span>
          </button>
        </div>

        <!-- Scope / Content Type Segmented Selector -->
        <div class="category-block">
          <span class="category-heading">CATEGORY</span>
          <div class="segmented segmented-lg" role="group">
            {#each CONTENT_TYPES as type (type.key)}
              <button
                type="button"
                class="segmented-btn"
                class:active={contentType === type.key}
                disabled={downloading || ((type.key === 'stories' || type.key === 'highlights') && !isInstagram)}
                onclick={() => {
                  contentType = type.key;
                  setDraft({ contentType: type.key });
                }}
              >
                {type.label}
              </button>
            {/each}
          </div>
        </div>

        <!-- Full-Width Download Primary Button -->
        <button
          type="button"
          class="download-primary-btn"
          onclick={startDownload}
          disabled={missingOutputDir || needsCookiesWarning || platformMismatch}
        >
          Download
        </button>
      </div>
    </section>
  {/if}

  <!-- Clean Saved Profiles Section (When URL bar is empty) -->
  {#if !url.trim()}
    <section class="saved-profiles-section">
      <div class="section-header">
        <span class="section-header-title">Saved Profiles</span>
        <span class="badge badge-neutral">{savedProfiles.length}</span>
      </div>

      {#if savedProfiles.length > 0}
        <div class="surface-card saved-profiles-card">
          {#each savedProfiles as profile (profile.url)}
            <div
              class="list-row interactive"
              role="button"
              tabindex="0"
              onclick={() => loadProfileDirectly(profile.url)}
              onkeydown={(e) => {
                if (e.key === 'Enter') loadProfileDirectly(profile.url);
              }}
            >
              {#if profileAvatars[profile.url] || getCachedCreator(profile.platform, profile.username || profile.url)?.avatar}
                <img
                  src={profileAvatars[profile.url] || getCachedCreator(profile.platform, profile.username || profile.url)?.avatar}
                  alt={profile.username || profile.platform}
                  class="preview-avatar-thumb"
                  onerror={() => {
                    delete profileAvatars[profile.url];
                  }}
                />
              {:else}
                <div class="preview-initials-badge" data-platform={profile.platform}>
                  <span>{(profile.username || profile.platform).slice(0, 2).toUpperCase()}</span>
                </div>
              {/if}
              <span class="list-row-content">
                <span class="list-row-title">{profile.username || profile.url}</span>
                <span class="list-row-sub">{profile.url} • {profile._platformLabel || profile.platform}</span>
              </span>
              <span class="list-row-trailing">
                <button
                  type="button"
                  class="btn btn-sm btn-secondary"
                  onclick={(e) => {
                    e.stopPropagation();
                    loadProfileDirectly(profile.url);
                  }}
                >
                  Download
                </button>
                <button
                  type="button"
                  class="btn btn-sm btn-ghost"
                  title="Remove Profile"
                  onclick={(e) => {
                    e.stopPropagation();
                    handleDeleteSavedProfile(profile);
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
      {:else}
        <div class="empty-state">
          <p class="empty-state-hint">No saved profiles yet. Enter a URL above and click Save.</p>
        </div>
      {/if}
    </section>
  {/if}
</div>

<style>
  .nami-home-stage {
    display: flex;
    flex-direction: column;
    align-items: center;
    gap: var(--space-4);
    width: 100%;
    max-width: 620px;
    margin: 0 auto;
  }

  /* Mascot Hero */
  .nami-mascot-wrapper {
    position: relative;
    display: flex;
    align-items: center;
    justify-content: center;
    padding: var(--space-3) 0 var(--space-2);
  }

  .nami-mascot-glow {
    position: absolute;
    width: 160px;
    height: 160px;
    border-radius: 50%;
    background: radial-gradient(circle, color-mix(in srgb, var(--accent) 22%, transparent) 0%, transparent 70%);
    pointer-events: none;
    z-index: 0;
  }

  .nami-mascot-icon {
    position: relative;
    z-index: 1;
    filter: drop-shadow(0 6px 12px rgba(0, 0, 0, 0.3));
  }

  /* Canonical LibreGet HomeUrlBar */
  .url-bar {
    position: relative;
    display: flex;
    align-items: center;
    gap: 8px;
    width: 100%;
    height: 58px;
    padding: 0 10px 0 18px;
    border-radius: 20px;
    background: var(--surface);
    box-shadow:
      inset 0 0 0 var(--hairline) var(--content-border),
      0 12px 32px rgba(var(--shadow-ink, 0, 0, 0), var(--elev-alpha-1, 0.2)),
      0 2px 6px rgba(var(--shadow-ink, 0, 0, 0), calc(var(--elev-alpha-1, 0.2) * 0.6));
    transition: box-shadow var(--duration-base) var(--ease-out);
  }

  .url-bar:focus-within {
    box-shadow:
      inset 0 0 0 1px color-mix(in srgb, var(--accent) 70%, transparent),
      0 0 0 5px var(--accent-soft),
      0 12px 32px rgba(var(--shadow-ink, 0, 0, 0), var(--elev-alpha-1, 0.2));
  }

  .url-input {
    flex: 1;
    min-width: 0;
    height: 100%;
    border: none;
    background: transparent;
    color: var(--text);
    font: inherit;
    font-size: 15px;
    letter-spacing: -0.01em;
    outline: none;
  }

  .url-input::placeholder {
    color: var(--text-dim);
  }

  .clear-btn {
    display: grid;
    place-items: center;
    width: 28px;
    height: 28px;
    border: none;
    border-radius: 50%;
    background: transparent;
    color: var(--text-faint);
    cursor: pointer;
    flex-shrink: 0;
  }

  .clear-btn:hover {
    color: var(--text-muted);
  }

  .go-btn {
    display: grid;
    place-items: center;
    width: 40px;
    height: 40px;
    flex-shrink: 0;
    border: none;
    border-radius: 50%;
    background: var(--cta);
    color: var(--on-cta);
    cursor: pointer;
    box-shadow: 0 4px 14px var(--accent-glow, rgba(255, 159, 10, 0.4));
    transition: background var(--duration-fast) var(--ease-out), transform var(--duration-fast) var(--ease-out);
  }

  .go-btn:hover:not(:disabled) {
    background: var(--cta-hover);
  }

  .go-btn:active:not(:disabled) {
    background: var(--cta-press);
    transform: scale(0.95);
  }

  .go-btn:disabled {
    opacity: 0.5;
    cursor: not-allowed;
  }

  .paste-btn {
    display: inline-flex;
    align-items: center;
    gap: 8px;
    height: 36px;
    padding: 0 8px 0 14px;
    flex-shrink: 0;
    border: none;
    border-radius: 999px;
    background: var(--fill-1);
    color: var(--text);
    font: inherit;
    font-size: var(--text-sm);
    font-weight: 600;
    cursor: pointer;
    transition: background var(--duration-fast) var(--ease-out);
  }

  .paste-btn:hover {
    background: var(--fill-2);
  }

  .paste-btn kbd {
    font: inherit;
    font-size: 11px;
    font-weight: 600;
    padding: 2px 6px;
    border-radius: 6px;
    background: var(--surface);
    color: var(--text-dim);
    box-shadow: inset 0 0 0 var(--hairline) var(--content-border);
  }

  /* Feedback */
  .feedback {
    display: flex;
    align-items: center;
    justify-content: center;
    gap: 6px;
    min-height: 20px;
    color: var(--text-muted);
    font-size: var(--text-sm);
    font-weight: 500;
  }

  .added-notice {
    gap: var(--space-2);
    font-size: var(--text-sm);
  }

  .added-notice-title {
    font-weight: 600;
    color: var(--text);
  }

  .added-notice-link {
    background: none;
    border: none;
    padding: 0;
    font: inherit;
    font-size: var(--text-sm);
    font-weight: 600;
    color: var(--accent-hi);
    cursor: pointer;
  }

  .added-notice-link:hover {
    text-decoration: underline;
    text-underline-offset: 2px;
  }

  .feedback-enter {
    animation: feedbackEnter var(--duration-base) var(--ease-out);
  }

  @keyframes feedbackEnter {
    from { opacity: 0; transform: translateY(-3px); }
    to { opacity: 1; transform: translateY(0); }
  }

  /* Creator Inspector Panel */
  .home-panel {
    width: 100%;
    background: var(--surface);
    border-radius: var(--radius-lg);
    box-shadow: inset 0 0 0 var(--hairline) var(--content-border);
    padding: var(--space-4);
    animation: panel-in var(--duration-slow) var(--ease-spring);
    box-sizing: border-box;
  }

  .home-panel-body {
    display: flex;
    flex-direction: column;
    gap: var(--space-4);
  }

  @keyframes panel-in {
    from { opacity: 0; transform: translateY(-6px) scale(0.99); }
    to { opacity: 1; transform: translateY(0) scale(1); }
  }

  .preview-container {
    display: flex;
    align-items: center;
    gap: var(--space-3);
    width: 100%;
  }

  .preview-thumbnail-wrapper {
    position: relative;
    flex-shrink: 0;
    border-radius: var(--radius-md);
    overflow: hidden;
    width: 52px;
    height: 52px;
    box-shadow: inset 0 0 0 var(--hairline) var(--content-border);
    background: var(--fill-2);
  }

  .preview-thumbnail {
    width: 100%;
    height: 100%;
    object-fit: cover;
    display: block;
  }

  .preview-initials-badge {
    width: 100%;
    height: 100%;
    display: flex;
    align-items: center;
    justify-content: center;
    font-size: 15px;
    font-weight: 700;
    color: var(--on-accent);
    background: var(--accent);
  }

  .preview-initials-badge[data-platform="instagram"] {
    background: linear-gradient(135deg, #833ab4, #fd1d1d, #fcb045);
    color: #ffffff;
  }

  .preview-initials-badge[data-platform="tiktok"] {
    background: #010101;
    color: #00f2fe;
  }

  .preview-initials-badge[data-platform="x"] {
    background: #000000;
    color: #ffffff;
  }

  .preview-initials-badge[data-platform="facebook"] {
    background: #1877f2;
    color: #ffffff;
  }

  .preview-info {
    flex: 1;
    min-width: 0;
    display: flex;
    flex-direction: column;
    gap: 3px;
  }

  .preview-title {
    font-size: var(--text-md);
    font-weight: 600;
    color: var(--text);
    margin: 0;
    overflow: hidden;
    text-overflow: ellipsis;
    white-space: nowrap;
  }

  .preview-author {
    font-size: var(--text-sm);
    color: var(--text-dim);
    margin: 0;
    overflow: hidden;
    text-overflow: ellipsis;
    white-space: nowrap;
  }

  .category-block {
    display: flex;
    flex-direction: column;
    gap: 6px;
  }

  .category-heading {
    font-size: 11px;
    font-weight: 600;
    letter-spacing: 0.05em;
    color: var(--text-dim);
  }

  .segmented-lg {
    width: 100%;
    display: flex;
  }

  .segmented-lg .segmented-btn {
    flex: 1;
    height: 28px;
    text-align: center;
  }

  /* Full-width orange download button */
  .download-primary-btn {
    display: inline-flex;
    align-items: center;
    justify-content: center;
    gap: var(--space-2);
    background: var(--cta);
    color: var(--on-cta);
    font-size: var(--text-md);
    font-weight: 600;
    letter-spacing: var(--track-snug);
    height: 42px;
    padding: 0 var(--space-4);
    border-radius: var(--radius-md);
    border: none;
    cursor: pointer;
    width: 100%;
    box-shadow: inset 0 0 0 var(--hairline) color-mix(in srgb, var(--on-cta) 12%, transparent);
    transition: background var(--duration-fast) var(--ease-out), transform var(--duration-fast) var(--ease-out);
  }

  .download-primary-btn:hover:not(:disabled) {
    background: var(--cta-hover);
  }

  .download-primary-btn:active:not(:disabled) {
    background: var(--cta-press);
    transform: scale(0.99);
  }

  .download-primary-btn:disabled {
    opacity: 0.5;
    cursor: not-allowed;
  }



  /* Saved Profiles Section */
  .saved-profiles-section {
    width: 100%;
    display: flex;
    flex-direction: column;
    gap: var(--space-2);
    margin-top: var(--space-2);
  }

  .saved-profiles-card {
    overflow: hidden;
  }

  .saved-profiles-card .preview-initials-badge {
    width: 32px;
    height: 32px;
    border-radius: var(--radius-sm);
    font-size: 11px;
    flex-shrink: 0;
  }

  .preview-avatar-thumb {
    width: 32px;
    height: 32px;
    border-radius: var(--radius-sm);
    object-fit: cover;
    flex-shrink: 0;
  }

  .preview-shimmer-badge {
    width: 48px;
    height: 48px;
    border-radius: var(--radius-sm);
    background: var(--surface-hover);
    position: relative;
    overflow: hidden;
    flex-shrink: 0;
  }

  .shimmer-pulse {
    width: 100%;
    height: 100%;
    background: linear-gradient(
      90deg,
      transparent 0%,
      rgba(255, 255, 255, 0.08) 50%,
      transparent 100%
    );
    animation: shimmer 1.5s infinite;
  }

  @keyframes shimmer {
    0% { transform: translateX(-100%); }
    100% { transform: translateX(100%); }
  }
</style>
