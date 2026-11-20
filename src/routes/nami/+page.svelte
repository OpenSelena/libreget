<script lang="ts">
  import DownloadManager from '$components/nami/DownloadManager.svelte';
  import ProfileManager from '$components/nami/ProfileManager.svelte';
  import HistoryManager from '$components/nami/HistoryManager.svelte';
  import SettingsManager from '$components/nami/SettingsManager.svelte';
  import { setToolbar } from '$lib/stores/toolbar-store.svelte';
  import { setDraft } from '$lib/stores/nami_download_store.svelte';
  import { t } from '$lib/i18n';

  type TabKey = 'download' | 'profiles' | 'history' | 'settings';

  let activeTab = $state<TabKey>('download');

  $effect(() => {
    return setToolbar({
      segments: [
        { id: 'download', label: $t('nami.tab_download') || 'Home' },
        { id: 'profiles', label: $t('nami.tab_profiles') || 'Profiles' },
        { id: 'history', label: $t('nami.tab_history') || 'History' },
        { id: 'settings', label: $t('nami.tab_settings') || 'Settings' },
      ],
      activeSegment: activeTab,
      onSegment: (id) => {
        activeTab = id as TabKey;
      },
    });
  });

  function switchTab(tab: TabKey) {
    activeTab = tab;
  }
</script>

<svelte:head>
  <title>{$t('nami.page_title') || 'LibreGet — Nami'}</title>
</svelte:head>

<div class="nami-container">
  <div class="nami-content" role="tabpanel" id="tabpanel-{activeTab}">
    {#if activeTab === 'download'}
      <DownloadManager
        onSwitchToProfiles={() => switchTab('profiles')}
        onSwitchToSettings={() => switchTab('settings')}
      />
    {:else if activeTab === 'profiles'}
      <ProfileManager
        onSelectProfileForDownload={(profileUrl) => {
          setDraft({ url: profileUrl, selectedProfileUrl: profileUrl });
          switchTab('download');
        }}
      />
    {:else if activeTab === 'history'}
      <HistoryManager onSwitchToDownload={() => switchTab('download')} />
    {:else}
      <SettingsManager />
    {/if}
  </div>
</div>

<style>
  .nami-container {
    flex: 1;
    min-height: 0;
    width: 100%;
    display: flex;
    flex-direction: column;
    overflow-y: auto;
    overscroll-behavior-y: contain;
    transform: translateZ(0);
    will-change: scroll-position;
  }

  .nami-content {
    width: 100%;
    max-width: 780px;
    margin: 0 auto;
    padding: var(--space-4) var(--space-5) var(--space-7);
    box-sizing: border-box;
    display: flex;
    flex-direction: column;
    gap: var(--space-4);
  }
</style>
