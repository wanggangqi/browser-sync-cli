import { ref, computed } from 'vue'
import { invoke } from '@tauri-apps/api/tauri'
import type { Space, SpaceConfig, BookmarkSyncData } from '@/types'

const config = ref<SpaceConfig | null>(null)
const isLoading = ref(false)
const error = ref<string | null>(null)

export function useSpaceStore() {
  const spaces = computed(() => config.value?.spaces || [])
  const activeSpaceId = computed(() => config.value?.activeSpaceId || 'default-chrome')
  const activeSpace = computed(() => {
    return spaces.value.find(s => s.id === activeSpaceId.value)
  })

  async function loadSpaces() {
    isLoading.value = true
    error.value = null
    try {
      config.value = await invoke<SpaceConfig>('get_spaces')
    } catch (e) {
      error.value = String(e)
      console.error('Failed to load spaces:', e)
    } finally {
      isLoading.value = false
    }
  }

  async function createSpace(
    name: string,
    type: 'local' | 'remote',
    apiUrl?: string,
    apiKey?: string,
    browser?: 'chrome' | 'edge'
  ) {
    try {
      const space = await invoke<Space>('create_space', {
        name,
        spaceType: type,
        apiUrl,
        apiKey,
        browser
      })
      await loadSpaces()
      return space
    } catch (e) {
      error.value = String(e)
      throw e
    }
  }

  async function updateSpace(space: Space) {
    try {
      await invoke('update_space', { space })
      await loadSpaces()
    } catch (e) {
      error.value = String(e)
      throw e
    }
  }

  async function deleteSpace(id: string) {
    try {
      await invoke('delete_space', { id })
      await loadSpaces()
    } catch (e) {
      error.value = String(e)
      throw e
    }
  }

  async function setActiveSpace(id: string) {
    try {
      await invoke('set_active_space', { id })
      // 重新加载空间配置以触发响应式更新
      await loadSpaces()
    } catch (e) {
      error.value = String(e)
      throw e
    }
  }

  async function fetchRemoteBookmarks(spaceId: string, apiUrl: string, apiKey?: string) {
    try {
      const data = await invoke<BookmarkSyncData>('fetch_remote_bookmarks', {
        apiUrl,
        apiKey
      })
      await invoke('save_space_cache', { spaceId, data })
      await loadSpaces()
      return data
    } catch (e) {
      error.value = String(e)
      throw e
    }
  }

  async function getSpaceCache(spaceId: string): Promise<BookmarkSyncData | null> {
    try {
      return await invoke<BookmarkSyncData | null>('get_space_cache', { spaceId })
    } catch (e) {
      console.error('Failed to get space cache:', e)
      return null
    }
  }

  return {
    config,
    spaces,
    activeSpaceId,
    activeSpace,
    isLoading,
    error,
    loadSpaces,
    createSpace,
    updateSpace,
    deleteSpace,
    setActiveSpace,
    fetchRemoteBookmarks,
    getSpaceCache
  }
}