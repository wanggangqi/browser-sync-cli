<script setup lang="ts">
import { ref, computed, onMounted, onUnmounted } from 'vue'
import { invoke } from '@tauri-apps/api/tauri'
import { listen, UnlistenFn } from '@tauri-apps/api/event'
import { open, save } from '@tauri-apps/api/dialog'

interface BookmarkNode {
  id: string
  title: string
  url?: string
  parent_id?: string
  index?: number
  date_added?: number
  children?: BookmarkNode[]
}

interface BookmarkSyncData {
  version: string
  last_sync: string
  bookmarks: BookmarkNode[]
}

const bookmarks = ref<BookmarkNode[]>([])
const searchQuery = ref('')
const status = ref('')
const statusType = ref<'success' | 'error' | 'info'>('info')
const isLoading = ref(false)

// Flatten bookmarks tree for display
function flattenBookmarks(nodes: BookmarkNode[], result: BookmarkNode[] = []): BookmarkNode[] {
  for (const node of nodes) {
    if (node.url) {
      result.push(node)
    }
    if (node.children) {
      flattenBookmarks(node.children, result)
    }
  }
  return result
}

const flatBookmarks = computed(() => {
  return flattenBookmarks(bookmarks.value)
})

const filteredBookmarks = computed(() => {
  if (!searchQuery.value) {
    return flatBookmarks.value
  }
  const query = searchQuery.value.toLowerCase()
  return flatBookmarks.value.filter(b =>
    b.title.toLowerCase().includes(query) ||
    (b.url && b.url.toLowerCase().includes(query))
  )
})

async function loadBookmarks() {
  isLoading.value = true
  try {
    const data = await invoke<BookmarkSyncData>('get_bookmarks')
    bookmarks.value = data.bookmarks || []
    showStatus('Bookmarks loaded successfully', 'success')
  } catch (error) {
    showStatus(`Failed to load bookmarks: ${error}`, 'error')
    bookmarks.value = []
  } finally {
    isLoading.value = false
  }
}

async function openUrl(url: string) {
  try {
    await invoke('open_url', { url })
  } catch (error) {
    showStatus(`Failed to open URL: ${error}`, 'error')
  }
}

async function exportBookmarks() {
  try {
    const filePath = await save({
      defaultPath: `bookmarks_export_${new Date().toISOString().slice(0, 10)}.json`,
      filters: [{ name: 'JSON', extensions: ['json'] }]
    })

    if (filePath) {
      await invoke('export_bookmarks_to_path', { path: filePath })
      showStatus(`Bookmarks exported to: ${filePath}`, 'success')
    }
  } catch (error) {
    showStatus(`Export failed: ${error}`, 'error')
  }
}

async function importBookmarks() {
  try {
    const filePath = await open({
      multiple: false,
      filters: [{ name: 'JSON', extensions: ['json'] }]
    })

    if (filePath) {
      await invoke('import_bookmarks', { path: filePath })
      await loadBookmarks()
      showStatus('Bookmarks imported successfully', 'success')
    }
  } catch (error) {
    showStatus(`Import failed: ${error}`, 'error')
  }
}

function showStatus(message: string, type: 'success' | 'error' | 'info') {
  status.value = message
  statusType.value = type
  setTimeout(() => {
    status.value = ''
  }, 3000)
}

let unlisten: UnlistenFn | null = null

onMounted(async () => {
  await loadBookmarks()

  // Listen for bookmark changes
  unlisten = await listen('bookmark-changed', () => {
    loadBookmarks()
  })
})

onUnmounted(() => {
  if (unlisten) {
    unlisten()
  }
})
</script>

<template>
  <div class="container">
    <h1>Bookmark Sync</h1>

    <div v-if="status" :class="['status', `status-${statusType}`]">
      {{ status }}
    </div>

    <input
      v-model="searchQuery"
      type="text"
      class="search-bar"
      placeholder="Search bookmarks..."
    />

    <div class="toolbar">
      <button class="btn" @click="loadBookmarks" :disabled="isLoading">
        {{ isLoading ? 'Loading...' : 'Refresh' }}
      </button>
      <button class="btn btn-secondary" @click="exportBookmarks">
        Export
      </button>
      <button class="btn btn-secondary" @click="importBookmarks">
        Import
      </button>
    </div>

    <div v-if="isLoading" class="loading">
      <div class="spinner"></div>
    </div>

    <div v-else-if="filteredBookmarks.length === 0" class="empty-state">
      <p v-if="searchQuery">No bookmarks match your search.</p>
      <p v-else>No bookmarks found. Add some bookmarks in your browser!</p>
    </div>

    <ul v-else class="bookmark-list">
      <li
        v-for="bookmark in filteredBookmarks"
        :key="bookmark.id"
        class="bookmark-item"
        @click="openUrl(bookmark.url!)"
      >
        <img
          v-if="bookmark.url"
          :src="`https://www.google.com/s2/favicons?domain=${bookmark.url}&sz=32`"
          class="bookmark-icon"
          alt=""
        />
        <span v-else class="bookmark-icon folder-icon">📁</span>
        <div class="bookmark-info">
          <div class="bookmark-title">{{ bookmark.title }}</div>
          <div v-if="bookmark.url" class="bookmark-url">{{ bookmark.url }}</div>
        </div>
      </li>
    </ul>
  </div>
</template>